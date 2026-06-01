# Component Diagram

## 这张图回答

系统由哪些模块组成？跨语言/跨进程的边界在哪？谁依赖谁？

## 图

```mermaid
graph TB
  subgraph FE[Webview Frontend - TypeScript]
    PT[Polling Trigger<br/>setInterval 2s]
    LR[List Renderer]
  end

  subgraph BE[Tauri Backend - Rust]
    Cmd["#tauri::command<br/>list_sessions"]
    PE[Process Enumerator<br/>sysinfo]
    JL[JSONL Locator<br/>~/.claude/projects/]
    JR[JSONL Reader<br/>tail last line]
    SC[State Classifier<br/>role + pending tool]
    TC[Tray Controller<br/>icon + title]
    WC[Window Controller<br/>show/hide popup]
  end

  subgraph EXT[External]
    OS[(OS process table)]
    FS[(~/.claude/projects/<br/>JSONL files)]
  end

  PT -.IPC invoke.-> Cmd
  Cmd --> PE
  PE --> OS
  Cmd --> JL
  JL --> FS
  Cmd --> JR
  JR --> FS
  Cmd --> SC
  Cmd -.IPC return.-> LR

  TrayEvt[Tray click event] --> WC
  Cmd -.derives count.-> TC
```

## 关键点

- **IPC 是唯一跨语言边界**：前端调用 `invoke("list_sessions")`，后端通过 `#[tauri::command]` 接收。所有数据用 serde 序列化。
- **Process Enumerator / JSONL Reader / State Classifier 是三个独立职责**：单元测试可以分别 mock。MVP 阶段它们都塞在 `session.rs` 一个文件，后续按需拆分。
- **Tray Controller 和 Window Controller 不响应前端 invoke**：它们响应 Tauri 自己的 tray event 回调，是完全独立的事件路径。
- **没有数据库、没有网络**：只读本地 FS 和 OS 进程表。这是为什么这个 app 不需要任何权限申请（除了首次打开时 macOS 例行的"未识别开发者"确认）。

## 取舍

没画 Webview runtime 本身（macOS 用 WKWebView）——那是 Tauri 的实现细节，不属于本项目模块。
