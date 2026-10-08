// Release builds use the GUI subsystem so Windows opens no console window
// beside the game. Debug builds keep the console for development output.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
// Only Steam reports achievements and presence; both are tested without it.
#[cfg(any(feature = "steam", test))]
mod achievements;
mod atlas;
mod audio;
mod diagnostics;
mod display;
mod input;
mod locale;
mod perf;
mod pixel_font;
#[cfg(any(feature = "steam", test))]
mod presence;
mod render;
mod settings;
mod smoke;
mod steam;
mod storage;
mod ui;

use ark::{
    Events, Game, Input, Medals, Mode, SectorSummary, Stage,
    clock::FixedClock,
    field::{BOTTOM, FIELD, LEFT, RIGHT, TOP},
    geom::V2,
    sectors::SectorId,
    tuning::ADVANCE_DELAY_TICKS,
};
use ark_text::Locale;
use audio::Audio;
use input::{Device, Gamepads};
use macroquad::prelude::*;
use perf::Perf;
use render::Renderer;
use std::{path::PathBuf, time::Instant};
use steam::Steam;
use storage::{Origin, Profile};
use ui::{Controls, Preview, Screen, Ui};

/// A results card for screens the smoke test shows without playing to them.
fn preview(stage: Stage) -> Option<Preview> {
    Some(Preview {
        stage,
        summary: SectorSummary {
            ticks: 18240,
            medals: Medals::ALL,
            bonus: 2000,
            ..SectorSummary::default()
        },
    })
}
fn flag(name: &str) -> bool {
    std::env::args().any(|a| a == name)
}
fn config(fullscreen: bool) -> Conf {
    Conf {
        window_title: "ARKONK".into(),
        window_width: 960,
        window_height: 900,
        high_dpi: true,
        fullscreen,
        sample_count: 1,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            #[cfg(target_os = "macos")]
            apple_gfx_api: if flag("--opengl") {
                miniquad::conf::AppleGfxApi::OpenGl
            } else {
                miniquad::conf::AppleGfxApi::Metal
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
/// Loads progress and returns where to save it. `None` disables saving: test
/// modes leave real progress alone, and an unreadable file must not be replaced.
fn load_progress(path: Option<PathBuf>) -> (Profile, Option<PathBuf>, bool) {
    let Some(path) = path else {
        return (Profile::default(), None, false);
    };
    let loaded = Profile::load(&path);
    for (file, why) in &loaded.set_aside {
        diagnostics::error(format_args!(
            "Unreadable progress kept as {} ({why})",
            file.display()
        ));
    }
    if loaded.origin == Origin::Backup {
        diagnostics::error("Progress restored from the last-known-good backup");
    }
    diagnostics::info(format_args!(
        "Progress: {:?} from {}",
        loaded.origin,
        path.display()
    ));
    if let Some(why) = &loaded.blocked {
        diagnostics::error(format_args!("Saving disabled this session: {why}"));
        return (loaded.profile, None, true);
    }
    (loaded.profile, Some(path), false)
}
struct Focus {
    lost: bool,
    focused: bool,
}
impl Default for Focus {
    fn default() -> Self {
        Self {
            lost: false,
            #[cfg(target_os = "macos")]
            focused: miniquad::native::macos::window_has_focus(),
            #[cfg(not(target_os = "macos"))]
            focused: true,
        }
    }
}
impl miniquad::EventHandler for Focus {
    fn update(&mut self) {}
    fn draw(&mut self) {}
    fn window_minimized_event(&mut self) {
        self.lost = true;
        self.focused = false;
    }
    fn window_restored_event(&mut self) {
        self.focused = true;
    }
}
fn enter(
    game: Game,
    current: &mut Game,
    ui: &mut Ui,
    renderer: &mut Renderer,
    clock: &mut FixedClock,
) {
    *current = game;
    ui.screen = Screen::Play;
    ui.paused = false;
    renderer.reset();
    clock.reset();
}
fn home(ui: &mut Ui, profile: &Profile) {
    ui.screen = Screen::Title;
    ui.paused = false;
    ui.choice = usize::from(profile.progress.checkpoint().is_none());
}
/// A GUI-subsystem process starts without a console. When launched from a
/// terminal (`--version`, smoke and perf tests), borrow the parent's console so
/// the output is visible. Redirected or piped handles are already valid and
/// are left alone.
#[cfg(windows)]
fn attach_parent_console() {
    unsafe extern "system" {
        fn GetStdHandle(id: u32) -> *mut std::ffi::c_void;
        fn AttachConsole(process: u32) -> i32;
    }
    const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
    const ATTACH_PARENT_PROCESS: u32 = u32::MAX;
    // SAFETY: both calls take plain integers and only touch process-wide
    // console state, before any other thread exists.
    unsafe {
        if GetStdHandle(STD_OUTPUT_HANDLE).is_null() {
            // Failure means there is no parent console (Explorer or Steam
            // launched the game), so there is nowhere to show output anyway.
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }
}
fn main() {
    #[cfg(windows)]
    attach_parent_console();
    if std::env::args().skip(1).any(|a| a == "--version") {
        println!("arkonk {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let smoke = [
        "--smoke-test",
        "--flow-test",
        "--perf-test",
        "--effects-test",
    ]
    .into_iter()
    .any(flag);
    let data = (!smoke).then(diagnostics::data_dir).flatten();
    diagnostics::init(data.as_ref().map(|d| d.join("logs")));
    // Before any window exists: Steam is launching another copy of the game.
    if steam::restart_through_steam() {
        return;
    }
    let (mut profile, path, save_blocked) = load_progress(data.map(|d| d.join("progress.txt")));
    profile.settings.fullscreen |= flag("--fullscreen");
    // Tests run on a desktop someone is using; `--show` is for watching one.
    #[cfg(target_os = "macos")]
    if smoke && !flag("--show") {
        miniquad::native::macos::run_hidden();
    }
    macroquad::Window::from_config(
        config(profile.settings.fullscreen),
        run(profile, path, save_blocked),
    );
}
async fn run(mut profile: Profile, path: Option<PathBuf>, save_blocked: bool) {
    prevent_quit();
    let flow = flag("--flow-test");
    let perf_test = flag("--perf-test");
    let effects = flag("--effects-test");
    let smoke = perf_test || effects || flow || flag("--smoke-test");
    // Test runs must not unlock achievements or show presence.
    let mut steam = if smoke {
        Steam::off()
    } else {
        Steam::init(&profile.progress)
    };
    let mut dirty = false;
    let mut last_save_attempt = -5.0;
    let mut game = Game::new();
    let (language, origin) = locale::choose(
        locale::flag(),
        profile.settings.locale,
        steam.language(),
        locale::system(),
    );
    let mut renderer = Renderer::new(language);
    diagnostics::set_backend(renderer.backend());
    let (atlas_w, atlas_h) = renderer.atlas_size();
    diagnostics::info(format_args!(
        "Language: {} ({origin:?}), glyph atlas {atlas_w}x{atlas_h}",
        renderer.locale().tag()
    ));
    diagnostics::info(format_args!(
        "Started: {}, window {}x{} at {}x scale, {}",
        renderer.backend(),
        screen_width(),
        screen_height(),
        screen_dpi_scale(),
        if profile.settings.fullscreen {
            "fullscreen"
        } else {
            "windowed"
        }
    ));
    let mut audio = Audio::new().await;
    let mut perf = Perf::new();
    let mut trace = perf::FrameTrace::new(perf_test);
    let mut ui = Ui::default();
    home(&mut ui, &profile);
    ui.save_error = save_blocked;
    if smoke && !flow {
        ui.screen = Screen::Play;
    }
    let mut clock = FixedClock::default();
    let mut stats = false;
    let mut display = display::Display::new(profile.settings.fullscreen, (960.0, 900.0));
    let mut last_frame = Instant::now();
    let mut last_mouse =
        render::mouse().unwrap_or(V2::new(render::WIDTH / 2.0, render::HEIGHT / 2.0));
    let mut mouse_control = false;
    let mut pending_launch = false;
    let mut cursor_visible = true;
    let mut frames = 0;
    let subscriber = macroquad::input::utils::register_input_subscriber();
    let mut focus = Focus::default();
    let mut pads = Gamepads::new(!smoke);
    loop {
        let now = get_time();
        let frame_time = Instant::now();
        let frame_seconds = if flow {
            1.0 / 60.0
        } else {
            frame_time.duration_since(last_frame).as_secs_f64()
        };
        last_frame = frame_time;
        if smoke && frames == if effects && !perf_test { 350 } else { 300 } {
            perf = Perf::new();
        }
        perf.frame(frame_seconds * 1000.0);
        macroquad::input::utils::repeat_all_miniquad_input(&mut focus, subscriber);
        steam.run_callbacks();
        if is_key_pressed(KeyCode::Q) || is_quit_requested() {
            break;
        }
        if is_key_pressed(KeyCode::M) {
            profile.settings.muted = !profile.settings.muted;
            dirty = true;
        }
        if is_key_pressed(KeyCode::LeftBracket) {
            profile.settings.volume = profile.settings.volume.saturating_sub(1);
            dirty = true;
        }
        if is_key_pressed(KeyCode::RightBracket) {
            profile.settings.volume = (profile.settings.volume + 1).min(settings::MAX_VOLUME);
            dirty = true;
        }
        if is_key_pressed(KeyCode::F3) {
            stats = !stats;
        }
        if is_key_pressed(KeyCode::F) {
            profile.settings.fullscreen = !profile.settings.fullscreen;
            dirty = true;
        }
        // A settings screen changes `profile.settings.fullscreen`; this applies it.
        let held = display.update(profile.settings.fullscreen, frame_seconds);
        audio.muted = profile.settings.muted;
        audio.volume = f32::from(profile.settings.volume) / f32::from(settings::MAX_VOLUME);
        let mouse = render::mouse().unwrap_or(last_mouse);
        let moved =
            !flow && ((mouse.x - last_mouse.x).abs() > 0.5 || (mouse.y - last_mouse.y).abs() > 0.5);
        if moved {
            mouse_control = true;
        }
        last_mouse = mouse;
        let pointer = vec2(mouse.x, mouse.y);
        // Asked every frame: the macOS minimize event also fires when the window moves.
        #[cfg(target_os = "macos")]
        let focused = miniquad::native::macos::window_has_focus();
        #[cfg(not(target_os = "macos"))]
        let focused = focus.focused;
        let pad = pads.poll(
            frame_seconds,
            focused,
            ui.paused || matches!(game.stage(), Stage::GameOver | Stage::Victory),
            moved || input::pointer_pressed(),
        );
        ui.device = pads.device;
        let Controls {
            click,
            confirm,
            escape,
            pause,
            up,
            down,
            left,
            right,
            restart,
            focus_lost,
        } = if flow {
            smoke::flow(frames, &mut game, &ui, &profile)
        } else {
            input::merge(Controls::read(), pad.controls)
        };
        focus.lost |= focus_lost || pad.lost;
        focus.lost |= steam.overlay_opened();
        if effects {
            smoke::effects(&mut game, frames);
        }
        let terminal = matches!(game.stage(), Stage::GameOver | Stage::Victory);
        if (!smoke || flow)
            && ui.screen == Screen::Play
            && !terminal
            && (focus.lost || (frame_seconds > 0.25 && game.stage() == Stage::Playing && !held))
        {
            if !ui.paused {
                diagnostics::info(format_args!(
                    "Paused automatically: {}",
                    if focus.lost {
                        "focus lost"
                    } else {
                        "frame stall"
                    }
                ));
            }
            ui.paused = true;
            ui.choice = 0;
            pending_launch = false;
            clock.reset();
        }
        focus.lost = false;
        let mut changed = false;
        match ui.screen {
            Screen::Title => {
                // Continue is unavailable without a checkpoint; never select it.
                let first = usize::from(profile.progress.checkpoint().is_none());
                let hovered = ui::hover_menu(pointer).filter(|&row| row >= first);
                ui.choice = ui::step_menu(ui.choice, first, up, down);
                if (moved || click)
                    && let Some(row) = hovered
                {
                    ui.choice = row;
                }
                if confirm || (click && hovered.is_some()) {
                    match ui.choice {
                        0 => {
                            if let Some(c) = profile.progress.checkpoint() {
                                enter(
                                    Game::resume(c),
                                    &mut game,
                                    &mut ui,
                                    &mut renderer,
                                    &mut clock,
                                );
                                changed = true;
                            }
                        }
                        1 => {
                            let fresh = Game::new();
                            profile.progress.begin(&fresh);
                            dirty = true;
                            enter(fresh, &mut game, &mut ui, &mut renderer, &mut clock);
                            changed = true;
                        }
                        _ => {
                            ui.screen = Screen::Sectors;
                            ui.sector = SectorId::clamped(profile.progress.unlocked_count() - 1);
                        }
                    }
                }
            }
            Screen::Sectors => {
                if escape {
                    home(&mut ui, &profile);
                }
                let at = ui.sector.index();
                if up {
                    ui.sector = SectorId::clamped(at.saturating_sub(1));
                }
                if down {
                    ui.sector = SectorId::clamped(at + 1);
                }
                if left {
                    ui.sector = SectorId::clamped(at.saturating_sub(4));
                }
                if right {
                    ui.sector = SectorId::clamped(at + 4);
                }
                if (moved || click)
                    && let Some(index) = ui::hover_sector(pointer)
                {
                    ui.sector = index;
                }
                if click && ui::back_rect().contains(pointer) {
                    home(&mut ui, &profile);
                }
                let play = click
                    && (ui::hover_sector(pointer).is_some() || ui::play_rect().contains(pointer));
                if (confirm || play)
                    && ui.sector.index() < profile.progress.unlocked_count()
                    && ui.screen == Screen::Sectors
                {
                    enter(
                        Game::start(ui.sector, Mode::Practice),
                        &mut game,
                        &mut ui,
                        &mut renderer,
                        &mut clock,
                    );
                    changed = true;
                }
            }
            Screen::Play => {
                if pause {
                    if terminal {
                        home(&mut ui, &profile);
                    } else {
                        ui.paused = !ui.paused;
                        ui.choice = 0;
                    }
                    pending_launch = false;
                    clock.reset();
                    changed = true;
                }
                if ui.screen == Screen::Play && (ui.paused || terminal) {
                    ui.choice = ui::step_menu(ui.choice, 0, up, down);
                    if (moved || click)
                        && let Some(row) = ui::hover_menu(pointer)
                    {
                        ui.choice = row;
                    }
                    if !changed
                        && (restart || confirm || (click && ui::hover_menu(pointer).is_some()))
                    {
                        let action = if restart { 1 } else { ui.choice };
                        match action {
                            0 if ui.paused => {
                                ui.paused = false;
                                clock.reset();
                            }
                            0 => {
                                ui.screen = Screen::Sectors;
                                ui.sector = game.sector();
                            }
                            1 => {
                                let fresh = if game.mode() == Mode::Journey {
                                    // Retrying restores the entry checkpoint; scores cannot be farmed.
                                    profile
                                        .progress
                                        .checkpoint()
                                        .map_or_else(Game::new, Game::resume)
                                } else {
                                    Game::start(game.sector(), Mode::Practice)
                                };
                                if fresh.mode() == Mode::Journey {
                                    profile.progress.begin(&fresh);
                                    dirty = true;
                                }
                                enter(fresh, &mut game, &mut ui, &mut renderer, &mut clock);
                            }
                            _ => home(&mut ui, &profile),
                        }
                        changed = true;
                    }
                } else if !changed && game.stage() == Stage::Cleared {
                    let next = confirm || (click && ui::next_rect().contains(pointer));
                    if next && game.stage_ticks() >= ADVANCE_DELAY_TICKS {
                        if game.mode() == Mode::Practice {
                            ui.screen = Screen::Sectors;
                            ui.sector = game.sector();
                        } else {
                            pending_launch = true;
                        }
                    }
                } else if !changed
                    && (confirm
                        || (click
                            && mouse.x >= LEFT
                            && mouse.x <= RIGHT
                            && mouse.y >= TOP
                            && mouse.y <= BOTTOM))
                {
                    pending_launch = true;
                }
            }
        }
        if changed {
            pending_launch = false;
        }
        let axis = input::merge_axis(
            (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)) as u8 as f32
                - (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) as u8 as f32,
            pad.axis,
        );
        if axis != 0.0 {
            mouse_control = false;
        }
        let mut input = Input {
            axis,
            target_x: mouse_control.then_some(mouse.x),
            launch: false,
        };
        if smoke && !flow {
            input.target_x = Some(
                game.balls()
                    .iter()
                    .find(|b| b.active)
                    .map_or(FIELD.center().x, |b| b.pos.x)
                    + (now as f32 * 0.7).sin() * 32.0,
            );
            pending_launch = matches!(game.stage(), Stage::Ready | Stage::Cleared)
                || game.balls().iter().any(|b| b.active && b.held);
        }
        let mut alpha = 1.0;
        if ui.screen == Screen::Play && !ui.paused && !terminal && !changed && !held {
            let frame = clock.advance(frame_seconds);
            alpha = frame.alpha;
            perf.dropped_ticks += frame.dropped;
            let mut events = Events::default();
            for _ in 0..frame.ticks {
                input.launch = pending_launch;
                pending_launch = false;
                let tick_start = get_time();
                let tick = game.step(input);
                perf.simulation((get_time() - tick_start) * 1000.0);
                renderer.record(&game, tick);
                events.merge(tick);
                if tick.clear {
                    profile.progress.finish(&game);
                    steam.cleared(&game, &profile.progress);
                    dirty = true;
                    ui.choice = 0;
                }
                if game.stage() == Stage::GameOver && game.mode() == Mode::Journey {
                    profile.progress.note_score(&game);
                    dirty = true;
                    ui.choice = 1;
                }
            }
            audio.play(events);
        } else {
            clock.reset();
        }
        if (ui.paused || ui.screen != Screen::Play) && profile.progress.note_score(&game) {
            dirty = true;
        }
        if dirty
            && now - last_save_attempt >= 5.0
            && (ui.screen != Screen::Play || ui.paused || game.stage() != Stage::Playing)
        {
            last_save_attempt = now;
            match path.as_ref().map(|p| profile.save(p)).transpose() {
                Ok(saved) => {
                    dirty = false;
                    // Without a path the notice reflects why saving is off.
                    ui.save_error &= saved.is_none();
                }
                Err(e) => {
                    ui.save_error = true;
                    diagnostics::error(format_args!("Could not save progress: {e}"));
                }
            }
        }
        steam.presence(ui.screen == Screen::Play, &game);
        let show_cursor = ui.device == Device::KeyboardMouse
            && (ui.screen != Screen::Play || ui.paused || game.stage() != Stage::Playing);
        if cursor_visible != show_cursor {
            show_mouse(show_cursor);
            cursor_visible = show_cursor;
        }
        perf.refresh(now, game.diagnostics().budget_exhausted);
        let draw_start = get_time();
        let actual_screen = ui.screen;
        let actual_pause = ui.paused;
        let measured = if perf_test { 3899 } else { 660 };
        let layouts = smoke && !flow && !perf_test && !effects;
        let layout_capture = if layouts && frames >= measured {
            smoke::layout(frames - measured, &mut ui)
        } else {
            None
        };
        if smoke && !flow && !perf_test {
            match frames {
                30 => ui.screen = Screen::Title,
                140 => ui.preview = preview(Stage::Ready),
                // Switching language rebuilds the glyph atlas mid-run.
                244 => {
                    ui.screen = Screen::Title;
                    renderer.set_locale(if language == Locale::De {
                        Locale::Ja
                    } else {
                        Locale::De
                    });
                }
                150 => ui.paused = true,
                160 => ui.preview = preview(Stage::GameOver),
                180 => ui.preview = preview(Stage::Cleared),
                190 => ui.screen = Screen::Sectors,
                250 | 260 | 270 | 280 | 290 => {
                    ui.device = Device::Gamepad;
                    match frames {
                        250 => ui.screen = Screen::Title,
                        260 => ui.paused = true,
                        270 => ui.screen = Screen::Sectors,
                        280 => ui.preview = preview(Stage::Cleared),
                        _ => ui.preview = preview(Stage::Ready),
                    }
                }
                _ => {}
            }
        }
        renderer.draw(
            &game,
            &ui,
            &profile,
            alpha,
            (stats || (smoke && frames == 170)).then_some(&perf),
        );
        if let Some((size, p)) = layout_capture {
            renderer.capture_at((&game, &ui, &profile), size, &p);
        }
        ui.preview = None;
        ui.screen = actual_screen;
        ui.paused = actual_pause;
        perf.draw((get_time() - draw_start) * 1000.0);
        if smoke && !flow && !perf_test {
            let capture = match frames {
                30 => Some("target/attract.png"),
                120 => Some("target/smoke-test.png"),
                140 => Some("target/ready.png"),
                150 => Some("target/paused.png"),
                160 => Some("target/game-over.png"),
                170 => Some("target/stats.png"),
                180 => Some("target/clear.png"),
                190 => Some("target/sectors.png"),
                230 => Some("target/resized.png"),
                244 => Some("target/locale-switch.png"),
                250 => Some("target/attract-pad.png"),
                260 => Some("target/paused-pad.png"),
                270 => Some("target/sectors-pad.png"),
                280 => Some("target/clear-pad.png"),
                290 => Some("target/ready-pad.png"),
                _ => None,
            };
            if let Some(p) = capture {
                renderer.capture(p);
            }
            if frames == 244 {
                renderer.set_locale(language);
            }
            if frames == 200 {
                request_new_screen_size(800.0, 600.0);
            }
            if frames == 240 {
                request_new_screen_size(960.0, 900.0);
            }
        }
        if perf_test && frames >= 300 {
            trace.push(frame_seconds, focus.focused);
        }
        if smoke && !flow && frames == measured {
            println!(
                "Render smoke: {} frames, score {}, collision caps {}, sounds loaded {}/17",
                frames + 1,
                game.score(),
                game.diagnostics().budget_exhausted,
                audio.loaded()
            );
            for line in &perf.lines {
                println!("{line}");
            }
        }
        if smoke && !flow && frames >= measured + if layouts { smoke::LAYOUT_FRAMES } else { 0 } {
            break;
        }
        if effects && !perf_test && frames == 330 {
            renderer.capture("target/effects.png");
        }
        if flow && matches!(frames, 35 | 45) {
            renderer.capture(if frames == 35 {
                "target/anchor.png"
            } else {
                "target/relay.png"
            });
        }
        if flow && frames >= 58 {
            break;
        }
        frames += 1;
        next_frame().await;
    }
    if perf_test {
        trace.report();
    }
    if !smoke {
        profile.progress.note_score(&game);
        if let Some(p) = &path
            && let Err(e) = profile.save(p)
        {
            diagnostics::error(format_args!("Could not save progress: {e}"));
        }
        diagnostics::info("Quit");
    }
}
