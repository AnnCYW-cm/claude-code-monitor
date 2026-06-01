# Animations — Timing + Visualization

> **Status:** v0.1 design freeze
> **Cross-ref:** [ux-design § 11 Animation budget](../../bmad/02-planning/ux-design.md), [constitution III.3](../../constitution.md)

---

## 1. Total animation budget

整个 app **只有 4 种动效**，全部在 popup 内：

| # | Animation | Duration | Easing | When |
|---|---|---|---|---|
| 1 | Popup show | 150ms | ease-out | tray left-click → popup 出现 |
| 2 | Popup hide | 100ms | ease-in | tray left-click (again) → popup 消失 |
| 3 | List item hover | 100ms | linear | mouseenter / mouseleave |
| 4 | List item expand/collapse | 150ms | ease-out | 点击列表项展开 / 收起 |

**全部禁用**的动效（[P1 不打扰](../../bmad/02-planning/ux-design.md) 红线）：
- ❌ tray icon 任何变化（数字 / 颜色 / pulse）
- ❌ tray icon 闪烁
- ❌ list 排序变化时的 transition
- ❌ session 进出列表的 transition
- ❌ spinner / loading animation（refresh 用文字状态 `...`）
- ❌ status badge 颜色 transition
- ❌ Popup show 时的 background blur fade
- ❌ 滚动条 transition

---

## 2. Animation 1: Popup show (150ms)

### 2.1 Timeline

```
T=0ms           T=37ms          T=75ms          T=112ms         T=150ms
┌─┐             ┌──────┐        ┌────────┐      ┌──────────┐    ┌──────────┐
│·│             │░░░░░░│        │ Header │      │ Header   │    │ Header   │
└─┘             │░░░░░░│        │ ░░░░░░ │      │ Content  │    │ Content  │
                └──────┘        └────────┘      └──────────┘    └──────────┘
opacity: 0      opacity: 0.4    opacity: 0.7    opacity: 0.95   opacity: 1
scale: 0.95     scale: 0.96     scale: 0.98     scale: 0.99     scale: 1
```

### 2.2 Easing curve

```
opacity:
1.0 │                                    ╭───
    │                              ╭─────
    │                        ╭─────
    │                  ╭─────
    │            ╭─────
    │      ╭─────
    │  ╭───
0.0 ├──┴─────────────────────────────────────
    0ms                              150ms

ease-out (cubic-bezier(0, 0, 0.2, 1)):
- 开始快，结尾慢
- 让用户感觉"瞬间出现+稳定停下"

scale: 0.95 → 1.0 同时间线，同 ease-out
```

### 2.3 实现 (CSS)

```css
@keyframes popup-show {
  from { opacity: 0; transform: scale(0.95); }
  to   { opacity: 1; transform: scale(1); }
}

.popup-visible {
  animation: popup-show 150ms cubic-bezier(0, 0, 0.2, 1);
}
```

### 2.4 边界

- 在 animation 期间用户再点 tray → 忽略（debounce by `is_visible()`）
- animation 期间用户按 Esc / ⌘W → 等 animation 完成后再 hide（避免抢动）

---

## 3. Animation 2: Popup hide (100ms)

### 3.1 Timeline

```
T=0ms           T=50ms          T=100ms
┌──────────┐    ┌──────────┐    ┌─┐
│ Header   │    │░░░░░░░░░░│    │ │     (空)
│ Content  │    │░░░░░░░░░░│    └─┘
└──────────┘    └──────────┘
opacity: 1      opacity: 0.5    opacity: 0
scale: 1        scale: 1        scale: 1 (不 shrink)
```

### 3.2 Easing curve

```
opacity:
1.0 ├──╮
    │   ╲
    │    ╲
    │     ╲
    │      ╲___
    │          ╲___
0.0 ├──────────────╲──
    0ms          100ms

ease-in (cubic-bezier(0.4, 0, 1, 1)):
- 开始慢，结尾快
- 让用户感觉"轻轻消失"
- hide 比 show 短 50ms（更迅速消失，不滞留）
```

**注**：hide 时**不 shrink scale**（保持 1.0）—— 仅 fade。原因：scale 收缩+fade 同时显得"被吞回"，不自然；纯 fade 显得"轻飘消失"。

### 3.3 实现

```css
@keyframes popup-hide {
  from { opacity: 1; }
  to   { opacity: 0; }
}

.popup-hiding {
  animation: popup-hide 100ms cubic-bezier(0.4, 0, 1, 1) forwards;
}
```

### 3.4 状态转换

```
visible → hiding (set animation class)
        → 100ms 后 → hidden (animation end, set display: none)
```

---

## 4. Animation 3: List item hover (100ms)

### 4.1 Timeline

```
T=0ms                   T=50ms                  T=100ms
┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│ item             │    │░░ item          ░│    │ █ item          █│
└──────────────────┘    └──────────────────┘    └──────────────────┘
background:             background:             background:
transparent             0.5 × controlBg         controlBg
```

### 4.2 Easing

```
linear (跟系统 hover 行为一致)
```

linear 而不是 ease-out 的原因：
- hover 行为是用户**正在移动鼠标**，鼠标位置是匀速的
- 用 ease-out 会有"过冲后停下"的违和感
- linear 跟系统 button hover 一致

### 4.3 实现

```css
.session {
  transition: background 100ms linear;
}
.session:hover {
  background: var(--control-bg);
}
```

### 4.4 鼠标离开

- mouseleave → 反向同 transition（100ms linear, transparent）
- transition 在两个方向都生效，无需独立 animation

---

## 5. Animation 4: List item expand/collapse (150ms)

### 5.1 Timeline (展开)

