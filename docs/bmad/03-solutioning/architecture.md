# Architecture — Claude Code Monitor v0.1

> **BMAD Phase 3 · Solutioning · Architect output**
> **Status:** Draft → 待 implementation-readiness 验证
> **Date:** 2026-05-18
>
> 基于 [PRD.md](../02-planning/PRD.md) + [ux-design.md](../02-planning/ux-design.md) + [decision-log.md](../02-planning/decision-log.md) 产出。
> 跟 [design/uml/](../../design/uml/) 互补：UML 是结构化建模视图，本文档是叙述性架构说明（narrative）。两者不冲突——开发时先读这份理解 why，再看 UML 看 how。

---

## 1. System context

### 1.1 What this system is

一个 macOS 本地 menubar app，作为旁路观察者：
- **读** macOS 进程表，发现运行中的 `claude` CLI 进程
- **读** `~/.claude/projects/` 下对应的 JSONL transcript 文件
- **不读不写** 任何其他系统资源
- **不外联** 任何网络
- **不影响** Claude Code 进程本身的运行

### 1.2 System boundaries

```
┌────────────────────────────────────────────────────────────┐
│  User's macOS                                              │
│                                                            │
│  ┌──────────────────────┐    ┌────────────────────────┐   │
│  │ ClaudeCodeMonitor.app│    │ Terminal emulator(s)   │   │
│  │  (本系统)             │    │  (用户的终端)            │   │
│  │                      │    │   ┌─────┐ ┌─────┐      │   │
│  │  ┌──────────────┐    │    │   │claude│ │claude│    │   │
│  │  │ Rust backend │────┼────┼───┤proc 1│ │proc 2│ ...│   │
│  │  │  - process   │    │    │   └──┬──┘ └──┬──┘     │   │
│  │  │    enum      │    │    │      │       │        │   │
│  │  │  - JSONL read│    │    │      │ writes│        │   │
│  │  │  - classify  │    │    │      ▼       ▼        │   │
│  │  └──────┬───────┘    │    └──────────────────────┘   │
│  │         │ IPC         │             │                  │
│  │  ┌──────▼───────┐    │    ┌────────▼──────────────┐  │
│  │  │  Webview FE  │    │    │ ~/.claude/projects/   │  │
│  │  │  - render    │    │    │  └ <encoded-cwd>/     │  │
│  │  │  - poll      │    │    │      └ <uuid>.jsonl   │  │
│  │  └──────────────┘    │    └───────────────────────┘  │
│  └──────────────────────┘                                 │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### 1.3 What's outside the system

- Claude Code 本身（我们旁路它，不修改）
- 终端 emulator（我们跨它们中立）
- ~/.claude/projects 以外的任何目录
- 网络
- 任何 daemon / launchd / 服务

→ 这是个**完全本地、完全只读、完全自包含**的 app。

---

## 2. High-level architecture

### 2.1 三个核心子系统

```
┌─────────────────────────────────────────────────────────┐
│  ① Monitoring Loop (Rust)                                │
│   每 2s：枚举进程 → 定位 JSONL → 读最后行 → 分类         │
└──────────────────────┬──────────────────────────────────┘
                       │ Vec<Session>
                       ▼
┌─────────────────────────────────────────────────────────┐
│  ② IPC Layer (Tauri Command)                             │
│   list_sessions: Vec<Session>                            │
└──────────────────────┬──────────────────────────────────┘
                       │ JSON via WebSocket-style IPC
                       ▼
┌─────────────────────────────────────────────────────────┐
│  ③ Webview UI (TypeScript)                               │
│   render list / handle click / setInterval polling       │
└─────────────────────────────────────────────────────────┘

