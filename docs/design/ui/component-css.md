# Component CSS Spec — Full Implementation Skeleton

> **Status:** v0.1 design freeze · ready for implementation
> **Cross-ref:** All `ui/*` files in this directory
> **Implementation:** [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md) + [S-009](../../bmad/03-solutioning/epics/story-009-expand-message.md) + [S-010](../../bmad/03-solutioning/epics/story-010-empty-state.md)
>
> 这份是**所有 CSS 的 single source**。开发时整个 `src/style.css` 就是本文件的实现。

---

## 1. 系统颜色 token 映射

macOS 系统颜色通过 CSS variables 映射，自动跟随 `prefers-color-scheme`：

```css
/* ============================================ */
/* Light mode (default)                          */
/* ============================================ */
:root {
  --label:                rgba(0, 0, 0, 0.85);
  --secondary-label:      rgba(0, 0, 0, 0.5);
  --tertiary-label:       rgba(0, 0, 0, 0.26);
  --quaternary-label:     rgba(0, 0, 0, 0.1);

  --window-bg:            rgb(236, 236, 236);
  --control-bg:           rgba(0, 0, 0, 0.05);  /* hover bg */
  --accent:               rgb(0, 122, 255);     /* system blue */
  --accent-rgb:           0, 122, 255;

  --separator:            rgba(0, 0, 0, 0.1);

  --system-yellow:        rgb(255, 204, 0);
  --system-yellow-rgb:    255, 204, 0;
  --system-yellow-dark:   rgb(153, 122, 0);

  --system-green:         rgb(52, 199, 89);
  --system-green-rgb:     52, 199, 89;
  --system-green-dark:    rgb(0, 122, 51);

  --system-gray:          rgb(142, 142, 147);
  --system-gray-rgb:      142, 142, 147;
}

/* ============================================ */
/* Dark mode                                     */
/* ============================================ */
@media (prefers-color-scheme: dark) {
  :root {
    --label:                rgba(255, 255, 255, 0.85);
    --secondary-label:      rgba(255, 255, 255, 0.55);
    --tertiary-label:       rgba(255, 255, 255, 0.25);
    --quaternary-label:     rgba(255, 255, 255, 0.1);

    --window-bg:            rgb(40, 40, 40);
    --control-bg:           rgba(255, 255, 255, 0.07);
    --accent:               rgb(10, 132, 255);
    --accent-rgb:           10, 132, 255;

    --separator:            rgba(255, 255, 255, 0.1);

    --system-yellow:        rgb(255, 214, 10);
    --system-yellow-dark:   rgb(255, 214, 10);  /* dark mode 直接用亮色 */

    --system-green:         rgb(48, 209, 88);
    --system-green-dark:    rgb(48, 209, 88);

    --system-gray:          rgb(174, 174, 178);
  }
}
```

**注**：上面的 RGB 值取自 [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/color)。开发时**实测**对比 macOS 系统 popover 颜色是否一致，必要时微调。

---

## 2. 字体 token

```css
:root {
  --font-sf-pro: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  --font-sf-mono: "SF Mono", "Menlo", "Monaco", "Courier New", monospace;
}
```

**字号 / weight 规范**（来自 [ux-design § 8 typography summary](../../bmad/02-planning/ux-design.md)）：

| 用途 | 字体 + weight + size |
|---|---|
| Title (header) | SF Pro Text 600 (Semibold) 13px |
| Body / name | SF Pro Text 500 (Medium) 13px |
| Secondary (preview / hint) | SF Pro Text 400 (Regular) 12px |
| Caption (status / duration) | SF Pro Text 400 (Regular) 11px |
| Expanded message | SF Mono 400 (Regular) 12px |

---

## 3. 全局重置

```css
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

html, body {
  width: 360px;
  min-height: 200px;
  font-family: var(--font-sf-pro);
  font-size: 13px;
  color: var(--label);
  background: var(--window-bg);
  -webkit-font-smoothing: antialiased;
  overflow: hidden;  /* popup 内部各区自己 scroll */
}

body {
  display: flex;
  flex-direction: column;
}
```

---

## 4. Header 组件

```css
.header {
  height: 40px;
  padding: 12px 14px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--separator);
  flex-shrink: 0;  /* 不被 list 挤压 */
}

.header .title {
  font: 600 13px var(--font-sf-pro);
  color: var(--label);
}

.header .refresh-btn {
  font: 400 12px var(--font-sf-pro);
  color: var(--label);
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 3px 8px;
  border-radius: 4px;
  transition: background 100ms linear;
}

.header .refresh-btn:hover {
  background: var(--control-bg);
}

.header .refresh-btn:active {
  background: rgba(var(--accent-rgb), 0.1);
}

.header .refresh-btn.loading {
  pointer-events: none;
  color: var(--tertiary-label);
}

.header .refresh-btn.loading::after {
  content: "...";  /* 简化的 loading indicator，不用 spinner */
}
```

