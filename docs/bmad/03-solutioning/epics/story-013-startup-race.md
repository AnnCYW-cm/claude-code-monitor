# S-013 · Startup race: process exists but JSONL not yet

**Epic:** [003 Robustness](epic-003-robustness.md)
**Status:** Pending
**Estimate:** S (half day)
**Owner:** caiyiwen

## Description

**As** a user starting a claude session
**I want** the session to show up in my monitor immediately (even before its JSONL is written)
**so that** I get instant feedback that monitor is tracking new sessions.

## Acceptance criteria

- claude 进程刚启动时（< 1s），JSONL 文件可能还不存在
- 此时 session 仍出现在列表里，`status = Unknown`
- 下一轮 refresh 大概率 JSONL 已就绪，status 切换到 Working/Waiting
- 对应 [UML 09 State](../../../design/uml/09-state-session.md) 的 Unknown 自循环 → 转 Working/Waiting
- [E2 启动竞态 acceptance](../../../product/user-stories.md#e2--开机时已有-n-个-session-在跑)

## Dev notes

- 在 `locate_jsonl()` (S-002) 中：
  ```rust
  match locate_jsonl(proc) {
      Ok(path) => /* normal flow */,
      Err(LocateError::DirNotFound) | Err(LocateError::NoJsonlFiles) => {
          // JSONL 还没建好 → 返回 Session::unknown
          return Session::unknown(proc.pid, proc.cwd.clone());
      }
      Err(LocateError::IoError(e)) => {
          log::warn!("locate failed pid={}: {}", proc.pid, e);
          return Session::unknown(proc.pid, proc.cwd.clone());
      }
  }
  ```
- `Session::unknown` constructor 已在 S-011 定义
- 注意：DirNotFound 跟"未启动" 是同一个错——必须依靠 RawProcess 已经在 list_processes 返回（即进程实际存在），才走这条 fallback；如果进程根本不存在那也根本不会进入这个分支
- 不要因为 JSONL 不存在就把进程从列表里隐藏！这是 E2 跟 R5 (不显示历史) 的关键区分：进程在 = 显示；进程不在 = 不显示

## Dependencies

- **Upstream**: S-002 (locate_jsonl)、S-011 (Session::unknown constructor)
- **Downstream**: 无

## Files to touch

- `src-tauri/src/session.rs` — `process_one_session()` 的 fallback 分支

## Test plan

### 手动测试
1. 打开 monitor app
2. 在 terminal 开一个 claude session
3. 立刻看 popup → 应该看到该 session 出现，status = Unknown
4. 等 2 秒（下一轮 refresh），同时 claude 应该已开始写 JSONL → status 切换到 Working 或 Waiting

### 模拟测试
1. mock RawProcess 列表含一个 pid 对应的 cwd 是不存在的目录
2. 调 list_sessions → 该 session 出现，status = Unknown
3. mkdir 那个目录 + 写一个有效 JSONL
4. 再调 list_sessions → status 切换正确

## Definition of Done

- [ ] 代码 merged
- [ ] 启动竞态 acceptance 通过
- [ ] [E2](../../../product/user-stories.md#e2--开机时已有-n-个-session-在跑) 全 acceptance 通过
- [ ] [UML 09 Unknown 自循环转移](../../../design/uml/09-state-session.md) 在代码里正确反映
