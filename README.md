# Claude Code Monitor

> A macOS menubar app that tells you which of your running Claude Code sessions is waiting for input.

[![CI](https://github.com/AnnCYW-cm/claude-code-monitor/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/AnnCYW-cm/claude-code-monitor/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)](#)

**Status:** alpha — all 13 MVP stories shipped (Epics 1+2+3), 95 tests pass.
End-to-end pipeline works: tray icon shows accurate waiting count, popup lists
sessions with status + last message preview, expand row to see full text.
No signed release `.app` yet — install from source for now.

## Why

If you run multiple Claude Code sessions in parallel (3–8 terminal tabs is common for heavy users), you regularly:

- miss a session that finished and is sitting idle waiting for you
- forget what one of them was doing
- can't tell at a glance which one needs attention first

This is a passive indicator that lives in your menubar. No notifications, no sound — just a number you can glance at and a list you can pop open.

## How it works

- Enumerates running `claude` CLI processes via `sysinfo` (filters by name and current UID).
- For each process, locates its active JSONL transcript at `~/.claude/projects/<encoded-cwd>/<session>.jsonl`. Path encoding rule: `/` → `-`, including the leading slash.
- Tails the file from the end (seek-from-end + chunk doubling — 10MB tail in ~1ms), reverse-scans past attachment / ai-title / metadata envelopes, and reads `message.stop_reason` on the last user/assistant entry:
  - `stop_reason == "end_turn"` → **Waiting** (you should look at this one)
  - any other value, or a `user` envelope → **Working**
  - unreadable / parse failure → **Unknown** (degraded gracefully, never crashes the list)
- The tray icon title shows the count of Waiting sessions. Click to see the full list (UI in progress).

JSONL schema verified against Claude Code 2.1.126 — see [`docs/spec/jsonl-schema.md`](./docs/spec/jsonl-schema.md). If the format changes, the spec and classifier are the only things to touch.

## Stack

- [Tauri 2.x](https://tauri.app) — Rust backend, web frontend, ~10MB bundle
- TypeScript + Vite (no framework — the popup is a single list)
- `libproc` + raw `proc_pidinfo` FFI for process enumeration + cwd (Activity Monitor uses the same APIs; `sysinfo` returns the wrong comm name for Node-based CLIs on macOS — see commit `32134e9`)
- Hand-rolled file logger (no `chrono`/`tokio` deps; one transitive crate `log = "0.4"`)

## v0.1 alpha — what works today

Built end-to-end and verified against a real 3-session macOS environment:

- ✅ Detects all running `claude` CLI processes for the current user
- ✅ Pairs each process to its own JSONL by file birth time (handles the case where multiple sessions share a cwd — common when running `claude` from `~/`)
- ✅ Classifies each session as Waiting / Working / Unknown based on the last `message.stop_reason` in its JSONL
- ✅ Tray title shows the count of Waiting sessions, clamped to `99+` if more than 99
- ✅ Popup window with traffic-light chrome, centered title, draggable
- ✅ Per-row status badge (yellow / green / gray) + duration ("3min" / "just now") + single-line preview
- ✅ Click row to expand the full last message in monospace
- ✅ Auto light/dark mode via system tokens
- ✅ Per-session panic isolation: one corrupt JSONL won't take down the whole list
- ✅ Logs to `~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log` for postmortem
- ✅ Quit cleanly from the tray menu

**Performance** (M-series Mac, debug build):
- `list_processes()`: ~1ms (NFR budget: 25ms)
- Full `session::list()` IPC: ~5-8ms with 3 live sessions (NFR budget: 50ms)
- 10MB JSONL tail read: ~0.8ms

**Known limitations** (deferred to v0.2+):

- No signed/notarized DMG yet — first launch requires the [Gatekeeper bypass](./docs/guides/install.md#gatekeeper-bypass-首次打开未签名-app)
- `last_message` preview is capped at 200 chars by the IPC layer; "expand" shows the same preview, not the full transcript (would need a separate `get_full_message(pid)` IPC)
- macOS only — no Windows / Linux plans
- No proper test fixture for parse-failure path; relies on real-env dogfood
- Window remembers no position between launches

## Develop

Prerequisites: macOS, Node 18+, Rust 1.85+ (Tauri 2.x transitive deps need edition2024).

```bash
npm install
npm run tauri:dev
```

The first `cargo build` will download ~500MB of crates. Subsequent builds are fast.

Build a release `.app`:

```bash
npm run tauri:build
```

Output lands in `src-tauri/target/release/bundle/`.

## Roadmap

### MVP (v0.1) — code complete, awaiting release packaging

**Epic 1 · Core monitoring**
- [x] [S-001](./docs/bmad/03-solutioning/epics/story-001-process-enumeration.md) Enumerate running `claude` processes (libproc + raw FFI)
- [x] [S-002](./docs/bmad/03-solutioning/epics/story-002-jsonl-locator.md) Locate active JSONL per process (birth-time pairing)
- [x] [S-003](./docs/bmad/03-solutioning/epics/story-003-jsonl-tail-reader.md) Multi-MB JSONL tail reader (seek-from-end)
- [x] [S-004](./docs/bmad/03-solutioning/epics/story-004-status-classifier.md) Parse last message → waiting/working/unknown
- [x] [S-005](./docs/bmad/03-solutioning/epics/story-005-list-sessions-command.md) `list_sessions` IPC + tray title sync

**Epic 2 · Menubar UI**
- [x] [S-006](./docs/bmad/03-solutioning/epics/story-006-tray-icon-menu.md) Tray icon + native Quit menu
- [x] [S-007](./docs/bmad/03-solutioning/epics/story-007-popup-window.md) Popup window create + click toggle
- [x] [S-008](./docs/bmad/03-solutioning/epics/story-008-session-list-render.md) Session list render with status badges + duration
- [x] [S-009](./docs/bmad/03-solutioning/epics/story-009-expand-message.md) Expand/collapse last message
- [x] [S-010](./docs/bmad/03-solutioning/epics/story-010-empty-state.md) Empty state with hint

**Epic 3 · Robustness**
- [x] [S-011](./docs/bmad/03-solutioning/epics/story-011-error-handling.md) JSONL parse failure → Unknown + panic isolation
- [x] [S-012](./docs/bmad/03-solutioning/epics/story-012-logging.md) Logging to `~/Library/Logs/...`
- [x] [S-013](./docs/bmad/03-solutioning/epics/story-013-startup-race.md) Startup race: process exists but JSONL not yet

**Release blockers** (not story-tracked):
- [ ] Notarized `.app` + DMG (avoid Gatekeeper warning) — currently $99/yr Apple Developer Program cost; deferring until alpha user feedback justifies
- [ ] 14-day formal dogfood retro using [the full template](./docs/guides/dogfood-retrospective-template.md)
- [ ] `cargo bench` (T020) for tracked perf baselines

### v0.2+ candidates

See [docs/roadmap/v0.2.md](./docs/roadmap/v0.2.md) for the full list. Highlights:

- `get_full_message(pid)` IPC so "expand" shows the full transcript, not the truncated 200-char preview
- Anchor popup to tray icon position instead of center-screen default
- Window position memory across launches
- Auto-hide popup when focus leaves
- Subsecond mtime check for sessions in same cwd (sharper pairing than birth time)

### Explicitly out of scope for MVP

- Jumping to the corresponding terminal tab (too many terminal emulators to support; opening Mission Control yourself is fine)
- Notifications, sound, badges
- Showing exited / historical sessions
- Search, statistics, configuration panel
- Cross-platform, multi-device sync, collaboration

## Documentation

完整文档体系在 [`docs/`](./docs/README.md)——按角色快速入口：

| 角色 | 起点 |
|---|---|
| 想了解产品做什么 (5 min) | [docs/bmad/01-analysis/product-brief.md](./docs/bmad/01-analysis/product-brief.md) |
| 想 implement 某个 story (20 min) | [docs/bmad/03-solutioning/epics/README.md](./docs/bmad/03-solutioning/epics/README.md) |
| 想理解某个设计为何这么定 | [docs/bmad/02-planning/decision-log.md](./docs/bmad/02-planning/decision-log.md)（12 个 ADR） |
| 想推翻产品红线 / 加新功能 | [docs/constitution.md](./docs/constitution.md) |
| 想贡献新文档 | [docs/README.md](./docs/README.md)（顶层索引 + 命名约定） |

**关键文档**：

- **产品定义**：[docs/bmad/01-analysis/product-brief.md](./docs/bmad/01-analysis/product-brief.md)
- **PRD**：[docs/bmad/02-planning/PRD.md](./docs/bmad/02-planning/PRD.md)
- **UX 设计**（含 ASCII mockup）：[docs/design/ui/](./docs/design/ui/README.md)
- **架构**（叙述）：[docs/bmad/03-solutioning/architecture.md](./docs/bmad/03-solutioning/architecture.md)
- **UML 10 图**：[docs/design/uml/](./docs/design/uml/00-index.md)
- **13 个 dev story**：[docs/bmad/03-solutioning/epics/](./docs/bmad/03-solutioning/epics/README.md)
- **install 指南** (WIP)：[docs/guides/install.md](./docs/guides/install.md)

## Contributing

See [docs/guides/CONTRIBUTING.md](./docs/guides/CONTRIBUTING.md) — covers:

- Bug reports (use [issue template](.github/ISSUE_TEMPLATE/bug_report.md))
- Feature requests (check [v0.2 candidates](./docs/roadmap/v0.2.md) first)
- PRs (see [PR template](.github/PULL_REQUEST_TEMPLATE.md))
- Red lines that won't change ([constitution](./docs/constitution.md))

## Security

See [SECURITY.md](./SECURITY.md). TL;DR: completely local, no network, no telemetry.
Report vulnerabilities to `caiyiwenann@gmail.com`.

## License

MIT — see [LICENSE](./LICENSE).