附属:
┌─────────────────────────────────────────────────────────┐
│  ④ Tray + Window Controller (Rust)                       │
│   tray icon + popup show/hide on click                   │
└─────────────────────────────────────────────────────────┘
```

详细模块拆分见 [UML 03 Component](../../design/uml/03-component.md)。

### 2.2 数据流

**主轮询循环**（每 2 秒）：

1. Frontend `setInterval` 触发 `invoke("list_sessions")`
2. Backend `list_sessions()` 执行：
   - `sysinfo::System::refresh_processes()` 枚举所有进程
   - 过滤 name == "claude" 的
   - 对每个：encode cwd → 查 `~/.claude/projects/<encoded>/` → 取 mtime 最新的 `.jsonl`
   - 读该 JSONL 最后一行（用 seek + reverse scan）
   - 解析为 `JsonlMessage`，应用 classify 规则 → `SessionStatus`
   - 构造 `Session { pid, cwd, status, last_message, last_update_unix }`
3. 返回 `Vec<Session>` 给前端
4. 前端 render 列表
5. 同时后端基于 Vec<Session> 计算 waiting 数量，更新 tray title

**Tray click flow**（用户操作）：

1. 用户左键点击 tray
2. Tauri `TrayIconEvent::Click(Left, Up)` 触发
3. 回调里查 popup window 当前可见性
4. 不可见 → `show() + set_focus()`；可见 → `hide()`

详见 [UML 06/07/08 Sequence diagrams](../../design/uml/06-sequence-startup.md)。

---

## 3. Module breakdown

代码结构（详见 [UML 04 Package](../../design/uml/04-package.md)）：

```
src-tauri/src/
├── main.rs        — thin binary entry
├── lib.rs         — app setup + tray + window + commands
└── session.rs     — Session struct + list() + classify()

