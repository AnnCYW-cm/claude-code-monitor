# Decision Log — Claude Code Monitor

> **BMAD Phase 2 · Planning · PM output (continuous)**
> **Status:** Living document — 每个重大决策追加一条
>
> 每条决策用 ADR-lite 格式：context / decision / status / consequences / alternatives considered。
> 编号 `ADR-NNN`。决策一旦写入 status=Accepted，反悔需要新 ADR 覆盖（status=Superseded by ADR-NNN）。

---

## ADR-001 · 选 Tauri 2.x 作为 app 框架

**Date**: 2026-05-16
**Status**: Accepted
**Deciders**: 作者

### Context

需要做 macOS 菜单栏 app。主要技术栈候选：Swift+SwiftUI、Tauri (Rust+Web)、Electron (Node+Web)。

### Decision

选 **Tauri 2.x** (Rust 后端 + TypeScript/Vite 前端，无前端框架)。

### Consequences

正面：
- 打包后 ~10MB，比 Electron 小一个数量级
- VS Code 写起来体验好（rust-analyzer 成熟）
- 跨终端 emulator 中立的 Rust 进程枚举 (sysinfo crate) 现成
- 开源用户分发友好（一个 .app 拖进去）

负面：
- 学习成本：Rust 比 JS/Swift 高
- Tauri 2.x 还在演进，API 偶有 breaking change
- 跨语言 debug（Rust ↔ JS via IPC）比单语言复杂

### Alternatives

| 方案 | Pros | Cons | 拒绝原因 |
|---|---|---|---|
| Swift + SwiftUI | 最原生、最小 (~3MB) | VS Code 体验差，要 Xcode | 跟用户「VS Code 工程」需求冲突 |
| Electron | 生态最大，npm 包多 | 100MB+ bundle，200MB+ RAM | 开源用户嫌大 |
| Tauri 2.x | 平衡 | 上面提的 | **胜出** |

---

## ADR-002 · 监控状态用 polling 不用 fs watcher

**Date**: 2026-05-17
**Status**: Accepted (MVP)
**Deciders**: 作者

### Context

监控 JSONL 文件变化有两种范式：
- (a) 前端 setInterval 2s 轮询调后端 `list_sessions`
- (b) 后端用 fs watcher (notify crate) 订阅 `~/.claude/projects/` 变化，event-driven 推到前端

### Decision

**MVP 用 (a) polling**。v0.2+ 评估是否切 (b)。

### Consequences

正面：
- 极简：setInterval + invoke + render，调试容易
- 跟 webview 可见性自然绑定（未来想隐藏时降频也好改）
- 跨进程 / 跨平台行为一致

负面：
- 浪费空 tick（用户没动作时也在轮询）
- 状态延迟最多 2s（用户难感知）
- 不是事件驱动，理论上"不够 reactive"

### Alternatives

| 方案 | Pros | Cons | 拒绝原因 |
|---|---|---|---|
| fs watcher (notify) | 事件驱动，零浪费 | macOS 上对 JSONL append 的事件投递有 quirk（合并/丢失） | 复杂度 vs 收益不值，MVP 拒 |
| 后端长定时器 + emit_event push | 跨进程一致性强 | 调试复杂 | MVP 不需要 |

---

## ADR-003 · 不实现「跳转到对应终端 tab」

**Date**: 2026-05-17 (列入产品定义 v0.2)
**Status**: Accepted
**Deciders**: 作者

### Context

直觉上，看到某 session waiting 后用户想"一键切过去"。技术上需要 app 跟终端 emulator 集成。

### Decision

