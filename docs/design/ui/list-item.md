# List Item — Visual Design

> **Status:** v0.1 design freeze
> **Cross-ref:** [ux-design § 5-6](../../bmad/02-planning/ux-design.md), [user-stories H2](../../product/user-stories.md#h2--弹开列表分诊优先级), [H3](../../product/user-stories.md#h3--不切走也能读到关键信息)
> **Implementation:** [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md), [S-009](../../bmad/03-solutioning/epics/story-009-expand-message.md)

---

## 1. Anatomy

```
┌──────────────────────────────────────────────────┐
│ ←14pt→                                  ←14pt→  │  ← 左右 padding 14pt
│ ┌─10pt─────────────────────────────────────┐  │  ← 上下 padding 10pt
│ │                                          │  │
│ │   ┌─────────┐                            │  │
│ │   │ name    │ · ┌──────┐ · 3min          │  │  ← Row 1: name + sep + badge + sep + duration
│ │   └─────────┘   │waiting│                │  │
│ │                  └──────┘                │  │
│ │   ↓ 4pt 间距                              │  │
│ │   message preview (single line, max ~50字) │  │  ← Row 2: 单行消息预览
│ │                                          │  │
│ └─10pt─────────────────────────────────────┘  │
│                                                  │
└──────────────────────────────────────────────────┘
                                ↑ 1px separator color (跨整宽，但 left/right padding 内)

Item 总高度 ≈ 10 + 16 (row1) + 4 (gap) + 14 (row2) + 10 = 54pt
```

---

## 2. 字段视觉 spec

| 元素 | 字体 / 字号 | 颜色 | 行为 |
|---|---|---|---|
| `name` (cwd 末段名) | SF Pro Text Medium 13pt | `labelColor` | 截断 ellipsis，max-width 240pt |
| separator `·` | SF Pro Text Regular 11pt | `tertiaryLabelColor` | name 跟 badge 之间 |
| status badge | SF Pro Text Regular 11pt | 状态色（见 § 3） | 圆角 4pt，padding 2pt/6pt |
| duration (`3min`) | SF Pro Text Regular 11pt | `secondaryLabelColor` | 仅 waiting 显示 |
| message preview | SF Pro Text Regular 12pt | `secondaryLabelColor` | 单行 ellipsis，max ~50 字 |

---

## 3. Status badge 颜色

按 [ux-design § 5.3](../../bmad/02-planning/ux-design.md)：

### 3.1 Waiting (黄)

```
Light mode:
  ┌──────────┐
  │ waiting  │   ← 背景 systemYellow 20% opacity
  └──────────┘    文字 systemYellow darken 60%（深黄）

Dark mode:
  ┌──────────┐
  │ waiting  │   ← 背景同 (透明度让它在深背景上是浅黄)
  └──────────┘    文字 systemYellow（亮黄）
```

### 3.2 Working (绿)

```
Light mode:
  ┌──────────┐
  │ working  │   ← 背景 systemGreen 20% opacity
  └──────────┘    文字 systemGreen darken 60%

Dark mode:
  ┌──────────┐
  │ working  │
  └──────────┘    文字 systemGreen 亮
```

### 3.3 Unknown (灰)

```
Light mode:
  ┌──────────┐
  │ unknown  │   ← 背景 systemGray 30% opacity
  └──────────┘    文字 secondaryLabelColor

Dark mode:
  ┌──────────┐
  │ unknown  │
  └──────────┘    跟 light 一致语义
```

---

## 4. 4 个交互状态 mockup

### 4.1 Default (collapsed, no interaction)

```
┌──────────────────────────────────────────────────┐
│                                                  │
│  api-server-tests · ⬛waiting · 1min            │
│  All 142 tests passed. Want me to commit...     │
│                                                  │
└──────────────────────────────────────────────────┘
背景: transparent (透出 windowBackgroundColor)
```

### 4.2 Hover (cursor 在 item 上)

```
┌──────────────────────────────────────────────────┐
│ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
│ ░ api-server-tests · ⬛waiting · 1min          ░ │
│ ░ All 142 tests passed. Want me to commit...   ░ │
│ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
└──────────────────────────────────────────────────┘
背景: controlBackgroundColor (浅灰 / 暗灰)
cursor: pointer
transition: background 100ms linear
```

### 4.3 Pressed (鼠标按下瞬间，< 100ms)

```
┌──────────────────────────────────────────────────┐
│ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ │
│ ▒ api-server-tests · ⬛waiting · 1min          ▒ │
│ ▒ All 142 tests passed. Want me to commit...   ▒ │
│ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ │
└──────────────────────────────────────────────────┘
背景: controlAccentColor 10% opacity (系统强调色透明)
持续: 仅按下瞬间
释放后: 触发 click → 切到 expanded
```

### 4.4 Expanded (点击后)

```
┌──────────────────────────────────────────────────┐
│                                                  │
│  api-server-tests · ⬛waiting · 1min            │  ← Row 1 不变
│  All 142 tests passed. Want me to commit...     │  ← Row 2 preview 可保留可隐藏（MVP 保留）
│                                                  │
│  ┌────────────────────────────────────────────┐ │  ← 展开区域开始
│  │                                            │ │
│  │ All 142 tests passed.                      │ │
│  │                                            │ │
│  │ Coverage: 87.3% (up from 84.1%)            │ │
│  │                                            │ │
│  │ [Bash] git diff --stat                     │ │  ← tool_use 块（MVP 简化）
│  │                                            │ │
│  │ Want me to commit with message             │ │
│  │ 'fix: token validation edge case'?         │ │
│  │                                            │ │
│  └────────────────────────────────────────────┘ │
│                                                  │
└──────────────────────────────────────────────────┘
背景: 同 hover (controlBackgroundColor) 暗示"激活"
展开区域: SF Mono Regular 12pt, labelColor
内 padding: 12pt 上下, 14pt 左右
```

---

## 5. 状态切换 (interaction state machine)

```mermaid
stateDiagram-v2
  [*] --> Default : list rendered
  Default --> Hover : mouseenter
  Hover --> Default : mouseleave
  Hover --> Pressed : mousedown
  Pressed --> Hover : mouseup (没移开)
  Pressed --> Default : mouseup (移开了)
  Hover --> Expanded : click (触发展开)
  Expanded --> Collapsed : click 同项 (收起)
  Expanded --> ExpandedNew : click 其他项 (收起当前 + 展开新)
  Default --> [*] : popup close (reset 所有展开)
```

**关键约束**（[ux-design § 5.5/5.6/6.1](../../bmad/02-planning/ux-design.md)）：

- 同一时刻最多 1 项 expanded
- 关闭 popup 重置所有展开态（下次打开默认 collapsed）

---

## 6. 4 种 status × 4 种 interaction 组合（speed grid）

| Status \ State | Default | Hover | Pressed | Expanded |
|---|---|---|---|---|
| **waiting** | yellow badge + duration | yellow badge + 灰背景 | yellow badge + accent | yellow badge + 灰背景 + 内容区 |
| **working** | green badge | green badge + 灰背景 | green badge + accent | green badge + 灰背景 + 内容区 |
| **unknown** | gray badge | gray badge + 灰背景 | gray badge + accent | gray badge + 灰背景 + "(unable to read)" |

注：unknown 状态展开内容是 `(unable to read transcript)`，不是完整消息。

---

## 7. tool_use 块渲染（MVP 简化）

### 7.1 MVP 实现

assistant 消息含 tool_use 块时，简化展示：

```
[ToolName] short args
```

示例：

| 实际 tool_use | MVP 渲染 |
|---|---|
| Bash with `git status` | `[Bash] git status` |
| Read with `/path/to/file.rs:42` | `[Read] /path/to/file.rs:42` |
| Edit with `(file, old, new)` | `[Edit] /path/to/file.rs` (省略 old/new) |
| Write with `(file)` | `[Write] /path/to/file.rs` |
| Glob with `**/*.ts` | `[Glob] **/*.ts` |
| Grep with `pattern` | `[Grep] pattern` |

→ 每行 ≤ 80 字符，超出截断 + `...`

### 7.2 v0.2+ 完整格式化（待 ADR）

按工具类型展开：
- Bash：等宽 + 语法高亮
- Read/Edit/Write：path + line range 高亮
- Glob/Grep：pattern + match count
- WebFetch/WebSearch：URL + summary

---

## 8. 长 cwd / 长消息处理

### 8.1 cwd 末段名超长

```
Default (≤ 24 char):
  my-project · ⬛waiting · 1min

Long (> 24 char, ellipsis):
  super-long-project-name-tha... · ⬛waiting · 1min

Max-width: 240pt (~30 字符等宽，~24 字 SF Pro Text Medium 13pt)
```

### 8.2 消息预览超长

```
Short (≤ 80 char):
  All tests passed. Commit?

Long (> 80 char, ellipsis):
  All 142 tests passed including the new auth flow. Coverage 87.3%, up...
```

策略：`message.split('\n')[0].slice(0, 80) + (over ? '…' : '')`

### 8.3 展开消息内容超长（> 200pt）

popup 整体不再增高（封顶 480pt），展开区域内部 scrollable：

```
┌──────────────────────────────────────────────────┐
│  api-server-tests · ⬛waiting · 1min            │
│  All 142 tests passed...                        │
│                                                  │
│  ┌────────────────────────────────────────────┐ │
│  │ Long message line 1                        │ │ ▮
│  │ Long message line 2                        │ │ ▯  ← 内部滚动
│  │ ...                                        │ │ ▯
│  │ Long message line N                        │ │ ░
│  └────────────────────────────────────────────┘ │
│                                                  │
└──────────────────────────────────────────────────┘
```

---

## 9. 排序规则视觉化

按 [user-stories H2 acceptance](../../product/user-stories.md#h2--弹开列表分诊优先级)：

```
列表顺序：
1. waiting 项（按 waiting_since asc，等得最久在最上）
2. working 项（按进程枚举顺序，MVP 不刻意排序）
3. unknown 项（按进程枚举顺序）

Example with 5 sessions:
┌────────────────────────────────┐
│ archive · ⬛waiting · 8min     │  ← 最久 waiting
│ deps    · ⬛waiting · 5min     │
│ api     · ⬛waiting · 1min     │  ← 最近 waiting
├────────────────────────────────┤
│ tests   · ⬛working            │  ← working 在 waiting 后
│ blog    · ⬛working            │
└────────────────────────────────┘
```

---

## 10. List separators

```
┌──────────────────────────────────────────────────┐
│  item 1                                          │
├──────────────────────────────────────────────────┤  ← separatorColor, 1px, 跨整宽
│  item 2                                          │
├──────────────────────────────────────────────────┤
│  item 3                                          │
└──────────────────────────────────────────────────┘
```

最后一项无下边线（borderless），让底边干净。
第一项无上边线（borderless），header 自己有下边线。

CSS：
```css
.session + .session { border-top: 1px solid var(--separator); }
```

---

## 11. Implementation checklist

实现 [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md) + [S-009](../../bmad/03-solutioning/epics/story-009-expand-message.md) 时对照本文件：

- [ ] List item 高度约 54pt（collapsed）
- [ ] cwd 末段名 SF Pro Text Medium 13pt + ellipsis 240pt
- [ ] separator `·` 用 tertiaryLabelColor
- [ ] Status badge 3 种颜色（waiting 黄 / working 绿 / unknown 灰）+ 各 light/dark 实测
- [ ] duration 仅 waiting 显示
- [ ] message preview 单行 ellipsis ~80 字
- [ ] Hover 状态变 controlBackgroundColor
- [ ] Pressed 状态变 controlAccentColor 10%
- [ ] Click 触发展开 / 切换展开
- [ ] 关 popup 重置展开态
- [ ] Expanded 区域 SF Mono 12pt
- [ ] tool_use 块 `[ToolName] args` 格式
- [ ] 长内容（> 200pt）内部滚动
- [ ] Separators 跨整宽 1px
- [ ] 排序：waiting first by duration desc，then working

---

## 12. CSS skeleton

详见 [component-css.md](component-css.md)。摘要：

```css
.session {
  padding: 10px 14px;
  cursor: pointer;
  transition: background 100ms linear;
}
.session:hover { background: var(--control-bg); }
.session:active { background: rgba(var(--accent-rgb), 0.1); }
.session.expanded { background: var(--control-bg); }

.session .row { display: flex; align-items: baseline; }
.session .name { font: 500 13px var(--sf-pro); color: var(--label); max-width: 240pt; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.session .sep { font: 11px var(--sf-pro); color: var(--tertiary-label); margin: 0 6px; }
.session .badge { font: 11px var(--sf-pro); padding: 2px 6px; border-radius: 4px; }
.session .badge.waiting { background: rgba(255, 204, 0, 0.2); color: rgb(153, 122, 0); }
.session .badge.working { background: rgba(52, 199, 89, 0.2); color: rgb(0, 122, 51); }
.session .badge.unknown { background: rgba(142, 142, 147, 0.3); color: var(--secondary-label); }
.session .duration { font: 11px var(--sf-pro); color: var(--secondary-label); margin-left: 6px; }
.session .message { margin-top: 4px; font: 12px var(--sf-pro); color: var(--secondary-label); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.session .expanded-content { margin-top: 8px; padding: 12px 14px; font: 12px var(--sf-mono); color: var(--label); white-space: pre-wrap; max-height: 200pt; overflow-y: auto; }
```
