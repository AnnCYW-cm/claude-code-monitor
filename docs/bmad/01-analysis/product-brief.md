# Product Brief — Claude Code Monitor

> **BMAD Phase 1 · Analysis · Analyst output (final)**
> **Status:** Draft → ready for Phase 2 (PRD)
> **Author:** 蔡逸雯 (caiyiwen)
> **Date:** 2026-05-17
>
> 这份是 Phase 1 的收敛产物。下游：[02-planning/PRD.md](../02-planning/PRD.md) 基于此写完整需求文档。
>
> Brief 的目的是「pin down 几条关键句」——一句话价值 / 目标用户 / 核心场景 / 边界。所有后续文档不能违反 brief，违反就要回来更新这份文档。

---

## 1. One-liner

> A macOS menubar app that tells you which of your running Claude Code sessions is waiting for input.

不到 20 字。能上 Show HN / 推特 bio。

---

## 2. The problem

当你同时运行 ≥3 个 Claude Code session 时（典型重度用户日常 3-8 个），你会经常：

| 痛点 | 频率 | 后果 |
|---|---|---|
| 某个 session 跑完了在等你，你没察觉 | 高（一天数次） | 该 session 平白干等数分钟到数十分钟 |
| 忘了某个 session 在干啥 | 中 | 切回去要重读 transcript 才能继续 |
| 不知道哪个最该先处理 | 中 | 凭直觉切，常常发现切错了 |

**关键洞察**：解决方案不是「主动通知」（那会变成第二个 Slack）。是「**被动感知**」——抬眼看一下菜单栏就知道现状。

参考调研：[market-research.md § 2.3](market-research.md) 列了用户当前的 DIY workaround，全部不 zero-config。

---

## 3. Target users

### 3.1 Primary user (MVP target)

**Heavy Claude Code User**：
- 同时跑 ≥3 个 `claude` CLI session 是常态（峰值 8）
- 一天用 CC ≥4 小时
- 多项目并行
- macOS 系统
- 命令行原生派（不依赖 IDE 集成）

**估算群体规模**：3,500 - 50,000 人（见 [market-research.md § 1](market-research.md)）

### 3.2 Secondary user (将受益但不优先)

- 用 tmux/screen 持久化 session 的用户（他们已经有 workaround，但仍受益）
- 轻度 CC 用户（偶尔 3+ session 时用得上）

### 3.3 Non-users (明确不服务)

- Linux/Windows 用户（菜单栏概念不通用）
- 团队/企业用户（监控他人 session）
- API-only 用户（不用 CLI）
- 单一 session 浅度用户（用不上）

---

## 4. Value proposition

### 4.1 To the user

> 我可以并行跑更多 session，不用担心漏看任何一个。我的注意力不被打断，但我永远知道现状。

具体好处：
1. **回收浪费时间**：不再让 session 空等
2. **降低 context switching cost**：菜单栏看一下 < 切 tab 重读 transcript
3. **不增加新焦虑源**：因为它从不打扰你

### 4.2 To the open-source community

> 一个示范：怎么用 Tauri 做 macOS menubar app + 怎么 hook 进 Claude Code transcript。

这是个 reference implementation，未来其他 Claude Code 周边工具可以借鉴。

### 4.3 To the author (蔡逸雯)

战略价值：
- **首个准备开源的 dev tool 项目**，建立技术信誉
- **个人 portfolio 上 Tauri/Rust 项目**（除了已有的 Python/TS 项目）
- **跟 AI 开发者社区建立 inbound 触点**

---

## 5. Scope

### 5.1 MVP (v0.1) — In scope

1. macOS 菜单栏 app，常驻
2. 自动发现所有运行中的 `claude` CLI 进程
3. 每个进程定位它的当前 JSONL transcript
4. 读 JSONL 最后一条消息，分类成 Waiting / Working / Unknown
5. tray icon 显示 "等你" session 数（waiting=0 时仅显示图标，≥1 显示数字；详 [ux-design § 2.3](../02-planning/ux-design.md)）
6. 点击 tray 弹出 popup 列出所有 session：cwd / 状态 / 已等时长 / 最后一条消息预览
7. 点击列表项展开看完整最后一条 assistant 消息
8. 右键 tray 弹 native menu，含 Quit
9. JSONL 损坏时显示 Unknown，下一轮重试
10. log file 到 `~/Library/Logs/...`

