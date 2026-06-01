# S-001 · Process enumeration (sysinfo wrapper)

**Epic:** [001 Core Monitoring](epic-001-core-monitoring.md)
**Status:** ✅ DONE (2026-05-18, all 11 tests pass — 7 unit + 4 integration)
**Estimate:** M (actual: ~1 hour incl. Rust toolchain upgrade)
**Owner:** caiyiwen

## Description

**As a** backend module
**I want to** enumerate all running `claude` CLI processes with their pid and cwd
**so that** downstream modules can locate each session's JSONL transcript.

## Acceptance criteria

- 函数签名：`pub fn list_processes() -> Vec<RawProcess>`
- `RawProcess { pid: u32, cwd: PathBuf, started_at: SystemTime }`
- 只返回 name == `"claude"` 或 cmd[0] basename == `claude` 的进程
- 跳过 UID 跟当前 app 进程不一致的（防 sudo 跑的 claude，见 [addendum § A.6](../../02-planning/addendum.md)）
- 返回耗时 < 25ms (10 processes 范围内) — 占 NFR-P1 总 50ms 的一半
- 进程数 ≥ 50 也不 panic
- 单元测试：mock sysinfo 返回固定列表，验证过滤逻辑

## Dev notes

- 使用 `sysinfo` crate (Cargo.toml 已加 0.30)
- `System::new()` + `System::refresh_processes()` 每次新建（不缓存——避免 stale）
- `Process::name()` 比 `cmd()` 简单，但 macOS 上 `name()` 是 short name（"claude"），check `cmd[0]` 是更稳的方案
- `Process::cwd()` 在 0.30 返回 `Option<&Path>`，None 时跳过（无 cwd 的 process 没法定位 JSONL）
- UID 检查：`Process::user_id()` vs `sysinfo::get_current_pid()` 的 owner

## Dependencies

- **Upstream**: 无
- **Downstream**: [S-002 JSONL locator](story-002-jsonl-locator.md)

## Files to touch

- `src-tauri/src/session.rs` — 新增 `list_processes()` 函数 + `RawProcess` struct
- `src-tauri/Cargo.toml` — 验证 sysinfo 0.30（应已添加）
- `src-tauri/tests/process_enum.rs` — 单元测试

## Test plan

### 单元测试
- mock 一组 process（含 claude / 非 claude / 不同 UID）验证过滤
- 边界：cmd 为空、name 长度 > 100、cwd None

### 集成测试 (手动)
1. `cargo run` 启动 app
2. 开两个真实 claude session
3. 调 `list_processes()` 通过 dbg!/log 验证返回 2 个 RawProcess
4. 关掉一个，再调，验证返回 1 个

### 性能测试
- `cargo bench` benchmark 在 10 / 50 / 100 process 数下耗时
- 加入 CI

## Definition of Done

- [ ] 代码 merged 到 main
- [ ] 单元测试通过 (≥ 5 个 case)
- [ ] 性能 benchmark 在 budget 内
- [ ] [architecture.md § 5.1](../architecture.md) 的 sysinfo budget 列实测数填上
- [ ] dogfood：1 天验证无误报/漏报
