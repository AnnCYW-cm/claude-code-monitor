# User Stories

> **Status:** Draft
>
> 25 条用户故事，覆盖 happy / edge / failure / reverse / longitudinal / adversarial 六类。
>
> **格式**：`As a [user], I want to [action], so that [benefit]` + acceptance criteria + notes。
> **用户角色**：统一为「Heavy CC User」（重度 Claude Code 用户，并行 ≥3 个 session 是常态），不在每条重复。
> **ID 编号**：`<类别字母><序号>`，如 `H1`、`E3`、`R5`。后续文档（[scenarios.md](scenarios.md)、PRD、test plan）引用故事用 ID。
>
> 凡是标 **MVP scope: out** 的，说明这条故事不在 v0.1 实现范围，但**思考要在文档里留痕**，避免未来争论"为啥不做"。

---

## H · Happy Path（常规、高频）

### H1 — 瞄一眼判断是否切走

**As a** heavy CC user
**I want to** glance at the menubar to see if any session is waiting for me
**so that** I can decide whether to interrupt my current work or keep going

**Acceptance criteria**
- 菜单栏 tray icon 始终展示 waiting 数量
- 视觉扫描 < 0.5s 内能完成判断（数字够大、对比度够高）
- tray 数字与列表内部计数永远一致（不会出现 tray 显 2、列表显 3）
- waiting=0 时的显示状态待 UI 决策（候选：显示 0 / 显示纯图标 / 灰化图标）

**Notes**
- 产品最高频用例（一天可能发生几十次）
- 跟产品定义 v0.2 的"被动感知 > 主动查询"核心价值直接对应

---

### H2 — 弹开列表分诊优先级

**As a** heavy CC user
**I want to** open the popup to see which sessions are waiting and their context
**so that** I can decide which one to handle first

**Acceptance criteria**
- 点击 tray 在 < 200ms 内弹出窗口
- 列表按 waiting 时长降序排序（等得越久越靠上）
- 每条 session 至少显示：cwd 末段名 / 状态 / **已等时长**（如 `waiting · 3min`，仅 waiting 状态显示）/ 最后一条消息前若干字
- 用户能在 2 秒内判断"该先处理哪个"

**Open question**
- 排序逻辑：waiting 时长 vs 字母序 vs 用户自定义？MVP 默认 waiting 时长降序；自定义 = R3 砍掉
- 已等时长精度：分钟级足够（< 1min 显示 "just now"），秒级精度无意义且产生视觉抖动
- **时长起点**：应为 JSONL 最后一条 assistant message 的 timestamp（不是 app 第一次发现 waiting 的时间——app 可能晚启动）。待 `spec/jsonl-schema.md`（待写）确认 JSONL 时间戳字段
- **Waiting/Working 块内排序**：waiting 优先于 working 已隐含；waiting 之间按时长降序已定；working 之间未定（候选：按进程启动时间 / 按 cwd 字母序 / 按枚举顺序）。MVP 默认按进程枚举顺序（不刻意排），等用户反馈再调整

---

### H3 — 不切走也能读到关键信息

**As a** heavy CC user
**I want to** read a session's last assistant message in full from the popup
**so that** I can decide whether it actually needs my action or can wait

**Acceptance criteria**
- 列表项点击后展开显示完整最后一条 assistant 消息（不截断）
- 包含 `tool_use` 块时也要可读：**MVP** 显示 tool 名 + 参数概要文本（如 `[Bash] git status` 或 `[Read] /path/to/file.rs:42`，单行）；**v0.2+** 按工具类型完整格式化（多行、语法高亮等）
- 弹窗在展开后高度自适应或可滚动
- **收起逻辑**：再次点击同一项收起；点击其他项时收起当前并展开新项（同一时刻最多 1 项展开）；关闭 popup 重置所有展开态
- 用户能不切 tab 就完成"判断需要我吗"

**Notes**
- 这条是 v0.2 砍掉"跳转到对应 tab"后的核心补偿——降级方案
- 没了这条，砍掉跳转就失去了价值

---

### H4 — 退出 app

**As a** heavy CC user
**I want to** quit the app explicitly
**so that** I can free menubar space when I'm not coding

