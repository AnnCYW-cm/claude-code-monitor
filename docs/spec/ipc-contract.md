# IPC Contract — 001-mvp

> **IPC contract: frontend ↔ backend interface specification** — originally from Spec Kit `/speckit.plan` supporting, mv to spec/ by [ADR-013](../bmad/02-planning/decision-log.md)
> **Status:** Draft (pending implementation validation)
>
> 定义 Tauri 前后端 IPC 的精确接口契约。前端调用 `invoke(...)`，后端用 `#[tauri::command]` 处理。
> 这份文档是 implementer 跨语言对齐的 single source of truth。

---

## 1. Commands overview

MVP 暴露 1 个 IPC command。

| Command | Direction | Sync/Async | Purpose |
|---|---|---|---|
| `list_sessions` | Frontend → Backend | sync (blocking) | 获取当前所有 claude session |

---

## 2. `list_sessions`

### 2.1 Signature

#### Backend (Rust)

```rust
use tauri::AppHandle;

#[tauri::command]
fn list_sessions(app: AppHandle) -> Vec<Session> {
    // ...
}
```

Registered in `lib.rs::run()`:

```rust
.invoke_handler(tauri::generate_handler![list_sessions])
```

#### Frontend (TypeScript)

```ts
import { invoke } from "@tauri-apps/api/core";

const sessions: Session[] = await invoke<Session[]>("list_sessions");
```

### 2.2 Request

**Parameters**: none

**Headers**: 由 Tauri runtime 添加，frontend 无需关心

### 2.3 Response

**Type**: `Vec<Session>` / `Session[]`

**Session schema**: 详见 [data-model.md § 1.1](../bmad/03-solutioning/data-model.md)。

### 2.4 Response sample

#### Empty (no sessions)

```json
[]
```

#### Multiple sessions (sorted)

```json
[
  {
    "pid": 12345,
    "cwd": "/Users/caiyiwen/work/api-server-tests",
    "status": "waiting",
    "last_message": "All 142 tests passed. Want me to commit?",
    "last_update_unix": 1763472510,
    "waiting_since_unix": 1763472510
  },
  {
    "pid": 12346,
    "cwd": "/Users/caiyiwen/personal/blog",
    "status": "waiting",
    "last_message": "Reformatted 8 headings. Review?",
    "last_update_unix": 1763472340,
    "waiting_since_unix": 1763472340
  },
  {
    "pid": 12347,
    "cwd": "/Users/caiyiwen/work/api-server",
    "status": "working",
    "last_message": "Running integration suite...",
    "last_update_unix": 1763472555,
    "waiting_since_unix": null
  },
  {
    "pid": 12348,
    "cwd": "/Users/caiyiwen/work/old-project",
    "status": "unknown",
    "last_message": null,
    "last_update_unix": null,
    "waiting_since_unix": null
  }
]
```

### 2.5 Ordering guarantees

返回数组按以下顺序排序：

1. **Primary**: status (`Waiting` < `Working` < `Unknown`)
2. **Secondary** (waiting 内): `waiting_since_unix` ASC（等得越久越靠前）
3. **Secondary** (working / unknown 内): MVP 不刻意排序 (`list_processes` 返回顺序)
   - v0.2+ 可改为 cwd 字母序或 process 启动时间

→ 客户端不应假定 working/unknown 内的稳定顺序。

### 2.6 Performance guarantees

| Constraint | Limit | 见 |
|---|---|---|
| Single invoke duration (10 session) | < 50ms | NFR-P1 |
| Single invoke duration (15 session) | < 100ms | NFR-P1 |
| Invoke under panic (single session) | 该 session = Unknown, others 正常 | NFR-R1 |

### 2.7 Errors

`list_sessions` 不直接返回 error。所有错误转化为：
- 单个 session status = `Unknown`
- 写入 log 文件

理由：前端不需要错误处理 UI；user-facing error 就是"该 session 显示 Unknown"。

### 2.8 Side effects

`list_sessions` 调用末尾执行：

```rust
let waiting_count = sessions.iter().filter(|s| s.status == SessionStatus::Waiting).count();
let title = if waiting_count > 0 { format!("{}", waiting_count) } else { String::new() };
app.tray_by_id("main").map(|t| t.set_title(Some(title)));
```

