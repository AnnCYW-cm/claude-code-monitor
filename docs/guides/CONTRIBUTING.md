# Contributing to Claude Code Monitor

谢谢你考虑贡献！本文档说明怎么参与。

---

## 在动手前

### 必读 (5 分钟)

1. [项目根 README](../../README.md) — 项目是干什么的
2. [docs/README](../README.md) — 文档体系导航
3. [constitution.md](../constitution.md) — 项目治理原则（红线）

### 必懂的红线

constitution 列了几条**永远不会改**的原则。**任何 PR 违反这些会被关闭**，除非先开 ADR 推翻：

- ❌ 不发通知 / 不响声音 / 不抢焦点（[I.1 被动感知红线](../constitution.md)）
- ❌ 不要求用户配置（[I.2 零配置红线](../constitution.md)）
- ❌ 不集成任何终端 emulator API（[I.4 跨终端中立](../constitution.md)）
- ❌ 不显示已退出 session（[I.3 此刻不是历史](../constitution.md)）
- ❌ 不引入：reqwest/hyper/notify/React/Vue/Svelte/osascript（[II.5 禁用依赖](../constitution.md)）

不确定？开 issue 问，别直接动手。

---

## 报 issue

### Bug report

用 [bug report template](../../.github/ISSUE_TEMPLATE/bug_report.md)，必填：

- macOS 版本（`sw_vers`）
- Claude Code 版本（`claude --version`）
- App 版本（tray 右键 menu，未来加 About 项后可以看；目前看 git tag）
- 重现步骤
- 期望 vs 实际
- log 文件相关片段（`~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`）

### Feature request

用 [feature request template](../../.github/ISSUE_TEMPLATE/feature_request.md)。**先 grep [v0.2.md](../roadmap/v0.2.md) 看是不是已经在候选清单**。

---

## 提 PR

### 找一个 story 干

1. 看 [epics/README](../bmad/03-solutioning/epics/README.md) → 找一个 "Pending" 的 dev story
2. 在 PR description 里 link 到该 story，例如：

   ```markdown
   Closes [S-001](../bmad/03-solutioning/epics/story-001-process-enumeration.md)
   ```
   （替换 S-NNN 和文件名为你 implement 的具体 story）
3. story 的 acceptance criteria 是你的"完成"标准

### 代码约定

按 [project-context.md](../bmad/03-solutioning/project-context.md)：

- Rust naming: `PascalCase` types / `snake_case` fns / `SCREAMING_SNAKE_CASE` consts
- 错误处理: 业务 fn 返回 `Result<T, E>`，runtime 禁用 `unwrap()`
- 注释: WHY 注释 > WHAT 注释
- 测试: session.rs ≥ 80% line coverage

### PR description

用 [PR template](../../.github/PULL_REQUEST_TEMPLATE.md)，含：

- Closes [S-NNN]
- What changed
- How tested
- Checklist

### Review 节奏

- 维护者每周末 1 次 office hour（[constitution V.3](../constitution.md)）
- 简单 PR 1 周内回；复杂 PR 2-3 周

---

## 什么 PR 会被接受

✅ 会接受：

- Bug fix（含测试）
- 完成某个 pending dev story 的实现
- 改进文档（typo / 链接 / 例子）
- 性能优化（带 benchmark 证明）
- 实测 macOS 各版本 Gatekeeper 步骤并回填 [install.md](install.md)
- 增加 test coverage

⚠️ 慎重接受（先讨论）：

- 重构（哪怕"看着更优雅"——MVP 阶段简单>聪明）
- 新功能（必须在 [v0.2.md](../roadmap/v0.2.md) 候选里，或者先开 issue 讨论）
- 大依赖升级（Tauri 主版本 bump 之类）

❌ 不接受：

- 违反 constitution 红线（除非先开 ADR）
- 添加 forbidden 依赖
- 增加 settings/preferences UI
- 跨平台代码（Linux/Windows）
- 任何 marketing/analytics/telemetry 代码

---

## 文档贡献

跟代码贡献等同重要。修文档：

1. 改某份 `.md`，注意：
   - 同概念跨文件复述要 sweep（参考 [docs/README § 维护负担提醒](../README.md)）
   - 链接保持 0 broken（CI 会查）
2. 自己用 `grep -rn '<改的内容>' docs/` 找 cross-file refs

---

## 准备 dev 环境

详 [quickstart § 1](../bmad/03-solutioning/quickstart.md)：

```bash
# Prerequisites: macOS 12+, Node 18+, Rust 1.77+
git clone https://github.com/AnnCYW-cm/claude-code-monitor.git
cd claude-code-monitor
npm install
npm run tauri:dev
# 第一次 ~10-15 min cargo download
```

---

## 沟通

- **Issue / PR** GitHub
- **Discord / 邮件** 暂无（v1.0 后看需求）
- **维护节奏** [constitution V.3](../constitution.md)

---

## 行为准则

按 [Contributor Covenant 2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/)。
v0.1 阶段不单独 `CODE_OF_CONDUCT.md`（社区还没形成），但作者承诺：
- 不容忍人身攻击 / 歧视
- Code 讨论对事不对人
- PR review 解释 why 不仅 say no

正式行为准则将在收到第一个外部 contributor PR 时添加。

---

## License

MIT（[LICENSE](../../LICENSE)）。提 PR = 同意你的贡献以 MIT 发布。
