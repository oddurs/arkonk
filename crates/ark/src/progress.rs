//! Progress through the journey, and the save file's text format.
//!
//! The rules here must agree with the simulation: which sectors a clear
//! unlocks, how medals and best times merge, and where a journey resumes.
//! The codec reads bytes and writes into any [`fmt::Write`], so it neither
//! allocates nor touches files; the front end decides where saves live.
//!
//! A save is ASCII text: the [`HEADER`] line, then one `key value…` line
//! per fact. Unknown keys are ignored and damaged lines are skipped, so one
//! bad byte costs one line, never the whole file. The front end stores its
//! settings in the same file, through [`entries`] and the `settings`
//! argument of [`Progress::encode`].
//!
//! Version 2 numbers the 64 sectors of the journey in order. Version 1
//! numbered the original twelve, in an order that no longer exists; a
//! version-1 save still reads, each fact moving with its sector's slug (see
//! [`V1_SECTORS`]), and is written back as version 2.
//!
//! ```
//! use ark::progress::Progress;
//!
//! let saved = b"ARKONK 2\nbest 24600\nunlocked 3\nrecord 0 7 15400\n";
//! let progress = Progress::decode(saved).expect("a version-2 save");
//! assert_eq!(progress.best_score(), 24600);
//! assert_eq!(progress.medal_count(), 3);
//!
//! let mut file = String::new();
//! progress.encode(&mut file, |_| Ok(())).expect("a String accepts any text");
//! assert_eq!(Progress::decode(file.as_bytes()), Some(progress));
//! ```
use crate::{
    Game, Medals, Mode, Stage,
    sectors::{SECTOR_COUNT, SectorId},
    tuning::MAX_LIVES,
};
use core::fmt;

/// The first line of every save this version writes.
pub const HEADER: &str = "ARKONK 2";
/// The first line of a save from before the 64-sector journey.
pub const HEADER_V1: &str = "ARKONK 1";

/// The twelve sectors a version-1 save numbers, in its order. Each still
/// exists under the same slug; `docs/journey.md` says where each went.
pub const V1_SECTORS: [&str; 12] = [
    "first_light",
    "satellites",
    "slipstream",
    "resonance",
    "prism",
    "crossfade",
    "undertow",
    "moonrise",
    "afterglow",
    "parallax",
    "supernova",
    "homecoming",
];

/// The most values a save line holds.
const MAX_VALUES: usize = 4;

/// The best a sector has gone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Record {
    /// Every medal ever earned there.
    pub medals: Medals,
    /// The fastest clear, in ticks; zero before the first.
    pub best_ticks: u32,
}

/// Where a journey resumes: the start of a sector, with the score, lives
/// and run time it had there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    /// The sector to start.
    pub sector: SectorId,
    /// Score on entering it.
    pub score: u32,
    /// Lives on entering it.
    pub lives: u8,
    /// Run time on entering it, in ticks.
    pub ticks: u32,
}

/// A player's progress: unlocks, medals, best times and score, and the
/// journey checkpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Progress {
    /// The best journey score.
    best_score: u32,
    /// Sectors unlocked, from the first: `1..=SECTOR_COUNT`.
    unlocked: usize,
    /// Per sector, in journey order.
    records: [Record; SECTOR_COUNT],
    /// Where the journey resumes, if one is under way.
    checkpoint: Option<Checkpoint>,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            best_score: 0,
            unlocked: 1,
            records: [Record::default(); SECTOR_COUNT],
            checkpoint: None,
        }
    }
}

impl Progress {
    /// The best journey score.
    pub fn best_score(&self) -> u32 {
        self.best_score
    }
    /// How many sectors are unlocked, counting from the first.
    pub fn unlocked_count(&self) -> usize {
        self.unlocked
    }
    /// Whether `sector` can be played.
    pub fn is_unlocked(&self, sector: SectorId) -> bool {
        sector.index() < self.unlocked
    }
    /// The best `sector` has gone.
    pub fn record(&self, sector: SectorId) -> Record {
        self.records[sector.index()]
    }
    /// Where the journey resumes, if one is under way.
    pub fn checkpoint(&self) -> Option<Checkpoint> {
        self.checkpoint
    }
    /// Medals earned across every sector.
    pub fn medal_count(&self) -> u32 {
        self.records.iter().map(|r| r.medals.count()).sum()
    }

    /// A journey has started or restarted at `game`'s sector: that is now
    /// where it resumes.
    pub fn begin(&mut self, game: &Game) {
        self.checkpoint = Some(game.checkpoint());
    }