---

## 5. List 容器

```css
.session-list {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

/* macOS native scrollbar style，无需自定义 */
```

---

## 6. List item

```css
.session {
  padding: 10px 14px;
  cursor: pointer;
  transition: background 100ms linear;
  user-select: none;
}

.session:hover {
  background: var(--control-bg);
}

.session:active {
  background: rgba(var(--accent-rgb), 0.1);
}

.session.expanded {
  background: var(--control-bg);  /* 跟 hover 一致，暗示激活 */
}

.session + .session {
  border-top: 1px solid var(--separator);
}

/* ─────────── Row 1: name · badge · duration ─────────── */

.session .row {
  display: flex;
  align-items: baseline;
  gap: 6px;
}

.session .name {
  font: 500 13px var(--font-sf-pro);
  color: var(--label);
  max-width: 240px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session .sep {
  font: 400 11px var(--font-sf-pro);
  color: var(--tertiary-label);
}

.session .badge {
  font: 400 11px var(--font-sf-pro);
  padding: 2px 6px;
  border-radius: 4px;
  text-transform: lowercase;
}

.session .badge.waiting {
  background: rgba(var(--system-yellow-rgb), 0.2);
  color: var(--system-yellow-dark);
}

.session .badge.working {
  background: rgba(var(--system-green-rgb), 0.2);
  color: var(--system-green-dark);
}

.session .badge.unknown {
  background: rgba(var(--system-gray-rgb), 0.3);
  color: var(--secondary-label);
}

.session .duration {
  font: 400 11px var(--font-sf-pro);
  color: var(--secondary-label);
}

/* ─────────── Row 2: message preview ─────────── */

.session .message {
  margin-top: 4px;
  font: 400 12px var(--font-sf-pro);
  color: var(--secondary-label);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ─────────── Expanded content ─────────── */

.session .expanded-content {
  max-height: 0;
  opacity: 0;
  overflow: hidden;
  transition:
    max-height 150ms cubic-bezier(0, 0, 0.2, 1),
    opacity   150ms cubic-bezier(0, 0, 0.2, 1) 50ms;
}

.session.expanded .expanded-content {
  max-height: 200pt;
  opacity: 1;
  margin-top: 8px;
}

.session .expanded-content .body {
  padding: 12px 14px;
  font: 400 12px var(--font-sf-mono);
  color: var(--label);
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 200pt;
  overflow-y: auto;
}

.session .expanded-content .tool-use {
  color: var(--secondary-label);
  /* [Bash] git status 这种简化展示，inline 在 body 里 */
}
```

---

## 7. Empty state

```css
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  padding: 24px 14px;
  text-align: center;
  min-height: 160px;
}

.empty-state .main {
  font: 400 13px var(--font-sf-pro);
  color: var(--secondary-label);
  margin-bottom: 12px;
}

.empty-state .hint {
  font: 400 12px var(--font-sf-pro);
  color: var(--tertiary-label);
}

.empty-state .hint code {
  font: 400 12px var(--font-sf-mono);
  background: var(--control-bg);
  padding: 1px 3px;
  border-radius: 3px;
}
```

---

## 8. Popup window (整体)

由于 popup window 是 Tauri webview，圆角 / 阴影由 macOS 控制。HTML/CSS 只控制内部 layout：

```css
/* popup body 已经在 § 3 设了 width 360, min-height 200 */

@keyframes popup-show {
  from { opacity: 0; transform: scale(0.95); }
  to   { opacity: 1; transform: scale(1); }
}

@keyframes popup-hide {
  from { opacity: 1; }
  to   { opacity: 0; }
}

body {
  animation: popup-show 150ms cubic-bezier(0, 0, 0.2, 1);
}

body.hiding {
  animation: popup-hide 100ms cubic-bezier(0.4, 0, 1, 1) forwards;
}
```

---

## 9. CSS classes 完整清单

| Class | 用途 | 详 |
|---|---|---|
| `.header` | popup 顶部 header 容器 | § 4 |
| `.header .title` | "Claude Code Monitor" 标题 | § 4 |
| `.header .refresh-btn` | refresh 按钮 | § 4 |
| `.header .refresh-btn.loading` | refresh 进行中 | § 4 |
| `.session-list` | 列表容器 | § 5 |
| `.session` | 单 session 列表项 | § 6 |
| `.session.expanded` | 列表项展开态 | § 6 |
| `.session .row` | name + badge + duration 行 | § 6 |
| `.session .name` | cwd 末段名 | § 6 |
| `.session .sep` | 中圆点分隔符 | § 6 |
| `.session .badge.waiting/working/unknown` | 状态徽章 | § 6 |
| `.session .duration` | 已等时长 | § 6 |
| `.session .message` | 单行消息预览 | § 6 |
| `.session .expanded-content` | 展开内容容器 | § 6 |
| `.session .expanded-content .body` | 完整消息正文 | § 6 |
| `.session .expanded-content .tool-use` | tool_use 块 inline 样式 | § 6 |
| `.empty-state` | 空列表 fallback | § 7 |
| `.empty-state .main` | 主文案 | § 7 |
| `.empty-state .hint` | hint 文案 | § 7 |
| `body.hiding` | popup 隐藏 animation | § 8 |

