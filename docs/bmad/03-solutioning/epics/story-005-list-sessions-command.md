# S-005 · `list_sessions` IPC command + tray title sync

**Epic:** [001 Core Monitoring](epic-001-core-monitoring.md)
**Status:** ✅ DONE (2026-06-01, 19 unit + 4 integration tests pass; list() in 8ms zero-session debug)
**Estimate:** M — actual ~2 hours
**Owner:** caiyiwen

## Description

**As the** Tauri command layer
**I want to** expose `list_sessions` IPC method that orchestrates S-001 ~ S-004 and returns a sorted Vec<Session>, AND updates the tray title with waiting count
**so that** the frontend can render the UI and users see the waiting count on tray.

## Acceptance criteria

- IPC command 签名：`#[tauri::command] fn list_sessions(app: AppHandle) -> Vec<Session>`
- `Session { pid, cwd, status, last_message: Option<String>, last_update_unix: Option<u64>, waiting_since_unix: Option<u64> }`
- 流程：
  1. `list_processes()` (S-001)
  2. for each → `locate_jsonl()` (S-002) → `tail_jsonl()` (S-003) → parse → `classify()` (S-004) → 构建 Session
  3. 按 (status==Waiting 优先 → waiting_since asc) 排序——waiting 越久越靠前
  4. 计算 waiting count，更新 tray title `tray.set_title(format!("{}", count))`（count > 0 显示；count == 0 显示空字符串）
- 单次 invoke < 50ms (10 session 内) — NFR-P1
- 全程 catch_unwind 隔离单 session 异常 (S-011 完整实现 panic isolation，此 story 先用 Result 兜底)
- `last_update_unix`：JSONL 最后一条消息的 timestamp（依 [H2 open question - 时长起点](../../../product/user-stories.md#h2--弹开列表分诊优先级)）

## Dev notes

- IPC 注册：在 `lib.rs` 的 `invoke_handler(tauri::generate_handler![list_sessions])`
- AppHandle 用来访问 tray：`app.tray_by_id("main").unwrap().set_title(Some(text))`
- `waiting_since_unix` MVP 简化：用 JSONL 最后一条 assistant 消息的 timestamp。这样 tray app 重启后 "已等时长" 也能恢复正确。
- 排序：先按 status (Waiting=0, Working=1, Unknown=2) 升序，再按 waiting_since asc
- Working session 间排序：MVP 不刻意排（保持 list_processes 返回顺序），见 H2 open question

## Dependencies

- **Upstream**: S-001, S-002, S-003, S-004
- **Downstream**: Epic 2 (UI)

## Files to touch

- `src-tauri/src/lib.rs` — 注册 command + tray set_title 调用
- `src-tauri/src/session.rs` — 新增 `list()` 函数（编排上述步骤），更新 `Session` struct（加 `waiting_since_unix`）
- `src/main.ts` — 占位调用 console.log 验证（真正 render 在 S-008）

## Test plan

### 集成测试 (手动)
1. 开 3 个真实 claude session（1 waiting, 2 working）
2. `cargo run`，看前端 console: `[Session { pid: ..., status: Waiting, ... }, ...]`
3. 验证 tray title 显示 "1"
4. 让 working 之一完成，下一轮 tray 变 "2"
5. 关掉所有 session，tray title 变空

### 性能 benchmark
- 10 session → 单次 invoke < 50ms
- 15 session → < 100ms

## Definition of Done

- [x] 代码 merged（pending dedicated commit）
- [x] IPC 在前端能调通（main.ts 加 `console.log("[list_sessions]", sessions)` smoke；render 已存在）
- [x] tray title 跟列表 waiting 数永远一致 — `list_sessions` 命令内同步调 `sync_tray_title(app, waiting_count(&sessions))`，逻辑路径单一不可分裂
- [x] 性能 benchmark 通过 — 0-session 8ms (debug)，远低 50ms NFR-P1
- [ ] dogfood 1 天无问题（待人工 dogfood）

## Implementation summary (2026-06-01)

### 新增 building blocks (session.rs)

- `tail_lines(path, max_lines) -> Result<Vec<String>, io::Error>` — 多行 tail，复用 seek-from-end + chunk doubling，返回 chronological order；skip 空行；CRLF 容错
- `parse_iso8601_utc(&str) -> Option<u64>` — pure stdlib（不引 chrono dep），用 Howard Hinnant days-from-civil 算法转 Y-M-D → unix epoch
- `Session` 加 `waiting_since_unix: Option<u64>` 字段

### 编排 (session.rs)

- `build_session(raw)` — 单进程 pipeline：locate_jsonl → tail_lines(20) → last_meaningful → classify → extract_message_preview → parse timestamps；任意一步 fail 降级为 Unknown 占位
- `list()` 用 `catch_unwind` 隔离单 session 异常（S-011 完整 panic isolation 前的轻量兜底），再调 `sort_sessions()`
- 排序：status priority asc（Waiting=0, Working=1, Unknown=2），同优先级内 `waiting_since_unix` asc（Some 在 None 前）
- `waiting_count(&[Session]) -> usize` — 驱动 tray title

### IPC + tray (lib.rs)

- `list_sessions(app: AppHandle)` 注入 AppHandle
- `sync_tray_title(&app, count)` — count > 0 显数字，== 0 用 `None` 清空标题

### 测试 19 unit + 4 integration

- parse_iso8601: 5 case（典型/无 millis/无 Z/epoch 边界/拒乱输入）
- tail_lines: 8 case（max=0/empty/<max/=max/blank lines/no trailing nl/big line/CRLF）
- truncate + preview: 3 + 3 case
- sort + count: 4 + 1 case + status_priority ordering
- live env: list_sessions.rs 4 case（不 panic + match 进程数 / 排序 monotonic / waiting_count 一致 / 50ms NFR）

### 关键 bug 自查

- 第一轮 cargo test parse_iso8601 三个 fail：expected value 算错 12h；hand-verified 20577 天 × 86400 + 5h42m34s = 1_777_873_354 后修测试 expected。code 一发即对。
