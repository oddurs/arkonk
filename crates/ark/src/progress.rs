//! Progress through the journey, and the save file's text format.
//!
//! The rules here must agree with the simulation: which sectors a clear
//! unlocks, how medals and best times merge, and where a journey resumes.
//! The codec reads bytes and writes into any [`fmt::Write`], so it neither
//! allocates nor touches files; the front end decides where saves live.
//!
//! A version-1 save is ASCII text: the [`HEADER`] line, then one
//! `key value…` line per fact. Unknown keys are ignored and damaged lines
//! are skipped, so one bad byte costs one line, never the whole file. The
//! front end stores its settings in the same file, through [`entries`] and
//! the `settings` argument of [`Progress::encode`].
//!
//! ```
//! use ark::progress::Progress;
//!
//! let saved = b"ARKONK 1\nbest 24600\nunlocked 3\nrecord 0 7 15400\n";
//! let progress = Progress::decode(saved).expect("a version-1 save");
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

/// The first line of every version-1 save.
pub const HEADER: &str = "ARKONK 1";

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

    /// Reads a save. `None` when `file` is not a version-1 save; inside one,
    /// lines that are damaged, out of range or unknown are skipped, and the
    /// last of duplicate lines wins.
    pub fn decode(file: &[u8]) -> Option<Self> {
        let mut p = Self::default();
        for entry in entries(file)? {
            match (entry.key(), entry.values()) {
                ("best", [n]) => p.best_score = *n,
                ("unlocked", [n]) => p.unlocked = (*n as usize).clamp(1, SECTOR_COUNT),
                ("record", [i, medals, ticks]) => {
                    if let Some(sector) = SectorId::new(*i as usize) {
                        p.records[sector.index()] = Record {
                            medals: Medals::from_bits(*medals as u8),
                            best_ticks: *ticks,
                        };
                    }
                }
                ("checkpoint", [sector, score, lives, ticks])
                    if (1..=u32::from(MAX_LIVES)).contains(lives) =>
                {
                    if let Some(sector) = SectorId::new(*sector as usize) {
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

    /// Writes a save: the header, the best score and unlocks, then whatever
    /// `settings` writes, then the records and checkpoint. That is version
    /// 1's order, so existing files are rewritten byte for byte.
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
            writeln!(out, "record {i} {} {}", r.medals.bits(), r.best_ticks)?;
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

/// The lines of a version-1 save, skipping damaged ones. `None` when `file`
/// does not start with [`HEADER`].
pub fn entries(file: &[u8]) -> Option<impl Iterator<Item = Entry<'_>>> {
    let mut lines = file.split(|&b| b == b'\n');
    let header = core::str::from_utf8(lines.next()?).ok()?;
    (header.trim_end() == HEADER).then(|| lines.filter_map(Entry::parse))
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
            Progress::decode(b"ARKONK 1\nbest 24600\nunlocked 7\nrecord 5 7 15400\n").unwrap();
        p.begin(&Game::start(SectorId::new(6).unwrap(), Mode::Journey));
        assert_eq!(Progress::decode(encoded(&p).as_bytes()), Some(p));
    }

    #[test]
    fn corrupt_values_cannot_unlock_invalid_sectors_or_lives() {
        let p = Progress::decode(
            b"ARKONK 1\nunlocked 999\ncheckpoint 20 0 90 0\nrecord 500 7 0\nrecord 0 255 25\nbest nonsense",
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
        let p = Progress::decode(b"ARKONK 1\nunlocked 2\ncheckpoint 2 100 3 50\n").unwrap();
        assert_eq!(p.checkpoint(), None);
        let p = Progress::decode(b"ARKONK 1\nunlocked 3\ncheckpoint 2 100 3 50\n").unwrap();
        assert_eq!(p.checkpoint().map(|c| c.sector.index()), Some(2));
    }

    #[test]
    fn entries_skip_damaged_lines_only() {
        let file = b"ARKONK 1\r\nbest 5\r\nbad \xff 1\nsettings 1 2 3\nmany 1 2 3 4 5\nkey +7\n";
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
