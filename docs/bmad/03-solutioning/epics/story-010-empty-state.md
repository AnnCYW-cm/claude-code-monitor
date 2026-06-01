# S-010 · Empty state render

**Epic:** [002 Menubar UI](epic-002-menubar-ui.md)
**Status:** ✅ DONE (2026-06-01, 随 S-008 一并落地)
**Estimate:** S — actual ~15min
**Owner:** caiyiwen

## Description

**As** a first-time user (or user with no active sessions)
**I want to** see a friendly empty state in the popup instead of a blank list
**so that** I understand the app is working and what to do next.

## Acceptance criteria

- 列表为空时（`sessions.length === 0`）→ 显示 empty state
- 文案（按 [ux-design § 7](../../02-planning/ux-design.md)）：
  - 主：`no claude sessions running`
  - 副 (hint): `start a session with \`claude\` in your terminal`
- 居中垂直 + 水平
- 字体：主 SF Pro Text Regular 13pt `secondaryLabelColor`；副 12pt `tertiaryLabelColor`
- 无插画 / 无 CTA 按钮（违反 R2 / P5）
- popup 高度此时为最小 (200pt)

## Dev notes

- 在 `render(sessions)` 函数开头判断 `sessions.length === 0` → render empty markup
- CSS class `.empty-state { display: flex; flex-direction: column; align-items: center; justify-content: center; ... }`
- 文案直接用 inline HTML，不引入 i18n（MVP 仅英文）

## Dependencies

- **Upstream**: S-008
- **Downstream**: 无

## Files to touch

- `src/main.ts` — empty state branch
- `src/style.css` — `.empty-state` 样式

## Test plan

### 手动测试
- 启动 app 时无 claude session → 看 empty state
- 启动 claude session → popup 自动从 empty 切到列表
- 关掉所有 session → 切回 empty
- light + dark mode 都看一下

### 跟 scenarios.md S4 mockup 对比

[S4 T+0:15 的 popup mockup](../../../product/scenarios.md) 就是此状态。

## Definition of Done

- [ ] 代码 merged
- [ ] empty state 跟 [E1 acceptance](../../../product/user-stories.md#e1--首次安装空状态) 一致
- [ ] 跟 [S4 mockup](../../../product/scenarios.md) 一致
- [ ] 切换 list↔empty 无 flicker
