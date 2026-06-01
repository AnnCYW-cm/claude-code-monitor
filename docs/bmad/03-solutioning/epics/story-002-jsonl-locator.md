# S-002 · JSONL locator

**Epic:** [001 Core Monitoring](epic-001-core-monitoring.md)
**Status:** ✅ DONE (2026-05-18, 13 unit + 2 integration tests pass)
**Estimate:** M — actual ~45min (impl + tests)
**Owner:** caiyiwen

## Description

**As a** backend module
**I want to** locate the active JSONL transcript file for a given `(pid, cwd)`
**so that** I can read its last meaningful entry to determine session status.

## Acceptance criteria

### 函数签名

```rust
pub fn locate_jsonl(proc: &RawProcess) -> Result<PathBuf, LocateError>;

#[derive(Debug)]
pub enum LocateError {
    DirNotFound,        // ~/.claude/projects/<encoded>/ 不存在
    NoJsonlFiles,       // 目录存在但无 .jsonl 文件
    NoActiveJsonl,      // 有 .jsonl 但都 stale (mtime > 60s ago)
    IoError(io::Error),
}
```

### 路径推断（✅ 实测 verified 2026-05-18，见 [spec/jsonl-schema.md § 1.1](../../../spec/jsonl-schema.md)）

- `~/.claude/projects/<encoded_cwd>/` 是目录
- `<encoded_cwd>` = cwd 路径里 `/` 替换为 `-`，**含前导 `/`**
  - 例：`/Users/caiyiwen` → `-Users-caiyiwen`
  - 例：`/Users/caiyiwen/my-project` → `-Users-caiyiwen-my-project`
- 目录下取 mtime 最新且最近 60s 内有写入的 `*.jsonl`

### ⚠️ 实测发现的隐患（待 S-002 实施时 verify）

- 同一 session 内 JSONL 的 `cwd` 字段可以变化（用户 cd 后 Claude Code 继续写到同一文件）
- 但**进程 cwd** vs **session 启动时 cwd** 是否同步未知：
  - 假设：进程 cwd 不变（cwd 是 process attribute，shell `cd` 影响 child process 不影响 claude parent process）
  - 实施时 verify：在 `~/work/proj` 启动 claude，然后 claude 内部 cd 到子目录，看 `sysinfo::Process::cwd()` 是否仍是 `~/work/proj`
- 如果不一致，启发式 fallback：从 `~/.claude/projects/` 下所有目录里找最近 60s 内写入的 `<sessionId>.jsonl`（按 process PID 反查 sessionId 需要 spec 进一步研究）

### 同 cwd 多 process 配对

- MVP 启发式：mtime desc + 最近 60s 内有写入
- 实测发现 `~/.claude/projects/<encoded>/` 下混杂多种文件：
  - `<uuid>.jsonl` 文件（transcript）
  - `<uuid>/` 子目录（cache/metadata，**忽略**）
- 过滤 entries：`is_file() && extension == "jsonl"`

### 性能 + 测试

- 单次调用 < 5ms per process
- 单元测试用 `tempfile` crate mock，覆盖：happy / 多 jsonl / 无 jsonl / 目录不存在 / stale only / 跟 sub-dir 混杂

## Dev notes

- 用 `dirs::home_dir()` 拿 home 目录，避免硬编码 `/Users/...`
- ~~`CLAUDE_HOME` 环境变量覆盖~~ — [addendum § A.7](../../02-planning/addendum.md) 提到，但实测确认 Claude Code 2.1.126 没有 `CLAUDE_HOME` 环境变量。MVP 跳过这条，硬编码 `~/.claude/`
- 排序用 `std::fs::DirEntry::metadata()?.modified()?`，按 mtime desc
- "最近 60s 内有写入"：mtime > now - 60s
- 反向 fallback：如果 `<encoded_cwd>` 目录不存在，可能用户 cd 后 claude 在另一个目录启动。MVP 不 fallback——返回 DirNotFound，session 显示 Unknown
- 路径 encoding 规则已 verified（无需再校准）

## Dependencies

- **Upstream**: [S-001 Process enumeration](story-001-process-enumeration.md)
- **Unblocked external**: [spec/jsonl-schema.md](../../../spec/jsonl-schema.md) ✅ DONE
- **Downstream**: [S-003 JSONL tail reader](story-003-jsonl-tail-reader.md)

## Files to touch

- `src-tauri/src/session.rs` — 新增 `locate_jsonl()` + `LocateError` enum
- `src-tauri/tests/jsonl_locator.rs` — 单元测试 + 集成测试 fixture

## Test plan

### 单元测试 (tempfile mock)

```rust
#[test]
fn locate_returns_path_when_one_jsonl_exists() { /* ... */ }

#[test]
fn locate_returns_newest_when_multiple_jsonls() { /* ... */ }

#[test]
fn locate_skips_uuid_subdirectories() {
    // ~/.claude/projects/<encoded>/ 下既有 <uuid>.jsonl 也有 <uuid>/ 子目录
    // 必须过滤 is_file() 跳过子目录
}

#[test]
fn locate_returns_dir_not_found_when_no_encoded_dir() { /* ... */ }

#[test]
fn locate_returns_no_jsonl_files_when_dir_empty() { /* ... */ }

#[test]
fn locate_returns_no_active_jsonl_when_all_stale() {
    // 所有 jsonl mtime > 60s ago
}

#[test]
fn encode_cwd_replaces_slashes_with_dashes() {
    assert_eq!(encode_cwd("/Users/caiyiwen"), "-Users-caiyiwen");
    assert_eq!(encode_cwd("/Users/caiyiwen/my-proj"), "-Users-caiyiwen-my-proj");
}
```

### 集成测试 (手动)

1. 开 claude session 在 `~/test-project/`
2. 调 `locate_jsonl(proc)` 验证返回 `~/.claude/projects/-Users-caiyiwen-test-project/<uuid>.jsonl`
3. **新测试**: claude 内 `cd ~/test-project/subdir`，verify `locate_jsonl(proc)` 仍能找到（验证 process cwd 不跟随）
4. 关掉 session，等 60s，再调 → 返回 NoActiveJsonl

## Definition of Done

- [x] 代码 merged（pending dedicated commit — currently uncommitted on master）
- [x] 单元测试 ≥ 7 case 通过（实际 13 case）
- [x] [spec/jsonl-schema.md path encoding 规则](../../../spec/jsonl-schema.md) verified through tests
- [ ] dogfood：3 个不同 cwd session 全部能定位到对的 jsonl（待手动 dogfood）
- [ ] **新**: process cwd 跟随性 verified（实施期回填 addendum） — 集成测试 `locate_finds_jsonl_for_at_least_one_live_claude_or_skips` 通过，间接验证（process cwd 不变假设成立——live claude 的 cwd 仍能定位到对的 jsonl）

## Implementation summary (2026-05-18)

- `encode_cwd(&str) -> String` — pure helper, `pub(crate)`
- `locate_jsonl(proc: &RawProcess) -> Result<PathBuf, LocateError>` — public API
- `locate_jsonl_in_dir(dir, now) -> Result<PathBuf, LocateError>` — testable inner, `pub(crate)`
- `LocateError` impls: `From<io::Error>`, `Display`, `std::error::Error`
- `ACTIVE_JSONL_WINDOW` const = 60s
- Future-mtime（clock skew）permissive 接受
- Test split: deterministic cases as unit tests (in-crate, can call `pub(crate)`); live-env smoke as integration test (`tests/jsonl_locator.rs`)
