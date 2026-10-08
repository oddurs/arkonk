//! The languages ARKONK ships, and how a BCP 47 tag or a Steam language
//! name chooses one.

/// A shipped language. English is the source; every other table is a
/// translation of it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Locale {
    #[default]
    En,
    Fr,
    De,
    /// Spanish as written in Spain.
    EsEs,
    /// Latin American Spanish (Steam's "latam").
    Es419,
    /// Brazilian Portuguese; also chosen for other Portuguese.
    PtBr,
    It,
    Pl,
    Ru,
    /// Simplified Chinese.
    ZhHans,
    Ja,
    Ko,
    /// English, accented, bracketed and about 40 % longer, so untranslated,
    /// clipped or concatenated text stands out. For testing only.
    Pseudo,
}

/// How a script lays out, as far as the UI cares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Script {
    /// Latin or Cyrillic: words separated by spaces, cased.
    Alphabetic,
    /// Chinese or Japanese: no spaces; a line may break between characters.
    Ideographic,
    /// Korean: spaces between words, no case.
    Hangul,
}

impl Locale {
    /// Every locale, the source first and the test locale last.
    pub const ALL: [Self; 13] = [
        Self::En,
        Self::Fr,
        Self::De,
        Self::EsEs,
        Self::Es419,
        Self::PtBr,
        Self::It,
        Self::Pl,
        Self::Ru,
        Self::ZhHans,
        Self::Ja,
        Self::Ko,
        Self::Pseudo,
    ];

    /// The canonical BCP 47 tag, as saved in settings. `en-XA` is the
    /// conventional pseudo-locale tag.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
            Self::De => "de",
            Self::EsEs => "es-ES",
            Self::Es419 => "es-419",
            Self::PtBr => "pt-BR",
            Self::It => "it",
            Self::Pl => "pl",
            Self::Ru => "ru",
            Self::ZhHans => "zh-Hans",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Pseudo => "en-XA",
        }
    }

    /// The language's name in itself, for a language picker. Never translated.
    pub const fn native_name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Fr => "Français",
            Self::De => "Deutsch",
            Self::EsEs => "Español (España)",
            Self::Es419 => "Español (Latinoamérica)",
            Self::PtBr => "Português (Brasil)",
            Self::It => "Italiano",
            Self::Pl => "Polski",
            Self::Ru => "Русский",
            Self::ZhHans => "简体中文",
            Self::Ja => "日本語",
            Self::Ko => "한국어",
            Self::Pseudo => "[Ƥśéúðö]",
        }
    }

    pub const fn script(self) -> Script {
        match self {
            Self::ZhHans | Self::Ja => Script::Ideographic,
            Self::Ko => Script::Hangul,
            _ => Script::Alphabetic,
        }
    }

    /// The best shipped match for a BCP 47 tag or a POSIX locale name
    /// (`de-AT`, `zh-Hans-CN`, `pt_PT.UTF-8`). `None` when no shipped
    /// language fits, such as Traditional Chinese.
    pub fn from_tag(tag: &str) -> Option<Self> {
        // POSIX names carry an encoding or modifier after the region.
        let tag = tag.split(['.', '@']).next().unwrap_or(tag);
        let mut parts = tag.split(['-', '_']).map(Lower);
        let language = parts.next()?;
        let mut script = None;
        let mut region = None;
        for part in parts {
            match part.len() {
                4 if script.is_none() && region.is_none() => script = Some(part),
                2 | 3 if region.is_none() => region = Some(part),
                _ => {}
            }
        }
        let is = |part: Option<Lower<'_>>, s: &str| part.is_some_and(|p| p.eq(s));
        Some(match language {
            l if l.eq("en") => {
                if is(region, "xa") {
                    Self::Pseudo
                } else {
                    Self::En
                }
            }
            l if l.eq("pseudo") => Self::Pseudo,
            l if l.eq("fr") => Self::Fr,
            l if l.eq("de") => Self::De,
            l if l.eq("es") => match region {
                Some(r) if LATIN_AMERICA.iter().any(|&code| r.eq(code)) => Self::Es419,
                _ => Self::EsEs,
            },
            l if l.eq("pt") => Self::PtBr,
            l if l.eq("it") => Self::It,
            l if l.eq("pl") => Self::Pl,
            l if l.eq("ru") => Self::Ru,
            l if l.eq("zh") => {
                let traditional = is(script, "hant")
                    || (script.is_none() && ["tw", "hk", "mo"].iter().any(|&r| is(region, r)));
                if traditional {
                    return None;
                }
                Self::ZhHans
            }
            l if l.eq("ja") => Self::Ja,
            l if l.eq("ko") => Self::Ko,
            _ => return None,
        })
    }

    /// The locale for a Steam language API name (`ISteamApps::GetCurrentGameLanguage`).
    pub fn from_steam(language: &str) -> Option<Self> {
        Some(match language {
            "english" => Self::En,
            "french" => Self::Fr,
            "german" => Self::De,
            "spanish" => Self::EsEs,
            "latam" => Self::Es419,
            "brazilian" | "portuguese" => Self::PtBr,
            "italian" => Self::It,
            "polish" => Self::Pl,
            "russian" => Self::Ru,
            "schinese" => Self::ZhHans,
            "japanese" => Self::Ja,
            "koreana" => Self::Ko,
            _ => return None,
        })
    }

    /// The thousands separator and the fewest integer digits that get one:
    /// CLDR's `minimumGroupingDigits` is 2 for Spanish and Polish, so they
    /// write 4-digit numbers ungrouped.
    pub(crate) const fn grouping(self) -> (char, usize) {
        match self {
            Self::En | Self::Es419 | Self::ZhHans | Self::Ja | Self::Ko | Self::Pseudo => (',', 4),
            // CLDR: narrow no-break space.
            Self::Fr => ('\u{202F}', 4),
            Self::De | Self::PtBr | Self::It => ('.', 4),
            Self::EsEs => ('.', 5),
            Self::Pl => ('\u{A0}', 5),
            Self::Ru => ('\u{A0}', 4),
        }
    }
}

