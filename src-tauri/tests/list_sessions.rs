//! End-to-end integration for `session::list()` — the S-005 orchestration
//! that chains S-001 → S-004 and returns the sorted Session list the IPC
//! command exposes.

use claude_code_monitor_lib::session::{
    list, list_processes, waiting_count, SessionStatus,
};

#[test]
fn list_does_not_panic_and_matches_process_count_or_skips() {
    let procs = list_processes();
    if procs.is_empty() {
        eprintln!("skip: no live claude processes");
        return;
    }

    let sessions = list();
    assert_eq!(
        sessions.len(),
        procs.len(),
        "list() should return one Session per running process (panic isolation produces Unknown placeholders, not drops)"
    );
}

#[test]
fn list_sorts_waiting_before_working_before_unknown() {
    let sessions = list();
    if sessions.is_empty() {
        eprintln!("skip: no live claude processes");
        return;
    }

    // Walk the list and assert priority is non-decreasing.
    let mut last_priority = 0u8;
    for s in &sessions {
        let p = match s.status {
            SessionStatus::Waiting => 0,
            SessionStatus::Working => 1,
            SessionStatus::Unknown => 2,
        };
        assert!(
            p >= last_priority,
            "list() out of order: saw status {:?} (priority {}) after priority {}\nfull list: {:#?}",
            s.status,
            p,
            last_priority,
            sessions
        );
        last_priority = p;
    }
}

#[test]
fn list_waiting_count_matches_sessions_with_waiting_status() {
    let sessions = list();
    let computed = sessions
        .iter()
        .filter(|s| s.status == SessionStatus::Waiting)
        .count();
    assert_eq!(waiting_count(&sessions), computed);
}

#[test]
fn list_completes_under_50ms_for_realistic_machine() {
    // NFR-P1: single invoke < 50ms for ≤10 sessions. We can't synthesise 10
    // claude processes here, but we can at least confirm the orchestration
    // doesn't blow past the budget on whatever's running on this host.
    let start = std::time::Instant::now();
    let sessions = list();
    let elapsed = start.elapsed();

    eprintln!(
        "list() returned {} sessions in {:?}",
        sessions.len(),
        elapsed
    );
    assert!(
        elapsed.as_millis() < 200,
        "list() took {:?} on debug build (release budget is 50ms; 200ms cap for debug)",
        elapsed
    );
}
