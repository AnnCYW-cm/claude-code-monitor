# Epic 003 · Robustness & Polish

**Status:** Pending
**Owner:** caiyiwen
**Sprint target:** Sprint 4 (Week 4)
**Estimate:** ~1 week

## Goal

让 app 在异常情况下不崩、可恢复、可 debug。完成此 epic 后 MVP 可以 release。

## Success criteria

- JSONL 损坏时单个 session 显示 Unknown，不影响其他 session
- 单个 session 处理 panic 不会让整个 app 崩
- 所有错误都有 log（路径 `~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`）
- 启动竞态（process 在但 JSONL 还没建好）处理正确
- 作者 14 天 dogfood 无 P0/P1 issue

## Stories

| ID | Story | Estimate | Status |
|---|---|---|---|
| [S-011](story-011-error-handling.md) | JSONL parse failure → Unknown + per-session panic isolation | M | ✅ DONE 2026-06-08 |
| [S-012](story-012-logging.md) | Logging to `~/Library/Logs/...` | S | ✅ DONE 2026-06-08 |
| [S-013](story-013-startup-race.md) | Startup race: process exists but JSONL not yet | S | ✅ DONE 2026-06-08 |

## Prerequisites

- ✅ Epic 1, 2 完成
- ✅ [F1, E2 user stories](../../../product/user-stories.md)
- ✅ [architecture.md § 6 failure modes](../architecture.md)

## Out of scope (留到 v0.2+)

- F2 进程卡死检测
- F3 launchd 守护进程
- A1 屏幕共享 demo mode
- A2 Focus mode 适配
- A3 sleep/wake 主动 refresh

## Release readiness checklist

完成此 epic + 下面这些非 story 工作后，MVP v0.1 可 release：

- [ ] [guides/install.md](../../../guides/install.md) 写完（含 Gatekeeper 实测步骤 - **F4 作者待办**）
- [ ] 主 [README.md](../../../../README.md) 更新（v0.1 release notes）
- [ ] [docs/spec/jsonl-schema.md](../../../spec/jsonl-schema.md) 完成
- [ ] 14 天 dogfood 报告
- [ ] release-002 branch + tag v0.1.0
- [ ] GitHub release 发布 + .dmg upload

## Risks

| Risk | Mitigation |
|---|---|
| dogfood 期发现重大 bug | Sprint 4 留 buffer time |
| log file 权限 / 路径 macOS 12 跟 15 不一致 | 实测 |

## Definition of Done (Epic level)

- [ ] 全部 3 stories 完成
- [ ] 故意制造 disk full → app 不崩，相关 session 显示 Unknown，恢复后自动 retry
- [ ] 故意构造畸形 JSONL → 单 session 显示 Unknown，其他 session 正常
- [ ] log 文件存在且可读
- [ ] 14 天连续运行无 crash
