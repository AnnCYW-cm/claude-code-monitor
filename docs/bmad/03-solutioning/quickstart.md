# Quickstart — 001-mvp

> **Dev setup + acceptance test plan + dogfood checklist** — originally from Spec Kit `/speckit.plan` supporting, mv to BMAD by [ADR-013](../02-planning/decision-log.md)
> **Status:** Draft → 实施期更新
>
> 给 contributor / dogfood user 的「从 0 到验证 MVP 工作」步骤。也是 release 前的 acceptance test plan。

---

## 1. Dev setup

### 1.1 Prerequisites

```bash
# 验证：
node --version       # 18+
npm --version        # 10+
rustc --version      # 1.77+
cargo --version      # 1.77+
git --version

# macOS：
sw_vers              # ProductVersion: 12 / 14 / 15
```

如缺少：
- Node: `brew install node`
- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 1.2 Clone + build

```bash
git clone https://github.com/<owner>/claude-code-monitor.git
cd claude-code-monitor
npm install                # ~3 min, installs Tauri CLI etc
npm run tauri:dev          # first time: 10-15 min (downloads ~500MB crates)
```

成功标志：
- terminal 输出 `Built application` 等字样
- 菜单栏出现 tray icon (黑色圆点)
- 没有报错

### 1.3 Build release

```bash
npm run tauri:build
```

输出位置：
- `.app`: `src-tauri/target/release/bundle/macos/ClaudeCodeMonitor.app`
- `.dmg`: `src-tauri/target/release/bundle/dmg/`

---

## 2. Acceptance test plan

实施完成后跑一遍，全过即可 release。

### 2.1 Discovery (FR-D)

**Test: 多 session 发现**

```
准备：
1. 关掉所有 claude session
2. 启动 ClaudeCodeMonitor.app
3. 点击 tray icon → 验证 popup 显示 "no claude sessions running"

操作：
4. 开 terminal 1, cd ~/work/proj-a, 启动 `claude`，给个任务
5. 开 terminal 2, cd ~/work/proj-b, 启动 `claude`，给个任务
6. 开 terminal 3, cd ~/personal/blog, 启动 `claude`，给个任务

验证：
✓ 等 2s, 点击 tray → popup 显示 3 个 session
✓ 每个 session 显示项目名 (proj-a / proj-b / blog)
✓ 每个 session 显示 status (waiting/working)
```

**Test: session 退出**

```
准备：上述 3 session 运行中
操作：在 terminal 1 输入 /quit 退出 claude
验证：
✓ 2s 内 popup 列表自动移除 proj-a
✓ tray title (waiting count) 同步更新
```

**Test: 启动竞态**

```
操作：
1. 开 terminal，cd ~/test-new-proj
2. 立刻启动 claude
3. < 1s 内点击 monitor tray
验证：
✓ proj 出现在列表，status=unknown
✓ 等 2s 后 status 切换到 working 或 waiting
```

### 2.2 Classification (FR-C)

**Test: Waiting state**

```
准备：1 个 claude session 正在跑长任务
操作：等 claude 完成回答
验证：
✓ 2s 内 status 切换为 waiting
✓ tray title 显示 "1"
✓ 列表项显示已等时长（开始时 "just now"，1min 后 "1min"）
```

**Test: Working state**

```
准备：1 个 claude session
操作：给 claude 一个新任务，claude 开始思考
验证：
✓ status 显示 working
✓ tray title 减 1（或显示空白）
✓ 列表显示 last assistant message preview
```

**Test: Unknown state**

```
准备：1 个 claude session 运行中
操作：
1. 找到该 session 的 JSONL 文件路径（用 lsof 或猜）
2. echo "corrupt" >> <jsonl>  # 故意破坏
验证：
✓ 下一轮 refresh 后该 session status=unknown
✓ last_message 显示 "(unable to read transcript)"
✓ 修复 JSONL（删后等 claude 重写）后下一轮自动恢复
```

### 2.3 Presentation (FR-P)

**Test: tray icon visibility**

```
✓ 启动 app 后 < 3s tray icon 出现在 menubar
✓ tray icon 在 light + dark mode 都可见且对比度足够
✓ tray icon 跟随系统 light/dark 自动变色 (template image)
```

**Test: tray title count**

```
✓ 0 waiting → tray 仅显示 icon (无数字)
✓ 1 waiting → tray 显示 "1"
✓ 3 waiting → tray 显示 "3"
✓ tray title 跟 popup 列表 waiting 计数永远一致
```

**Test: popup show/hide**

```
✓ 左键点 tray → popup 弹出 < 200ms
✓ 再左键点 tray → popup 隐藏
✓ 隐藏后 app 仍在运行（tray icon 在）
```

**Test: native menu (Quit)**

