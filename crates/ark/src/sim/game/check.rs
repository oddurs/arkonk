//! The rules' invariants, checkable after any tick.
use super::{Game, Stage};
use crate::{
    field::{BALL_RADIUS, BOTTOM, Cell, LEFT, RIGHT, TOP},
    tuning::{
        ANCHOR_CHARGES, MAX_LIVES, PADDLE_WIDTH, PHASE_CONTACTS, SLOW_SECONDS, WIDE_PADDLE_WIDTH,
        WIDE_SECONDS,
    },
};

/// Counters that say how the simulation is coping.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Diagnostics {
    /// Ticks in which a ball spent its whole collision budget and was held
    /// at its last safe position. Authored play never does this.
    pub budget_exhausted: u64,
}

/// A broken invariant: a bug in the rules, or a sandbox setup that no game
/// could reach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Violation {
    /// The board's remaining count disagrees with its hit points.
    RemainingCount,
    /// A brick has more hit points than any layout gives.
    BrickHp(Cell),
    /// A ball slot that is not in play is marked as held.
    HeldInactive(usize),
    /// Anchor holds more than one ball.
    HeldTwice,
    /// A ball's position is not finite, or it is far outside the field.
    BallOutside(usize),
    /// The paddle has a width no rule gives it, or is outside the walls.
    Paddle,
    /// Lives are zero outside game over, or above the maximum.
    Lives,
    /// A power timer or charge count is outside its range.
    Powers,
    /// The run has less play time than the sector inside it.
    Clocks,
    /// The random generator reached zero, where it would stay.
    Rng,
}

/// The board's thickest armour.
const MAX_HP: u8 = 3;

impl Game {
    /// Checks the invariants the rules maintain, and returns the first that
    /// does not hold.
    pub fn validate(&self) -> Result<(), Violation> {
        let mut live = 0;
        for cell in Cell::all() {
            let hp = self.board.hp(cell);
            if hp > MAX_HP {
                return Err(Violation::BrickHp(cell));
            }
            live += usize::from(hp > 0);
        }
        if live != self.board.remaining() {
            return Err(Violation::RemainingCount);
        }
        let margin = 2.0 * BALL_RADIUS;
        for (slot, ball) in self.balls.iter().enumerate() {
            if ball.held && !ball.active {
                return Err(Violation::HeldInactive(slot));
            }
            let p = ball.pos;
            let inside = p.x >= LEFT - margin
                && p.x <= RIGHT + margin
                && p.y >= TOP - margin
                && p.y <= BOTTOM + margin;
            if ball.active && !inside {
                return Err(Violation::BallOutside(slot));
            }
        }
        if self.balls.iter().filter(|b| b.held).count() > 1 {
            return Err(Violation::HeldTwice);
        }
        let paddle = self.paddle;
        let half = paddle.width / 2.0;
        if !(paddle.width == PADDLE_WIDTH || paddle.width == WIDE_PADDLE_WIDTH)
            || !(paddle.x >= LEFT + half && paddle.x <= RIGHT - half)
        {
            return Err(Violation::Paddle);
        }
        if (self.lives == 0) != (self.stage == Stage::GameOver) || self.lives > MAX_LIVES {
            return Err(Violation::Lives);
        }
        let powers = self.powers;
        if !(0.0..=WIDE_SECONDS).contains(&powers.wide_seconds)
            || !(0.0..=SLOW_SECONDS).contains(&powers.slow_seconds)
            || powers.anchor_charges > ANCHOR_CHARGES
            || self.balls.iter().any(|b| b.phase_charges > PHASE_CONTACTS)
        {
            return Err(Violation::Powers);
        }
        if self.run_ticks < self.sector_ticks {
            return Err(Violation::Clocks);
        }
        if self.rng.is_stuck() {
            return Err(Violation::Rng);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Input, Power, field::CellSet, geom::V2};

    #[test]
    fn sandbox_setups_stay_valid_and_bad_ones_are_reported() {
        let mut g = Game::new();
        assert_eq!(g.validate(), Ok(()));
        g.step(Input {
            launch: true,
            ..Input::default()
        });
        let mut sandbox = g.sandbox();
        sandbox.fill_board(2, CellSet::ALL);
        sandbox.grant(Power::Multi);
        sandbox.place_ball(1, V2::new(300.0, 500.0), V2::new(200.0, -400.0));
        assert_eq!(g.validate(), Ok(()));
        let corner = Cell::new(0).unwrap();
        g.sandbox().set_brick(corner, 4, false);
        assert_eq!(g.validate(), Err(Violation::BrickHp(corner)));
        g.sandbox().set_brick(corner, 1, false);
        g.sandbox()
            .place_ball(2, V2::new(5000.0, 500.0), V2::default());
        assert_eq!(g.validate(), Err(Violation::BallOutside(2)));
    }
}
