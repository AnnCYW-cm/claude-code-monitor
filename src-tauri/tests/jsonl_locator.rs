//! Integration tests for S-002 (JSONL locator) against the real
//! `~/.claude/projects/` environment.
//!
//! These tests are conditional on the host actually having a running claude
//! session (the test author's machine usually does, since the suite is run
//! from within Claude Code). When no live session exists we skip rather than
//! fail — CI can decide separately whether to require coverage here.

use claude_code_monitor_lib::session::{list_processes, locate_jsonl, LocateError};

#[test]
fn locate_finds_jsonl_for_at_least_one_live_claude_or_skips() {
    let procs = list_processes();
    if procs.is_empty() {
        eprintln!("skip: no live claude processes on this host");
        return;
    }

    // Walk through each found process — we need at least ONE to resolve to a
    // jsonl. The others may legitimately fail with NoActiveJsonl (a session
    // sitting idle for >60s) or DirNotFound (a brand-new session that hasn't
    // written its first message yet), so don't insist every process succeeds.
    let mut any_ok = false;
    for raw in &procs {
        match locate_jsonl(raw) {
            Ok(path) => {
                assert!(
                    path.exists(),
                    "locate_jsonl returned a non-existent path: {:?}",
                    path
                );
                assert_eq!(
                    path.extension().and_then(|s| s.to_str()),
                    Some("jsonl"),
                    "returned path is not a .jsonl: {:?}",
                    path
                );
                let path_str = path.to_string_lossy();
                assert!(
                    path_str.contains("/.claude/projects/"),
                    "path not under ~/.claude/projects/: {:?}",
                    path
                );
                any_ok = true;
                break;
            }
            Err(e) => {
                eprintln!("process pid={} cwd={:?}: {}", raw.pid, raw.cwd, e);
            }
        }
    }

    assert!(
        any_ok,
        "no live claude process resolved to an active jsonl — check that the test was run with at least one fresh claude session"
    );
}

#[test]
fn locate_dir_not_found_for_synthetic_unused_cwd() {
    use std::path::PathBuf;
    use std::time::SystemTime;

    // Synthesise a RawProcess with a cwd that almost certainly has no
    // corresponding ~/.claude/projects/<encoded>/ directory.
    let proc = claude_code_monitor_lib::session::RawProcess {
        pid: 999_999,
        cwd: PathBuf::from("/nonexistent/path/used/by/no/claude/session"),
        started_at: SystemTime::now(),
    };

    let err = locate_jsonl(&proc).expect_err("expected DirNotFound");
    assert!(matches!(err, LocateError::DirNotFound), "got {:?}", err);
}
