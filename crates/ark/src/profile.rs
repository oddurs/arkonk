//! Versioned local progress. Disk access happens at menu/sector boundaries.
use crate::{
    Game, Medals, Mode, Stage,
    sectors::{SECTOR_COUNT, SectorId},
};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Record {
    pub medals: Medals,
    pub best_ticks: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checkpoint {
    pub sector: SectorId,
    pub score: u32,
    pub lives: u8,
    pub ticks: u32,
}
impl Checkpoint {
    pub fn game(self) -> Game {
        let mut game = Game::start(self.sector, Mode::Journey);
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
    pub records: [Record; SECTOR_COUNT],
    pub checkpoint: Option<Checkpoint>,
    pub muted: bool,
    pub volume: u8,
    /// Restored at launch; the app loop applies changes on the next frame.
    pub fullscreen: bool,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            best_score: 0,
            unlocked: 1,
            records: [Record::default(); SECTOR_COUNT],
            checkpoint: None,
            muted: false,
            volume: 6,
            fullscreen: false,
        }
    }
}
impl Profile {
    pub fn medals(&self) -> u32 {
        self.records.iter().map(|r| r.medals.count()).sum()
    }
    pub fn begin(&mut self, game: &Game) {
        self.checkpoint = Some(Checkpoint {
            sector: game.sector,
            score: game.score,
            lives: game.lives,
            ticks: game.run_ticks,
        });
    }
    pub fn finish(&mut self, game: &Game) {
        let record = &mut self.records[game.sector.index()];
        record.medals |= game.summary.medals;
        if record.best_ticks == 0 || game.summary.ticks < record.best_ticks {
            record.best_ticks = game.summary.ticks;
        }
        self.unlocked = self
            .unlocked
            .max((game.sector.index() + 2).min(SECTOR_COUNT));
        if game.mode == Mode::Journey {
            self.best_score = self.best_score.max(game.score);
            self.checkpoint = match game.sector.next() {
                Some(next) if game.stage != Stage::Victory => Some(Checkpoint {
                    sector: next,
                    score: game.score,
                    lives: game.lives,
                    ticks: game.run_ticks,
                }),
                _ => None,
            };
        }
    }
    /// `None` when the text is not a version-1 profile. Malformed lines inside a
    /// valid profile are skipped so one damaged record cannot discard the rest.
    pub fn decode(text: &str) -> Option<Self> {
        let mut p = Self::default();
        let mut lines = text.lines();
        if lines.next()?.trim_end() != "ARKONK 1" {
            return None;
        }
        for line in lines {
            let mut words = line.split_whitespace();
            let key = words.next().unwrap_or("");
            let Ok(nums) = words.map(str::parse::<u32>).collect::<Result<Vec<_>, _>>() else {
                continue;
            };
            match (key, nums.as_slice()) {
                ("best", [n]) => p.best_score = *n,
                ("unlocked", [n]) => p.unlocked = (*n as usize).clamp(1, SECTOR_COUNT),
                // Older saves carry a retired display flag between the two.
                ("settings", [mute, volume] | [mute, _, volume]) => {
                    p.muted = *mute != 0;
                    p.volume = (*volume).min(10) as u8;
                }
                ("display", [fullscreen]) => p.fullscreen = *fullscreen == 1,
                ("record", [i, medals, ticks]) if (*i as usize) < SECTOR_COUNT => {
                    p.records[*i as usize] = Record {
                        medals: Medals::from_bits(*medals as u8),
                        best_ticks: *ticks,
                    }
                }
                ("checkpoint", [level, score, lives, ticks]) if (1..=5).contains(lives) => {
                    if let Some(sector) = SectorId::new(*level as usize) {
                        p.checkpoint = Some(Checkpoint {
                            sector,
                            score: *score,
                            lives: *lives as u8,
                            ticks: *ticks,
                        })
                    }
                }
                _ => {}
            }
        }
        // Never offer a checkpoint in a sector that is still locked.
        if p.checkpoint.is_some_and(|c| c.sector.index() >= p.unlocked) {
            p.checkpoint = None;
        }
        Some(p)
    }
    pub fn encode(&self) -> String {
        use std::fmt::Write;
        let mut out = format!(
            "ARKONK 1\nbest {}\nunlocked {}\nsettings {} {}\ndisplay {}\n",
            self.best_score,
            self.unlocked,
            u8::from(self.muted),
            self.volume,
            u8::from(self.fullscreen)
        );
        for (i, r) in self.records.iter().enumerate() {
            let _ = writeln!(out, "record {i} {} {}", r.medals.bits(), r.best_ticks);
        }
        if let Some(c) = self.checkpoint {
            let _ = writeln!(
                out,
                "checkpoint {} {} {} {}",
                c.sector.index(),
                c.score,
                c.lives,
                c.ticks
            );
        }
        out
    }
    /// Loads progress, falling back to the last-known-good backup. A file that
    /// exists but cannot be decoded is never treated as absent: it is moved
    /// aside, or, if that fails, saving is blocked so it cannot be overwritten.
    pub fn load(path: &Path) -> Loaded {
        let backup = backup_path(path);
        let mut loaded = Loaded {
            profile: Self::default(),
            origin: Origin::New,
            set_aside: Vec::new(),
            blocked: None,
        };
        let main = read(path);
        if let Read::Valid(p) = main {
            loaded.profile = p;
            loaded.origin = Origin::Saved;
            return loaded;
        }
        if let Read::Invalid(why) = &main {
            loaded.quarantine(path, why.clone());
        }
        match read(&backup) {
            Read::Valid(p) => {
                loaded.profile = p;
                loaded.origin = Origin::Backup;
            }
            Read::Invalid(why) => loaded.quarantine(&backup, why),
            Read::Missing if matches!(main, Read::Missing) => {
                if let Ok(text) = fs::read_to_string(path.with_file_name("best.txt"))
                    && let Ok(best) = text.trim().parse()
                {
                    loaded.profile.best_score = best;
                    loaded.origin = Origin::Legacy;
                }
            }
            Read::Missing => {}
        }
        loaded
    }
    /// Writes through a synced temporary file. The previous save becomes the
    /// backup only if it still decodes, so the backup is always last-known-good.
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
        if matches!(read(path), Read::Valid(_)) {
            fs::rename(path, backup_path(path))?;
        }
        fs::rename(temp, path)
    }
}

