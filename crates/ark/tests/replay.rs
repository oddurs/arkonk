//! Golden replays: scripted sessions whose gameplay state is hashed after
//! every tick and recorded every 1,024 ticks. Any change to the rules, the
//! collision code or the order of random draws changes a digest.
//!
//! Cosmetic state (particles, notices, relay flashes) is left out, so moving
//! effects to the presentation side cannot disturb these snapshots.
//!
//! The simulation is `f32` and calls the platform's `sin` and `cos`, so the
//! digests are bit-exact only where they were recorded: macOS arm64 at
//! opt-level 0 (`debug_assertions` stands in for it). Optimized Apple builds
//! fuse `sin` and `cos` into one `__sincosf_stret` call, which rounds
//! differently.
mod common;

use ark::{
    Events, Game, Input, Mode, Power, Stage,
    field::{CELLS, Cell, CellSet},
    sectors::SectorId,
};
use common::script::Pilot;
use std::fmt::Write;

const CHECKPOINT: u32 = 1024;

/// FNV-1a over the gameplay state, chained from tick to tick.
struct Digest(u64);

impl Digest {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
    fn bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    }
    fn u8(&mut self, v: u8) {
        self.bytes(&[v]);
    }
    fn bool(&mut self, v: bool) {
        self.u8(u8::from(v));
    }
    fn u32(&mut self, v: u32) {
        self.bytes(&v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.bytes(&v.to_le_bytes());
    }
    fn f32(&mut self, v: f32) {
        self.u32(v.to_bits());
    }

    fn game(&mut self, g: &Game) {
        self.u8(stage(g.stage));
        self.u8(match g.mode {
            Mode::Journey => 0,
            Mode::Practice => 1,
        });
        self.u8(g.sector.index() as u8);
        self.u32(g.score);
        self.u8(g.lives);
        self.u32(g.board.remaining() as u32);
        self.u32(g.board.initial() as u32);
        for cell in Cell::all() {
            self.u8(g.board.hp(cell));
            self.bool(g.board.is_core(cell));
            self.u8(g.board.relay_countdown(cell));
        }
        for b in &g.balls {
            for v in [b.pos, b.previous, b.velocity] {
                self.f32(v.x);
                self.f32(v.y);
            }
            self.bool(b.active);
            self.bool(b.held);
            self.f32(b.held_offset);
            self.u8(b.phase_charges);
            let phased = b
                .phased
                .iter()
                .fold(0_u128, |bits, c| bits | 1 << c.index());
            self.bytes(&phased.to_le_bytes());
        }
        for d in &g.capsules {
            self.f32(d.pos.x);
            self.f32(d.pos.y);
            self.u8(power(d.power));
            self.bool(d.active);
        }
        self.f32(g.paddle.x);
        self.f32(g.paddle.previous);
        self.f32(g.paddle.width);
        self.f32(g.powers.wide_seconds);
        self.f32(g.powers.slow_seconds);
        self.u8(g.powers.anchor_charges);
        self.u32(g.sector_ticks);
        self.u32(g.run_ticks);
        self.u32(g.stage_ticks);
        self.u32(g.combo);
        self.u32(g.best_combo);
        let s = g.summary;
        self.u32(s.ticks);
        self.u8(s.medals.bits());
        self.u32(s.bonus);
        self.u32(s.best_combo);
        self.bool(s.life_earned);
        self.u64(g.collision_caps);
    }

    fn events(&mut self, e: &Events) {
        for flag in [
            e.brick,
            e.paddle,
            e.wall,
            e.lost,
            e.clear,
            e.pickup,
            e.launch,
            e.caught,
            e.relay,
            e.phase_hit,
        ] {
            self.bool(flag);
        }
        self.u32(e.combo);
    }
}

fn stage(stage: Stage) -> u8 {
    match stage {
        Stage::Ready => 0,
        Stage::Playing => 1,
        Stage::Cleared => 2,
        Stage::GameOver => 3,
        Stage::Victory => 4,
    }
}

fn stage_name(s: Stage) -> &'static str {
    ["ready", "playing", "cleared", "game over", "victory"][usize::from(stage(s))]
}

fn power(p: Power) -> u8 {
    match p {
        Power::Wide => 0,
        Power::Slow => 1,
        Power::Multi => 2,
        Power::Anchor => 3,
        Power::Phase => 4,
    }
}

fn ball_x(game: &Game) -> Option<f32> {
    game.balls.iter().find(|b| b.active).map(|b| b.pos.x)
}

/// Records one session: a line per checkpoint with a readable summary and
/// the chained digest. `script` sees the game before each tick and returns
/// its input; it may also change the game first, as a test fixture would.
struct Recorder {
    digest: Digest,
    tick: u32,
    out: String,
}

impl Recorder {
    fn new(title: &str) -> Self {
        Self {
            digest: Digest::new(),
            tick: 0,
            out: format!("{title}\n"),
        }
    }

