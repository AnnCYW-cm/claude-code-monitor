# Success Metrics

> **Purpose**：把分散在 [product-brief § 6](../bmad/01-analysis/product-brief.md) + [market-research § 6](../bmad/01-analysis/market-research.md) + [roadmap/v0.1.md § 3](../roadmap/v0.1.md) 的成功标准集中到一个 dashboard 文档，含**怎么测**。

---

## 1. Tier 系统

成功不是 binary。分 4 个 tier，每个 tier 有独立 metric 集：

| Tier | 含义 | 时间窗口 |
|---|---|---|
| **T0 · Personal** | 作者自己 dogfood，证明产品解决真痛点 | release 前 14 天 |
| **T1 · Foundational** | 开源后头一个月，证明有真用户 | release 后 0-30 天 |
| **T2 · Growing** | 3 个月，证明项目能自主增长 | release 后 30-90 天 |
| **T3 · Established** | 6 个月，成为 Claude Code 周边经典 | release 后 90-180 天 |

---

## 2. T0 · Personal (作者 dogfood)

| Metric | Target | 怎么测 | Status |
|---|---|---|---|
| 连续使用天数 | ≥ 14 天不删 | 日历 | ⏳ MVP 后 |
| 每天打开 popup 次数 | ≥ 5 次/天 | 手动记 + log file 的 `INFO popup_shown` 行 count | ⏳ |
| 实际"救我"事件 | ≥ 3 次（"靠它发现某 session 等我 > 5 min"）| dogfood retrospective.md 记录 | ⏳ |
| Crash 次数 | 0 | log file `ERROR panic` 行 count | ⏳ |
| 24h RSS 增长 | < 50MB | Activity Monitor 第 0 和第 14 天对比 | ⏳ |
| 空闲 CPU avg | < 0.5% (M1) | Activity Monitor 1h 平均 | ⏳ |

**Tier pass 条件**：上述全部达成。

**Tier fail 处理**：
- 任一 metric 显著未达 → 不进入 T1 (alpha)，先 fix
- 若多个未达 → 回到 implement 阶段重做相关 story

---

## 3. T1 · Foundational (release 后 30 天)

| Metric | Target | 怎么测 | 当前 |
|---|---|---|---|
| GitHub stars | ≥ 10 | `gh api repos/<owner>/claude-code-monitor` → `stargazers_count` | — |
| Issues from non-author | ≥ 3 | `gh issue list --author '!<owner>'` | — |
| Closed alpha 用户 D14 retention | ≥ 50% (5/10) | 1-on-1 follow-up 聊 14 天后还用不用 | — |
| 4/5 alpha 用户能复述"啊有用瞬间" | true | beta 访谈结构化 | — |
| 推特发布的 like / repost | ≥ 50 / ≥ 5 | twitter analytics | — |
| Show HN 排名 | 当天 top 30 | HN front page screenshot | — |

**T1 pass 条件**：4/6 达成（不要求全过，"foundational"=有用户基础）

---

## 4. T2 · Growing (release 后 90 天)

| Metric | Target | 怎么测 | 当前 |
|---|---|---|---|
| GitHub stars | ≥ 100 | gh api | — |
| External contributor PR merged | ≥ 1 | `gh pr list --author '!<owner>' --state merged` | — |
| Active issues responded < 1 week | ≥ 80% | `gh issue list --created '>=YYYY-MM-DD'` 跟 first response time | — |
| 非作者 fork count | ≥ 10 | gh api | — |
| Homebrew cask submit | merged | brew tap formula PR 状态 | — |
| 第三方文章 / video 提及 | ≥ 1 | Google alert + 推特搜索 | — |

**T2 pass 条件**：4/6 达成

---

## 5. T3 · Established (release 后 6 个月)

| Metric | Target | 怎么测 | 当前 |
|---|---|---|---|
| GitHub stars | ≥ 500 | gh api | — |
| External contributors | ≥ 5 不同人 PR merged | gh api | — |
| 中文技术媒体报道 | ≥ 1 | 微信 / 36kr / InfoQ etc | — |
| Anthropic 官方 retweet / 提及 | true | Anthropic 官推搜索 | — |
| Awesome-claude-code list 收录 | true | 该 list 自我 PR + merged | — |
| 出现在 dev tool 教程类内容里 | ≥ 1 | 推特 / YouTube 搜索 | — |

**T3 pass 条件**：3/6 达成（"established" 不需要全部，但要有 traction signal）

---

## 6. 反指标（达成这些 = 偏离目标）

| 反指标 | 阈值 | 含义 |
|---|---|---|
| 用户要求"加通知"次数 | > 10 issues / 30d | 我们 target user 群体可能不对——重新评估"被动感知" |
| 用户要求"加配置"次数 | > 5 issues / 30d | 同上，re-evaluate zero-config |
| 平均响应 issue 时间 | > 7 天 | 作者承诺没履行，影响信誉 |
| 作者 git commit 频率 | > 30 天无 commit (release 后 3 个月内) | 项目 stale 风险 |

**触发任一**：暂停 feature 开发 → 复盘 → 写 retrospective 决定方向。

---

## 7. 不度量（明确）

| 不度量 | 理由 |
|---|---|
| DAU / MAU | 完全本地无 telemetry，无法测 |
| Conversion (free → pro) | 没有 pro 层 |
| Revenue | 不商业化 |
| ARR / MRR | 同上 |
| Time-on-app | 跟产品目标反——我们要"app 隐形"不要"用户停留" |

---

## 8. 数据来源

| 数据 | 工具 | 频率 |
|---|---|---|
| GitHub stars / forks / issues / PRs | `gh api` CLI | weekly |
| Twitter analytics | twitter.com/i/account_analytics | weekly |
| HN ranking | manually note 一次 | 一次 (launch day) |
| User feedback (qualitative) | issues + 1-on-1 访谈 | ongoing |
| Personal usage stats | log file grep + dogfood retro | 14 天总结 |

---

## 9. 跟踪流程

每周末 1 次（[constitution V.3 pace](../constitution.md)）：

```bash
# 跑这套 cmd，把结果写到 docs/roadmap/metrics-YYYY-MM.md
gh api repos/<owner>/claude-code-monitor | jq '{stars: .stargazers_count, forks: .forks_count}'
gh issue list --state open --limit 100 | wc -l
gh pr list --state merged --author '!<owner>' --limit 100 | wc -l
# ...
```

→ 这是 v0.2+ 的事，MVP 阶段手动记录到 dogfood retro。

---

## 10. 报告节奏

| 时点 | 报告类型 | 受众 |
|---|---|---|
| Dogfood D14 | retrospective.md | 作者自己 |
| Alpha 结束 | alpha-report.md | 作者自己 |
| Launch D7 | first-week-blog.md | 推特 + 公众号 |
| Launch D30 | first-month-blog.md | 同上 |
| Launch D90 | first-quarter-retrospective.md | 长文 blog post |
| Launch D180 | 6-month-look-back.md | 是否进入 maintenance / pivot / sunset |

---

## 11. Cross-reference

| Topic | Where |
|---|---|
| 原始 success criteria | [product-brief.md § 6](../bmad/01-analysis/product-brief.md) |
| Tier 阈值 source | [market-research.md § 6](../bmad/01-analysis/market-research.md) |
| Release stages | [roadmap/v0.1.md § 3](../roadmap/v0.1.md) |
| Dogfood retrospective template | [guides/dogfood-retrospective-template.md](../guides/dogfood-retrospective-template.md) |
| Launch plan | [roadmap/launch-plan.md](../roadmap/launch-plan.md) |
