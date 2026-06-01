# Epics & Dev Stories — Index

> **BMAD Phase 3 · Solutioning · PM output**
> **Status:** Draft → ready for sprint planning
>
> 从 [PRD](../../02-planning/PRD.md) + [Architecture](../architecture.md) + [user-stories](../../../product/user-stories.md) 切分出的 **dev story**。
>
> **跟 product user-stories 的关系**：product user story 是用户视角（"As a CC user, I want to glance the menubar..."），dev story 是开发者视角（"实现 process enumeration 函数，返回 Vec<RawProcess>"）。一个 product story 可能对应多个 dev story，或者多个 product story 共享同一个 dev story。
>
> **总览**：3 个 Epic，13 个 dev story。MVP v0.1 完成全部即可发布。

---

## Epics

| # | Epic | 目标 | Story 数 | 估算工时 |
|---|---|---|---|---|
| 1 | [Core Monitoring Loop](epic-001-core-monitoring.md) | 进程枚举 → JSONL 定位 → 读最后行 → 分类 → IPC 暴露 | 5 | ~2 周 |
| 2 | [Menubar UI](epic-002-menubar-ui.md) | tray icon + popup + 列表 render + 展开 | 5 | ~1 周 |
| 3 | [Robustness & Polish](epic-003-robustness.md) | 错误处理 + log + 启动竞态 + 测试 | 3 | ~1 周 |

总 13 stories，单作者预估 **~4 周完成 MVP**（不含 release 准备）。

---

## Story 完整列表（按推荐实现顺序）

### Epic 1 — Core Monitoring Loop

| ID | Story | 依赖 |
|---|---|---|
| [S-001](story-001-process-enumeration.md) | Process enumeration（sysinfo wrapper） | — |
| [S-002](story-002-jsonl-locator.md) | JSONL locator（cwd → path） | S-001 + spec/jsonl-schema.md |
| [S-003](story-003-jsonl-tail-reader.md) | JSONL tail reader（读最后一行） | S-002 |
| [S-004](story-004-status-classifier.md) | Status classifier（role + pending tool_use） | S-003 + UML 09 |
| [S-005](story-005-list-sessions-command.md) | `list_sessions` IPC command + tray title sync | S-001..004 |

### Epic 2 — Menubar UI

| ID | Story | 依赖 |
|---|---|---|
| [S-006](story-006-tray-icon-menu.md) | Tray icon + native menu (Quit) | S-005 |
| [S-007](story-007-popup-window.md) | Popup window create + show/hide on click | S-006 |
| [S-008](story-008-session-list-render.md) | Session list render (cwd / status / duration / preview) | S-005, S-007 |
| [S-009](story-009-expand-message.md) | Expand/collapse last message + tool_use simple render | S-008 |
| [S-010](story-010-empty-state.md) | Empty state render | S-008 |

### Epic 3 — Robustness & Polish

| ID | Story | 依赖 |
|---|---|---|
| [S-011](story-011-error-handling.md) | JSONL parse failure → Unknown + per-session panic isolation | S-004 |
| [S-012](story-012-logging.md) | Logging to `~/Library/Logs/...` | S-005 |
| [S-013](story-013-startup-race.md) | Startup race: process exists but JSONL not yet | S-002 |

---

## Story 模板

每个 story 文档包含以下字段：

```markdown
# S-NNN · <Short title>

**Epic:** epic-XXX-<name>
**Status:** Pending / In Progress / Done / Blocked
**Estimate:** S/M/L (小=半天 / 中=1-2天 / 大=3-5天)
**Owner:** <author>

## Description (As a / I want / So that)
开发者视角的 story 描述

## Acceptance criteria
- 可测试的具体条件

## Dev notes
- 实现路径提示
- API 选择
- 关键边界情况

## Dependencies
- Upstream: 哪些 story 必须先完成
- Downstream: 哪些 story 等这个完成

## Files to touch
- 代码文件清单

## Test plan
- 单元测试 / 集成测试 / 手动测试

## Definition of Done
- [ ] 代码 merged 到 main
- [ ] 单元测试通过
- [ ] 文档更新（如果有 API 变化）
- [ ] dogfood 验证（自己用了一天没问题）
```

