# Project Context — Implementation Conventions

> **BMAD Phase 3 · Solutioning · Analyst output (after architecture)**
> **Status:** Draft → 实现期持续更新
>
> 这份文档是给 dev agent / contributor / 未来的自己看的「项目实现规范」。它定义：
> - 编码约定（命名 / 错误处理 / 注释）
> - 模块边界（什么放哪里、谁不该 import 谁）
> - 依赖列表（用什么、不用什么）
> - 测试约定
> - PR / commit 约定
>
> 跟 [architecture.md](architecture.md) 互补：architecture 说 what 和 why，project-context 说 how（你写代码时要遵守什么）。

---

## 1. 技术栈快查

| 层 | 选择 | 版本 | 备注 |
|---|---|---|---|
| App 框架 | Tauri | 2.x | [ADR-001](../02-planning/decision-log.md#adr-001--选-tauri-2x-作为-app-框架) |
| 后端语言 | Rust | 1.77+ | edition 2021 |
| 进程枚举 | sysinfo crate | 0.30 | 锁住 minor 版本 |
| Serialization | serde + serde_json | 1.x | 标准 |
| Logging | log + flexi_logger | 0.4 + 0.27 | [S-012](epics/story-012-logging.md) |
| 前端语言 | TypeScript | 5.4+ | strict mode |
| 前端 bundler | Vite | 5.2+ | Tauri 模板默认 |
| 前端框架 | 无（vanilla） | — | [ADR-006](../02-planning/decision-log.md#adr-006--前端不引入框架vanilla-ts) |

---

## 2. 目录结构（代码部分）

```
claude-code-monitor/
├── src/                          ← Frontend (TypeScript)
│   ├── main.ts                   ← Entry, polling, render
│   └── style.css                 ← All styles (依 ux-design.md)
├── src-tauri/                    ← Backend (Rust)
│   ├── src/
│   │   ├── main.rs               ← Binary entry (thin)
│   │   ├── lib.rs                ← App setup, tray, window, commands
│   │   └── session.rs            ← Session struct + all session logic
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── icons/
│       └── icon.png              ← 32×32 tray icon
├── index.html                    ← Webview entry
├── package.json
├── vite.config.ts
├── tsconfig.json
└── docs/                         ← 全部文档
```

---

## 3. 模块边界与依赖规则

### 3.1 后端模块依赖图

```
main.rs ────┐
            ▼
         lib.rs ──┬──→ session.rs
                  │
                  └──→ (future) ui.rs / log.rs / config.rs
```

**规则**:
- `main.rs` 只调用 `lib.rs::run()`，不含业务逻辑
- `lib.rs` 调用 `session.rs`，**不** 反向（session.rs 不调用 lib.rs 的任何东西）
- `session.rs` 是 pure logic + IO，**不依赖** tauri / webview
- 如果 session.rs 需要 emit event 给 frontend，必须通过 `lib.rs` 包装暴露（保持 session.rs 可独立 unit test）

### 3.2 前端模块

MVP 阶段只有一个 `main.ts`。如果未来增长：

```
src/
├── main.ts             ← entry
├── api.ts              ← IPC wrapper (invoke calls)
├── render.ts           ← DOM render functions
└── state.ts            ← UI state (expanded item etc)
```

但 MVP 不预拆，单文件 < 200 行可读。

### 3.3 不允许的依赖

跟 [constitution § II.5](../../constitution.md) 保持一致（constitution 是 single source of truth）：

| 不允许 | 理由 |
|---|---|
| http client (reqwest / hyper / isahc 等) | NFR-S1 不访问网络 (constitution I.1) |
| 我们的代码显式调用 `tokio::` API | [ADR-007](../02-planning/decision-log.md#adr-007--list_sessions-ipc-设计为同步阻塞) 同步阻塞 (Tauri 内部自带 tokio runtime 是正常的，不计) |
| `notify` crate (fs watcher) | [ADR-002](../02-planning/decision-log.md#adr-002--监控状态用-polling-不用-fs-watcher) |
| React / Vue / Svelte 等前端框架 | [ADR-006](../02-planning/decision-log.md#adr-006--前端不引入框架vanilla-ts) |
| `osascript` / AppleScript 调用 | 跟终端 emulator 解耦 (constitution I.4) |
| Notification crate / 任何 macOS Notification API | [ADR-004](../02-planning/decision-log.md#adr-004--永不通知产品红线) 红线 (constitution I.1) |

**额外（本文档加，constitution 没列）**：
| 不允许 | 理由 |
|---|---|
| 前端引入 jQuery / underscore / lodash | 不需要，纯 vanilla JS 够 |

→ CI 加 grep 阻断（详见 § 9）。

---

## 4. 命名约定

### 4.1 Rust

- Type / struct / enum / trait：`PascalCase` (`Session`, `JsonlEnvelope`, `NestedMessage`, `ContentValue`, `ContentBlock`, `LocateError`)
- Function / method / variable / module：`snake_case` (`list_sessions`, `tail_jsonl`, `last_meaningful`)
- Constants：`SCREAMING_SNAKE_CASE` (`POLL_INTERVAL_SECS`)
- 用 `unwrap()` 仅在 setup（panic 即 fail-fast）；runtime 必须用 `?` 或 match
- JSONL envelope 字段从 camelCase rename：`#[serde(rename = "type")]` 处理 `type` keyword；`#[serde(rename = "parentUuid")]` 处理 camelCase 来源字段。详见 [data-model.md § 2.2-2.4](data-model.md)

### 4.2 TypeScript

- Type / interface：`PascalCase` (`Session`, `RenderOptions`)
- Function / variable：`camelCase` (`refreshSessions`, `expandedPid`)
- Constants：`SCREAMING_SNAKE_CASE` (`POLL_INTERVAL_MS = 2000`)
- 文件名：`kebab-case` if multi-word (`main.ts`, `api-types.ts`)

### 4.3 IPC / serde 字段

- Rust struct 字段：`snake_case`
- 序列化到 JSON：保持 `snake_case`（用 `#[serde(rename_all = "snake_case")]` 如果需要）
- TS interface 镜像 Rust struct：用 `snake_case` 字段（不要 camelCase 转换增加 friction）

```rust
#[derive(Serialize)]
pub struct Session {
    pub pid: u32,
    pub cwd: String,
    pub status: SessionStatus,
    pub last_message: Option<String>,
    pub last_update_unix: Option<u64>,
}
```

```ts
interface Session {
  pid: number;
  cwd: string;
  status: "waiting" | "working" | "unknown";
  last_message: string | null;
  last_update_unix: number | null;
}
```

### 4.4 Tauri commands

- `snake_case` 函数名 (`list_sessions`)
- 前端 `invoke("list_sessions")`——命令名不变 case

---

## 5. 错误处理约定

### 5.1 Rust

- 业务函数返回 `Result<T, E>`，自定义 `E` enum
- IO 错误 wrap 成自定义 enum 变种（`MyError::Io(io::Error)`）
- 用 `thiserror` crate 简化 enum 定义（如果引入）—— MVP 手写

```rust
#[derive(Debug)]
pub enum LocateError {
    DirNotFound,
    NoJsonlFiles,
    IoError(io::Error),
}

impl From<io::Error> for LocateError {
    fn from(e: io::Error) -> Self { LocateError::IoError(e) }
}
```

- `list_sessions` IPC 不返回 Result（前端不处理），错误转化为 `status=Unknown` + log
- 顶层 `setup` 闭包内 panic 是可接受的（fail-fast）

### 5.2 TypeScript

- 用 try/catch + `Promise.catch()`
- IPC error 不挂 UI，console.error + 显示 stale data

```ts
async function refresh() {
  try {
    const sessions = await invoke<Session[]>("list_sessions");
    render(sessions);
  } catch (e) {
    console.error("list_sessions failed", e);
    // 保持 stale UI
  }
}
```

---

## 6. 注释 / 文档约定

### 6.1 通用

- **不写废话注释**（已被 well-named identifier 解释的不写）
- WHY 注释>WHAT 注释
- 复杂算法 / 非显然边界写一段说明
- 公开 API（`pub fn`）写 doc comment（`///`）

### 6.2 Rust doc

```rust
/// Read the last line of a JSONL file without loading the whole file.
///
/// Implementation: seeks to EOF and scans backward for `\n`.
/// Cost: < 10ms for files up to 100MB (see [architecture.md § 5.1]).
///
/// Returns `Ok(None)` if file is empty.
pub fn tail_jsonl(path: &Path) -> Result<Option<String>, io::Error> {
    // ...
}
```

### 6.3 TypeScript

```ts
/**
 * Format unix seconds elapsed into a human-readable string.
 *
 * - < 60s → "just now"
 * - 1+ min → "Nmin"
 *
 * Matches ux-design.md § 5.3 spec.
 */
function formatDuration(unixSeconds: number): string { /* ... */ }
```

---

## 7. 测试约定

### 7.1 单元测试位置

- Rust: 同文件 `#[cfg(test)] mod tests { ... }`
- 或单独 `src-tauri/tests/<topic>.rs` for integration

### 7.2 测试命名

```rust
#[test]
fn tail_jsonl_returns_none_on_empty_file() { /* ... */ }

#[test]
fn classify_returns_waiting_when_assistant_msg_has_no_pending_tool() { /* ... */ }
```

→ 形如 `<subject>_<expected>_<when_condition>`，能读出测试意图。

### 7.3 mock 策略

- 文件系统 mock：用 `tempfile` crate 真实临时目录
- sysinfo mock：trait-based dependency injection（如果有时间），否则直接构造 `RawProcess` 字面量

### 7.4 测试覆盖率目标

- session.rs: ≥ 80%（关键路径）
- lib.rs: 整体集成测试（手动 + dogfood）
- frontend: 暂不要求（依靠 manual + visual regression）

---

## 8. 性能约束

跟 [architecture.md § 5](architecture.md) + [PRD § 6.1](../02-planning/PRD.md) 一致。代码 PR 必须满足：

- `list_sessions` < 50ms (10 session) - benchmark in CI
- `tail_jsonl` < 10ms (100MB file) - benchmark
- popup show/hide < 200ms - manual
- 空闲 CPU < 0.5% (M1) - dogfood 用 Activity Monitor

---

## 9. CI 规则（防红线被破）

`.github/workflows/lint.yml` (待加) 应包含：

```yaml
- name: Forbid forbidden patterns
  run: |
    # 不允许 http client
    ! grep -rn "reqwest\|hyper\|isahc" src-tauri/src/

    # 不允许 notification
    ! grep -rn "Notification\|notify\|osascript" src-tauri/src/
    ! grep -rn "Notification\|alert(" src/

    # 不允许 tokio runtime (sysinfo 用 std::process 不依赖 tokio)
    ! grep -rn "tokio::" src-tauri/src/

    # 不允许前端引入框架
    ! grep -rn "react\|vue\|svelte" package.json

    # 不允许 osascript / AppleScript
    ! grep -rn "osascript\|AppleScript" src-tauri/src/
```

CI 阻断 = PR 不能 merge。违反需要先开 ADR 推翻 R1/R2/R3/R4 之一。

---

## 10. PR 约定

### 10.1 PR title

格式：`<type>: <short desc>`
- `feat:` 新功能
- `fix:` bug fix
- `docs:` 文档
- `chore:` 工程基础设施
- `refactor:` 代码重构（无功能改动）
- `test:` 测试

例：
- `feat(s-001): implement process enumeration with sysinfo`
- `fix(s-008): list re-render flickers when sessions count changes`

### 10.2 PR description

```markdown
## Story
- Closes `S-NNN` (替换为实际 story ID，链接到 `docs/bmad/03-solutioning/epics/story-NNN-xxx.md`)

## What changed
- 简要描述

## How tested
- 测试方法 + 结果

## Checklist
- [ ] All acceptance criteria met
- [ ] Unit tests added
- [ ] Performance budget verified (if applicable)
- [ ] Docs updated (if applicable)
```

### 10.3 Commit 约定

- 一个 commit 一个 logical change
- commit message 第一行 ≤ 70 字符
- body 写 WHY 不写 WHAT (diff 自己看)
- 不写 `WIP`，要写就 squash

---

## 11. Cargo / npm 命令

| 命令 | 用途 |
|---|---|
| `npm install` | 装前端依赖 |
| `npm run tauri:dev` | 开发模式（hot reload） |
| `npm run tauri:build` | release build (生成 .app + .dmg) |
| `cargo test --manifest-path=src-tauri/Cargo.toml` | 跑 Rust 测试 |
| `cargo bench --manifest-path=src-tauri/Cargo.toml` | 跑 benchmark |
| `cargo clippy --manifest-path=src-tauri/Cargo.toml` | lint Rust |
| `npm run build` | 仅前端 build |

---

## 12. macOS-specific 注意

- **代码签名**：MVP 不签名（成本 + 复杂度）。release build 后用户首次打开会触发 Gatekeeper（F4）
- **Universal binary**：用 `cargo build --target universal2-apple-darwin` 同时 x86_64 + arm64
- **Bundle**: Tauri build 自动产 `.app` bundle 在 `src-tauri/target/release/bundle/macos/`
- **icon set**: tray PNG (32×32) + bundle icon (.icns)，[ux-design § 13](../02-planning/ux-design.md) flag 待办

---

## 13. 跟现有 scaffold 的关系

`src-tauri/src/` 当前已有 scaffold 实现：
- `main.rs` — 已完成（thin entry）
- `lib.rs` — 已完成（tray + window + IPC stub）
- `session.rs` — **占位**（只按 name match，没读 JSONL）

每个 epic / story 完成后更新对应文件，详见 [epics/README.md § Dev story → Product story 反向映射](epics/README.md)。

---

## 14. 待办（**作者待办**）

- [ ] CI workflow 写好（`.github/workflows/lint.yml`）
- [ ] benchmark 工程结构（`src-tauri/benches/`）
- [ ] 编辑器配置统一（`.editorconfig`）
- [ ] git hook (pre-commit 跑 clippy + cargo fmt)

---

## 15. 在哪查约定

| 类型问题 | 看哪 |
|---|---|
| "我应该用什么 crate" | 本文档 § 3.3 不允许列表 + § 1 stack |
| "新代码放哪个文件" | 本文档 § 2-3 + [architecture § 3](architecture.md) |
| "命名怎么起" | § 4 |
| "错误怎么处理" | § 5 |
| "测试怎么写" | § 7 |
| "为什么这样设计" | [decision-log.md](../02-planning/decision-log.md) |
| "用户要的是什么" | [user-stories.md](../../product/user-stories.md) + [PRD.md](../02-planning/PRD.md) |
