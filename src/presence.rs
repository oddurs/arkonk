//! What friends see in their Steam list, in their own language. The game
//! sends only tokens and keys; Steam fills in the text from the localization
//! files in `docs/steam/rich_presence/`, one per language, which a test
//! writes from the string tables and keeps current.
use ark::{Game, Mode, sectors::SectorId};
#[cfg(test)]
use ark_text::{Locale, TextId};
#[cfg(test)]
use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Presence {
    Menus,
    Journey(SectorId),
    Practice(SectorId),
}
impl Presence {
    pub fn of(playing: bool, game: &Game) -> Self {
        match (playing, game.mode()) {
            (false, _) => Self::Menus,
            (true, Mode::Journey) => Self::Journey(game.sector()),
            (true, Mode::Practice) => Self::Practice(game.sector()),
        }
    }
    /// The `steam_display` token.
    pub fn token(self) -> &'static str {
        match self {
            Self::Menus => "#Menus",
            Self::Journey(_) => "#Journey",
            Self::Practice(_) => "#Practice",
        }
    }
    /// The `sector` and `name` keys, e.g. `("03", "slipstream")`. The name
    /// is the sector's slug: each friend's Steam client looks up the
    /// `#Sector_<slug>` token in its own language.
    pub fn sector(self) -> Option<(String, &'static str)> {
        let (Self::Journey(sector) | Self::Practice(sector)) = self else {
            return None;
        };
        Some((format!("{:02}", sector.index() + 1), sector.sector().slug))
    }
}

#[cfg(test)]
/// Text as a quoted VDF value: no break marks, quotes and backslashes escaped.
fn quoted(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars().filter(|&c| c != '\u{200B}') {
        if matches!(c, '"' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// The rich presence localization file for `locale`; `None` for the pseudo
/// locale, which Steam does not know. Only the test that writes and checks
/// the committed files needs it.
#[cfg(test)]
fn file(locale: Locale) -> Option<String> {
    let language = locale.steam_name()?;
    // The table's slots become Steam's: `{0}` the sector key, `{1}` the
    // name token that the name key picks, in the friend's language.
    let line = |id: TextId| {
        ark_text::template(locale, id)
            .replace("{0}", "%sector%")
            .replace("{1}", "{#Sector_%name%}")
    };
    let mut out = String::new();
    let _ = writeln!(
        out,
        "\"lang\"\n{{\n\t\"Language\"\t\"{language}\"\n\t\"Tokens\"\n\t{{"
    );
    for (token, id) in [
        ("#Menus", TextId::PresenceMenus),
        ("#Journey", TextId::PresenceJourney),
        ("#Practice", TextId::PresencePractice),
    ] {
        let _ = writeln!(out, "\t\t\"{token}\"\t{}", quoted(&line(id)));
    }
    for sector in SectorId::all() {
        let name = ark_text::template(locale, TextId::SectorName(sector));
        let slug = sector.sector().slug;
        let _ = writeln!(out, "\t\t\"#Sector_{slug}\"\t{}", quoted(name));
    }
    let _ = writeln!(out, "\t}}\n}}");
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    #[test]
    fn menus_and_sectors_read_naturally() {
        let mut game = Game::start(SectorId::clamped(2), Mode::Journey);
        assert_eq!(Presence::of(false, &game), Presence::Menus);
        assert_eq!(Presence::Menus.sector(), None);
        let journey = Presence::of(true, &game);
        assert_eq!(journey, Presence::Journey(SectorId::clamped(2)));
        assert_eq!(journey.sector(), Some(("03".into(), "slipstream")));
        game = Game::start(SectorId::FIRST, Mode::Practice);
        let practice = Presence::of(true, &game);
        assert_eq!(practice, Presence::Practice(SectorId::FIRST));
        assert_eq!(practice.sector(), Some(("01".into(), "first_light")));
    }

    /// The committed files must match the tables, so a translation change
    /// cannot leave Steam showing old text. `ARKONK_WRITE_PRESENCE=1` rewrites
    /// them.
    #[test]
    fn localization_files_match_the_tables() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/steam/rich_presence");
        let write = std::env::var_os("ARKONK_WRITE_PRESENCE").is_some();
        let mut stale = Vec::new();
        for locale in Locale::ALL {
            let Some(expected) = file(locale) else {
                continue;
            };
            let path = dir.join(format!("{}.vdf", locale.steam_name().unwrap()));
            if write {
                fs::create_dir_all(&dir).unwrap();
                fs::write(&path, &expected).unwrap();
            } else if fs::read_to_string(&path).ok().as_deref() != Some(expected.as_str()) {
                stale.push(path.display().to_string());
            }
        }
        assert!(
            stale.is_empty(),
            "out of date; rerun with ARKONK_WRITE_PRESENCE=1: {stale:?}"
        );
    }

    #[test]
    fn every_file_defines_every_token_the_game_sends() {
        let first = SectorId::FIRST;
        for locale in Locale::ALL {
            let Some(vdf) = file(locale) else { continue };
            for p in [
                Presence::Menus,
                Presence::Journey(first),
                Presence::Practice(first),
            ] {
                assert!(
                    vdf.contains(&format!("\"{}\"", p.token())),
                    "{locale:?} {p:?}"
                );
            }
            for sector in SectorId::all() {
                let token = format!("\"#Sector_{}\"", sector.sector().slug);
                assert!(vdf.contains(&token), "{locale:?} {token}");
            }
            assert!(vdf.contains("%sector%") && vdf.contains("{#Sector_%name%}"));
            assert!(!vdf.contains('\u{200B}'), "{locale:?}");
        }
    }
}
