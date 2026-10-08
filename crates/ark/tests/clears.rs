//! Every sector can be cleared, and its par time comes from clearing it.
//!
//! The benchmark's autopilot plays each sector in Practice: the paddle
//! follows the first live ball, offset by a slow 36-unit sine wave that
//! varies the return angle, and serve is always pressed. It runs three
//! times per sector with the wave's phase a third of a period apart.
//!
//! Each run must clear within [`BUDGET`] of play without losing the game.
//! Par (the Swift medal's target) is the median of the three clear times ×
//! 1.5, rounded up to the next 5 seconds; see `docs/journey.md`.
//!
//! Clear times are exact only where the replay goldens are recorded, and a
//! run can fall into a rally that never meets the last brick, which a
//! different rounding of `sin` can cause or cure. So that check runs there.
//! Everywhere, each sector must clear in at least one of six runs, a sixth
//! of the wave apart: no sector is impossible.
//!
//! `cargo test -p ark --test clears -- --nocapture` prints the table.
use ark::{
    Game, Input, Mode, Stage,
    clock::TICK_HZ,
    field::FIELD,
    sectors::{SECTOR_COUNT, SectorId},
};

/// The most play a clear may take: ten minutes.
const BUDGET: u32 = 10 * 60 * TICK_HZ;
/// The autopilot's steering wave, per tick, as in `benches/tick.rs`.
const WAVE: f32 = 0.003;
/// A sixth of the wave's period, in ticks.
const SIXTH: u32 = 349;

/// Plays `sector` with the autopilot, its wave started `shift` ticks in.
/// The play time it took to clear, or `None` when it did not.
fn clear(sector: SectorId, shift: u32) -> Option<u32> {
    let mut game = Game::start(sector, Mode::Practice);
    let mut tick = shift;
    while game.sector_ticks() <= BUDGET {
        let x = game
            .balls()
            .iter()
            .find(|b| b.active)
            .map_or(FIELD.center().x, |b| b.pos.x)
            + (tick as f32 * WAVE).sin() * 36.0;
        game.step(Input {
            target_x: Some(x),
            launch: true,
            ..Input::default()
        });
        tick += 1;
        match game.stage() {
            Stage::Cleared => return Some(game.sector_ticks()),
            Stage::GameOver => return None,
            _ => {}
        }
    }
    None
}

/// Par for three clear times: the median × 1.5, rounded up to 5 s.
fn par(mut ticks: [u32; 3]) -> u32 {
    ticks.sort_unstable();
    (ticks[1] * 3).div_ceil(2 * TICK_HZ * 5) * 5
}

/// Every sector's three clear times, failing on any that does not clear.
fn clears() -> Vec<[u32; 3]> {
    let times: Vec<_> = SectorId::all()
        .map(|id| [0, 2 * SIXTH, 4 * SIXTH].map(|shift| clear(id, shift)))
        .collect();
    for (id, runs) in SectorId::all().zip(&times) {
        let seconds = runs.map(|r| r.map(|t| t / TICK_HZ));
        println!(
            "{:2} {:<14} {:?}  par {}",
            id.index() + 1,
            id.sector().slug,
            seconds,
            id.sector().par_seconds
        );
    }
    let failed: Vec<_> = SectorId::all()
        .zip(&times)
        .filter(|(_, runs)| runs.iter().any(Option::is_none))
        .map(|(id, _)| id.sector().slug)
        .collect();
    assert!(
        failed.is_empty(),
        "not cleared by the autopilot: {failed:?}"
    );
    times
        .iter()
        .map(|runs| runs.map(Option::unwrap_or_default))
        .collect()
}

#[test]
fn the_autopilot_clears_every_sector() {
    let impossible: Vec<_> = SectorId::all()
        .filter(|&id| (0..6).all(|k| clear(id, k * SIXTH).is_none()))
        .map(|id| id.sector().slug)
        .collect();
    assert!(impossible.is_empty(), "never cleared: {impossible:?}");
    assert_eq!(SectorId::all().count(), SECTOR_COUNT);
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64", debug_assertions)),
    ignore = "clear times are exact where the replay goldens are recorded"
)]
fn par_times_follow_the_autopilot() {
    let wrong: Vec<_> = SectorId::all()
        .zip(clears())
        .filter(|(id, runs)| id.sector().par_seconds != par(*runs))
        .map(|(id, runs)| (id.sector().slug, par(runs)))
        .collect();
    assert!(wrong.is_empty(), "par should be: {wrong:?}");
}

#[test]
fn par_rounds_the_median_up() {
    assert_eq!(par([10 * TICK_HZ, 40 * TICK_HZ, 400 * TICK_HZ]), 60);
    assert_eq!(par([40 * TICK_HZ + 1, 0, 9999 * TICK_HZ]), 65);
    assert_eq!(par([0; 3]), 0);
}