**Acceptance criteria**
- 右键 tray icon 弹出 native menu，含 Quit 项
- 选中 Quit 后 app 完整退出，无残留进程
- 不会丢失任何用户数据（因为本来就不保留状态）

**Notes**
- 没有"暂停 / 隐藏"中间状态——MVP 不做。要么开要么关。

---

## E · Edge Case（边界、低频但要 cover）

### E1 — 首次安装空状态

**As a** new user who just installed the app
**I want to** see something meaningful when no claude sessions are running
**so that** I understand the app works and what it'll do once I open a session

**Acceptance criteria**
- tray icon 出现在菜单栏（不会因为 0 session 就隐藏）
- 弹开列表显示 empty state，文案"no claude sessions running"
- 可选：empty state 加一行 hint "start a session with `claude` in your terminal"

**Notes**
- 用户决策"app 装对了"全靠这一刻——参考 [scenarios.md § S4](scenarios.md)

---

### E2 — 开机时已有 N 个 session 在跑

**As a** heavy CC user who reboots my laptop
**I want to** see all currently running claude sessions immediately when the app starts
**so that** I don't lose context after a reboot

**Acceptance criteria**
- app 启动后 2s 内完成首次扫描（不能让用户对空界面困惑）
- claude 进程被 shell 通过 nohup / tmux / screen 拉起的也能识别
- 历史 JSONL（进程已退出的）不显示
- **启动竞态**：如果 claude 进程刚启动 JSONL 还没建好，session 仍出现在列表里 `status=Unknown`，下一轮 refresh 大概率分类完成（对应 [UML 09 State 的 Unknown 转移](../design/uml/09-state-session.md)）

**Notes**
- 真实场景：用户用 nohup / tmux / screen 等方式持久化运行 claude，电脑重启后这些 claude 进程可能仍在跑（取决于 shell 配置和持久化插件如 tmux-resurrect）。monitor 启动后应立即把它们枚举出来。

---

### E3 — 仅 1 个 session 在跑

**As a** user who only has 1 claude session running
**I want to** still get value from the app
**so that** my install isn't wasted in low-load periods

**Acceptance criteria**
- 1 session waiting 时，tray 仍显示数字"1"（不要变图标了事）
- 列表仍可弹开，UX 不退化
- 不会因为 ≤1 session 就隐藏自己

**Notes**
- 反直觉：app 价值不只在多 session 时——单 session "长跑没察觉"也是核心痛点

---

### E4 — 极多 session（>10）

**As a** super-heavy user with 10+ sessions
**I want to** still be able to navigate the list comfortably
**so that** the tool doesn't break down at my actual peak load

**Acceptance criteria**
- 列表超过 **8 条**后开启滚动（基于 480pt popup max - 40pt header = 440pt 可用 ÷ 54pt/item ≈ 8.1 items 容量；详 [ux-design § 3.1](../bmad/02-planning/ux-design.md) + [design/ui/popup-window.md § 5.1](../design/ui/popup-window.md)）
- tray 数字正确显示两位数，不被字符宽度截断
- 性能：`list_sessions` invoke < 100ms 即使 15 session（基线见 [UML 07 Refresh](../design/uml/07-sequence-refresh.md)，10 session 内 < 50ms 是 happy path 目标）

**Notes**
- 8 session 是当前作者峰值，15 是设计 headroom
- 跟下文 L3 互锁

---

### E5 — 同一 cwd 多个 session

**As a** user who runs two `claude` instances in the same directory
**I want to** see them as separate entries in the list
**so that** I don't get confused about which one is which

**Acceptance criteria**
- 列表按 pid 区分，不按 cwd 合并
- 显示上的区分手段：cwd 同名时附加 pid 或启动时间作为副标题
- 各自的 last_message 互不污染

**Open question**
- 进程 → JSONL 配对策略：cwd + 启动时间最匹配？cwd + 进程 pid 写入 JSONL 元数据？需要 `spec/jsonl-schema.md`（待写）实地考察后确定

---

### E6 — session 退出瞬间

