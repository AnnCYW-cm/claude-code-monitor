# S-007 · Popup window create + show/hide on click

**Epic:** [002 Menubar UI](epic-002-menubar-ui.md)
**Status:** Pending
**Estimate:** M (1-2 days)
**Owner:** caiyiwen

## Description

**As** a macOS user
**I want** clicking the tray icon to open a popup window showing my sessions; clicking again to close it
**so that** I can quickly see status without committing to opening a full app window.

## Acceptance criteria

- App 启动时预创建 webview window (label="main")，初始 `visible=false`
- 窗口属性 ([ux-design § 3.1](../../02-planning/ux-design.md))：
  - 360pt × 480pt
  - `decorations=false`（无标题栏）
  - `resizable=false`
  - `skipTaskbar=true`
  - `alwaysOnTop=true`
- 左键 click tray → 如果 window 不可见 → `show() + set_focus()`；可见 → `hide()`
- 弹出到显示 < 200ms ([NFR-P2](../../02-planning/PRD.md))
- 关闭 popup 不退出 app（window hide 而不是 close）
- Quit menu 是唯一退出途径

## Dev notes

- 代码大部分已在 scaffold `lib.rs::on_tray_icon_event` 实现
- 关键 API:
  - `app.get_webview_window("main")` 取 handle
  - `window.is_visible()?` 查可见性
  - `window.show()?` / `window.hide()?`
  - `window.set_focus()?` 把窗口拉到前台
- 触发条件: `TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. }` — 用 `Up` 不用 `Down` 避免长按抖动
- 窗口在 `tauri.conf.json` 里声明 (已 scaffold 配好)
- 位置：MVP 不主动定位（用 Tauri 默认）。v0.2+ 锚定到 tray rect

## Dependencies

- **Upstream**: S-006 (tray icon 必须先就位)
- **Downstream**: S-008, S-009, S-010 (UI 内容)

## Files to touch

- `src-tauri/src/lib.rs` — `on_tray_icon_event` 闭包 (已在 scaffold)
- `src-tauri/tauri.conf.json` — `windows[0]` 配置 (已在 scaffold)

## Test plan

### 手动测试
- 启动 app，window 不可见（看不到任何 webview）
- 左键 click tray → window 弹出 < 200ms 视觉感知
- 再点 → 隐藏
- 长按 → 不会触发多次（依赖 Up event）
- 隐藏 popup 后 app 继续跑（tray icon 还在 = ps 还在）
- 切到全屏 app → popup 仍能弹（alwaysOnTop）

### 性能测试
- 用 `console.time` 测 click → window 可见的时间
- 重复 10 次，p99 < 200ms

## Definition of Done

- [ ] 代码 merged
- [ ] popup 弹出/隐藏稳定
- [ ] [H2 acceptance](../../../product/user-stories.md#h2--弹开列表分诊优先级) "点击 tray < 200ms" 实测通过
- [ ] [UML 08 Sequence](../../../design/uml/08-sequence-tray-click.md) 跟实现一致
