# Docs

项目所有文档的入口。代码在 `src/` 和 `src-tauri/`；本目录只装文档。

> **设计期原则：宁详勿略。**
> 边界写清楚、why 写清楚、视角写清楚——避免后期实现跑偏。
> 每份文档的目标读者是「6 个月后的自己 / 第一次接手的贡献者」，不是此刻全知全能的我们。

---

## TL;DR — 这是什么

**Claude Code Monitor**：macOS 菜单栏 app，让你同时跑多个 Claude Code session 时一眼知道哪个在等输入。

- 状态：v0.1 MVP 设计期，Tauri scaffold 已建（src-tauri/ + src/），实现未开始
- 阻塞项：[spec/jsonl-schema.md](spec/jsonl-schema.md) 待实测 + [guides/install.md](guides/install.md) 待写 Gatekeeper 步骤
- 文档体量：64 个 markdown / ~11.6K 行（对一个 MVP 是重资产——by design，[constitution § II.1](constitution.md) 把 documentation 设为 source of truth）

---

## 我该从哪开始读？（按角色）

### A · 想了解这个产品做什么 (5 分钟)

1. [bmad/01-analysis/product-brief.md](bmad/01-analysis/product-brief.md) — 一页 pin-down
2. [product/scenarios.md](product/scenarios.md) — 5 个使用日剧本

### B · 想 implement 某个 story (20 分钟)

1. [bmad/03-solutioning/project-context.md](bmad/03-solutioning/project-context.md) — 编码约定 / 禁用列表
2. [bmad/03-solutioning/epics/README.md](bmad/03-solutioning/epics/README.md) — 找你的 story
3. story 文件 → [design/ui/](design/ui/README.md) 看对应 UI spec → 写代码

### C · 想理解某个设计决策为什么这么定 (10 分钟)

1. [bmad/02-planning/decision-log.md](bmad/02-planning/decision-log.md) — 12 个 ADR 集中地
2. 找不到答案：[bmad/02-planning/addendum.md](bmad/02-planning/addendum.md) edge case 讨论

### D · 想推翻某个产品红线 / 加新功能 (必读)

1. [constitution.md](constitution.md) — 红线 + 治理流程（顶层文件）
2. [bmad/02-planning/PRD.md](bmad/02-planning/PRD.md) § 14 Out of scope
3. 必须先开新 ADR 覆盖现有的（参考 ADR template 在 decision-log 末尾）

### E · 想了解整个文档体系怎么组织的（贡献新文档前）

