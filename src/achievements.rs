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
    /// Every sector of the chapter cleared.
    Chapter(Chapter),
    /// Every medal of the chapter's sectors.
    ChapterMedals(Chapter),
    AllMedals,
    Homecoming,
    Chain,
}
impl Achievement {
    /// Every achievement, for checking the partner-site table.
    #[cfg(test)]
    fn all() -> Vec<Self> {
        [Self::FirstLight, Self::Clean, Self::Swift]
            .into_iter()
            .chain(Chapter::ALL.map(Self::Chapter))
            .chain(Chapter::ALL.map(Self::ChapterMedals))
            .chain([Self::AllMedals, Self::Homecoming, Self::Chain])
            .collect()
    }
    /// The partner site's name. The three chapters of the 12-sector
    /// journey keep theirs.
    pub fn api_name(self) -> &'static str {
        match self {
            Self::FirstLight => "FIRST_LIGHT",
            Self::Clean => "CLEAN",
            Self::Swift => "SWIFT",
            Self::Chapter(c) => match c {
                Chapter::Daybreak => "CHAPTER_DAYBREAK",
                Chapter::Morning => "CHAPTER_MORNING",
                Chapter::Zenith => "CHAPTER_ZENITH",
                Chapter::GoldenHour => "CHAPTER_GOLDEN_HOUR",
                Chapter::Afterlight => "CHAPTER_AFTERLIGHT",
                Chapter::BlueHour => "CHAPTER_BLUE_HOUR",
                Chapter::Eclipse => "CHAPTER_ECLIPSE",
                Chapter::Aurora => "CHAPTER_AURORA",
            },
            Self::ChapterMedals(c) => match c {
                Chapter::Daybreak => "MEDALS_DAYBREAK",
                Chapter::Morning => "MEDALS_MORNING",
                Chapter::Zenith => "MEDALS_ZENITH",
                Chapter::GoldenHour => "MEDALS_GOLDEN_HOUR",
                Chapter::Afterlight => "MEDALS_AFTERLIGHT",
                Chapter::BlueHour => "MEDALS_BLUE_HOUR",
                Chapter::Eclipse => "MEDALS_ECLIPSE",
                Chapter::Aurora => "MEDALS_AURORA",
            },
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
    ]
    .into_iter()
    .chain(Chapter::ALL.map(|c| (Achievement::Chapter(c), chapter(c, Medals::CLEAR))))
    .chain(Chapter::ALL.map(|c| (Achievement::ChapterMedals(c), chapter(c, Medals::ALL))))
    .chain([(
        Achievement::AllMedals,
        medals(None).all(|m| m == Medals::ALL),
    )])
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
        let mut file = String::from("ARKONK 2\n");
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
        medals[45] = Medals::CLEAR | Medals::SWIFT;
        assert_eq!(
            from_progress(&with_medals(medals)),
            [Achievement::FirstLight, Achievement::Swift]
        );
    }

    #[test]
    fn chapters_need_every_sector_in_that_chapter() {
        use Achievement::{Chapter as Cleared, ChapterMedals};
        let mut medals = [Medals::NONE; SECTOR_COUNT];
        medals[..8].fill(Medals::CLEAR);
        medals[8..15].fill(Medals::ALL);
        let earned = from_progress(&with_medals(medals));
        assert!(earned.contains(&Cleared(Chapter::Daybreak)));
        assert!(!earned.contains(&ChapterMedals(Chapter::Daybreak)));
        assert!(!earned.contains(&Cleared(Chapter::Morning)));
        assert!(!earned.contains(&ChapterMedals(Chapter::Morning)));
        medals[15] = Medals::ALL;
        let earned = from_progress(&with_medals(medals));
        assert!(earned.contains(&Cleared(Chapter::Morning)));
        assert!(earned.contains(&ChapterMedals(Chapter::Morning)));
        assert!(!earned.contains(&Cleared(Chapter::Zenith)));
        assert!(!earned.contains(&Achievement::AllMedals));
    }

    #[test]
    fn all_192_medals_unlock_every_profile_achievement() {
        let p = with_medals([Medals::ALL; SECTOR_COUNT]);
        assert_eq!(p.medal_count(), 192);
        let earned = from_progress(&p);
        assert_eq!(earned.len(), 3 + 2 * Chapter::ALL.len() + 1);
        assert!(earned.contains(&Achievement::AllMedals));
        let mut almost = [Medals::ALL; SECTOR_COUNT];
        almost[SECTOR_COUNT - 1] = Medals::CLEAR | Medals::CLEAN;
        let earned = from_progress(&with_medals(almost));
        assert!(!earned.contains(&Achievement::AllMedals));
        assert!(!earned.contains(&Achievement::ChapterMedals(Chapter::Aurora)));
        assert!(earned.contains(&Achievement::Chapter(Chapter::Aurora)));
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
        let all = Achievement::all();
        for (i, a) in all.iter().enumerate() {
            assert!(doc.contains(&format!("`{}`", a.api_name())), "{a:?}");
            assert!(!all[..i].iter().any(|b| b.api_name() == a.api_name()));
        }
        assert_eq!(all.len(), 22);
        assert!(doc.contains(&format!("`{MEDALS_STAT}`")));
        assert!(doc.contains("| 0 / 192 / 0 |"));
    }
}
