# Claude Code Monitor

> A macOS menubar app that tells you which of your running Claude Code sessions is waiting for input.

[![CI](https://github.com/AnnCYW-cm/claude-code-monitor/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/AnnCYW-cm/claude-code-monitor/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.77%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)](#)

**Status:** alpha — backend pipeline (Epic 1, S-001..S-005) complete and tested
(84 tests pass). Menubar UI in progress; no signed release `.app` yet.

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
- `sysinfo` for process enumeration

## Develop

Prerequisites: macOS, Node 18+, Rust 1.77+.

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

### MVP (v0.1)

- [x] Project scaffold
- [x] Enumerate running `claude` processes ([S-001](./docs/bmad/03-solutioning/epics/story-001-process-enumeration.md))
- [x] Locate active JSONL per process ([S-002](./docs/bmad/03-solutioning/epics/story-002-jsonl-locator.md))
- [x] Multi-MB JSONL tail reader ([S-003](./docs/bmad/03-solutioning/epics/story-003-jsonl-tail-reader.md))
- [x] Parse last message → waiting / working / unknown ([S-004](./docs/bmad/03-solutioning/epics/story-004-status-classifier.md))
- [x] `list_sessions` IPC + tray title sync ([S-005](./docs/bmad/03-solutioning/epics/story-005-list-sessions-command.md))
- [ ] Popup list with cwd / status / last message preview (Epic 2)
- [ ] Click row to expand full last message (Epic 2)
- [ ] Notarized `.app` for download (v0.1 release blocker)

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
