//! The languages ARKONK ships, and how a BCP 47 tag or a Steam language
//! name chooses one.

/// A shipped language: every language Steam offers. English is the
/// source; every other table is a translation of it.
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
    /// Arabic: written right to left.
    Ar,
    Bg,
    /// Traditional Chinese, as written in Taiwan.
    ZhHant,
    Cs,
    Da,
    Nl,
    Fi,
    El,
    Hu,
    Id,
    /// Norwegian Bokmål; also chosen for Nynorsk and plain Norwegian.
    Nb,
    /// European Portuguese.
    PtPt,
    Ro,
    Sv,
    Th,
    Tr,
    Uk,
    Vi,
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
    /// Thai: no spaces between words; the tables mark where a line may
    /// break with U+200B ZERO WIDTH SPACE. Vowel and tone marks stack.
    Thai,
    /// Arabic: right to left, letters join their neighbours.
    Arabic,
}

impl Locale {
    /// Every locale, the source first and the test locale last.
    pub const ALL: [Self; 31] = [
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
        Self::Ar,
        Self::Bg,
        Self::ZhHant,
        Self::Cs,
        Self::Da,
        Self::Nl,
        Self::Fi,
        Self::El,
        Self::Hu,
        Self::Id,
        Self::Nb,
        Self::PtPt,
        Self::Ro,
        Self::Sv,
        Self::Th,
        Self::Tr,
        Self::Uk,
        Self::Vi,
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
            Self::Ar => "ar",
            Self::Bg => "bg",
            Self::ZhHant => "zh-Hant",
            Self::Cs => "cs",
            Self::Da => "da",
            Self::Nl => "nl",
            Self::Fi => "fi",
            Self::El => "el",
            Self::Hu => "hu",
            Self::Id => "id",
            Self::Nb => "nb",
            Self::PtPt => "pt-PT",
            Self::Ro => "ro",
            Self::Sv => "sv",
            Self::Th => "th",
            Self::Tr => "tr",
            Self::Uk => "uk",
            Self::Vi => "vi",
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
            Self::Ar => "العربية",
            Self::Bg => "Български",
            Self::ZhHant => "繁體中文",
            Self::Cs => "Čeština",
            Self::Da => "Dansk",
            Self::Nl => "Nederlands",
            Self::Fi => "Suomi",
            Self::El => "Ελληνικά",
            Self::Hu => "Magyar",
            Self::Id => "Bahasa Indonesia",
            Self::Nb => "Norsk",
            Self::PtPt => "Português (Portugal)",
            Self::Ro => "Română",
            Self::Sv => "Svenska",
            Self::Th => "ไทย",
            Self::Tr => "Türkçe",
            Self::Uk => "Українська",
            Self::Vi => "Tiếng Việt",
            Self::Pseudo => "[Ƥśéúðö]",
        }
    }

    pub const fn script(self) -> Script {
        match self {
            Self::ZhHans | Self::ZhHant | Self::Ja => Script::Ideographic,
            Self::Ko => Script::Hangul,
            Self::Th => Script::Thai,
            Self::Ar => Script::Arabic,
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
            l if l.eq("pt") => match region {
                Some(r) if EUROPEAN_PORTUGUESE.iter().any(|&code| r.eq(code)) => Self::PtPt,
                _ => Self::PtBr,
            },
            l if l.eq("it") => Self::It,
            l if l.eq("pl") => Self::Pl,
            l if l.eq("ru") => Self::Ru,
            l if l.eq("zh") => {
                let traditional = is(script, "hant")
                    || (script.is_none() && ["tw", "hk", "mo"].iter().any(|&r| is(region, r)));
                if traditional {
                    Self::ZhHant
                } else {
                    Self::ZhHans
                }
            }
            l if l.eq("ja") => Self::Ja,
            l if l.eq("ko") => Self::Ko,
            l if l.eq("ar") => Self::Ar,
            l if l.eq("bg") => Self::Bg,
            l if l.eq("cs") => Self::Cs,
            l if l.eq("da") => Self::Da,
            l if l.eq("nl") => Self::Nl,
            l if l.eq("fi") => Self::Fi,
            l if l.eq("el") => Self::El,
            l if l.eq("hu") => Self::Hu,
            // "in" is Indonesian's withdrawn code, still reported by old Java and Android.
            l if l.eq("id") || l.eq("in") => Self::Id,
            l if l.eq("nb") || l.eq("no") || l.eq("nn") => Self::Nb,
            l if l.eq("ro") => Self::Ro,
            l if l.eq("sv") => Self::Sv,
            l if l.eq("th") => Self::Th,
            l if l.eq("tr") => Self::Tr,
            l if l.eq("uk") => Self::Uk,
            l if l.eq("vi") => Self::Vi,
            _ => return None,
        })
    }

    /// Steam's API language name (`ISteamApps::GetCurrentGameLanguage`,
    /// and the `Language` of a rich presence file). The pseudo locale has none.
    pub const fn steam_name(self) -> Option<&'static str> {
        Some(match self {
            Self::En => "english",
            Self::Fr => "french",
            Self::De => "german",
            Self::EsEs => "spanish",
            Self::Es419 => "latam",
            Self::PtBr => "brazilian",
            Self::It => "italian",
            Self::Pl => "polish",
            Self::Ru => "russian",
            Self::ZhHans => "schinese",
            Self::Ja => "japanese",
            Self::Ko => "koreana",
            Self::Ar => "arabic",
            Self::Bg => "bulgarian",
            Self::ZhHant => "tchinese",
            Self::Cs => "czech",
            Self::Da => "danish",
            Self::Nl => "dutch",
            Self::Fi => "finnish",
            Self::El => "greek",
            Self::Hu => "hungarian",
            Self::Id => "indonesian",
            Self::Nb => "norwegian",
            Self::PtPt => "portuguese",
            Self::Ro => "romanian",
            Self::Sv => "swedish",
            Self::Th => "thai",
            Self::Tr => "turkish",
            Self::Uk => "ukrainian",
            Self::Vi => "vietnamese",
            Self::Pseudo => return None,
        })
    }

    /// The locale for a Steam language API name.
    pub fn from_steam(language: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|l| l.steam_name() == Some(language))
    }

    /// The thousands separator and the fewest integer digits that get one,
    /// from CLDR (`symbols-numberSystem-latn.group` and
    /// `minimumGroupingDigits`). Where the minimum is 2, as in Spanish,
    /// Italian or Polish, four-digit numbers stay ungrouped. Arabic uses
    /// Latin digits, CLDR's default numbering system for `ar`.
    pub(crate) const fn grouping(self) -> (char, usize) {
        const NBSP: char = '\u{A0}';
        match self {
            Self::En
            | Self::Es419
            | Self::ZhHans
            | Self::ZhHant
            | Self::Ja
            | Self::Ko
            | Self::Ar
            | Self::Th
            | Self::Pseudo => (',', 4),
            // A narrow no-break space.
            Self::Fr => ('\u{202F}', 4),
            Self::De
            | Self::PtBr
            | Self::Da
            | Self::Nl
            | Self::El
            | Self::Id
            | Self::Ro
            | Self::Tr
            | Self::Vi => ('.', 4),
            Self::EsEs | Self::It => ('.', 5),
            Self::Ru | Self::Cs | Self::Fi | Self::Nb | Self::Sv | Self::Uk => (NBSP, 4),
            Self::Pl | Self::PtPt | Self::Bg | Self::Hu => (NBSP, 5),
        }
    }
}

