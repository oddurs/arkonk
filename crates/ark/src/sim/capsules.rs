//! Falling capsules, and the director that decides when one drops and which.
use super::{power::Power, rng::Rng};
use crate::{
    geom::V2,
    sectors::SectorId,
    tuning::{DROP_CHANCE, DROP_DRY_SPELL, DROP_MIN_GAP, DROP_ORDER, OPENING_BREAK, POWER_UNLOCKS},
};

/// A capsule slot.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Capsule {
    /// Centre.
    pub pos: V2,
    /// What catching it grants.
    pub power: Power,
    /// Whether the slot holds a falling capsule.
    pub active: bool,
}

/// Drop cadence for one sector. Each sector teaches its opening capsule
/// first; after that, random drops never leave a long dry spell, and later
/// sectors unlock more of the list.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DropDirector {
    /// Bricks broken by a ball this sector.
    direct_breaks: u32,
    /// Bricks broken by a ball since the last capsule appeared.
    since_drop: u32,
    /// Whether the opening capsule has been caught.
    opening_collected: bool,
}

impl DropDirector {
    /// A director for a fresh sector.
    pub(crate) const fn new() -> Self {
        Self {
            direct_breaks: 0,
            since_drop: 0,
            opening_collected: false,
        }
    }

    /// Called when a ball breaks a brick; returns the capsule to drop, if
    /// any. Relay blasts never call it, so a chain cannot rain capsules.
    pub(crate) fn on_break(&mut self, rng: &mut Rng, sector: SectorId) -> Option<Power> {
        self.direct_breaks += 1;
        self.since_drop += 1;
        let due = self.direct_breaks == OPENING_BREAK
            || self.since_drop >= DROP_DRY_SPELL
            || (self.since_drop >= DROP_MIN_GAP && rng.next_f32() < DROP_CHANCE);
        if !due {
            return None;
        }
        Some(if self.opening_collected {
            let count = POWER_UNLOCKS[sector.index()];
            DROP_ORDER[((rng.next_f32() * count as f32) as usize).min(count - 1)]
        } else {
            sector.sector().opening
        })
    }

    /// A capsule found a free slot and is falling.
    pub(crate) fn dropped(&mut self) {
        self.since_drop = 0;
    }

    /// `power` was caught in `sector`.
    pub(crate) fn collected(&mut self, power: Power, sector: SectorId) {
        if power == sector.sector().opening {
            self.opening_collected = true;
        }
    }
}
