# Constitution — Claude Code Monitor

> **Project governance principles · binding**
> **Version:** v0.1 (跟项目主版本一致)
> **Status:** Effective for v0.1 dev period (2026-05-18 起)
> **Authority:** Author (caiyiwen). Changes via PR + ADR.
>
> 项目治理原则。每个 spec / plan / task / implementation 都必须对齐这些原则。
> 修改本文件需要先开 [ADR](bmad/02-planning/decision-log.md) + PR 评审。
>
> Originally from Spec Kit `/speckit.constitution` flow, mv 到 docs/ 顶层 by [ADR-013](bmad/02-planning/decision-log.md).

---

## I. Product principles (产品原则)

### I.1 Passive awareness over active query (被动感知 > 主动查询)

App 必须让用户「抬眼可知」，而不是「主动检查」。这意味着：
- **永不打断**：不弹通知 / 不响声音 / 不闪烁 / 不抢焦点
- **永不需要主动查**：tray icon 上的数字必须 0 延迟反映状态变化（≤ 2s polling 周期）
- **永不变成第二个 Slack**：任何让用户需要"处理它"的设计都是反原则

→ 红线。违反需要新 ADR 推翻。

### I.2 Zero configuration (零配置)

App 必须装完即用：
- 不登录
- 不要 API key / token
- 不让用户选目录
- 不设阈值 / 偏好 / 主题
- 不创建配置面板（连入口都没有）

→ 红线。违反 = 引入第一处配置 → 引发更多配置请求 → 走向 IntelliJ Settings 噩梦。

### I.3 Now, not history (此刻，不是历史)

App 只反映当前运行中的 session：
- 进程退出 = 立即从列表移除
- 不显示历史已退出 session
- 不做"5 分钟前等过你"
- 不做 session 搜索 / 时间线

→ 心智模型：dashboard，不是 logbook。

### I.4 Cross-emulator neutrality (跨终端中立)

App 必须跟所有 macOS 终端 emulator 工作：
- 不能集成任何特定 emulator 的 API
- 不能依赖窗口标题约定
- 不能"跳转到对应 tab"

→ 这迫使 app 只用 OS 进程表 + 文件系统作为数据源。

### I.5 Stability over feature flashing (稳定胜过功能光鲜)

熟练用户（用了 ≥ 2 周）最讨厌"东西又变了"：
- v0.1 生命周期内 UI 不变
- 不主动推升级
- 不弹"探索新功能"角标
- 不弹评分提示

→ 产品哲学：fade into background。

---

## II. Code quality principles (代码质量原则)

### II.1 Documentation as source of truth (文档是真理之源)

- 每个 PR 必须同步更新相关文档
- 文档跟代码不一致 = 代码错（除非有充分理由更新文档）
- 设计文档（spec / plan / architecture）在代码之前

### II.2 Simple > clever (简单胜过聪明)

- 单文件能装下不要拆模块（MVP 阶段）
- 同步阻塞能用不要 async
- 手写 DOM 能用不要引框架
- 编译 fmt 通过就 merge，不追求 perfect

### II.3 No silent failure (无静默失败)

- 所有 `Result::Err` 必须 log（不能 `.ok()` 吞掉）
- 所有 panic 必须有 catch_unwind 兜底（除非是 setup 期 fail-fast）
- log 错误必须含上下文（pid / file path / etc）

### II.4 Test what matters (测关键)

- session.rs (核心逻辑): ≥ 80% line coverage
- IPC commands: 集成测试覆盖
- UI: manual + dogfood
- 性能 SLA: benchmark in CI

### II.5 No forbidden dependencies (禁用依赖)

