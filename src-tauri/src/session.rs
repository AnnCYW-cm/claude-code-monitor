use libproc::libproc::bsd_info::BSDInfo;
use libproc::libproc::proc_pid;
use libproc::processes::{pids_by_type, ProcFilter};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::os::raw::{c_int, c_void};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// How fresh a JSONL file must be to count as "active". JSONL is append-only
/// by claude — if a session's transcript hasn't been written to in this
/// window we assume the session is dead even though the process is still
/// listed (S-002 acceptance + addendum § A.1).
const ACTIVE_JSONL_WINDOW: Duration = Duration::from_secs(60);

/// Classification of a claude session's current state.
///
/// Serialized as lowercase ("waiting" / "working" / "unknown") to match the
/// frontend TypeScript contract
/// (see [data-model.md § 1.2](../../../../docs/bmad/03-solutioning/data-model.md)).
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Waiting,
    Working,
    Unknown,
}

/// Session DTO crossing IPC to frontend.
/// Schema mirrored in `src/main.ts` (snake_case kept across the boundary).
/// See [docs/bmad/03-solutioning/data-model.md § 1.1](../../../../docs/bmad/03-solutioning/data-model.md).
#[derive(Serialize, Clone, Debug)]
pub struct Session {
    pub pid: u32,
    pub cwd: String,
    pub status: SessionStatus,
    pub last_message: Option<String>,
    pub last_update_unix: Option<u64>,
    /// For `Waiting` sessions: when the assistant finished its turn
    /// (= `last_update_unix` at the moment we observed Waiting). Used by the
    /// UI to render "已等 N min" and by sort logic to surface oldest-wait first.
    /// `None` for Working / Unknown — and for Waiting sessions whose JSONL
    /// timestamp couldn't be parsed.
    pub waiting_since_unix: Option<u64>,
}

/// Raw process info, internal type (not crossing IPC).
/// Output of S-001 process enumeration; consumed by S-002 (JSONL locator).
/// See [docs/bmad/03-solutioning/data-model.md § 2.1](../../../../docs/bmad/03-solutioning/data-model.md).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawProcess {
    pub pid: u32,
    pub cwd: PathBuf,
    pub started_at: SystemTime,
}

/// Enumerate currently running `claude` CLI processes owned by the current user.
///
/// **Why libproc + raw FFI instead of `sysinfo`** (2026-06-01, dogfood finding):
/// `sysinfo` on macOS returns the underlying executable name (`"node"` for the
/// Node-based Claude Code CLI), not the `comm` set via `process.title`, so the
/// real `claude` CLI was being missed entirely. `sysinfo` also can't read other
/// processes' `cwd` on macOS without root. We use libproc (the same API
/// Activity Monitor uses) for name + uid + start time, and a hand-rolled
/// `proc_pidinfo(PROC_PIDVNODEPATHINFO)` FFI for cwd.
///
/// Filters:
/// - process name (`comm`) is one of `claude`, `claude.exe`, `claude-code`
/// - process UID == current process UID (skips sudo-run instances and other
///   users' processes; see [addendum § A.6](../../../../docs/bmad/02-planning/addendum.md))
/// - cwd is resolvable via VnodePathInfo
///
/// Cost target: < 25ms for 10 processes (half of NFR-P1 50ms budget).
pub fn list_processes() -> Vec<RawProcess> {
    let current_uid = current_user_uid();
    let pids = pids_by_type(ProcFilter::All).unwrap_or_default();
    pids.into_iter()
        .filter_map(|pid| extract_raw(pid as i32, current_uid))
        .collect()
}

/// Get the current effective UID — used to filter to "our user's" claude
/// processes. Always returns `Some` in practice; `None` is reserved for the
/// rare case where we want to fail-open (e.g. tests that don't run as a
/// real user).
fn current_user_uid() -> Option<u32> {
    // SAFETY: getuid() is always safe to call.
    Some(unsafe { libc::getuid() })
}

/// Pure helper: does this comm name match one of the known Claude Code CLI
/// identifiers? Exposed for unit tests.
///
/// Verified 2026-06-01: actual `comm` on macOS for npm-installed Claude Code
/// 2.1.x is `claude.exe`. The `claude` and `claude-code` variants are kept as
/// belt-and-braces for older versions / homebrew installs / shim wrappers.
pub(crate) fn is_claude_name(lowered: &str) -> bool {
    matches!(lowered, "claude" | "claude.exe" | "claude-code")
}

/// Pure helper: process UID matches current. Fail-open when current_uid is
/// `None` (treat as "we don't know, accept it").
pub(crate) fn uid_matches_current(proc_uid: u32, current_uid: Option<u32>) -> bool {
    current_uid.is_none_or(|c| proc_uid == c)
}

/// Pull RawProcess for a single PID via libproc (name + BSDInfo) and raw FFI
/// (cwd). Returns `None` if the process isn't a claude CLI, isn't ours,
/// has no cwd, or has exited mid-scan.
fn extract_raw(pid: i32, current_uid: Option<u32>) -> Option<RawProcess> {
    let name = proc_pid::name(pid).ok()?;
    if !is_claude_name(&name.to_lowercase()) {
        return None;
    }
    let bsd = proc_pid::pidinfo::<BSDInfo>(pid, 0).ok()?;
    if !uid_matches_current(bsd.pbi_uid, current_uid) {
        return None;
    }
    let cwd = cwd_for_pid(pid)?;
    let started_at = UNIX_EPOCH + Duration::from_secs(bsd.pbi_start_tvsec);
    Some(RawProcess {
        pid: pid as u32,
        cwd,
        started_at,
    })
}

// ============================================================================
// Raw FFI: PROC_PIDVNODEPATHINFO for reading another process's cwd on macOS.
// libproc 0.14 explicitly returns "not implemented for macos" for pidcwd, so
// we drop down to libc. See sys/proc_info.h for the struct layout.
// ============================================================================

/// `proc_pidinfo` flavor 9 = `PROC_PIDVNODEPATHINFO`, returns the process's
/// current + root directories as `vnode_info_path` pairs.
const PROC_PIDVNODEPATHINFO: c_int = 9;

/// Hand-computed sizes from `<sys/proc_info.h>`:
/// - `vinfo_stat`         = 136 bytes (file stat-like)
/// - `vnode_info`         = vinfo_stat + int + int + fsid_t = 152
/// - `vnode_info_path`    = vnode_info + char[MAXPATHLEN] = 152 + 1024 = 1176
/// - `proc_vnodepathinfo` = 2 × vnode_info_path = 2352 (cdir + rdir)
const VNODE_INFO_SIZE: usize = 152;
const MAXPATHLEN: usize = 1024;
const VNODE_INFO_PATH_SIZE: usize = VNODE_INFO_SIZE + MAXPATHLEN;
const PROC_VNODEPATHINFO_SIZE: usize = 2 * VNODE_INFO_PATH_SIZE;

extern "C" {
    fn proc_pidinfo(
        pid: c_int,
        flavor: c_int,
        arg: u64,
        buffer: *mut c_void,
        buffersize: c_int,
    ) -> c_int;
}

