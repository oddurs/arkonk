//! The status grammar as timing: durations drain toward their centre, the
//! last two seconds of one pulse, and events flash and are gone within
//! 180 ms. Pure functions of time, so the rules are tested without a window.
use std::f32::consts::TAU;

/// How long before a duration ends it starts to pulse, in seconds.
pub(super) const WARN: f32 = 2.0;
/// Pulses a second while a duration runs out.
const PULSE_HZ: f32 = 4.0;
/// The faintest a running-out drain gets.
const PULSE_LOW: f32 = 0.35;
/// The longest any event flash lasts.
pub(super) const FLASH_MAX: f32 = 0.18;
/// Reduced effects: every flash at this strength, and no longer than
/// [`REDUCED_LENGTH`].
const REDUCED_STRENGTH: f32 = 0.4;
const REDUCED_LENGTH: f32 = 0.12;

/// How long a drain `full` long is with `left` of `total` seconds to go:
/// the whole line at the start, nothing at the end, shrinking evenly.
pub(super) fn drain(left: f32, total: f32, full: f32) -> f32 {
    if total > 0.0 {
        full * (left / total).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Whether a duration with `left` seconds to go is in its last two.
pub(super) fn running_out(left: f32) -> bool {
    left > 0.0 && left <= WARN
}

/// A drain's opacity with `left` seconds to go: steady, then between full
/// and 35 % four times a second through its last two seconds, starting
/// full. Reduced effects hold it steady; the caller turns it white instead.
pub(super) fn pulse(left: f32, reduced: bool) -> f32 {
    if reduced || !running_out(left) {
        return 1.0;
    }
    let wave = 0.5 + 0.5 * (TAU * PULSE_HZ * (WARN - left)).cos();
    PULSE_LOW + (1.0 - PULSE_LOW) * wave
}

/// An event flash: how long it lasts and how much of that it holds at
/// full before fading out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Flash {
    pub length: f32,
    pub hold: f32,
}
impl Flash {
    /// A brick hit: rim to white for 60 ms, eased back by 120 ms.
    pub const HIT: Self = Self::new(0.12, 0.06);
    /// A broken brick's rim lifting off as a ring.
    pub const BREAK: Self = Self::new(FLASH_MAX, 0.0);
    /// A ball off the paddle: the pearl to white, the keel wide.
    pub const CONTACT: Self = Self::new(0.16, 0.0);
    /// A capsule caught: the keel in the power's hue. The design asks
    /// 200 ms; the grammar caps every flash at 180.
    pub const CATCH: Self = Self::new(FLASH_MAX, 0.0);
    /// A bounce lighting the wall near it.
    pub const WALL: Self = Self::new(0.12, 0.0);
    /// A relay core going off: its rim lit, a ring and sparks, over the
    /// simulation's 150 ms blast.
    pub const IGNITE: Self = Self::new(0.15, 0.0);

    const fn new(length: f32, hold: f32) -> Self {
        Self { length, hold }
    }
    /// How long this flash lasts under the player's effects setting.
    pub fn length(self, reduced: bool) -> f32 {
        if reduced {
            self.length.min(REDUCED_LENGTH)
        } else {
            self.length
        }
    }
    /// Its strength `elapsed` seconds after the event: full through the
    /// hold, then falling evenly to nothing at its end. Reduced effects
    /// play it at 40 %, over at most 120 ms.
    pub fn at(self, elapsed: f32, reduced: bool) -> f32 {
        let length = self.length(reduced);
        if !(0.0..length).contains(&elapsed) {
            return 0.0;
        }
        let hold = self.hold * length / self.length;
        let strength = if reduced { REDUCED_STRENGTH } else { 1.0 };
        if elapsed < hold {
            strength
        } else {
            strength * (1.0 - (elapsed - hold) / (length - hold))
        }
    }
    /// How far through it is, from 0 to 1, for what grows as it fades.
    pub fn progress(self, elapsed: f32, reduced: bool) -> f32 {
        (elapsed / self.length(reduced)).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Flash; 6] = [
        Flash::HIT,
        Flash::BREAK,
        Flash::CONTACT,
        Flash::CATCH,
        Flash::WALL,
        Flash::IGNITE,
    ];

    #[test]
    fn a_drain_closes_evenly_to_nothing() {
        let full = 94.0;
        assert_eq!(drain(14.0, 14.0, full), full);
        assert_eq!(drain(7.0, 14.0, full), full / 2.0);
        assert_eq!(drain(0.0, 14.0, full), 0.0);
        // A fresh grant can sit a tick over its total; it never overflows.
        assert_eq!(drain(14.1, 14.0, full), full);
        assert_eq!(drain(3.0, 0.0, full), 0.0);
    }

    #[test]
    fn only_the_last_two_seconds_pulse() {
        assert_eq!(pulse(2.01, false), 1.0);
        assert_eq!(pulse(12.0, false), 1.0);
        assert!(running_out(2.0) && running_out(0.01));
        assert!(!running_out(0.0) && !running_out(2.01));
        // It starts at full, dips to 35 % an eighth of a second later, and
        // is back four times a second.
        assert!((pulse(2.0, false) - 1.0).abs() < 1e-4);
        assert!((pulse(2.0 - 0.125, false) - PULSE_LOW).abs() < 1e-4);
        assert!((pulse(1.75, false) - 1.0).abs() < 1e-4);
        for k in 0..200 {
            let a = pulse(k as f32 / 100.0, false);
            assert!((PULSE_LOW - 1e-4..=1.0 + 1e-4).contains(&a));
        }
        // Reduced effects never pulse.
        assert_eq!(pulse(1.875, true), 1.0);
    }

    #[test]
    fn every_flash_is_gone_within_180_ms() {
        for f in ALL {
            assert!(f.length <= FLASH_MAX, "{f:?}");
            assert!(f.at(0.0, false) > 0.99, "{f:?} starts full");
            assert_eq!(f.at(f.length, false), 0.0, "{f:?}");
            assert_eq!(f.at(1.0, false), 0.0);
            assert_eq!(f.at(-0.01, false), 0.0);
        }
        // A hit holds white for 60 ms and eases back by 120.
        assert_eq!(Flash::HIT.at(0.059, false), 1.0);
        assert!((Flash::HIT.at(0.09, false) - 0.5).abs() < 1e-4);
        assert_eq!(Flash::HIT.at(0.12, false), 0.0);
    }

    #[test]
    fn reduced_effects_flash_at_40_percent_for_120_ms() {
        for f in ALL {
            assert!(f.length(true) <= REDUCED_LENGTH);
            assert!((f.at(0.0, true) - REDUCED_STRENGTH).abs() < 1e-6);
            assert_eq!(f.at(REDUCED_LENGTH, true), 0.0, "{f:?}");
            for k in 0..20 {
                assert!(f.at(k as f32 / 100.0, true) <= REDUCED_STRENGTH);
            }
        }
        assert_eq!(Flash::BREAK.progress(0.06, true), 0.5);
    }
}
