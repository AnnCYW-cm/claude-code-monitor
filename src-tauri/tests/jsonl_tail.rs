//! Integration tests for S-003 (tail_jsonl) — both synthetic large-file
//! perf checks and an end-to-end smoke that chains S-001 → S-002 → S-003
//! against the live `~/.claude/projects/` environment.

use claude_code_monitor_lib::session::{
    classify, last_meaningful, list_processes, locate_jsonl, tail_jsonl, SessionStatus,
};

#[test]
fn end_to_end_live_claude_session_tails_to_valid_json_or_skips() {
    let procs = list_processes();
    if procs.is_empty() {
        eprintln!("skip: no live claude processes on this host");
        return;
    }

    let mut any_ok = false;
    for raw in &procs {
        let Ok(path) = locate_jsonl(raw) else {
            continue;
        };
        let Ok(Some(line)) = tail_jsonl(&path) else {
            continue;
        };

        // The last meaningful line in a claude JSONL must be valid JSON
        // (per docs/spec/jsonl-schema.md). If it isn't, our locator picked
        // the wrong file or the tail reader corrupted bytes.
        let parsed: serde_json::Result<serde_json::Value> = serde_json::from_str(&line);
        assert!(
            parsed.is_ok(),
            "tail line is not valid JSON: {:?}\nfrom: {:?}",
            line,
            path
        );

        // Top-level type field should be present per the schema (one of
        // user/assistant/file-history-snapshot/permission-mode/attachment/
        // ai-title/last-prompt). We don't enforce the exact value — just
        // that the envelope structure exists.
        let value = parsed.unwrap();
        assert!(
            value.get("type").is_some(),
            "tail line missing top-level `type` field: {}\nfrom: {:?}",
            line,
            path
        );

        any_ok = true;
        break;
    }

    assert!(
        any_ok,
        "no live claude session yielded a tail-able jsonl — at least one running session should resolve end-to-end"
    );
}

#[test]
fn end_to_end_classify_returns_valid_status_for_live_session_or_skips() {
    let procs = list_processes();
    if procs.is_empty() {
        eprintln!("skip: no live claude processes on this host");
        return;
    }

    // Single-line tail will only correctly classify when the final line is
    // a user/assistant envelope (not attachment / ai-title / etc). If our
    // current process is the test runner itself, the tail line is likely
    // assistant-mid-tool-use (this very tool call). Either way, the status
    // must be one of the three variants — never panic.
    let mut any_classified = false;
    for raw in &procs {
        let Ok(path) = locate_jsonl(raw) else {
            continue;
        };
        let Ok(Some(line)) = tail_jsonl(&path) else {
            continue;
        };

        let status = classify(last_meaningful(std::slice::from_ref(&line)));
        // Every variant is valid; this test asserts classify is total.
        let _: SessionStatus = status;
        eprintln!(
            "pid={} cwd={:?} tail_first_60={:?} status={:?}",
            raw.pid,
            raw.cwd,
            &line[..line.len().min(60)],
            status
        );
        any_classified = true;
    }

    assert!(
        any_classified,
        "no live process produced a classifiable tail line"
    );
}

#[test]
fn tail_handles_10mb_file_under_50ms_debug() {
    use std::io::Write;
    use tempfile::TempDir;

    let dir = TempDir::new().unwrap();
    let path = dir.path().join("10mb.jsonl");
    let mut f = std::fs::File::create(&path).unwrap();
    // ~10MB of one-line JSONL records.
    let mut buf = String::with_capacity(10_500_000);
    for i in 0..100_000 {
        buf.push_str(&format!(r#"{{"i":{},"pad":"{}"}}"#, i, "x".repeat(80)));
        buf.push('\n');
    }
    f.write_all(buf.as_bytes()).unwrap();
    drop(f);

    let start = std::time::Instant::now();
    let last = tail_jsonl(&path).unwrap();
    let elapsed = start.elapsed();

    let last = last.expect("last line should exist");
    assert!(last.contains(r#""i":99999"#), "got: {}", last);
    assert!(
        elapsed.as_millis() < 50,
        "10MB tail took {:?} on debug build; release-build budget is 10ms",
        elapsed
    );
    eprintln!("10MB tail elapsed (debug): {:?}", elapsed);
}
