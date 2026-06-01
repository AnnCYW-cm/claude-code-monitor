# Market Research — Claude Code Monitor

> **BMAD Phase 1 · Analysis · Analyst output**
> **Status:** Draft (2026-05-17，作者主观调研，未做用户访谈)
>
> 这份文档评估市场背景、竞品、定位、风险。对一个 single-author 开源工具，"market" 不是商业意义上的市场，而是「用户基础是否存在、有多大、能不能触达」。

---

## 1. Market framing

### 1.1 用户基础规模估算

**Claude Code 用户规模**（公开信息推断）：
- Anthropic 没公开 Claude Code DAU/MAU
- 间接信号：GitHub 上 `claude-code-action`、`claude-code-sdk` 等周边项目 star 数（万级）
- 推特/X 上 "@claudeai" 提及 + Claude Code 内容互动率
- 推断：Claude Code 当前用户约 **10 万 - 50 万** 量级（2026 年）

**Heavy CC user 子集**（同时跑 ≥3 session 的）：
- 推断为 5-15% 的 Claude Code 用户
- 推断绝对值：**5,000 - 75,000 人**

**重叠：macOS 用户**：
- Claude Code 是 CLI，跨平台。但开发者群体里 macOS 占比 ~60-70%（StackOverflow Developer Survey 常年数据）
- Heavy CC user 中 macOS 占比预计更高（≥70%，因为重度开发者 Mac 比例更高）

**目标 TAM（可触达）**：约 **3,500 - 50,000 人**。

→ 这是个**小众但精准**的市场。商业化困难，但作为开源工具积累信誉、扩大作者技术影响力，规模足够。

### 1.2 用户痛点强度

不是所有 Heavy CC user 都觉得"找不到 waiting 的 session" 是痛点。痛点强度跟以下相关：

| 因素 | 痛点强度推测 |
|---|---|
| 同时 session 数 ≥5 | 高 |
| 一天用 CC 时间 ≥4 小时 | 高 |
| 同时多项目并行 | 高 |
| 用 tmux/screen 持久化 session | 中 |
| 单一项目深度使用 | 低（少切换） |
| 偶尔用 CC | 低（不会忘） |

→ **核心 target user 是「多项目并行 + 长时间使用 + 不用 tmux 自己管」的子群体**。占 Heavy CC user 估计 30-50%，即 **1,000 - 25,000 人**。

---

## 2. Competitive landscape

### 2.1 直接竞品（同问题域）

| 工具 | 形态 | 优势 | 劣势 | 状态 |
|---|---|---|---|---|
| **None known** | — | — | — | 截至 2026-05 未发现专门为 Claude Code 多 session 监控做的开源工具 |

→ **没有直接竞品**——这是个未被服务的微观需求。

风险分析：「没有竞品」可能是因为：
- (a) 没人发现这个痛点 → 真空市场，机会
- (b) 痛点强度不够大，用户能忍 → 市场太小
- (c) Anthropic 自己快要发布 official monitor → 撞车

(a) 和 (b) 都需要 beta 测试验证。(c) 是真实威胁——Anthropic 加官方功能我们就被吃掉。

**应对**：保持小、保持开源、保持极简，做到 Anthropic 不屑做的"利基 polish"。

### 2.2 邻接竞品（相似问题域）

| 类别 | 代表工具 | 跟我们的关系 |
|---|---|---|
| **通用菜单栏 status** | iStat Menus、Bartender、Stats（开源） | 提供 menubar utility 的 UX 范式，但不监控 LLM session |
| **进程监控** | htop、btop、Process Monitor.app | 看 CPU/RAM，不看 LLM context |
| **LLM 通用 dashboard** | Anthropic Console、OpenAI Dashboard | 看 API usage / cost，不看本地 CLI session |
| **AI assistant orchestration** | Cursor、Continue、Aider | 自己是 AI 工具，不监控 Claude Code |
| **Notification batchers** | Streamtools、Hammerspoon scripts | 通用通知工具，需要用户自己 hack |
| **tmux dashboard plugins** | tmux-resurrect、tmux-continuum | 持久化 tmux session，不知道 CC 状态 |

→ 我们填的是「menubar utility × LLM session awareness × 跨终端 emulator 中立」的交叉空白。

### 2.3 DIY 解决方案（用户自己 hack 的）

调研发现 Reddit r/ClaudeAI、HN、Twitter 上有用户讨论过类似痛点，常见 workaround：

| Workaround | 局限 |
|---|---|
| `watch -n 5 'ls -lt ~/.claude/projects/*.jsonl \| head'` 写个 shell loop | 文本输出难读、要主动看终端 |
| terminal-notifier hooked into shell prompt | 跟具体 shell 配置耦合、有打断 |
| iTerm trigger（看到特定文本时通知） | 跟 iTerm 强绑定、只看当前可见的 tab |
| Tmux pane 持续 `tail -f` JSONL | 占用 pane、要解析 JSONL |
| Just remember and check manually | 累 |

→ 没有 zero-config 的工具，全是 hack。这印证了 [product-brief](product-brief.md) 的"zero-config" R2 价值主张。

---

## 3. Positioning

### 3.1 一句话定位

> The "menubar dot" for parallel Claude Code sessions — glance to know who's waiting, click to know why, never interrupt.

### 3.2 定位矩阵

```
                  打断用户
                       ↑
                       │
   Notification        │
   batchers            │   Slack/邮件
                       │
   ────────────────────┼────────────────────→  深度集成
   被动感知            │                       UI
                       │
   ★ Claude Code       │   Cursor / IDE
     Monitor           │   sidebar
                       │
                       ↓
                  不打扰用户
```

我们位于「不打扰 × 不深度集成」象限——这个象限唯一的产品。