/// Read `pid`'s current working directory via `proc_pidinfo`.
/// Returns `None` if the process has exited, the kernel refuses (rare, only
/// in heavily sandboxed configurations), or the path is empty.
pub(crate) fn cwd_for_pid(pid: i32) -> Option<PathBuf> {
    let mut buf = vec![0u8; PROC_VNODEPATHINFO_SIZE];
    let ret = unsafe {
        proc_pidinfo(
            pid,
            PROC_PIDVNODEPATHINFO,
            0,
            buf.as_mut_ptr() as *mut c_void,
            PROC_VNODEPATHINFO_SIZE as c_int,
        )
    };
    if ret <= 0 {
        return None;
    }
    // cwd lives at offset VNODE_INFO_SIZE within pvi_cdir (which itself starts
    // at offset 0 of the returned buffer).
    let path_start = VNODE_INFO_SIZE;
    let path_end = path_start + MAXPATHLEN;
    let path_bytes = &buf[path_start..path_end];
    let nul = path_bytes.iter().position(|&b| b == 0).unwrap_or(0);
    if nul == 0 {
        None
    } else {
        Some(PathBuf::from(
            String::from_utf8_lossy(&path_bytes[..nul]).into_owned(),
        ))
    }
}

// ============================================================================
// S-002 · JSONL locator
// ============================================================================

/// Errors from locating a session's active JSONL transcript.
/// See [docs/bmad/03-solutioning/data-model.md § 2.3](../../../../docs/bmad/03-solutioning/data-model.md).
#[derive(Debug)]
pub enum LocateError {
    /// `~/.claude/projects/<encoded-cwd>/` doesn't exist.
    DirNotFound,
    /// Directory exists but contains no `*.jsonl` files (subdirectories ignored).
    NoJsonlFiles,
    /// Directory has `*.jsonl` files but none modified within the active window.
    NoActiveJsonl,
    /// Underlying filesystem error.
    IoError(io::Error),
}

impl From<io::Error> for LocateError {
    fn from(e: io::Error) -> Self {
        LocateError::IoError(e)
    }
}

impl std::fmt::Display for LocateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocateError::DirNotFound => write!(f, "encoded-cwd directory not found"),
            LocateError::NoJsonlFiles => write!(f, "no *.jsonl files in directory"),
            LocateError::NoActiveJsonl => {
                write!(f, "no *.jsonl modified within {:?}", ACTIVE_JSONL_WINDOW)
            }
            LocateError::IoError(e) => write!(f, "io: {}", e),
        }
    }
}

impl std::error::Error for LocateError {}

/// Pure helper: encode an absolute cwd into the directory-name claude uses
/// under `~/.claude/projects/`. Verified 2026-05-18 against Claude Code 2.1.126
/// (see [docs/spec/jsonl-schema.md § 1.1](../../../../docs/spec/jsonl-schema.md)).
///
/// Rule: replace every `/` with `-`, including the leading `/`.
///
/// ```
/// // /Users/caiyiwen           → -Users-caiyiwen
/// // /Users/caiyiwen/my-proj   → -Users-caiyiwen-my-proj
/// ```
pub(crate) fn encode_cwd(cwd: &str) -> String {
    cwd.replace('/', "-")
}

/// Locate the active JSONL transcript file for a running claude process.
///
/// Heuristic: under `~/.claude/projects/<encoded-cwd>/`, pick the `.jsonl`
/// file with the most recent mtime, provided it was modified within the last
/// [`ACTIVE_JSONL_WINDOW`] (60s). Subdirectories (containing per-session
/// cache/metadata) are ignored.
///
/// Known limitation: same-cwd multi-process pairing is a heuristic and can
/// mismatch under rare race conditions
/// (see [addendum § A.1](../../../../docs/bmad/02-planning/addendum.md)).
///
/// Cost target: < 5ms per process (S-002 acceptance).
pub fn locate_jsonl(proc: &RawProcess) -> Result<PathBuf, LocateError> {
    let home = dirs::home_dir().ok_or_else(|| {
        LocateError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            "home directory not resolvable",
        ))
    })?;
    let encoded = encode_cwd(&proc.cwd.to_string_lossy());
    let dir = home.join(".claude").join("projects").join(&encoded);

    locate_jsonl_in_dir(&dir, SystemTime::now())
}

/// Pure (well, IO-only-on-`dir`) helper for `locate_jsonl`. Split out so unit
/// tests can call it with a tempfile path + a fixed `now` for stale-window
/// determinism.
pub(crate) fn locate_jsonl_in_dir(dir: &Path, now: SystemTime) -> Result<PathBuf, LocateError> {
    if !dir.exists() {
        return Err(LocateError::DirNotFound);
    }

    let mut newest: Option<(PathBuf, SystemTime)> = None;
    let mut any_jsonl_seen = false;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip subdirectories — same-name `<uuid>/` dirs hold cache/metadata
        // and live alongside `<uuid>.jsonl` files (verified 2026-05-18).
        if !entry.file_type()?.is_file() {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
            continue;
        }

        any_jsonl_seen = true;
        let mtime = entry.metadata()?.modified()?;

        match &newest {
            None => newest = Some((path, mtime)),
            Some((_, current_mtime)) if mtime > *current_mtime => newest = Some((path, mtime)),
            _ => {}
        }
    }

    if !any_jsonl_seen {
        return Err(LocateError::NoJsonlFiles);
    }

    let (path, mtime) = newest.expect("any_jsonl_seen => newest is Some");

    // "Active" = modified within the window. Stale → caller treats session as
    // Unknown (next refresh will retry).
    match now.duration_since(mtime) {
        Ok(age) if age <= ACTIVE_JSONL_WINDOW => Ok(path),
        Ok(_) => Err(LocateError::NoActiveJsonl),
        // Clock skew: mtime in the future. Be permissive — accept it.
        Err(_) => Ok(path),
    }
}

// ============================================================================
// S-003 · JSONL tail reader
// ============================================================================

/// Initial chunk size for the seek-from-end tail scan. Most claude JSONL lines
/// are well under 4KB (a single envelope wrapping a short message). When the
/// last line is bigger we double the chunk and re-read.
const TAIL_INITIAL_CHUNK: u64 = 4096;