| 禁用 | 原因 |
|---|---|
| http client (reqwest/hyper/isahc/...) | I.1 不外联 |
| notify crate (fs watcher) | [ADR-002](bmad/02-planning/decision-log.md#adr-002--监控状态用-polling-不用-fs-watcher) |
| 我们代码显式 `tokio::` API | [ADR-007](bmad/02-planning/decision-log.md#adr-007--list_sessions-ipc-设计为同步阻塞)。注：Tauri 2.x 内部 transitively 引入 tokio runtime 是正常的（无法避免），禁的是**我们 application code 写 async tokio 调用** |
| Notification API / 任何系统通知 | I.1 红线 |
| osascript / AppleScript | I.4 跨终端中立 |
| React / Vue / Svelte | [ADR-006](bmad/02-planning/decision-log.md#adr-006--前端不引入框架vanilla-ts) |
| jQuery / underscore / lodash (前端) | 不需要 |

CI 自动 grep 阻断（详见 [project-context § 9](bmad/03-solutioning/project-context.md)）。

---

## III. UX consistency principles (UX 一致性原则)

### III.1 macOS native look-and-feel

- 用系统字体 (SF Pro / SF Mono)
- 用系统色（labelColor / systemYellow / etc）
- 跟随 light/dark mode 自动切换
- 圆角 / 阴影遵循 macOS Sonoma+ popover 风格

### III.2 Single source of UX truth

[ux-design.md](bmad/02-planning/ux-design.md) 是所有 visual decision 的 single source of truth：
- 尺寸 / 字体 / 颜色 / 间距 / 动效预算都在那
- 代码 PR 改 UI 必须先改 ux-design.md（如果是新规范）或符合现有规范

### III.3 Animation budget

整个 app 只允许 4 种动效（见 [ux-design § 11](bmad/02-planning/ux-design.md)）：
- popup show / hide / hover / expand

其他动效（spinner / fade-on-change / pulse）一律禁止。

### III.4 Accessibility minimum

- 对比度 ≥ WCAG AA
- VoiceOver 至少能读 tray icon 状态
- 键盘导航 v0.2+ 实现，MVP 不强制

---

## IV. Performance principles (性能原则)

### IV.1 Hard budgets

| 指标 | 上限 |
|---|---|
| `list_sessions` invoke (10 session) | < 50ms |
| `list_sessions` invoke (15 session) | < 100ms |
| popup show/hide | < 200ms |
| 启动到 tray icon 出现 | < 3s |
| 空闲 CPU (M1) | < 0.5% avg |
| 24h RSS 增长 | < 50MB |

超过 budget = 不能 release。

### IV.2 Lazy > eager

- 不预加载（按需 IPC）
- 不缓存（每轮 refresh 重读 JSONL）
- 例外：webview window 预创建 + hide（show 时零延迟）

### IV.3 Measure before optimize

- 没 benchmark 数据不优化
- benchmark in CI，回归即阻断

---

## V. Governance (治理)

### V.1 Authority

- 唯一 maintainer：caiyiwen
- 接受 PR 的标准：符合本 constitution + 通过 CI + acceptance criteria 全过

### V.2 Changing the constitution

修改本文件需要：
1. 新开 [ADR](bmad/02-planning/decision-log.md)，论证 why
2. PR 评审 ≥ 1 周（让社区 review）
3. 通过后 merge + 在 CHANGELOG 注明

紧急修复（typo / 链接修复）不需要走流程。

### V.3 Pace

- MVP 阶段（v0.1）：维护者每周末 1 次 office hour（review issue / PR）
- 稳定后（v1.0+）：每两周一次

### V.4 What we won't change

以下原则 **永久** 不变（除非项目方向 fundamentally 改）：
- I.1 被动感知（红线）
- I.2 零配置（红线）
- 跨终端中立（核心定位）
- 完全本地，不外联

如果要改这些，应该是 fork 一个新项目。

### V.5 License

MIT ([ADR-009](bmad/02-planning/decision-log.md#adr-009--开源-mit-license))。

---

## VI. Conflict resolution (冲突解决)

如果 spec / plan / task 之间冲突：

1. **此 constitution 最高**——其他文档必须服从
2. **次之：[product-brief](bmad/01-analysis/product-brief.md)**——产品定义
3. **次之：[PRD](bmad/02-planning/PRD.md)**——细化需求
4. **次之：[architecture](bmad/03-solutioning/architecture.md)**——技术决策
5. **最低：实现代码**——遵循上面所有

发现冲突 = 更新上层文档，下层跟进。

---

## VII. Living document

本文件随项目演进维护：
- 发现现有原则被反复违反 → 修订原则（评估是否原则本身错了）
- 发现新需要的原则 → 新增条目（按 V.2 流程）
- 版本号跟项目主版本一致（当前 v0.1）

---

**Signed by**: caiyiwen, 2026-05-18
**Next review**: v1.0 release 前
