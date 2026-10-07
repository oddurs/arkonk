//! Graphical integration driver: inputs go through the real application handlers.
use crate::{
    display::MIN_PHYSICAL,
    input::{Dir, Presses, pad_controls},
    ui::{Controls, Screen, Ui},
};
use ark::{
    clock::TICK_HZ,
    field::{BALL_RADIUS as RADIUS, CELLS, Cell, GRID_X, GRID_Y, PADDLE_Y, cell_rect},
    game::*,
    geom::V2,
    profile::Profile,
    sectors::SectorId,
};
use macroquad::prelude::{request_new_screen_size, screen_dpi_scale, screen_height, screen_width};
pub fn flow(frame: u32, game: &mut Game, ui: &Ui, profile: &Profile) -> Controls {
    let mut keys = Controls::default();
    let none = Presses::default();
    let (south, east, west, start) = (
        Presses {
            south: true,
            ..none
        },
        Presses { east: true, ..none },
        Presses { west: true, ..none },
        Presses {
            start: true,
            ..none
        },
    );
    match frame {
        1 | 2 | 5 | 9 | 10 | 14 | 17 | 18 | 19 | 22 | 24 | 25 | 28 => keys.confirm = true,
        7 | 8 | 12 | 13 | 15 | 16 => keys.down = true,
        6 | 11 | 23 | 30 => {
            keys.escape = true;
            keys.pause = true;
        }
        3 | 20 => {
            assert_eq!(game.phase, Phase::Playing);
            assert!(
                game.mode
                    == if frame == 3 {
                        Mode::Journey
                    } else {
                        Mode::Practice
                    }
            );
            game.bricks.fill(0);
            game.remaining = 0;
        }
        4 | 21 => {
            assert_eq!(game.phase, Phase::Cleared);
            assert!(profile.unlocked >= 2);
            assert_eq!(profile.checkpoint.unwrap().sector.index(), 1);
            game.phase_ticks = TICK_HZ / 2;
        }
        26 => keys.focus_lost = true,
        27 => assert!(ui.paused),
        29 => {
            assert!(!ui.paused);
            assert_eq!(game.phase, Phase::Playing);
        }
        31 => keys.restart = true,
        32 => {
            assert!(ui.screen == Screen::Play && !ui.paused);
            assert_eq!(game.phase, Phase::Ready);
            assert_eq!(game.sector.index(), 1);
            assert_eq!(game.score, profile.checkpoint.unwrap().score);
            assert_eq!(profile.records[0].medals, 7);
            assert_eq!(profile.records[1].medals, 7);
            println!(
                "UI flow passed: new journey, clear, checkpoint, home, continue, practice, focus pause, resume, retry"
            );
        }
        33 | 39 => keys.confirm = true,
        34 => {
            game.apply_power(Power::Anchor);
            game.apply_power(Power::Phase);
            game.balls[0].pos = V2::new(game.paddle_x + 25.0, PADDLE_Y - RADIUS - 0.5);
            game.balls[0].velocity = V2::new(0.0, 400.0);
        }
        35 => {
            assert!(game.balls[0].held);
            assert_eq!(game.anchor_charges, 2);
            game.apply_power(Power::Wide);
            game.apply_power(Power::Slow);
            game.apply_power(Power::Multi);
        }
        36 | 38 => keys.pause = true,
        37 => {
            assert!(ui.paused);
            assert!(game.balls[0].held);
        }
        40 => {
            assert!(!game.balls[0].held);
            assert_eq!(game.balls[0].phase_hits, 3);
            *game = Game::start(SectorId::clamped(8), Mode::Practice);
            game.step(&Input {
                launch: true,
                ..Input::default()
            });
            game.apply_power(Power::Phase);
            let core = Cell::all().find(|c| game.cores[c.index()]).unwrap();
            let r = cell_rect(core);
            game.balls[0].pos = V2::new(r.x - RADIUS - 1.0, r.y + r.h / 2.0);
            game.balls[0].velocity = V2::new(500.0, 0.0);
        }
        45 => {
            assert!(game.relay_flash.iter().any(|&f| f > 0));
            assert_eq!(game.collision_caps, 0);
        }
        50 => println!(
            "Mechanics flow passed: Anchor hold, combined powers, pause/resume, release, Phase, relay ignition"
        ),
        // Gamepad buttons, mapped exactly as a connected controller's would be.
        51 => keys = pad_controls(start, None, false),
        52 => {
            assert!(ui.paused);
            keys = pad_controls(east, None, true);
        }
        53 => {
            assert!(!ui.paused);
            keys = pad_controls(start, None, false);
        }
        54 => {
            assert!(ui.paused);
            keys = pad_controls(none, Some(Dir::Down), true);
        }
        55 => {
            assert_eq!(ui.choice, 1);
            keys = pad_controls(south, None, true);
        }
        56 | 58 => {
            assert!(ui.screen == Screen::Play && !ui.paused);
            assert_eq!(game.phase, Phase::Ready);
            assert!(game.sector.index() == 8 && game.mode == Mode::Practice);
            if frame == 56 {
                keys = pad_controls(start, None, false);
            } else {
                println!("Gamepad flow passed: Start pause, B resume, d-pad, A retry, X retry");
            }
        }
        57 => {
            assert!(ui.paused);
            keys = pad_controls(west, None, true);
        }
        _ => {}
    }
    keys
}

