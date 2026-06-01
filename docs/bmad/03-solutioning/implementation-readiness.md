# Implementation Readiness Check

> **BMAD Phase 3 · Solutioning · Architect output**
> **Status:** Draft → 通过即可进入 epics/stories 切分
>
> 这份文档是 PRD ↔ Architecture 的交叉验证 checklist。每个 FR / NFR 都要找到对应实现路径，每个 architecture 决策都要找到对应需求依据。
> 通过标准：所有 ✅，无 ❓未决。

---

## 1. FR ↔ Architecture coverage

### 1.1 Core monitoring loop

| FR | Architecture 实现 | 状态 |
|---|---|---|
| FR-1 枚举 claude 进程 | `session.rs::list()` 用 sysinfo refresh_processes() + filter name | ✅ |
| FR-2 定位 JSONL transcript | `session.rs::list()` encode cwd → 查 ~/.claude/projects/<encoded>/ → 取 mtime 最新 .jsonl | ✅ |
| FR-3 读 JSONL 最后一行 | `session.rs` seek to EOF + reverse scan to `\n` | ✅ |
| FR-4 分类 Waiting/Working/Unknown | `session.rs::classify(jsonl_last)` 按 [UML 09 State](../../design/uml/09-state-session.md) | ✅ |
| FR-5 每 2s 重复 | 前端 setInterval 2s 调 list_sessions | ✅ |
| FR-6 进程退出移除 | sysinfo 不再列 → list_sessions 不返回它 → 前端列表自动不显示 | ✅ |

### 1.2 Menubar UI

| FR | Architecture 实现 | 状态 |
|---|---|---|
| FR-7 tray icon 常驻 | `lib.rs` setup 注册 TrayIconBuilder + activation_policy=Accessory | ✅ |
| FR-8 显示 waiting 数 | `list_sessions` 末尾 count waiting → `tray.set_title()` | ✅ |
| FR-9 tray 和列表数字一致 | 同一 `list_sessions` invoke 末尾计算，原子更新 | ✅ |
| FR-10 左键点 tray 弹 popup | `on_tray_icon_event` 匹配 Click(Left, Up) → window.show() | ✅ |
| FR-11 右键弹 native menu | `menu_on_left_click(false)`，默认右键弹 menu | ✅ |
| FR-12 列表显示 4 字段 | 前端 render，CSS 见 [ux-design § 5](../02-planning/ux-design.md) | ✅ |
| FR-13 按 waiting 时长降序排序 | 前端 / 后端任一处排序——MVP 后端 sort 后返回 | ✅ |
| FR-14 点列表项展开消息 | 前端 click handler 切换 expanded state | ✅ |
| FR-15 最多 1 项展开 | 前端 state 维护 `expandedPid: u32 \| null` | ✅ |
| FR-16 empty state | 前端 render：sessions.length === 0 → empty | ✅ |

### 1.3 Robustness

| FR | Architecture 实现 | 状态 |
|---|---|---|
| FR-17 JSONL parse 失败 = Unknown | `session.rs::classify(None)` 返回 Unknown，下一轮重试 | ✅ |
| FR-18 启动竞态 = Unknown | 同上：JSONL 不存在 → None → Unknown | ✅ |
| FR-19 log file 路径 | `log` crate 初始化时写 ~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log | ✅ |

### 1.4 Negative requirements

| FR | 实现保证（怎么确保不会被加进去） |
|---|---|
| FR-N1 不通知 | 不引入 `notification` / `osascript` 调用；CI grep 阻断这些关键字 |
| FR-N2 不配置 | 不实现 settings/preferences UI；不读 ~/Library/Preferences |
| FR-N3 不命名/标签 | 不持久化 session metadata；session 只是内存中的 Vec |
| FR-N4 不跳转 tab | 不引入 osascript / iTerm Python API / AppleScript |
| FR-N5 不显示历史 | session 仅来自当前运行的 process；不读 historical JSONL |

→ 这些是"不实现"，靠 PR review 把关。建议加 CI 规则（grep 禁用关键字）。

---

## 2. NFR ↔ Architecture coverage

### 2.1 Performance

