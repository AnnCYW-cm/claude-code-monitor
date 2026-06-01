# Claude Code Monitor

> A macOS menubar app that tells you which of your running Claude Code sessions is waiting for input.

**Status:** alpha — MVP in progress. Not yet usable end-to-end.

## Why

If you run multiple Claude Code sessions in parallel (3–8 terminal tabs is common for heavy users), you regularly:

- miss a session that finished and is sitting idle waiting for you
- forget what one of them was doing
- can't tell at a glance which one needs attention first

This is a passive indicator that lives in your menubar. No notifications, no sound — just a number you can glance at and a list you can pop open.

## How it works (planned)

- Enumerates running `claude` CLI processes via `sysinfo`.
- For each process, locates its current JSONL transcript in `~/.claude/projects/<encoded-cwd>/`.
- Reads the last message; if `role == "assistant"` and there is no pending `tool_use`, the session is **waiting**. Otherwise it is **working**.
- Tray icon shows the count of waiting sessions. Click to see the list.

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
- [ ] Enumerate running `claude` processes
- [ ] Locate active JSONL per process
- [ ] Parse last message → waiting / working
- [ ] Tray icon shows waiting count
- [ ] Popup list with cwd / status / last message preview
- [ ] Click row to expand full last message

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