/// Where launch progress came from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Origin {
    Saved,
    Backup,
    Legacy,
    New,
}
#[derive(Debug)]
pub struct Loaded {
    pub profile: Profile,
    pub origin: Origin,
    /// Unreadable files moved out of the way, with the reason each failed.
    pub set_aside: Vec<(PathBuf, String)>,
    /// Set when saving would overwrite a file that could not be read or moved.
    pub blocked: Option<String>,
}
impl Loaded {
    fn quarantine(&mut self, file: &Path, why: String) {
        match set_aside(file) {
            Ok(moved) => self.set_aside.push((moved, why)),
            Err(e) => {
                self.blocked = Some(format!(
                    "{} is unreadable ({why}) and could not be moved aside ({e})",
                    file.display()
                ))
            }
        }
    }
}

/// Real profiles are a few hundred bytes; anything larger is not one of ours
/// and must not stall startup.
const READ_LIMIT: u64 = 64 * 1024;
enum Read {
    Missing,
    Valid(Profile),
    Invalid(String),
}
fn read(path: &Path) -> Read {
    use std::io::Read as _;
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Read::Missing,
        Err(e) => return Read::Invalid(e.to_string()),
    };
    let mut bytes = Vec::new();
    if let Err(e) = file.take(READ_LIMIT + 1).read_to_end(&mut bytes) {
        return Read::Invalid(e.to_string());
    }
    if bytes.len() as u64 > READ_LIMIT {
        return Read::Invalid("larger than 64 KiB".into());
    }
    // Invalid UTF-8 only spoils the lines it touches; decode skips those.
    match Profile::decode(&String::from_utf8_lossy(&bytes)) {
        Some(p) => Read::Valid(p),
        None => Read::Invalid("not an ARKONK 1 profile".into()),
    }
}
fn backup_path(path: &Path) -> PathBuf {
    path.with_extension("bak")
}
fn set_aside(path: &Path) -> io::Result<PathBuf> {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let name = path
        .file_name()
        .map_or_else(|| "progress".into(), |n| n.to_string_lossy().into_owned());
    let target = path.with_file_name(format!("unreadable-{stamp}-{name}"));
    fs::rename(path, &target)?;
    Ok(target)
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
            fullscreen: true,
            ..Profile::default()
        };
        p.records[5] = Record {
            medals: Medals::ALL,
            best_ticks: 15400,
        };
        p.begin(&Game::start(SectorId::new(6).unwrap(), Mode::Journey));
        assert_eq!(Profile::decode(&p.encode()), Some(p));
    }
    #[test]
    fn corrupt_values_cannot_unlock_invalid_sectors_or_lives() {
        let p = Profile::decode(
            "ARKONK 1\nunlocked 999\nsettings 0 1 300\ncheckpoint 20 0 90 0\nrecord 500 7 0\nrecord 0 255 25\nbest nonsense",
        )
        .unwrap();
        assert_eq!(p.unlocked, SECTOR_COUNT);
        assert_eq!(p.volume, 10);
        assert_eq!(p.checkpoint, None);
        assert_eq!(p.records[0].medals, Medals::ALL);
        assert_eq!(Profile::decode("unknown version"), None);
        assert_eq!(Profile::decode(""), None);
    }
    #[test]
    fn legacy_settings_keep_sound_and_volume() {
        let p = Profile::decode("ARKONK 1\nsettings 1 0 4\n").unwrap();
        assert!(p.muted);
        assert_eq!(p.volume, 4);
    }
    #[test]
    fn practice_records_do_not_replace_journey_checkpoint() {
        let mut p = Profile::default();
        p.begin(&Game::new());
        let saved = p.checkpoint;
        let mut g = Game::start(SectorId::FIRST, Mode::Practice);
        g.stage = Stage::Cleared;
        g.summary.medals = Medals::ALL;
        g.summary.ticks = 1200;
        g.score = 5000;
        p.finish(&g);
        g.summary.ticks = 1500;
        g.summary.medals = Medals::CLEAR;
        p.finish(&g);
        assert_eq!(p.checkpoint, saved);
        assert_eq!(p.best_score, 0);
        assert_eq!(
            p.records[0],
            Record {
                medals: Medals::ALL,
                best_ticks: 1200
            }
        );
        assert_eq!(p.unlocked, 2);
    }
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("arkonk-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir.join("progress.txt")
    }
    fn sample() -> Profile {
        let mut p = Profile {
            unlocked: 5,
            best_score: 24600,
            ..Profile::default()
        };
        p.records[0].medals = Medals::ALL;
        p.records[3].medals = Medals::CLEAR;
        p
    }
    fn set_aside_files(path: &Path) -> Vec<Vec<u8>> {
        let mut files: Vec<_> = fs::read_dir(path.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("unreadable-")
            })
            .collect();
        files.sort();
        files.iter().map(|p| fs::read(p).unwrap()).collect()
    }
    #[test]
    fn saving_replaces_existing_profile() {
        let path = scratch("replace");
        let mut p = Profile::default();
        p.save(&path).unwrap();
        p.best_score = 123;
        p.save(&path).unwrap();
        let loaded = Profile::load(&path);
        assert_eq!((loaded.profile, loaded.origin), (p, Origin::Saved));
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
    #[test]
    fn previous_save_becomes_the_backup() {
        let path = scratch("rotate");
        let first = sample();
        first.save(&path).unwrap();
        let mut second = first.clone();
        second.best_score = 30000;
        second.save(&path).unwrap();
        assert_eq!(read_valid(&backup_path(&path)), first);
        assert_eq!(read_valid(&path), second);
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
    fn read_valid(path: &Path) -> Profile {
        match read(path) {
            Read::Valid(p) => p,
            _ => panic!("{} is not a valid profile", path.display()),
        }
    }
    #[test]
    fn one_corrupted_byte_keeps_everything_else() {
        let path = scratch("byte");
        let p = sample();
        let mut bytes = p.encode().into_bytes();
        // Damage only the record line for sector 4.
        let at = p.encode().find("record 3").unwrap() + 2;
        bytes[at] = 0xff;
        fs::write(&path, bytes).unwrap();
        let loaded = Profile::load(&path);
        assert_eq!(loaded.origin, Origin::Saved);
        let mut expected = p;
        expected.records[3] = Record::default();
        assert_eq!(loaded.profile, expected);
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
    #[test]
    fn corrupt_main_recovers_from_backup_and_is_preserved() {
        let path = scratch("recover");
        let good = sample();
        fs::write(backup_path(&path), good.encode()).unwrap();
        fs::write(&path, b"\0\0garbage\xff").unwrap();
        let loaded = Profile::load(&path);
        assert_eq!(loaded.origin, Origin::Backup);
        assert_eq!(loaded.profile, good);
        assert_eq!(loaded.blocked, None);
        assert_eq!(set_aside_files(&path), [b"\0\0garbage\xff".to_vec()]);
        // The next save must not rotate anything unreadable into the backup.
        let mut next = good.clone();
        next.best_score += 1;
        next.save(&path).unwrap();
        assert_eq!(read_valid(&backup_path(&path)), good);
        assert_eq!(read_valid(&path), next);
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
    #[test]
    fn missing_main_recovers_from_backup() {
        // A crash between the two renames in `save` leaves only the backup.
        let path = scratch("missing");
        fs::write(backup_path(&path), sample().encode()).unwrap();
        let loaded = Profile::load(&path);
        assert_eq!((loaded.profile, loaded.origin), (sample(), Origin::Backup));
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
    #[test]
    fn both_corrupt_start_fresh_without_destroying_either() {
        let path = scratch("both");
        fs::write(&path, "ARKONK 9\nbest 1").unwrap();
        fs::write(backup_path(&path), vec![b'x'; READ_LIMIT as usize + 1]).unwrap();
        let loaded = Profile::load(&path);
        assert_eq!(loaded.origin, Origin::New);
        assert_eq!(loaded.profile, Profile::default());
        assert_eq!(loaded.set_aside.len(), 2);
        Profile::default().save(&path).unwrap();
        Profile::default().save(&path).unwrap();
        let preserved = set_aside_files(&path);
        assert!(preserved.contains(&b"ARKONK 9\nbest 1".to_vec()));
        assert!(preserved.iter().any(|f| f.len() == READ_LIMIT as usize + 1));
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn unreadable_file_that_cannot_move_blocks_saving() {
        use std::os::unix::fs::PermissionsExt;
        let path = scratch("blocked");
        let dir = path.parent().unwrap();
        // A directory in place of the file cannot be read as a profile.
        fs::create_dir(&path).unwrap();
        fs::set_permissions(dir, fs::Permissions::from_mode(0o555)).unwrap();
        let loaded = Profile::load(&path);
        let saved = Profile::default().save(&path);
        fs::set_permissions(dir, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(loaded.blocked.is_some());
        assert_eq!(loaded.origin, Origin::New);
        assert!(
            saved.is_err(),
            "a read-only data directory reports an error"
        );
        fs::remove_dir_all(dir).unwrap();
    }
}
