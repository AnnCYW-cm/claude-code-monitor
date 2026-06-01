# S-008 · Session list render

**Epic:** [002 Menubar UI](epic-002-menubar-ui.md)
**Status:** Pending
**Estimate:** M (2 days)
**Owner:** caiyiwen

## Description

**As** a user looking at the popup
**I want to** see all my sessions as a list with cwd name / status badge / duration / message preview
**so that** I can scan and decide which session needs attention.

## Acceptance criteria

- Popup 主体是一个 list，每个 session 一行
- 每行显示（按 [ux-design § 5](../../02-planning/ux-design.md)）：
  - **name**: cwd 末段名（最大宽度 240pt，超出 ellipsis）
  - **separator**: `·`
  - **status badge**: `waiting` (黄底) / `working` (绿底) / `unknown` (灰底)
  - **duration**（仅 waiting 状态）: `3min` 或 `just now` (<1min)
  - **message preview**: 单行，前 ~50 字，ellipsis
- 列表按后端返回的顺序 render（waiting 在前按时长降序、working 在后）
- 列表超过 6 行开启垂直滚动（NFR-FR-E4）
- 每 2s 通过 `invoke("list_sessions")` 自动刷新
- 列表变化时直接重排，无 animation（[ux-design § 11](../../02-planning/ux-design.md)）
- 颜色 / 字体严格按 ux-design 文档

## Dev notes

- `src/main.ts` 完全重写
- DOM 手写（无框架），用 `document.createElement` + `appendChild`
- CSS 用 system colors (CSS vars) + `prefers-color-scheme`
- Duration formatting:
  ```ts
  function formatDuration(unixSeconds: number): string {
    const elapsed = Math.floor((Date.now() / 1000 - unixSeconds) / 60);
    if (elapsed < 1) return "just now";
    return `${elapsed}min`;
  }
  ```
- Message preview: `message.split('\n')[0].slice(0, 80)` (大致 50 字符)
- 列表 render 用 `innerHTML = ""` + 重建（10 session 量级，重建成本 < 5ms）
- 滚动：CSS `overflow-y: auto`，max-height = popup-height - header-height

## Dependencies

- **Upstream**: S-005 (IPC), S-007 (popup window)
- **Downstream**: S-009 (expand), S-010 (empty state)

## Files to touch

- `src/main.ts` — 全量改写
- `src/style.css` — 全量改写 (按 ux-design 规范)
- `index.html` — 微调 header

## Test plan

### 手动测试
- 3 个 session (mix waiting/working) → 列表渲染正确
- 1 session waiting 5min → 显示 "5min"
- 1 session waiting just now → 显示 "just now"
- 8 session → 列表可滚动，第 7-8 项需要滚
- cwd 超长 → 末段名 ellipsis
- 消息超长 → preview ellipsis
- light/dark mode → 颜色自动切换

### Visual regression
- 截图 light + dark mode
- 跟 ux-design.md 规范对比

## Definition of Done

- [ ] 代码 merged
- [ ] 实测匹配 [scenarios.md S1/S2](../../../product/scenarios.md) 的 mockup
- [ ] 列表 render 性能：10 session render < 5ms
- [ ] 颜色对比度 ≥ WCAG AA
