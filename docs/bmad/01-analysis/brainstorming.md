# Brainstorming — Claude Code Monitor

> **BMAD Phase 1 · Analysis · Analyst output**
> **Status:** Draft (single-author solo session, 2026-05-17)
>
> 这份文档是项目启动期的散开式思考记录。BMAD 流程里的 brainstorming 通常是团队 facilitated session 的产物；这里改造为「单作者结构化自问」格式：列出我能想到的所有相关方向，再收敛。
>
> 目的不是「定方案」，而是「不要漏掉视角」。后续 PRD / architecture / story 都从这里收敛。

---

## 0. 起点问题

**给定**：作者（蔡逸雯）是重度 Claude Code 用户。日常同时运行 3-8 个 `claude` CLI session 处理不同任务。

**痛点**：
- 切到 A session 写代码时，B session（在跑测试）完成了在等输入——我察觉不到
- 忘了某个 session 在干什么，切回去要重读 transcript 才能继续
- 不知道哪个最该先处理

**问题**：怎样不打扰我的当下工作流，又能让我「需要时一眼可见」每个 session 的状态？

---

## 1. Divergent — 所有可能的方向

### 1.1 形态维度

| 候选 | 优点 | 缺点 |
|---|---|---|
| **菜单栏 app** | 抬眼可见、不抢焦点、macOS 原生位置 | 只在 Mac 上工作 |
| 终端内 split pane | 跟当前 workflow 同地点 | 占终端空间、要每个 session 都配 |
| 浏览器 dashboard | 跨平台、可远程 | 要主动切窗口看，违反「被动感知」 |
| Notification center 推送 | 立即触达 | 打断、跟 R1 "不打断" 红线冲突 |
| iOS/Apple Watch 推送 | 离开屏幕也能知道 | 太重，over-engineering for v0.1 |
| tmux status bar 集成 | tmux 用户原生 | tmux 用户子集太小 |
| VS Code extension | 跟 IDE 集成 | 不在 IDE 工作时失效 |
| Stream Deck 集成 | 物理设备反馈 | 设备小众 |

→ **菜单栏 app 胜出**。被动感知 + macOS 原生位置 + 跨终端 emulator 中立（不绑定 iTerm/Terminal/Ghostty 任何一个）。

### 1.2 监控数据源维度

| 数据源 | 优点 | 缺点 |
|---|---|---|
| **JSONL transcript** (`~/.claude/projects/...`) | source of truth、Claude Code 自己写的 | 格式可能变（Claude Code 自身演进） |
| Hook 进 Claude Code 的 lifecycle | 实时、零延迟 | 需要 Claude Code 暴露 hook API，目前未知 |
| 屏幕内容 OCR | 100% 通用 | 重、不稳、隐私敏感 |
| 进程的 stdout/stderr pipe | 跟得上每个 token | 要拦截已运行进程，技术门槛高 |
| pty 录制 | 完整保真 | 跟 JSONL 重复 |

→ **JSONL transcript 胜出**。已经存在、稳定写入、不需要任何特权操作。

### 1.3 状态分类维度

最少需要分几种状态？

| 状态 | 含义 | 信号源 |
|---|---|---|
| **Waiting** | Claude 完了在等我输入 | last role == assistant 且无 pending tool_use |
| **Working** | Claude 在思考/工具调用 | 其他 |
| **Unknown** | JSONL 读不到 | parse 失败 / 刚启动还没写 |
| ~~Stalled~~ | 进程在但 JSONL 长时间没更新 | mtime 阈值（**MVP 不做**：复杂任务 thinking 也可能 5min+ 不写） |
| ~~Errored~~ | Claude 进入错误状态 | 需要识别错误模式（**MVP 不做**） |
| ~~Idle~~ | 用户主动暂停 | Claude Code 没"暂停"概念 |

→ MVP 三态：Waiting / Working / Unknown。

### 1.4 UI 信息密度维度

popup 里每条 session 显示什么？

| 字段候选 | 价值 | 成本 |
|---|---|---|
| cwd 末段名 | **必需**——识别项目 | 0 |
| 状态（waiting/working/unknown） | **必需** | 0 |
| 已等时长（waiting 用） | 决定先处理谁 | 需要追踪状态进入时间 |
| 最后一条 assistant 消息 | **必需**——决定是否要切走 | 需要 JSONL parse |
| PID | 同 cwd 多 session 时区分 | 低 |
| 启动时间 | 历史定位 | 低 |
| token 用量 | 成本意识 | 需要 Claude Code 暴露或者 OCR |
| 工具调用次数 | 复杂度信号 | 中 |
| ~~评分/标签~~ | 用户管理 | 违反 R3 "不需要管理" |

→ MVP 显示前 4 项 + 同 cwd 多 session 时附加 PID 或启动时间。

### 1.5 触发机制维度

何时刷新状态？

