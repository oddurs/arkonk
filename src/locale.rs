//! Which language the game speaks: an explicit `--locale`, then the player's
//! saved choice, then Steam's game language, then the operating system's
//! preferences, then English. A candidate this build cannot draw (CJK, Thai
//! or Arabic without the `scripts` feature) is skipped.
use ark_text::Locale;

/// Where the chosen locale came from, for the log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Flag,
    Setting,
    Steam,
    System,
    Default,
}

/// Picks the first usable locale. `system` lists the OS preferences as
/// BCP 47 tags, most preferred first.
pub fn choose(
    flag: Option<Locale>,
    setting: Option<Locale>,
    steam: Option<Locale>,
    system: impl IntoIterator<Item = String>,
) -> (Locale, Origin) {
    let usable = |l: &Locale| ark_glyphs::supports(*l);
    flag.filter(usable)
        .map(|l| (l, Origin::Flag))
        .or_else(|| setting.filter(usable).map(|l| (l, Origin::Setting)))
        .or_else(|| steam.filter(usable).map(|l| (l, Origin::Steam)))
        .or_else(|| {
            system
                .into_iter()
                .filter_map(|tag| Locale::from_tag(&tag))
                // The pseudo locale is for testing; only `--locale` reaches it.
                .find(|l| usable(l) && *l != Locale::Pseudo)
                .map(|l| (l, Origin::System))
        })
        .unwrap_or((Locale::En, Origin::Default))
}

/// The operating system's preferred languages, most preferred first.
pub fn system() -> impl Iterator<Item = String> {
    sys_locale::get_locales()
}

/// The value of `--locale <tag>`, if given. An unknown tag is logged and
/// ignored rather than stopping the game.
pub fn flag() -> Option<Locale> {
    let mut args = std::env::args().skip_while(|a| a != "--locale").skip(1);
    let tag = args.next()?;
    let locale = Locale::from_tag(&tag);
    if locale.is_none() {
        crate::diagnostics::error(format_args!("Unknown --locale {tag:?}; ignoring it"));
    }
    locale
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn earlier_sources_win() {
        let os = tags(&["fr-CA"]);
        assert_eq!(
            choose(
                Some(Locale::Pseudo),
                Some(Locale::De),
                Some(Locale::It),
                os.clone()
            ),
            (Locale::Pseudo, Origin::Flag)
        );
        assert_eq!(
            choose(None, Some(Locale::De), Some(Locale::It), os.clone()),
            (Locale::De, Origin::Setting)
        );
        assert_eq!(
            choose(None, None, Some(Locale::It), os.clone()),
            (Locale::It, Origin::Steam)
        );
        assert_eq!(choose(None, None, None, os), (Locale::Fr, Origin::System));
    }

    #[test]
    fn system_preferences_skip_unshipped_languages() {
        let os = tags(&["tlh", "zu-ZA", "pt-PT", "en-US"]);
        assert_eq!(choose(None, None, None, os), (Locale::PtPt, Origin::System));
        let os = tags(&["zh-Hant-TW", "en-US"]);
        let expected = if ark_glyphs::supports(Locale::ZhHant) {
            Locale::ZhHant
        } else {
            Locale::En
        };
        assert_eq!(choose(None, None, None, os).0, expected);
        assert_eq!(
            choose(None, None, None, tags(&["en-XA"])),
            (Locale::En, Origin::Default)
        );
        assert_eq!(
            choose(None, None, None, Vec::new()),
            (Locale::En, Origin::Default)
        );
    }

    #[test]
    fn locales_this_build_cannot_draw_are_skipped() {
        let (locale, _) = choose(Some(Locale::Ja), None, None, tags(&["de"]));
        if ark_glyphs::supports(Locale::Ja) {
            assert_eq!(locale, Locale::Ja);
        } else {
            assert_eq!(locale, Locale::De);
        }
    }
}
