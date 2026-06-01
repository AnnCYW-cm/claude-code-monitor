# Launch Plan — v0.1 Public Release

> **Status**：Draft（等 v0.1 done 当天最后定稿）
> **Audience**：作者自己（执行 launch 时按本文档逐条做）

---

## 1. 发布前 checklist

按 [v0.1.md § 6](v0.1.md) 全过 + 下面这些 launch-specific：

### 必备产物

- [ ] GitHub Release v0.1.0 创建（含 DMG + SHA-256 + release notes）
- [ ] [CHANGELOG.md](../../CHANGELOG.md) v0.1.0 entry 填完
- [ ] [项目根 README](../../README.md) 加 screenshot（[ux-design § 13 design assets](../bmad/02-planning/ux-design.md) flag 的 **作者待办**）
- [ ] [install.md](../guides/install.md) 至少 macOS 26 路径实测过
- [ ] GitHub repo metadata:
  - Description: "A macOS menubar app that tells you which of your running Claude Code sessions is waiting for input."
  - Topics: `macos`, `menubar`, `tauri`, `rust`, `claude`, `claude-code`, `developer-tools`
  - Social preview image (1280×640，未来生成)

### 文档 placeholder sweep（release 前必做）

文档里 13 处 `<owner>` 占位 + 5 处 "GitHub Release URL 待填" 必须替换为真实值：

```bash
# 找所有 <owner> 占位
grep -rn "<owner>" docs/ README.md CHANGELOG.md SECURITY.md .github/

# Sweep（假设 owner = caiyiwen）
grep -rl "<owner>" . | xargs sed -i '' 's|<owner>|caiyiwen|g'

# 填 GitHub Release URL（DMG 上传后）
# 在 docs/guides/install.md + SECURITY.md 找 "URL 待 v0.1 release 后填入" 处填上

# 推特文案里 "作者推特" 替换为真实 handle
grep -rn "作者推特\|@<your-handle>" docs/roadmap/launch-plan.md
```

完成 sweep 才 push tag + release。

### 朋友圈预热

- [ ] Closed alpha 用户已通知 "Public release 中，请帮转推"

---

## 2. 渠道（按 [market-research § 4](../bmad/01-analysis/market-research.md) 4 P0 / 2 P1）

### P0-1 · GitHub README + Topics

**核心**：repo description 一句话 + topics SEO + README 第一屏视觉

→ 已在准备阶段。

### P0-2 · Show HN

**最佳 timing**：周二/周三/周四上午 10:00-11:00 EST（流量高峰，避周末）

**Title**（candidates，30 字符以内）：

| 候选 | 评估 |
|---|---|
| `Show HN: Claude Code Monitor – Menubar app for parallel CC sessions` | 直白，SEO 友好 |
| `Show HN: I built a menubar app to track parallel Claude Code sessions` | 个人化 |
| `Show HN: A passive-awareness menubar for Claude Code` | 强调"被动感知" |

**推荐**：`Show HN: Claude Code Monitor – Menubar app for parallel CC sessions`

**Post body**（draft，发布前调）：

```
I run 3-8 Claude Code sessions in parallel for different tasks every day,
and I kept losing track of which one was waiting for input. Workarounds
(shell loops, iTerm triggers, tmux tail) all required active polling.

Claude Code Monitor is a tiny menubar app that just shows:
- A counter of waiting sessions (e.g., `👁 3`)
- A popup list with cwd / status / how long it's been waiting / last message

Anti-features (won't change):
- No notifications, no sound, no badges — never interrupts you
- Zero config — no login, no API key, no settings panel
- macOS only, completely local (no network)

Built with Tauri 2.x + Rust + vanilla TypeScript. ~10MB bundle.

Open source MIT: https://github.com/<owner>/claude-code-monitor

Docs cover BMAD-method process (PRD, decision log, 13 dev stories, etc) —
might be useful as a reference for solo open-source dev tools.

Happy to answer questions about: Tauri menubar gotchas, Claude Code JSONL
parsing, why I rejected fs-watcher in favor of polling.
```

**预期反馈话题**：

- "Why not just notifications?" → 解释 anti-feature philosophy
- "Linux/Windows?" → 解释 macOS only 决策（cross-emulator neutrality）
- "Anthropic Claude Code 是否会出官方版本？" → 解释差异化（anti-features）
- "JSONL parsing 怎么保稳？" → 指向 [jsonl-schema.md](../spec/jsonl-schema.md)

### P0-3 · 作者推特

**Tweet 1** (launch announce):

```
just shipped Claude Code Monitor 🎉

a tiny macOS menubar app that tells you which of your running
Claude Code sessions is waiting for input.

no notifications, no settings, no nothing — just a number you can glance at.

open source MIT: github.com/<owner>/claude-code-monitor

#ClaudeCode #DevTools
```

**Tweet 2** (hours later, behind-the-scenes):

```
TIL while building this:

- Claude Code writes per-session JSONL to ~/.claude/projects/<encoded-cwd>/
- "End of turn" signal is message.stop_reason == "end_turn" (not "no pending tool_use" 😅)
- Same session's cwd field can change mid-transcript when user `cd`s

full schema spec: github.com/<owner>/claude-code-monitor/blob/main/docs/spec/jsonl-schema.md
```

**Tweet 3** (next day, meta):

