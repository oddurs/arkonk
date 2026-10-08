//! Player settings, saved in the same file as progress.
use ark::progress::Entry;
use ark_text::{Locale, TextId};
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
    /// Effects: no glow, breathing or pulsing, softer flashes, fewer shards.
    pub reduced_effects: bool,
    /// Contrast: untinted glass, full-hue rims, lifted dim hues.
    pub high_contrast: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            muted: false,
            volume: 6,
            fullscreen: false,
            locale: None,
            reduced_effects: false,
            high_contrast: false,
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
                ("effects", [reduced]) => s.reduced_effects = *reduced == 1,
                ("contrast", [high]) => s.high_contrast = *high == 1,
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
        // byte-identical to earlier versions, which skip these lines.
        if let Some(locale) = self.locale {
            writeln!(out, "locale {}", locale.tag())?;
        }
        if self.reduced_effects {
            writeln!(out, "effects 1")?;
        }
        if self.high_contrast {
            writeln!(out, "contrast 1")?;
        }
        Ok(())
    }
}

/// One row of the Settings sheet. The sheet lists [`ROWS`] in order, so a
/// new setting is a variant here, a row there, and its arms below.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Row {
    Sound,
    Volume,
    Display,
    Language,
    Effects,
    Contrast,
}

/// The Settings sheet, top to bottom.
pub const ROWS: &[Row] = &[
    Row::Sound,
    Row::Volume,
    Row::Display,
    Row::Language,
    Row::Effects,
    Row::Contrast,
];

/// How a row shows its value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    /// A switch, on or off.
    Toggle(bool),
    /// A level from 0 to its maximum.
    Level(u8, u8),
    /// A word from the string tables.
    Text(TextId),
    /// A language's own name, never translated.
    Native(&'static str),
}

