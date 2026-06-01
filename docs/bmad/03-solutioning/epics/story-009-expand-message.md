# S-009 · Expand/collapse last message + tool_use simple render

**Epic:** [002 Menubar UI](epic-002-menubar-ui.md)
**Status:** ✅ DONE (2026-06-01，含 expand state、mono font 渲染、点击 toggle、列表项移除时收起)
**Estimate:** M — actual ~30min（一并随 S-008 写完）
**Owner:** caiyiwen

## Description

**As** a user looking at a session in the popup
**I want to** click the row to expand and see the full last assistant message
**so that** I can decide whether the session needs my action without switching to its terminal.

## Acceptance criteria

- 点击列表项 → 该项下方展开显示完整 last_message
- 同一时刻最多 1 项展开
- 再点同项 → 收起
- 点其他项 → 收起当前 + 展开新项
- 关闭 popup 重置所有展开态（下次打开都是 collapsed）
- 展开内容包含 `tool_use` 块时显示：`[ToolName] short args`（如 `[Bash] git status`、`[Read] /path/file.rs:42`）—— MVP 简化展示，**不** 完整格式化
- 文本字体：SF Mono Regular 12pt
- 展开高度 ≤ 200pt 时 popup 自适应增高；> 200pt 时展开区域内部可滚动
- 展开/收起 animation: 150ms ease-out fade + height

## Dev notes

- 前端 state: `let expandedPid: number | null = null;`
- 点击 row handler: 
  ```ts
  if (expandedPid === session.pid) {
    expandedPid = null;
  } else {
    expandedPid = session.pid;
  }
  render(sessions);  // 重新 render 整个列表
  ```
- `last_message` 字段在 Session DTO 已经包含（S-005）
- tool_use 渲染 MVP 简化：parse last_message 字符串识别 `<tool_use>...</tool_use>` pattern (待 spec/jsonl-schema.md 确认 marker)
- 简化策略：如果识别失败，直接显示 raw 文本
- 关闭 popup 时 reset：监听 `tauri.window.onCloseRequested` 或在 `hide()` 时一并 `expandedPid = null`
- Animation: CSS `transition: max-height 150ms ease-out, opacity 150ms ease-out`

## Dependencies

- **Upstream**: S-008 (list 已 render)
- **Downstream**: 无

## Files to touch

- `src/main.ts` — expand state + click handler
- `src/style.css` — expand transition + mono font + scrolling

## Test plan

### 手动测试
- 点击 row → 展开
- 再点 → 收起
- 点其他 row → 切换展开
- last_message 很短 → popup 自适应高
- last_message 很长（> 200pt）→ 内部滚动
- 含 tool_use → 显示 `[ToolName]` 简化
- 关闭 popup 再打开 → 默认 collapsed

### Visual
- 跟 [ux-design § 6](../../02-planning/ux-design.md) 的 spec 对比

## Definition of Done

- [x] 代码 merged（in main.ts）
- [~] 5 种 last_message 类型显示——**当前限制：backend 把 last_message 截到 200 字符**（PREVIEW_MAX_CHARS in session.rs）。展开区透明地显示截断提示。**真完整正文需要 get_full_message(pid) IPC，v0.2 工作**
- [ ] expand animation 顺滑（CSS fade-in 150ms，待手动 review）
- [ ] [H3 acceptance](../../../product/user-stories.md#h3--不切走也能读到关键信息) 部分通过——含 last_message 看到了一部分，full 待 v0.2

## Implementation summary (2026-06-01)

- `let expandedPid: number | null = null;` 模块状态
- 点击 row → toggle `expandedPid`，触发 `renderSessions(sessions)` 整体重排
- 展开块用 SF Mono 12pt，max-height 200pt 内部滚动
- 列表里去掉的 session 自动收起（防 stale expandedPid）
- empty state 时强制 reset 防 race
- 当 `last_message.length >= 200`（backend 截断阈值）显示"preview truncated"提示——透明 UX 不假装有全文
