//! Context-free 2D geometry: vectors, rectangles, and the swept circle test
//! that every collision in the game goes through. Nothing here knows about
//! bricks or paddles.
use core::ops::{Add, AddAssign, Mul, Sub};

/// A point or a displacement in world pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V2 {
    /// Rightward.
    pub x: f32,
    /// Downward.
    pub y: f32,
}

impl V2 {
    /// A vector from its components.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// The dot product.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    /// The Euclidean length.
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    /// This direction at unit length; zero stays zero.
    pub fn normalized(self) -> Self {
        let length = self.length();
        if length > 0.0 {
            self * (1.0 / length)
        } else {
            Self::default()
        }
    }
    /// The point `t` of the way from `self` to `other`.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Add for V2 {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self::new(self.x + b.x, self.y + b.y)
    }
}

impl AddAssign for V2 {
    fn add_assign(&mut self, b: Self) {
        *self = *self + b;
    }
}

impl Sub for V2 {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self::new(self.x - b.x, self.y - b.y)
    }
}

impl Mul<f32> for V2 {
    type Output = Self;
    fn mul(self, b: f32) -> Self {
        Self::new(self.x * b, self.y * b)
    }
}

/// An axis-aligned rectangle: top-left corner and size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

impl Rect {
    /// The middle of the rectangle.
    pub fn center(self) -> V2 {
        V2::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
}

/// The first contact of a swept circle.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    /// When, as a fraction of the sweep in `[0, 1]`.
    pub t: f32,
    /// The unit surface normal at the contact, pointing out of the rectangle.
    pub normal: V2,
}

/// Sweeps a circle of `radius` from `p` by `delta` against `rect` and returns
/// the first contact, if any.
///
/// The test is exact: four faces pushed out by the radius, and four rounded
/// corners solved as circle–point intersections, so a ball grazing a corner
/// is not stopped by an imaginary square. Contacts the circle is moving away
/// from are ignored, so a ball resting on a surface can always leave it.
///
/// ```
/// use ark::geom::{Rect, V2, sweep_circle_rect};
///
/// let brick = Rect { x: 10.0, y: 10.0, w: 10.0, h: 10.0 };
/// // Moving straight up into the bottom face, 100 px in one step: no tunneling.
/// let hit = sweep_circle_rect(V2::new(15.0, 100.0), V2::new(0.0, -200.0), 2.0, brick)
///     .expect("the sweep crosses the brick");
/// assert!((hit.t - 0.39).abs() < 1e-5);
/// assert_eq!(hit.normal, V2::new(0.0, 1.0));
/// ```
pub fn sweep_circle_rect(p: V2, delta: V2, radius: f32, rect: Rect) -> Option<Hit> {
    let mut best: Option<Hit> = None;
    let mut consider = |t: f32, normal: V2| {
        if (0.0..=1.0).contains(&t) && delta.dot(normal) < -0.00001 && best.is_none_or(|h| t < h.t)
        {
            best = Some(Hit { t, normal });
        }
    };
    if delta.x != 0.0 {
        for (x, normal) in [
            (rect.x - radius, V2::new(-1.0, 0.0)),
            (rect.x + rect.w + radius, V2::new(1.0, 0.0)),
        ] {
            let t = (x - p.x) / delta.x;
            let y = p.y + delta.y * t;
            if y >= rect.y && y <= rect.y + rect.h {
                consider(t, normal);
            }
        }
    }
    if delta.y != 0.0 {
        for (y, normal) in [
            (rect.y - radius, V2::new(0.0, -1.0)),
            (rect.y + rect.h + radius, V2::new(0.0, 1.0)),
        ] {
            let t = (y - p.y) / delta.y;
            let x = p.x + delta.x * t;
            if x >= rect.x && x <= rect.x + rect.w {
                consider(t, normal);
            }
        }
    }
    let a = delta.dot(delta);
    if a > 0.000001 {
        for (x, sx) in [(rect.x, -1.0), (rect.x + rect.w, 1.0)] {
            for (y, sy) in [(rect.y, -1.0), (rect.y + rect.h, 1.0)] {
                let offset = p - V2::new(x, y);
                let b = offset.dot(delta);
                let c = offset.dot(offset) - radius * radius;
                let discriminant = b * b - a * c;
                if discriminant >= 0.0 {
                    let t = (-b - discriminant.sqrt()) / a;
                    let contact = offset + delta * t;
                    // Only the corner's own quadrant is rounded; elsewhere a
                    // face test above already covers the contact.
                    if contact.x * sx >= -0.0001 && contact.y * sy >= -0.0001 {
                        consider(t, contact.normalized());
                    }
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    const BOX: Rect = Rect {
        x: 10.0,
        y: 10.0,
        w: 10.0,
        h: 10.0,
    };
    #[test]
    fn fast_ball_cannot_tunnel() {
        let h = sweep_circle_rect(V2::new(15.0, 100.0), V2::new(0.0, -200.0), 2.0, BOX).unwrap();
        assert!((h.t - 0.39).abs() < 0.00001);
        assert_eq!(h.normal, V2::new(0.0, 1.0));
    }
    #[test]
    fn rounded_corner_not_expanded_square() {
        assert!(sweep_circle_rect(V2::new(8.1, 7.0), V2::new(0.0, 1.2), 2.0, BOX).is_none());
        let h = sweep_circle_rect(V2::new(5.0, 5.0), V2::new(10.0, 10.0), 2.0, BOX).unwrap();
        assert!((h.t - (0.5 - 2.0_f32.sqrt() / 10.0)).abs() < 0.00001);
        assert!(h.normal.x < -0.7 && h.normal.y < -0.7);
    }
    #[test]
    fn parallel_and_departing_balls_do_not_hit() {
        assert!(sweep_circle_rect(V2::new(5.0, 5.0), V2::new(50.0, 0.0), 2.0, BOX).is_none());
        assert!(sweep_circle_rect(V2::new(8.0, 15.0), V2::new(-10.0, 0.0), 2.0, BOX).is_none());
        assert!(sweep_circle_rect(V2::new(5.0, 5.0), V2::default(), 2.0, BOX).is_none());
    }
}
