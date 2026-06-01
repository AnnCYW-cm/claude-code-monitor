# Empty State — Visual Design

> **Status:** v0.1 design freeze
> **Cross-ref:** [ux-design § 7](../../bmad/02-planning/ux-design.md), [user-stories E1](../../product/user-stories.md#e1--首次安装空状态), [scenarios S4](../../product/scenarios.md)
> **Implementation:** [S-010](../../bmad/03-solutioning/epics/story-010-empty-state.md)

---

## 1. Mockup

### 1.1 Light mode

```
┌──────────────────────────────────────────────┐
│  Claude Code Monitor              refresh    │  ← Header（始终显示）
├──────────────────────────────────────────────┤
│                                              │
│                                              │
│                                              │
│                                              │  ↑
│       no claude sessions running             │  │ vertical center
│                                              │  │
│  start a session with `claude` in            │  │
│            your terminal                     │  ↓
│                                              │
│                                              │
│                                              │
└──────────────────────────────────────────────┘
                  360pt × 200pt (min height)

文字色：
- "no claude sessions running" → secondaryLabelColor
- "start a session..." (hint) → tertiaryLabelColor
```

### 1.2 Dark mode

视觉布局相同，颜色 token 自动反色（macOS dynamic system colors）：

```
┌──────────────────────────────────────────────┐
│  Claude Code Monitor              refresh    │
├──────────────────────────────────────────────┤
│                                              │
│                                              │
│       no claude sessions running             │
│                                              │
│  start a session with `claude` in            │
│            your terminal                     │
│                                              │
│                                              │
└──────────────────────────────────────────────┘
```

无需独立 dark mode CSS——CSS variables 由 `prefers-color-scheme` 自动切换。

---

## 2. 字段 spec

| 元素 | 字体 | 颜色 | 行为 |
|---|---|---|---|
| 主文案 "no claude sessions running" | SF Pro Text Regular 13pt | `secondaryLabelColor` | 居中（水平 + 垂直） |
| Hint "start a session with \`claude\` in your terminal" | SF Pro Text Regular 12pt | `tertiaryLabelColor` | 居中，主文案下方 |
| 间距 主文案 → hint | 12pt | — | — |
| 容器 padding | 24pt 上下 | — | empty state 容器自身 |

`\`claude\`` 是 inline code 显示——稍微深色 + 微 background：
- 字体 SF Mono Regular 12pt
- 背景 `controlBackgroundColor` 50% opacity
- 圆角 3pt
- padding 1pt 3pt

---

## 3. 文案约定

### MVP（v0.1）

```
no claude sessions running

start a session with `claude` in your terminal
```

### Why English

- target user 是开发者，英文 OK
- 简短易读
- 跟 ChatGPT / GitHub 等开发工具的 empty state 调性一致

### 不做的文案变体

- ❌ "👋 Hello! No sessions running yet" — 太活泼，违反 P5 不变是美
- ❌ "Run `claude` in any terminal to get started" — 太长，hint 风格已经合适
- ❌ 多语言（中文 / 日文 / 法文）— MVP 仅英文，i18n v1.0+ 评估

---

## 4. 视觉空间分布

```
Popup 高度 200pt 时的布局：
┌──────────────────────────────────────────────┐
│  Header (40pt)                               │  ← 固定
├──────────────────────────────────────────────┤
│                                              │  ↑ 24pt top padding
│                                              │  
│       [main text 16pt]                       │  ← 主文案 (line-height 16pt for 13pt font)
│                                              │
│              [12pt gap]                      │
│                                              │
│  [hint text 30pt across 2 lines]             │  ← hint 可能换行
│                                              │
│                                              │  ↓ 24pt bottom padding
└──────────────────────────────────────────────┘
```

实际高度计算：
- header: 40pt
- top padding: 24pt
- 主文案: 16pt
- gap: 12pt
- hint (2 行): ~30pt
- bottom padding: 24pt
- = 146pt

→ < 200pt min height，popup 自然保持 200pt（不会收缩到 146pt）。

---

## 5. 什么时候显示 empty state

| 情境 | 是否 empty state |
|---|---|
| 用户首次安装 app，无任何 claude session | ✓ |
| 用户开过 session 但都退出了 | ✓ |
| 所有 session 都 status = unknown | ✗（仍显示列表，每项 unknown） |
| sysinfo 返回空（API 异常） | ✓（fallback） |
| `list_sessions` IPC 抛错 | ✗（不切到 empty，保持上次 stale 数据 + console.error） |

→ 触发条件：`sessions.length === 0`（无论原因）

---

## 6. 状态切换 animation

```
当 list 从 N 项 → 0 项：
- 列表内容 fade out 100ms
- 同时 empty state fade in 100ms (slight 50ms overlap OK)

当 list 从 0 项 → N 项：
- empty state fade out 100ms
- 列表 fade in 100ms
```

实际 MVP 简化：**无 transition，瞬时切换**（[ux-design § 11 budget](../../bmad/02-planning/ux-design.md) 不在 animation 列表）。

→ 切换瞬间可能 flicker——可接受，因为切换不频繁。

---

## 7. 没做的设计变体

| 不做 | 理由 |
|---|---|
| Illustration / 卡通图 | 违反 [P5 不变是美](../../bmad/02-planning/ux-design.md) + 增加 maintenance（图片需要 light/dark 各一套） |
| "Get started" 按钮 | 用户已经知道怎么开 terminal，按钮多余 |
| 教程链接 | onboarding 靠 R2 零配置 + 自然交互，文档 + tutorial 都不在 app 里 |
| 历史 session 链接 | 违反 [R5 不显示历史](../../product/user-stories.md#r5--不该展示历史已退出-session) |
| 配置入口 | 违反 [R2 零配置](../../product/user-stories.md#r2--不该要求配置才能用) |
| social link / about | app 内不该有这些 |

---

## 8. Implementation checklist

实现 [S-010](../../bmad/03-solutioning/epics/story-010-empty-state.md) 时对照本文件：

- [ ] `sessions.length === 0` 触发 empty state render
- [ ] 主文案 "no claude sessions running" 居中
- [ ] hint "start a session with `claude` in your terminal" 居中，主文案下方 12pt
- [ ] hint 里 `claude` 是 inline code 样式（mono + bg）
- [ ] light/dark mode 各实测
- [ ] popup 容器高度保持 200pt min（不收缩到 146pt）
- [ ] 列表 ↔ empty 切换无 transition（瞬时）
- [ ] 跟 [scenarios S4 T+0:15 mockup](../../product/scenarios.md) 一致

---

## 9. CSS skeleton

```css
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px 14px;
  min-height: 160px; /* 配合 header 40pt 凑 200pt min */
  text-align: center;
}

.empty-state .main {
  font: 13px var(--sf-pro);
  color: var(--secondary-label);
  margin-bottom: 12px;
}

.empty-state .hint {
  font: 12px var(--sf-pro);
  color: var(--tertiary-label);
}

.empty-state .hint code {
  font: 12px var(--sf-mono);
  background: var(--control-bg);
  padding: 1px 3px;
  border-radius: 3px;
}
```

HTML skeleton:

```html
<div class="empty-state">
  <div class="main">no claude sessions running</div>
  <div class="hint">start a session with <code>claude</code> in your terminal</div>
</div>
```
