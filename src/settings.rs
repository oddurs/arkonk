//! Player settings, saved in the same file as progress.
use ark::progress::Entry;
use ark_text::Locale;
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
    /// The player's language choice; `None` follows Steam or the system.
    pub locale: Option<Locale>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            muted: false,
            volume: 6,
            fullscreen: false,
            locale: None,
        }
    }
}

impl Settings {
    /// Reads the settings lines from a save's entries and, for the one
    /// line whose value is a word, from the raw `file`; other keys belong
    /// to progress and are ignored here.
    pub fn decode<'a>(entries: impl Iterator<Item = Entry<'a>>, file: &[u8]) -> Self {
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
        s.locale = locale(file);
        s
    }

    /// Writes the settings lines.
    pub fn encode(&self, out: &mut impl fmt::Write) -> fmt::Result {
        writeln!(out, "settings {} {}", u8::from(self.muted), self.volume)?;
        writeln!(out, "display {}", u8::from(self.fullscreen))?;
        // Only an explicit choice is written, so saves without one stay
        // byte-identical to earlier versions, which skip this line.
        match self.locale {
            Some(locale) => writeln!(out, "locale {}", locale.tag()),
            None => Ok(()),
        }
    }
}

/// The last well-formed `locale <bcp47>` line. The progress codec skips it
/// as damaged (its value is not a number), which is why older builds
/// ignore it; an unknown tag reads as no choice.
fn locale(file: &[u8]) -> Option<Locale> {
    file.rsplit(|&b| b == b'\n')
        .find_map(|line| {
            let mut words = std::str::from_utf8(line).ok()?.split_whitespace();
            match (words.next(), words.next(), words.next()) {
                (Some("locale"), Some(tag), None) => Some(Locale::from_tag(tag)),
                _ => None,
            }
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark::progress::entries;

    fn decode(file: &[u8]) -> Settings {
        Settings::decode(entries(file).unwrap(), file)
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

    #[test]
    fn saves_without_a_language_follow_the_system() {
        let s = decode(b"ARKONK 1\nsettings 0 6\ndisplay 0\n");
        assert_eq!(s, Settings::default());
    }

    #[test]
    fn a_chosen_language_round_trips() {
        let s = Settings {
            locale: Some(Locale::PtBr),
            ..Settings::default()
        };
        let mut file = String::from("ARKONK 1\n");
        s.encode(&mut file).unwrap();
        assert!(file.ends_with("locale pt-BR\n"), "{file}");
        assert_eq!(decode(file.as_bytes()), s);
        // The progress codec reads the same file and skips the line.
        assert!(
            entries(file.as_bytes())
                .unwrap()
                .all(|e| e.key() != "locale")
        );
    }

    #[test]
    fn unknown_or_damaged_language_lines_are_no_choice() {
        assert_eq!(decode(b"ARKONK 1\nlocale tlh\n").locale, None);
        assert_eq!(decode(b"ARKONK 1\nlocale\n").locale, None);
        assert_eq!(decode(b"ARKONK 1\nlocale de fr\n").locale, None);
        assert_eq!(decode(b"ARKONK 1\nlocale \xff\n").locale, None);
        assert_eq!(
            decode(b"ARKONK 1\nlocale ja\r\nlocale de\n").locale,
            Some(Locale::De)
        );
    }
}
