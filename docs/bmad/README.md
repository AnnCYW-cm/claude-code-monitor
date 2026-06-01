# BMAD — Index

> **BMAD-METHOD** (Breakthrough Method for Agile AI-Driven Development) 方法论产出。
> 详见 [bmad-method.org](https://docs.bmad-method.org/)。
>
> 文档存放偏离 BMAD 默认 `_bmad-output/` 位置，统一放 `docs/bmad/` ([ADR-011](02-planning/decision-log.md#adr-011--bmad-和-spec-kit-产物放在-docs-下而不是工具默认位置))。

---

## 文档清单

```
docs/bmad/
├── README.md                       ← 本文件
├── 01-analysis/                    ← Phase 1 · Analyst
│   ├── brainstorming.md            ← 散开式思考记录
│   ├── market-research.md          ← 市场 / 竞品 / 定位 / 风险
│   └── product-brief.md            ← 一页 pin-down brief
├── 02-planning/                    ← Phase 2 · PM + UX
│   ├── PRD.md                      ← 完整产品需求文档
│   ├── decision-log.md             ← 12 个 ADR
│   ├── addendum.md                 ← PRD 补充：edge case / 术语 / 反驳
│   └── ux-design.md                ← UI/UX 完整规范
└── 03-solutioning/                 ← Phase 3 · Architect + PM
    ├── architecture.md             ← 叙述性架构（UML 的补充）
    ├── implementation-readiness.md ← PRD↔Arch 交叉验证 checklist
    ├── project-context.md          ← 编码约定 / 模块边界 / 禁用列表
    └── epics/
        ├── README.md               ← Epic 索引 + dev story → product story 反向映射
        ├── epic-001-core-monitoring.md
        ├── epic-002-menubar-ui.md
        ├── epic-003-robustness.md
        └── story-001 ~ story-013 (13 个 dev story)
```

## BMAD 流程对应

| Phase | Agent | 我们的文件 |
|---|---|---|
| Phase 1 · Analysis | Analyst | `01-analysis/*` (3 files) |
| Phase 2 · Planning | PM | `02-planning/PRD.md` + `decision-log.md` + `addendum.md` |
| Phase 2 · Planning | UX Designer | `02-planning/ux-design.md` |
| Phase 3 · Solutioning | Architect | `03-solutioning/architecture.md` + `implementation-readiness.md` |
| Phase 3 · Solutioning | PM | `03-solutioning/epics/` (epic + story) |
| Phase 3 · Solutioning | Analyst | `03-solutioning/project-context.md` |
| Phase 4 · Implementation | Dev | 暂无（还没进入实施阶段） |

## 阅读顺序建议

**新 contributor**：

1. [product-brief.md](01-analysis/product-brief.md) — 5 分钟看产品定义
2. [PRD.md](02-planning/PRD.md) — 15 分钟看完整需求
3. [architecture.md](03-solutioning/architecture.md) — 20 分钟看架构
4. [epics/README.md](03-solutioning/epics/README.md) — 找一个 story 开始干
5. [project-context.md](03-solutioning/project-context.md) — 写代码前必读

**深度理解**：

6. [decision-log.md](02-planning/decision-log.md) — "为什么这么设计"
7. [addendum.md](02-planning/addendum.md) — edge case 处理细则
8. [ux-design.md](02-planning/ux-design.md) — UI 像素级规范
9. [implementation-readiness.md](03-solutioning/implementation-readiness.md) — 阻塞项 + open question

## 历史：Spec Kit 双轨制（已撤销）

原先 BMAD 跟 [GitHub Spec Kit](https://github.com/github/spec-kit) 并存（详 [ADR-011 + ADR-013](02-planning/decision-log.md)），2026-05-18 重构后 spec-kit/ 目录撤销，独立产物 mv 进 BMAD：

| Spec Kit 产物 | → 现在的位置 |
|---|---|
| `constitution.md` | [`docs/constitution.md`](../constitution.md)（顶层） |
| `tasks.md` | [`03-solutioning/tasks.md`](03-solutioning/tasks.md) |
| `data-model.md` | [`03-solutioning/data-model.md`](03-solutioning/data-model.md) |
| `research.md` | [`03-solutioning/research-notes.md`](03-solutioning/research-notes.md) |
| `quickstart.md` | [`03-solutioning/quickstart.md`](03-solutioning/quickstart.md) |
| `contracts/ipc-contract.md` | [`docs/spec/ipc-contract.md`](../spec/ipc-contract.md) |
| `spec.md` | 删除（重复 PRD） |
| `plan.md` | 删除（重复 architecture） |

## Open questions / 阻塞项

进入 implementation 前必须解决（详 [implementation-readiness § 4](03-solutioning/implementation-readiness.md)）：

- **OQ-1 / OQ-2**: Claude Code JSONL 字段格式 + process↔JSONL 配对 → 写 [`spec/jsonl-schema.md`](../spec/jsonl-schema.md)
- **OQ-3**: macOS 12/14/15 Gatekeeper UX 实测 → 写 [`guides/install.md`](../guides/install.md)

## 待办登记

按 [`docs/README.md` 待办协议](../README.md)，所有待办用 `**作者待办**` 标记。集中查：

```bash
grep -rn '作者待办' docs/bmad/
```
