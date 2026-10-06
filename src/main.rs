mod audio;
mod perf;
mod render;

use arkonk::game::*;
use audio::Audio;
use macroquad::prelude::*;
use perf::Perf;
use render::{Renderer, View};
use std::{fs, path::PathBuf};

fn config() -> Conf {
    Conf {
        window_title: "ARKONK / Rust arcade".into(),
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

fn score_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    let base =
        std::env::var_os("HOME").map(|p| PathBuf::from(p).join("Library/Application Support"));
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share")));
    base.map(|p| p.join("arkonk/best.txt"))
}

fn save_score(path: &Option<PathBuf>, score: u32) {
    if let Some(path) = path {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Err(error) = fs::write(path, score.to_string()) {
            eprintln!("Could not save best score: {error}");
        }
    }
}

#[macroquad::main(config)]
async fn main() {
    prevent_quit();
    let smoke = std::env::args().any(|a| a == "--smoke-test");
    let path = if smoke { None } else { score_path() };
    let mut best = path
        .as_ref()
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);
    let mut saved_best = best;
    let mut game = Game::new();
    let mut renderer = Renderer::new();
    let mut audio = Audio::new().await;
    let mut perf = Perf::new();
    let mut started = smoke;
    let mut paused = false;
    let mut stats = false;
    let mut fullscreen = false;
    let mut accumulator = 0.0_f64;
    let mut last_frame = get_time();
    let mut last_mouse = View::new().mouse();
    let mut mouse_control = false;
    let mut pending_launch = false;
    let mut frames = 0;
    loop {
        let now = get_time();
        let frame_seconds = (now - last_frame).max(0.0);
        last_frame = now;
        perf.frame(frame_seconds * 1000.0);
        if is_key_pressed(KeyCode::Q) || is_quit_requested() {
            break;
        }
        if is_key_pressed(KeyCode::M) {
            audio.muted = !audio.muted;
        }
        if is_key_pressed(KeyCode::F3) {
            stats = !stats;
        }
        if is_key_pressed(KeyCode::F) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }
        if is_key_pressed(KeyCode::R)
            || (started
                && matches!(game.phase, Phase::GameOver | Phase::Victory)
                && is_key_pressed(KeyCode::Enter))
        {
            game = Game::new();
            renderer = Renderer::new();
            started = true;
            paused = false;
            pending_launch = false;
            accumulator = 0.0;
        }
        let launch = is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter)
            || is_mouse_button_pressed(MouseButton::Left);
        let pause_key = is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape);
        if started && (pause_key || (paused && launch)) {
            paused = !paused;
            pending_launch = false;
            accumulator = 0.0;
        } else if launch {
            started = true;
            pending_launch = true;
        }
        let mouse = View::new().mouse();
        if (mouse.x - last_mouse.x).abs() > 0.5 || (mouse.y - last_mouse.y).abs() > 0.5 {
            mouse_control = true;
        }
        last_mouse = mouse;
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
        if smoke {
            input.mouse_x = Some(
                game.balls
                    .iter()
                    .find(|b| b.active)
                    .map_or(WIDTH / 2.0, |b| b.pos.x),
            );
            pending_launch = game.phase == Phase::Ready;
        }
        if started && !paused {
            accumulator += frame_seconds;
            // Bound catch-up to eight ticks; report discarded time explicitly.
            let budget = f64::from(DT) * 8.0;
            if accumulator > budget {
                perf.dropped_ticks += ((accumulator - budget) / f64::from(DT)).floor() as u64;
                accumulator = budget;
            }
            while accumulator >= f64::from(DT) {
                input.launch = pending_launch;
                pending_launch = false;
                let tick_start = get_time();
                game.step(&input);
                perf.simulation((get_time() - tick_start) * 1000.0);
                renderer.record(&game);
                audio.play(game.events);
                accumulator -= f64::from(DT);
            }
        } else {
            accumulator = 0.0;
        }
        best = best.max(game.score);
        if !smoke && matches!(game.phase, Phase::GameOver | Phase::Victory) && best > saved_best {
            save_score(&path, best);
            saved_best = best;
        }
        perf.refresh(now, game.collision_caps);
        let alpha = if paused || !started {
            1.0
        } else {
            (accumulator / f64::from(DT)) as f32
        };
        let draw_start = get_time();
        renderer.draw(
            &game,
            alpha,
            started,
            paused,
            best,
            audio.muted,
            stats.then_some(&perf),
        );
        perf.draw((get_time() - draw_start) * 1000.0);
        if smoke && frames == 120 {
            get_screen_data().export_png("target/smoke-test.png");
        }
        if smoke && frames >= 180 {
            println!(
                "Render smoke test: 181 frames, score {}, collision caps {}, sounds loaded {}/7",
                game.score,
                game.collision_caps,
                audio.loaded()
            );
            for line in &perf.lines {
                println!("{line}");
            }
            break;
        }
        frames += 1;
        next_frame().await;
    }
    if best > saved_best {
        save_score(&path, best);
    }
}