    /// `game` has just cleared its sector: merge its medals and time, unlock
    /// the next sector, and in a journey move the checkpoint past it (or
    /// clear it after the last). Practice never touches the journey.
    pub fn finish(&mut self, game: &Game) {
        let summary = game.summary();
        let record = &mut self.records[game.sector().index()];
        record.medals |= summary.medals;
        if record.best_ticks == 0 || summary.ticks < record.best_ticks {
            record.best_ticks = summary.ticks;
        }
        self.unlocked = self
            .unlocked
            .max((game.sector().index() + 2).min(SECTOR_COUNT));
        if game.mode() == Mode::Journey {
            self.raise_best_score(game.score());
            self.checkpoint = match game.sector().next() {
                Some(next) if game.stage() != Stage::Victory => Some(Checkpoint {
                    sector: next,
                    ..game.checkpoint()
                }),
                _ => None,
            };
        }
    }

    /// Records a journey's score as the best if it beats it. True when it
    /// did; Practice scores never count.
    pub fn note_score(&mut self, game: &Game) -> bool {
        game.mode() == Mode::Journey && self.raise_best_score(game.score())
    }

    /// Makes `score` the best journey score if it beats it, as when importing
    /// one from an older save. True when it did.
    pub fn raise_best_score(&mut self, score: u32) -> bool {
        let beaten = score > self.best_score;
        self.best_score = self.best_score.max(score);
        beaten
    }

    /// Reads a save. `None` when `file` is not a save of either version;
    /// inside one, lines that are damaged, out of range or unknown are
    /// skipped, and the last of duplicate lines wins. A version-1 save is
    /// mapped onto the journey by slug: records and the checkpoint move
    /// with their sectors, and every sector it had open stays open.
    pub fn decode(file: &[u8]) -> Option<Self> {
        let v1 = version(file)? == 1;
        // A sector by its number in this file's version.
        let sector = |i: u32| {
            if v1 {
                V1_SECTORS
                    .get(i as usize)
                    .and_then(|slug| SectorId::from_slug(slug))
            } else {
                SectorId::new(i as usize)
            }
        };
        let mut p = Self::default();
        for entry in entries(file)? {
            match (entry.key(), entry.values()) {
                ("best", [n]) => p.best_score = *n,
                ("unlocked", [n]) if v1 => {
                    // Open through the furthest new place of any sector the
                    // old journey had open.
                    let open = (*n as usize).clamp(1, V1_SECTORS.len());
                    p.unlocked = (0..open as u32)
                        .filter_map(sector)
                        .map(|s| s.index() + 1)
                        .max()
                        .unwrap_or(1);
                }
                ("unlocked", [n]) => p.unlocked = (*n as usize).clamp(1, SECTOR_COUNT),
                ("record", [i, medals, ticks]) => {
                    if let Some(sector) = sector(*i) {
                        p.records[sector.index()] = Record {
                            medals: Medals::from_bits(*medals as u8),
                            best_ticks: *ticks,
                        };
                    }
                }
                ("checkpoint", [at, score, lives, ticks])
                    if (1..=u32::from(MAX_LIVES)).contains(lives) =>
                {
                    if let Some(sector) = sector(*at) {
                        p.checkpoint = Some(Checkpoint {
                            sector,
                            score: *score,
                            lives: *lives as u8,
                            ticks: *ticks,
                        });
                    }
                }
                _ => {}
            }
        }
        // Never offer a checkpoint in a sector that is still locked.
        if p.checkpoint.is_some_and(|c| !p.is_unlocked(c.sector)) {
            p.checkpoint = None;
        }
        Some(p)
    }

    /// Writes a version-2 save: the header, the best score and unlocks, then
    /// whatever `settings` writes, then a record for each sector with a
    /// medal or a time, and the checkpoint. The order is version 1's.
    pub fn encode<W: fmt::Write>(
        &self,
        out: &mut W,
        settings: impl FnOnce(&mut W) -> fmt::Result,
    ) -> fmt::Result {
        writeln!(out, "{HEADER}")?;
        writeln!(out, "best {}", self.best_score)?;
        writeln!(out, "unlocked {}", self.unlocked)?;
        settings(out)?;
        for (i, r) in self.records.iter().enumerate() {
            if *r != Record::default() {
                writeln!(out, "record {i} {} {}", r.medals.bits(), r.best_ticks)?;
            }
        }
        if let Some(c) = self.checkpoint {
            writeln!(
                out,
                "checkpoint {} {} {} {}",
                c.sector.index(),
                c.score,
                c.lives,
                c.ticks
            )?;
        }
        Ok(())
    }
}

/// One `key value…` line of a save, with every value a whole number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry<'a> {
    /// The first word.
    key: &'a str,
    /// The values after it; only the first `len` are used.
    values: [u32; MAX_VALUES],
    /// How many values the line has.
    len: usize,
}

