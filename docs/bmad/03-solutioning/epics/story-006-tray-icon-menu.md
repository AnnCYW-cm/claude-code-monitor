# S-006 · Tray icon + native menu (Quit)

**Epic:** [002 Menubar UI](epic-002-menubar-ui.md)
**Status:** Pending
**Estimate:** S (half day)
**Owner:** caiyiwen

## Description

**As** a macOS user
**I want to** see a Claude Code Monitor icon in my menubar with a Quit option in the right-click menu
**so that** I can recognize the app's presence and exit it.

## Acceptance criteria

- Tray icon 在 menubar 出现，使用 `src-tauri/icons/icon.png`（MVP 占位）
- icon 设为 template image（macOS 自动 light/dark 着色）—— `icon_as_template(true)`
- 右键点击 tray → 弹出 native menu，含一项 "Quit"
- 点击 Quit → app 完整退出（`app.exit(0)`）
- 左键点击 tray → 不弹 menu（在 S-007 接管）—— `menu_on_left_click(false)`
- App 启动时设 `activation_policy = Accessory`（隐藏 dock icon）
- tray icon 跟随 macOS 系统 light/dark mode 自动变色

## Dev notes

- 代码已在当前 `src-tauri/src/lib.rs` scaffold 实现大部分，本 story 主要是验证 + 完善
- 关键 API：
  - `tauri::tray::TrayIconBuilder::with_id("main")`
  - `.icon(app.default_window_icon().unwrap().clone())`
  - `.icon_as_template(true)`
  - `.menu(&menu)` where `menu = Menu::with_items(app, &[&quit_item])`
  - `.menu_on_left_click(false)`
  - `.on_menu_event(|app, event| { if event.id == "quit" { app.exit(0); } })`
- `set_activation_policy(ActivationPolicy::Accessory)` macOS-only，cfg-gated

## Dependencies

- **Upstream**: S-005 (需要 list_sessions 提供 waiting count 给 tray title — S-005 已包含 set_title 调用)
- **Downstream**: S-007 (popup window 需要响应左键 click，本 story 把 menu_on_left_click 设 false)

## Files to touch

- `src-tauri/src/lib.rs` — tray setup (已在 scaffold 里，需要 review + 完善)

## Test plan

### 手动测试
- `cargo tauri dev` 启动
- 检查 menubar 出现 icon
- 切换 macOS 暗色模式，icon 变色
- 右键点击 → menu 弹出，含 Quit
- 点击 Quit → app 退出
- 重新启动 → 再次出现
- 检查 dock 无 icon（Accessory）

### 跨版本测试
- macOS 12, 14, 15 各一次

## Definition of Done

- [ ] 代码 merged
- [ ] macOS 12/14/15 都验证过（**作者待办** F4 同时做）
- [ ] tray title 跟 [H1](../../../product/user-stories.md#h1--瞄一眼判断是否切走) acceptance 一致
- [ ] Quit 干净退出（无残留进程，ps 验证）
