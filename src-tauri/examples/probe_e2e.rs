//! E2E smoke: invokes `session::list()` (the exact backend code that
//! `list_sessions` IPC fronts) against the live host environment and dumps
//! results. Compares process counts against ground truth (`ps`).
//!
//! Run with: `cargo run --example probe_e2e`

use claude_code_monitor_lib::session::{list, list_processes, waiting_count};
use std::process::Command;
use std::time::Instant;

fn main() {
    println!("=== E2E probe: session::list() vs ground truth ===\n");

    // Ground truth via `ps`.
    let my_uid = unsafe { libc::getuid() }.to_string();
    let ps_out = Command::new("ps")
        .args(["-axo", "uid,comm"])
        .output()
        .expect("ps");
    let ps_stdout = String::from_utf8_lossy(&ps_out.stdout);
    let ps_count = ps_stdout
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut iter = line.split_whitespace();
            let uid = iter.next()?;
            let comm_full = iter.collect::<Vec<_>>().join(" ");
            if uid != my_uid {
                return None;
            }
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

    println!("ps ground truth: {} claude processes for uid {}", ps_count, my_uid);

    // Backend layer 1: list_processes (S-001)
    let t0 = Instant::now();
    let raws = list_processes();
    let t_enum = t0.elapsed();
    println!("\n--- S-001 list_processes() -> {} procs ({:?}) ---", raws.len(), t_enum);
    for r in &raws {
        println!("  pid={} cwd={:?}", r.pid, r.cwd);
    }

    // Backend full pipeline (S-001 → S-002 → S-003 → S-004 → S-005)
    let t0 = Instant::now();
    let sessions = list();
    let t_list = t0.elapsed();
    println!("\n--- S-005 session::list() -> {} sessions ({:?}) ---", sessions.len(), t_list);
    for s in &sessions {
        let msg_preview = s
            .last_message
            .as_deref()
            .map(|m| {
                let first_line = m.lines().next().unwrap_or("");
                if first_line.chars().count() > 60 {
                    let truncated: String = first_line.chars().take(60).collect();
                    format!("{}...", truncated)
                } else {
                    first_line.to_string()
                }
            })
            .unwrap_or_else(|| "(none)".to_string());
        println!(
            "  pid={:<6} status={:<8?} waiting_since={:?} cwd={}",
            s.pid, s.status, s.waiting_since_unix, s.cwd
        );
        println!("    last_message: {}", msg_preview);
    }

    let waiting = waiting_count(&sessions);
    println!("\n--- Tray state ---");
    println!("  waiting count = {}", waiting);
    let tray_title = match waiting {
        0 => "(empty, icon only)".to_string(),
        n if n >= 100 => "\"99+\"".to_string(),
        n => format!("\"{}\"", n),
    };
    println!("  tray title    = {}", tray_title);

    // Health checks
    println!("\n=== Health summary ===");
    let p_enum = t_enum.as_millis();
    let p_list = t_list.as_millis();
    println!("  list_processes(): {}ms {}", p_enum, if p_enum < 25 { "✅ NFR-P1 (<25ms)" } else { "⚠️  over 25ms" });
    println!("  list():           {}ms {}", p_list, if p_list < 50 { "✅ NFR-P1 (<50ms)" } else { "⚠️  over 50ms" });

    if ps_count == 0 {
        println!("  ps vs ours:       skip (no live claude on host)");
    } else if sessions.is_empty() {
        println!("  ps vs ours:       ❌ ps sees {} but we return 0 — 2026-06-01 dogfood bug regression!", ps_count);
        std::process::exit(1);
    } else {
        let delta = ps_count as i32 - sessions.len() as i32;
        println!(
            "  ps vs ours:       ps={} list()={} delta={} {}",
            ps_count,
            sessions.len(),
            delta,
            if delta.abs() <= 2 {
                "✅ within race-condition tolerance"
            } else {
                "⚠️  large delta, check filter logic"
            }
        );
    }
}