/// Read the last non-empty line of a file without slurping the whole thing.
///
/// Strategy: seek to `end - chunk`, scan that buffer for the last `\n`, return
/// what comes after. If no `\n` lands inside the chunk (last line is bigger
/// than the chunk), double the chunk size and retry. Worst case we read the
/// whole file — but typical JSONL lines are < 4KB so we usually read once.
///
/// Returns:
/// - `Ok(Some(line))` — the last non-empty line (trailing `\n`/`\r` stripped)
/// - `Ok(None)` — file is empty OR contains only whitespace newlines
/// - `Err(io::Error)` — underlying read failure
///
/// UTF-8 handling: we slice on byte boundaries (`\n` is single-byte ASCII so
/// this is safe) and decode the final slice with `from_utf8_lossy` to tolerate
/// any partial multi-byte sequence at the very start (which shouldn't happen
/// in well-formed JSONL but defends against truncated writes during scan).
///
/// Cost target: < 10ms for files up to 100MB (NFR-P1, story S-003).
pub fn tail_jsonl(path: &Path) -> Result<Option<String>, io::Error> {
    let mut file = File::open(path)?;
    let size = file.metadata()?.len();
    if size == 0 {
        return Ok(None);
    }

    let mut chunk_size: u64 = TAIL_INITIAL_CHUNK.min(size);
    let mut buf: Vec<u8> = Vec::new();

    loop {
        let offset = size - chunk_size;
        buf.clear();
        buf.resize(chunk_size as usize, 0);
        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(&mut buf)?;

        // Trim trailing \n and \r — these don't count as content.
        let trimmed_end = buf
            .iter()
            .rposition(|b| *b != b'\n' && *b != b'\r')
            .map(|i| i + 1)
            .unwrap_or(0);
        let trimmed = &buf[..trimmed_end];

        if trimmed.is_empty() {
            // Chunk is all newlines/whitespace. If we've already loaded the
            // entire file, the file itself is empty content.
            if offset == 0 {
                return Ok(None);
            }
            // Otherwise extend backwards.
        } else if let Some(nl) = trimmed.iter().rposition(|b| *b == b'\n') {
            let last_line = &trimmed[nl + 1..];
            return Ok(Some(String::from_utf8_lossy(last_line).into_owned()));
        } else if offset == 0 {
            // Whole file is one line (no newlines at all).
            return Ok(Some(String::from_utf8_lossy(trimmed).into_owned()));
        }

        // Need more bytes. Double the chunk, capped by file size.
        let next = (chunk_size.saturating_mul(2)).min(size);
        if next == chunk_size {
            // Already at full file; algorithmically can't make progress.
            // Reach here only if `trimmed.is_empty()` && offset == 0 was
            // false above, which can't happen — but be defensive.
            return Ok(Some(String::from_utf8_lossy(trimmed).into_owned()));
        }
        chunk_size = next;
    }
}

/// Read up to `max_lines` non-empty lines from the end of `path`, returned in
/// chronological order (oldest first, newest last). This is the multi-line
/// generalisation of [`tail_jsonl`] — needed because `last_meaningful` may
/// have to reverse-scan past several non-meaningful envelopes (attachment,
/// ai-title, file-history-snapshot, permission-mode, last-prompt) before
/// landing on a `user`/`assistant` entry.
///
/// Algorithm: same seek-from-end + chunk doubling as `tail_jsonl`, but we
/// keep collecting lines until either (a) we have `max_lines` of them past
/// the chunk boundary, or (b) we've already read the whole file.
///
/// `max_lines == 0` returns `Ok(vec![])`. Empty file returns `Ok(vec![])`.
///
/// Cost: O(`max_lines × typical_line_size`) — usually well under 100KB and
/// much faster than the 10ms tail budget in practice.
pub fn tail_lines(path: &Path, max_lines: usize) -> Result<Vec<String>, io::Error> {
    if max_lines == 0 {
        return Ok(Vec::new());
    }

    let mut file = File::open(path)?;
    let size = file.metadata()?.len();
    if size == 0 {
        return Ok(Vec::new());
    }

    let mut chunk_size: u64 = TAIL_INITIAL_CHUNK.min(size);
    let mut buf: Vec<u8> = Vec::new();

    loop {
        let offset = size - chunk_size;
        buf.clear();
        buf.resize(chunk_size as usize, 0);
        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(&mut buf)?;

        // Count newlines: we need >= max_lines + 1 newlines to be sure we
        // have `max_lines` complete lines whose start boundary is inside the
        // buffer (the +1 is the cut before the oldest line we'll return).
        // Exception: if offset == 0 we've reached the file head, so any
        // partial leading line counts.
        let newlines: Vec<usize> = buf
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if *b == b'\n' { Some(i) } else { None })
            .collect();

        let have_enough = newlines.len() > max_lines || offset == 0;
        if have_enough {
            return Ok(extract_last_n_nonempty_lines(
                &buf,
                &newlines,
                max_lines,
                offset == 0,
            ));
        }

        let next = (chunk_size.saturating_mul(2)).min(size);
        if next == chunk_size {
            // Already at full file — same as offset == 0.
            return Ok(extract_last_n_nonempty_lines(
                &buf, &newlines, max_lines, true,
            ));
        }
        chunk_size = next;
    }
}

/// Pull up to `max_lines` non-empty lines from `buf`, treating it either as a
/// suffix-fragment of a larger file (`at_head=false`, so the bytes before
/// the first `\n` are an incomplete line we must discard) or as the full file
/// (`at_head=true`, leading bytes are kept). Returned in chronological order.
fn extract_last_n_nonempty_lines(
    buf: &[u8],
    newlines: &[usize],
    max_lines: usize,
    at_head: bool,
) -> Vec<String> {
    // Determine the byte offset where line content starts. When we're at the
    // file head we start at 0; otherwise we start just past the first \n
    // (discarding the partial leading line).
    let start = if at_head {
        0
    } else {
        // First \n's position + 1. newlines is non-empty here because the
        // `have_enough` check ensures at least max_lines+1 newlines.
        newlines.first().map(|i| *i + 1).unwrap_or(buf.len())
    };

    // Split body into lines.
    let body = &buf[start..];
    let mut out: Vec<String> = Vec::with_capacity(max_lines);
    for raw in body.split(|b| *b == b'\n') {
        // Strip trailing \r (CRLF tolerance).
        let trimmed = raw.strip_suffix(b"\r").unwrap_or(raw);
        if trimmed.is_empty() {
            continue;
        }
        out.push(String::from_utf8_lossy(trimmed).into_owned());
    }

    // Keep only the last `max_lines`.
    if out.len() > max_lines {
        let drop = out.len() - max_lines;
        out.drain(..drop);
    }
    out
}

/// Parse a Claude JSONL `timestamp` field (ISO 8601 UTC, e.g.
/// `"2026-05-04T05:42:34.919Z"`) into seconds since the Unix epoch.
///
/// Pure stdlib — we avoid a `chrono` dependency because the format is rigid
/// (always UTC, always this exact shape per verified schema). The optional
/// fractional seconds are dropped (sub-second precision is not used by the UI).
///
/// Returns `None` on any parse failure, so caller treats unparseable timestamps
/// the same as missing ones.
pub(crate) fn parse_iso8601_utc(s: &str) -> Option<u64> {
    let s = s.trim_end_matches('Z');
    let (date, time) = s.split_once('T')?;

    let mut dp = date.split('-');
    let year: i32 = dp.next()?.parse().ok()?;
    let month: u32 = dp.next()?.parse().ok()?;
    let day: u32 = dp.next()?.parse().ok()?;
    if dp.next().is_some() {
        return None;
    }

    // Drop fractional seconds if present.
    let time_no_frac = time.split('.').next()?;
    let mut tp = time_no_frac.split(':');
    let hour: u32 = tp.next()?.parse().ok()?;
    let minute: u32 = tp.next()?.parse().ok()?;
    let second: u32 = tp.next()?.parse().ok()?;
    if tp.next().is_some() {
        return None;
    }

    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    // tolerate leap second
    {
        return None;
    }

    let days = days_from_civil(year, month, day);
    if days < 0 {
        return None; // dates before 1970 don't fit u64 epoch.
    }

    let total = (days as u64)
        .checked_mul(86_400)?
        .checked_add(hour as u64 * 3_600)?
        .checked_add(minute as u64 * 60)?
        .checked_add(second as u64)?;
    Some(total)
}

