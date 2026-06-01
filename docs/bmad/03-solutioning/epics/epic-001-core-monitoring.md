# Epic 001 · Core Monitoring Loop

**Status:** Pending
**Owner:** caiyiwen
**Sprint target:** Sprint 1-2 (Week 1-2)
**Estimate:** ~2 weeks

## Goal

让后端能够：枚举 claude 进程 → 找到每个进程对应的 JSONL → 读最后一条消息 → 分类成 waiting/working/unknown → 通过 IPC 暴露给前端。

完成此 epic 后，**没有 UI** 也能用 `tauri dev` 启动 + 在前端 console 打印结果验证后端工作正常。

## Success criteria

- 用户启动 3 个 claude session，调 `list_sessions` 返回准确的 3 个 Session 对象
- 状态分类正确（waiting / working / unknown 都能产生）
- 单次 invoke < 50ms (10 session 内) — NFR-P1
- tray icon 标题正确显示 waiting count
- 全程不修改 claude 进程，不影响它们运行

## Stories

| ID | Story | Estimate | Status |
|---|---|---|---|
| [S-001](story-001-process-enumeration.md) | Process enumeration（sysinfo wrapper） | M | ✅ DONE 2026-05-18 |
| [S-002](story-002-jsonl-locator.md) | JSONL locator | M | ✅ DONE 2026-05-18 |
| [S-003](story-003-jsonl-tail-reader.md) | JSONL tail reader | M | ✅ DONE 2026-05-21 |
| [S-004](story-004-status-classifier.md) | Status classifier | S | ✅ DONE 2026-05-21 |
| [S-005](story-005-list-sessions-command.md) | `list_sessions` IPC command + tray title sync | M | ✅ DONE 2026-06-01 |

## Prerequisites

- ✅ Tauri scaffold (已完成)
- ❗ [spec/jsonl-schema.md](../../../spec/jsonl-schema.md)（必须先写）
- ✅ [architecture.md § 5 budget](../architecture.md)（性能预算）
- ✅ [UML 09 State](../../../design/uml/09-state-session.md)（分类规则）

## Out of scope (留到 epic 2/3)

- 任何 UI 改动（epic 2）
- 错误处理 / log（epic 3）
- 启动竞态精细处理（epic 3 - S-013）

## Risks

| Risk | Mitigation |
|---|---|
| JSONL 格式跟假设不符 | spec/jsonl-schema.md 实测先行 |
| sysinfo 在 macOS 15 性能问题 | S-001 实测，超 budget 找替代 |
| 同 cwd 多 session 配对错误 | S-002 用 mtime + 写入活跃度启发式 |

## Definition of Done (Epic level)

- [ ] 全部 5 stories 完成
- [ ] `cargo test` 全通过
- [ ] 手动测试：3 个真实 claude session 下 `list_sessions` 返回正确
- [ ] 性能 benchmark 写在 `src-tauri/tests/perf.rs`，CI 跑
