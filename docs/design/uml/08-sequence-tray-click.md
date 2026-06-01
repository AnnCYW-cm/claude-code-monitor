# Sequence Diagram — Tray Click

## 这张图回答

用户在菜单栏左键点击图标，到看到弹窗（或关闭弹窗）之间发生了什么？

## 图

```mermaid
sequenceDiagram
  actor User
  participant Tray as Tray icon
  participant Handler as on_tray_icon_event
  participant App as AppHandle
  participant Win as WebviewWindow

  User->>Tray: left-click + release
  Tray->>Handler: TrayIconEvent::Click<br/>{button: Left, state: Up}

  Handler->>App: app_handle()
  App->>Win: get_webview_window("main")
  Win-->>Handler: WebviewWindow

  Handler->>Win: is_visible()?

  alt currently hidden
    Win-->>Handler: false
    Handler->>Win: show()
    Handler->>Win: set_focus()
    Note over Win: popup appears<br/>(default position; anchoring is post-MVP)
  else currently visible
    Win-->>Handler: true
    Handler->>Win: hide()
    Note over Win: popup disappears
  end
```

## 关键点

- **匹配 `MouseButtonState::Up` 而不是 Down**：避免长按抖动产生重复 toggle。
- **toggle 语义**：同一个图标的点击既能开也能关。比"点击开 / 失焦关"更可预测——这是菜单栏 utility 的惯例（参考：1Password、Magnet、Bartender）。
- **没有定位 popup 到 tray 附近**：MVP 用 webview window 默认位置（屏幕中央或上次位置）。正式版需要用 macOS API 把窗口锚定到 tray icon 下方，那是 v0.2 的 task。
- **右键不在此图**：右键弹出 native menu（含 Quit），由 `on_menu_event` 处理，与本图独立。

## 未来扩展（不在 MVP）

- 失焦自动关闭（点击窗口外消失）：监听 `WindowEvent::Focused(false)`。
- 定位锚定到 tray：拿 tray rect + 计算偏移，调 `window.set_position()`。
- ESC 关闭弹窗：监听全局快捷键 / 窗口键盘事件。
