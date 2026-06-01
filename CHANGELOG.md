# Changelog

All notable changes to this project will be documented in this file.

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Added

- 文档体系完整建立（11K+ 行 / 60+ 文件，详 [docs/](docs/README.md)）
- Tauri 2.x scaffold (`src-tauri/` + `src/`)
- BMAD 方法论产物（产品 brief / PRD / architecture / 13 dev stories / etc）
- UI 设计文档集（含 ASCII mockup / CSS skeleton）
- 项目治理 constitution（红线 + 性能 budget + 禁用依赖）
- 项目根 [README](README.md) 含 docs/ 入口

### Documented (待 implement)

- 30 个 TDD-ordered tasks（[docs/bmad/03-solutioning/tasks.md](docs/bmad/03-solutioning/tasks.md)）
- 完整 [JSONL schema](docs/spec/jsonl-schema.md)（实测 against Claude Code 2.1.126）
- [Install guide](docs/guides/install.md)（macOS 26 推断版，跨版本待实测）

### Blocked (跨版本实测)

- macOS 12 / 14 / 15 Gatekeeper bypass 步骤 — 需 beta 用户帮测

---

## [0.1.0] - TBD (target Q3 2026)

First public release。Theme: "Passive awareness for parallel Claude Code sessions"。

### Added

(Will list after MVP implementation completes. See [roadmap/v0.1.md § 6 Definition of v0.1 done](docs/roadmap/v0.1.md).)

### Known limitations

- 仅 macOS（无 Linux/Windows）
- 不签名/不公证（首次打开走 Gatekeeper bypass，详 [install.md](docs/guides/install.md)）
- macOS 12-25 Gatekeeper 路径未完整实测
- 同 cwd 多 session 配对启发式（极少 case 可能 mismatch）

---

## How to update this file

When releasing a new version:

1. Move `[Unreleased]` content to `[X.Y.Z] - YYYY-MM-DD`
2. Add fresh empty `[Unreleased]` section
3. Update version reference in [docs/roadmap/v0.X.md](docs/roadmap/)
4. Tag the commit (`git tag v0.1.0`)
5. Push tag (`git push --tags`)
6. Create GitHub Release

### Categories（来自 Keep a Changelog）

- `Added` — 新功能
- `Changed` — 现有功能改动
- `Deprecated` — 即将删除的功能
- `Removed` — 已删除的功能
- `Fixed` — bug fix
- `Security` — 安全相关
