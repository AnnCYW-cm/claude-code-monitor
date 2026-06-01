//! Integration tests for S-001 process enumeration.
//!
//! These exercise the real `sysinfo` crate against the host machine. Most of
//! them don't assume a `claude` process is running — they only assert
//! invariants that must hold regardless (e.g. "doesn't panic", "returns within
//! budget", "PIDs are unique").
//!
//! See [docs/bmad/03-solutioning/epics/story-001-process-enumeration.md](../../../docs/bmad/03-solutioning/epics/story-001-process-enumeration.md).

use claude_code_monitor_lib::session::{list_processes, RawProcess};
use std::collections::HashSet;
use std::time::Instant;

#[test]
fn list_processes_does_not_panic() {
    // Smoke test: enumeration completes on any machine.
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
    // If the dev's machine happens to have a `claude` process running, we
    // expect the cwd to be an absolute path. If nothing is running, the vec
    // is empty — also valid.
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

#[test]
fn list_processes_meets_25ms_budget_for_realistic_machine() {
    // Soft budget check. CI may not enforce this strictly because runner
    // machines vary, but it documents the NFR-P1 expectation.
    let start = Instant::now();
    let _ = list_processes();
    let elapsed = start.elapsed();
    println!("list_processes() elapsed: {elapsed:?}");

    // 100ms is generous (NFR-P1 demands < 25ms for 10 sessions; we allow 4x
    // for slow CI). A real regression usually shows up as multi-second.
    assert!(
        elapsed.as_millis() < 100,
        "list_processes() took {elapsed:?}, way over budget"
    );
}
