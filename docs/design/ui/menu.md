# Native Menu (右键 tray) — Visual Design

> **Status:** v0.1 design freeze
> **Cross-ref:** [ux-design § 2.5](../../bmad/02-planning/ux-design.md), [user-stories H4](../../product/user-stories.md#h4--退出-app), [decision-log ADR-008](../../bmad/02-planning/decision-log.md#adr-008--tray-icon-左键-toggle-popup右键弹-menu)
> **Implementation:** [S-006](../../bmad/03-solutioning/epics/story-006-tray-icon-menu.md)

---

## 1. Mockup

### 1.1 MVP（仅 Quit）

右键点击 tray icon 后：

```
Menubar:                  CCM 3
                          ┌──────────────┐
                          │  Quit  ⌘Q    │  ← native macOS NSMenu
                          └──────────────┘
                            ↑ 锚定到 tray icon 下方
                            （macOS 自动定位）
```

**Spec**：
- 渲染：macOS native NSMenu（由 Tauri tray API 调用，不是 webview）
- 跟随系统 light/dark mode 自动
- 圆角 / 阴影 / blur 跟随系统 native style
- 字体 / 颜色 全 system
- ⌘Q 快捷键自动绑定（NSMenuItem 标准）

---

## 2. Menu items 清单

### MVP

| ID | Label | Action | Keyboard shortcut |
|---|---|---|---|
| `quit` | "Quit" | `app.exit(0)` | ⌘Q |

仅 1 项。最简。

### v0.2+ 候选（**不在 MVP**）

```
┌──────────────────────────┐
│  Show Popup           ⌘O │   ← v0.2+: 也可以从 menu 打开 popup（备用入口）
│  Hide Popup           ⌘W │
│  ──────────────────────  │
│  Refresh now          ⌘R │   ← v0.2+: 立即触发 list_sessions
│  ──────────────────────  │
│  About...                │   ← v0.2+: 显示版本号 / 链接到 GitHub
│  ──────────────────────  │
│  Quit                 ⌘Q │
└──────────────────────────┘
```

**这些 v0.2+ 项 MVP 都不做**。理由：
- "Show/Hide Popup" — 用户左键 tray 已经能做（备用入口冗余）
- "Refresh now" — 2s 自动 refresh 够，不需要手动
- "About..." — 弹窗会违反 [R2 零配置](../../product/user-stories.md#r2--不该要求配置才能用)（"about" 弹窗算配置面板的边缘扩展），且不是用户实际需要

→ **MVP menu 仅 Quit**。简洁红线。

---

## 3. Menu interaction

### 3.1 触发方式

- macOS 默认行为：右键 tray icon → menu 弹出
- `menu_on_left_click(false)` 显式关闭"左键也弹 menu"
- 左键由 popup 接管（[S-007](../../bmad/03-solutioning/epics/story-007-popup-window.md)）

### 3.2 关闭方式

- 点击 menu item → 触发 action + menu 自动消失
- 点击 menu 外区域 → menu 消失，无 action
- Esc → menu 消失
- ⌘Q → 触发 Quit（同点击 Quit 项）

---

## 4. 跟其他菜单栏 utility 的对比

参考其他 macOS menubar app 的 menu pattern：

| App | 左键 | 右键 |
|---|---|---|
| Bartender | 弹自己的 popover | 弹 menu (Settings / Quit) |
| 1Password 7 | 弹大 popover | 弹 menu (Settings / Quit / About) |
| Magnet | 弹 menu | 同左键（无差别） |
| Itsycal | 弹日历 popover | 弹 menu (Preferences / Quit) |
| Stats | 弹 stats popover | 弹 menu (Open / Preferences / Quit) |

→ 我们跟 Bartender / 1Password / Itsycal 等惯例一致：左 popover + 右 menu。
→ 但比他们 menu **更简**：仅 Quit，无 Settings / About / Preferences。

---

## 5. Quit 行为细节

| 触发 | 结果 |
|---|---|
| 点击 "Quit" menu item | `app.exit(0)` → 完整退出 |
| ⌘Q 全局快捷键 | 同上 |
| 退出过程 | 清理 webview window + tray icon + log flush，~50ms 内完成 |
| 退出后 | tray icon 消失，所有 process 终止 |

**注意**：
- ⌘Q **不是全局快捷键**，只有 menu 打开时才响应（macOS NSMenuItem 标准行为）
- 关闭 popup 不退出 app（仅 hide）
- 进程死亡（crash）跟 Quit 不一样——Quit 是干净退出（log "INFO: shutting down"），crash 是 panic

---

## 6. Native menu 样式（不由我们控制）

```
macOS Sonoma+ NSMenu 默认风格：
┌──────────────────────────┐
│                          │
│   Quit              ⌘Q   │  ← 普通 text menu item（不是 checkbox）
│                          │
└──────────────────────────┘

- 圆角约 6pt
- 背景：semitransparent + blur (vibrancy effect)
- 文字：SF Pro Text 13pt
- 快捷键：等宽显示，右对齐
- 阴影：系统默认 menu shadow
- Hover 当前项：背景变 systemAccentColor（蓝色）
```

→ 全部 macOS native，我们不能也不应该自定义。

---

## 7. Edge cases

### 7.1 用户长按 tray icon

macOS 默认行为：长按 ≈ 0.5s → 弹 menu（同右键）。我们继承。

### 7.2 用户在 popup 显示时右键 tray

→ menu 弹出，popup 仍在（不互斥）。
→ 选 Quit → app 退出，popup 一起消失。

### 7.3 系统级 Cmd+Q 在 app 不在前台时

→ NSMenuItem 的 ⌘Q 仅在 menu 打开时绑定。
→ Tauri menubar app 默认无主窗口，Cmd+Q 全局不被本 app 拦截。
→ 想退出必须右键 → Quit（或 menu item）。

---

## 8. Implementation checklist

实现 [S-006 中 menu 部分](../../bmad/03-solutioning/epics/story-006-tray-icon-menu.md) 时对照本文件：

- [ ] `Menu::with_items(app, &[&quit_item])` 仅含 Quit
- [ ] `MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)` 注册
- [ ] `.menu_on_left_click(false)` 关闭左键弹 menu
- [ ] `on_menu_event` 闭包中 `if event.id == "quit" { app.exit(0); }`
- [ ] 实测：右键 tray → menu 弹出 → 点 Quit → app 退出 → tray icon 消失
- [ ] 实测：长按 tray → menu 弹出
- [ ] 实测：menu 打开时 ⌘Q → 退出
- [ ] light + dark mode menu 都正常（macOS auto）

---

## 9. Code skeleton

详见 [S-006](../../bmad/03-solutioning/epics/story-006-tray-icon-menu.md)。摘要：

```rust
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};

// in setup():
let quit_item = MenuItem::with_id(app, "quit", "Quit", true, Some("CmdOrCtrl+Q"))?;
let menu = Menu::with_items(app, &[&quit_item])?;

let _tray = TrayIconBuilder::with_id("main")
    .icon(app.default_window_icon().unwrap().clone())
    .icon_as_template(true)
    .menu(&menu)
    .menu_on_left_click(false)  // 关闭左键弹 menu，让左键给 popup
    .on_menu_event(|app, event| {
        if event.id.as_ref() == "quit" {
            app.exit(0);
        }
    })
    .on_tray_icon_event(|tray, event| {
        // 左键 toggle popup, 见 popup-window.md
    })
    .build(app)?;
```

---

## 10. v0.2+ 演进路径

如果未来加 menu items（违反 MVP 简洁但有强需求时），按这个 priority：

1. **About** — 显示版本号 + GitHub link（最低门槛扩展）
2. **Refresh now** — 立即触发 list_sessions（debug 用）
3. **Show/Hide Popup** — 备用入口（兼容性）
4. **Open log file** — 在 Finder 显示 `~/Library/Logs/...`（debug）
5. ❌ **Preferences** — 永远不加（违反 [R2 零配置](../../product/user-stories.md#r2--不该要求配置才能用)）

任何 menu item 改动需要 ADR。
