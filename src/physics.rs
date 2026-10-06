use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

impl V2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalized(self) -> Self {
        let length = self.length();
        if length > 0.0 {
            self * (1.0 / length)
        } else {
            Self::default()
        }
    }
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

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub t: f32,
    pub normal: V2,
}

/// Exact continuous circle vs. rectangle: four faces and four rounded corners.
/// Time is a fraction of `delta`; normals point out of the rectangle.
pub fn sweep_circle(p: V2, delta: V2, radius: f32, rect: Rect) -> Option<Hit> {
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
        let h = sweep_circle(V2::new(15.0, 100.0), V2::new(0.0, -200.0), 2.0, BOX).unwrap();
        assert!((h.t - 0.39).abs() < 0.00001);
        assert_eq!(h.normal, V2::new(0.0, 1.0));
    }
    #[test]
    fn rounded_corner_not_expanded_square() {
        assert!(sweep_circle(V2::new(8.1, 7.0), V2::new(0.0, 1.2), 2.0, BOX).is_none());
        let h = sweep_circle(V2::new(5.0, 5.0), V2::new(10.0, 10.0), 2.0, BOX).unwrap();
        assert!((h.t - (0.5 - 2.0_f32.sqrt() / 10.0)).abs() < 0.00001);
        assert!(h.normal.x < -0.7 && h.normal.y < -0.7);
    }
    #[test]
    fn parallel_and_departing_balls_do_not_hit() {
        assert!(sweep_circle(V2::new(5.0, 5.0), V2::new(50.0, 0.0), 2.0, BOX).is_none());
        assert!(sweep_circle(V2::new(8.0, 15.0), V2::new(-10.0, 0.0), 2.0, BOX).is_none());
        assert!(sweep_circle(V2::new(5.0, 5.0), V2::default(), 2.0, BOX).is_none());
    }
}
