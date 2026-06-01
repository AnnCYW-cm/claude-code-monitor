# Sequence Diagram — Startup

## 这张图回答

用户双击 ClaudeCodeMonitor.app 后，到 tray icon 出现在菜单栏之间，发生了什么？

## 图

```mermaid
sequenceDiagram
  actor User
  participant macOS
  participant main as main.rs
  participant lib as lib.rs::run
  participant Builder as tauri::Builder
  participant Tray as TrayIconBuilder
  participant Win as Webview window

  User->>macOS: double-click .app
  macOS->>main: exec binary
  main->>lib: claude_code_monitor_lib::run()
  lib->>Builder: Builder::default()
  lib->>Builder: plugin(shell)
  lib->>Builder: invoke_handler([list_sessions])
  lib->>Builder: setup(closure)

  Note over Builder: enter setup closure
  Builder->>lib: set_activation_policy(Accessory)
  Note right of Builder: hides dock icon —<br/>menubar-only app

  Builder->>Tray: with_id("main")
  Tray->>Tray: icon(default_window_icon)
  Tray->>Tray: menu([Quit])
  Tray->>Tray: on_menu_event / on_tray_icon_event
  Tray-->>Builder: TrayIcon handle

  Builder->>Win: create from tauri.conf.json<br/>(visible=false)
  Win-->>Builder: WebviewWindow handle

  Builder->>lib: run event loop
  Note over User,Win: tray icon visible in menubar,<br/>window hidden, ready for clicks
```

## 关键点

- **setup closure 是一次性的**：所有初始化在这里完成。之后只剩事件循环。
- **Activation Policy = Accessory**：macOS 特有，让 app 不在 Dock 显示、不在 Cmd+Tab 切换器里。menubar-only app 的标配。
- **Webview window 是预创建 + 隐藏**：不是每次点击 tray 才新建。首次点击只是 `show()`，几乎零延迟。
- **`default_window_icon` 是 placeholder**：MVP 阶段用 32×32 黑色圆点，正式发布前换。

## 失败路径（未在图中）

- 如果 setup 闭包 panic（比如 tray 初始化失败），app 直接退出。MVP 不做 graceful recovery——出问题就是 bug，不是 runtime case。
- 如果用户禁用了 webview（极少见的 macOS 限制），window 创建会失败，app 仍能跑但点击 tray 无响应。
