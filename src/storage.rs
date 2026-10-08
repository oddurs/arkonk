//! The save file on disk: `progress.txt`, holding progress and settings.
//!
//! Writes go through a synced temporary file, and the previous save is kept
//! as `progress.bak` only while it still reads. A file that exists but cannot
//! be read is never treated as absent: it is moved aside, or, if that fails,
//! saving is blocked so it cannot be overwritten. Disk access happens at
//! menu and sector boundaries only.
use crate::settings::Settings;
use ark::progress::{Progress, entries};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Everything saved: the player's progress and settings.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Profile {
    pub progress: Progress,
    pub settings: Settings,
}

impl Profile {
    /// `None` when `file` is not a save. A version-1 save is mapped onto
    /// the 64-sector journey; the next save writes version 2.
    pub fn decode(file: &[u8]) -> Option<Self> {
        Some(Self {
            progress: Progress::decode(file)?,
            settings: Settings::decode(entries(file)?, file),
        })
    }

    /// The save file's text.
    pub fn encode(&self) -> String {
        let mut out = String::new();
        self.progress
            .encode(&mut out, |out| self.settings.encode(out))
            .expect("writing to a String cannot fail");
        out
    }

    /// Loads progress, falling back to the last-known-good backup.
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
                    loaded.profile.progress.raise_best_score(best);
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

/// Real saves are a few hundred bytes; anything larger is not one of ours
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
    match Profile::decode(&bytes) {
        Some(p) => Read::Valid(p),
        None => Read::Invalid("not an ARKONK profile".into()),
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
    use ark::{Game, Mode, sectors::SectorId};

    fn profile(file: &str) -> Profile {
        Profile::decode(file.as_bytes()).unwrap()
    }

    /// Marks every line end, so a snapshot also pins the final newline and
    /// shows that no line ends in `\r`.
    fn visible(file: &str) -> String {
        file.replace('\n', "\\n\n")
    }

    // The version-2 file, byte for byte. Version 1's layout is kept, with
    // only earned records written.

    #[test]
    fn encoded_layout() {
        // Out of order on purpose: the layout comes from the encoder.
        let p = profile(
            "ARKONK 2\ncheckpoint 6 31250 4 99000\nrecord 5 3 30001\nsettings 1 3\n\
             display 1\nrecord 0 7 15400\nunlocked 7\nbest 24600\nrecord 40 0 0\n",
        );
        insta::assert_snapshot!(visible(&p.encode()));
    }

    #[test]
    fn default_layout() {
        insta::assert_snapshot!(visible(&Profile::default().encode()));
    }

    /// Damaged, duplicated, out-of-range and unknown lines, decoded and
    /// written back: the result pins which lines survive and how values are
    /// clamped.
    #[test]
    fn damaged_file_rewritten() {
        let file: &[u8] = b"ARKONK 2\r\n\
            best 100\r\n\
            best\xc2\xa05\n\
            best 24600\n\
            unlocked 999\n\
            settings 1 0 4\n\
            display 1\n\
            record 3 255 7000\n\
            record 64 7 1\n\
            record 0 7 1 2\n\
            record 1 \xff 9\n\
            record 2 3 +40\n\
            checkpoint 5 31250 4 99000\n\
            future 1 2 3 4 5\n\
            \tunlocked\t11 \n\
            volume 3\n";
        let p = Profile::decode(file).unwrap();
        insta::assert_snapshot!(visible(&p.encode()));
    }

    #[test]
    fn a_save_from_before_languages_still_loads() {
        let file = "ARKONK 1\nbest 24600\nunlocked 3\nsettings 1 4\ndisplay 1\n\
                    record 0 7 15400\nrecord 1 3 0\nrecord 2 0 0\nrecord 3 0 0\n\
                    record 4 0 0\nrecord 5 0 0\nrecord 6 0 0\nrecord 7 0 0\n\
                    record 8 0 0\nrecord 9 0 0\nrecord 10 0 0\nrecord 11 0 0\n\
                    checkpoint 2 3100 3 9000\n";
        let p = profile(file);
        assert_eq!(p.settings.locale, None);
        assert_eq!((p.settings.muted, p.settings.volume), (true, 4));
        assert_eq!(
            p.encode(),
            "ARKONK 2\nbest 24600\nunlocked 17\nsettings 1 4\ndisplay 1\n\
             record 0 7 15400\nrecord 8 3 0\ncheckpoint 16 3100 3 9000\n"
        );
    }

    /// A save the last 12-sector build wrote (`tests/fixtures`): a German
    /// player six sectors into a journey, with reduced effects. Everything
    /// moves with its sector's slug, and the settings are untouched.
    #[test]
    fn a_real_version_1_save_moves_onto_the_journey() {
        use ark::{Medals, progress::V1_SECTORS};
        let file = include_bytes!("../tests/fixtures/progress-v1.txt");
        let old = std::str::from_utf8(file).unwrap();
        assert!(old.starts_with("ARKONK 1\n"));
        let p = Profile::decode(file).unwrap();
        let s = &p.settings;
        assert_eq!((s.volume, s.locale), (7, Some(ark_text::Locale::De)));
        assert!(s.reduced_effects && !s.high_contrast);
        let progress = &p.progress;
        assert_eq!(progress.best_score(), 60800);
        for (old, slug) in V1_SECTORS.iter().enumerate().take(6) {
            let line = old_line(old_text(file), old);
            let id = SectorId::from_slug(slug).unwrap();
            let record = progress.record(id);
            assert_eq!(record.medals, Medals::ALL, "{slug}");
            assert_eq!(line, format!("record {old} 7 {}", record.best_ticks));
        }
        // The seventh old sector, Undertow, holds the checkpoint; it is now
        // sector 28, and everything up to it is open.
        let undertow = SectorId::from_slug("undertow").unwrap();
        let checkpoint = progress.checkpoint().unwrap();
        assert_eq!(checkpoint.sector, undertow);
        assert_eq!((checkpoint.score, checkpoint.lives), (60800, 4));
        assert_eq!(progress.unlocked_count(), undertow.index() + 1);
        assert_eq!(progress.medal_count(), 18);
        let rewritten = p.encode();
        insta::assert_snapshot!(visible(&rewritten));
        assert_eq!(Profile::decode(rewritten.as_bytes()), Some(p));
    }
    fn old_text(file: &[u8]) -> &str {
        std::str::from_utf8(file).unwrap()
    }
    /// The `record` line for version-1 sector `old`.
    fn old_line(text: &str, old: usize) -> String {
        let key = format!("record {old} ");
        text.lines()
            .find(|l| l.starts_with(&key))
            .unwrap()
            .to_owned()
    }

    #[test]
    fn progress_and_settings_round_trip() {
        let mut p = profile("ARKONK 2\nbest 24600\nunlocked 7\nrecord 5 7 15400\n");
        p.settings = Settings {
            muted: true,
            volume: 3,
            fullscreen: true,
            locale: Some(ark_text::Locale::Ja),
            reduced_effects: true,
            high_contrast: true,
        };
        p.progress
            .begin(&Game::start(SectorId::new(6).unwrap(), Mode::Journey));
        assert_eq!(Profile::decode(p.encode().as_bytes()), Some(p));
        assert_eq!(Profile::decode(b"unknown version"), None);
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("arkonk-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir.join("progress.txt")
    }
    const SAMPLE: &str = "ARKONK 2\nbest 24600\nunlocked 5\nrecord 0 7 0\nrecord 3 1 0\n";
    fn sample() -> Profile {
        profile(SAMPLE)
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
        p.progress.raise_best_score(123);
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
        second.progress.raise_best_score(30000);
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
        let expected = profile(&SAMPLE.replace("record 3 1 0\n", ""));
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
        next.progress
            .raise_best_score(good.progress.best_score() + 1);
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
    fn legacy_best_score_is_imported_when_nothing_else_exists() {
        let path = scratch("legacy");
        fs::write(path.with_file_name("best.txt"), "  4321\n").unwrap();
        let loaded = Profile::load(&path);
        assert_eq!(loaded.origin, Origin::Legacy);
        assert_eq!(loaded.profile.progress.best_score(), 4321);
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