/// Fill every visual pool and ignite a full relay board repeatedly. This is a
/// rendering stress fixture, deliberately denser than any shipped sector.
pub fn effects(game: &mut Game, frame: u32) {
    if frame.is_multiple_of(90) {
        *game = Game::start(SectorId::clamped(10), Mode::Practice);
        game.bricks.fill(1);
        game.cores.fill(true);
        game.remaining = CELLS;
        game.step(&Input {
            launch: true,
            ..Input::default()
        });
        for power in [
            Power::Multi,
            Power::Anchor,
            Power::Wide,
            Power::Slow,
            Power::Phase,
        ] {
            game.apply_power(power);
        }
        for (i, ball) in game.balls.iter_mut().enumerate() {
            ball.pos = V2::new(200.0 + i as f32 * 240.0, 445.0);
            ball.previous = ball.pos;
            ball.velocity = V2::new(80.0, -900.0);
        }
    }
    for (i, p) in game.particles.iter_mut().enumerate() {
        if p.life < 0.2 {
            *p = Particle {
                pos: V2::new(
                    GRID_X + (i % 60) as f32 * 12.0,
                    GRID_Y + (i % 17) as f32 * 17.0,
                ),
                velocity: V2::new(20.0, 80.0),
                life: 0.8,
                hue: i % 7,
            };
        }
    }
    for (i, d) in game.drops.iter_mut().enumerate() {
        if !d.active {
            *d = Drop {
                pos: V2::new(125.0 + i as f32 * 62.0, 470.0),
                power: [
                    Power::Wide,
                    Power::Slow,
                    Power::Multi,
                    Power::Anchor,
                    Power::Phase,
                ][i % 5],
                active: true,
            };
        }
    }
}

/// Physical sizes rendered offscreen after the main smoke run: Steam Deck,
/// 1080p, 1440p, ultrawide, and 4:3.
pub const LAYOUTS: [(&str, u32, u32); 5] = [
    ("deck", 1280, 800),
    ("1080p", 1920, 1080),
    ("1440p", 2560, 1440),
    ("ultrawide", 3440, 1440),
    ("4x3", 1024, 768),
];
/// Frames after the main run: one per layout and screen, then a too-small
/// window request that the minimum size must refuse.
pub const LAYOUT_FRAMES: u32 = LAYOUTS.len() as u32 * 3 + 40;

/// Shows play, title, and sector screens in turn; returns the layout and
/// capture path for this frame.
pub fn layout(frame: u32, ui: &mut Ui) -> Option<((u32, u32), String)> {
    let index = (frame / 3) as usize;
    let Some(&(name, w, h)) = LAYOUTS.get(index) else {
        let step = frame - LAYOUTS.len() as u32 * 3;
        if step == 0 {
            request_new_screen_size(10.0, 10.0);
        }
        if step == 39 {
            let dpi = screen_dpi_scale();
            let (pw, ph) = (screen_width() * dpi, screen_height() * dpi);
            println!("Minimum window: requested 10x10, got {pw}x{ph} physical");
            assert!(
                pw >= MIN_PHYSICAL.0 - 1.0 && ph >= MIN_PHYSICAL.1 - 1.0,
                "window shrank below its minimum: {pw}x{ph}"
            );
        }
        return None;
    };
    let (screen, suffix) = [
        (Screen::Play, ""),
        (Screen::Title, "-title"),
        (Screen::Sectors, "-sectors"),
    ][(frame % 3) as usize];
    ui.screen = screen;
    Some(((w, h), format!("target/layout-{name}{suffix}.png")))
}