| NFR | 设计保证 | 验证方式 |
|---|---|---|
| NFR-P1 `list_sessions` < 50ms (10 session) | 见 [architecture § 5.1 budget](architecture.md) | benchmark in CI |
| NFR-P2 popup < 200ms | 预创建 window + hide/show toggle，零创建开销 | 实测 |
| NFR-P3 启动后首扫 < 2s | 启动后立即触发首次 invoke（不等 2s tick） | 实测 |
| NFR-P4 24h RSS < 50MB | 无累积状态，每轮新 Vec<Session>，无 leak | 24h dogfood 观察 |
| NFR-P5 空闲 CPU < 0.5% | 2s polling + 单次 < 50ms → < 50/2000 = 2.5%——超 budget? | **❗待验证：实测看是否过高** |

❗ **风险**: NFR-P5 数学上算下来可能超 budget（2.5% > 0.5%）。需要：
- 实测真实 CPU usage（sysinfo refresh 不一定全程占 50ms CPU，多数是 syscall wait）
- 如果超 budget，降 polling 频率到 5s 或后端 cache 进程列表

→ **加入 epic 1 实测验收**。

### 2.2 Compatibility

| NFR | 设计保证 |
|---|---|
| NFR-C1 macOS 12+ | Tauri 2.x min macOS 11；我们设 12+ 留余 |
| NFR-C2 Apple Silicon + Intel | Rust universal binary，cargo build --target 双架构 |
| NFR-C3 跨终端 emulator | 只看 OS 进程表，不依赖任何 emulator API |
| NFR-C4 Claude Code 版本 | spec/jsonl-schema.md 锁定字段假设 |

### 2.3 Reliability

| NFR | 设计保证 |
|---|---|
| NFR-R1 单 session 失败不影响其他 | session.rs loop per-process，单个 panic 用 catch_unwind 隔离 | ⚠️ |
| NFR-R2 重启恢复全状态 | 无持久化状态，重启即重扫 | ✅ |
| NFR-R3 crash 率 < 1% | 靠测试 + dogfood，无静态保证 | ⏳ |

⚠️ **未决**: NFR-R1 需要 panic isolation。MVP 实现需明确：每个 session 处理用 `std::panic::catch_unwind` 包起来。
→ **正式实现在 [S-011 (epic 3)](epics/story-011-error-handling.md)**。但 epic 1 的 [S-005 list_sessions](epics/story-005-list-sessions-command.md) 应该**临时**用简单 Result 兜底（不靠 catch_unwind），等 S-011 才换全面 panic isolation。

### 2.4 Security & privacy

| NFR | 设计保证 | 状态 |
|---|---|---|
| NFR-S1 不访问网络 | 不引入 reqwest/hyper 等网络 crate；CI grep 阻断 | ✅ |
| NFR-S2 不读 ~/.claude/ 以外 | 代码里硬编码 ~/.claude/ 路径 | ✅ |
| NFR-S3 不读敏感字段 | JsonlMessage struct 只 deserialize 我们需要的字段 (role + content)，token 字段不 parse | ✅ |
| NFR-S4 屏幕共享时隐藏预览 | MVP 不做 | OK (deferred) |

### 2.5 Maintainability

| NFR | 设计保证 |
|---|---|
| NFR-M1 docs/ 比代码先成熟 | 当前状态：文档密度 >> 代码密度 ✅ |
| NFR-M2 第一个 PR 一晚上上手 | 看 docs/ 流程 → architecture → uml → stories | ⏳ |
| NFR-M3 log 详尽 | architecture § 9 定 log format | ✅ |

### 2.6 Distribution

| NFR | 设计保证 |
|---|---|
| NFR-D1 DMG < 15MB | Tauri release build 一般 ~10MB | ✅ |
| NFR-D2 安装 ≤ 3 步 | DMG drag-to-Applications + Gatekeeper bypass + open | ✅ |
| NFR-D3 启动到 tray < 3s | Tauri cold start ~1s + setup ~500ms | ✅ |

---

## 3. Architecture decisions ↔ requirement coverage

每条 ADR 必须服务于至少一个 FR/NFR 或产品红线：