### 5.2 Explicit non-goals (MVP 拒)

| 不做 | 理由 |
|---|---|
| 通知（toast / sound / badge） | 红线：被动感知 > 主动查询 ([R1](../../product/user-stories.md#r1--不该主动打断用户)) |
| 任何配置面板 | 红线：零配置 ([R2](../../product/user-stories.md#r2--不该要求配置才能用)) |
| Session 命名 / 标签 / 备注 | 红线：不需要管理 ([R3](../../product/user-stories.md#r3--不该需要用户记每个-session-是干嘛)) |
| 跳转到对应终端 tab | 工程量翻倍 + 跨 emulator 适配 ([R4](../../product/user-stories.md#r4--不该接管切到对应-tab)) |
| 历史已退出 session | "now not history" 心智模型 ([R5](../../product/user-stories.md#r5--不该展示历史已退出-session)) |
| 跨平台（Linux/Windows） | 见 [market-research.md § 7](market-research.md) |
| 团队/共享视图 | 非 target user |
| 进程"卡死"检测 | 误报代价高于价值 ([F2](../../product/user-stories.md#f2--claude-进程僵死)) |
| 守护进程 / 自动重启 | 设计妥协，重启成本 < 1s ([F3](../../product/user-stories.md#f3--app-自己崩了)) |

### 5.3 Deferred (v0.2+ 候选)

- 跳转 tab（如果用户强烈要求）
- 屏幕共享时 demo mode
- macOS Focus 期间策略
- 笔记本待机/唤醒主动 refresh
- fs watcher 替代 polling
- Homebrew cask 发布

---

## 6. Success metrics

### 6.1 Personal (作者 dogfood)

- 14 天连续使用不删
- 每天打开 popup ≥5 次
- 至少 3 次"靠它发现某个 session 等我已超过 5 分钟"实际事件

### 6.2 Open source (release 后)

- D14 retention（用户装上 14 天后仍在 launchctl 列表里）≥ 50%（hard to measure without telemetry，靠用户访谈）
- GitHub star ≥ 10（1 month）
- 有非作者 contributor 提 PR（3 months）

详见 [market-research.md § 6](market-research.md)。

---

## 7. Strategic context

### 7.1 为什么现在做

| 时机因素 | 解释 |
|---|---|
| Claude Code 进入广泛使用阶段 | 2026 年是 CC 用户基础起飞期 |
| 重度用户的工作流痛点开始浮现 | 个人/社区都开始 DIY workaround，没人做 polished tool |
| Tauri 2.x 成熟 | 一年前 Tauri menubar app 是 alpha，现在稳了 |
| Anthropic 还未做官方 monitor | 窗口期 |
| 作者刚完成另一个项目，有 bandwidth | 时间充足 |

### 7.2 跟作者其他项目的关系

| 项目 | 关系 |
|---|---|
| Skill book / AI 编程书 | 本项目是「书里讲的方法的实战」 |
| AI 一线观察者公众号 | 写一篇 "从 0 开源一个 dev tool" 系列 |
| AI 产品付费专栏 | 案例之一 |
| Next Life iOS App | 不同领域，无重叠 |

---

## 8. Risks summary

详见 [market-research.md § 5](market-research.md)。Top 3：

1. **Anthropic 出官方版本** → 差异化靠 anti-features（永不通知）
2. **JSONL 格式变** → spec/jsonl-schema.md 锁定字段 + CI 测试
3. **单作者 burnout** → 文档先行降低 contributor 门槛

---

## 9. Open questions（带入 PRD 解决）

1. **Waiting/Working 块内排序**（特别是 working session 间）—— H2 已 flag
2. **JSONL 时长起点字段**—— H2 已 flag，等 spec/jsonl-schema.md 实测
3. **macOS 12/14/15 Gatekeeper UX**—— F4 已 flag，**作者待办**实测
4. **是否做 launchd 守护**—— F3 已 flag，MVP 拒，v0.2+ 看反馈
5. **开源后维护承诺频率**—— market-research.md 建议每周末 office hour，待 README 落地时确认

---

## 10. Sign-off

| 角色 | 状态 |
|---|---|
| Analyst | ✅ Brief 已收敛 |
| PM（待） | ⏳ 进入 [PRD.md](../02-planning/PRD.md) |
| Architect（待） | ⏳ 进入 [architecture.md](../03-solutioning/architecture.md) |

→ Phase 1 完成。下游开始。
