//! Setting up situations that play would take too long to reach.
use super::{Ball, Capsule, Game, Power, Stage};
use crate::{
    field::{CELLS, Cell, CellSet},
    geom::V2,
    sim::effects::Effects,
};

/// Privileged access to a [`Game`] for tests, benchmarks and demos.
///
/// Each operation keeps the game's own bookkeeping consistent: editing the
/// board keeps the count of remaining bricks, a placed ball is in play, and
/// a granted power acts as if its capsule had been caught. None of it is
/// reachable through [`Game::step`], so a replay that uses the sandbox must
/// record its calls along with its inputs.
#[derive(Debug)]
pub struct Sandbox<'a> {
    /// The game being arranged.
    game: &'a mut Game,
}

impl<'a> Sandbox<'a> {
    /// Opens `game` for arranging.
    pub(super) fn new(game: &'a mut Game) -> Self {
        Self { game }
    }

    /// Replaces every brick with one of `hp` hit points, marking `cores` as
    /// relay cores. The new board counts as the sector's starting board, and
    /// pending blasts are cancelled.
    pub fn fill_board(&mut self, hp: u8, cores: CellSet) {
        self.game.board.reset([hp; CELLS], cores);
    }

    /// Removes every brick. The next tick in play clears the sector.
    pub fn clear_board(&mut self) {
        self.fill_board(0, CellSet::EMPTY);
    }

    /// Sets one cell's brick: its hit points and whether it is a relay core.
    pub fn set_brick(&mut self, cell: Cell, hp: u8, core: bool) {
        self.game.board.set(cell, hp, core);
    }

    /// Puts the ball in `slot` in play at `pos`, moving at `velocity`, and
    /// out of Anchor's hold. A ball already in play keeps its Phase charges;
    /// an empty slot gets a fresh ball.
    pub fn place_ball(&mut self, slot: usize, pos: V2, velocity: V2) {
        let ball = &mut self.game.balls[slot];
        if !ball.active {
            *ball = Ball::default();
        }
        *ball = Ball {
            pos,
            previous: pos,
            velocity,
            active: true,
            held: false,
            ..*ball
        };
    }

    /// Gives the ball in `slot` a new velocity. Its position and whether
    /// Anchor holds it do not change; a held ball leaves with it.
    pub fn aim_ball(&mut self, slot: usize, velocity: V2) {
        self.game.balls[slot].velocity = velocity;
    }

    /// Puts a falling capsule of `power` at `pos` in capsule `slot`.
    pub fn spawn_capsule(&mut self, slot: usize, pos: V2, power: Power) {
        self.game.capsules[slot] = Capsule {
            pos,
            power,
            active: true,
        };
    }

    /// Grants `power` as if its capsule had just been caught.
    pub fn grant(&mut self, power: Power) {
        self.game.apply_power(power);
    }

    /// Lets `ticks` pass with nothing moving. The stage clock always
    /// advances; the sector and run clocks only during play, as in a tick.
    pub fn elapse(&mut self, ticks: u32) {
        let game = &mut *self.game;
        game.stage_ticks = game.stage_ticks.saturating_add(ticks);
        if game.stage == Stage::Playing {
            game.sector_ticks = game.sector_ticks.saturating_add(ticks);
            game.run_ticks = game.run_ticks.saturating_add(ticks);
        }
    }

    /// The cosmetic pools. They carry no rules, so any value is allowed.
    pub fn effects(&mut self) -> &mut Effects {
        &mut self.game.effects
    }
}