---

## Implementation phases (by user story 视角)

跟 Sprint 视角互补：phase 是按"功能层"分组（什么完成才进入下一层），Sprint 是按"时间周"分组。

### Phase 1 · Discovery + Classification (backend, MVP-blocking)

| Order | What | Story | Files |
|---|---|---|---|
| 1.1 | Process enumeration | [S-001](story-001-process-enumeration.md) | session.rs |
| 1.2 | JSONL locator | [S-002](story-002-jsonl-locator.md) | session.rs |
| 1.3 | JSONL tail reader | [S-003](story-003-jsonl-tail-reader.md) | session.rs |
| 1.4 | Status classifier | [S-004](story-004-status-classifier.md) | session.rs |
| 1.5 | list_sessions IPC + tray title | [S-005](story-005-list-sessions-command.md) | lib.rs + session.rs |

**Exit criteria**: console.log 验证 IPC 返回正确 Session 数组，tray title 正确。

### Phase 2 · Presentation (frontend + tray UI)

| Order | What | Story | Files |
|---|---|---|---|
| 2.1 | Tray icon + Quit menu | [S-006](story-006-tray-icon-menu.md) | lib.rs |
| 2.2 | Popup window toggle | [S-007](story-007-popup-window.md) | lib.rs + tauri.conf.json |
| 2.3 | Session list render | [S-008](story-008-session-list-render.md) | main.ts + style.css |
| 2.4 | Expand/collapse message | [S-009](story-009-expand-message.md) | main.ts + style.css |
| 2.5 | Empty state | [S-010](story-010-empty-state.md) | main.ts + style.css |

**Exit criteria**: 作者可以 dogfood 完整 UX。

### Phase 3 · Robustness

| Order | What | Story | Files |
|---|---|---|---|
| 3.1 | Error handling + panic isolation | [S-011](story-011-error-handling.md) | session.rs |
| 3.2 | Logging | [S-012](story-012-logging.md) | lib.rs + Cargo.toml |
| 3.3 | Startup race | [S-013](story-013-startup-race.md) | session.rs |

**Exit criteria**: 14 天 dogfood 无 crash。

### Phase 4 · Release prep (非代码)

- [`guides/install.md`](../../../guides/install.md) — Gatekeeper bypass 实测（**作者待办**）
- [`spec/jsonl-schema.md`](../../../spec/jsonl-schema.md) — 实测 JSONL 格式
- 项目根 `README.md` — quick start + screenshots
- DMG build + GitHub release

---

## Sprint 建议

### Sprint 1 (Week 1)
- S-001, S-002, S-003 (Core Loop 半部分)
- 阻塞前置：spec/jsonl-schema.md 必须完成

### Sprint 2 (Week 2)
- S-004, S-005 (Core Loop 收尾)
- S-006 (Tray icon)

### Sprint 3 (Week 3)
- S-007, S-008, S-009, S-010 (UI 收尾)

### Sprint 4 (Week 4)
- S-011, S-012, S-013 (Robustness)
- Dogfood 14 天连续验证
- 准备 alpha release

---

## 跟现有代码骨架的关系

当前 `src-tauri/src/lib.rs` + `src-tauri/src/session.rs` 是 scaffold（占位实现）。每个 story 完成后更新对应代码：

