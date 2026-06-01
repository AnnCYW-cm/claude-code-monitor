# S-012 · Logging to `~/Library/Logs/...`

**Epic:** [003 Robustness](epic-003-robustness.md)
**Status:** Pending
**Estimate:** S (half day)
**Owner:** caiyiwen

## Description

**As** the app
**I want to** write structured logs to a standard macOS location
**so that** users and the author can debug issues without enabling verbose mode.

## Acceptance criteria

- Log 路径：`~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`
- 启动时确保目录存在（mkdir -p）
- 格式：`{ISO 8601 timestamp} [{LEVEL}] {message}`
  - 例：`2026-05-18T14:08:30Z [INFO] startup: tray icon registered, polling started`
- Levels:
  - `ERROR`: parse 失败、tray init 失败、IO 错误
  - `WARN`: 配对 fallback、JSONL 不存在
  - `INFO`: 启动 / 退出 / tray click
  - `DEBUG`: 每次 refresh tick（默认关，`RUST_LOG=debug` 开）
- App 启动时 log 一行 startup banner（版本 + log 路径）
- 写 log 失败时 silently fail（不能因为 log 影响主功能 - addendum § A.6）

## Dev notes

- Crate 选择：`log` (facade) + `env_logger` 或 `simplelog` 或 `flexi_logger`
- 建议 `flexi_logger`——支持 macOS 标准位置、自动 rotation 配置（虽然 MVP 不 rotate）
- 或者最简：手写 `WriteLogger` impl，用 `std::fs::OpenOptions::append`
- 默认 level 在 release 是 INFO，debug 时是 DEBUG
- 路径生成：
  ```rust
  let log_dir = dirs::home_dir()
      .unwrap()
      .join("Library/Logs/com.caiyiwen.claude-code-monitor");
  std::fs::create_dir_all(&log_dir).ok();  // silent fail
  let log_path = log_dir.join("main.log");
  ```

## Dependencies

- **Upstream**: 无（独立基础设施）
- **Downstream**: S-011 (parse failure 要写 log)

## Files to touch

- `src-tauri/src/lib.rs` — log init 在 `run()` 开头
- `src-tauri/Cargo.toml` — 加 `log = "0.4"` + `flexi_logger = "0.27"` (或 simplelog)
- `src-tauri/src/session.rs` — 所有 error path 用 `log::error!` / `log::warn!`

## Test plan

### 手动测试
- `cargo run` → log 文件出现在 `~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`
- 第一行是 startup banner
- 触发一些 normal 行为 → INFO log 出现
- 故意 corrupt JSONL → ERROR log 出现
- `RUST_LOG=debug cargo run` → DEBUG log 出现（每 2s 一条 refresh tick）

### 路径 edge case
- 删掉 log dir → 启动重建
- chmod 777 否定写权限 → silent fail，app 仍跑

## Definition of Done

- [ ] 代码 merged
- [ ] log file 路径正确 (macOS 标准)
- [ ] format 跟 [architecture § 9.3](../architecture.md) 一致
- [ ] dogfood 1 天 → log 文件 < 10MB（合理）
- [ ] [F1 acceptance "log file 记录"](../../../product/user-stories.md#f1--jsonl-损坏读不到) 实测
