//! Graphical integration driver: inputs go through the real application handlers.
use crate::ui::{Controls, Screen, Ui};
use arkonk::{game::*, profile::Profile};
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
        _ => {}
    }
    keys
}