| Story | 影响文件 | Product user story |
|---|---|---|
| S-001 | `src-tauri/src/session.rs::list_processes()` | [E2](../../../product/user-stories.md#e2--开机时已有-n-个-session-在跑) |
| S-002 | `src-tauri/src/session.rs::locate_jsonl()` | [E5](../../../product/user-stories.md#e5--同一-cwd-多个-session) |
| S-003 | `src-tauri/src/session.rs::tail_jsonl()` | (基础设施，无单一 product story) |
| S-004 | `src-tauri/src/session.rs::classify()` | [F1](../../../product/user-stories.md#f1--jsonl-损坏读不到), [UML 09](../../../design/uml/09-state-session.md) |
| S-005 | `src-tauri/src/lib.rs` IPC command + `src-tauri/src/session.rs::list()` | [H1](../../../product/user-stories.md#h1--瞄一眼判断是否切走), [E3](../../../product/user-stories.md#e3--仅-1-个-session-在跑), [E6](../../../product/user-stories.md#e6--session-退出瞬间) |
| S-006 | `src-tauri/src/lib.rs` tray setup | [H1](../../../product/user-stories.md#h1--瞄一眼判断是否切走), [H4](../../../product/user-stories.md#h4--退出-app) |
| S-007 | `src-tauri/src/lib.rs` window controller + `tauri.conf.json` | [H2](../../../product/user-stories.md#h2--弹开列表分诊优先级) |
| S-008 | `src/main.ts` + `src/style.css` | [H2](../../../product/user-stories.md#h2--弹开列表分诊优先级), [E4](../../../product/user-stories.md#e4--极多-session10) |
| S-009 | `src/main.ts` | [H3](../../../product/user-stories.md#h3--不切走也能读到关键信息) |
| S-010 | `src/main.ts` | [E1](../../../product/user-stories.md#e1--首次安装空状态) |
| S-011 | `src-tauri/src/session.rs` panic isolation | [F1](../../../product/user-stories.md#f1--jsonl-损坏读不到), NFR-R1 |
| S-012 | `src-tauri/src/lib.rs` log init + `src-tauri/Cargo.toml` (add `log` + `flexi_logger`) | [F1](../../../product/user-stories.md#f1--jsonl-损坏读不到), NFR-M3 |
| S-013 | `src-tauri/src/session.rs` JSONL not found path | [E2 启动竞态](../../../product/user-stories.md#e2--开机时已有-n-个-session-在跑) |

---

## Dev story → Product story 反向映射

| Product story | 实现于 dev story |
|---|---|
| [H1 瞄一眼](../../../product/user-stories.md#h1--瞄一眼判断是否切走) | S-005 (tray title), S-006 (tray icon) |
| [H2 弹开列表](../../../product/user-stories.md#h2--弹开列表分诊优先级) | S-007 (popup), S-008 (list render) |
| [H3 读完整消息](../../../product/user-stories.md#h3--不切走也能读到关键信息) | S-009 (expand) |
| [H4 退出](../../../product/user-stories.md#h4--退出-app) | S-006 (Quit menu) |
| [E1 empty state](../../../product/user-stories.md#e1--首次安装空状态) | S-010 |
| [E2 开机已有 session](../../../product/user-stories.md#e2--开机时已有-n-个-session-在跑) | S-001, S-013 |
| [E3 仅 1 session](../../../product/user-stories.md#e3--仅-1-个-session-在跑) | S-005 (覆盖所有数量) |
| [E4 极多 session](../../../product/user-stories.md#e4--极多-session10) | S-008 (滚动) |
| [E5 同 cwd 多 session](../../../product/user-stories.md#e5--同一-cwd-多个-session) | S-002 (配对策略) |
| [E6 退出瞬间](../../../product/user-stories.md#e6--session-退出瞬间) | S-005 (列表自动更新) |
| [F1 JSONL 损坏](../../../product/user-stories.md#f1--jsonl-损坏读不到) | S-011 |
| [F2 卡死](../../../product/user-stories.md#f2--claude-进程僵死) | (v0.2+) |
| [F3 app 崩溃](../../../product/user-stories.md#f3--app-自己崩了) | 文档 only (install.md) |
| [F4 Gatekeeper](../../../product/user-stories.md#f4--gatekeeper-拦截) | 文档 only (install.md) |
| R1-R5 | 靠 PR review + CI grep |
| L1-L3 | beta 期观察，无具体 story |
| A1-A3 | (v0.2+) |
