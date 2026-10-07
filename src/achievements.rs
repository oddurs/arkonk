//! Achievements derived from saved progress and sector results. The derivation is
//! pure so it can be tested without a Steam client; `docs/steam/achievements.md`
//! is the matching partner-site configuration.
use ark::{
    Game, Medals, Mode, SectorSummary, Stage,
    progress::Progress,
    sectors::{Chapter, SectorId},
};

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
    /// Every achievement, for checking the partner-site table.
    #[cfg(test)]
    const ALL: [Self; 12] = [
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

/// Everything saved progress proves, so medals earned before Steam was
/// present unlock on the next launch.
pub fn from_progress(progress: &Progress) -> Vec<Achievement> {
    let medals = |chapter: Option<Chapter>| {
        SectorId::all()
            .filter(move |s| chapter.is_none_or(|c| s.sector().chapter == c))
            .map(|s| progress.record(s).medals)
    };
    let any = |medal| medals(None).any(|m: Medals| m.contains(medal));
    let chapter = |c, wanted| medals(Some(c)).all(|m: Medals| m.contains(wanted));
    [
        (Achievement::FirstLight, any(Medals::CLEAR)),
        (Achievement::Clean, any(Medals::CLEAN)),
        (Achievement::Swift, any(Medals::SWIFT)),
        (
            Achievement::Daybreak,
            chapter(Chapter::Daybreak, Medals::CLEAR),
        ),
        (
            Achievement::BlueHour,
            chapter(Chapter::BlueHour, Medals::CLEAR),
        ),
        (
            Achievement::Afterlight,
            chapter(Chapter::Afterlight, Medals::CLEAR),
        ),
        (
            Achievement::DaybreakMedals,
            chapter(Chapter::Daybreak, Medals::ALL),
        ),
        (
            Achievement::BlueHourMedals,
            chapter(Chapter::BlueHour, Medals::ALL),
        ),
        (
            Achievement::AfterlightMedals,
            chapter(Chapter::Afterlight, Medals::ALL),
        ),
        (
            Achievement::AllMedals,
            medals(None).all(|m| m == Medals::ALL),
        ),
    ]
    .into_iter()
    .filter_map(|(a, earned)| earned.then_some(a))
    .collect()
}

/// Feats only visible in the moment a sector is cleared; the profile does not
/// record them.
pub fn from_clear(game: &Game) -> Vec<Achievement> {
    let journey_complete = game.mode() == Mode::Journey && game.stage() == Stage::Victory;
    from_results(journey_complete, &game.summary())
}

fn from_results(journey_complete: bool, summary: &SectorSummary) -> Vec<Achievement> {
    let mut earned = Vec::new();
    if journey_complete {
        earned.push(Achievement::Homecoming);
    }
    if summary.best_combo >= CHAIN_TARGET {
        earned.push(Achievement::Chain);
    }
    earned
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark::{Input, clock::TICK_HZ, sectors::SECTOR_COUNT};
    use std::fmt::Write;

    fn with_medals(medals: [Medals; SECTOR_COUNT]) -> Progress {
        let mut file = String::from("ARKONK 1\n");
        for (i, m) in medals.into_iter().enumerate() {
            let ticks = if m == Medals::NONE { 0 } else { 24000 };
            writeln!(file, "record {i} {} {ticks}", m.bits()).unwrap();
        }
        Progress::decode(file.as_bytes()).unwrap()
    }

    #[test]
    fn fresh_profile_earns_nothing() {
        assert!(from_progress(&Progress::default()).is_empty());
    }

    #[test]
    fn single_medals_unlock_their_firsts() {
        let mut medals = [Medals::NONE; SECTOR_COUNT];
        medals[5] = Medals::CLEAR | Medals::SWIFT;
        assert_eq!(
            from_progress(&with_medals(medals)),
            [Achievement::FirstLight, Achievement::Swift]
        );
    }

    #[test]
    fn chapters_need_every_sector_in_that_chapter() {
        let mut medals = [Medals::NONE; SECTOR_COUNT];
        medals[..4].fill(Medals::CLEAR);
        medals[4..7].fill(Medals::ALL);
        let earned = from_progress(&with_medals(medals));
        assert!(earned.contains(&Achievement::Daybreak));
        assert!(!earned.contains(&Achievement::DaybreakMedals));
        assert!(!earned.contains(&Achievement::BlueHour));
        assert!(!earned.contains(&Achievement::BlueHourMedals));
        medals[7] = Medals::ALL;
        let earned = from_progress(&with_medals(medals));
        assert!(earned.contains(&Achievement::BlueHour));
        assert!(earned.contains(&Achievement::BlueHourMedals));
        assert!(!earned.contains(&Achievement::AllMedals));
    }

    #[test]
    fn thirty_six_medals_unlock_every_profile_achievement() {
        let p = with_medals([Medals::ALL; SECTOR_COUNT]);
        assert_eq!(p.medal_count(), 36);
        let earned = from_progress(&p);
        assert_eq!(earned.len(), 10);
        assert!(earned.contains(&Achievement::AllMedals));
    }

    #[test]
    fn profile_finish_feeds_the_derivation() {
        let mut p = Progress::default();
        let mut g = Game::start(SectorId::FIRST, Mode::Practice);
        g.step(Input {
            launch: true,
            ..Input::default()
        });
        // Too slow for Swift.
        g.sandbox()
            .elapse(SectorId::FIRST.sector().par_seconds * TICK_HZ);
        g.sandbox().clear_board();
        g.step(Input::default());
        assert_eq!(g.summary().medals, Medals::CLEAR | Medals::CLEAN);
        p.finish(&g);
        assert_eq!(
            from_progress(&p),
            [Achievement::FirstLight, Achievement::Clean]
        );
    }

    #[test]
    fn journey_victory_and_long_chains_come_from_the_clear() {
        let short = SectorSummary {
            best_combo: CHAIN_TARGET - 1,
            ..SectorSummary::default()
        };
        let long = SectorSummary {
            best_combo: CHAIN_TARGET,
            ..short
        };
        assert_eq!(from_results(true, &short), [Achievement::Homecoming]);
        assert_eq!(
            from_results(true, &long),
            [Achievement::Homecoming, Achievement::Chain]
        );
        assert_eq!(from_results(false, &long), [Achievement::Chain]);
        // Practice cannot finish the journey, however the sector ends.
        for mode in [Mode::Journey, Mode::Practice] {
            let mut g = Game::start(SectorId::clamped(SECTOR_COUNT - 1), mode);
            g.step(Input {
                launch: true,
                ..Input::default()
            });
            g.sandbox().clear_board();
            g.step(Input::default());
            assert_eq!(from_clear(&g).is_empty(), mode == Mode::Practice);
        }
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
