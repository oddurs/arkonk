//! What friends see in their Steam list. Tokens must match the localization file
//! uploaded to Steam, `docs/steam/rich_presence.vdf`.
use ark::{Game, Mode, sectors::SectorId};

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
    /// The `%sector%` and `%name%` substitutions, e.g. `("03", "Slipstream")`.
    /// The uploaded presence strings are English only, so the name is too.
    pub fn sector(self) -> Option<(String, String)> {
        let (Self::Journey(sector) | Self::Practice(sector)) = self else {
            return None;
        };
        let name = ark_text::template(ark_text::Locale::En, ark_text::TextId::SectorName(sector));
        Some((format!("{:02}", sector.index() + 1), name.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menus_and_sectors_read_naturally() {
        let mut game = Game::start(SectorId::clamped(2), Mode::Journey);
        assert_eq!(Presence::of(false, &game), Presence::Menus);
        assert_eq!(Presence::Menus.sector(), None);
        let journey = Presence::of(true, &game);
        assert_eq!(journey, Presence::Journey(SectorId::clamped(2)));
        assert_eq!(journey.sector(), Some(("03".into(), "Slipstream".into())));
        game = Game::start(SectorId::FIRST, Mode::Practice);
        let practice = Presence::of(true, &game);
        assert_eq!(practice, Presence::Practice(SectorId::FIRST));
        assert_eq!(practice.sector(), Some(("01".into(), "First Light".into())));
    }

    #[test]
    fn localization_file_defines_every_token() {
        let vdf = include_str!("../docs/steam/rich_presence.vdf");
        let first = SectorId::FIRST;
        for p in [
            Presence::Menus,
            Presence::Journey(first),
            Presence::Practice(first),
        ] {
            assert!(vdf.contains(&format!("\"{}\"", p.token())), "{p:?}");
        }
        assert!(vdf.contains("%sector%") && vdf.contains("%name%"));
    }
}