/// Region subtags that read Latin American Spanish, plus the UN M.49 code.
/// Region subtags whose Portuguese follows Portugal's norm rather than Brazil's.
const EUROPEAN_PORTUGUESE: [&str; 10] =
    ["pt", "ao", "mz", "cv", "gw", "st", "tl", "mo", "lu", "gq"];

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
            ("pt-PT", Some(Locale::PtPt)),
            ("pt-AO", Some(Locale::PtPt)),
            ("pt", Some(Locale::PtBr)),
            ("nn-NO", Some(Locale::Nb)),
            ("no", Some(Locale::Nb)),
            ("in-ID", Some(Locale::Id)),
            ("tr-TR", Some(Locale::Tr)),
            ("ar-EG", Some(Locale::Ar)),
            ("zh-Hans-CN", Some(Locale::ZhHans)),
            ("zh-CN", Some(Locale::ZhHans)),
            ("zh-TW", Some(Locale::ZhHant)),
            ("zh-Hant-HK", Some(Locale::ZhHant)),
            ("zh-HK", Some(Locale::ZhHant)),
            ("ja-JP", Some(Locale::Ja)),
            ("KO-kr", Some(Locale::Ko)),
            ("pseudo", Some(Locale::Pseudo)),
            ("tlh", None),
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
        assert_eq!(Locale::from_steam("tchinese"), Some(Locale::ZhHant));
        assert_eq!(Locale::from_steam("portuguese"), Some(Locale::PtPt));
        assert_eq!(Locale::from_steam("brazilian"), Some(Locale::PtBr));
        assert_eq!(Locale::from_steam("klingon"), None);
        // Steam names are distinct, so each maps back to its locale.
        for locale in Locale::ALL {
            if let Some(name) = locale.steam_name() {
                assert_eq!(Locale::from_steam(name), Some(locale), "{name}");
            }
        }
    }
}