```
✓ 右键点 tray → 弹出 native menu，含 "Quit"
✓ 点击 Quit → app 完整退出（tray icon 消失）
✓ ps -ax | grep ClaudeCodeMonitor → 无残留进程
```

**Test: list render**

```
✓ 3 session waiting + 2 working → 列表显示 5 行
✓ 顺序：3 waiting 在前（按时长降序），2 working 在后
✓ 每行：项目名 / 状态徽章 / 时长 / 单行消息预览
✓ 状态徽章颜色：waiting 黄 / working 绿 / unknown 灰
✓ 项目名超长用 ellipsis (...)
✓ 消息预览超长用 ellipsis
```

**Test: list scroll**

```
准备：开 8 个 claude session（不同 cwd）
✓ popup 列表前 6 行可见，第 7-8 行需滚动
✓ 滚动条样式跟随 macOS (auto-hide)
```

**Test: expand/collapse**

```
✓ 点击列表项 → 该项下方展开完整 last assistant message
✓ 字体：SF Mono
✓ 再点同项 → 收起
✓ 点其他项 → 收起当前 + 展开新项
✓ 关闭 popup 再打开 → 默认全部 collapsed
```

**Test: empty state**

```
准备：关掉所有 claude session
✓ popup 显示 "no claude sessions running"
✓ 副标题 "start a session with `claude` in your terminal"
✓ 字体居中
```

### 2.4 Reliability (FR-R)

**Test: per-session error isolation**

```
准备：3 个 session，corrupt 中间一个 JSONL
✓ 1 个 session status=unknown
✓ 另 2 个 session 正常显示
✓ 整个 app 不挂
✓ log 文件有 ERROR 条目记录原因
```

**Test: log file**

```
✓ 文件存在: ~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log
✓ 启动后第一行是 startup banner
✓ ERROR / WARN / INFO 各 level 都能看到
✓ format: 2026-05-18T... [LEVEL] msg
```

**Test: restart recovery**

```
准备：app 运行中，3 个 session
操作：Quit app → 立即重新打开
✓ 重启后 2s 内 popup 显示原 3 session
✓ status 正确恢复
```

### 2.5 Negative requirements (FR-N)

**Test: no notification (grep)**

```bash
grep -rn "Notification\|notify\|osascript" src-tauri/src/
# Expected: no matches
grep -rn "alert(\|Notification" src/
# Expected: no matches
```

**Test: no config UI**

```
✓ popup 内无设置图标 / 偏好按钮
✓ 右键 menu 只有 Quit, 无 "Preferences" / "Settings"
✓ 无任何要求填字段的弹窗
```

**Test: no network**

```bash
grep -rn "reqwest\|hyper\|http::" src-tauri/src/
# Expected: no matches

# 或运行时验证：
sudo lsof -i -P | grep ClaudeCodeMonitor
# Expected: 无 outbound connection
```

**Test: no jump-to-tab**

```bash
grep -rn "osascript\|AppleScript\|iTerm" src-tauri/src/
# Expected: no matches
```

**Test: no historical sessions**

```
准备：1 session 已退出 (JSONL still in ~/.claude/projects/)
✓ popup 不显示该 session
```

### 2.6 Performance (NFR-P)

**Test: list_sessions duration (benchmark)**

```bash
cargo bench --manifest-path=src-tauri/Cargo.toml
# Expected:
# list_sessions (10 sessions) < 50ms
# list_sessions (15 sessions) < 100ms
```

**Test: popup show time (manual)**

```
1. console.time("popup") 在 frontend
2. 点击 tray
3. 看 console.timeEnd
✓ < 200ms (typically 50-100ms)
```

**Test: idle CPU**

```
Activity Monitor → ClaudeCodeMonitor → CPU %
✓ 空闲时 < 0.5% avg (M1)
```

**Test: 24h memory**

```
1. 记录启动时 RSS (Activity Monitor)
2. 跑 24 小时
3. 记录 RSS
✓ 增长 < 50MB
```

### 2.7 Compatibility (NFR-C)

**Test: macOS version**

在 macOS 12 / 14 / 15 各一台机器上跑一遍上述全部 acceptance test。

**Test: terminal emulator neutrality**

```
准备：iTerm + Terminal.app 各开一个 claude
✓ 两个 session 都被发现
✓ 两个都能正确分类 status
```

### 2.8 Distribution (NFR-D)

**Test: DMG size**

```bash
ls -lh src-tauri/target/release/bundle/dmg/
# Expected: < 15MB
```

**Test: install flow**

```
1. DMG mount
2. 拖 .app 到 Applications
3. 双击 .app → Gatekeeper 警告
4. 按 install.md 步骤 bypass
5. tray icon 出现
✓ 总步骤 ≤ 3
```

---

## 3. Manual dogfood checklist (14 day)

### Daily

