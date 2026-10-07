//! A bounded plain-text log and crash reports under the data directory's `logs/`.
//! Release builds may have no console, so these files are the only record.
use std::{
    fmt::{Display, Write as _},
    fs::{self, File, OpenOptions},
    io::{self, Write},
    panic,
    path::{Path, PathBuf},
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

/// Larger logs are rotated to `arkonk.1.log` at launch, and one session never
/// writes more than this, so the folder stays under twice this size plus reports.
const LOG_LIMIT: u64 = 256 * 1024;
const CRASH_REPORTS_KEPT: usize = 10;

struct Log {
    file: File,
    written: u64,
}
static LOG: Mutex<Option<Log>> = Mutex::new(None);
static LOG_DIR: OnceLock<PathBuf> = OnceLock::new();
static CONTEXT: OnceLock<String> = OnceLock::new();
static BACKEND: OnceLock<&'static str> = OnceLock::new();
static WORKER_PANICKED: AtomicBool = AtomicBool::new(false);

/// Per-user data directory for progress and logs. `ARKONK_DATA_DIR` overrides
/// it so a test run or a support session never touches real progress.
pub fn data_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("ARKONK_DATA_DIR") {
        return Some(PathBuf::from(dir));
    }
    #[cfg(target_os = "macos")]
    let base =
        std::env::var_os("HOME").map(|p| PathBuf::from(p).join("Library/Application Support"));
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share")));
    base.map(|p| p.join("arkonk"))
}

/// Installs the panic hook, then opens the log when a directory is given.
/// Without one (test modes), messages still reach stderr.
pub fn init(logs: Option<PathBuf>) {
    let _ = CONTEXT.set(format!(
        "ARKONK {} / {} {} ({})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
        os_version()
    ));
    install_hook();
    let Some(dir) = logs else { return };
    match open_log(&dir) {
        Ok(log) => {
            if let Ok(mut slot) = LOG.lock() {
                *slot = Some(log);
            }
            prune_crash_reports(&dir);
            let _ = LOG_DIR.set(dir);
        }
        Err(e) => eprintln!("Could not open log in {}: {e}", dir.display()),
    }
}

pub fn set_backend(name: &'static str) {
    let _ = BACKEND.set(name);
}

pub fn info(message: impl Display) {
    write_line("info", message);
}

pub fn error(message: impl Display) {
    write_line("error", message);
}

/// The only worker thread in the process is the Windows/Linux audio mixer, which
/// panics when no output device opens. After that, sound must stay off.
pub fn worker_panicked() -> bool {
    WORKER_PANICKED.load(Ordering::Relaxed)
}

fn write_line(level: &str, message: impl Display) {
    let line = format!("{} {level} {message}\n", timestamp(now()));
    eprint!("{line}");
    // try_lock: a panic raised while logging must not deadlock the panic hook.
    let Ok(mut slot) = LOG.try_lock() else { return };
    let Some(log) = slot.as_mut() else { return };
    if log.written >= LOG_LIMIT {
        return;
    }
    let text = if log.written + line.len() as u64 >= LOG_LIMIT {
        "log size limit reached; further messages go to stderr only\n"
    } else {
        &line
    };
    // Logging is the error channel; a failed write has nowhere better to go.
    if log.file.write_all(text.as_bytes()).is_ok() {
        log.written += text.len() as u64;
    }
}

fn open_log(dir: &Path) -> io::Result<Log> {
    fs::create_dir_all(dir)?;
    let path = dir.join("arkonk.log");
    if fs::metadata(&path).is_ok_and(|m| m.len() > LOG_LIMIT) {
        fs::rename(&path, dir.join("arkonk.1.log"))?;
    }
    let file = OpenOptions::new().create(true).append(true).open(path)?;
    Ok(Log { file, written: 0 })
}

fn prune_crash_reports(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut reports: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("crash-") && n.ends_with(".txt"))
        })
        .collect();
    // Timestamped names sort chronologically.
    reports.sort();
    let excess = reports.len().saturating_sub(CRASH_REPORTS_KEPT);
    for old in &reports[..excess] {
        if let Err(e) = fs::remove_file(old) {
            error(format_args!(
                "Could not remove old crash report {}: {e}",
                old.display()
            ));
        }
    }
}