impl Row {
    pub const fn name(self) -> TextId {
        match self {
            Row::Sound => TextId::SettingSound,
            Row::Volume => TextId::SettingVolume,
            Row::Display => TextId::SettingDisplay,
            Row::Language => TextId::SettingLanguage,
            Row::Effects => TextId::SettingEffects,
            Row::Contrast => TextId::SettingContrast,
        }
    }
    /// What the help line calls changing it: a choice from a list is
    /// selected, a level or switch adjusted.
    pub const fn verb(self) -> TextId {
        match self {
            Row::Language => TextId::ActionSelect,
            _ => TextId::ActionAdjust,
        }
    }
    pub fn value(self, s: &Settings) -> Value {
        match self {
            Row::Sound => Value::Toggle(!s.muted),
            Row::Volume => Value::Level(s.volume, MAX_VOLUME),
            Row::Display => Value::Text(if s.fullscreen {
                TextId::Fullscreen
            } else {
                TextId::DisplayWindow
            }),
            Row::Language => match s.locale {
                Some(locale) => Value::Native(locale.native_name()),
                None => Value::Text(TextId::LanguageSystem),
            },
            Row::Effects => Value::Text(if s.reduced_effects {
                TextId::EffectsReduced
            } else {
                TextId::LookStandard
            }),
            Row::Contrast => Value::Text(if s.high_contrast {
                TextId::ContrastHigh
            } else {
                TextId::LookStandard
            }),
        }
    }
    /// Moves the value one step forward or back; switches and lists wrap,
    /// levels stop at their ends. Applied live by the caller.
    pub fn step(self, s: &mut Settings, forward: bool, drawable: impl Fn(Locale) -> bool) {
        match self {
            Row::Sound => s.muted = !s.muted,
            Row::Display => s.fullscreen = !s.fullscreen,
            Row::Effects => s.reduced_effects = !s.reduced_effects,
            Row::Contrast => s.high_contrast = !s.high_contrast,
            Row::Volume if forward => s.volume = (s.volume + 1).min(MAX_VOLUME),
            Row::Volume => s.volume = s.volume.saturating_sub(1),
            Row::Language => {
                // Following Steam or the system comes first; the test
                // locale is never offered.
                let choices: Vec<Option<Locale>> = std::iter::once(None)
                    .chain(
                        Locale::ALL
                            .into_iter()
                            .filter(|&l| l != Locale::Pseudo && drawable(l))
                            .map(Some),
                    )
                    .collect();
                let at = choices.iter().position(|&c| c == s.locale).unwrap_or(0);
                let n = choices.len();
                s.locale = choices[if forward {
                    (at + 1) % n
                } else {
                    (at + n - 1) % n
                }];
            }
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
    fn effects_and_contrast_round_trip_and_old_saves_keep_the_defaults() {
        // A save from before these settings reads as standard.
        let old = decode(b"ARKONK 1\nsettings 0 6\ndisplay 1\nlocale de\n");
        assert!(!old.reduced_effects && !old.high_contrast);
        assert!(old.fullscreen);
        // Standard writes nothing new, so such a save stays byte-identical.
        let mut file = String::from("ARKONK 1\n");
        Settings::default().encode(&mut file).unwrap();
        assert!(!file.contains("effects") && !file.contains("contrast"));
        for (reduced_effects, high_contrast) in [(true, false), (false, true), (true, true)] {
            let s = Settings {
                reduced_effects,
                high_contrast,
                ..Settings::default()
            };
            let mut file = String::from("ARKONK 1\n");
            s.encode(&mut file).unwrap();
            assert_eq!(decode(file.as_bytes()), s, "{file}");
        }
        // Damaged lines are no choice.
        let s = decode(b"ARKONK 1\neffects 7\ncontrast\n");
        assert!(!s.reduced_effects && !s.high_contrast);
    }

    #[test]
    fn effects_and_contrast_rows_switch() {
        let mut s = Settings::default();
        assert_eq!(Row::Effects.value(&s), Value::Text(TextId::LookStandard));
        assert_eq!(Row::Contrast.value(&s), Value::Text(TextId::LookStandard));
        Row::Effects.step(&mut s, true, |_| true);
        Row::Contrast.step(&mut s, false, |_| true);
        assert_eq!(Row::Effects.value(&s), Value::Text(TextId::EffectsReduced));
        assert_eq!(Row::Contrast.value(&s), Value::Text(TextId::ContrastHigh));
        assert!(s.reduced_effects && s.high_contrast);
        Row::Effects.step(&mut s, false, |_| true);
        assert!(!s.reduced_effects);
    }

    #[test]
    fn rows_step_their_values() {
        let mut s = Settings::default();
        Row::Sound.step(&mut s, true, |_| true);
        assert_eq!(Row::Sound.value(&s), Value::Toggle(false));
        for _ in 0..20 {
            Row::Volume.step(&mut s, true, |_| true);
        }
        assert_eq!(Row::Volume.value(&s), Value::Level(MAX_VOLUME, MAX_VOLUME));
        Row::Volume.step(&mut s, false, |_| true);
        assert_eq!(s.volume, MAX_VOLUME - 1);
        Row::Display.step(&mut s, true, |_| true);
        assert_eq!(Row::Display.value(&s), Value::Text(TextId::Fullscreen));
    }

    #[test]
    fn languages_cycle_from_system_through_what_can_be_drawn() {
        let mut s = Settings::default();
        assert_eq!(Row::Language.value(&s), Value::Text(TextId::LanguageSystem));
        // A build without the `scripts` atlases: Latin, Greek and Cyrillic.
        let latin = |l: Locale| l.script() == ark_text::Script::Alphabetic;
        let mut seen = Vec::new();
        for _ in 0..28 {
            Row::Language.step(&mut s, true, latin);
            seen.push(s.locale);
        }
        // Twenty-four drawable languages, then back to System.
        assert_eq!(seen[0], Some(Locale::En));
        assert_eq!(seen[24], None);
        assert!(!seen.contains(&Some(Locale::Pseudo)));
        assert!(!seen.contains(&Some(Locale::Ja)));
        assert!(!seen.contains(&Some(Locale::Ar)));
        // Back from System wraps to the last language.
        s.locale = None;
        Row::Language.step(&mut s, false, latin);
        assert_eq!(s.locale, Some(Locale::Vi));
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