- [ ] App 启动正常
- [ ] tray title 跟实际 waiting 数一致
- [ ] popup 弹出无延迟
- [ ] 没有任何打断（无通知/声音/弹窗）

### Weekly

- [ ] 查 log file，无重复 ERROR
- [ ] Activity Monitor 看 RSS 没飙
- [ ] 没有"啊为什么不通知" 的内心冲动（如有，写到 retrospective）

### End of 14 days

- [ ] 完成 retrospective.md
- [ ] 列实际触发的 user stories (验证 [scenarios](../../product/scenarios.md))
- [ ] 列 14 天里发现的 issue
- [ ] 决定是否 alpha release

---

## 4. Quick checklist (grep-able, for CI / 批量 verify)

上面 § 2 是详细 test plan（含准备/操作/验证步骤），下面是同样验收点的 grep-friendly 短 checklist——CI 自动化或快速 progress tracking 用。

### Discovery
- [ ] 启动 3 个真实 claude session，列表正确显示 3 个
- [ ] 关掉其中一个，列表自动移除（≤ 2s 内）
- [ ] claude 进程在但 JSONL 还没建好时，session 显示 status=Unknown
- [ ] sudo 跑的 claude 不出现在列表（UID 不一致）

### Classification
- [ ] 让 claude 完成回答 → session 显示 waiting
- [ ] 让 claude 思考中 → session 显示 working
- [ ] 故意 corrupt JSONL → session 显示 unknown，不消失
- [ ] 下一轮 refresh 自动重试

### Presentation
- [ ] Menubar 出现 icon
- [ ] 1 个 waiting → tray 显示 "1"
- [ ] 0 个 waiting → tray 显示空字符串（仅图标）
- [ ] 左键点 tray → popup < 200ms 弹出
- [ ] 右键点 tray → native menu 显示 Quit
- [ ] 列表每行：项目名 / 状态徽章 / 时长 / 消息预览
- [ ] 排序：waiting 在上按时长降序，working 在下
- [ ] 点列表项展开完整消息
- [ ] 同时最多 1 项展开
- [ ] 关闭 popup 重置展开态
- [ ] 0 session 显示 empty state

### Reliability
- [ ] 单 session JSONL corrupt 不影响其他 session
- [ ] 故意触发 panic 不挂 app（单 session unknown）
- [ ] log 写入 `~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`
- [ ] 重启 app 状态完整恢复

### Negative requirements (CI grep verify)
- [ ] `grep -rn "Notification\|notify\|osascript" src-tauri/src/` 无匹配
- [ ] `grep -rn "alert(\|Notification" src/` 无匹配
- [ ] `grep -rn "reqwest\|hyper\|isahc" src-tauri/src/` 无匹配
- [ ] `grep -rn "react\|vue\|svelte" package.json` 无匹配
- [ ] 没有 settings UI 入口（人工 verify popup）
- [ ] 不显示退出的 session

### Performance
- [ ] benchmark: 10 session 单次 invoke < 50ms
- [ ] benchmark: 15 session 单次 invoke < 100ms
- [ ] 实测: popup 弹出 < 200ms (用 console.time)
- [ ] 实测: 24h 后 Activity Monitor RSS 增长 < 50MB
- [ ] 实测: 空闲 CPU < 0.5% (M1) - 见 [implementation-readiness § 2.1](implementation-readiness.md) 风险

### Compatibility
- [ ] macOS 12 实测
- [ ] macOS 14 实测
- [ ] macOS 15 实测 (Sequoia Gatekeeper 特别注意)
- [ ] iTerm + Terminal.app 各开 1 个 claude 都被发现
- [ ] M1 + Intel 都跑（如有 Intel 机器）

### Distribution
- [ ] DMG 大小 < 15MB
- [ ] [`guides/install.md`](../../guides/install.md) 写完，三种方式 (DMG / brew / source) 都说清
- [ ] Gatekeeper bypass 实测 (各 macOS 版本)
- [ ] 项目根 README 含 quick start

---

## 5. Cross-reference

| Topic | Where |
|---|---|
| PRD (产品需求) | [PRD.md](../02-planning/PRD.md) |
| Architecture | [architecture.md](architecture.md) |
| Tasks (30 个 TDD-ordered) | [tasks.md](tasks.md) |
| Data model | [data-model.md](data-model.md) |
| IPC contract | [../../../spec/ipc-contract.md](../../spec/ipc-contract.md) |
| Research notes (技术选型背景) | [research-notes.md](research-notes.md) |
| Stories (acceptance details) | [docs/product/user-stories.md](../../product/user-stories.md) |
| Scenarios (实际剧本) | [docs/product/scenarios.md](../../product/scenarios.md) |
| Constitution (项目治理) | [../../../constitution.md](../../constitution.md) |
