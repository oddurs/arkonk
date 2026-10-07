//! The paddle: where it is, where it was a tick ago, and how a ball leaves it.
use crate::{
    clock::DT,
    field::{BALL_RADIUS, FIELD, LEFT, PADDLE_Y, RIGHT},
    geom::{Rect, V2},
    tuning::{
        CATCH_ABOVE, CATCH_BELOW, CATCH_MARGIN, KEYBOARD_SPEED, MAX_BOUNCE_ANGLE, PADDLE_HEIGHT,
        PADDLE_WIDTH, SERVE_GAP,
    },
};

/// Where a served or held ball rests: on the paddle, clear of it by the gap.
const REST_Y: f32 = PADDLE_Y - BALL_RADIUS - SERVE_GAP;

/// The paddle. It moves on a fixed line at [`PADDLE_Y`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Paddle {
    /// Centre, now.
    pub x: f32,
    /// Centre at the start of this tick; collisions sweep between the two.
    pub previous: f32,
    /// Full width.
    pub width: f32,
}

impl Paddle {
    /// A normal-width paddle in the middle of the field.
    pub(crate) fn new() -> Self {
        let x = FIELD.center().x;
        Self {
            x,
            previous: x,
            width: PADDLE_WIDTH,
        }
    }

    /// Moves to `target` if the pointer gives one, otherwise by `axis` at
    /// keyboard speed, and stays inside the walls.
    pub(crate) fn steer(&mut self, target: Option<f32>, axis: f32) {
        self.previous = self.x;
        self.x = target.unwrap_or(self.x + axis * KEYBOARD_SPEED * DT);
        self.keep_inside();
    }

    /// Pulls the paddle back inside the walls at its current width.
    pub(crate) fn keep_inside(&mut self) {
        self.x = self
            .x
            .clamp(LEFT + self.width / 2.0, RIGHT - self.width / 2.0);
    }

    /// Horizontal speed over this tick, in pixels per second.
    pub fn speed(&self) -> f32 {
        (self.x - self.previous) / DT
    }

    /// The paddle `elapsed` seconds into this tick.
    pub(crate) fn rect_at(&self, elapsed: f32) -> Rect {
        Rect {
            x: self.previous + self.speed() * elapsed - self.width / 2.0,
            y: PADDLE_Y,
            w: self.width,
            h: PADDLE_HEIGHT,
        }
    }

    /// Where along the paddle `x` hit it, `elapsed` seconds into the tick:
    /// -1 at the left end, 0 in the centre, 1 at the right end.
    pub(crate) fn hit_offset(&self, x: f32, elapsed: f32) -> f32 {
        let centre = self.previous + (self.x - self.previous) * (elapsed / DT);
        ((x - centre) / (self.width / 2.0)).clamp(-1.0, 1.0)
    }

    /// The unit direction a ball leaves in after hitting at `offset`: straight
    /// up from the centre, tilting toward [`MAX_BOUNCE_ANGLE`] at the ends.
    pub(crate) fn bounce(offset: f32) -> V2 {
        let angle = offset * MAX_BOUNCE_ANGLE;
        V2::new(angle.sin(), -angle.cos())
    }

    /// Where a ball waits to be served.
    pub fn serve_point(&self) -> V2 {
        V2::new(self.x, REST_Y)
    }

    /// Where Anchor holds a ball caught at `offset`.
    pub fn hold_point(&self, offset: f32) -> V2 {
        V2::new(self.x + offset * self.width / 2.0, REST_Y)
    }

    /// Whether a capsule at `p` is caught.
    pub(crate) fn catches(&self, p: V2) -> bool {
        p.y >= PADDLE_Y - CATCH_ABOVE
            && p.y <= PADDLE_Y + CATCH_BELOW
            && (p.x - self.x).abs() < self.width / 2.0 + CATCH_MARGIN
    }
}
