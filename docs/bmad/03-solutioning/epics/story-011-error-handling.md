# S-011 · JSONL parse failure → Unknown + per-session panic isolation

**Epic:** [003 Robustness](epic-003-robustness.md)
**Status:** Pending
**Estimate:** M (1-2 days)
**Owner:** caiyiwen

## Description

**As** a developer
**I want** error handling so that a single bad JSONL or panic during session processing doesn't bring down the whole app or hide other sessions
**so that** the app stays reliable in production.

## Acceptance criteria

- JSONL parse 失败 → 该 session `status = Unknown`，`last_message = Some("(unable to read transcript)")`
- 不让 session 从列表消失（[F1 acceptance](../../../product/user-stories.md#f1--jsonl-损坏读不到)）
- 下一轮 refresh 自动重试，无需用户干预
- 单个 session 处理过程中 panic → catch 住，记 log，该 session status = Unknown，**不影响其他 session 处理**
- `list_sessions` 整体不 panic
- 错误都 log 到文件（依 S-012）

## Dev notes

- session.rs::list() 里的 per-session loop:
  ```rust
  let mut sessions = Vec::new();
  for raw_proc in list_processes() {
      let session = match std::panic::catch_unwind(|| {
          process_one_session(&raw_proc)
      }) {
          Ok(s) => s,
          Err(_panic) => {
              log::error!("panic processing pid={}", raw_proc.pid);
              Session::unknown(raw_proc.pid, raw_proc.cwd.clone())
          }
      };
      sessions.push(session);
  }
  ```
- `process_one_session` 内部用 Result，正常 Err 不 panic（panic 只是兜底）
- `Session::unknown(pid, cwd)` 是 constructor，返回 status=Unknown 的 Session
- catch_unwind 要求闭包内不能跨越 unwind boundary 持有非 UnwindSafe 类型——如果有问题，用 AssertUnwindSafe wrapper
- JSONL parse 失败的具体 case：
  - serde_json::Error (字段缺失 / 类型不对)
  - tail_jsonl 返回 None
  - UTF-8 invalid
  - 文件被另一个进程独占（不太可能但 catch）

## Dependencies

- **Upstream**: S-004 (classify), S-005 (list)
- **Downstream**: S-012 (log file)

## Files to touch

- `src-tauri/src/session.rs` — per-session try/catch + Session::unknown constructor
- `src-tauri/Cargo.toml` — 加 `log = "0.4"`（如果还没）

## Test plan

### 单元测试
- 构造一个 invalid JSONL（非法 UTF-8 / 非法 JSON / 缺字段）→ classify 返回 Unknown
- mock 一个 panic in process_one_session → catch_unwind 返回 Err，整个 list 不挂

### 集成测试
- 用 `dd if=/dev/urandom of=fake.jsonl bs=1k count=1` 造 binary noise
- 把这个文件放进 `~/.claude/projects/test/` 启动 claude
- 看 popup 显示 Unknown + "(unable to read transcript)"

### Chaos test
- 在 process_one_session 里随机 `panic!()` 验证恢复
- 用 `dd` 把 jsonl 切成 0 bytes → 验证 Unknown
- mid-write 中断（kill -9 claude 进程在写 JSONL 时）→ 验证下一轮自动恢复

## Definition of Done

- [ ] 代码 merged
- [ ] 故意 corrupt 1 个 jsonl，其他 session 仍正常显示
- [ ] panic test 通过（手动注入 panic 不挂 app）
- [ ] [F1 acceptance](../../../product/user-stories.md#f1--jsonl-损坏读不到) 全通过
- [ ] [NFR-R1](../../02-planning/PRD.md) "单 session 失败不影响其他" 实测
