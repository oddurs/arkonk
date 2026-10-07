//! Player settings, saved in the same file as progress.
use ark::progress::Entry;
use std::fmt;

/// The loudest volume step.
pub const MAX_VOLUME: u8 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub muted: bool,
    /// `0..=MAX_VOLUME`.
    pub volume: u8,
    /// Restored at launch; the app loop applies changes on the next frame.
    pub fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            muted: false,
            volume: 6,
            fullscreen: false,
        }
    }
}

impl Settings {
    /// Reads the settings lines from a save's entries; other keys belong to
    /// progress and are ignored here.
    pub fn decode<'a>(entries: impl Iterator<Item = Entry<'a>>) -> Self {
        let mut s = Self::default();
        for entry in entries {
            match (entry.key(), entry.values()) {
                // Older saves carry a retired display flag between the two.
                ("settings", [mute, volume] | [mute, _, volume]) => {
                    s.muted = *mute != 0;
                    s.volume = (*volume).min(u32::from(MAX_VOLUME)) as u8;
                }
                ("display", [fullscreen]) => s.fullscreen = *fullscreen == 1,
                _ => {}
            }
        }
        s
    }

    /// Writes the settings lines.
    pub fn encode(&self, out: &mut impl fmt::Write) -> fmt::Result {
        writeln!(out, "settings {} {}", u8::from(self.muted), self.volume)?;
        writeln!(out, "display {}", u8::from(self.fullscreen))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark::progress::entries;

    fn decode(file: &[u8]) -> Settings {
        Settings::decode(entries(file).unwrap())
    }

    #[test]
    fn legacy_settings_keep_sound_and_volume() {
        let s = decode(b"ARKONK 1\nsettings 1 0 4\n");
        assert!(s.muted);
        assert_eq!(s.volume, 4);
    }

    #[test]
    fn volume_is_clamped_and_display_needs_exactly_one() {
        let s = decode(b"ARKONK 1\nsettings 0 1 300\ndisplay 2\n");
        assert_eq!(s.volume, MAX_VOLUME);
        assert!(!s.fullscreen);
    }
}
