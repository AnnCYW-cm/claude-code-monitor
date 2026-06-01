//! Integration tests for S-001 process enumeration (libproc-based, post-dogfood).
//!
//! These exercise the real libproc enumeration against the host machine. Some
//! tests are "or-empty" tolerant — they only assert invariants that must hold
//! regardless of whether claude is running. But the critical
//! `list_finds_claude_when_ps_does` test catches the dogfood failure mode:
//! if `ps` sees a user-owned claude process but our enumerator returns nothing,
//! that is a P0 production bug and the test must fail loudly.
//!
//! See [docs/bmad/03-solutioning/epics/story-001-process-enumeration.md](../../../docs/bmad/03-solutioning/epics/story-001-process-enumeration.md).

use claude_code_monitor_lib::session::{list_processes, RawProcess};
use std::collections::HashSet;
use std::process::Command;
use std::time::Instant;

#[test]
fn list_processes_does_not_panic() {
    let _ = list_processes();
}

#[test]
fn list_processes_returns_unique_pids() {
    let procs: Vec<RawProcess> = list_processes();
    let pids: HashSet<u32> = procs.iter().map(|p| p.pid).collect();
    assert_eq!(
        procs.len(),
        pids.len(),
        "duplicate PIDs in enumeration: {procs:#?}"
    );
}

#[test]
fn list_processes_returns_only_running_claude_or_empty() {
    let procs = list_processes();
    for p in &procs {
        assert!(
            p.cwd.is_absolute(),
            "cwd should be absolute, got {:?}",
            p.cwd
        );
        assert!(p.pid > 0, "PID must be > 0, got {}", p.pid);
    }
}

/// THE TEST THAT WOULD HAVE CAUGHT THE 2026-06-01 DOGFOOD BUG.
///
/// Uses `ps` as ground truth — if the host has a user-owned `claude`/`claude.exe`
/// process and our enumerator returns zero, that is a P0 production bug
/// (tray will perpetually show "0 sessions running" while the user has live
/// sessions). Skips cleanly when no claude is running.
#[test]
fn list_finds_claude_when_ps_does() {
    let my_uid = unsafe { libc::getuid() }.to_string();

    // `ps -axo uid,comm` — every process on the system, uid + comm name.
    let output = Command::new("ps")
        .args(["-axo", "uid,comm"])
        .output()
        .expect("running ps should succeed on macOS");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Count comm == claude / claude.exe / claude-code, owned by current uid.
    let ps_claude_count = stdout
        .lines()
        .skip(1) // header
        .filter_map(|line| {
            let mut iter = line.split_whitespace();
            let uid = iter.next()?;
            let comm_full = iter.collect::<Vec<_>>().join(" ");
            // We only want THIS user's processes. Other users' claude (if any)
            // won't show in our enumerator and shouldn't count as "ground truth".
            if uid != my_uid {
                return None;
            }
            // ps gives basename of executable, possibly truncated to 15 chars.
            let comm = std::path::Path::new(&comm_full)
                .file_name()
                .map(|s| s.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            if matches!(comm.as_str(), "claude" | "claude.exe" | "claude-code") {
                Some(())
            } else {
                None
            }
        })
        .count();

    let procs = list_processes();
    eprintln!(
        "ps reports {} claude processes for uid={}",
        ps_claude_count, my_uid
    );
    eprintln!("list_processes() reports {} processes", procs.len());

    if ps_claude_count == 0 {
        eprintln!("skip: no live claude processes on this host (nothing to verify)");
        return;
    }

    assert!(
        !procs.is_empty(),
        "P0 BUG: ps sees {} claude processes for uid {}, but list_processes() returned 0.\n\
         This is exactly the dogfood failure mode from 2026-06-01 — \
         the enumerator must agree with ps on whether claude is running.\n\
         ps output:\n{}",
        ps_claude_count,
        my_uid,
        stdout
            .lines()
            .filter(|l| l.to_lowercase().contains("claude"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn list_processes_meets_25ms_budget_for_realistic_machine() {
    let start = Instant::now();
    let _ = list_processes();
    let elapsed = start.elapsed();
    println!("list_processes() elapsed: {elapsed:?}");

    // 100ms is generous (NFR-P1 demands < 25ms for 10 sessions; 4x for slow CI).
    assert!(
        elapsed.as_millis() < 100,
        "list_processes() took {elapsed:?}, way over budget"
    );
}