**不做**。详见 [user-stories R4](../../product/user-stories.md#r4--不该接管切到对应-tab)。

### Consequences

正面：
- 工程量减半（不需要适配 iTerm/Terminal/Ghostty/Warp/Alacritty 各自 API）
- 跨终端 emulator 中立（NFR-C3）
- 用户已经会用 Cmd+Tab / Mission Control

负面：
- 价值砍掉一截（用户多两步操作）
- 后续会持续有用户提 issue 要求加（要在 PRD § 14 明示）

### 补偿措施

- H3 补：点列表项展开完整最后一条消息，**不切走也能判断要不要切**
- popup 标记清楚 cwd 末段名，用户切 tab 时定位快

### Alternatives

| 方案 | 拒绝原因 |
|---|---|
| 集成 iTerm Python API | 只覆盖一个 emulator |
| 用 AppleScript 找窗口标题 | 依赖用户终端配置 |
| 让用户自己设置 tab 命名约定 | 违反 R2 零配置 |

---

## ADR-004 · 永不通知（产品红线）

**Date**: 2026-05-17 (列入产品定义 v0.2)
**Status**: Accepted, **Red line**
**Deciders**: 作者

### Context

直觉上，"等你的 session" 应该弹通知。但所有作者用过的通知工具最后都被关掉。

### Decision

**永不实现通知 / sound / badge / 抢焦点**。详见 [R1](../../product/user-stories.md#r1--不该主动打断用户)。

未来任何加通知的 PR：**先 ADR 覆盖本条**。

### Consequences

正面：
- 产品定位清晰：「被动感知 > 主动查询」
- 跟 Slack/邮件区分开
- 不被关掉（关掉了价值=0）

负面：
- 部分用户会要求加（"我希望声音提醒"）
- 跟其他菜单栏工具的"默认行为"不一致

### Alternatives

| 方案 | 拒绝原因 |
|---|---|
| 可配置通知（默认关闭） | 违反 R2 零配置；配置面板入口本身就是诱惑 |
| 仅 critical 通知（如 30 分钟以上 waiting） | 阈值争议无穷；"critical" 主观；MVP 不做 |

---

## ADR-005 · 数据源选 JSONL 不 hook Claude Code lifecycle

**Date**: 2026-05-17
**Status**: Accepted
**Deciders**: 作者

### Context

需要知道每个 claude session 的状态。两种方式：
- (a) 解析 Claude Code 写的 JSONL transcript
- (b) Hook 进 Claude Code 的 lifecycle event（如果有 SDK / plugin API）

### Decision

**(a) JSONL**。

### Consequences

正面：
- JSONL 已经存在、稳定写入、不需要 Claude Code 暴露任何 API
- 完全旁路，对 Claude Code 进程 0 影响
- Claude Code 自己崩了 JSONL 仍在，monitor 仍能恢复

负面：
- 受 Claude Code JSONL 格式稳定性约束（格式变我们坏）
- 有 ~2s polling 延迟（hook 是 0 延迟）
- 不能识别 Claude Code 进程内部更细状态（如 thinking vs tool_call）

### Mitigations

- 锁定我们依赖的字段到 `spec/jsonl-schema.md`（待写）
- CI 测试自动校验 JSONL 格式假设
- 监听 Claude Code 升级公告

### Alternatives

| 方案 | 拒绝原因 |
|---|---|
| Hook Claude Code SDK | 目前未知是否有 hook API，假设没有 |
| 屏幕 OCR | 重、不稳、隐私敏感 |
| 拦截 stdout/stderr | 要拦截已运行进程，技术门槛高，跨终端 emulator 不可靠 |

---

## ADR-006 · 前端不引入框架（vanilla TS）

**Date**: 2026-05-16
**Status**: Accepted (MVP)
**Deciders**: 作者

### Context

Tauri 前端可以用任何 web 框架。MVP UI 很简单：一个 header + 一个列表 + 一个 empty state。

### Decision

**vanilla TypeScript + Vite，无框架**。

### Consequences

正面：
- bundle 更小（无 React/Vue/Svelte 包袱）
- 维护负担低（无升级 React 18→19 之类的事）
- contributor 入门门槛低（无前端框架前置知识）

负面：
- 列表 render 是手写 DOM，比 React 啰嗦
- 未来 UI 复杂化（如果违反 MVP 范围）会回头加框架

### Reconsider trigger

如果 UI 复杂度增长到：
- 状态管理逻辑超过 200 行
- 出现 ≥3 个互动组件
→ 评估引入 Preact / Svelte（更轻量）。React 不考虑（太重）。

---

## ADR-007 · `list_sessions` IPC 设计为同步阻塞

**Date**: 2026-05-17
**Status**: Accepted (MVP)
**Deciders**: 作者

### Context

前端调用 `invoke("list_sessions")` 时，整个过程（进程枚举 + JSONL 读 + 分类）是阻塞还是 async streaming？

### Decision

**同步阻塞**。前端 `await invoke(...)`，后端按顺序完成所有 session 处理后返回 `Vec<Session>`。

### Consequences

正面：
- 简单：一次 invoke 一次响应
- 前端 render 是 atomic（要么全旧要么全新，无中间态）
- 调试容易

负面：
- 单次 invoke 时长 = max(所有 session 处理时间之和)，10 session 内 < 50ms 可接受
- 极端情况（100+ session）会卡——但那超出 MVP 范围

### Reconsider trigger

如果 user feedback 显示 invoke 超过 100ms 影响体验：
- 切 async streaming（emit_event per session）
- 或后端缓存 + 增量更新

---

## ADR-008 · Tray icon 左键 toggle popup，右键弹 menu

**Date**: 2026-05-17
**Status**: Accepted
**Deciders**: 作者

### Context

macOS 菜单栏 app 的两种交互范式：
- (a) 单击弹 menu，菜单里有 Show window 项
- (b) 左键弹 popup window，右键弹 menu

### Decision

**(b)**。左键 → popup；右键 → native menu（含 Quit）。

### Consequences

正面：
- popup 首屏即可见信息（不需要先点 menu 再点 "Show"）
- 符合 Bartender、1Password、Magnet 等菜单栏 utility 惯例
- Quit 不污染主 popup（在右键 menu 里）

负面：
- macOS 默认 tray click 是弹 menu，要 explicit 设置 `menu_on_left_click(false)`
- 部分用户可能习惯左键弹 menu（早期 macOS 范式）

---

## ADR-009 · 开源 MIT license

**Date**: 2026-05-16
**Status**: Accepted
**Deciders**: 作者

### Context

开源项目的标准 license：MIT / Apache 2.0 / GPL / AGPL。

### Decision

**MIT**。

### Consequences

正面：
- 最宽松，最大化采用率
- 跟 Tauri 自身 license (Apache + MIT) 兼容
- 短，不啰嗦
- 商业使用无门槛（虽然 MVP 不商业化）

负面：
- 不能强制下游开源（GPL 系才能）
- 如果未来商业化要重写 license（MIT 不可撤销但可以双 license）

### Alternatives

| 方案 | 拒绝原因 |
|---|---|
| Apache 2.0 | 更长 + 专利条款，对个人项目过度 |
| GPL/AGPL | 限制下游使用 |
| 自定义 license | 不要 |

---

## ADR-010 · 文档目录约定：顶层不带编号，内部用 NN- 前缀

**Date**: 2026-05-17
**Status**: Accepted
**Deciders**: 作者

### Context

`docs/` 下需要多个子目录（product / design / spec / roadmap / research / guides / bmad / spec-kit）。是否给顶层目录编号决定阅读顺序？

### Decision

**顶层不编号、内部用 NN- 前缀**。详见 [docs/README.md § 命名规则](../../README.md)。

### Consequences

正面：
- 跨文档相对链接稳定（不会因为重排目录破链）
- 顶层按类别命名，语义清晰
- 内部按数字排序，IDE/GitHub 显示顺序 = 推荐阅读顺序

负面：
- 加新顶层目录时无强制顺序（需要在 README 索引手动列）

### Trigger to reconsider

如果顶层目录数超过 12 个（认知负担过大），评估二级分类（如 `docs/agile-methods/bmad/` + `docs/agile-methods/spec-kit/`）。

---

## ADR-011 · BMAD 和 Spec Kit 产物放在 docs/ 下而不是工具默认位置

**Date**: 2026-05-18
**Status**: **Superseded by [ADR-013](#adr-013--撤销-bmad-spec-kit-双轨制保留-bmad)**（spec-kit/ 整个目录已撤销，本 ADR 的"BMAD 和 Spec Kit 都放 docs/" 不再适用）
**Deciders**: 作者

### Context

- BMAD 默认产物位置：`_bmad-output/planning-artifacts/`
- Spec Kit 默认产物位置：`.specify/specs/`

我们已有 `docs/` 顶层约定（ADR-010）。

### Decision

**违反两个工具的默认位置，统一放 `docs/bmad/` 和 `docs/spec-kit/`**。

### Consequences

正面：
- 一致的 docs/ 入口（用户从一个地方进文档世界）
- README 索引可控
- Git history 集中

负面：
- 如果未来用 BMAD/Spec Kit CLI 工具直接生成新文档，需要手动 move 到 docs/
- 跟工具默认行为不一致，contributor 第一次可能困惑

### Mitigation

- docs/bmad/README.md 和 docs/spec-kit/README.md 解释这个偏离 + 怎么从工具默认位置 move 过来（待写）

---

## ADR-012 · 已等时长 UI 格式：`waiting · 3min`（中圆点）

**Date**: 2026-05-18
**Status**: Accepted
**Deciders**: 作者

### Context

review 中发现 user-stories 和 scenarios 用了两种格式：`waiting · 3min` 和 `waiting (3min)`。需要统一。

### Decision

**中圆点 `waiting · 3min`**。

### Consequences

正面：
- 更紧凑，符合 macOS 风格（参考 Spotlight）
- 跟 `cwd-name · waiting · 3min · "msg..."` 字段分隔符一致

负面：
- 在不支持中圆点的字体下可能显示成 box
- 略微 less common 比起 parentheses

---

## ADR-013 · 撤销 BMAD + Spec Kit 双轨制，保留 BMAD

**Date**: 2026-05-18
**Status**: Accepted
**Deciders**: 作者
**Supersedes**: [ADR-011](#adr-011--bmad-和-spec-kit-产物放在-docs-下而不是工具默认位置)

### Context

[ADR-011](#adr-011--bmad-和-spec-kit-产物放在-docs-下而不是工具默认位置) 决定让 BMAD 和 Spec Kit 双轨并存（在 `docs/bmad/` 和 `docs/spec-kit/`）。
"整体 review"（2026-05-18）发现：

- BMAD + Spec Kit 共产生 ~6900 行 / 28 文件，但 70% 内容重叠（PRD vs spec.md / architecture vs plan.md / decision-log vs constitution / etc）
- Contributor 不知道看哪个 — 同一个产品需求两份描述
- 同概念跨文档复述维护负担高（2s polling 在 12 文件 / Tauri 2.x 在 10 文件）
- 文档 11.6K 行 vs 代码 269 行 scaffold = 43:1，跟产品哲学（简单、被动、零配置）反差大

### Decision

**撤销 spec-kit 双轨制，保留 BMAD**。具体执行：

- **保留并 mv 独立价值文件**（spec-kit 有但 BMAD 没等价物的）：
  - `spec-kit/memory/constitution.md` → `docs/constitution.md`（顶层）
  - `spec-kit/specs/001-mvp/tasks.md` → `bmad/03-solutioning/tasks.md`
  - `spec-kit/specs/001-mvp/data-model.md` → `bmad/03-solutioning/data-model.md`
  - `spec-kit/specs/001-mvp/contracts/ipc-contract.md` → `spec/ipc-contract.md`
  - `spec-kit/specs/001-mvp/quickstart.md` → `bmad/03-solutioning/quickstart.md`
  - `spec-kit/specs/001-mvp/research.md` → `bmad/03-solutioning/research-notes.md`

- **merge 独立内容**（删除前 carry-over）：
  - `spec-kit/specs/001-mvp/spec.md § 7 acceptance checklist (41 项)` → `quickstart.md § 4 Quick checklist`
  - `spec-kit/specs/001-mvp/plan.md § 4 implementation by phase` → `epics/README.md § Implementation phases`

- **删除**:
  - `spec-kit/README.md`
  - `spec-kit/specs/001-mvp/spec.md`（独立内容 carry 后）
  - `spec-kit/specs/001-mvp/plan.md`（独立内容 carry 后）
  - `spec-kit/` 整个目录

### Consequences

**正面**:
- 64 → 60 文件（减 4），~11,600 → ~9,500 行（减约 2,100 行真重复）
- 顶层目录 7 → 6（去 spec-kit/）
- Contributor 不再"该看哪个" 困惑
- 维护负担降低（同概念跨文档少一份冗余）
- 文档 vs 代码比例从 43:1 改善（但仍重资产）

**负面**:
- 跨文档 link sweep 30+ 处
- 部分 file header 仍带 "Spec Kit ·" attribution，保留作历史 trace
- 失去对照视角（"两套方法论都跑一遍"的实验价值被牺牲掉）

### Alternatives 考虑过

| 方案 | 拒绝原因 |
|---|---|
| 保持双轨制（ADR-011 现状） | 维护负担 + contributor 困惑 |
| 砍 BMAD 保 Spec Kit | BMAD 我们写得更完整（含 UX design、epics、stories），失去太多 |
| 改造为 single 方法论混合体（重命名 BMAD → "Custom"） | 改名增加 contributor 入门成本，无实质价值 |

### Reconsider trigger

- 如果未来 Spec Kit 出官方 CLI 工具 + AI agent 能自动同步两套文档 → 重新评估
- 如果项目长到需要更严格的 "intent → implementation" trace（spec-kit 强项），考虑重新引入

---

## Template for new ADR

```
## ADR-NNN · <短标题>

**Date**: YYYY-MM-DD
**Status**: Proposed / Accepted / Superseded by ADR-XXX
**Deciders**: <人>

### Context
<问题背景>

### Decision
<决定了什么>

### Consequences
**正面**:
- ...
**负面**:
- ...

### Alternatives
<其他考虑过的方案 + 拒绝原因>

### Reconsider trigger（可选）
<什么情况下回头改这条决策>
```
