//! The gameplay random generator.
use crate::tuning::RNG_SEED;

/// xorshift32: tiny, fast, and the same on every platform. Its state is
/// never zero, which would be a fixed point.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Rng(u32);

impl Rng {
    /// The generator every game starts from.
    pub(crate) const fn new() -> Self {
        Self(RNG_SEED)
    }

    /// Whether the state reached zero, which xorshift never leaves.
    pub(crate) fn is_stuck(&self) -> bool {
        self.0 == 0
    }

    /// The next value, uniform in `[0, 1]`.
    pub(crate) fn next_f32(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0 as f32 / u32::MAX as f32
    }
}
