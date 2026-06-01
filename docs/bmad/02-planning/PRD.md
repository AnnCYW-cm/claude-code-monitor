# PRD — Claude Code Monitor v0.1

> **BMAD Phase 2 · Planning · PM output**
> **Status:** Draft → 待 Architect review (implementation-readiness check)
> **Version:** v0.1 (MVP)
> **Author:** PM agent (扮演)
> **Date:** 2026-05-17
>
> 基于 [product-brief.md](../01-analysis/product-brief.md) 展开。下游 [architecture.md](../03-solutioning/architecture.md) + [epics/](../03-solutioning/epics/) 基于此切分。
>
> 本 PRD 跟现有 [product/user-stories.md](../../product/user-stories.md) 和 [product/scenarios.md](../../product/scenarios.md) 互补：那两份是用户视角，本 PRD 是产品视角，整合需求、约束、验收。

---

## 1. Document control

| 字段 | 值 |
|---|---|
| 产品 | Claude Code Monitor |
| 版本 | v0.1 (MVP) |
| 目标 release | 内部 dogfood → alpha → public release (3 个阶段) |
| 最早可发布日 | 完成 MVP 实现 + Gatekeeper 文档实测后 |
| 决策记录 | [decision-log.md](decision-log.md) |
| 补充说明 | [addendum.md](addendum.md) |

---

## 2. Background & motivation

详见 [product-brief.md § 2-3](../01-analysis/product-brief.md)。

简述：作者每天并行运行 3-8 个 `claude` CLI session，频繁遇到"某个 session 完了在等我但我没察觉"的问题。现有所有 workaround（shell loop / iTerm trigger / tmux tail）都不 zero-config。市面无直接竞品。

---

## 3. Goals & non-goals

### 3.1 Product goals (v0.1)

| ID | Goal | Success signal |
|---|---|---|
| G1 | 让用户**抬眼可知**任一 session 是否在等输入 | 视觉扫描 < 0.5s 完成判断 |
| G2 | 让用户**点开可知**所有 session 当前状态 + 内容 | popup 弹出 < 200ms |
| G3 | 用户**无需配置**即可使用 | 装完到第一次"啊有用" < 20 分钟 |
| G4 | app 本身**永不打断**用户工作 | 0 通知 / 0 抢焦点 / 0 配置弹窗 |
| G5 | 跨任何 macOS 终端 emulator 工作 | iTerm/Terminal/Ghostty/Warp/Alacritty 都生效 |

### 3.2 Non-goals (v0.1)

详见 [product-brief.md § 5.2](../01-analysis/product-brief.md)。摘要：
- 不通知 / 不配置 / 不管理 session / 不跳转 tab / 不显示历史 / 不跨平台 / 不团队协作 / 不卡死检测 / 不守护

---

## 4. User segments

| Segment | 描述 | 占比 | 优先级 |
|---|---|---|---|
| **Heavy CC user (primary)** | 同时跑 ≥3 session、一天 ≥4 小时 CC、多项目并行、macOS | ~50% of CC user 中 macOS 重度子集 | P0 |
| Light CC user (secondary) | 偶尔 3+ session | ~30% | P1 |
| Tmux power user | 持久化 session、有自己 workaround | ~10% | P2 |
| 非 macOS / 非 CC 用户 | — | — | 不服务 |

详细 user persona 见 [scenarios.md](../../product/scenarios.md) 的 5 个剧本。

---

## 5. Functional requirements

### 5.1 Core monitoring loop（必需）

