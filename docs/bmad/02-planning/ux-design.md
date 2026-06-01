# UX Design — Claude Code Monitor v0.1

> **BMAD Phase 2 · Planning · UX Designer output**
> **Status:** Draft → 待 Architect 引用
>
> 基于 [PRD.md](PRD.md) 的 UX requirements 章节展开。下游被 [architecture.md](../03-solutioning/architecture.md) 和 [epics/](../03-solutioning/epics/) 引用。
>
> 本文档定义所有可视元素的精确规格——尺寸、颜色、字体、间距、动效、交互态。
> 对应代码已经在 `src/style.css`（占位 MVP 风格），本文档是迭代到 v0.1 release 的目标设计。

---

## 1. Design principles

### P1 · 不打扰
- 永不弹通知
- tray icon 变化不闪烁
- 数字变化无动效（避免余光抓眼）

### P2 · 抬眼可读
- tray icon 数字 13pt（跟 macOS menubar 系统文字一致，详 § 2.3）
- 颜色对比度 ≥ WCAG AA（4.5:1 vs 系统背景）

### P3 · 零认知负担
- 列表项一眼可读（最多 4 个字段）
- 状态颜色编码（waiting=黄、working=绿、unknown=灰）跟随系统约定

### P4 · macOS 原生感
- 系统字体 SF Pro
- 系统色 (System Yellow / Green / Gray)
- 列表项分隔线用 system separator color
- popup 圆角 + 阴影跟 macOS Sonoma+ 风格

