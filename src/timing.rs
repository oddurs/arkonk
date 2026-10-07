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

/// Holds the simulation while the window changes size or display mode, so a
/// resize or fullscreen transition can neither drain a ball nor read as a stall.
pub struct Settle {
    size: (f32, f32),
    remaining: f64,
}
impl Settle {
    /// The window must keep one size this long before play resumes.
    pub const QUIET: f64 = 0.5;
    /// A mode switch animates (a macOS Space transition takes about 0.7 s).
    pub const MODE_SWITCH: f64 = 1.0;
    pub fn new(size: (f32, f32)) -> Self {
        Self {
            size,
            remaining: 0.0,
        }
    }
    /// True while the simulation must hold. A frame that arrives inside the
    /// window is held even if it is long: it belongs to the transition.
    pub fn hold(&mut self, size: (f32, f32), mode_switched: bool, seconds: f64) -> bool {
        if size != self.size {
            self.size = size;
            self.remaining = self.remaining.max(Self::QUIET);
        }
        if mode_switched {
            self.remaining = Self::MODE_SWITCH;
        }
        let hold = self.remaining > 0.0;
        self.remaining -= seconds.max(0.0);
        hold
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
    #[test]
    fn steady_window_never_holds() {
        let mut settle = Settle::new((960.0, 900.0));
        for _ in 0..1000 {
            assert!(!settle.hold((960.0, 900.0), false, 1.0 / 120.0));
        }
    }
    #[test]
    fn resize_holds_until_size_is_quiet() {
        let mut settle = Settle::new((960.0, 900.0));
        let frame = 1.0 / 60.0;
        // A live drag: the size changes every frame for half a second.
        for i in 1..=30 {
            assert!(settle.hold((960.0 + i as f32, 900.0), false, frame));
        }
        let mut held = 0;
        while settle.hold((990.0, 900.0), false, frame) {
            held += 1;
        }
        let quiet = held as f64 * frame;
        assert!((quiet - Settle::QUIET).abs() <= frame, "held {quiet} s");
    }
    #[test]
    fn long_transition_frames_are_held_not_stalls() {
        let mut settle = Settle::new((960.0, 900.0));
        assert!(settle.hold((960.0, 900.0), true, 1.0 / 120.0));
        // The window server blocks presentation during the switch.
        assert!(settle.hold((960.0, 900.0), false, 0.4));
        assert!(settle.hold((1728.0, 1117.0), false, 0.45));
        assert!(settle.hold((1728.0, 1117.0), false, 0.3));
        assert!(!settle.hold((1728.0, 1117.0), false, 1.0 / 120.0));
    }
}
