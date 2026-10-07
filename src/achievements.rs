//! Achievements derived from saved progress and sector results. The derivation is
//! pure so it can be tested without a Steam client; `docs/steam/achievements.md`
//! is the matching partner-site configuration.
use crate::{
    game::{Game, Mode, Phase},
    levels::LEVELS,
    profile::Profile,
};

const CLEAR: u8 = 1;
const CLEAN: u8 = 2;
const SWIFT: u8 = 4;
const ALL_MEDALS: u8 = CLEAR | CLEAN | SWIFT;
/// The best chain, as shown on the results card, that earns `Chain`.
pub const CHAIN_TARGET: u32 = 20;
/// Integer stat backing the progress bar on `AllMedals`.
pub const MEDALS_STAT: &str = "MEDALS";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Achievement {
    FirstLight,
    Clean,
    Swift,
    Daybreak,
    BlueHour,
    Afterlight,
    DaybreakMedals,
    BlueHourMedals,
    AfterlightMedals,
    AllMedals,
    Homecoming,
    Chain,
}
impl Achievement {
    pub const ALL: [Self; 12] = [
        Self::FirstLight,
        Self::Clean,
        Self::Swift,
        Self::Daybreak,
        Self::BlueHour,
        Self::Afterlight,
        Self::DaybreakMedals,
        Self::BlueHourMedals,
        Self::AfterlightMedals,
        Self::AllMedals,
        Self::Homecoming,
        Self::Chain,
    ];
    pub fn api_name(self) -> &'static str {
        match self {
            Self::FirstLight => "FIRST_LIGHT",
            Self::Clean => "CLEAN",
            Self::Swift => "SWIFT",
            Self::Daybreak => "CHAPTER_DAYBREAK",
            Self::BlueHour => "CHAPTER_BLUE_HOUR",
            Self::Afterlight => "CHAPTER_AFTERLIGHT",
            Self::DaybreakMedals => "MEDALS_DAYBREAK",
            Self::BlueHourMedals => "MEDALS_BLUE_HOUR",
            Self::AfterlightMedals => "MEDALS_AFTERLIGHT",
            Self::AllMedals => "ALL_MEDALS",
            Self::Homecoming => "JOURNEY_COMPLETE",
            Self::Chain => "CHAIN_REACTION",
        }
    }
}

/// Everything the saved profile proves, so medals earned before Steam was
/// present unlock on the next launch.
pub fn from_profile(profile: &Profile) -> Vec<Achievement> {
    let medals = |chapter: Option<usize>| {
        profile
            .records
            .iter()
            .zip(LEVELS.iter())
            .filter(move |(_, level)| chapter.is_none_or(|c| level.chapter == c))
            .map(|(record, _)| record.medals)
    };
    let any = |bit| medals(None).any(|m| m & bit != 0);
    let chapter = |c, bits| medals(Some(c)).all(|m| m & bits == bits);
    [
        (Achievement::FirstLight, any(CLEAR)),
        (Achievement::Clean, any(CLEAN)),
        (Achievement::Swift, any(SWIFT)),
        (Achievement::Daybreak, chapter(0, CLEAR)),
        (Achievement::BlueHour, chapter(1, CLEAR)),
        (Achievement::Afterlight, chapter(2, CLEAR)),
        (Achievement::DaybreakMedals, chapter(0, ALL_MEDALS)),
        (Achievement::BlueHourMedals, chapter(1, ALL_MEDALS)),
        (Achievement::AfterlightMedals, chapter(2, ALL_MEDALS)),
        (
            Achievement::AllMedals,
            medals(None).all(|m| m == ALL_MEDALS),
        ),
    ]
    .into_iter()
    .filter_map(|(a, earned)| earned.then_some(a))
    .collect()
}

/// Feats only visible in the moment a sector is cleared; the profile does not
/// record them.
pub fn from_clear(game: &Game) -> Vec<Achievement> {
    let mut earned = Vec::new();
    if game.mode == Mode::Journey && game.phase == Phase::Victory {
        earned.push(Achievement::Homecoming);
    }
    if game.summary.best_combo >= CHAIN_TARGET {
        earned.push(Achievement::Chain);
    }
    earned
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{game::LEVEL_COUNT, profile::Record};

    fn with_medals(medals: [u8; LEVEL_COUNT]) -> Profile {
        let mut p = Profile::default();
        for (record, m) in p.records.iter_mut().zip(medals) {
            *record = Record {
                medals: m,
                best_ticks: if m == 0 { 0 } else { 24000 },
            };
        }
        p
    }

    #[test]
    fn fresh_profile_earns_nothing() {
        assert!(from_profile(&Profile::default()).is_empty());
    }

    #[test]
    fn single_medals_unlock_their_firsts() {
        let mut medals = [0; LEVEL_COUNT];
        medals[5] = CLEAR | SWIFT;
        assert_eq!(
            from_profile(&with_medals(medals)),
            [Achievement::FirstLight, Achievement::Swift]
        );
    }

    #[test]
    fn chapters_need_every_sector_in_that_chapter() {
        let mut medals = [0; LEVEL_COUNT];
        medals[..4].fill(CLEAR);
        medals[4..7].fill(ALL_MEDALS);
        let earned = from_profile(&with_medals(medals));
        assert!(earned.contains(&Achievement::Daybreak));
        assert!(!earned.contains(&Achievement::DaybreakMedals));
        assert!(!earned.contains(&Achievement::BlueHour));
        assert!(!earned.contains(&Achievement::BlueHourMedals));
        medals[7] = ALL_MEDALS;
        let earned = from_profile(&with_medals(medals));
        assert!(earned.contains(&Achievement::BlueHour));
        assert!(earned.contains(&Achievement::BlueHourMedals));
        assert!(!earned.contains(&Achievement::AllMedals));
    }

    #[test]
    fn thirty_six_medals_unlock_every_profile_achievement() {
        let p = with_medals([ALL_MEDALS; LEVEL_COUNT]);
        assert_eq!(p.medals(), 36);
        let earned = from_profile(&p);
        assert_eq!(earned.len(), 10);
        assert!(earned.contains(&Achievement::AllMedals));
    }

    #[test]
    fn profile_finish_feeds_the_derivation() {
        let mut p = Profile::default();
        let mut g = Game::at(0, Mode::Practice);
        g.phase = Phase::Cleared;
        g.summary.medals = CLEAR | CLEAN;
        p.finish(&g);
        assert_eq!(
            from_profile(&p),
            [Achievement::FirstLight, Achievement::Clean]
        );
    }

    #[test]
    fn journey_victory_and_long_chains_come_from_the_clear() {
        let mut g = Game::at(LEVEL_COUNT - 1, Mode::Journey);
        g.phase = Phase::Victory;
        g.summary.best_combo = CHAIN_TARGET - 1;
        assert_eq!(from_clear(&g), [Achievement::Homecoming]);
        g.summary.best_combo = CHAIN_TARGET;
        assert_eq!(
            from_clear(&g),
            [Achievement::Homecoming, Achievement::Chain]
        );
        // Practice cannot finish the journey, however the sector ends.
        let mut g = Game::at(LEVEL_COUNT - 1, Mode::Practice);
        g.phase = Phase::Cleared;
        assert!(from_clear(&g).is_empty());
    }

    #[test]
    fn partner_site_table_lists_every_api_name() {
        let doc = include_str!("../docs/steam/achievements.md");
        for a in Achievement::ALL {
            assert!(doc.contains(&format!("`{}`", a.api_name())), "{a:?}");
        }
        assert!(doc.contains(&format!("`{MEDALS_STAT}`")));
    }
}