---

## 10. HTML skeleton

完整 `index.html` 的 `<body>` 部分：

```html
<body>
  <header class="header">
    <span class="title">Claude Code Monitor</span>
    <button class="refresh-btn" id="refresh-btn">refresh</button>
  </header>

  <main class="session-list" id="session-list">
    <!-- 由 main.ts 动态 render -->
    <!-- 例：当 sessions.length > 0 -->
    <div class="session">
      <div class="row">
        <span class="name">api-server-tests</span>
        <span class="sep">·</span>
        <span class="badge waiting">waiting</span>
        <span class="sep">·</span>
        <span class="duration">1min</span>
      </div>
      <div class="message">All 142 tests passed...</div>
      <div class="expanded-content">
        <div class="body">All 142 tests passed.

Coverage: 87.3%

[Bash] git diff --stat

Want me to commit with message 'fix: token validation edge case'?</div>
      </div>
    </div>
    <!-- ... 更多 .session ... -->

    <!-- 或：当 sessions.length === 0 -->
    <!--
    <div class="empty-state">
      <div class="main">no claude sessions running</div>
      <div class="hint">start a session with <code>claude</code> in your terminal</div>
    </div>
    -->
  </main>
</body>
```

JS render 函数见 [list-item.md § 12](list-item.md) 和 [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md)。

---

## 11. CSS 总字数估算

按上面 spec，完整 `src/style.css` 约：

- variables: ~50 lines
- header: ~20 lines
- list container: ~5 lines
- list item + states + expanded: ~80 lines
- empty state: ~20 lines
- animations: ~15 lines
- 总计：**~190 lines**

→ MVP 单文件 CSS，无 build step（Vite 直接处理）。

---

## 12. Light/dark mode 实测要点

实现 [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md) 后必测：

| 元素 | Light mode 实测 | Dark mode 实测 |
|---|---|---|
| popup 背景 | 跟系统 popover 一致（淡灰） | 跟系统 popover 一致（深灰） |
| header 文字 | 黑色 | 白色 |
| separator | 微透明黑 | 微透明白 |
| waiting badge | 黄底深黄字 | 半透明黄底亮黄字 |
| working badge | 绿底深绿字 | 半透明绿底亮绿字 |
| unknown badge | 灰底次级字 | 灰底次级字 |
| hover bg | 浅灰 | 浅白透明 |
| expanded bg | 同 hover | 同 hover |
| mono 字体 | 黑字 | 白字 |

→ **跟 macOS 系统 NSPopover 对比**：打开 Bartender / 1Password / Itsycal 等 menubar utility，对比颜色"接近"即可。完全像素一致不必。

---

## 13. 没做的 CSS 高级特性

| 不做 | 原因 |
|---|---|
| CSS-in-JS | MVP 无前端框架 |
| PostCSS / Sass | Vite 默认 CSS 够 |
| Tailwind | 引入又一层 |
| CSS modules | 单文件无 namespace 冲突 |
| 自定义 scrollbar | macOS native 更顺 |
| Animation library | 用 native CSS transition |
| Backdrop-filter blur | macOS popover 自己有 vibrancy，CSS 加 blur 会双层 |

---

## 14. Implementation checklist

实现 CSS 时对照本文件：

- [ ] CSS variables 跟 macOS HIG 颜色 lookup 一致
- [ ] `@media (prefers-color-scheme: dark)` 切换正确
- [ ] 所有字号 / weight 按 § 2 table
- [ ] 状态徽章颜色（黄/绿/灰）light/dark 实测对比 ≥ WCAG AA
- [ ] hover transition 100ms linear
- [ ] expand transition 150ms ease-out (opacity 50ms delay)
- [ ] popup show 150ms ease-out scale + opacity
- [ ] popup hide 100ms ease-in opacity (无 scale)
- [ ] 列表分隔线只在 item 间，不在首末
- [ ] empty state 跟列表互斥，瞬时切换
- [ ] 总 CSS 行数 ≤ 250（防膨胀）
- [ ] 实测 Bartender / 1Password 等做对比 UX