```
fun fact: this 270-line scaffold project has 12,000 lines of docs.

ratio 44:1 documentation:code 🫠

it's by design (single-author open source = future contributors need
deep context) but i'm watching whether the upfront cost pays off.

i'll write a retrospective when v1.0 ships.
```

### P0-4 · Anthropic Discord 社区

**Channel**：`#community-projects` or `#dev-tools` (whichever exists)

**Post**（短，社区不喜欢 self-promo 长文）：

```
hey - just shipped a tiny utility for those of us running multiple
parallel claude code sessions: github.com/<owner>/claude-code-monitor

menubar app, shows which session is waiting for input. zero config,
no notifications, completely local. mac only for now.

would love feedback from heavy users.
```

### P1-1 · Awesome Claude Code 类列表

找 `awesome-claude-code` GitHub list（如存在），提 PR 加入。

### P1-2 · 中文社区

**微信**：作者社交网络（蔡逸雯朋友圈 + AI 相关群）

**Post 草稿**（中文）：

```
开源了一个我自己每天用的小工具：Claude Code Monitor。

如果你同时跑 3+ 个 claude session（比如一个写代码一个跑测试一个改文档），
经常会有"某个等你的session 没察觉" 的问题。

这个工具是 macOS 菜单栏的一个小图标，告诉你有几个在等你。
点开看具体哪个、最后一条消息说啥。

没有通知（不会打扰），没有配置（装完即用），不联网（完全本地）。

GitHub: https://github.com/<owner>/claude-code-monitor

挺简单的工具，但文档写得过分认真（12K 行 doc / 270 行 scaffold）。
开源项目 + 单作者 + 想让未来的 contributor 接得住，所以...

欢迎试用 + 提 issue。
```

**知乎**：发"AI 编程"话题，标题 "如何避免在同时跑多个 Claude Code session 时漏看某个？"

### P2 · Reddit (噪音大，低优先级)

- `r/ClaudeAI` — 简化版 HN post
- `r/macapps` — 强调菜单栏 utility

### P3 · Product Hunt

⚠️ **不推荐**：dev tool 在 PH 转化差，且需要协调 "hunters"。略过。

---

## 3. Timing

```
D-7 (release 前 1 周): closed alpha 反馈处理完，install.md 实测完
D-1: 准备 launch 文案 + screenshot
D 0: 
  - 早 6:00: 创建 GitHub Release v0.1.0
  - 早 10:00 EST: Show HN 发帖
  - 早 10:30 EST: 推特发布（Tweet 1）
  - 晚 8:00: Anthropic Discord 发帖
D+1:
  - 推特 Tweet 2 (technical takeaways)
  - 处理 HN 评论 + GitHub issues
D+2:
  - 推特 Tweet 3 (meta retrospective)
  - 中文社区分发（微信/知乎）
D+7: 一周回顾 — 看 GitHub stars / issues / PRs / 用户反馈
D+30: 一月回顾 — 决定是否进 v0.2
```

---

## 4. 应对预期负面反馈

| 反馈类型 | 应对 |
|---|---|
| "Why not Linux/Windows?" | 链接 [PRD § 9 Constraints](../bmad/02-planning/PRD.md) + [market-research § 7 OQ-1/2](../bmad/01-analysis/market-research.md) |
| "Why no notifications?" | 链接 [ADR-004 永不通知红线](../bmad/02-planning/decision-log.md) + [scenarios S2 重负载场景](../product/scenarios.md) 解释"不打扰胜过提醒" |
| "为啥不直接做成 IDE 插件 / Web UI" | 链接 [brainstorming § 1.1 形态维度](../bmad/01-analysis/brainstorming.md) |
| "Anthropic 会出官方的吧" | 链接 [market-research § 2.3 + § 3.3 差异化](../bmad/01-analysis/market-research.md) |
| "为啥不签名" | 链接 [SECURITY.md](../../SECURITY.md) + [install.md Gatekeeper § Why no signing](../guides/install.md) |
| 文档太多/过度工程 | 承认+解释 [docs/README "维护负担提醒"段](../README.md) |

---

## 5. 成功标准

详 [success-metrics.md](../product/success-metrics.md) + [v0.1.md § 3.3 / 3.4 stages](v0.1.md)：

| 时间 | 标准 |
|---|---|
| Day 1 launch | HN front page (visible) + 50 GitHub stars |
| Week 1 | 10+ GitHub stars cumulative + 3+ issues from non-author |
| Month 1 | 50+ stars + 1+ external PR |
| Month 3 | 100+ stars + 5+ external contributors + Anthropic 提及（aspirational） |

---

## 6. Launch 后

- 14 天 calendar 内**不接受 feature PR**，只接 bug fix（防 scope creep）
- 30 天后整理 issue → 决定 v0.2 prioritization（[v0.2.md § 2](v0.2.md)）
- 写 retrospective 博客（"shipped my first open-source dev tool — what worked"）
- 决定下一步：v0.2 推进 vs 进入 maintenance 模式

---

## 7. 不在 launch 范围

| 不做 | 理由 |
|---|---|
| 付费层 / Pro 版 | v0.1 brief 明确不商业化 |
| 邮件 newsletter | 单作者维护负担 |
| Discord server | 等 100+ stars 再说 |
| 视频 demo | 单作者非营销专长，dev tool 让用户看 GIF/screenshot 够 |
| LinkedIn 推广 | 受众不对口 |
| 中文播客 | 时间投入 / 受众转化比例不值 |