| ADR | 服务于 |
|---|---|
| 001 Tauri 2.x | NFR-D1 (bundle 小) + NFR-M2 (VS Code 友好) |
| 002 Polling 不 fs watcher | 简化复杂度（NFR-M2），可接受的延迟 |
| 003 不跳转 tab | NFR-C3 (跨 emulator) + brief MVP 范围 |
| 004 永不通知 | R1 红线 |
| 005 用 JSONL 不 hook | NFR-S1 (本地) + 不依赖 Claude Code SDK |
| 006 无前端框架 | NFR-D1 (bundle 小) |
| 007 同步阻塞 invoke | NFR-P1 < 100ms 够用 |
| 008 左键 popup 右键 menu | UX 惯例 |
| 009 MIT license | 战略 |
| 010 docs 命名约定 | NFR-M1/M2 |
| 011 BMAD/Spec Kit 放 docs/ | NFR-M1 |
| 012 时长格式 | UX 一致性 |

→ 所有 ADR 都有 owner。

---

## 4. Outstanding open questions

| ID | 问题 | Owner | Blocking? |
|---|---|---|---|
| OQ-1 | JSONL 实际字段名 / 时间戳格式 | Architect (spec/jsonl-schema.md) | ❗ 阻塞 epic 1 |
| OQ-2 | 同 cwd 多 session 配对策略 | 同 OQ-1 | ❗ 阻塞 epic 1 |
| OQ-3 | macOS 12/14/15 Gatekeeper 步骤 | 作者实测 (F4) | ❗ 阻塞 release |
| OQ-4 | CPU usage 实测 (NFR-P5) | epic 1 实施时 | ⚠️ |
| OQ-5 | Working session 间排序 | UX 反馈 | OK (deferred to feedback) |
| OQ-6 | demo mode 触发方式 | v0.2+ ADR | OK (deferred) |
| OQ-7 | launchd 守护要不要做 | v0.2+ 反馈 | OK (deferred) |
| OQ-8 | popup 失焦自动 hide | v0.2+ | OK (deferred) |
| OQ-9 | popup 锚定到 tray rect | v0.2+ | OK (deferred) |

→ **3 个 ❗阻塞项必须在进入 epic 1 实施前解决**：
- OQ-1, OQ-2 → 实测 JSONL 然后写 spec/jsonl-schema.md
- OQ-3 → 作者实测 macOS 各版本 Gatekeeper UX

→ **1 个 ⚠️验证项**在实施过程中验证：
- OQ-4 → epic 1 CPU benchmark 步骤

---

## 5. Risks not covered by architecture

| Risk | 来源 | 缓解 |
|---|---|---|
| Anthropic 出官方 monitor | brief § 5.1 | anti-features 差异化 |
| 作者 burnout | brief § 8 | 文档先行降低 contributor 门槛 |
| Claude Code JSONL 格式 breaking | brief § 8 | spec/jsonl-schema.md + CI 测试 |
| Gatekeeper 越来越严 | F4 | 长期考虑申请 Apple Developer Program 签名 |

这些都是产品/战略风险，不是架构能解决的。flag 在这里以便 release 时再 review。

---

## 6. Readiness verdict

### 6.1 Can we start epic 1?

| 检查项 | 通过? |
|---|---|
| 所有 FR 有对应实现路径 | ✅ |
| 所有 NFR 设计上可达 | ✅（NFR-P5 待实测） |
| 关键风险有缓解策略 | ✅ |
| Open questions 已标 owner | ✅ |
| 阻塞项已识别 | ✅（3 个） |

### 6.2 Conditions to enter implementation

**必须先做**:
1. 实测 ~/.claude/projects/*/*.jsonl 实际格式 → 写 spec/jsonl-schema.md
2. 解决 OQ-1, OQ-2
3. 作者实测 macOS Gatekeeper 各版本步骤 → 写 install.md

**实施中验证**:
- OQ-4: CPU usage 实测，超 budget 调 polling 频率
- NFR-R1: 加 panic isolation per session

### 6.3 Sign-off

| 角色 | 状态 |
|---|---|
| Architect | ⚠️ **条件性通过** — 设计 OK，但 3 个阻塞 OQ 必须先解决，否则不能进入 implementation |
| PM | ⏳ 等 review |
| 作者 | ⏳ 等阻塞项动手（OQ-1/2 实测 JSONL + OQ-3 实测 Gatekeeper） |

→ **下一步**：[epics/](epics/) — 切 dev story 和 epic