    fn run(
        &mut self,
        game: &mut Game,
        ticks: u32,
        mut script: impl FnMut(u32, &mut Game) -> Input,
    ) {
        for _ in 0..ticks {
            let input = script(self.tick, game);
            game.step(&input);
            self.digest.game(game);
            self.digest.events(&game.events);
            self.tick += 1;
            if self.tick.is_multiple_of(CHECKPOINT) {
                let _ = writeln!(
                    self.out,
                    "{:>6}  sector {:02} {:<9} score {:>6}  lives {}  bricks {:>2}  {:016x}",
                    self.tick,
                    game.sector.index() + 1,
                    stage_name(game.stage),
                    game.score,
                    game.lives,
                    game.board.remaining(),
                    self.digest.0,
                );
            }
        }
    }
}

fn pilot(seed: u64) -> impl FnMut(u32, &mut Game) -> Input {
    let mut pilot = Pilot::new(seed);
    move |_, game| pilot.input(ball_x(game))
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64", debug_assertions)),
    ignore = "f32 goldens are recorded on macOS arm64 at opt-level 0"
)]
fn every_sector() {
    let mut out = String::new();
    for sector in 0..12 {
        for (seed, mode) in [(1, Mode::Journey), (2, Mode::Practice)] {
            let mut game = Game::start(SectorId::new(sector).unwrap(), mode);
            let mut r = Recorder::new(&format!("sector {:02} {mode:?} seed {seed}", sector + 1));
            r.run(&mut game, 6 * CHECKPOINT, pilot(seed * 100 + sector as u64));
            out += &r.out;
        }
    }
    insta::assert_snapshot!(out);
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64", debug_assertions)),
    ignore = "f32 goldens are recorded on macOS arm64 at opt-level 0"
)]
fn journey() {
    let mut out = String::new();
    for seed in [7, 8, 9] {
        let mut game = Game::new();
        let mut r = Recorder::new(&format!("journey seed {seed}"));
        let mut pilot = pilot(seed);
        r.run(&mut game, 120 * CHECKPOINT, |tick, game| {
            if matches!(game.stage, Stage::GameOver | Stage::Victory) {
                *game = Game::new();
            }
            pilot(tick, game)
        });
        out += &r.out;
    }
    insta::assert_snapshot!(out);
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64", debug_assertions)),
    ignore = "f32 goldens are recorded on macOS arm64 at opt-level 0"
)]
fn full_board_relay() {
    let mut game = Game::start(SectorId::new(0).unwrap(), Mode::Journey);
    game.step(&Input {
        launch: true,
        ..Input::default()
    });
    game.board.reset([1; CELLS], CellSet::ALL);
    let mut r = Recorder::new("full board of relay cores");
    r.run(&mut game, 4 * CHECKPOINT, pilot(11));
    insta::assert_snapshot!(r.out);
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64", debug_assertions)),
    ignore = "f32 goldens are recorded on macOS arm64 at opt-level 0"
)]
fn every_power() {
    const ORDER: [Power; 5] = [
        Power::Wide,
        Power::Slow,
        Power::Multi,
        Power::Anchor,
        Power::Phase,
    ];
    let mut game = Game::start(SectorId::new(4).unwrap(), Mode::Practice);
    let mut r = Recorder::new("every power in turn, twice");
    let mut pilot = pilot(12);
    r.run(&mut game, 8 * CHECKPOINT, |tick, game| {
        let turn = (tick / 600) as usize;
        if tick % 600 == 300 && turn < 2 * ORDER.len() && game.stage == Stage::Playing {
            game.apply_power(ORDER[turn % ORDER.len()]);
        }
        pilot(tick, game)
    });
    insta::assert_snapshot!(r.out);
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64", debug_assertions)),
    ignore = "f32 goldens are recorded on macOS arm64 at opt-level 0"
)]
fn anchor_hold() {
    let mut game = Game::start(SectorId::new(1).unwrap(), Mode::Journey);
    let mut r = Recorder::new("anchor catches held for seconds, then released");
    let mut pilot = pilot(13);
    r.run(&mut game, 6 * CHECKPOINT, |tick, game| {
        if game.stage == Stage::Playing && game.powers.anchor_charges == 0 {
            game.apply_power(Power::Anchor);
        }
        let mut input = pilot(tick, game);
        input.launch = game.stage != Stage::Playing || tick % 960 == 0;
        input
    });
    insta::assert_snapshot!(r.out);
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64", debug_assertions)),
    ignore = "f32 goldens are recorded on macOS arm64 at opt-level 0"
)]
fn sector_endings() {
    let mut out = String::new();
    // The last sector of each chapter: medals, the chapter's extra life, the
    // next chapter's opening and, after the twelfth, victory.
    for sector in [3, 7, 11] {
        let mut game = Game::start(SectorId::new(sector).unwrap(), Mode::Journey);
        let mut kept = 0;
        for cell in Cell::all().rev() {
            if game.board.hp(cell) > 0 {
                if kept < 3 {
                    kept += 1;
                } else {
                    game.board.set(cell, 0, game.board.is_core(cell));
                }
            }
        }
        let mut r = Recorder::new(&format!(
            "sector {:02} down to its last three bricks",
            sector + 1
        ));
        r.run(&mut game, 8 * CHECKPOINT, pilot(20 + sector as u64));
        out += &r.out;
    }
    insta::assert_snapshot!(out);
}