```
T=0ms                   T=75ms                  T=150ms
┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│ name · badge     │    │ name · badge     │    │ name · badge     │
│ preview ...      │    │ preview ...      │    │ preview ...      │
└──────────────────┘    │ ░░░░░░░░░░░░░░░░ │    │ ┌──────────────┐ │
                        │ ░░░░░░░░░░░░░░░░ │    │ │ full message │ │
height: 54pt            └──────────────────┘    │ └──────────────┘ │
opacity (expand): 0     height: ~80pt           └──────────────────┘
                        opacity: 0.5            height: ~130pt
                                                opacity: 1
```

### 5.2 Easing

```
ease-out (cubic-bezier(0, 0, 0.2, 1))
- 跟 popup show 一致
- 内容"展开"感
```

### 5.3 实现

```css
.session .expanded-content {
  max-height: 0;
  opacity: 0;
  overflow: hidden;
  transition: max-height 150ms cubic-bezier(0, 0, 0.2, 1),
              opacity 150ms cubic-bezier(0, 0, 0.2, 1) 50ms;  /* 50ms delay for content */
}

.session.expanded .expanded-content {
  max-height: 200pt;  /* 上限，超出走 internal scroll */
  opacity: 1;
}
```

注：opacity 比 height transition 慢 50ms 启动——避免高度变化时内容闪烁出现。

### 5.4 收起 timeline

收起反过来：

```
T=0ms                   T=150ms
┌──────────────────┐    ┌──────────────────┐
│ name · badge     │    │ name · badge     │
│ preview ...      │    │ preview ...      │
│ ┌──────────────┐ │    └──────────────────┘
│ │ full message │ │    height: 54pt
│ └──────────────┘ │    opacity (expand): 0
└──────────────────┘
height: ~130pt
opacity: 1
```

CSS 自动反向（同 transition）。

---

## 6. 关键设计决策

### 6.1 为什么 popup show 150ms 而 hide 100ms？

- show 150ms：让用户感知"东西出现"，给眼睛适应时间
- hide 100ms：消失要果断，不滞留视野
- 不对称是有意的（参考 iOS / macOS system animation）

### 6.2 为什么 hover 用 linear 而 show/hide 用 ease curve？

- hover 跟随用户鼠标移动（用户主动驱动）→ linear
- show/hide / expand 是 app 主导的状态切换 → ease 给"自然感"

### 6.3 为什么 expand 用 max-height 而不是 height?

- `height: auto` 在多数浏览器/WebView **不能 transition**（CSS 2024 新提案 `interpolate-size: allow-keywords` 支持，但 Safari/WebKit 支持滞后）
- `max-height: 0 → 200pt` 可靠地 animate
- 副作用：max-height 200pt 即使内容更短也设到 200pt——但 overflow: hidden 不影响视觉
- 如果未来 WebKit 全支持 `interpolate-size`，可切回 `height: auto` 让 max-height 限制不必要

### 6.4 为什么不做"列表项进出 list" animation？

- session 出现 / 退出列表是 polling 自然结果（每 2s）
- 加 fade-in / slide-in 会让 user 感知"东西在变"（违反 P1 不打扰）
- MVP 接受瞬移；v0.2+ 评估

---

## 7. Animation testing

### 7.1 视觉自查

每个 animation 实测后用 macOS 自带屏幕录制（Cmd+Shift+5）录一段，主观评估：
- 顺滑度（无卡顿）
- 时长感觉（不太快不太慢）
- easing 自然度

### 7.2 性能验证

Chrome DevTools (Tauri webview 用 WebKit，但能用 Chrome devtools 接 debug)：
- Performance tab 录制
- 检查 frame rate ≥ 60fps
- 检查无 jank / layout thrashing

---

## 8. Implementation checklist

实现各 animation 时对照：

### Popup show/hide ([S-007](../../bmad/03-solutioning/epics/story-007-popup-window.md))
- [ ] Show animation 150ms ease-out (opacity + scale)
- [ ] Hide animation 100ms ease-in (仅 opacity)
- [ ] hide 后 set `display: none`（不只是 opacity:0）
- [ ] Show 时 set `display: block` 后才能 animate
- [ ] 实测 60fps 无 jank

### List item hover ([S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md))
- [ ] background transition 100ms linear
- [ ] hover 离开反向 transition
- [ ] cursor: pointer

### List item expand ([S-009](../../bmad/03-solutioning/epics/story-009-expand-message.md))
- [ ] max-height transition 150ms ease-out
- [ ] opacity transition 150ms ease-out + 50ms delay
- [ ] 内容 max-height 200pt
- [ ] 超过 200pt internal scroll
- [ ] 收起反向 animation

### Forbidden (CI grep + review 把关)
- [ ] 无 `tray icon animation` / `tray pulse`
- [ ] 无 `badge color transition`
- [ ] 无 `loading spinner`
- [ ] 无 sort change animation

---

## 9. CSS 全部 keyframes 汇总

```css
/* ============================================ */
/* Animation 1: Popup show                      */
/* ============================================ */
@keyframes popup-show {
  from { opacity: 0; transform: scale(0.95); }
  to   { opacity: 1; transform: scale(1); }
}

.popup--visible {
  animation: popup-show 150ms cubic-bezier(0, 0, 0.2, 1);
}

/* ============================================ */
/* Animation 2: Popup hide                      */
/* ============================================ */
@keyframes popup-hide {
  from { opacity: 1; }
  to   { opacity: 0; }
}

.popup--hiding {
  animation: popup-hide 100ms cubic-bezier(0.4, 0, 1, 1) forwards;
}

/* ============================================ */
/* Animation 3: List item hover                 */
/* ============================================ */
.session {
  transition: background 100ms linear;
}

/* ============================================ */
/* Animation 4: List item expand                */
/* ============================================ */
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
}
```
