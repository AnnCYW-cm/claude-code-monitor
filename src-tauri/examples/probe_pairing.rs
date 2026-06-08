//! Verify the pairing hypothesis: each claude process's started_at should
//! match the birth time of exactly one jsonl in its cwd's encoded dir.

use claude_code_monitor_lib::session::list_processes;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn encode_cwd(cwd: &str) -> String {
    cwd.replace('/', "-")
}

fn fmt_time(t: SystemTime) -> String {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{} (unix)", secs)
}

fn main() {
    let procs = list_processes();
    println!("=== {} live claude processes ===\n", procs.len());

    // Group by cwd to highlight same-cwd collisions.
    let mut by_cwd: std::collections::HashMap<String, Vec<&_>> = Default::default();
    for p in &procs {
        let cwd_key = p.cwd.to_string_lossy().into_owned();
        by_cwd.entry(cwd_key).or_default().push(p);
    }

    for (cwd, group) in &by_cwd {
        println!("--- cwd = {} ({} processes) ---", cwd, group.len());
        for p in group {
            println!("  pid={:<6} started_at={}", p.pid, fmt_time(p.started_at));
        }

        let encoded = encode_cwd(cwd);
        let home = dirs::home_dir().unwrap();
        let dir = home.join(".claude").join("projects").join(&encoded);
        if !dir.exists() {
            println!("  ⚠️  jsonl dir does not exist: {:?}", dir);
            continue;
        }

        // Collect jsonl files with their birth + mtime.
        let mut jsonls: Vec<(String, SystemTime, SystemTime)> = Vec::new();
        for entry in fs::read_dir(&dir).unwrap() {
            let entry = entry.unwrap();
            if !entry.file_type().unwrap().is_file() {
                continue;
            }
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
                continue;
            }
            let meta = entry.metadata().unwrap();
            let birth = meta.created().unwrap_or(UNIX_EPOCH);
            let mtime = meta.modified().unwrap_or(UNIX_EPOCH);
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            jsonls.push((name, birth, mtime));
        }

        // Show jsonls that might be relevant (mtime within last 30 days).
        let now = SystemTime::now();
        let cutoff = std::time::Duration::from_secs(30 * 24 * 60 * 60);
        jsonls.retain(|(_, _, m)| now.duration_since(*m).map(|d| d < cutoff).unwrap_or(true));
        jsonls.sort_by_key(|(_, b, _)| *b);

        println!("  jsonls in dir (recent {}):", jsonls.len());
        for (name, birth, mtime) in &jsonls {
            println!(
                "    {} birth={} mtime={}",
                name,
                fmt_time(*birth),
                fmt_time(*mtime)
            );
        }

        // For each process, find the jsonl with closest birth time.
        println!("\n  Pairing by closest birth-to-started_at:");
        for p in group {
            let proc_t = p
                .started_at
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            let best = jsonls
                .iter()
                .map(|(name, birth, _)| {
                    let bt = birth
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    (name, bt, (bt - proc_t).abs())
                })
                .min_by_key(|(_, _, dist)| *dist);
            if let Some((name, _, dist)) = best {
                println!("    pid={} → {} (delta={}s)", p.pid, name, dist);
            }
        }
        println!();
    }
}
