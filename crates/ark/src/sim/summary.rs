//! How a cleared sector went.
use core::ops::{BitOr, BitOrAssign};

/// A set of the three medals a sector awards.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Medals(u8);

impl Medals {
    /// No medals.
    pub const NONE: Self = Self(0);
    /// Finished the sector.
    pub const CLEAR: Self = Self(1);
    /// Finished without losing a life.
    pub const CLEAN: Self = Self(2);
    /// Finished within the sector's target time.
    pub const SWIFT: Self = Self(4);
    /// All three.
    pub const ALL: Self = Self(7);

    /// From a saved bit pattern; bits that name no medal are dropped.
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits & Self::ALL.0)
    }
    /// The bit pattern, as saved.
    pub const fn bits(self) -> u8 {
        self.0
    }
    /// Whether every medal in `other` is in this set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
    /// How many medals the set holds.
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }
}

impl BitOr for Medals {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for Medals {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// The results of a cleared sector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SectorSummary {
    /// Active play time, in ticks.
    pub ticks: u32,
    /// Medals earned.
    pub medals: Medals,
    /// Points awarded for the clear and its medals.
    pub bonus: u32,
    /// The longest chain of breaks between paddle returns.
    pub best_combo: u32,
    /// Whether finishing the chapter earned an extra life.
    pub life_earned: bool,
}