**As a** user looking at the popup when a session's process exits
**I want to** see it smoothly disappear from the list
**so that** the UI doesn't flicker or jump distractingly

**Acceptance criteria**
- 退出的 session 从列表移除（不留尸）
- 移除有过渡（淡出 / 折叠）——MVP 允许瞬移，过渡是 nice-to-have
- waiting 数字同步减小，与列表一致

**Notes**
- 平滑过渡是 polish，不是 correctness——MVP 功能正确优先

---

## F · Failure（异常路径，用户视角）

### F1 — JSONL 损坏读不到

**As a** user whose session's JSONL got corrupted (truncated, disk full, etc)
**I want to** still see the session in the list with a clear "unknown" state
**so that** I don't lose awareness of a running session due to a parse failure

**Acceptance criteria**
- parse 失败时 status = Unknown（不让 session 消失）
- last_message 显示 "(unable to read transcript)" 或类似 fallback 文案
- 下一轮 refresh 自动重试，不需要用户干预
- 日志记录原因便于 debug：路径约定 `~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`（macOS 标准位置）

**Open question**
- log 路径具体格式 / 滚动策略 / 最大体积，待 `spec/logging.md`（待写）阶段确定

**Notes**
- 核心价值：app 不会让一个 session "凭空消失"。消失只能因为进程退出。

---

### F2 — claude 进程僵死

**As a** user whose claude session is stuck (process alive but not writing to JSONL)
**I want to** somehow notice this state
**so that** I don't wait forever thinking it's still thinking

**Acceptance criteria（分版本）**
- **MVP**：状态保持 Working 不变（无法区分"在 thinking"和"卡死"）
- **v0.2+**：JSONL mtime 超过 X 分钟未更新时，可显示 "stalled?" 提示

**MVP scope: out**

**Notes**
- 检测"卡死"是开放问题：Claude thinking 也可能 5min+ 没输出（复杂任务）。阈值选不好会误报
- 在文档里 flag——避免未来争论"为啥不识别卡死的"

---

### F3 — app 自己崩了

**As a** user whose monitor app process died unexpectedly
**I want to** know my app crashed
**so that** I can restart it rather than working blindly thinking it's still watching

**Acceptance criteria**
- MVP：菜单栏图标消失 = 用户发现 app 死了的唯一信号
- 用户重启方式：打开 ClaudeCodeMonitor.app
- 不做 launchd 守护进程（v0.2+ 考虑）
- 文档（README + `guides/install.md`）明确说明这一点

**Notes**
- 这是设计妥协。重度用户应该愿意"挂菜单栏看看图标在不在" < 1s 成本，比维护守护进程值得
- 但要 flag——某些用户可能希望 launchd 自动重启

---

### F4 — Gatekeeper 拦截

**As a** first-time user opening an unsigned .app
**I want to** know how to bypass macOS Gatekeeper warning
**so that** I can actually run the app

**Acceptance criteria**
- `guides/install.md` 有专门小节，含截图（理想）或 step-by-step（最少）
- 三种打开方式都覆盖：DMG / Homebrew cask / 从 source build
- README 在 Quick Start 部分链接到 install.md
- install.md 内的步骤准确——macOS 12 / 14 / 15 的 Gatekeeper UX 差异较大（特别是 15 / Sequoia 加强了 Gatekeeper），各版本需实测

**Author TODO**
- **作者待办**：macOS 12 / 14 / 15 各版本实测 Gatekeeper UX，校准 install.md 步骤——不要凭印象写

**Notes**
- 开源项目无签名是默认状态。等有付费 / 赞助机制后再考虑签名/公证
- 参考 [scenarios.md § S4](scenarios.md) 里 T+0:14 的 onboarding 摩擦

---

## R · Reverse（反向用例——不该做什么）

### R1 — 不该主动打断用户

**As a** user deeply focused on coding
**I want the app to never interrupt me proactively**
**so that** my flow isn't broken

**What this rules out**
- 不弹通知（macOS notification center）
- 不响声音
- 不抢焦点（弹窗 / 模态 / 强制聚焦）
- 不闪烁 / 抖动 tray icon

