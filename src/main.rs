mod audio;
mod crt;
mod perf;
mod pixel_font;
mod render;
mod smoke;
mod ui;

use arkonk::{game::*, profile::Profile, timing::FixedClock};
use audio::Audio;
use macroquad::prelude::*;
use perf::Perf;
use render::{Renderer, View};
use std::path::PathBuf;
use ui::{Controls, Screen, Ui};

fn config() -> Conf {
    Conf {
        window_title: "ARKONK".into(),
        window_width: 960,
        window_height: 900,
        high_dpi: true,
        sample_count: 1,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
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
#[derive(Default)]
struct Focus {
    lost: bool,
}
impl miniquad::EventHandler for Focus {
    fn update(&mut self) {}
    fn draw(&mut self) {}
    fn window_minimized_event(&mut self) {
        self.lost = true;
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
#[macroquad::main(config)]
async fn main() {
    prevent_quit();
    let flow = std::env::args().any(|a| a == "--flow-test");
    let smoke = flow || std::env::args().any(|a| a == "--smoke-test");
    let path = if smoke { None } else { profile_path() };
    let mut profile = path
        .as_ref()
        .map_or_else(Profile::default, |p| Profile::load(p));
    if std::env::args().any(|a| a == "--no-crt") {
        profile.crt = false;
    }
    let mut dirty = false;
    let mut last_save_attempt = -5.0;
    let mut game = Game::new();
    let mut renderer = Renderer::new();
    let mut audio = Audio::new().await;
    let mut perf = Perf::new();
    let mut ui = Ui::default();
    home(&mut ui, &profile);
    if smoke && !flow {
        ui.screen = Screen::Play;
    }
    let mut clock = FixedClock::default();
    let mut stats = false;
    let mut fullscreen = false;
    let mut last_frame = get_time();
    let mut last_mouse = View::new().mouse(profile.crt);
    let mut mouse_control = false;
    let mut pending_launch = false;
    let mut frames = 0;
    let subscriber = macroquad::input::utils::register_input_subscriber();
    let mut focus = Focus::default();
    loop {
        let now = get_time();
        let frame_seconds = if flow {
            1.0 / 60.0
        } else {
            (now - last_frame).max(0.0)
        };
        last_frame = now;
        perf.frame(frame_seconds * 1000.0);
        macroquad::input::utils::repeat_all_miniquad_input(&mut focus, subscriber);
        if is_key_pressed(KeyCode::Q) || is_quit_requested() {
            break;
        }
        if is_key_pressed(KeyCode::M) {
            profile.muted = !profile.muted;
            dirty = true;
        }
        if is_key_pressed(KeyCode::C) {
            profile.crt = !profile.crt;
            last_mouse = View::new().mouse(profile.crt);
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
        let mouse = View::new().mouse(profile.crt);
        let moved =
            !flow && ((mouse.x - last_mouse.x).abs() > 0.5 || (mouse.y - last_mouse.y).abs() > 0.5);
        if moved {
            mouse_control = true;
        }
        last_mouse = mouse;
        let pointer = vec2(mouse.x, mouse.y);
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
            Controls::read()
        };
        focus.lost |= focus_lost;
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
                if up {
                    ui.choice = (ui.choice + 2) % 3;
                }
                if down {
                    ui.choice = (ui.choice + 1) % 3;
                }
                if (moved || click)
                    && let Some(row) = ui::hover_menu(pointer)
                {
                    ui.choice = row;
                }
                if confirm || (click && ui::hover_menu(pointer).is_some()) {
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
                if click && Rect::new(105.0, 100.0, 120.0, 44.0).contains(pointer) {
                    home(&mut ui, &profile);
                }
                if (confirm || (click && ui::hover_sector(pointer).is_some()))
                    && ui.sector < profile.unlocked
                    && ui.screen == Screen::Sectors
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
                    if up {
                        ui.choice = (ui.choice + 2) % 3;
                    }
                    if down {
                        ui.choice = (ui.choice + 1) % 3;
                    }
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
                    let next = confirm
                        || (click && Rect::new(280.0, 622.0, 400.0, 44.0).contains(pointer));
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
        let axis = (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)) as u8 as f32
            - (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) as u8 as f32;
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
            pending_launch = matches!(game.phase, Phase::Ready | Phase::Cleared);
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
        show_mouse(ui.screen != Screen::Play || ui.paused || game.phase != Phase::Playing);
        perf.refresh(now, game.collision_caps);
        let draw_start = get_time();
        let actual_phase = game.phase;
        let actual_screen = ui.screen;
        let actual_pause = ui.paused;
        if smoke && !flow && profile.crt {
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
        if smoke && !flow {
            let capture = match frames {
                30 if profile.crt => Some("target/attract.png"),
                120 => Some(if profile.crt {
                    "target/smoke-test.png"
                } else {
                    "target/smoke-plain.png"
                }),
                150 if profile.crt => Some("target/paused.png"),
                160 if profile.crt => Some("target/game-over.png"),
                170 if profile.crt => Some("target/stats.png"),
                180 if profile.crt => Some("target/clear.png"),
                190 if profile.crt => Some("target/sectors.png"),
                230 if profile.crt => Some("target/resized.png"),
                _ => None,
            };
            if let Some(p) = capture {
                get_screen_data().export_png(p);
            }
            if profile.crt && frames == 200 {
                request_new_screen_size(800.0, 600.0);
            }
            if profile.crt && frames == 240 {
                request_new_screen_size(960.0, 900.0);
            }
            if frames >= 660 {
                println!(
                    "Render smoke: 661 frames, CRT {}, score {}, collision caps {}, sounds loaded {}/14",
                    profile.crt,
                    game.score,
                    game.collision_caps,
                    audio.loaded()
                );
                for line in &perf.lines {
                    println!("{line}");
                }
                break;
            }
        }
        if flow && frames >= 32 {
            break;
        }
        if smoke && frames == 300 {
            perf = Perf::new();
        }
        frames += 1;
        next_frame().await;
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
