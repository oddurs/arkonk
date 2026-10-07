//! What friends see in their Steam list. Tokens must match the localization file
//! uploaded to Steam, `docs/steam/rich_presence.vdf`.
use ark::{
    game::{Game, Mode},
    levels::LEVELS,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Presence {
    Menus,
    Journey(usize),
    Practice(usize),
}
impl Presence {
    pub fn of(playing: bool, game: &Game) -> Self {
        match (playing, game.mode) {
            (false, _) => Self::Menus,
            (true, Mode::Journey) => Self::Journey(game.level),
            (true, Mode::Practice) => Self::Practice(game.level),
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
    pub fn sector(self) -> Option<(String, String)> {
        let (Self::Journey(level) | Self::Practice(level)) = self else {
            return None;
        };
        let name = LEVELS[level]
            .name
            .split_whitespace()
            .map(|word| {
                let (first, rest) = word.split_at(1);
                first.to_owned() + &rest.to_lowercase()
            })
            .collect::<Vec<_>>()
            .join(" ");
        Some((format!("{:02}", level + 1), name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menus_and_sectors_read_naturally() {
        let mut game = Game::at(2, Mode::Journey);
        assert_eq!(Presence::of(false, &game), Presence::Menus);
        assert_eq!(Presence::Menus.sector(), None);
        let journey = Presence::of(true, &game);
        assert_eq!(journey, Presence::Journey(2));
        assert_eq!(journey.sector(), Some(("03".into(), "Slipstream".into())));
        game = Game::at(0, Mode::Practice);
        let practice = Presence::of(true, &game);
        assert_eq!(practice, Presence::Practice(0));
        assert_eq!(practice.sector(), Some(("01".into(), "First Light".into())));
    }

    #[test]
    fn localization_file_defines_every_token() {
        let vdf = include_str!("../docs/steam/rich_presence.vdf");
        for p in [Presence::Menus, Presence::Journey(0), Presence::Practice(0)] {
            assert!(vdf.contains(&format!("\"{}\"", p.token())), "{p:?}");
        }
        assert!(vdf.contains("%sector%") && vdf.contains("%name%"));
    }
}