**Why**
- 产品定义 v0.2 核心价值：被动感知 > 主动查询
- 一旦开始打断，就跟 Slack / 邮件没区别——而 Slack/邮件正是用户想逃避的对象

**Notes**
- 这条是"产品红线"——任何未来 PR 想加通知功能，都要先在 ADR 里推翻这条 story

---

### R2 — 不该要求配置才能用

**As a** user who just installed the app
**I want to** use it immediately without any setup
**so that** the time-to-value is sub-30 seconds

**What this rules out**
- 不登录账号
- 不配 API key / token
- 不选监控目录（自动用 `~/.claude/projects/`）
- 不设阈值 / 偏好 / 主题
- 不创建数据库 / index

**Acceptance criteria**
- 装完打开即用，0 个 setup step
- 配置面板根本不存在（避免被未来开发者诱惑打开）

**Notes**
- 跟 [scenarios.md § S4](scenarios.md) 的 30 分钟 onboarding 验证互锁

---

### R3 — 不该需要用户记每个 session 是干嘛

**As a** user with 5+ parallel sessions
**I want to** identify each session at a glance without remembering what I started them for
**so that** the cognitive load stays low

**Acceptance criteria**
- 每个列表项的 cwd 末段名是主要身份信号（项目目录名通常 = 项目身份）
- 最后一条 assistant 消息提供任务上下文

**What this rules out**
- 不实现 session 命名功能
- 不实现 session 标签 / 分组
- 不实现 session 笔记

**Notes**
- 跟 R2 一致：所有"管理 session"的功能都砍——管理 = 配置 = 摩擦

---

### R4 — 不该接管"切到对应 tab"

**As a** user who needs to respond to a session
**I want to use my normal terminal switching workflow (Cmd+Tab, Mission Control)**
**so that** the monitor app doesn't fight with my muscle memory

**What this rules out**
- 不实现"点击列表项跳转到对应终端 tab"
- 不集成任何终端 emulator API
- 不依赖窗口标题约定

**Why**
- 产品定义 v0.2 明确砍：跨终端 emulator 适配代价高、用户已有 Cmd+Tab 习惯
- 这条节省的工程量是项目能 MVP 化的关键

**Notes**
- 也对应 [UML 10 Deployment](../design/uml/10-deployment.md) 的"跨终端 emulator 中立"红利

---

### R5 — 不该展示历史已退出 session

**As a** user
**I want the list to only show currently running sessions**
**so that** the UI reflects "now" not "history"

**What this rules out**
- 不显示历史 JSONL 对应的已退出 session
- 不做"5 分钟前等过你"的历史回放
- 不做 session 搜索 / 时间线

**Why**
- "now" 是产品的核心心智模型——一份 dashboard 而不是一份日志

---

## L · Longitudinal（时间维度，使用模式演化）

### L1 — 新手期（第 1 周）

**As a** new user in week 1
**I want to** develop the habit of glancing at the menubar
**so that** the app actually changes my workflow

