//! The five capsules and the timers and charges they leave behind.
use crate::clock::DT;

/// A capsule's power.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Power {
    /// A wider paddle for a while.
    #[default]
    Wide,
    /// Slower balls for a while.
    Slow,
    /// Splits a ball into up to three.
    Multi,
    /// Sticky catches: the paddle holds a ball until it is released.
    Anchor,
    /// Balls pass through their next few brick contacts, damaging each.
    Phase,
}

impl Power {
    /// Every power, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Wide,
        Self::Slow,
        Self::Multi,
        Self::Anchor,
        Self::Phase,
    ];

    /// A stable lowercase key for string tables and stats; never displayed.
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Wide => "wide",
            Self::Slow => "slow",
            Self::Multi => "multi",
            Self::Anchor => "anchor",
            Self::Phase => "phase",
        }
    }
}

/// The powers in effect. Phase charges belong to each ball instead.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PowerState {
    /// Seconds of Wide left.
    pub wide_seconds: f32,
    /// Seconds of Slow left.
    pub slow_seconds: f32,
    /// Anchor catches left.
    pub anchor_charges: u8,
}

impl PowerState {
    /// Whether Wide is active.
    pub fn wide(&self) -> bool {
        self.wide_seconds > 0.0
    }
    /// Whether Slow is active.
    pub fn slow(&self) -> bool {
        self.slow_seconds > 0.0
    }
    /// Counts the timers down by one tick; true when Slow has just run out.
    pub(crate) fn tick(&mut self) -> bool {
        self.wide_seconds = (self.wide_seconds - DT).max(0.0);
        let was_slow = self.slow();
        self.slow_seconds = (self.slow_seconds - DT).max(0.0);
        was_slow && self.slow_seconds == 0.0
    }
}
