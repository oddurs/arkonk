mod audio;
mod input;
mod perf;
mod pixel_font;
mod render;
mod smoke;
mod steam;
mod ui;

use arkonk::{game::*, presence::Presence, profile::Profile, timing::FixedClock};
use audio::Audio;
use input::{Device, Gamepads};
use macroquad::prelude::*;
use perf::Perf;
use render::{Renderer, View};
use std::{path::PathBuf, time::Instant};
use steam::Steam;
use ui::{Controls, Screen, Ui};

fn config() -> Conf {
    Conf {
        window_title: "ARKONK".into(),
        window_width: 960,
        window_height: 900,
        high_dpi: true,
        fullscreen: std::env::args().any(|a| a == "--fullscreen"),
        sample_count: 1,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            #[cfg(target_os = "macos")]
            apple_gfx_api: if std::env::args().any(|a| a == "--opengl") {
                miniquad::conf::AppleGfxApi::OpenGl
            } else {
                miniquad::conf::AppleGfxApi::Metal
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
fn profile_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    let base =
        std::env::var_os("HOME").map(|p| PathBuf::from(p).join("Library/Application Support"));
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share")));
    base.map(|p| p.join("arkonk/progress.txt"))
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
    ui.choice = usize::from(profile.checkpoint.is_none());
}
fn main() {
    // Before any window exists: Steam is launching another copy of the game.
    if steam::restart_through_steam() {
        return;
    }
    macroquad::Window::from_config(config(), run());
}
async fn run() {
    prevent_quit();
    let flow = std::env::args().any(|a| a == "--flow-test");
    let perf_test = std::env::args().any(|a| a == "--perf-test");
    let effects = std::env::args().any(|a| a == "--effects-test");
    let smoke = perf_test || effects || flow || std::env::args().any(|a| a == "--smoke-test");
    let path = if smoke { None } else { profile_path() };
    let mut profile = path
        .as_ref()
        .map_or_else(Profile::default, |p| Profile::load(p));
    // Test runs must not unlock achievements or show presence.
    let mut steam = if smoke {
        Steam::off()
    } else {
        Steam::init(&profile)
    };
    let mut dirty = false;
    let mut last_save_attempt = -5.0;
    let mut game = Game::new();
    let mut renderer = Renderer::new();
    let mut audio = Audio::new().await;
    let mut perf = Perf::new();
    let mut trace = perf::FrameTrace::new(perf_test);
    let mut ui = Ui::default();
    home(&mut ui, &profile);
    if smoke && !flow {
        ui.screen = Screen::Play;
    }
    let mut clock = FixedClock::default();
    let mut stats = false;
    let mut fullscreen = std::env::args().any(|a| a == "--fullscreen");
    let mut last_frame = Instant::now();
    let mut last_mouse = View::new().mouse();
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
            profile.muted = !profile.muted;
            dirty = true;
        }
        if is_key_pressed(KeyCode::LeftBracket) {
            profile.volume = profile.volume.saturating_sub(1);
            dirty = true;
        }
        if is_key_pressed(KeyCode::RightBracket) {
            profile.volume = (profile.volume + 1).min(10);
            dirty = true;
        }
        if is_key_pressed(KeyCode::F3) {
            stats = !stats;
        }
        if is_key_pressed(KeyCode::F) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }
        audio.muted = profile.muted;
        audio.volume = f32::from(profile.volume) / 10.0;
        let mouse = View::new().mouse();
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
            ui.paused || matches!(game.phase, Phase::GameOver | Phase::Victory),
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
        let terminal = matches!(game.phase, Phase::GameOver | Phase::Victory);
        if (!smoke || flow)
            && ui.screen == Screen::Play
            && !terminal
            && (focus.lost || (frame_seconds > 0.25 && game.phase == Phase::Playing))
        {
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
                let first = usize::from(profile.checkpoint.is_none());
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
                            if let Some(c) = profile.checkpoint {
                                enter(c.game(), &mut game, &mut ui, &mut renderer, &mut clock);
                                changed = true;
                            }
                        }
                        1 => {
                            let fresh = Game::new();
                            profile.begin(&fresh);
                            dirty = true;
                            enter(fresh, &mut game, &mut ui, &mut renderer, &mut clock);
                            changed = true;
                        }
                        _ => {
                            ui.screen = Screen::Sectors;
                            ui.sector = profile.unlocked - 1;
                        }
                    }
                }
            }
            Screen::Sectors => {
                if escape {
                    home(&mut ui, &profile);
                }
                if up {
                    ui.sector = ui.sector.saturating_sub(1);
                }
                if down {
                    ui.sector = (ui.sector + 1).min(LEVEL_COUNT - 1);
                }
                if left {
                    ui.sector = ui.sector.saturating_sub(4);
                }
                if right {
                    ui.sector = (ui.sector + 4).min(LEVEL_COUNT - 1);
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
                if (confirm || play) && ui.sector < profile.unlocked && ui.screen == Screen::Sectors
                {
                    enter(
                        Game::at(ui.sector, Mode::Practice),
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
                                ui.sector = game.level;
                            }
                            1 => {
                                let fresh = if game.mode == Mode::Journey {
                                    // Retrying restores the entry checkpoint; scores cannot be farmed.
                                    profile.checkpoint.map_or_else(Game::new, |c| c.game())
                                } else {
                                    Game::at(game.level, Mode::Practice)
                                };
                                if fresh.mode == Mode::Journey {
                                    profile.begin(&fresh);
                                    dirty = true;
                                }
                                enter(fresh, &mut game, &mut ui, &mut renderer, &mut clock);
                            }
                            _ => home(&mut ui, &profile),
                        }
                        changed = true;
                    }
                } else if !changed && game.phase == Phase::Cleared {
                    let next = confirm || (click && ui::next_rect().contains(pointer));
                    if next && game.phase_ticks >= TICK_HZ / 2 {
                        if game.mode == Mode::Practice {
                            ui.screen = Screen::Sectors;
                            ui.sector = game.level;
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
            mouse_x: mouse_control.then_some(mouse.x),
            launch: false,
        };
        if smoke && !flow {
            input.mouse_x = Some(
                game.balls
                    .iter()
                    .find(|b| b.active)
                    .map_or(WIDTH / 2.0, |b| b.pos.x)
                    + (now as f32 * 0.7).sin() * 32.0,
            );
            pending_launch = matches!(game.phase, Phase::Ready | Phase::Cleared)
                || game.balls.iter().any(|b| b.active && b.held);
        }
        let mut alpha = 1.0;
        if ui.screen == Screen::Play && !ui.paused && !terminal && !changed {
            let frame = clock.advance(frame_seconds);
            alpha = frame.alpha;
            perf.dropped_ticks += frame.dropped;
            let mut events = Events::default();
            for _ in 0..frame.steps {
                input.launch = pending_launch;
                pending_launch = false;
                let tick_start = get_time();
                game.step(&input);
                perf.simulation((get_time() - tick_start) * 1000.0);
                renderer.record(&game);
                events.merge(game.events);
                if game.events.clear {
                    profile.finish(&game);
                    steam.cleared(&game, &profile);
                    dirty = true;
                    ui.choice = 0;
                }
                if game.phase == Phase::GameOver && game.mode == Mode::Journey {
                    profile.best_score = profile.best_score.max(game.score);
                    dirty = true;
                    ui.choice = 1;
                }
            }
            audio.play(events);
        } else {
            clock.reset();
        }
        if game.mode == Mode::Journey
            && (ui.paused || ui.screen != Screen::Play)
            && game.score > profile.best_score
        {
            profile.best_score = game.score;
            dirty = true;
        }
        if dirty
            && now - last_save_attempt >= 5.0
            && (ui.screen != Screen::Play || ui.paused || game.phase != Phase::Playing)
        {
            last_save_attempt = now;
            match path.as_ref().map(|p| profile.save(p)).transpose() {
                Ok(_) => {
                    dirty = false;
                    ui.save_error = false;
                }
                Err(e) => {
                    ui.save_error = true;
                    eprintln!("Could not save progress: {e}");
                }
            }
        }
        steam.presence(Presence::of(ui.screen == Screen::Play, &game));
        let show_cursor = ui.device == Device::KeyboardMouse
            && (ui.screen != Screen::Play || ui.paused || game.phase != Phase::Playing);
        if cursor_visible != show_cursor {
            show_mouse(show_cursor);
            cursor_visible = show_cursor;
        }
        perf.refresh(now, game.collision_caps);
        let draw_start = get_time();
        let actual_phase = game.phase;
        let actual_screen = ui.screen;
        let actual_pause = ui.paused;
        if smoke && !flow && !perf_test {
            match frames {
                30 => ui.screen = Screen::Title,
                150 => ui.paused = true,
                160 => game.phase = Phase::GameOver,
                180 => {
                    game.phase = Phase::Cleared;
                    game.summary.medals = 7;
                    game.summary.ticks = 18240;
                    game.summary.bonus = 2000;
                }
                190 => ui.screen = Screen::Sectors,
                250 | 260 | 270 | 280 | 290 => {
                    ui.device = Device::Gamepad;
                    match frames {
                        250 => ui.screen = Screen::Title,
                        260 => ui.paused = true,
                        270 => ui.screen = Screen::Sectors,
                        280 => game.phase = Phase::Cleared,
                        _ => game.phase = Phase::Ready,
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
        game.phase = actual_phase;
        ui.screen = actual_screen;
        ui.paused = actual_pause;
        perf.draw((get_time() - draw_start) * 1000.0);
        if smoke && !flow && !perf_test {
            let capture = match frames {
                30 => Some("target/attract.png"),
                120 => Some("target/smoke-test.png"),
                150 => Some("target/paused.png"),
                160 => Some("target/game-over.png"),
                170 => Some("target/stats.png"),
                180 => Some("target/clear.png"),
                190 => Some("target/sectors.png"),
                230 => Some("target/resized.png"),
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
        if smoke && !flow && frames >= if perf_test { 3899 } else { 660 } {
            println!(
                "Render smoke: {} frames, score {}, collision caps {}, sounds loaded {}/17",
                frames + 1,
                game.score,
                game.collision_caps,
                audio.loaded()
            );
            for line in &perf.lines {
                println!("{line}");
            }
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
        if game.mode == Mode::Journey {
            profile.best_score = profile.best_score.max(game.score);
        }
        if let Some(p) = &path
            && let Err(e) = profile.save(p)
        {
            eprintln!("Could not save progress: {e}");
        }
    }
}