**Acceptance criteria**
- **目标**：每个 beta 用户都至少经历一次 "切回某个 session 发现它已经在等候若干分钟" 的瞬间——这是建立扫菜单栏习惯的核心触发点
- **验收门槛**：5 个 beta 用户中至少 4 个能复述这个瞬间（80% 通过率即视为达成；预留 1 个用户场景不匹配的容错）
- 不需要看说明书也能用起来（empty state + 自然语言提示足够，跟 [E1](#e1--首次安装空状态) 互锁）

**Open question**
- 是否需要 first-run onboarding tooltip？MVP 不做，在 [scenarios.md § S4](scenarios.md) 验证是否必要
- 如果新用户在第一周流失率高，回头加 onboarding hint

**Notes**
- 这条不是 implement 的功能，是**观察 + iterate** 的故事
- 验证方式：beta 用户访谈
- 前提条件：用户在 beta 期至少每天跑 3+ session，否则触发频率不够
- 转化机制假设：用户必须**亲身**经历一次"切回去发现已等了一会儿"才会建立扫菜单栏习惯——光看说明书不够

---

### L2 — 熟练期（第 2-4 周）

**As a** user who's been using the app for 2-4 weeks
**I want the app to fade into the background of my attention**
**so that** I'm using it without thinking about it

**Acceptance criteria**
- app 的 UI 在 v0.1 生命周期内不变化（避免"东西又变了"的熟练用户挫败感）
- 性能稳定：连续运行 24h 无可感知 lag、无内存泄漏（RSS 增长 < 50MB / 24h）
- 维护负担可观察行为见下文 "What this rules out"——该列表全部成立即视为通过

**What this rules out**
- 不在用户用了 N 天后弹"评价 5 星"提示
- 不在 app 内推自己的其他产品
- 不在 UI 上加"探索新功能"角标
- 不强制升级（升级走用户主动下载新版本，app 不自带升级提示）

**Notes**
- 稳定性 > 新功能。L2 是验证产品哲学。
- 参考 [scenarios.md § S5](scenarios.md)

---

### L3 — 重负载演化

**As a** user whose session count grew from 3 to 8 over time
**I want the UI to scale gracefully
**so that** my growth in usage doesn't break the tool

**Acceptance criteria**
- 列表 6+ 条后开启滚动，不挤压（具体阈值在 E4 acceptance 已固化为 6）
- 重要状态（"等你"数字）始终视觉突出，不被淹没

**Notes**
- 性能 SLA 不在此重复，见 [E4](#e4--极多-session10) acceptance
- 8 session 是当前作者峰值，15 是设计 headroom
- 跟上文 [E4](#e4--极多-session10) 互锁；[scenarios.md § S2](scenarios.md) 是 stress 验证

---

## A · Adversarial（特殊使用环境）

### A1 — 演示屏幕 / Zoom 共享时

**As a** user sharing my screen in a meeting
**I want to** not accidentally leak content from other sessions
**so that** sensitive code/messages don't show up to viewers

**Acceptance criteria（分版本）**
- **MVP**：用户自行 Quit app 在 demo 前（`guides/install.md` 说明）
- **v0.2+**：进入"demo mode" 隐藏 `last_message` 预览——**实施前必须 ADR 决策具体触发方式**：
  - 备选 (a)：自动检测屏幕共享需 CGDisplay 权限申请，破坏 R2 零配置
  - 备选 (b)：用户主动按钮切 demo mode，保 R2 但增加一次手动操作
- tray icon 本身（图标 + 数字）信息含量极低，无敏感性，不需要任何隐藏

**Notes**
- MVP 阶段优先级低（重度 CC 用户不常 demo），但要 flag——避免未来加自动检测时忘记权衡 R2 零配置

---

### A2 — macOS Focus / DND 开启时

**As a** user in Focus mode
**I want the app to respect my "do not disturb" intent
**so that** it doesn't become another distraction source

**Acceptance criteria**
- **MVP**：app 行为不变（反正本来就不发通知，对 Focus 透明）
- **v0.2+**：考虑 Focus 期间 tray 数字变灰 / 隐藏，减少视觉刺激

**Notes**
- 因为 MVP 本身不主动通知，Focus 模式下 app 实际上已经是 Focus-friendly
- 这条更像是"不要倒退"的护栏——未来如果加了通知功能，必须先满足 A2

---

### A3 — 笔记本待机后唤醒

**As a** user closing the lid and reopening later
**I want the app to recover and resume monitoring
**so that** I don't come back to a stale or frozen state

**Acceptance criteria**
- 唤醒后 2s 内完成首次刷新（不是等下一个 2s tick）
- 期间退出的 claude 进程被正确移除
- 新启动的 claude 进程被正确发现
- app 自身的 timer 不卡死

**Notes**
- macOS 唤醒事件：`NSWorkspaceDidWakeNotification`——MVP 不主动监听，依赖 setInterval 自然恢复
- v0.2+ 可订阅事件，唤醒后立即触发一次 refresh

---

## 引用其他文档

- 产品定义 v0.2：`product/definition.md`（待写为正式文档）
- 场景剧本：[scenarios.md](scenarios.md)
- UML 设计图索引：[design/uml/00-index.md](../design/uml/00-index.md)
- 待规划 spec：`spec/jsonl-schema.md`（用 E5、F1 的 acceptance 反推格式需求）