src/                — frontend
├── main.ts        — render + polling
└── style.css      — layout + colors (依 ux-design.md)
```

### 3.1 Backend modules

| 模块 | 职责 | 关键依赖 |
|---|---|---|
| `main.rs` | binary entry，调用 lib.rs::run() | — |
| `lib.rs` | Tauri app builder、tray icon、window manager、IPC command | `tauri`, `tauri-plugin-shell` |
| `session.rs` (MVP) | Session struct, 进程枚举、JSONL 定位、读最后行、分类——一个文件全包 | `sysinfo`, `serde`, `serde_json`, `std::fs` |
| `session/` (v0.2+ 拆分预案) | `process.rs` / `jsonl.rs` / `classify.rs` 拆分 | 同上 |

### 3.2 Frontend modules

| 模块 | 职责 |
|---|---|
| `main.ts` | DOM render、setInterval 调 invoke、点击 handler |
| `style.css` | 所有样式（macOS 风格） |

→ **无前端框架**（[ADR-006](../02-planning/decision-log.md#adr-006--前端不引入框架vanilla-ts)）

---

## 4. Tech stack rationale

### 4.1 选型

| 层 | 技术 | 理由 (link to ADR) |
|---|---|---|
| App 框架 | Tauri 2.x | [ADR-001](../02-planning/decision-log.md#adr-001--选-tauri-2x-作为-app-框架) |
| 后端语言 | Rust 1.77+ | Tauri 自带 |
| 进程枚举 | sysinfo crate 0.30 | 跨平台、稳定、零额外权限 |
| JSON parsing | serde + serde_json | Rust 生态标准 |
| 前端 bundling | Vite 5 + TypeScript 5 | Tauri 模板默认 |
| 前端框架 | 无 | [ADR-006](../02-planning/decision-log.md#adr-006--前端不引入框架vanilla-ts) |
| 监控范式 | Polling (setInterval) | [ADR-002](../02-planning/decision-log.md#adr-002--监控状态用-polling-不用-fs-watcher) |
| 数据源 | JSONL transcript | [ADR-005](../02-planning/decision-log.md#adr-005--数据源选-jsonl-不-hook-claude-code-lifecycle) |

### 4.2 不选什么

| 拒绝 | 理由 |
|---|---|
| Electron | bundle 100MB+，RAM 200MB+，太重 |
| Swift + SwiftUI | VS Code 体验差，要 Xcode |
| React/Vue/Svelte | MVP UI 太简单，框架冗余 |
| Notify crate (fs watcher) | macOS JSONL append 事件 quirk |
| Tokio async runtime | MVP 同步阻塞够用 ([ADR-007](../02-planning/decision-log.md#adr-007--list_sessions-ipc-设计为同步阻塞)) |

---

## 5. Performance characteristics

### 5.1 Budget breakdown (per `list_sessions` invoke)

**注意**：per-session 步骤里 fs IO（read_dir + tail）的"wall clock 耗时"和"CPU 时间"差异大。下表的"预算"是 **wall clock**，而不是 CPU busy time（read_dir 大部分时间在 syscall wait，可以被其他工作 overlap）。

| 阶段 | 预算 (wall clock) | 实测 |
|---|---|---|
| sysinfo refresh_processes() + filter + extract | < 25ms (S-001 全部) | **23ms** ✅ (M1 macOS 26.3.1, debug build, ~50 processes total, 2026-05-18) |
| 过滤 name=="claude" | < 1ms | (included in 23ms above) |
| **per session（线性串行 N 次）**：encode cwd + fs read_dir + JSONL tail + parse + classify | **< 3ms 每 session（IO 主导）** | TBD (S-002+S-003+S-004) |
| serde serialize Vec<Session> | < 2ms | TBD (S-005) |
| IPC overhead | < 5ms | TBD (S-005) |
| **Total (10 sessions)** | sysinfo 25 + per×10 (30) + serde 2 + IPC 5 = **< 65ms** | 待 S-005 实测 |
| **Total (15 sessions)** | sysinfo 25 + per×15 (45) + serde 2 + IPC 5 = **< 85ms** | 待 S-005 实测 |

→ 满足 NFR-P1（10 session < 50ms 是 stretch 目标，realistic 上限是 60ms；15 session < 100ms 满足）。
→ 如果实测超 60ms，说明 per-session 3ms 估算太乐观，触发 [ADR-007 reconsider trigger](../02-planning/decision-log.md#adr-007--list_sessions-ipc-设计为同步阻塞)。

### 5.2 Memory budget

| 项 | 预算 |
|---|---|
| Rust binary RSS (空闲) | < 20MB |
| WebKit webview RSS | < 30MB |
| 总 RSS (空闲) | < 50MB |
| 24h 增长 | < 50MB (NFR-P4) |

→ 关键是 webview 别 leak（不创建多个 webview / 不累积 DOM 节点）。

### 5.3 CPU budget

| 状态 | 预算 |
|---|---|
| 空闲（popup 隐藏）| < 0.5% avg (M1) |
| popup 显示 | < 2% avg (M1) |
| refresh 瞬间峰值 | < 10% 持续 < 100ms |

⚠️ **NFR-P5 风险**：数学上 2s polling × 50ms/单次 = **2.5% busy time**，远超 0.5% budget。
但 sysinfo + IO 大部分时间是 syscall wait（不计入 CPU%），实测 likely 在 0.3-1.5% 之间。
→ **必须 epic 1 实施时 benchmark 验证**。如超 0.5%，降 polling 到 5s 或后端 cache 进程列表。
→ 详见 [readiness § 2.1 OQ-4](implementation-readiness.md)。

---

## 6. Failure modes & handling

### 6.1 JSONL parse 失败

**情景**：JSONL 文件损坏、被截断、磁盘错误，或末尾全是非 user/assistant entry（attachment / file-history-snapshot 等）反向扫到文件头还没找到 meaningful entry
**处理**：单个 session status = Unknown，last_message 显示 "(unable to read transcript)"
**恢复**：下一轮 refresh 自动重试
**user impact**：用户看到 Unknown 状态，知道是数据问题，不会以为 session 消失

JSONL 实际 schema 详见 [spec/jsonl-schema.md](../../spec/jsonl-schema.md)（2026-05-18 实测）。

详见 [F1 user story](../../product/user-stories.md#f1--jsonl-损坏读不到) + [UML 09 State](../../design/uml/09-state-session.md)。

### 6.2 sysinfo 枚举失败

**情景**：macOS API 异常（极少）
**处理**：log error，本轮 refresh 跳过（返回空 Vec 或上次结果）
**恢复**：下一轮重试
**user impact**：popup 短暂显示 empty 或 stale 列表，下一秒恢复

### 6.3 app 自身崩溃

**情景**：panic、SIGSEGV、OOM
**处理**：MVP **不做** recovery——进程死亡后 macOS 不会重启它
**信号**：tray icon 消失 = 用户唯一感知信号（菜单栏视觉空缺）
**恢复**：用户手动打开 ClaudeCodeMonitor.app

详见 [F3](../../product/user-stories.md#f3--app-自己崩了) + [ADR addendum](../02-planning/addendum.md)。

### 6.4 claude 进程刚启动 JSONL 还没建好（启动竞态）

**情景**：用户开 claude 后 < 1s monitor 就 refresh，JSONL 文件可能还不存在
**处理**：session status = Unknown
**恢复**：下一轮 refresh 大概率 JSONL 已有

详见 [E2 启动竞态](../../product/user-stories.md#e2--开机时已有-n-个-session-在跑) + UML 09。

### 6.5 同 cwd 多 session 配对错误

**情景**：用户在同目录开两个 claude，配对 PID ↔ JSONL 可能 mismatch
**处理**：MVP 用 "mtime 最新且最近 60s 内有写入" 启发式
**known limitation**：极少 race condition 下可能错位
**长期**：等 spec/jsonl-schema.md 确认 JSONL 是否记录 pid

详见 [addendum § A.1](../02-planning/addendum.md)。

### 6.6 磁盘 ENOSPC（写 log 失败）

**情景**：磁盘满，log file 写不进
**处理**：silently fail（不能因为 log 失败影响主功能）
**恢复**：磁盘有空间后下次 log 自然成功

### 6.7 macOS 16+ 进程枚举 API 变化

**情景**：将来 macOS 收紧 process introspection 权限
**处理**：sysinfo crate 会跟进更新，我们升级
**应急**：如果 sysinfo 太慢/坏，回退到 `/usr/bin/pgrep claude` parse stdout

---

## 7. Threat model

### 7.1 攻击面

| 输入 | 信任级别 | 验证 |
|---|---|---|
| 进程列表 (from sysinfo) | 系统信任 | 无需验证 |
| JSONL 内容 (from ~/.claude/) | 半信任 (用户写的 / Claude Code 写的) | parse 时校验 schema |
| 用户点击 / 键盘输入 | 信任 | 无需验证 |

### 7.2 已知风险

| 风险 | 严重度 | 缓解 |
|---|---|---|
| JSONL 含恶意构造的字段触发 parser panic | 低 | serde 是 safe Rust，最坏返回 Err |
| ~/.claude/ 被替换为大文件（DoS） | 极低 | 我们只读最后行，不读整文件 |
| 用户的 ~/.claude/projects/ 含敏感 token？ | 低 | 我们只读 transcript message 字段，不读 token 字段 |

### 7.3 显式没做的安全措施

- 不加签名校验 (Gatekeeper 自己做)
- 不加自我完整性校验
- 不加 anti-tampering

→ MVP 不必要。

---

## 8. Concurrency model

### 8.1 当前 (MVP)

- Tauri 主事件循环跑 macOS run loop
- `list_sessions` invoke 在 Tauri command worker thread 同步阻塞
- 多个 invoke 不并发（前端 polling 间隔 2s 远大于单次 invoke < 100ms）
- 无显式 mutex / lock（无共享状态）

### 8.2 v0.2+ 演进路径

如果 invoke 时长接近 polling 间隔（说明 session 数太多或 JSONL 太大）：
- 切 async streaming：后端 emit_event per session，前端增量 render
- 引入 tokio runtime
- 或后端缓存 session 列表，invoke 只返回 diff

---

## 9. Logging strategy

### 9.1 Log 路径

`~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`

详见 [F1 acceptance](../../product/user-stories.md#f1--jsonl-损坏读不到)。

### 9.2 Log level

| Level | 触发 |
|---|---|
| ERROR | JSONL parse 失败、sysinfo error、tray 创建失败 |
| WARN | session 配对启发式 fallback、JSONL 文件不存在 |
| INFO | app 启动 / 退出、tray click |
| DEBUG | 每次 refresh tick (默认关，env var 开) |

### 9.3 Format

```
2026-05-18T14:08:30Z [INFO] startup: tray icon registered, polling started
2026-05-18T14:08:32Z [WARN] session pid=12345: JSONL parse failed, marking Unknown
```

ISO 8601 timestamp + level + 上下文 + message。

### 9.4 Rotation

- MVP: 不 rotate，单文件无限增长
- v0.2+: 按 size (10MB) 或 day rotate

→ flag 在 spec/logging.md（待写）。

---

## 10. Dependencies risk

| 依赖 | 版本 | 维护活跃度 | 替换难度 | 风险 |
|---|---|---|---|---|
| tauri | 2.x | Anthropic 没投但 community 活跃 | 高（架构性） | 中 |
| sysinfo | 0.30 | 活跃 | 中（API 类似） | 低 |
| serde | 1.x | Rust 生态基石 | 极低 | 极低 |
| Claude Code JSONL 格式 | 2.1.126 (✅ 2026-05-18 锁定) | Anthropic 控制 | N/A | **高** |

→ 最大风险是 Claude Code JSONL 格式变。[spec/jsonl-schema.md](../../spec/jsonl-schema.md) 已锁定字段假设。CI 测试加 fixture 校验是后续 task。

---

## 11. Trade-offs explicitly accepted

| Trade-off | Cost | Benefit |
|---|---|---|
| Polling vs event-driven | 浪费空 tick CPU | 调试简单、跨 fs 行为一致 |
| 同步阻塞 invoke vs streaming | 极端场景卡 | 简单 atomic |
| 无前端框架 | DOM render 啰嗦 | bundle 小、维护简单 |
| 不签名 .app | Gatekeeper 摩擦 | 0 成本 |
| 不做 launchd 守护 | 用户偶尔要手动重启 | 减少架构层级 |
| 不跨平台 | 用户 base 小 | macOS 实现极致 |
| 单文件 session.rs | 文件会变大 | MVP 阶段易读 |

---

## 12. 跟现有 UML 文档的对应关系

| 架构关切 | UML 文档 |
|---|---|
| 用户视角行为 | [UML 01 Use Case](../../design/uml/01-use-case.md) |
| 用户使用流程 | [UML 02 Activity](../../design/uml/02-activity-end-to-end.md) |
| 模块拆分 | [UML 03 Component](../../design/uml/03-component.md) |
| 代码包结构 | [UML 04 Package](../../design/uml/04-package.md) |
| 数据结构 | [UML 05 Class](../../design/uml/05-class.md) |
| 启动流程 | [UML 06 Sequence: Startup](../../design/uml/06-sequence-startup.md) |
| Refresh 轮询 | [UML 07 Sequence: Refresh](../../design/uml/07-sequence-refresh.md) |
| Tray click | [UML 08 Sequence: Tray Click](../../design/uml/08-sequence-tray-click.md) |
| Session 状态机 | [UML 09 State](../../design/uml/09-state-session.md) |
| 部署拓扑 | [UML 10 Deployment](../../design/uml/10-deployment.md) |

→ UML 是规范化建模，本文档是叙述性架构。两者一起读。

---

## 13. Implementation readiness

进入下一文档：[implementation-readiness.md](implementation-readiness.md) 做交叉验证 PRD ↔ Architecture。