| FR-ID | 需求 | 关联 story |
|---|---|---|
| FR-1 | App 启动后，自动枚举所有运行中的 `claude` CLI 进程 | [E2](../../product/user-stories.md#e2--开机时已有-n-个-session-在跑) |
| FR-2 | 对每个 claude 进程，定位它当前在写的 JSONL transcript（路径在 `~/.claude/projects/<encoded-cwd>/<uuid>.jsonl`） | — |
| FR-3 | 读取 JSONL 最后一条消息 | — |
| FR-4 | 基于最后一条消息分类成 Waiting / Working / Unknown | [UML 09 State](../../design/uml/09-state-session.md) |
| FR-5 | 每 2 秒重复 FR-1 ~ FR-4 | [UML 07 Refresh](../../design/uml/07-sequence-refresh.md) |
| FR-6 | 进程退出后从列表移除（不留尸） | [E6](../../product/user-stories.md#e6--session-退出瞬间) |

### 5.2 Menubar UI（必需）

| FR-ID | 需求 | 关联 story |
|---|---|---|
| FR-7 | macOS 菜单栏常驻一个 tray icon | [H1](../../product/user-stories.md#h1--瞄一眼判断是否切走) |
| FR-8 | tray icon 上显示当前 waiting session 数量（如 "3"） | H1 |
| FR-9 | tray icon 与 popup 列表的 waiting 数量永远一致 | H1 acceptance |
| FR-10 | 左键点击 tray icon → 弹出 popup window | [H2](../../product/user-stories.md#h2--弹开列表分诊优先级) |
| FR-11 | 右键点击 tray icon → 弹出 native menu（含 Quit） | [H4](../../product/user-stories.md#h4--退出-app) |
| FR-12 | popup 列表显示每个 session：cwd 末段名 / 状态 / 已等时长（waiting 时） / 最后一条 assistant 消息预览 | H2 |
| FR-13 | popup 列表按 waiting 时长降序排序（等得越久越靠上） | H2 |
| FR-14 | 点击列表项展开显示完整最后一条 assistant 消息 | [H3](../../product/user-stories.md#h3--不切走也能读到关键信息) |
| FR-15 | 同一时刻 popup 最多 1 项展开；再点同项收起；点其他项收起当前并展开新项 | H3 |
| FR-16 | popup empty state：列表为空时显示 "no claude sessions running" | [E1](../../product/user-stories.md#e1--首次安装空状态) |

### 5.3 Robustness（必需）

| FR-ID | 需求 | 关联 story |
|---|---|---|
| FR-17 | JSONL parse 失败时，session status = Unknown（不让消失），下一轮 refresh 自动重试 | [F1](../../product/user-stories.md#f1--jsonl-损坏读不到) |
| FR-18 | claude 进程刚启动 JSONL 还没建好时，session 仍出现在列表 status=Unknown | [E2 启动竞态](../../product/user-stories.md#e2--开机时已有-n-个-session-在跑) |
| FR-19 | log file 路径：`~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log` | [F1](../../product/user-stories.md#f1--jsonl-损坏读不到) |

### 5.4 Negative requirements（必需不做）

"Spec Kit ID" 列原是双轨制时 spec.md 的对应编号（spec-kit/ 重构后已删，编号保留作为历史 trace）：

| FR-ID | 不做什么 | 关联 story | (legacy) Spec Kit ID |
|---|---|---|---|
| FR-N1 | 不发任何通知（toast / sound / badge） | [R1](../../product/user-stories.md#r1--不该主动打断用户) | F-N-1 |
| FR-N2 | 不抢焦点 / 不闪烁 tray | [R1](../../product/user-stories.md#r1--不该主动打断用户) | F-N-2 |
| FR-N3 | 不要求任何用户配置（无登录 / 无 API key / 无目录选择 / 无阈值设定 / 无配置面板） | [R2](../../product/user-stories.md#r2--不该要求配置才能用) | F-N-3 |
| FR-N4 | 不实现 session 命名 / 标签 / 分组 / 备注 | [R3](../../product/user-stories.md#r3--不该需要用户记每个-session-是干嘛) | F-N-4 |
| FR-N5 | 不实现"点击跳转到对应终端 tab" | [R4](../../product/user-stories.md#r4--不该接管切到对应-tab) | F-N-5 |
| FR-N6 | 不显示已退出的历史 session | [R5](../../product/user-stories.md#r5--不该展示历史已退出-session) | F-N-6 |
| FR-N7 | 不访问任何网络（已在 NFR-S1，这里冗余明示）| — | F-N-7 |

---

## 6. Non-functional requirements

### 6.1 Performance

| NFR-ID | 指标 | 目标 |
|---|---|---|
| NFR-P1 | `list_sessions` invoke 单次耗时 | < 50ms（10 session 内），< 100ms（15 session） |
| NFR-P2 | tray 点击到 popup 显示 | < 200ms |
| NFR-P3 | 启动后首次扫描 | < 2s |
| NFR-P4 | 连续运行 24h 内存增长 | RSS < 50MB |
| NFR-P5 | CPU 占用（空闲时） | < 0.5% 平均（M1/M2 macOS）—— **风险项**：数学上 2s polling × 单次 50ms = 2.5%，实测可能超 budget。如超 → 降 polling 频率到 5s 或后端 cache。详见 [readiness § 2.1](../03-solutioning/implementation-readiness.md) |

### 6.2 Compatibility

| NFR-ID | 指标 | 目标 |
|---|---|---|
| NFR-C1 | macOS 版本支持 | 目标 macOS 12+，**实测仅 26.3.1**（作者机器），12-25 等 beta 用户反馈 |
| NFR-C2 | 硬件 | Apple Silicon (M1+) 优先；Intel 兼容（待实测） |
| NFR-C3 | 终端 emulator 兼容性 | iTerm2 / Terminal.app / Ghostty / Warp / Alacritty 任一 |
| NFR-C4 | Claude Code 版本 | 基线: 2.1.126 (2026-05-18 实测)；JSONL schema 锁定见 [spec/jsonl-schema.md](../../spec/jsonl-schema.md)；向前兼容靠 serde `#[serde(default)]` + Option fields |

### 6.3 Reliability

| NFR-ID | 指标 | 目标 |
|---|---|---|
| NFR-R1 | 单个 session 的 JSONL parse 失败不影响其他 session | 必须 |
| NFR-R2 | app 自身崩溃后重启可恢复全部状态 | 必须（因为状态全在 JSONL 里） |
| NFR-R3 | 启动期 crash 率 | < 1% (基于 alpha 用户反馈) |

### 6.4 Security & privacy

| NFR-ID | 指标 | 目标 |
|---|---|---|
| NFR-S1 | 不访问网络 | 必须（app 完全本地） |
| NFR-S2 | 不读 ~/.claude/ 以外的目录 | 必须 |
| NFR-S3 | 不读敏感字段（如 OAuth token） | 必须（只读 transcript 内容字段） |
| NFR-S4 | 屏幕共享 / demo 时的内容保护 | v0.1: 不实现自动隐藏（用户需自行 Quit app）；v0.2+: ADR 决策（见 [A1](../../product/user-stories.md#a1--演示屏幕--zoom-共享时)） |

### 6.5 Maintainability

| NFR-ID | 指标 | 目标 |
|---|---|---|
| NFR-M1 | 文档驱动开发 | docs/ 比代码先成熟 |
| NFR-M2 | 第一个 contributor PR 难度 | 看 docs/ 一晚上能上手 |
| NFR-M3 | log 详尽便于 debug | log 必须含时间戳 + session pid + 操作 |

### 6.6 Distribution

| NFR-ID | 指标 | 目标 |
|---|---|---|
| NFR-D1 | DMG 大小 | < 15MB |
| NFR-D2 | 安装步骤数 | ≤ 3（含 Gatekeeper bypass） |
| NFR-D3 | 第一次启动到 tray icon 出现 | < 3s |

---

## 7. User stories reference

完整 25 条 user stories 见 [product/user-stories.md](../../product/user-stories.md)。本 PRD 通过 FR 章节引用了具体 story。

类别分布：
- H · Happy Path: 4 条
- E · Edge Case: 6 条
- F · Failure: 4 条
- R · Reverse (不做什么): 5 条
- L · Longitudinal (时间维度): 3 条
- A · Adversarial (特殊环境): 3 条

5 个使用日剧本见 [product/scenarios.md](../../product/scenarios.md)。

---

## 8. UX requirements

详见 [ux-design.md](ux-design.md)。摘要：

- **tray icon**：32×32 模板图标 + waiting 计数字符
- **popup**：360×480 webview window，macOS 风格
- **list item**：单行 cwd + 状态徽章 + 时长 + 单行消息预览
- **展开态**：完整 assistant 消息，含 tool_use 简化展示
- **empty state**：居中文案 + hint
- **macOS 风格**：SF Pro 字体、系统色、macOS Sonoma+ popover 圆角与阴影

---

## 9. Constraints

| 约束 | 来源 | 影响 |
|---|---|---|
| 仅 macOS | brief 边界 | 缩小 user base 但保证 polish |
| Tauri 2.x + Rust + TypeScript | [ADR-001](decision-log.md) | 团队技能 + 体积/分发 trade-off |
| 不签名 / 不公证（v0.1） | 成本 + 无 Apple Dev Program | F4 Gatekeeper 摩擦 |
| 单作者维护 | 资源 | 紧守 MVP 范围，不接受 PR 扩展功能 |
| 完全本地 | 隐私 + 简单 | 不能云端 sync / 不能远程触发 |
| 开源 MIT | 战略 | 任何商业化要重新评估 license |

---

## 10. Dependencies

### 10.1 External

| 依赖 | 用途 | 风险 |
|---|---|---|
| Claude Code CLI 写 JSONL 到 `~/.claude/projects/` | 数据源 | 格式变 |
| macOS 12+ 进程枚举 API | 进程发现 | 系统更新可能影响 |
| sysinfo Rust crate | 进程枚举封装 | 维护活跃度 |
| Tauri 2.x | app 框架 | 稳定但 2.x 还在演进 |
| WebKit (macOS 内置) | webview | 跟 macOS 一起更新 |

### 10.2 Internal

无（这是个新项目）。

---

## 11. Acceptance criteria (PRD-level)

PRD 视为 done 当：

- [ ] 所有 FR / NFR 都有对应 epic / dev story 覆盖
- [ ] 所有 R (Reverse / 不做) story 在 PRD 里有 FR-N 对应条
- [ ] [decision-log.md](decision-log.md) 涵盖所有有争议的决策
- [ ] [architecture.md](../03-solutioning/architecture.md) 完成且 implementation-readiness 通过
- [ ] [epics/](../03-solutioning/epics/) 切完所有 dev story

---

## 12. Open questions

带入 architecture / epics 阶段解决：

1. **JSONL 字段细节**：需要实测 `cat ~/.claude/projects/*/*.jsonl` 校准 [spec/jsonl-schema.md](../../spec/jsonl-schema.md)（待写）
2. **Working session 在 popup 内排序**：MVP 默认按进程枚举顺序，等用户反馈再调整
3. **macOS 12 / 14 / 15 Gatekeeper UX**：**作者待办** F4
4. **launchd 守护**：v0.2+ 看反馈
5. **demo mode 触发方式**：v0.2+ ADR 决策

---

## 13. Release plan

| Stage | 标准 | 渠道 |
|---|---|---|
| **Dogfood** | 作者自己 14 天连续用 | 仅作者 |
| **Closed alpha** | 5-10 个 Heavy CC user 朋友 | 私下推 |
| **Public beta** | alpha 反馈处理完 | GitHub release + 推特 |
| **v1.0** | beta 期 30 天无 P0/P1 issue | Show HN + Anthropic Discord + Homebrew cask 提交 |

---

## 14. Out of scope (再次明确，避免 scope creep)

凡是下面这些功能的 issue / PR，**不接受**（除非先 ADR 推翻 R1/R2/R3/R4/R5 之一）：

- 通知（任何形式）
- 配置面板
- session 命名/标签/笔记/分组
- 跳转到终端 tab
- 历史已退出 session
- 跨平台（Linux/Windows）
- 团队/协作功能
- 卡死检测
- 自动守护进程（launchd）

→ 进入 [decision-log.md](decision-log.md) 查决策依据。