→ 保证 tray title 跟前端 render 的列表 waiting 计数永远一致（[H1 acceptance](../product/user-stories.md#h1--瞄一眼判断是否切走) NFR-FR-9）。

### 2.9 Idempotency

- Pure read operation, idempotent
- 多次连续调用返回值可能不同（reflects current state）

### 2.10 Frequency

- Frontend 每 2s 调用一次 (`setInterval(refresh, 2000)`)
- 用户手动点 refresh 按钮也触发一次

---

## 3. Future commands (v0.2+)

预留 namespace，MVP 不实现：

| Command | Purpose | 适用 |
|---|---|---|
| `get_session_full(pid)` | 获取某 session 完整 history | 历史回看（违反 R5，不会做） |
| `dismiss_session(pid)` | 暂时隐藏某 session | "snooze" 功能（违反 R3，不会做） |
| `set_polling_interval(secs)` | 调整轮询频率 | 用户配置（违反 R2，不会做） |
| `kill_session(pid)` | 杀某 claude 进程 | 接管控制（违反 brief，不会做） |

→ 这些 command **永久不实现**，标在这里防止未来重复评估。

---

## 4. Events (Backend → Frontend push)

MVP **无 events**。所有交互都是 frontend pull。

### 4.1 Considered (rejected for MVP)

| Event | Purpose | Rejection reason |
|---|---|---|
| `sessions_changed` | 后端检测变化 push 给前端 | 违反 [ADR-002](../bmad/02-planning/decision-log.md#adr-002--监控状态用-polling-不用-fs-watcher) (用 polling)，前端 polling 已经覆盖 |
| `session_waiting` | 通知 waiting 状态进入 | 违反 [R1](../product/user-stories.md#r1--不该主动打断用户) 红线 |
| `app_will_quit` | 退出前 push | 不需要（无 cleanup） |

---

## 5. Tray events (Tauri framework events)

不属于 IPC command 但相关：

| Event | Handled in | Action |
|---|---|---|
| `TrayIconEvent::Click(Left, Up)` | `lib.rs::on_tray_icon_event` | toggle popup window |
| `TrayIconEvent::Click(Right, ...)` | (default) | macOS 弹 native menu |
| `MenuEvent("quit")` | `lib.rs::on_menu_event` | `app.exit(0)` |

详见 [UML 08 Sequence: Tray Click](../design/uml/08-sequence-tray-click.md)。

---

## 6. Serialization rules

| Rule | Reason |
|---|---|
| `snake_case` field names | 跨语言一致 (无 case 转换 friction) |
| Enums serialized as lowercase strings (`"waiting"` not `"Waiting"`) | TS 友好 (string literals) |
| `Option<T>` → `T \| null` | TS 一致 |
| `u64` timestamps as JSON number | 足够（< 2^53） |
| Strings UTF-8 | std |

---

## 7. Versioning

### MVP

- 无 schema version 字段
- 假设 frontend 和 backend 同 binary 部署
- breaking change = 重写 frontend + 重 build

### Future

如果暴露 IPC 给第三方插件（v1.0+ unlikely）：
- Add `schema_version: u32` to Session
- Only additive changes (no remove)
- Breaking → new command name (e.g. `list_sessions_v2`)

---

## 8. Validation

### 8.1 Test coverage

| Test | Where |
|---|---|
| `list_sessions` returns Vec<Session> | `src-tauri/tests/ipc.rs` |
| Empty case returns `[]` | Same |
| Status enum serializes lowercase | `src-tauri/tests/session_serde.rs` |
| Sort order: waiting first, then duration desc | Same |
| Tray title side effect updates correctly | Manual test |
| Performance: 10 session < 50ms | `src-tauri/benches/perf.rs` (criterion) |

### 8.2 Contract test (for frontend)

```ts
// src/__tests__/contract.test.ts (if we add Jest later)
const sessions = await invoke<Session[]>("list_sessions");
expect(Array.isArray(sessions)).toBe(true);
sessions.forEach(s => {
  expect(typeof s.pid).toBe("number");
  expect(typeof s.cwd).toBe("string");
  expect(["waiting", "working", "unknown"]).toContain(s.status);
  // ...
});
```

MVP 阶段前端无 test framework，靠 manual + console.log 验证。

---

## 9. Cross-reference

| Topic | Where |
|---|---|
| Type definitions | [data-model.md](../bmad/03-solutioning/data-model.md) |
| FR coverage | [spec.md § 5](../bmad/02-planning/PRD.md) |
| Implementation plan | [plan.md § 4](../bmad/03-solutioning/architecture.md) |
| Backend story | [S-005](../bmad/03-solutioning/epics/story-005-list-sessions-command.md) |
| Frontend story | [S-008](../bmad/03-solutioning/epics/story-008-session-list-render.md) |
| Sequence diagram | [UML 07](../design/uml/07-sequence-refresh.md) |