### 3.3 跟「Anthropic 官方未来可能出的工具」的定位差

如果 Anthropic 出官方 monitor，必然：
- 深度集成（直接 hook Claude Code lifecycle，不靠 JSONL parse）
- 跨平台（Mac / Linux / Windows）
- 跟 Anthropic Console 联动（云端 dashboard）
- 通知/badge/sound 都做（覆盖所有用户偏好）

我们差异化：
- macOS only，极简
- **明确不做通知**——这是 anti-feature 红线
- 完全本地，不外联
- 文档驱动开发，contributor 友好

→ Anthropic 出官方版我们也不死，因为 anti-features（不做通知）是我们的差异点。

---

## 4. Adoption / distribution 策略

### 4.1 触达路径

| 渠道 | 预计触达 | 成本 | 优先级 |
|---|---|---|---|
| GitHub README + Topics | 自然搜索 | 0 | P0 |
| Show HN | 1 次曝光机会 | 0（要选好 timing） | P0 |
| 作者推特发布 | 1k-5k 触达 | 0 | P0 |
| Anthropic Discord 社区分享 | 准确目标用户 | 0 | P1 |
| Awesome Claude Code 类列表 | 长尾流量 | 0 | P1 |
| Reddit r/ClaudeAI / r/macapps | 中等规模、噪声大 | 0 | P2 |
| Product Hunt | 噪声大、用户群不准 | 0 | P3 |
| 中文社区（微信群 / 知乎） | 作者社交网络 | 0 | P2 |

### 4.2 transitions

| 用户旅程阶段 | 触发 | 关键摩擦 |
|---|---|---|
| 知道存在 | 推特/HN/GitHub | 看到一句话能 get 价值 |
| 决定试 | README quick start | 30 秒内决定值不值得 build from source |
| 装上 | brew cask / build from source | Gatekeeper（[F4](../../product/user-stories.md#f4--gatekeeper-拦截)） |
| 第一次"啊有用" | T+17 分钟（[S4](../../product/scenarios.md)） | 必须有空 state 引导 |
| 留存 | 自然嵌入工作流 | 不打扰、不维护 |

→ **核心摩擦在 Gatekeeper**。MVP 没签名/没公证，用户卡在「无法打开」是最大流失点。install.md 必须写到位。

### 4.3 推荐发布节奏

| 阶段 | 时机 | 内容 |
|---|---|---|
| **Closed alpha** | 完成 MVP 后 1 周 | 推给 5-10 个 Heavy CC user 朋友，访谈 |
| **Public release** | alpha 反馈处理完 | 推特 + Show HN + Anthropic Discord |
| **Homebrew cask** | 收到 ≥20 个 GitHub star 后 | brew tap 提交 |
| **稳定期** | 3 个月不再大改 | 写 retrospective 博客 |

---

## 5. Risks

| 风险 | 概率 | 影响 | 缓解 |
|---|---|---|---|
| Anthropic 出官方版本 | 中 | 高（吃掉用户） | 差异化"anti-features"、积极开源博取信誉 |
| Claude Code JSONL 格式变 | 中 | 高（功能坏） | spec/jsonl-schema.md 锁定依赖字段、CI 自动测试 |
| 用户群比预期小 | 中 | 中（影响留存信号） | beta 期就调研规模，及时缩范围 |
| 单作者 burnout，项目 stale | 中 | 中（开源信誉受损） | 文档先行让别人能接手、明确 maintenance 节奏 |
| 安全/隐私质疑（读 ~/.claude/） | 低 | 中 | README 透明说明读什么、不上传任何东西 |
| 性能问题 (sysinfo 太慢) | 低 | 高（用户卸载） | architecture + 测试覆盖 |
| macOS 15+ Gatekeeper 越来越严 | 高 | 中 | 找赞助签名 / 加入 Apple Developer Program |

---

## 6. Success metrics

不是商业 metric，是「这个项目算不算成功」的判定标准：

| Tier | 标准 | 达成义意 |
|---|---|---|
| **最低**：作者自己用 | 14 天不删 + 每天打开 ≥5 次 | 解决了真痛点 |
| **基础**：开源 1 month | ≥10 stars + ≥3 issues from 真实用户 | 有用户基础 |
| **理想**：开源 3 months | ≥100 stars + ≥5 contributors PR | 项目自主增长 |
| **超额**：开源 6 months | Anthropic 官方提及 / Homebrew cask 通过 / 中文技术媒体报道 | 成为 Claude Code 周边经典 |

---

## 7. Open questions（待 PRD 决策）

1. **是否支持 Linux？** Claude Code 在 Linux 也有用户，但菜单栏概念不通用。MVP 拒。v0.2+ 看反馈。
2. **是否做 Windows 任务栏版本？** Windows 上 CC 用户更少。v0.2+ 看反馈。
3. **是否开放扩展点？**（让用户写 plugin 加 status / column）。MVP 拒——会带来 R2 "zero config" 妥协。
4. **作者维护承诺**：写在 README 还是文档里？多久 review issue？建议「每周末一次 office hour」明示。

---

## 8. Conclusion

- **市场**：3,500 - 50,000 人的精准小众市场。商业化不可行，但作为开源声誉项目和 portfolio 价值充足。
- **竞品**：直接无对手，邻接有几个但不重叠。最大威胁是 Anthropic 出官方版本。
- **定位**：「不打扰 × 不深度集成」象限唯一产品。差异化靠 anti-features。
- **触达**：GitHub + 推特 + HN + Anthropic Discord 四渠道足以覆盖 90% target users。
- **风险**：JSONL 格式变 + Gatekeeper 摩擦 + 作者 burnout 是 top 3。

→ 进入下一步：[product-brief.md](product-brief.md) 收敛成正式 brief。