impl<'a> Entry<'a> {
    /// The first word.
    pub fn key(&self) -> &'a str {
        self.key
    }
    /// The values after it.
    pub fn values(&self) -> &[u32] {
        &self.values[..self.len]
    }

    /// Parses one line. `None` for blank lines, lines that are not UTF-8,
    /// and lines with a value that is not a `u32` or with more values than
    /// any key uses.
    fn parse(line: &'a [u8]) -> Option<Self> {
        let mut words = core::str::from_utf8(line).ok()?.split_whitespace();
        let mut entry = Self {
            key: words.next()?,
            values: [0; MAX_VALUES],
            len: 0,
        };
        for word in words {
            *entry.values.get_mut(entry.len)? = word.parse().ok()?;
            entry.len += 1;
        }
        Some(entry)
    }
}

/// The lines of a save of either version, skipping damaged ones. `None`
/// when `file` does not start with [`HEADER`] or [`HEADER_V1`].
pub fn entries(file: &[u8]) -> Option<impl Iterator<Item = Entry<'_>>> {
    version(file)?;
    Some(file.split(|&b| b == b'\n').skip(1).filter_map(Entry::parse))
}

/// The save format `file` is written in: 1 or 2. `None` when it is not a
/// save.
fn version(file: &[u8]) -> Option<u8> {
    let header = file.split(|&b| b == b'\n').next()?;
    match core::str::from_utf8(header).ok()?.trim_end() {
        HEADER => Some(2),
        HEADER_V1 => Some(1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Input, clock::TICK_HZ};

    fn encoded(p: &Progress) -> String {
        let mut out = String::new();
        p.encode(&mut out, |_| Ok(())).unwrap();
        out
    }

    /// `sector`, cleared after `ticks` of play.
    fn cleared(sector: SectorId, mode: Mode, ticks: u32) -> Game {
        let mut g = Game::start(sector, mode);
        g.step(Input {
            launch: true,
            ..Input::default()
        });
        g.sandbox().elapse(ticks);
        g.sandbox().clear_board();
        g.step(Input::default());
        assert!(matches!(g.stage(), Stage::Cleared | Stage::Victory));
        g
    }

    #[test]
    fn progress_round_trips() {
        let mut p =
            Progress::decode(b"ARKONK 2\nbest 24600\nunlocked 7\nrecord 5 7 15400\n").unwrap();
        p.begin(&Game::start(SectorId::new(6).unwrap(), Mode::Journey));
        assert_eq!(Progress::decode(encoded(&p).as_bytes()), Some(p));
    }

    #[test]
    fn corrupt_values_cannot_unlock_invalid_sectors_or_lives() {
        let p = Progress::decode(
            b"ARKONK 2\nunlocked 999\ncheckpoint 20 0 90 0\nrecord 500 7 0\nrecord 0 255 25\nbest nonsense",
        )
        .unwrap();
        assert_eq!(p.unlocked_count(), SECTOR_COUNT);
        assert_eq!(p.checkpoint(), None);
        assert_eq!(p.record(SectorId::FIRST).medals, Medals::ALL);
        assert_eq!(Progress::decode(b"unknown version"), None);
        assert_eq!(Progress::decode(b""), None);
        assert_eq!(Progress::decode(b"ARKONK\xff 1\nbest 5"), None);
    }

    #[test]
    fn a_locked_checkpoint_is_dropped() {
        let p = Progress::decode(b"ARKONK 2\nunlocked 2\ncheckpoint 2 100 3 50\n").unwrap();
        assert_eq!(p.checkpoint(), None);
        let p = Progress::decode(b"ARKONK 2\nunlocked 3\ncheckpoint 2 100 3 50\n").unwrap();
        assert_eq!(p.checkpoint().map(|c| c.sector.index()), Some(2));
    }

    #[test]
    fn entries_skip_damaged_lines_only() {
        let file = b"ARKONK 2\r\nbest 5\r\nbad \xff 1\nsettings 1 2 3\nmany 1 2 3 4 5\nkey +7\n";
        let read: Vec<_> = entries(file)
            .unwrap()
            .map(|e| (e.key(), e.values().to_vec()))
            .collect();
        assert_eq!(
            read,
            [
                ("best", vec![5]),
                ("settings", vec![1, 2, 3]),
                ("key", vec![7]),
            ]
        );
    }

    /// Where a version-1 sector number lands now.
    fn moved(old: usize) -> usize {
        SectorId::from_slug(V1_SECTORS[old]).unwrap().index()
    }

    #[test]
    fn every_version_1_sector_still_exists() {
        let mut places: Vec<_> = (0..V1_SECTORS.len()).map(moved).collect();
        assert_eq!(places, [0, 8, 16, 23, 24, 20, 27, 40, 32, 35, 39, 63]);
        places.sort_unstable();
        places.dedup();
        assert_eq!(places.len(), V1_SECTORS.len());
        assert_eq!(moved(0), 0, "the journey still starts at First Light");
        assert_eq!(moved(11), SECTOR_COUNT - 1, "and still ends at Homecoming");
    }

    #[test]
    fn a_version_1_save_moves_with_its_slugs() {
        let file = b"ARKONK 1\nbest 31000\nunlocked 6\nrecord 0 7 15400\n\
            record 1 3 20000\nrecord 4 1 30000\nrecord 5 0 0\nrecord 12 7 1\n\
            checkpoint 5 9000 4 72000\n";
        let p = Progress::decode(file).unwrap();
        assert_eq!(p.best_score(), 31000);
        assert_eq!(p.medal_count(), 3 + 2 + 1);
        let at = |slug| p.record(SectorId::from_slug(slug).unwrap());
        assert_eq!(at("first_light").medals, Medals::ALL);
        assert_eq!(at("satellites").best_ticks, 20000);
        assert_eq!(at("prism").medals, Medals::CLEAR);
        // Sector 6 of 12 was crossfade, now 21 of 64; prism, now 25, was
        // open too, so everything through it is.
        let checkpoint = p.checkpoint().unwrap();
        assert_eq!(checkpoint.sector, SectorId::from_slug("crossfade").unwrap());
        assert_eq!(
            (checkpoint.score, checkpoint.lives, checkpoint.ticks),
            (9000, 4, 72000)
        );
        assert_eq!(p.unlocked_count(), 25);
        // Written back as version 2, it reads the same.
        let rewritten = encoded(&p);
        assert!(rewritten.starts_with("ARKONK 2\n"));
        assert_eq!(Progress::decode(rewritten.as_bytes()), Some(p));
    }

    #[test]
    fn a_finished_version_1_journey_opens_every_sector() {
        let p = Progress::decode(b"ARKONK 1\nunlocked 12\nrecord 11 7 9000\n").unwrap();
        assert_eq!(p.unlocked_count(), SECTOR_COUNT);
        assert_eq!(
            p.record(SectorId::clamped(SECTOR_COUNT)).medals,
            Medals::ALL
        );
        let fresh = Progress::decode(b"ARKONK 1\nunlocked 1\n").unwrap();
        assert_eq!(fresh, Progress::default());
        // Out-of-range old numbers clamp to the old journey, never past it.
        let wild = Progress::decode(b"ARKONK 1\nunlocked 4000\ncheckpoint 12 5 3 0\n").unwrap();
        assert_eq!(wild.unlocked_count(), SECTOR_COUNT);
        assert_eq!(wild.checkpoint(), None);
    }

    #[test]
    fn version_2_writes_only_what_was_earned() {
        let mut p = Progress::default();
        assert_eq!(encoded(&p), "ARKONK 2\nbest 0\nunlocked 1\n");
        p.finish(&cleared(SectorId::clamped(40), Mode::Practice, 2400));
        assert_eq!(
            encoded(&p),
            "ARKONK 2\nbest 0\nunlocked 42\nrecord 40 7 2401\n"
        );
    }

    #[test]
    fn practice_records_do_not_replace_journey_checkpoint() {
        let mut p = Progress::default();
        p.begin(&Game::new());
        let saved = p.checkpoint();
        p.finish(&cleared(SectorId::FIRST, Mode::Practice, 1199));
        let par = SectorId::FIRST.sector().par_seconds * TICK_HZ;
        let slow = cleared(SectorId::FIRST, Mode::Practice, par + 300);
        assert_eq!(slow.summary().medals, Medals::CLEAR | Medals::CLEAN);
        p.finish(&slow);
        assert_eq!(p.checkpoint(), saved);
        assert_eq!(p.best_score(), 0);
        assert_eq!(
            p.record(SectorId::FIRST),
            Record {
                medals: Medals::ALL,
                best_ticks: 1200
            }
        );
        assert_eq!(p.unlocked_count(), 2);
    }

    #[test]
    fn journey_clears_move_the_checkpoint_and_the_best_score() {
        let mut p = Progress::default();
        let g = cleared(SectorId::FIRST, Mode::Journey, 0);
        p.finish(&g);
        let next = p.checkpoint().unwrap();
        assert_eq!(next.sector.index(), 1);
        assert_eq!((next.score, next.lives), (g.score(), g.lives()));
        assert_eq!(p.best_score(), g.score());
        assert!(!p.note_score(&g));
        assert!(p.raise_best_score(g.score() + 1));
        let last = SectorId::new(SECTOR_COUNT - 1).unwrap();
        let won = cleared(last, Mode::Journey, 0);
        assert_eq!(won.stage(), Stage::Victory);
        p.finish(&won);
        assert_eq!(p.checkpoint(), None);
    }
}