### P5 · 不变是美
- v0.1 内 UI 不变（[L2 acceptance](../../product/user-stories.md#l2--熟练期第-2-4-周)）
- 熟练用户的肌肉记忆比 visual polish 重要

---

## 2. Tray icon

### 2.1 Anatomy

```
┌─────────────────┐
│  [icon]  [N]    │   ← menubar 单行高度（22pt 在标准 macOS）
└─────────────────┘
```

- **icon**: 32×32 模板图标（template image），macOS 自动按 light/dark mode 着色
- **N**: waiting session 数（0 = 不显示数字仅显示 icon；≥1 显示数字）

### 2.2 Icon spec

| 属性 | 值 |
|---|---|
| 尺寸 | 32×32 px (1x), 64×64 (2x retina) |
| 格式 | PNG, RGBA, template-ready (黑色 + alpha) |
| 视觉 | MVP: 占位实心圆点；v1.0: 设计师定制（建议：眼睛轮廓 / 监视器轮廓 / 圆形带切口） |
| 对齐 | 居中，安全区 4px |

**MVP 占位**：已生成 `src-tauri/icons/icon.png`（32×32 黑色实心圆）。

### 2.3 Number rendering

| 状态 | 显示 |
|---|---|
| waiting = 0 | 不显示数字，仅图标 |
| waiting = 1-99 | 显示数字 |
| waiting ≥ 100 | 显示 "99+" |

字体：SF Pro Text Regular 13pt（跟 menubar 系统文字一致）
颜色：`labelColor`（系统自动 light/dark）
位置：icon 右侧，间距 4px

### 2.4 No animation

- 数字变化无 fade/scale/slide
- 不闪烁
- 不变色（永远 system label color）

### 2.5 State indicator (考虑过，**不做**)

不做：
- tray 图标本身不因为有 waiting 而变色
- 不加红点 / 黄点 / pulsing dot
- 全靠数字传达 "有 waiting"

→ 理由：参考 [decision-log ADR-004 永不通知红线](decision-log.md#adr-004--永不通知产品红线)，状态变色 = 视觉打断

---

## 3. Popup window

### 3.1 Dimensions

| 属性 | 值 |
|---|---|
| 宽度 | 360pt（固定） |
| 高度 | 最小 200pt（empty state）；最大 480pt；列表项约 54pt/项，header 40pt → popup 内可容纳 ~8 项不滚，**超过 8 项进入滚动**（[user-stories E4](../../product/user-stories.md#e4--极多-session10) 已同步） |
| 圆角 | 10pt（跟 macOS Sonoma popover 一致） |
| 阴影 | 系统默认 popover shadow |
| 背景 | `windowBackgroundColor`（自动 light/dark） |
| 边框 | 1px 系统 separator color |

### 3.2 Window behavior

| 行为 | 规则 |
|---|---|
| 位置 | MVP: 默认位置（webview 启动位置）；v0.2+: 锚定到 tray icon 下方 |
| 总在最前 | `alwaysOnTop = true` |
| 装饰 | 无标题栏 (`decorations = false`) |
| 可缩放 | 不可 (`resizable = false`) |
| 隐藏 | 任务栏不显示 (`skipTaskbar = true`) |
| 失焦 | MVP: 保持显示；v0.2+: 失焦自动 hide |

### 3.3 Layout 层级

```
┌──────────────────────────────────────────┐ ← popup
│ ┌──────────────────────────────────────┐ │
│ │  Header (40pt)                       │ │ ← 14px font, 12px padding
│ │  [title]              [refresh btn]  │ │
│ └──────────────────────────────────────┘ │
│ ┌──────────────────────────────────────┐ │
│ │                                      │ │
│ │  Body (variable height)              │ │ ← 列表或 empty state
│ │                                      │ │
│ │  ┌─────────────────────────────────┐ │ │
│ │  │ List item 1                     │ │ │
│ │  ├─────────────────────────────────┤ │ │
│ │  │ List item 2                     │ │ │
│ │  ├─────────────────────────────────┤ │ │
│ │  │ ...                             │ │ │
│ │  └─────────────────────────────────┘ │ │
│ │                                      │ │
│ └──────────────────────────────────────┘ │
└──────────────────────────────────────────┘
```

---

## 4. Header

### 4.1 Layout

```
┌────────────────────────────────────────┐
│                                        │ 12pt
│  Claude Code Monitor      [refresh]    │
│                                        │ 12pt
└────────────────────────────────────────┘
```

| 元素 | 字体 | 颜色 | 说明 |
|---|---|---|---|
| Title "Claude Code Monitor" | SF Pro Text Semibold 13pt | `labelColor` | 永远显示 |
| Refresh button | SF Pro Text Regular 12pt | `labelColor` | 文字按钮，无边框，点击触发立即 refresh |

### 4.2 Refresh button behavior

| 状态 | 视觉 |
|---|---|
| 默认 | 文字 "refresh"，无背景 |
| Hover | 浅灰背景 (`controlBackgroundColor`) |
| Pressed | 暗灰背景 |
| Loading（refresh 进行中）| 文字变为 "..."，禁用点击 |

→ Loading 状态用文字变化，**不用 spinner**（避免视觉抖动）。

---

## 5. List item

### 5.1 Collapsed anatomy

```
┌────────────────────────────────────────┐
│ 10pt                                   │
│  ┌─────────┐                           │
│  │ name    │ ╌╌╌  status · 3min        │ ← row 1
│  └─────────┘                           │
│   message preview (single line) ...    │ ← row 2
│ 10pt                                   │
└────────────────────────────────────────┘
```

### 5.2 Spec

| 元素 | 字体 | 颜色 | 备注 |
|---|---|---|---|
| name (cwd 末段名) | SF Pro Text Medium 13pt | `labelColor` | 截断 ellipsis；最大宽度 240pt |
| status text | SF Pro Text Regular 11pt | 状态色 | 见 5.3 |
| duration ("3min") | SF Pro Text Regular 11pt | `secondaryLabelColor` | 仅 waiting 状态显示 |
| separator (`·`) | SF Pro Text Regular 11pt | `tertiaryLabelColor` | name 跟 status 之间 |
| message preview | SF Pro Text Regular 12pt | `secondaryLabelColor` | 单行 ellipsis；最多 ~50 字 |

### 5.3 Status colors

| Status | 文字 | 背景 (badge) | 文字颜色 (badge 内) |
|---|---|---|---|
| waiting | "waiting" | `systemYellow` 20% opacity | `systemYellow` darken to 60% |
| working | "working" | `systemGreen` 20% opacity | `systemGreen` darken to 60% |
| unknown | "unknown" | `systemGray` | `secondaryLabelColor` |

Badge 样式：
- 圆角 4pt
- padding: 2pt vertical / 6pt horizontal
- font-size: 11pt

### 5.4 Spacing

| 间距 | 值 |
|---|---|
| List item 上下 padding | 10pt |
| List item 左右 padding | 14pt |
| Row 1 → Row 2 间距 | 4pt |
| 列表项间分隔线 | 1px `separatorColor` |

### 5.5 Hover state

- 整行背景变 `controlBackgroundColor`（浅灰）
- cursor: pointer
- 鼠标离开恢复

### 5.6 Active/pressed state

- 整行背景变 `controlAccentColor` 10% opacity
- 短暂（< 100ms）

---

## 6. Expanded state (展开消息)

### 6.1 Trigger

点击列表项 → 该项下方展开消息全文。再点同项收起；点其他项收起当前并展开新项。

### 6.2 Layout

```
┌────────────────────────────────────────┐
│  name        ╌╌╌  waiting · 3min       │ ← Row 1 (collapsed header 不变)
│  preview line ...                      │ ← Row 2 (preview, optionally 隐藏)
├────────────────────────────────────────┤
│                                        │ 8pt
│  ┌──────────────────────────────────┐  │
│  │                                  │  │
│  │  Full assistant message body     │  │ ← 完整内容
│  │  ...                             │  │
│  │  ...                             │  │
│  │                                  │  │
│  │  [Bash] git status               │  │ ← tool_use 简化展示 (MVP)
│  │                                  │  │
│  └──────────────────────────────────┘  │
│                                        │ 12pt
└────────────────────────────────────────┘
```

### 6.3 Spec

| 元素 | 字体 | 颜色 | 备注 |
|---|---|---|---|
| 展开消息正文 | SF Mono Regular 12pt | `labelColor` | mono 字体便于阅读代码块 |
| tool_use 块 (MVP) | SF Mono Regular 12pt | `secondaryLabelColor` | 格式 `[ToolName] short args` |
| tool_use 块 (v0.2+) | SF Mono + 语法高亮 | varies | 按工具类型完整格式化 |

### 6.4 高度策略

- 展开消息内容 ≤ 200pt：popup 整体自适应增高
- 内容 > 200pt：popup 总高度封顶 480pt，展开区域内部可滚动
- 滚动条样式：macOS 默认（auto-hide）

### 6.5 Animation

- 展开 / 收起：fade + height transition, 150ms ease-out
- 内容 fade-in 50ms 延迟，避免高度变化时内容闪烁

→ 这是 popup **内容区**的唯一动效。整个 app 的全部 4 种动效见 [§ 11 Animation budget](#11-animation-budget)。
→ tray icon 和列表项排序变化无任何动效（P1 不打扰）。

---

## 7. Empty state

### 7.1 Trigger

列表为空（无 claude session 运行时）

### 7.2 Layout

```
┌────────────────────────────────────────┐
│                                        │
│                                        │
│       no claude sessions running       │
│                                        │
│  start a session with `claude` in      │
│            your terminal               │
│                                        │
│                                        │
└────────────────────────────────────────┘
```

### 7.3 Spec

| 元素 | 字体 | 颜色 |
|---|---|---|
| 主文案 "no claude sessions running" | SF Pro Text Regular 13pt | `secondaryLabelColor` |
| Hint "start a session with `claude` in your terminal" | SF Pro Text Regular 12pt | `tertiaryLabelColor` |
| 垂直居中 | — | popup 中央 |

### 7.4 No CTA, no illustration

- 不画 illustration（违反 P5 不变是美 + 增加 maintenance cost）
- 不放 "Get started" 按钮（用户已经知道怎么开 terminal）

---

## 8. Typography summary

| 用途 | Font / Weight / Size |
|---|---|
| Title (header) | SF Pro Text Semibold 13pt |
| Body / name | SF Pro Text Medium 13pt |
| Secondary text (preview / hint) | SF Pro Text Regular 12pt |
| Caption (status / duration) | SF Pro Text Regular 11pt |
| Expanded message body | SF Mono Regular 12pt |

→ **不引入自定义字体**。SF 系列是 macOS 自带，零下载。

---

## 9. Color summary

依赖 macOS dynamic system colors（自动 light/dark）：

| Token | 用途 |
|---|---|
| `labelColor` | 主文字 |
| `secondaryLabelColor` | 次文字 |
| `tertiaryLabelColor` | 三级文字 / hint |
| `windowBackgroundColor` | popup 背景 |
| `controlBackgroundColor` | hover 背景 |
| `controlAccentColor` | pressed 背景（系统强调色） |
| `separatorColor` | 分隔线 |
| `systemYellow` / `systemGreen` / `systemGray` | 状态徽章 |

实现层：CSS variables 由 `prefers-color-scheme` media query 切换。

---

## 10. Accessibility (a11y)

| 项 | 要求 |
|---|---|
| 对比度 | 所有文字 WCAG AA (4.5:1) |
| 键盘导航 | popup 内 Tab 切换列表项；Enter 触发展开；Esc 关闭 popup |
| VoiceOver | tray icon `accessibilityLabel = "Claude Code Monitor, N waiting"` |
| 列表项 ARIA | role="button", aria-expanded="true/false" |
| 焦点环 | popup 内焦点用系统默认蓝色焦点环 |

### MVP scope vs v0.2+ 蓝图

**MVP 必做**：
- 对比度 ≥ WCAG AA
- VoiceOver: tray icon `accessibilityLabel = "Claude Code Monitor, N waiting"`

**v0.2+ 蓝图（上面表格列的是 v0.2+ 目标，非 MVP 规范）**：
- 键盘导航（Tab / Enter / Esc）
- 列表项 ARIA role
- 焦点环 visible

→ MVP 主要 click-driven，键盘可访问性 v0.2+ 补。

---

## 11. Animation budget

整个 app 的动效总清单（不超过这些）：

| 动效 | 时长 | 缓动 | 说明 |
|---|---|---|---|
| Popup 显示 | 150ms | ease-out | fade + scale 0.95→1 |
| Popup 隐藏 | 100ms | ease-in | fade |
| 列表项 hover 变色 | 100ms | linear | 背景色过渡 |
| 列表项展开/收起 | 150ms | ease-out | 高度 + opacity |

**不做的动效**：
- tray icon 任何变化
- 列表项排序变化（直接重排，无动画）
- 数字变化
- session 进出列表（瞬移，无 transition）——MVP 允许 jumpy，v0.2+ 看
- spinner / loading 动画（refresh 用文字状态）

---

## 12. Responsive behavior

### 12.1 macOS dock 位置

- Dock 在底/左/右：tray icon 仍在 menubar，无影响
- Menubar hidden（全屏 app）：tray 不可见，但 app 仍跑，退出全屏即恢复

### 12.2 多 monitor

- macOS 决定 tray 在哪个 screen（通常是主 screen）
- popup 跟随 tray 所在 screen

### 12.3 Notch (M1/M2 Pro / Air)

- tray icon 自动避开 notch
- popup 不被遮挡（macOS 自动处理 menubar 区域）

---

## 13. Design assets

| 资产 | 位置 | 状态 |
|---|---|---|
| Tray icon 32×32 PNG | `src-tauri/icons/icon.png` | MVP 占位（黑色实心圆） |
| Tray icon 64×64 @2x | `src-tauri/icons/icon@2x.png` | **作者待办** |
| App bundle icon (.icns) | `src-tauri/icons/icon.icns` | **作者待办** |
| DMG background | `src-tauri/icons/dmg-bg.png` | v0.2+ |
| README screenshot | `docs/assets/screenshot.png` | **作者待办**（release 前） |

---

## 14. Out of scope for UX (MVP)

- 主题切换（违反 R2 零配置）
- 字体大小可调（系统级缩放足够）
- 列表项右键菜单（如 "copy cwd path"）—— v0.2+ 评估
- 列表项拖拽排序（用户排序违反 R3）
- 自定义状态颜色（违反 R2）
- 列表过滤/搜索（违反 "now not history" 心智，列表本来就短）
- popup 位置记忆（macOS 会自动记 webview window 位置）

---

## 15. Open questions

> 注：以下都是 **v0.2+ 评估项**，MVP 行为按 § 3.2 spec 已确定（"失焦保持显示"）。这里只是预留思考。

1. **Popup 是否在失焦时自动 hide？** MVP 按 § 3.2 spec = 不自动 hide；v0.2+ 收集反馈再决定改不改。
2. **是否给 tray icon 加 ⌥+点击 弹 debug menu？** v0.2+ 评估。
3. **键盘快捷键全局触发 popup？** 违反 R2 零配置（要选键），MVP 不做；v0.2+ 也不建议。
4. **暗色模式 separator 是否需要更明显？** 等真机实测。

---

## 16. Sign-off

| 角色 | 状态 |
|---|---|
| UX Designer | ✅ 规范完整 |
| PM | ⏳ 待 review，确认跟 PRD FR 一致 |
| Architect | ⏳ 待 review，确认实现可行（性能 budget 跟 NFR-P1 不冲突） |
| 作者 | ⏳ 待实测 dogfood |

→ 下一步：[architecture.md](../03-solutioning/architecture.md)
