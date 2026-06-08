//! Minimal append-only file logger for S-012.
//!
//! **Why hand-roll** instead of `simplelog` / `flexi_logger` / `fern`:
//! - simplelog pulls in `chrono` (transitively); we already avoid chrono for
//!   timestamp parsing (see `session::parse_iso8601_utc`)
//! - flexi_logger pulls in async runtime bits we don't need
//! - The constitution favours minimal deps for an open-source tray app
//!
//! Format matches `architecture § 9.3` + `story-012 § AC`:
//!     2026-06-08T14:08:30Z [INFO] startup: tray icon registered
//!
//! Failures (log dir not creatable, file lock poisoned, write error) are
//! silently swallowed — logging must NEVER affect main app behaviour
//! (addendum § A.6).

use log::{Level, Log, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// macOS standard log location.
fn log_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    Some(
        home.join("Library/Logs/com.caiyiwen.claude-code-monitor")
            .join("main.log"),
    )
}

/// Append-only file logger. The Mutex guards a single open File handle —
/// `Log::log` is invoked from arbitrary threads, so we serialise writes.
struct FileLogger {
    file: Mutex<File>,
    max_level: Level,
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let ts = format_iso8601_now();
        // Best-effort: if the lock is poisoned (a previous thread panicked
        // while holding it), recover the inner mutex and write anyway.
        let mut file = match self.file.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        // Both writeln and flush are silent on failure (disk full / etc).
        let _ = writeln!(*file, "{} [{}] {}", ts, record.level(), record.args());
        let _ = file.flush();
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}

/// Initialise the global logger.
///
/// Returns `Ok(path)` so the caller can log a startup banner that includes
/// the log file path. Returns `Err(reason)` if initialisation failed — the
/// caller should print the reason to stderr and continue (no logging is
/// always better than no app).
pub fn init() -> Result<PathBuf, String> {
    let path = log_path().ok_or_else(|| "home directory not resolvable".to_string())?;
    let dir = path.parent().expect("log_path always has parent");
    std::fs::create_dir_all(dir).map_err(|e| format!("create log dir: {}", e))?;

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("open log file: {}", e))?;

    // debug build = DEBUG (one line per refresh tick for inspection);
    // release build = INFO (silent on the happy path).
    let level = if cfg!(debug_assertions) {
        Level::Debug
    } else {
        Level::Info
    };

    let logger = FileLogger {
        file: Mutex::new(file),
        max_level: level,
    };
    log::set_boxed_logger(Box::new(logger)).map_err(|e| format!("set_boxed_logger: {}", e))?;
    log::set_max_level(level.to_level_filter());
    Ok(path)
}

// ============================================================================
// ISO 8601 formatting (UTC, second precision) — inverse of
// `session::days_from_civil`. Hand-rolled to keep zero new deps.
// ============================================================================

fn format_iso8601_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86400) as i64;
    let time_secs = (secs % 86400) as u32;
    let (y, m, d) = civil_from_days(days);
    let h = time_secs / 3600;
    let mi = (time_secs / 60) % 60;
    let s = time_secs % 60;
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", y, m, d, h, mi, s)
}

/// Howard Hinnant's days-to-civil algorithm. Public domain.
/// See https://howardhinnant.github.io/date_algorithms.html.
/// Input: days since Unix epoch (1970-01-01).
/// Output: (year, month [1..12], day [1..31]).
fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = (yoe as i32) + (era as i32) * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_from_days_epoch() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn civil_from_days_day_one() {
        assert_eq!(civil_from_days(1), (1970, 1, 2));
    }

    #[test]
    fn civil_from_days_known_anchor() {
        // 20577 days after epoch = 2026-05-04 (verified by hand in S-005).
        assert_eq!(civil_from_days(20577), (2026, 5, 4));
    }

    #[test]
    fn civil_from_days_leap_year_feb29() {
        // 2024-02-29 = 19782 days after epoch.
        assert_eq!(civil_from_days(19782), (2024, 2, 29));
    }

    #[test]
    fn iso8601_now_shape() {
        let s = format_iso8601_now();
        // Loose shape check: 4-digit year, dashes/colons in fixed positions,
        // trailing Z. Don't assert exact value since "now" varies.
        assert_eq!(s.len(), 20, "got {:?}", s);
        assert!(s.ends_with('Z'));
        assert_eq!(s.chars().nth(4), Some('-'));
        assert_eq!(s.chars().nth(7), Some('-'));
        assert_eq!(s.chars().nth(10), Some('T'));
        assert_eq!(s.chars().nth(13), Some(':'));
        assert_eq!(s.chars().nth(16), Some(':'));
    }
}
