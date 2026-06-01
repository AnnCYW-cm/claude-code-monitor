# UI Design — Visual Mockups + Component Spec

> **Purpose**: 把 [ux-design.md](../../bmad/02-planning/ux-design.md) 的文字规范扩展成**可视的、可直接照实现的**设计文档。
>
> 阅读对象：**实现 UI 的开发者**（包括作者自己 + 未来 contributor）。
>
> 跟 [ux-design.md](../../bmad/02-planning/ux-design.md) 的边界：
> - `ux-design.md`（在 BMAD planning）：**规范**（哪些字体 / 颜色 / 间距 / 动效）
> - `ui/`（本目录）：**视觉化 + 实现 spec**（ASCII mockup + 各状态对比 + CSS skeleton + edge case）

---

## 文档清单

| # | 文件 | 内容 |
|---|---|---|
| 01 | [tray-icon.md](tray-icon.md) | Tray icon 各 waiting count / light-dark mode mockup + sizing + 切换图 |
| 02 | [popup-window.md](popup-window.md) | Popup 整体框架 mockup（多场景） + 尺寸 + 位置 + 显隐 animation 序列 |
| 03 | [list-item.md](list-item.md) | List item anatomy + 4 状态 mockup + 3 status 颜色变体 + edge case |
| 04 | [empty-state.md](empty-state.md) | Empty state mockup + 文案 + light/dark 对比 |
| 05 | [menu.md](menu.md) | 右键 native menu mockup + items + 未来扩展占位 |
| 06 | [animations.md](animations.md) | 4 个动效 timing 序列 + easing 曲线 + CSS keyframes 草稿 |
| 07 | [component-css.md](component-css.md) | 所有 CSS class + selector spec + system color variables + light/dark media query 实现 skeleton |

---

## 设计原则速查（来自 [ux-design § 1](../../bmad/02-planning/ux-design.md)）

| P | 含义 | 落到本目录意味着 |
|---|---|---|
| **P1 不打扰** | 永不弹通知、tray 不闪烁、数字变化无动效 | 所有 mockup 必须验证「视觉变化是否会抢眼」 |
| **P2 抬眼可读** | 13pt menubar 字体、对比度 ≥ WCAG AA | 所有颜色 mockup 必须 light/dark 都给 |
| **P3 零认知负担** | 一眼可读、状态颜色编码（黄/绿/灰）跟随系统约定 | 状态徽章颜色不能创新 |
| **P4 macOS 原生感** | SF Pro 字体、系统色、Sonoma+ popover 风格 | 不发明 token，全用系统 |
| **P5 不变是美** | v0.1 生命周期 UI 不变 | 本目录是 v0.1 spec 冻结版 |

---

## ASCII mockup 约定

为方便 markdown 渲染 + diff，所有 mockup 用 ASCII：

```
┌─────────────────────────────┐  ← 圆角用 ┌─┐└─┘
│  ████  Header               │  ← 实心区域用 █
│  ────────────────────────── │  ← 分隔线用 ─
│                             │
│  ▸ name    badge   1min     │  ← collapsed list item
│                             │
│  ▾ name    badge   1min     │  ← expanded list item (▾ 表示展开态)
│  ┌──────────────────────┐  │
│  │ full message...       │  │
│  └──────────────────────┘  │
│                             │
└─────────────────────────────┘
```

**注**：ASCII 是粗略示意，**像素级精确尺寸看 spec 表格**。Figma/Sketch 暂不引入（成本 vs 单作者收益不值）。

### 单位约定

文档里全用 **pt**（macOS 原生单位）。CSS 实现时按 `1pt = 1px`（webview 中 CSS pixel 跟 pt 1:1 等价，macOS retina 自动放大 2×）。

不要混淆：
- macOS native（NSStatusItem / NSMenuItem）用 **pt**
- Webview CSS 用 **px**
- 我们 spec 用 pt，CSS 实现可写 px 同值（数字相等）

## 跟现有代码 scaffold 的关系

当前 `src/main.ts` + `src/style.css` 是占位（最初 Tauri 脚手架产物）。本目录是"目标设计"——每个 [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md) / [S-009](../../bmad/03-solutioning/epics/story-009-expand-message.md) / [S-010](../../bmad/03-solutioning/epics/story-010-empty-state.md) 实施时按本目录 spec 重写。

---

## 跟 user-stories / scenarios 的关系

| user-story / scenario | 对应 UI 设计文件 |
|---|---|
| [H1 瞄一眼](../../product/user-stories.md#h1--瞄一眼判断是否切走) | [tray-icon.md](tray-icon.md) |
| [H2 弹开列表](../../product/user-stories.md#h2--弹开列表分诊优先级) | [popup-window.md](popup-window.md) + [list-item.md](list-item.md) |
| [H3 读完整消息](../../product/user-stories.md#h3--不切走也能读到关键信息) | [list-item.md § expanded state](list-item.md) |
| [H4 退出](../../product/user-stories.md#h4--退出-app) | [menu.md](menu.md) |
| [E1 empty state](../../product/user-stories.md#e1--首次安装空状态) | [empty-state.md](empty-state.md) |
| [E4 极多 session](../../product/user-stories.md#e4--极多-session10) | [popup-window.md § 滚动场景](popup-window.md) |
| [S1 典型周二](../../product/scenarios.md) | [popup-window.md § 3 session mockup](popup-window.md) |
| [S2 重负载](../../product/scenarios.md) | [popup-window.md § 8 session mockup](popup-window.md) |
| [S4 onboarding](../../product/scenarios.md) | [empty-state.md](empty-state.md) + [tray-icon.md § 初次启动](tray-icon.md) |

---

## 未做（明确 out of scope）

- **Figma/Sketch 源文件**：单作者 + 文本驱动开发，不引入设计工具
- **图标设计稿**（v1.0 设计师定制）：MVP 占位黑色圆点
- **DMG 安装界面背景** ：v0.2+
- **App 启动 splash screen**：MVP 无 splash（直接 tray icon 出现）
- **Onboarding tooltip / first-run hint**：MVP 无（依靠 empty state 文案）

详见 [ux-design § 14 Out of scope](../../bmad/02-planning/ux-design.md)。

---

## 反馈节奏

UI 设计在 v0.1 release 前可调整，release 后冻结到 v0.2。修改约定：

- **小调整**（颜色微调 / 字号 ±1pt）：直接 PR 改本目录 + 同步 ux-design.md
- **结构调整**（layout 重排 / 新增组件）：必须先开 ADR + 改本目录 + 改 ux-design.md
- **跟 constitution III.2 single source of UX truth 一致**：[ux-design.md](../../bmad/02-planning/ux-design.md) 仍是规范源头，本目录是它的视觉化扩展