| 机制 | 优点 | 缺点 |
|---|---|---|
| **前端 setInterval 2s 轮询** | 简单可控、跟窗口可见性自然绑定 | 浪费空 tick |
| 后端 fs watcher (notify crate) | 事件驱动、零浪费 | macOS 对 JSONL append 事件有 quirk（合并/丢失） |
| 后端长定时器 + push to FE | 一致性强 | 调试复杂 |
| 用户手动按 refresh | 完全可控 | 违反"被动感知" |

→ **MVP 选 setInterval**。简单先行，v0.2 再考虑 fs watcher。

---

## 2. Converging — 涌现的主题

经过上面的散开，几个核心 theme 浮上来：

### Theme A：被动感知 > 主动查询
所有候选里，"不打断" 是反复出现的约束。一旦做通知/推送/抢焦点，这个产品就变成了 Slack/邮件——而 Slack/邮件正是用户想逃避的对象。

→ 这变成产品红线（[user-stories R1](../../product/user-stories.md)）。

### Theme B：跨终端 emulator 中立
作者用 iTerm、同事用 Terminal.app、明天可能换 Ghostty——这个工具不能绑定任何一个。

→ 这要求 app **只看 OS 进程表 + 文件系统**，不集成任何终端 API。

→ 这又导致砍掉「点击跳转到对应 tab」（要适配 N 个 emulator，工程量爆炸）。

### Theme C：用户已经习惯的能力不接管
Cmd+Tab 切窗口、Mission Control 找进程、scrollback 翻历史——这些用户都熟。我不要"做得更好"而是"不重复做"。

→ 让用户自己用 Cmd+Tab 切走，app 只负责"提示要不要切"。

### Theme D：MVP 是文档驱动开发
作者本人就是 Claude Code 重度用户，目标受众跟自己重合，需求验证成本极低。
但项目要开源——所以文档体系比代码本身先成熟，避免后期 contributor 看不懂。

→ 进入 BMAD 流程，先把 PRD / architecture / story 全写透。

---

## 3. Key insights

1. **「砍掉跳转 tab」是这个项目能 MVP 化的关键决策**。它换来了跨终端 emulator 中立和工程量可控。这个 trade-off 必须在 PRD 里明确论证，否则后续 contributor 一定会想加。

2. **「等你时长 UI」是 power user 的核心信号**。普通用户「有就行」，但作者这种 8 个并行 session 的用户，必须看到时长才能分诊。所以 H2 acceptance 把时长定为必须，不是 nice-to-have。

3. **「不做通知」反直觉但正确**。99% 的同类工具会做通知（"toast/badge/sound"），但所有作者用过的通知工具最后都会被关掉——所以它们的实际价值≈0。被动感知（抬眼看一下）是更稳的设计。

4. **JSONL 格式是隐藏依赖**。我们整个监控建立在 Claude Code 写的 JSONL 之上。Claude Code 改格式我们就坏。这个风险要在 architecture 里 flag，并准备一份 `spec/jsonl-schema.md` 锁定我们依赖的字段。

5. **「单一开发者 + 公开开源」是双重身份**。私下我快速迭代不需要文档；公开我必须有文档承担 contributor 入门成本。所以 BMAD 流程对这个项目特别合适——它强制产物。

---

## 4. Things to verify（待 Phase 2-3 解决）

| 待验证 | 验证方式 | 阶段 |
|---|---|---|
| Claude Code JSONL 实际格式 | `cat ~/.claude/projects/<encoded>/*.jsonl` | architecture |
| sysinfo crate 0.30 在 macOS 14/15 的进程枚举性能 | benchmark | architecture |
| Tauri 2.x tray_icon 在 macOS 15 的稳定性 | hello-world prototype | architecture |
| WKWebView 弹窗在 macOS Stage Manager 下的位置行为 | 实测 | ux-design |
| 2s 轮询的 CPU/电池开销 | Activity Monitor + powermetrics | architecture |
| "Heavy CC user" 群体规模（市场） | 群里调研 / 推特 poll | market-research |

---

## 5. Next steps

按 BMAD Phase 顺序：

1. → [market-research.md](market-research.md) — 验证市场需求 + 竞品扫描
2. → [product-brief.md](product-brief.md) — 把这些散开的 idea 收敛成产品定义
3. → [02-planning/PRD.md](../02-planning/PRD.md) — 把 brief 展开成完整需求
4. → [02-planning/ux-design.md](../02-planning/ux-design.md) — UI/UX 细节
5. → [03-solutioning/architecture.md](../03-solutioning/architecture.md) — 技术架构
6. → [03-solutioning/epics/](../03-solutioning/epics/) — 切 dev stories

---

## 附：放弃的方向（写出来防止反复想）

- **付费版**：MVP 阶段不考虑商业化。这是个 utility 不是 SaaS。
- **团队协作 / 共享视图**：用户群不需要看别人的 session。
- **历史回放**：违反 R5 "now not history"。
- **iOS companion app**：违反"被动感知"——掏手机看本身就是打断。
- **AI insight**："你今天有 N 个 session 超过 1 小时没回" 这种自动洞察——增加注意力负担，违反 L2 "fade into background"。
- **自定义主题/皮肤**：违反 R2 "零配置"。
