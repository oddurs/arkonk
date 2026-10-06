//! Versioned local progress. Disk access happens at menu/sector boundaries.
use crate::game::{Game, LEVEL_COUNT, Mode, Phase};
use std::{fs, io, path::Path};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Record {
    pub medals: u8,
    pub best_ticks: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checkpoint {
    pub level: usize,
    pub score: u32,
    pub lives: u8,
    pub ticks: u32,
}
impl Checkpoint {
    pub fn game(self) -> Game {
        let mut game = Game::at(self.level, Mode::Journey);
        game.score = self.score;
        game.lives = self.lives;
        game.run_ticks = self.ticks;
        game
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub best_score: u32,
    pub unlocked: usize,
    pub records: [Record; LEVEL_COUNT],
    pub checkpoint: Option<Checkpoint>,
    pub muted: bool,
    pub crt: bool,
    pub volume: u8,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            best_score: 0,
            unlocked: 1,
            records: [Record::default(); LEVEL_COUNT],
            checkpoint: None,
            muted: false,
            crt: true,
            volume: 6,
        }
    }
}
impl Profile {
    pub fn medals(&self) -> u32 {
        self.records.iter().map(|r| r.medals.count_ones()).sum()
    }
    pub fn begin(&mut self, game: &Game) {
        self.checkpoint = Some(Checkpoint {
            level: game.level,
            score: game.score,
            lives: game.lives,
            ticks: game.run_ticks,
        });
    }
    pub fn finish(&mut self, game: &Game) {
        let record = &mut self.records[game.level];
        record.medals |= game.summary.medals;
        if record.best_ticks == 0 || game.summary.ticks < record.best_ticks {
            record.best_ticks = game.summary.ticks;
        }
        self.unlocked = self.unlocked.max((game.level + 2).min(LEVEL_COUNT));
        if game.mode == Mode::Journey {
            self.best_score = self.best_score.max(game.score);
            self.checkpoint = if game.phase == Phase::Victory {
                None
            } else {
                Some(Checkpoint {
                    level: game.level + 1,
                    score: game.score,
                    lives: game.lives,
                    ticks: game.run_ticks,
                })
            };
        }
    }
    pub fn decode(text: &str) -> Self {
        let mut p = Self::default();
        let mut lines = text.lines();
        if lines.next() != Some("ARKONK 1") {
            return p;
        }
        for line in lines {
            let mut words = line.split_whitespace();
            let key = words.next().unwrap_or("");
            let Ok(nums) = words.map(str::parse::<u32>).collect::<Result<Vec<_>, _>>() else {
                continue;
            };
            match (key, nums.as_slice()) {
                ("best", [n]) => p.best_score = *n,
                ("unlocked", [n]) => p.unlocked = (*n as usize).clamp(1, LEVEL_COUNT),
                ("settings", [mute, crt, volume]) => {
                    p.muted = *mute != 0;
                    p.crt = *crt != 0;
                    p.volume = (*volume).min(10) as u8;
                }
                ("record", [i, medals, ticks]) if (*i as usize) < LEVEL_COUNT => {
                    p.records[*i as usize] = Record {
                        medals: (*medals as u8) & 7,
                        best_ticks: *ticks,
                    }
                }
                ("checkpoint", [level, score, lives, ticks])
                    if (*level as usize) < LEVEL_COUNT && (1..=5).contains(lives) =>
                {
                    p.checkpoint = Some(Checkpoint {
                        level: *level as usize,
                        score: *score,
                        lives: *lives as u8,
                        ticks: *ticks,
                    })
                }
                _ => {}
            }
        }
        // Never offer a checkpoint in a sector that is still locked.
        if p.checkpoint.is_some_and(|c| c.level >= p.unlocked) {
            p.checkpoint = None;
        }
        p
    }
    pub fn encode(&self) -> String {
        use std::fmt::Write;
        let mut out = format!(
            "ARKONK 1\nbest {}\nunlocked {}\nsettings {} {} {}\n",
            self.best_score,
            self.unlocked,
            u8::from(self.muted),
            u8::from(self.crt),
            self.volume
        );
        for (i, r) in self.records.iter().enumerate() {
            let _ = writeln!(out, "record {i} {} {}", r.medals, r.best_ticks);
        }
        if let Some(c) = self.checkpoint {
            let _ = writeln!(
                out,
                "checkpoint {} {} {} {}",
                c.level, c.score, c.lives, c.ticks
            );
        }
        out
    }
    pub fn load(path: &Path) -> Self {
        if let Ok(text) = fs::read_to_string(path) {
            return Self::decode(&text);
        }
        let mut p = Self::default();
        if let Ok(text) = fs::read_to_string(path.with_file_name("best.txt")) {
            p.best_score = text.trim().parse().unwrap_or(0);
        }
        p
    }
    pub fn save(&self, path: &Path) -> io::Result<()> {
        use std::io::Write;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp = path.with_extension("tmp");
        let mut file = fs::File::create(&temp)?;
        file.write_all(self.encode().as_bytes())?;
        file.sync_all()?;
        drop(file);
        fs::rename(temp, path)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn progress_and_settings_round_trip() {
        let mut p = Profile {
            unlocked: 7,
            best_score: 24600,
            muted: true,
            volume: 3,
            ..Profile::default()
        };
        p.records[5] = Record {
            medals: 7,
            best_ticks: 15400,
        };
        p.begin(&Game::at(6, Mode::Journey));
        assert_eq!(Profile::decode(&p.encode()), p);
    }
    #[test]
    fn corrupt_values_cannot_unlock_invalid_sectors_or_lives() {
        let p = Profile::decode(
            "ARKONK 1\nunlocked 999\nsettings 0 1 300\ncheckpoint 20 0 90 0\nrecord 500 7 0\nrecord 0 255 25\nbest nonsense",
        );
        assert_eq!(p.unlocked, LEVEL_COUNT);
        assert_eq!(p.volume, 10);
        assert_eq!(p.checkpoint, None);
        assert_eq!(p.records[0].medals, 7);
        assert_eq!(Profile::decode("unknown version"), Profile::default());
    }
    #[test]
    fn practice_records_do_not_replace_journey_checkpoint() {
        let mut p = Profile::default();
        p.begin(&Game::new());
        let saved = p.checkpoint;
        let mut g = Game::at(0, Mode::Practice);
        g.phase = Phase::Cleared;
        g.summary.medals = 7;
        g.summary.ticks = 1200;
        g.score = 5000;
        p.finish(&g);
        g.summary.ticks = 1500;
        g.summary.medals = 1;
        p.finish(&g);
        assert_eq!(p.checkpoint, saved);
        assert_eq!(p.best_score, 0);
        assert_eq!(
            p.records[0],
            Record {
                medals: 7,
                best_ticks: 1200
            }
        );
        assert_eq!(p.unlocked, 2);
    }
    #[test]
    fn saving_replaces_existing_profile() {
        let dir = std::env::temp_dir().join(format!(
            "arkonk-profile-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let path = dir.join("progress.txt");
        let mut p = Profile::default();
        p.save(&path).unwrap();
        p.best_score = 123;
        p.save(&path).unwrap();
        assert_eq!(Profile::load(&path), p);
        fs::remove_dir_all(dir).unwrap();
    }
}
