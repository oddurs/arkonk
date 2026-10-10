//! A ball, and how it moves through one tick.
//!
//! Movement is continuous: each step sweeps the ball's circle along its path
//! against the walls, the moving paddle and the live bricks the path can
//! reach, and stops at the earliest contact. The game reacts to that contact
//! (a bounce, a catch, damage) and the ball continues with what is left of
//! the tick, until the tick is used up or the collision budget runs out.
use super::{board::Board, paddle::Paddle};
use crate::{
    clock::DT,
    field::{
        BALL_RADIUS, BOTTOM, Cell, CellSet, LEFT, PADDLE_Y, RIGHT, TOP, cell_rect, swept_cells,
    },
    geom::{V2, sweep_circle_rect},
    tuning::{MIN_VERTICAL, STEEPEN_BELOW_SPEED},
};

/// A ball slot.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ball {
    /// Centre now.
    pub pos: V2,
    /// Centre at the start of this tick; the renderer interpolates from it.
    pub previous: V2,
    /// Pixels per second.
    pub velocity: V2,
    /// Whether the slot holds a ball in play.
    pub active: bool,
    /// Whether Anchor is holding it on the paddle.
    pub held: bool,
    /// Where on the paddle it is held, from -1 (left end) to 1 (right end).
    pub held_offset: f32,
    /// Brick contacts it will pass through instead of bouncing.
    pub phase_charges: u8,
    /// Bricks it is passing through; each is ignored until the ball has
    /// fully left it.
    pub phased: CellSet,
}

impl Ball {
    /// Stops ignoring phased bricks the ball no longer overlaps.
    pub(crate) fn leave_phased(&mut self) {
        for cell in self.phased.iter() {
            let r = cell_rect(cell);
            if self.pos.x < r.x - BALL_RADIUS
                || self.pos.x > r.x + r.w + BALL_RADIUS
                || self.pos.y < r.y - BALL_RADIUS
                || self.pos.y > r.y + r.h + BALL_RADIUS
            {
                self.phased.remove(cell);
            }
        }
    }

    /// Reflects the velocity off a surface with unit `normal`.
    pub(crate) fn bounce(&mut self, normal: V2) {
        self.velocity = self.velocity - normal * (2.0 * self.velocity.dot(normal));
        if self.velocity.length() < STEEPEN_BELOW_SPEED {
            self.velocity = steepen(self.velocity);
        }
    }
}

/// Tilts a velocity that is too close to horizontal up to [`MIN_VERTICAL`],
/// keeping its speed and sideways direction. Flat rallies would otherwise go
/// on forever.
pub(crate) fn steepen(velocity: V2) -> V2 {
    let speed = velocity.length();
    if speed < 1.0 {
        return velocity;
    }
    let mut direction = velocity * (1.0 / speed);
    if direction.y.abs() < MIN_VERTICAL {
        direction.y = MIN_VERTICAL * if direction.y < 0.0 { -1.0 } else { 1.0 };
        direction.x = (1.0 - direction.y * direction.y).sqrt() * direction.x.signum();
    }
    direction * speed
}

/// The first thing a ball's path touches.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Contact {
    /// Nothing: the ball reached the end of the tick.
    None,
    /// A wall, with the surface normal.
    Wall(V2),
    /// The paddle's top or ends, with the surface normal.
    Paddle(V2),
    /// The brick in a cell, with the surface normal.
    Brick(Cell, V2),
    /// The open bottom edge: the ball is gone.
    Drain,
}

/// Less than this much of a tick left, in seconds, counts as none.
const SPENT: f32 = 0.000001;

/// One ball's progress through a tick.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Sweep {
    /// Seconds of the tick still to move.
    remaining: f32,
    /// Seconds of the tick already moved.
    elapsed: f32,
}

impl Sweep {
    /// The whole tick ahead.
    pub(crate) const fn new() -> Self {
        Self {
            remaining: DT,
            elapsed: 0.0,
        }
    }

    /// Whether the tick is used up.
    pub(crate) fn is_done(&self) -> bool {
        self.remaining < SPENT
    }

    /// Seconds of the tick already moved.
    pub(crate) fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Ends the tick for this ball where it is.
    pub(crate) fn stop(&mut self) {
        self.remaining = 0.0;
    }

    /// Moves `ball` along its velocity to the first contact, or to the end of
    /// the tick, and reports what it touched. Later candidates win exact
    /// ties: a brick over the paddle, the paddle over a wall.
    pub(crate) fn advance(&mut self, ball: &mut Ball, paddle: &Paddle, board: &Board) -> Contact {
        let delta = ball.velocity * self.remaining;
        let mut first = 1.0;
        let mut contact = Contact::None;
        // Times past the end of the step mark walls the ball moves away from.
        for (t, wall) in [
            (
                if delta.x < 0.0 {
                    (LEFT + BALL_RADIUS - ball.pos.x) / delta.x
                } else {
                    2.0
                },
                Contact::Wall(V2::new(1.0, 0.0)),
            ),
            (
                if delta.x > 0.0 {
                    (RIGHT - BALL_RADIUS - ball.pos.x) / delta.x
                } else {
                    2.0
                },
                Contact::Wall(V2::new(-1.0, 0.0)),
            ),
            (
                if delta.y < 0.0 {
                    (TOP + BALL_RADIUS - ball.pos.y) / delta.y
                } else {
                    2.0
                },
                Contact::Wall(V2::new(0.0, 1.0)),
            ),
            (
                if delta.y > 0.0 {
                    (BOTTOM + BALL_RADIUS - ball.pos.y) / delta.y
                } else {
                    2.0
                },
                Contact::Drain,
            ),
        ] {
            if t >= 0.0 && t <= first {
                first = t;
                contact = wall;
            }
        }
        // Only a falling ball above the paddle's top edge can be returned; it
        // is never scooped up from below. The sweep runs in the paddle's own
        // frame, so a moving paddle meets the ball where both actually are.
        if ball.velocity.y > 0.0
            && ball.pos.y <= PADDLE_Y
            && let Some(hit) = sweep_circle_rect(
                ball.pos,
                delta - V2::new(paddle.speed() * self.remaining, 0.0),
                BALL_RADIUS,
                paddle.rect_at(self.elapsed),
            )
            && hit.t <= first
        {
            first = hit.t;
            contact = Contact::Paddle(hit.normal);
        }
        for cell in swept_cells(ball.pos, ball.pos + delta, BALL_RADIUS) {
            if board.is_solid(cell)
                && !ball.phased.contains(cell)
                && let Some(hit) = sweep_circle_rect(ball.pos, delta, BALL_RADIUS, cell_rect(cell))
                && hit.t <= first
            {
                first = hit.t;
                contact = Contact::Brick(cell, hit.normal);
            }
        }
        ball.pos += delta * first;
        self.elapsed += self.remaining * first;
        self.remaining *= 1.0 - first;
        contact
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_bounces_are_steepened_without_changing_speed() {
        let v = steepen(V2::new(500.0, -1.0));
        assert!(v.y < -100.0);
        assert!((v.length() - 500.001).abs() < 0.01);
    }
}
