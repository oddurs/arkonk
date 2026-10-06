//! Fixed-step scheduling with bounded catch-up and a preserved fractional tick.
use crate::game::TICK_HZ;
#[derive(Default)]
pub struct FixedClock {
    remainder: f64,
}
pub struct Frame {
    pub steps: u32,
    pub alpha: f32,
    pub dropped: u64,
}
impl FixedClock {
    pub fn reset(&mut self) {
        self.remainder = 0.0;
    }
    pub fn advance(&mut self, seconds: f64) -> Frame {
        self.remainder += seconds.max(0.0) * f64::from(TICK_HZ);
        let ticks = (self.remainder + 1e-9).floor() as u64;
        self.remainder = (self.remainder - ticks as f64).max(0.0);
        Frame {
            steps: ticks.min(16) as u32,
            alpha: self.remainder as f32,
            dropped: ticks.saturating_sub(16),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_of_display_refresh() {
        for hz in [30, 60, 120, 144, 165, 240, 360] {
            let mut clock = FixedClock::default();
            let mut ticks = 0;
            for _ in 0..hz * 10 {
                let frame = clock.advance(1.0 / hz as f64);
                ticks += frame.steps;
                assert_eq!(frame.dropped, 0);
            }
            assert_eq!(ticks, TICK_HZ * 10, "{hz} Hz");
        }
    }
    #[test]
    fn stall_is_bounded_and_counted() {
        let mut clock = FixedClock::default();
        let f = clock.advance(1.002);
        assert_eq!(f.steps, 16);
        assert_eq!(f.dropped, 224);
        assert!((f.alpha - 0.48).abs() < 0.001);
        clock.reset();
        assert_eq!(clock.advance(0.0).steps, 0);
    }
}
