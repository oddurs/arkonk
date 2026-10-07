//! Fixed-step scheduling. The simulation advances in whole ticks of 1/240 s,
//! whatever the display's refresh rate; the leftover fraction of a tick is
//! kept for the next frame and offered to the renderer for interpolation.

/// Simulation ticks per second. Every rule is defined per tick, so this is
/// one constant on every platform.
pub const TICK_HZ: u32 = 240;
/// Seconds per tick.
pub const DT: f32 = 1.0 / TICK_HZ as f32;
/// The most ticks one frame may run. A longer stall drops the excess instead
/// of fast-forwarding the player into a lost life.
pub const MAX_CATCH_UP: u32 = 16;

/// Converts elapsed frame time into whole ticks.
#[derive(Debug, Default)]
pub struct FixedClock {
    /// Ticks owed but not yet run, always below one after `advance`.
    remainder: f64,
}

/// What one frame should simulate.
#[derive(Debug)]
pub struct Frame {
    /// Ticks to run now, at most [`MAX_CATCH_UP`].
    pub ticks: u32,
    /// How far into the next tick this frame is, in `[0, 1)`; the renderer
    /// interpolates ball positions by it.
    pub alpha: f32,
    /// Ticks owed beyond [`MAX_CATCH_UP`] and discarded.
    pub dropped: u64,
}

impl FixedClock {
    /// Forgets any partial tick, as after a pause.
    pub fn reset(&mut self) {
        self.remainder = 0.0;
    }

    /// Accounts for `seconds` of elapsed time and says how many ticks to run.
    pub fn advance(&mut self, seconds: f64) -> Frame {
        self.remainder += seconds.max(0.0) * f64::from(TICK_HZ);
        let ticks = (self.remainder + 1e-9).floor() as u64;
        self.remainder = (self.remainder - ticks as f64).max(0.0);
        Frame {
            ticks: ticks.min(u64::from(MAX_CATCH_UP)) as u32,
            alpha: self.remainder as f32,
            dropped: ticks.saturating_sub(u64::from(MAX_CATCH_UP)),
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
                ticks += frame.ticks;
                assert_eq!(frame.dropped, 0);
            }
            assert_eq!(ticks, TICK_HZ * 10, "{hz} Hz");
        }
    }
    #[test]
    fn stall_is_bounded_and_counted() {
        let mut clock = FixedClock::default();
        let f = clock.advance(1.002);
        assert_eq!(f.ticks, 16);
        assert_eq!(f.dropped, 224);
        assert!((f.alpha - 0.48).abs() < 0.001);
        clock.reset();
        assert_eq!(clock.advance(0.0).ticks, 0);
    }
}
