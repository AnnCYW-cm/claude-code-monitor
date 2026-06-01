# Roadmap

> **Purpose**：时间维度文档——什么时候做什么 / 已经做了什么 / 下一步做什么。
>
> 跟 [docs/bmad/02-planning/PRD.md § 13 Release plan](../bmad/02-planning/PRD.md) 互补：PRD 是产品视角的 stage，本目录展开时间表 + 各版本具体范围。

---

## 文档清单

| 文件 | 内容 |
|---|---|
| [v0.1.md](v0.1.md) | MVP 范围 / 阻塞 / 目标 release 时间 |
| [v0.2.md](v0.2.md) | v0.2 候选功能清单（来自 v0.1 的"deferred"项） |
| [launch-plan.md](launch-plan.md) | 发布渠道 + 文案模板 + 时机 |
| `CHANGELOG.md` | 见项目根 [/CHANGELOG.md](../../CHANGELOG.md) |

---

## Release lifecycle

```
[dogfood] → [closed alpha] → [public beta] → [v1.0]
   ↓             ↓                ↓             ↓
作者自用      5-10 朋友测      GitHub release  Show HN + brew cask
14 天         私下推             + 推特           + Anthropic Discord
```

详见 [v0.1.md](v0.1.md) 各 stage 标准 + [launch-plan.md](launch-plan.md) 渠道执行。

---

## 跟 epics 的关系

- [`bmad/03-solutioning/epics/`](../bmad/03-solutioning/epics/README.md) 是 **what to build**（dev story 维度）
- 本目录 `roadmap/` 是 **when to ship**（版本 + 时间维度）

例：`v0.1.md` 列出 v0.1 含哪些 epic/story，`launch-plan.md` 列出 v0.1 怎么发布。