/// Region subtags that read Latin American Spanish, plus the UN M.49 code.
const LATIN_AMERICA: [&str; 21] = [
    "419", "ar", "bo", "cl", "co", "cr", "cu", "do", "ec", "gt", "hn", "mx", "ni", "pa", "pe",
    "pr", "py", "sv", "us", "uy", "ve",
];

/// A tag subtag compared without regard to ASCII case, without allocating.
#[derive(Clone, Copy)]
struct Lower<'a>(&'a str);
impl Lower<'_> {
    fn eq(self, lower: &str) -> bool {
        self.0.eq_ignore_ascii_case(lower)
    }
    fn len(self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_round_trip() {
        for locale in Locale::ALL {
            assert_eq!(Locale::from_tag(locale.tag()), Some(locale), "{locale:?}");
        }
    }

    #[test]
    fn os_and_posix_tags_negotiate() {
        for (tag, expected) in [
            ("en-US", Some(Locale::En)),
            ("de_AT.UTF-8", Some(Locale::De)),
            ("es-MX", Some(Locale::Es419)),
            ("es_ES", Some(Locale::EsEs)),
            ("es", Some(Locale::EsEs)),
            ("pt-PT", Some(Locale::PtBr)),
            ("zh-Hans-CN", Some(Locale::ZhHans)),
            ("zh-CN", Some(Locale::ZhHans)),
            ("zh-TW", None),
            ("zh-Hant-HK", None),
            ("ja-JP", Some(Locale::Ja)),
            ("KO-kr", Some(Locale::Ko)),
            ("pseudo", Some(Locale::Pseudo)),
            ("tr-TR", None),
            ("", None),
            ("C", None),
        ] {
            assert_eq!(Locale::from_tag(tag), expected, "{tag}");
        }
    }

    #[test]
    fn steam_languages_map() {
        assert_eq!(Locale::from_steam("latam"), Some(Locale::Es419));
        assert_eq!(Locale::from_steam("koreana"), Some(Locale::Ko));
        assert_eq!(Locale::from_steam("tchinese"), None);
    }
}
