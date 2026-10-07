//! Graphical integration driver: inputs go through the real application handlers.
use crate::ui::{Controls, Screen, Ui};
use arkonk::{game::*, physics::V2, profile::Profile};
pub fn flow(frame: u32, game: &mut Game, ui: &Ui, profile: &Profile) -> Controls {
    let mut keys = Controls::default();
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
            assert_eq!(profile.checkpoint.unwrap().level, 1);
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
            assert_eq!(game.level, 1);
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
            *game = Game::at(8, Mode::Practice);
            game.step(&Input {
                launch: true,
                ..Input::default()
            });
            game.apply_power(Power::Phase);
            let core = game.cores.iter().position(|&b| b).unwrap();
            let r = Game::brick_rect(core);
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
        _ => {}
    }
    keys
}

/// Fill every visual pool and ignite a full relay board repeatedly. This is a
/// rendering stress fixture, deliberately denser than any shipped sector.
pub fn effects(game: &mut Game, frame: u32) {
    if frame.is_multiple_of(90) {
        *game = Game::at(10, Mode::Practice);
        game.bricks.fill(1);
        game.cores.fill(true);
        game.remaining = ROWS * COLS;
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