fn install_hook() {
    let main = thread::current().id();
    let previous = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let thread = thread::current();
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or("non-text panic payload");
        let location = info
            .location()
            .map_or_else(|| "unknown".to_string(), ToString::to_string);
        if thread.id() == main {
            let report = crash_report(
                message,
                &location,
                &std::backtrace::Backtrace::force_capture().to_string(),
            );
            match LOG_DIR.get().map(|dir| write_crash_report(dir, &report)) {
                Some(Ok(path)) => eprintln!("Crash report: {}", path.display()),
                Some(Err(e)) => eprintln!("Could not write crash report: {e}"),
                None => {}
            }
            error(format_args!("panic: {message} at {location}"));
        } else {
            WORKER_PANICKED.store(true, Ordering::Relaxed);
            error(format_args!(
                "{} thread stopped: {message} at {location}",
                thread.name().unwrap_or("worker")
            ));
        }
        previous(info);
    }));
}

fn crash_report(message: &str, location: &str, backtrace: &str) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "ARKONK crash report");
    let _ = writeln!(out, "time: {}", timestamp(now()));
    let _ = writeln!(
        out,
        "build: {}",
        CONTEXT.get().map_or("unknown", String::as_str)
    );
    let _ = writeln!(out, "backend: {}", BACKEND.get().unwrap_or(&"not started"));
    let _ = writeln!(out, "panic: {message}");
    let _ = writeln!(out, "at: {location}");
    let _ = writeln!(out, "\nbacktrace:\n{backtrace}");
    out
}

fn write_crash_report(dir: &Path, report: &str) -> io::Result<PathBuf> {
    let name = format!("crash-{}.txt", timestamp(now()).replace(':', "-"));
    let path = dir.join(name);
    fs::write(&path, report)?;
    Ok(path)
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// UTC ISO 8601 without a calendar dependency (Hinnant's civil-from-days).
fn timestamp(unix: u64) -> String {
    let days = (unix / 86_400) as i64;
    let seconds = unix % 86_400;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    )
}

fn os_version() -> String {
    #[cfg(target_os = "macos")]
    let version = fs::read_to_string("/System/Library/CoreServices/SystemVersion.plist")
        .ok()
        .and_then(|plist| {
            let after = plist.split("<key>ProductVersion</key>").nth(1)?;
            let value = after.split("<string>").nth(1)?.split("</string>").next()?;
            Some(format!("macOS {value}"))
        });
    // SteamOS and other distributions identify themselves here.
    #[cfg(target_os = "linux")]
    let version = fs::read_to_string("/etc/os-release").ok().and_then(|text| {
        text.lines()
            .find_map(|l| l.strip_prefix("PRETTY_NAME="))
            .map(|v| v.trim_matches('"').to_string())
    });
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let version: Option<String> = None;
    version.unwrap_or_else(|| "version unknown".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_are_utc_calendar_dates() {
        assert_eq!(timestamp(0), "1970-01-01T00:00:00Z");
        assert_eq!(timestamp(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(timestamp(1_791_322_212), "2026-10-06T21:30:12Z");
    }

    #[test]
    fn crash_report_names_the_failure() {
        let report = crash_report("boom", "src/main.rs:1:1", "frame 0");
        for part in [
            "panic: boom",
            "at: src/main.rs:1:1",
            "backend:",
            "build:",
            "frame 0",
        ] {
            assert!(report.contains(part), "{part} missing from {report}");
        }
    }

    #[test]
    fn worker_panic_marks_audio_lost_without_a_crash_report() {
        // The test thread stands in for the main thread.
        install_hook();
        assert!(thread::spawn(|| panic!("no audio device")).join().is_err());
        assert!(worker_panicked());
    }

    #[test]
    fn oversized_log_rotates_and_old_reports_are_pruned() {
        let dir = std::env::temp_dir().join(format!("arkonk-logs-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("arkonk.log"), vec![b'x'; LOG_LIMIT as usize + 1]).unwrap();
        for i in 0..CRASH_REPORTS_KEPT + 3 {
            fs::write(dir.join(format!("crash-2026-01-{i:02}.txt")), "").unwrap();
        }
        let log = open_log(&dir).unwrap();
        assert_eq!(log.file.metadata().unwrap().len(), 0);
        assert_eq!(
            fs::metadata(dir.join("arkonk.1.log")).unwrap().len(),
            LOG_LIMIT + 1
        );
        prune_crash_reports(&dir);
        assert!(!dir.join("crash-2026-01-02.txt").exists());
        assert!(dir.join("crash-2026-01-03.txt").exists());
        fs::remove_dir_all(dir).unwrap();
    }
}