/// Howard Hinnant's days-from-civil algorithm: number of days from the Unix
/// epoch (1970-01-01) to the given proleptic Gregorian date. Negative for
/// dates before 1970. Public domain.
/// See https://howardhinnant.github.io/date_algorithms.html
fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as i64; // [0, 399]
    let m_shifted = if m > 2 { m - 3 } else { m + 9 } as i64;
    let doy = (153 * m_shifted + 2) / 5 + d as i64 - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era as i64 * 146097 + doe - 719468
}

// ============================================================================
// S-004 · JSONL envelope types + classifier
// ============================================================================

/// One line of a claude JSONL transcript. Layout verified 2026-05-18 against
/// Claude Code 2.1.126 — see [spec/jsonl-schema.md § 2](../../../../docs/spec/jsonl-schema.md).
///
/// `kind` is the top-level discriminator (`"type"` on the wire). Possible
/// values include `user`, `assistant`, `attachment`, `file-history-snapshot`,
/// `permission-mode`, `ai-title`, `last-prompt`. Only `user`/`assistant` carry
/// a nested `message` we care about for classification.
///
/// Unknown JSON fields are silently ignored (serde default).
#[derive(Deserialize, Debug)]
pub struct JsonlEnvelope {
    #[serde(rename = "type")]
    pub kind: String,
    pub uuid: String,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    pub timestamp: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub cwd: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    pub message: Option<NestedMessage>,
}

/// The nested message inside a `user`/`assistant` envelope.
/// See [data-model.md § 2.3](../../../../docs/bmad/03-solutioning/data-model.md).
#[derive(Deserialize, Debug)]
pub struct NestedMessage {
    pub role: String,
    #[serde(default)]
    pub content: ContentValue,
    pub stop_reason: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
}

/// Message content is either a plain string (typical for user input) or an
/// array of structured `ContentBlock`s (assistant responses + tool_result
/// wrappers). `serde(untagged)` lets us discriminate on shape.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentValue {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

impl Default for ContentValue {
    fn default() -> Self {
        ContentValue::Blocks(vec![])
    }
}

/// Discriminated by `"type"` field inside the block.
/// See [data-model.md § 2.4](../../../../docs/bmad/03-solutioning/data-model.md).
#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    Thinking {
        thinking: String,
        #[serde(default)]
        signature: Option<String>,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool_use_id: String,
        content: serde_json::Value,
    },
}

/// Reverse-scan a slice of JSONL lines and return the last envelope whose
/// `type` is `user` or `assistant`. Non-meaningful entries (attachment /
/// file-history-snapshot / permission-mode / ai-title / last-prompt) and
/// unparseable lines are skipped — the scan continues until we find a
/// meaningful envelope or reach the start of the slice.
///
/// Returns `None` when no meaningful envelope exists in the given lines.
///
/// **Important**: unlike the sample in data-model.md which used `?` on the
/// parse result (a bug that would short-circuit on any garbled line), this
/// implementation continues past parse failures.
pub fn last_meaningful(lines: &[String]) -> Option<JsonlEnvelope> {
    for line in lines.iter().rev() {
        if let Ok(env) = serde_json::from_str::<JsonlEnvelope>(line) {
            if env.kind == "user" || env.kind == "assistant" {
                return Some(env);
            }
        }
    }
    None
}

/// Decide a session's status from its last meaningful envelope.
///
/// Pure function — no IO, no allocations beyond what serde already did.
/// See [data-model.md § 1.2](../../../../docs/bmad/03-solutioning/data-model.md)
/// and [UML 09 state](../../../../docs/design/uml/09-state-session.md) for
/// the truth table.
///
/// | Input | Output |
/// |---|---|
/// | `None` | `Unknown` |
/// | `assistant` + `stop_reason == "end_turn"` | `Waiting` |
/// | `assistant` + any other stop_reason (incl. None) | `Working` |
/// | `user` (incl. tool_result wrappers) | `Working` |
pub fn classify(env: Option<JsonlEnvelope>) -> SessionStatus {
    let env = match env {
        None => return SessionStatus::Unknown,
        Some(e) => e,
    };
    if env.kind != "assistant" {
        return SessionStatus::Working;
    }
    let stop = env.message.as_ref().and_then(|m| m.stop_reason.as_deref());
    match stop {
        Some("end_turn") => SessionStatus::Waiting,
        _ => SessionStatus::Working,
    }
}

// ============================================================================
// S-005 · IPC orchestration
// ============================================================================

/// How many tail lines we feed to `last_meaningful`. JSONL files commonly end
/// with 1-5 non-meaningful envelopes (attachment / ai-title / etc); 20 gives
/// generous headroom while keeping the read well under 100KB in practice.
const TAIL_LINES_FOR_CLASSIFY: usize = 20;

/// Max preview length pushed to the frontend for `last_message`. The dropdown
/// truncates further visually; this is just to bound the IPC payload.
const PREVIEW_MAX_CHARS: usize = 200;

/// Full per-process pipeline: locate JSONL → tail multi-line → reverse-scan to
/// last meaningful envelope → classify → enrich. Any single-step failure
/// degrades to `Unknown` rather than aborting — the UI shows "Unknown" with no
/// preview, and the next refresh retries.
fn build_session(raw: &RawProcess) -> Session {
    let mut session = Session {
        pid: raw.pid,
        cwd: raw.cwd.to_string_lossy().into_owned(),
        status: SessionStatus::Unknown,
        last_message: None,
        last_update_unix: None,
        waiting_since_unix: None,
    };

    let Ok(path) = locate_jsonl(raw) else {
        return session;
    };
    let Ok(lines) = tail_lines(&path, TAIL_LINES_FOR_CLASSIFY) else {
        return session;
    };
    let env_opt = last_meaningful(&lines);
    if let Some(env) = &env_opt {
        session.last_update_unix = parse_iso8601_utc(&env.timestamp);
        session.last_message = extract_message_preview(env);
    }
    session.status = classify(env_opt);
    // Waiting-since == when the assistant turn ended. For MVP this is the
    // last_update_unix of the assistant envelope we just classified.
    if session.status == SessionStatus::Waiting {
        session.waiting_since_unix = session.last_update_unix;
    }
    session
}

/// Pull a short human-readable preview from a JSONL envelope's message body.
/// Used to populate `Session.last_message` for the dropdown.
///
/// - `user` + `ContentValue::Text` → the text itself
/// - `assistant` + `ContentValue::Blocks` → first `Text` block's content
/// - tool_use / tool_result / Thinking → skipped; returns `None`
///
/// Truncated to `PREVIEW_MAX_CHARS` chars (char-aware so we don't split
/// multi-byte sequences).
pub(crate) fn extract_message_preview(env: &JsonlEnvelope) -> Option<String> {
    let msg = env.message.as_ref()?;
    let raw = match &msg.content {
        ContentValue::Text(s) => s.clone(),
        ContentValue::Blocks(blocks) => blocks.iter().find_map(|b| match b {
            ContentBlock::Text { text } => Some(text.clone()),
            _ => None,
        })?,
    };
    Some(truncate_chars(&raw, PREVIEW_MAX_CHARS))
}

fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max).collect();
        out.push('…');
        out
    }
}

/// Sort priority: Waiting first (smallest), Working next, Unknown last.
pub(crate) fn status_priority(s: SessionStatus) -> u8 {
    match s {
        SessionStatus::Waiting => 0,
        SessionStatus::Working => 1,
        SessionStatus::Unknown => 2,
    }
}

/// In-place sort: by status priority asc, then `waiting_since_unix` asc
/// (`Some` before `None`, oldest wait first). Within the same status (e.g.
/// two Workings) we preserve the input order — MVP doesn't try to rank
/// Working sessions, see [H2 in user-stories.md](../../../../docs/product/user-stories.md).
pub(crate) fn sort_sessions(sessions: &mut [Session]) {
    sessions.sort_by(|a, b| {
        status_priority(a.status)
            .cmp(&status_priority(b.status))
            .then_with(|| match (a.waiting_since_unix, b.waiting_since_unix) {
                (Some(x), Some(y)) => x.cmp(&y),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            })
    });
}

/// IPC handler returning the enriched, sorted Session list.
///
/// Per-session work is wrapped in `catch_unwind` so that a single corrupted
/// JSONL or unexpected panic degrades to an `Unknown` row instead of taking
/// the whole list down. Full structured panic isolation arrives in S-011;
/// this is the lightweight fallback.
pub fn list() -> Vec<Session> {
    let raws = list_processes();
    let mut sessions: Vec<Session> = raws
        .iter()
        .map(|raw| {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| build_session(raw)))
                .unwrap_or_else(|_| Session {
                    pid: raw.pid,
                    cwd: raw.cwd.to_string_lossy().into_owned(),
                    status: SessionStatus::Unknown,
                    last_message: None,
                    last_update_unix: None,
                    waiting_since_unix: None,
                })
        })
        .collect();
    sort_sessions(&mut sessions);
    sessions
}

/// Count sessions whose status is `Waiting`. Drives the tray title.
pub fn waiting_count(sessions: &[Session]) -> usize {
    sessions
        .iter()
        .filter(|s| s.status == SessionStatus::Waiting)
        .count()
}

