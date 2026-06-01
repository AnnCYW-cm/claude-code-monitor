# Epic 002 · Menubar UI

**Status:** Pending
**Owner:** caiyiwen
**Sprint target:** Sprint 2-3 (Week 2-3)
**Estimate:** ~1 week

## Goal

让用户看到：tray icon + waiting 数字 + 弹出 popup + 列表 + 展开消息 + 空状态。

完成此 epic 后，作者可以**自己 dogfood** 整个产品。

## Success criteria

- 启动 app，tray icon 出现在 menubar
- 有 claude session waiting 时，tray 显示 "等你" 数字
- 左键 tray → popup 弹出 < 200ms (NFR-P2)
- 列表显示每个 session：cwd 末段名 / 状态徽章 / 时长 / 消息预览
- 点击列表项展开完整消息
- 右键 tray → native menu 显示 Quit
- 无 claude session 时，popup 显示 empty state

## Stories

| ID | Story | Estimate | Status |
|---|---|---|---|
| [S-006](story-006-tray-icon-menu.md) | Tray icon + native menu (Quit) | S | ✅ DONE 2026-06-01（scaffold + "99+" 兜底）|
| [S-007](story-007-popup-window.md) | Popup window create + show/hide on click | M | ✅ DONE 2026-06-01（scaffold 验完）|
| [S-008](story-008-session-list-render.md) | Session list render | M | ✅ DONE 2026-06-01 |
| [S-009](story-009-expand-message.md) | Expand/collapse last message + tool_use simple render | M | ✅ DONE 2026-06-01 |
| [S-010](story-010-empty-state.md) | Empty state render | S | ✅ DONE 2026-06-01 |

## Prerequisites

- ✅ Epic 1 完成（需要 `list_sessions` 返回数据）
- ✅ [ux-design.md](../../02-planning/ux-design.md)（设计规范）
- ✅ [UML 06/07/08 Sequence diagrams](../../../design/uml/06-sequence-startup.md)

## Out of scope

- popup 锚定到 tray icon 下方（v0.2+）
- 失焦自动 hide（v0.2+）
- 键盘导航（v0.2+ a11y）
- 暗色模式优化（依赖系统自动切换，无需特殊代码，但需要实测）

## Risks

| Risk | Mitigation |
|---|---|
| WKWebView 在 macOS 15 弹窗位置异常 | 实测，必要时升级 Tauri 版本 |
| 列表 render 性能问题（>15 session 卡） | NFR-P1 已 budget，超了优化 |
| 暗色模式 separator 不明显 | ux-design § 15 open question，实测调 |

## Definition of Done (Epic level)

- [ ] 全部 5 stories 完成
- [ ] 作者 dogfood 1 天无 UI bug
- [ ] light + dark mode 都正常
- [ ] M1 + Intel mac 都测过（如果有 Intel 机器）