跳到下面 [§ 目录约定](#目录约定) 看每个子目录的边界 + [§ 我要新写一份文档怎么办](#我要新写一份文档怎么办)。

### F · 想贡献 (报 bug / 提 PR)

→ [guides/CONTRIBUTING.md](guides/CONTRIBUTING.md)

### G · 想知道 release 怎么发 / launch 文案

→ [roadmap/launch-plan.md](roadmap/launch-plan.md) + [roadmap/v0.1.md § 6 Definition of done](roadmap/v0.1.md)

---

## 当前已有

### `product/` — 产品视角文档

| 文件 | 内容 |
|---|---|
| [user-stories.md](product/user-stories.md) | 25 条用户故事（H/E/F/R/L/A 六类：happy / edge / failure / reverse / longitudinal / adversarial） |
| [scenarios.md](product/scenarios.md) | 5 个使用日剧本（典型 / 重负载 / 出错 / onboarding / 稳定使用） |
| [success-metrics.md](product/success-metrics.md) | 4 tier 成功标准 + 怎么测 + 反指标 + 数据来源 |
| `definition.md` | **不写**——[bmad/01-analysis/product-brief.md](bmad/01-analysis/product-brief.md) 已是 single source of truth |

### `roadmap/` — 时间维度文档

| 文件 | 内容 |
|---|---|
| [README.md](roadmap/README.md) | 索引 + release lifecycle 流程 |
| [v0.1.md](roadmap/v0.1.md) | MVP 范围 + blockers + 4 release stages + 时间表 + Definition of done |
| [v0.2.md](roadmap/v0.2.md) | v0.2 候选清单（来自 v0.1 deferred 项，5 类 + 永不做的） |
| [launch-plan.md](roadmap/launch-plan.md) | 发布渠道（HN / Twitter / Anthropic Discord）+ 文案模板 + 时机 + 应对负面反馈 |

### `bmad/` — BMAD 方法论产物（agent pipeline 视角）

BMAD = Breakthrough Method for Agile AI-Driven Development。详见 [bmad-method.org](https://docs.bmad-method.org/)。文档存放偏离了 BMAD 默认 `_bmad-output/` 位置，统一放 `docs/bmad/` ([ADR-011](bmad/02-planning/decision-log.md#adr-011--bmad-和-spec-kit-产物放在-docs-下而不是工具默认位置))。

| 阶段 | 文件 | 内容 |
|---|---|---|
| Phase 1 · Analysis | [01-analysis/brainstorming.md](bmad/01-analysis/brainstorming.md) | 散开式思考记录 |
| | [01-analysis/market-research.md](bmad/01-analysis/market-research.md) | 市场 / 竞品 / 定位 / 风险 |
| | [01-analysis/product-brief.md](bmad/01-analysis/product-brief.md) | 收敛产品 brief（一页 pin-down） |
| Phase 2 · Planning | [02-planning/PRD.md](bmad/02-planning/PRD.md) | 完整产品需求文档 |
| | [02-planning/decision-log.md](bmad/02-planning/decision-log.md) | 12 个 ADR（架构 / 设计 / 流程决策） |
| | [02-planning/addendum.md](bmad/02-planning/addendum.md) | PRD 补充：edge case / 术语 / 反驳 |
| | [02-planning/ux-design.md](bmad/02-planning/ux-design.md) | UI/UX 完整规范 |
| Phase 3 · Solutioning | [03-solutioning/architecture.md](bmad/03-solutioning/architecture.md) | 叙述性架构（UML 的补充） |
| | [03-solutioning/implementation-readiness.md](bmad/03-solutioning/implementation-readiness.md) | PRD↔Arch 交叉验证 checklist |
| | [03-solutioning/project-context.md](bmad/03-solutioning/project-context.md) | 编码约定 / 模块边界 / 禁用列表 |
| | [03-solutioning/epics/README.md](bmad/03-solutioning/epics/README.md) | 3 epic + 13 dev story 索引 |

### `spec/` — 格式 / 接口规格（部分 placeholder）

| 文件 | Status |
|---|---|
| [spec/README.md](spec/README.md) | 索引 |
| [spec/jsonl-schema.md](spec/jsonl-schema.md) | **TBD** —— 阻塞 BMAD S-002/S-004 |
| [spec/logging.md](spec/logging.md) | **TBD** |

→ IPC contract 在 spec-kit 目录，未来可能 mv 到 spec/。

### `guides/` — 操作指南

| 文件 | Status |
|---|---|
| [guides/README.md](guides/README.md) | 索引 |
| [guides/install.md](guides/install.md) | Draft（macOS 26 推断版，12-15 跨版本待 beta 测） |
| [guides/CONTRIBUTING.md](guides/CONTRIBUTING.md) | ✅ DONE — 怎么报 bug / 提 PR / 贡献文档 |
| [guides/dogfood-retrospective-template.md](guides/dogfood-retrospective-template.md) | ✅ DONE — 14 天 dogfood 后填的 template |

### `design/ui/` — 视觉设计 / UI 实现 spec（7 个文件）

把 [bmad/02-planning/ux-design.md](bmad/02-planning/ux-design.md) 的文字规范扩展成**可视的、可直接照实现的**设计文档。含详细 ASCII mockup、各状态对比、组件 CSS spec。

| 文件 | 内容 |
|---|---|
| [README.md](design/ui/README.md) | 索引 + 设计原则速查 + 跟 ux-design.md 的边界 |
| [tray-icon.md](design/ui/tray-icon.md) | 各 waiting count / light-dark mode mockup + spec + state machine |
| [popup-window.md](design/ui/popup-window.md) | 整体框架 mockup（多场景）+ 尺寸 + 位置 + 显隐 animation |
| [list-item.md](design/ui/list-item.md) | Anatomy + 4 状态 mockup + 3 status 颜色变体 + edge case |
| [empty-state.md](design/ui/empty-state.md) | Empty state mockup + 文案 + light/dark 对比 |
| [menu.md](design/ui/menu.md) | 右键 native menu mockup + items + 未来扩展占位 |
| [animations.md](design/ui/animations.md) | 4 个动效 timing 序列 + easing 曲线 + CSS keyframes |
| [component-css.md](design/ui/component-css.md) | 全部 CSS class + selector + system color variables + 完整 implementation skeleton (~190 lines) |

### `design/uml/` — 10 张 UML 图，按推荐阅读顺序

| # | 文件 | 回答的问题 |
|---|---|---|
| 00 | [index](design/uml/00-index.md) | 这套 UML 怎么读 |
| 01 | [Use Case](design/uml/01-use-case.md) | 谁用、能做什么 |
| 02 | [Activity](design/uml/02-activity-end-to-end.md) | 用户一次完整使用的端到端流程 |
| 03 | [Component](design/uml/03-component.md) | 系统由哪些模块、跨语言边界在哪 |
| 04 | [Package](design/uml/04-package.md) | 代码包的组织、编译期依赖 |
| 05 | [Class](design/uml/05-class.md) | 关键数据结构、派生关系 |
| 06 | [Sequence · Startup](design/uml/06-sequence-startup.md) | 双击 .app 到 tray 出现之间发生了什么 |
| 07 | [Sequence · Refresh](design/uml/07-sequence-refresh.md) | 每次 2s 轮询的前后端协作 |
| 08 | [Sequence · Tray Click](design/uml/08-sequence-tray-click.md) | 点击 tray 时的响应 |
| 09 | [State · Session](design/uml/09-state-session.md) | session 的状态机（unknown / working / waiting） |
| 10 | [Deployment](design/uml/10-deployment.md) | 部署在用户机器上的拓扑 |

---

## 目录约定

每个顶层目录是一个文档大类。下表说明边界、视角、触发条件——决定一份文档「该放哪里」「什么时候要新写一份」。

> **当前实际存在的目录**：`product/` / `bmad/` / `design/{uml,ui}/` / `spec/` / `guides/` / `roadmap/`（6 个）+ 顶层 `constitution.md` + 项目根 `CHANGELOG.md` / `SECURITY.md`
> **约定预留但还没建**：`research/`、`design/adr/`（到第一份文档落地时一起 `mkdir`）
> 这跟"不预建空目录" 约定一致——预留约定 ≠ 预建目录。
>
> **2026-05-18 重构记录**：原 `spec-kit/` 目录已删除，独立内容 mv 到 BMAD/spec 各处，重复内容 merge 进 PRD/architecture/decision-log。详 [ADR-013 in decision-log.md](bmad/02-planning/decision-log.md)。

### `product/` — 产品视角

| 项 | 说明 |
|---|---|
| **放什么** | 产品定义、PRD（需求文档）、用户场景、用户画像、价值主张、明确的「不做」清单 |
| **不放什么** | 技术实现细节（属于 `design/`）；JSONL 字段 / IPC 协议（属于 `spec/`）；时间表（属于 `roadmap/`） |
| **视角** | 用户/业务视角——「做什么 / 为谁做 / 不做什么 / 为什么这么定」 |
| **何时新建** | 启动新功能或大改、用户场景发生变化、产品边界要重新界定 |
| **典型文件** | `definition.md`（产品定义快照，对应当前 v0.2）、`prd.md`（详细需求文档）、`scenarios.md`（用户场景剧本） |
| **跟相邻目录的边界** | vs `research/`：调研是探索，产品文档是结论。结论稳定后，对应的 research 笔记可以保留作佐证，但**决策不能只活在 research 里** |

### `design/` — 技术设计

| 项 | 说明 |
|---|---|
| **放什么** | UML 图、架构决策记录（ADR）、模块设计、关键算法说明 |
| **不放什么** | 用户视角的需求（→ `product/`）；精确的字段/协议定义（→ `spec/`）；具体代码（属于 `src/` 和 `src-tauri/`） |
| **视角** | 工程视角，回答「怎么实现」——但只到**设计层**，不到代码层 |
| **何时新建** | 新功能进入实现前、关键架构决策需要留痕、新增模块需要建模 |
| **典型子目录** | `uml/`（视图建模）；`adr/`（一份决策一文件，命名 `NNNN-<title>.md`，如 `0001-pick-tauri.md`、`0002-poll-vs-fs-watch.md`） |
| **跟相邻目录的边界** | vs `spec/`：spec 是契约（字段名、类型、约束），design 是设计意图（为什么用这套字段、有哪些备选） |

### `spec/` — 接口与格式规格

| 项 | 说明 |
|---|---|
| **放什么** | 数据格式（如 Claude Code JSONL schema）、IPC 字段约定、对外/对内 API 的精确定义 |
| **不放什么** | 设计意图（→ `design/`）；为什么这么定（→ `design/adr/`）；调研过程（→ `research/`） |
| **视角** | 契约视角——读这份文档的人在 implement 或集成时不用问任何人就能写对 |
| **何时新建** | 新增数据格式 / 新增 IPC command / 字段定义发生变化（这种情况要 versioned，旧版本另起一份） |
| **典型文件** | `jsonl-schema.md`（Claude Code transcript 字段）、`ipc-contract.md`（前后端 IPC 字段） |

### `roadmap/` — 时间与版本

| 项 | 说明 |
|---|---|
| **放什么** | 版本计划、changelog、milestone、里程碑回顾 |
| **不放什么** | 需求本身（→ `product/`）；技术任务清单（属于 issue tracker，不属于文档） |
| **视角** | 时间视角——「什么时候做什么 / 已经做了什么 / 下一步做什么」 |
| **何时新建** | 进入下一个版本周期、发布新版本（追加 changelog） |
| **典型文件** | `v0.1.md`（v0.1 范围与目标）、`v0.2.md`、`CHANGELOG.md`（发布后追加） |

### `research/` — 调研笔记

| 项 | 说明 |
|---|---|
| **放什么** | 一次性的调研结果——实地考察 JSONL 格式、对比方案、库选型笔记、用户访谈纪要 |
| **不放什么** | 已经成型的产品/设计决策（→ `product/`、`design/adr/`、`spec/`） |
| **视角** | 探索视角——这是 work-in-progress 草稿地带 |
| **何时新建** | 需要为某个决策找证据 / 比较多个方案 / 探索一个未知领域；一次调研一个文件 |
| **典型文件** | 按主题命名，如 `claude-code-jsonl-format.md`、`tray-positioning-options.md`、`competitive-landscape.md` |
| **关键约定** | 调研完成、结论被采纳后，**结论要搬到正式文档**（`design/adr/` 或 `spec/`），research 文件可保留作佐证。调研结果被推翻或废弃，**删掉文件**而不是留着——过时的调研留下来会误导未来读者 |

### `guides/` — 操作指南

| 项 | 说明 |
|---|---|
| **放什么** | install 指南、dev setup、贡献指南、release 流程、调试指南 |
| **不放什么** | 设计思路（→ `design/`）；为什么这么操作（如果重要，在指南里加短"why"段落，不要单独成文） |
| **视角** | 操作视角——读者要按步骤动手 |
| **何时新建** | 新增一类操作流程（如首次配 CI、首次发版） |
| **典型文件** | `install.md`、`dev-setup.md`、`CONTRIBUTING.md`、`release.md` |

---

## 命名规则（及 why）

| 规则 | 例 | Why |
|---|---|---|
| 顶层目录**不带编号** | `product/`，不是 `01-product/` | 重排目录顺序时不用改链接，跨文档的相对链接保持稳定 |
| 目录内部用 `NN-` 前缀控制阅读顺序 | `uml/00-index.md` → `01-use-case.md` → … | 文件名字典序 = 推荐阅读顺序；GitHub 网页和 IDE 都按字典序展示 |
| 索引文件用 `00-index.md` 或 `README.md` | `docs/README.md`、`uml/00-index.md` | `README.md` 是 GitHub 自动渲染入口（顶层用它）；目录内部用 `00-index.md` 跟编号兄弟同形态 |
| 一份文档一个文件 | ADR 一份决策一文件 | 评审、变更、归档都以单文件为单位；不要把多个独立主题塞一份大文档 |
| 文件名小写连字符 | `dev-setup.md`，不是 `DevSetup.md` 不是 `dev_setup.md` | 跨平台兼容（macOS 大小写不敏感、Linux 敏感）；URL 友好 |
| 不预建空目录 | 第一份文档落地时一起 `mkdir` | 空目录是过度设计，且让人误以为有内容 |
| 文档版本通过文件名而不是 git 历史区分 | `prd.md` → `prd-v2.md`（如果大改且需共存） | git 历史适合追踪小改；大版本并存（比如要同时给两批读者看）用文件名 |

---

## 我要新写一份文档怎么办

1. **判断类型**：用上面"目录约定"的"放什么 / 不放什么"对照，定位到顶层目录。**判断不了大概率说明这份文档职责混乱，先拆**。
2. **判断是否已存在**：`grep -r <主题词> docs/` 看有没有同主题文档。**优先 update 已有，不要并存两份**——并存两份必然其中一份过期。
3. **如果是新目录**：`mkdir` + 第一份文档 + 在本 README「当前已有」小节加一行链接。
4. **如果是已有目录里加一份**：按目录内 `NN-` 编号规则取下一个号；必要时重排兄弟节点编号（用 `git mv` 保留历史）。
5. **写完自检**：
   - 6 个月后第一次看的人能看懂吗？
   - 边界、why、视角写清楚了吗？
   - 跟相邻文档的链接打通了吗？

---

## 文档生命周期

| 阶段 | 处理 |
|---|---|
| **草稿期** | 文件头加 `Status: Draft`；可以频繁迭代；不要急着对外宣传 |
| **稳定期** | 删除 `Status` 标签或标 `Status: Stable`；后续变更进入 review 节奏 |
| **过时但有参考价值** | 文件头标 `Status: Superseded by <link>`，**不要原地删**——可能有外部引用 |
| **完全过时无价值** | 删除文件；如果担心外部引用，留一份 30 字以内的 stub 指向替代文档 |

**文档不更新比没文档更危险**——读者会按过时信息决策。每次代码改动如果影响某份文档，**在同一个 PR 里改文档**，不要拆开。

---

## 待办协议

文档里需要后续完成的事项**统一标记为 `**作者待办**`**，方便集中查找：

```bash
grep -rn '作者待办' docs/
```

不要散落在 issue tracker 或脑子里——文档自带 todo 才能跟着文档迭代，PR diff 也能看见。

---

## 文档级 vs 代码级的边界

- 本目录（`docs/`）放**项目级**文档：设计、产品、规划、调研。
- 代码内部的注释 / docstring **不属于这里**——它们跟代码同生共死，由代码审查管。
- 顶层 `README.md`（项目根那份）是**项目对外门面**——只放"是什么 / 怎么跑 / 链接到 docs/"，不放设计细节。

---

## 维护负担提醒（"重资产文档"代价）

文档系统目前 11.6K 行 / 64 文件（vs 代码 scaffold ~270 行 = **43:1**）。这跟产品哲学（简单、被动、零配置）反差——by design 但要警惕。

**同概念跨文档复述**带来的维护负担（同步时容易漏改）：

| 概念 | 出现文件数 | 改的时候必须同步 |
|---|---|---|
| "2s polling" | ~12 | 改 polling 频率要 sweep 全文档 |
| "Tauri 2.x" | ~10 | 升级 Tauri 主版本要 sweep |
| "永不通知" | ~8 | 砍这条红线必须先开 ADR + sweep |
| "Heavy CC user" | ~7 | 重新定义 target user 要 sweep |
| "360pt popup width" | ~5 | UI 尺寸调整要同步 |
| `~/Library/Logs/com.caiyiwen...` log 路径 | 多处 | 改 bundle id 要 sweep |

**改任何一个时检查清单**：

```bash
grep -rn "<改的内容>" docs/
```

→ 这是接受冗余换取**每个文档独立可读**的代价。如果某个 sweep 漏改，CI 加 cross-doc consistency check（待加 - 作者待办）。