// ============================================================================
// Unit tests for pure helpers (S-001 T005 / T006).
// Integration tests that exercise sysinfo live in `tests/process_enum.rs`.
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_claude_name_accepts_known_variants() {
        // `claude.exe` is the comm npm-installed Claude Code 2.1.x reports
        // on macOS (verified 2026-06-01). `claude` and `claude-code` covered
        // for older / brew installs / shim wrappers.
        assert!(is_claude_name("claude"));
        assert!(is_claude_name("claude.exe"));
        assert!(is_claude_name("claude-code"));
    }

    #[test]
    fn is_claude_name_is_case_insensitive_via_caller() {
        assert!(is_claude_name("claude"));
        assert!(!is_claude_name("Claude")); // case must be normalised upstream
        assert!(!is_claude_name("CLAUDE.EXE"));
    }

    #[test]
    fn is_claude_name_rejects_unrelated() {
        assert!(!is_claude_name("claudia"));
        assert!(!is_claude_name("openai"));
        assert!(!is_claude_name(""));
        assert!(!is_claude_name("clau"));
        assert!(!is_claude_name("claude.app"));
        assert!(!is_claude_name("node")); // the sysinfo trap we used to fall into
    }

    #[test]
    fn uid_matches_current_same_uid() {
        assert!(uid_matches_current(501, Some(501)));
    }

    #[test]
    fn uid_matches_current_different_uid() {
        assert!(!uid_matches_current(501, Some(0)));
    }

    #[test]
    fn uid_matches_current_fails_open_when_current_unknown() {
        // Treat "we don't know our own uid" as "accept any" — better than
        // erroring out and showing zero sessions.
        assert!(uid_matches_current(501, None));
        assert!(uid_matches_current(0, None));
    }

    // ------------------------------------------------------------------------
    // S-002 · JSONL locator
    // ------------------------------------------------------------------------

    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn encode_cwd_replaces_slashes_with_dashes() {
        // The canonical case verified against Claude Code 2.1.126 on 2026-05-18.
        assert_eq!(encode_cwd("/Users/caiyiwen"), "-Users-caiyiwen");
    }

    #[test]
    fn encode_cwd_handles_nested_path() {
        assert_eq!(
            encode_cwd("/Users/caiyiwen/Desktop/claude-code-monitor"),
            "-Users-caiyiwen-Desktop-claude-code-monitor"
        );
    }

    #[test]
    fn encode_cwd_preserves_existing_hyphens() {
        // claude doesn't escape pre-existing hyphens; round-tripping back is
        // therefore lossy. We only need the encode direction.
        assert_eq!(encode_cwd("/Users/x/my-proj"), "-Users-x-my-proj");
    }

    #[test]
    fn encode_cwd_handles_root() {
        assert_eq!(encode_cwd("/"), "-");
    }

    /// Write a `.jsonl` file with given basename and a tiny payload.
    fn touch_jsonl(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut f = File::create(&path).expect("create jsonl");
        writeln!(f, r#"{{"type":"user"}}"#).expect("write");
        path
    }

    /// Force a file's mtime to `when` for stale-window tests.
    fn set_mtime(path: &Path, when: SystemTime) {
        let f = File::options().write(true).open(path).expect("open");
        f.set_modified(when).expect("set mtime");
    }

    #[test]
    fn locate_returns_path_when_one_jsonl_exists() {
        let dir = TempDir::new().unwrap();
        let expected = touch_jsonl(dir.path(), "session-aaa.jsonl");

        let found = locate_jsonl_in_dir(dir.path(), SystemTime::now()).expect("ok");
        assert_eq!(found, expected);
    }

    #[test]
    fn locate_returns_newest_when_multiple_jsonls() {
        let dir = TempDir::new().unwrap();
        let older = touch_jsonl(dir.path(), "old.jsonl");
        let newer = touch_jsonl(dir.path(), "new.jsonl");

        // Set explicit mtimes so the test doesn't rely on filesystem timestamp
        // granularity (some FSs round to whole seconds).
        let now = SystemTime::now();
        set_mtime(&older, now - Duration::from_secs(10));
        set_mtime(&newer, now - Duration::from_secs(1));

        let found = locate_jsonl_in_dir(dir.path(), now).expect("ok");
        assert_eq!(found, newer);
    }

    #[test]
    fn locate_skips_uuid_subdirectories() {
        let dir = TempDir::new().unwrap();
        // Subdirectory shaped like a uuid — the real claude env mixes these
        // with the actual .jsonl transcripts.
        fs::create_dir(dir.path().join("3f8a-cache")).unwrap();
        let expected = touch_jsonl(dir.path(), "real.jsonl");

        let found = locate_jsonl_in_dir(dir.path(), SystemTime::now()).expect("ok");
        assert_eq!(found, expected);
    }

    #[test]
    fn locate_returns_dir_not_found_when_no_encoded_dir() {
        let dir = TempDir::new().unwrap();
        let missing = dir.path().join("does-not-exist");

        let err = locate_jsonl_in_dir(&missing, SystemTime::now()).unwrap_err();
        assert!(matches!(err, LocateError::DirNotFound), "got {:?}", err);
    }

    #[test]
    fn locate_returns_no_jsonl_files_when_dir_empty() {
        let dir = TempDir::new().unwrap();

        let err = locate_jsonl_in_dir(dir.path(), SystemTime::now()).unwrap_err();
        assert!(matches!(err, LocateError::NoJsonlFiles), "got {:?}", err);
    }

    #[test]
    fn locate_returns_no_jsonl_files_when_only_subdirs_and_nonjsonl() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();
        // Non-jsonl file: should also be ignored, not counted as "seen".
        File::create(dir.path().join("notes.txt")).unwrap();

        let err = locate_jsonl_in_dir(dir.path(), SystemTime::now()).unwrap_err();
        assert!(matches!(err, LocateError::NoJsonlFiles), "got {:?}", err);
    }

    #[test]
    fn locate_returns_no_active_jsonl_when_all_stale() {
        let dir = TempDir::new().unwrap();
        touch_jsonl(dir.path(), "stale.jsonl");

        // Advance "now" past the 60s window — file's actual mtime is ~now, but
        // we pretend it's 2 minutes later.
        let future = SystemTime::now() + Duration::from_secs(120);

        let err = locate_jsonl_in_dir(dir.path(), future).unwrap_err();
        assert!(matches!(err, LocateError::NoActiveJsonl), "got {:?}", err);
    }

    #[test]
    fn locate_accepts_file_within_active_window() {
        let dir = TempDir::new().unwrap();
        let path = touch_jsonl(dir.path(), "fresh.jsonl");
        let now = SystemTime::now();
        set_mtime(&path, now - Duration::from_secs(30)); // 30s < 60s window

        let found = locate_jsonl_in_dir(dir.path(), now).expect("ok");
        assert_eq!(found, path);
    }

    #[test]
    fn locate_handles_future_mtime_permissively() {
        let dir = TempDir::new().unwrap();
        let path = touch_jsonl(dir.path(), "future.jsonl");
        let now = SystemTime::now();
        set_mtime(&path, now + Duration::from_secs(5)); // clock skew

        let found = locate_jsonl_in_dir(dir.path(), now).expect("ok");
        assert_eq!(found, path);
    }

    // ------------------------------------------------------------------------
    // S-003 · JSONL tail reader
    // ------------------------------------------------------------------------

    /// Write `content` to a fresh file inside `dir` and return its path.
    fn write_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut f = File::create(&path).expect("create");
        f.write_all(content).expect("write");
        path
    }

    #[test]
    fn tail_returns_none_on_empty_file() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "empty.jsonl", b"");
        assert_eq!(tail_jsonl(&p).unwrap(), None);
    }

    #[test]
    fn tail_returns_only_line_when_no_trailing_newline() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "no-nl.jsonl", b"line1");
        assert_eq!(tail_jsonl(&p).unwrap().as_deref(), Some("line1"));
    }

    #[test]
    fn tail_returns_only_line_when_trailing_newline() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "one-nl.jsonl", b"line1\n");
        assert_eq!(tail_jsonl(&p).unwrap().as_deref(), Some("line1"));
    }

    #[test]
    fn tail_returns_last_of_three_lines() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "three.jsonl", b"line1\nline2\nline3\n");
        assert_eq!(tail_jsonl(&p).unwrap().as_deref(), Some("line3"));
    }

    #[test]
    fn tail_returns_last_meaningful_when_multiple_trailing_newlines() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "trail.jsonl", b"line1\nline2\n\n\n\n");
        assert_eq!(tail_jsonl(&p).unwrap().as_deref(), Some("line2"));
    }

    #[test]
    fn tail_returns_none_when_file_is_all_newlines() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "nl-only.jsonl", b"\n\n\n");
        assert_eq!(tail_jsonl(&p).unwrap(), None);
    }

    #[test]
    fn tail_handles_line_larger_than_initial_chunk() {
        // Force the doubling path: line size > 4KB initial chunk.
        let dir = TempDir::new().unwrap();
        let big_line = "x".repeat(10_000); // 10KB single line
        let content = format!("first\n{}\n", big_line);
        let p = write_file(dir.path(), "big.jsonl", content.as_bytes());
        assert_eq!(tail_jsonl(&p).unwrap().as_deref(), Some(big_line.as_str()));
    }

    #[test]
    fn tail_handles_utf8_content() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "utf8.jsonl", "前一行\n你好世界 🌍\n".as_bytes());
        assert_eq!(tail_jsonl(&p).unwrap().as_deref(), Some("你好世界 🌍"));
    }

    #[test]
    fn tail_handles_crlf_line_endings() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "crlf.jsonl", b"line1\r\nline2\r\n");
        assert_eq!(tail_jsonl(&p).unwrap().as_deref(), Some("line2"));
    }

    // ------------------------------------------------------------------------
    // S-004 · Status classifier
    // ------------------------------------------------------------------------

    /// Build a minimal valid JSONL envelope as a String, with the given kind
    /// and optional stop_reason. Used to exercise classify branches.
    fn envelope(kind: &str, stop_reason: Option<&str>) -> String {
        let message = if kind == "user" || kind == "assistant" {
            let stop = stop_reason
                .map(|s| format!(r#","stop_reason":"{}""#, s))
                .unwrap_or_default();
            format!(
                r#","message":{{"role":"{}","content":"hi"{}}}"#,
                if kind == "assistant" {
                    "assistant"
                } else {
                    "user"
                },
                stop
            )
        } else {
            String::new()
        };
        format!(
            r#"{{"type":"{}","uuid":"u-{}","timestamp":"2026-05-21T00:00:00.000Z","sessionId":"s-1"{}}}"#,
            kind, kind, message
        )
    }

    #[test]
    fn classify_returns_unknown_when_envelope_is_none() {
        assert_eq!(classify(None), SessionStatus::Unknown);
    }

    #[test]
    fn classify_returns_working_when_kind_is_user() {
        let env: JsonlEnvelope = serde_json::from_str(&envelope("user", None)).unwrap();
        assert_eq!(classify(Some(env)), SessionStatus::Working);
    }

    #[test]
    fn classify_returns_waiting_when_assistant_end_turn() {
        let env: JsonlEnvelope =
            serde_json::from_str(&envelope("assistant", Some("end_turn"))).unwrap();
        assert_eq!(classify(Some(env)), SessionStatus::Waiting);
    }

    #[test]
    fn classify_returns_working_when_assistant_tool_use() {
        let env: JsonlEnvelope =
            serde_json::from_str(&envelope("assistant", Some("tool_use"))).unwrap();
        assert_eq!(classify(Some(env)), SessionStatus::Working);
    }

    #[test]
    fn classify_returns_working_when_assistant_max_tokens() {
        let env: JsonlEnvelope =
            serde_json::from_str(&envelope("assistant", Some("max_tokens"))).unwrap();
        assert_eq!(classify(Some(env)), SessionStatus::Working);
    }

    #[test]
    fn classify_returns_working_when_assistant_no_stop_reason() {
        let env: JsonlEnvelope = serde_json::from_str(&envelope("assistant", None)).unwrap();
        assert_eq!(classify(Some(env)), SessionStatus::Working);
    }

    #[test]
    fn classify_returns_working_when_assistant_unknown_stop_reason() {
        // Forward-compat: any future stop_reason variant defaults to Working.
        let env: JsonlEnvelope =
            serde_json::from_str(&envelope("assistant", Some("refusal"))).unwrap();
        assert_eq!(classify(Some(env)), SessionStatus::Working);
    }

    #[test]
    fn last_meaningful_skips_attachment_and_ai_title_entries() {
        // Lines in chronological order; reverse scan should land on assistant
        // (index 1), skipping attachment (2) and ai-title (3).
        let lines = vec![
            envelope("user", None),
            envelope("assistant", Some("end_turn")),
            envelope("attachment", None),
            envelope("ai-title", None),
        ];
        let env = last_meaningful(&lines).expect("should find assistant");
        assert_eq!(env.kind, "assistant");
        assert_eq!(env.uuid, "u-assistant");
    }

    #[test]
    fn last_meaningful_returns_none_when_no_user_or_assistant() {
        let lines = vec![
            envelope("file-history-snapshot", None),
            envelope("permission-mode", None),
            envelope("attachment", None),
        ];
        // Even though these envelopes are missing many fields, the JSON parses
        // successfully (timestamp/sessionId still synthesised) — last_meaningful
        // simply finds no user/assistant kind.
        assert!(last_meaningful(&lines).is_none());
    }

    #[test]
    fn last_meaningful_returns_none_on_empty_lines() {
        let lines: Vec<String> = vec![];
        assert!(last_meaningful(&lines).is_none());
    }

    #[test]
    fn last_meaningful_skips_unparseable_lines_and_continues_back() {
        // Garbled line between two meaningful ones — reverse scan should hop
        // past the garbage and find the assistant entry.
        let lines = vec![
            envelope("assistant", Some("end_turn")),
            r#"not valid json at all"#.to_string(),
            envelope("attachment", None),
        ];
        let env = last_meaningful(&lines).expect("should skip garbage and find assistant");
        assert_eq!(env.kind, "assistant");
    }

    #[test]
    fn last_meaningful_picks_user_when_after_assistant() {
        // user message right after assistant turn → reverse scan picks user
        // (the more recent one). This corresponds to "user just typed".
        let lines = vec![
            envelope("assistant", Some("end_turn")),
            envelope("user", None),
        ];
        let env = last_meaningful(&lines).expect("should find user");
        assert_eq!(env.kind, "user");
        assert_eq!(classify(Some(env)), SessionStatus::Working);
    }

    #[test]
    fn jsonl_envelope_deserialises_real_assistant_line_shape() {
        // Mirrors the actual shape verified 2026-05-18 against Claude Code 2.1.126:
        // top-level type/uuid/timestamp/sessionId/cwd/version/gitBranch,
        // message nested with role/content array and stop_reason.
        let line = r#"{
            "type":"assistant",
            "uuid":"abc-123",
            "parentUuid":"def-456",
            "timestamp":"2026-05-04T05:42:34.919Z",
            "sessionId":"sess-1",
            "cwd":"/Users/caiyiwen",
            "version":"2.1.126",
            "gitBranch":"main",
            "message":{
                "id":"msg-1",
                "type":"message",
                "role":"assistant",
                "content":[{"type":"text","text":"hello"}],
                "stop_reason":"end_turn",
                "model":"claude-opus-4-7"
            }
        }"#;
        let env: JsonlEnvelope = serde_json::from_str(line).expect("parses");
        assert_eq!(env.kind, "assistant");
        assert_eq!(env.uuid, "abc-123");
        let msg = env.message.as_ref().unwrap();
        assert_eq!(msg.role, "assistant");
        assert_eq!(msg.stop_reason.as_deref(), Some("end_turn"));
        match &msg.content {
            ContentValue::Blocks(blocks) => assert_eq!(blocks.len(), 1),
            ContentValue::Text(_) => panic!("assistant content should be blocks"),
        }
    }

    #[test]
    fn jsonl_envelope_deserialises_real_user_text_content() {
        let line = r#"{
            "type":"user",
            "uuid":"u-1",
            "timestamp":"2026-05-04T05:42:34.000Z",
            "sessionId":"sess-1",
            "message":{"role":"user","content":"please continue"}
        }"#;
        let env: JsonlEnvelope = serde_json::from_str(line).unwrap();
        match env.message.unwrap().content {
            ContentValue::Text(s) => assert_eq!(s, "please continue"),
            ContentValue::Blocks(_) => panic!("user content should be text"),
        }
    }

    // ------------------------------------------------------------------------
    // S-005 · Orchestration helpers
    // ------------------------------------------------------------------------

    #[test]
    fn parse_iso8601_typical_with_millis() {
        // 2026-05-04T05:42:34.919Z → unix epoch seconds. Hand-verified:
        // 20577 days since epoch × 86400 + 5h42m34s = 1_777_873_354.
        assert_eq!(
            parse_iso8601_utc("2026-05-04T05:42:34.919Z"),
            Some(1_777_873_354)
        );
    }

    #[test]
    fn parse_iso8601_without_millis() {
        assert_eq!(
            parse_iso8601_utc("2026-05-04T05:42:34Z"),
            Some(1_777_873_354)
        );
    }

    #[test]
    fn parse_iso8601_without_trailing_z() {
        // Tolerated — claude always emits Z but be permissive.
        assert_eq!(
            parse_iso8601_utc("2026-05-04T05:42:34"),
            Some(1_777_873_354)
        );
    }

    #[test]
    fn parse_iso8601_day_one() {
        // Second-day anchor: 1970-01-02T00:00:00Z = exactly 86400 seconds.
        assert_eq!(parse_iso8601_utc("1970-01-02T00:00:00Z"), Some(86_400));
    }

    #[test]
    fn parse_iso8601_epoch_boundary() {
        // The first second of the unix epoch.
        assert_eq!(parse_iso8601_utc("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(parse_iso8601_utc("1970-01-01T00:00:01Z"), Some(1));
    }

    #[test]
    fn parse_iso8601_rejects_garbage() {
        assert_eq!(parse_iso8601_utc(""), None);
        assert_eq!(parse_iso8601_utc("not a date"), None);
        assert_eq!(parse_iso8601_utc("2026-13-01T00:00:00Z"), None); // bad month
        assert_eq!(parse_iso8601_utc("2026-05-04 05:42:34Z"), None); // space not T
        assert_eq!(parse_iso8601_utc("1969-12-31T23:59:59Z"), None); // before epoch
    }

    // ---- tail_lines ----

    #[test]
    fn tail_lines_returns_empty_on_max_zero() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "x.jsonl", b"a\nb\nc\n");
        assert!(tail_lines(&p, 0).unwrap().is_empty());
    }

    #[test]
    fn tail_lines_returns_empty_on_empty_file() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "empty.jsonl", b"");
        assert!(tail_lines(&p, 5).unwrap().is_empty());
    }

    #[test]
    fn tail_lines_returns_all_when_file_smaller_than_max() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "small.jsonl", b"a\nb\nc\n");
        let out = tail_lines(&p, 10).unwrap();
        assert_eq!(out, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn tail_lines_returns_last_n_when_file_has_more() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "many.jsonl", b"a\nb\nc\nd\ne\n");
        let out = tail_lines(&p, 3).unwrap();
        assert_eq!(out, vec!["c".to_string(), "d".to_string(), "e".to_string()]);
    }

    #[test]
    fn tail_lines_skips_blank_lines_in_middle() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "blank.jsonl", b"a\n\nb\n\n\nc\n");
        let out = tail_lines(&p, 5).unwrap();
        assert_eq!(out, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn tail_lines_handles_no_trailing_newline() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "no-tail-nl.jsonl", b"a\nb\nc");
        let out = tail_lines(&p, 5).unwrap();
        assert_eq!(out, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn tail_lines_handles_line_bigger_than_initial_chunk() {
        let dir = TempDir::new().unwrap();
        let big = "x".repeat(10_000);
        let content = format!("first\n{}\nlast\n", big);
        let p = write_file(dir.path(), "big.jsonl", content.as_bytes());
        let out = tail_lines(&p, 2).unwrap();
        assert_eq!(out, vec![big, "last".to_string()]);
    }

    #[test]
    fn tail_lines_handles_crlf() {
        let dir = TempDir::new().unwrap();
        let p = write_file(dir.path(), "crlf.jsonl", b"a\r\nb\r\nc\r\n");
        let out = tail_lines(&p, 5).unwrap();
        assert_eq!(out, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    // ---- truncate + preview ----

    #[test]
    fn truncate_chars_shorter_than_max_unchanged() {
        assert_eq!(truncate_chars("hello", 10), "hello");
    }

    #[test]
    fn truncate_chars_appends_ellipsis() {
        assert_eq!(truncate_chars("0123456789", 5), "01234…");
    }

    #[test]
    fn truncate_chars_respects_unicode_boundaries() {
        // 5 chars (not bytes); ellipsis added.
        assert_eq!(truncate_chars("你好世界你好世界", 4), "你好世界…");
    }

    #[test]
    fn extract_preview_from_user_text_content() {
        let env: JsonlEnvelope = serde_json::from_str(&envelope("user", None)).unwrap();
        assert_eq!(extract_message_preview(&env).as_deref(), Some("hi"));
    }

    #[test]
    fn extract_preview_from_assistant_blocks_finds_first_text() {
        let line = r#"{
            "type":"assistant","uuid":"u","timestamp":"2026-05-04T05:42:34Z","sessionId":"s",
            "message":{"role":"assistant","content":[
                {"type":"thinking","thinking":"hmm"},
                {"type":"text","text":"final answer"}
            ]}
        }"#;
        let env: JsonlEnvelope = serde_json::from_str(line).unwrap();
        assert_eq!(
            extract_message_preview(&env).as_deref(),
            Some("final answer")
        );
    }

    #[test]
    fn extract_preview_returns_none_when_only_tool_blocks() {
        let line = r#"{
            "type":"assistant","uuid":"u","timestamp":"2026-05-04T05:42:34Z","sessionId":"s",
            "message":{"role":"assistant","content":[
                {"type":"tool_use","id":"t1","name":"Bash","input":{"command":"ls"}}
            ]}
        }"#;
        let env: JsonlEnvelope = serde_json::from_str(line).unwrap();
        assert!(extract_message_preview(&env).is_none());
    }

    // ---- sort + waiting_count ----

    fn fake_session(pid: u32, status: SessionStatus, waiting_since: Option<u64>) -> Session {
        Session {
            pid,
            cwd: format!("/proj/{}", pid),
            status,
            last_message: None,
            last_update_unix: waiting_since,
            waiting_since_unix: waiting_since,
        }
    }

    #[test]
    fn sort_puts_waiting_first_then_working_then_unknown() {
        let mut v = vec![
            fake_session(3, SessionStatus::Unknown, None),
            fake_session(1, SessionStatus::Working, None),
            fake_session(2, SessionStatus::Waiting, Some(100)),
        ];
        sort_sessions(&mut v);
        assert_eq!(v.iter().map(|s| s.pid).collect::<Vec<_>>(), vec![2, 1, 3]);
    }

    #[test]
    fn sort_within_waiting_orders_by_oldest_wait_first() {
        let mut v = vec![
            fake_session(1, SessionStatus::Waiting, Some(200)), // newer
            fake_session(2, SessionStatus::Waiting, Some(100)), // older — should come first
            fake_session(3, SessionStatus::Waiting, Some(150)),
        ];
        sort_sessions(&mut v);
        assert_eq!(v.iter().map(|s| s.pid).collect::<Vec<_>>(), vec![2, 3, 1]);
    }

    #[test]
    fn sort_waiting_with_none_timestamp_sinks_below_some() {
        let mut v = vec![
            fake_session(1, SessionStatus::Waiting, None),
            fake_session(2, SessionStatus::Waiting, Some(100)),
        ];
        sort_sessions(&mut v);
        assert_eq!(v.iter().map(|s| s.pid).collect::<Vec<_>>(), vec![2, 1]);
    }

    #[test]
    fn waiting_count_counts_only_waiting() {
        let v = vec![
            fake_session(1, SessionStatus::Waiting, Some(1)),
            fake_session(2, SessionStatus::Working, None),
            fake_session(3, SessionStatus::Waiting, Some(2)),
            fake_session(4, SessionStatus::Unknown, None),
        ];
        assert_eq!(waiting_count(&v), 2);
    }

    #[test]
    fn status_priority_ordering() {
        assert!(status_priority(SessionStatus::Waiting) < status_priority(SessionStatus::Working));
        assert!(status_priority(SessionStatus::Working) < status_priority(SessionStatus::Unknown));
    }

    #[test]
    fn session_status_serialises_lowercase() {
        // Confirms wire format matches the TS contract.
        assert_eq!(
            serde_json::to_string(&SessionStatus::Waiting).unwrap(),
            r#""waiting""#
        );
        assert_eq!(
            serde_json::to_string(&SessionStatus::Working).unwrap(),
            r#""working""#
        );
        assert_eq!(
            serde_json::to_string(&SessionStatus::Unknown).unwrap(),
            r#""unknown""#
        );
    }

    #[test]
    fn tail_perf_on_1mb_file_under_10ms() {
        // Sanity check the seek-from-end approach scales constantly. A
        // 1MB file with ~10K short lines should still resolve in a few ms.
        let dir = TempDir::new().unwrap();
        let mut buf = Vec::with_capacity(1_100_000);
        for i in 0..10_000 {
            buf.extend_from_slice(format!(r#"{{"i":{}}}"#, i).as_bytes());
            buf.push(b'\n');
        }
        let p = write_file(dir.path(), "1mb.jsonl", &buf);

        let start = std::time::Instant::now();
        let last = tail_jsonl(&p).unwrap();
        let elapsed = start.elapsed();

        assert_eq!(last.as_deref(), Some(r#"{"i":9999}"#));
        assert!(
            elapsed.as_millis() < 50,
            "1MB tail took {:?} (debug build, budget is 10ms release; 50ms cap for debug)",
            elapsed
        );
    }
}
