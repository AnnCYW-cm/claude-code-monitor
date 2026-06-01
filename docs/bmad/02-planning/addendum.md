# Addendum — PRD Supplement

> **BMAD Phase 2 · Planning · PM output**
> **Status:** Draft, 跟 [PRD.md](PRD.md) 同节奏维护
>
> Addendum 收录不适合放在主 PRD 但需要文档化的补充说明：edge case 处理细则、未来 migration 注意事项、open questions 的延伸讨论、专业术语澄清。

---

## A. Edge case 处理细则

### A.1 同 cwd 多个 claude 进程的 JSONL 配对

**问题**：用户在同一目录 (`~/work/foo`) 同时启动两个 `claude` 进程，会产生两个 JSONL 文件（不同 uuid）。怎么把进程 PID 跟 JSONL 文件对应？

**MVP 策略**：
- 对每个 PID，找 `~/.claude/projects/<encoded-cwd>/` 下 mtime 最新且**最近 60s 内有写入**的 JSONL
- 多个进程在同一目录时，按"进程启动时间最接近 JSONL 创建时间"匹配

**known limitation**：极少数 case 下可能 mismatch（如两个进程几乎同时启动）。MVP 接受这个 risk。

**实测发现（2026-05-18，见 [spec/jsonl-schema.md](../../spec/jsonl-schema.md)）**：
- JSONL envelope 含 `sessionId` 字段（=文件名 uuid），但**不含进程 pid**——靠 pid 直接配对不可行
- JSONL envelope 含 `cwd` 字段，但这是 envelope 写入时的当前 cwd（用户 cd 后会变），不可靠作为配对 key
- 进程 cwd 跟 session 启动时 cwd 是否同步**未知**（[S-002 实施时 verify](../03-solutioning/epics/story-002-jsonl-locator.md)）

**调整后的策略（仍是启发式）**：
- 默认按 `<encoded-cwd>` 目录 + mtime + 活跃判定
- 如果 [S-002 实施时](../03-solutioning/epics/story-002-jsonl-locator.md) 发现 process cwd 不跟随用户 cd，沿用上面 MVP 策略 ✓
- 如果跟随了（不太可能），fallback: 扫所有 `<encoded>/` 目录找最近活跃的 `<sessionId>.jsonl`，通过启动时间匹配（实测后再 refine）

### A.2 macOS Stage Manager / Mission Control 下的 popup 位置

**问题**：用户开 Stage Manager 时，webview popup 的默认位置可能跑到屏幕外。

**MVP**：不主动定位（用 Tauri 默认）。如果跑屏外，用户可以拖回。

**v0.2+**：调用 macOS NSStatusItem API 获取 tray rect，把 popup 锚定到 tray 下方。

### A.3 macOS 多 monitor 下 tray 在哪个屏

**已知行为**：macOS 自己决定 menubar 在哪个 monitor (主显示器)，tray icon 跟随。我们不干涉。

### A.4 JSONL 文件 > 100MB 的性能

**问题**：长会话 JSONL 可能超过 100MB。我们只读最后一行，不应该 OOM。

**MVP 实现**：seek 到 EOF，向前扫到第一个 `\n`，只读最后一行。不要 `read_to_string` 整个文件。

### A.5 `~/.claude/projects/<encoded-cwd>/` 不存在

**情景**：claude 进程刚启动，JSONL 目录还没建好。
**处理**：session status = Unknown，下一轮 refresh 重试。
**对应**：FR-18 启动竞态。

### A.6 用户用 sudo 跑 claude

**已知**：sudo 下的 claude 进程，JSONL 路径在 `/var/root/.claude/projects/` 而不是 `~/.claude/projects/`。
**MVP**：不支持。检测到 claude 进程 UID 跟 monitor app 进程 UID 不一致时，跳过（不显示）。
**未来**：加 README 警告。

### A.7 用户重命名 ~/.claude/

**已知**：如果用户设了 `CLAUDE_HOME` 或 symlink ~/.claude，我们要跟随。
**MVP**：先按 `$HOME/.claude/projects/` 查；如果 `CLAUDE_HOME` 环境变量存在，优先它。
**flag**: 这个环境变量名字猜的，要 spec/jsonl-schema.md 阶段验证 Claude Code 实际是否支持这个变量。

---

## B. 未来 migration 注意事项

### B.1 Claude Code JSONL 格式 breaking change

**触发**：Anthropic 发布 Claude Code 新版本，JSONL 字段名/结构改变。

**应对**：
1. CI 测试自动校验 JSONL 假设（每个 release 跑一次）
2. 检测到 parse 失败率 > 10%，自动 alert（v0.2+ 加 telemetry）
3. 必要时发紧急小版本 patch
4. spec/jsonl-schema.md 维护多版本兼容矩阵

### B.2 Tauri 2.x → 3.x

**已知**：Tauri 仍在主版本演进，未来可能 breaking。
**应对**：每个 major bump 评估，必要时单开 branch 渐进迁移。

### B.3 macOS 16 / 17 进程枚举 API 变化

**已知**：sysinfo crate 会跟，但可能 lag。
**应对**：CI 在最新 macOS beta 上跑。

---

## C. 术语表

| 术语 | 定义 |
|---|---|
| **session** | 一个运行中的 `claude` CLI 进程 + 它当前在写的 JSONL transcript |
| **waiting** | session 的状态：Claude 完成回答在等用户输入（last role == assistant, no pending tool_use） |
| **working** | session 的状态：Claude 在思考或工具调用中 |
| **unknown** | session 的状态：JSONL 读不到或 parse 失败 |
| **tray icon** | macOS 菜单栏右侧那个图标 |
| **popup** | 点击 tray icon 弹出的小窗口 |
| **cwd** | claude 进程的当前工作目录（current working directory） |
| **JSONL** | JSON Lines，Claude Code 写的 transcript 格式，每行一个 JSON 对象 |
| **pending tool_use** | assistant 消息里包含 tool_use 块但还没对应的 tool_result——表示 claude 在等工具执行 |
| **Heavy CC user** | 同时跑 ≥3 个 claude session 的重度用户（产品 target） |
| **被动感知** | 不主动通知，靠用户抬眼扫菜单栏感知状态 |

---

## D. Open questions 延伸讨论

### D.1 为什么不做"每个 session 平均等待时长" 统计

**有人会问**：能不能在 popup 顶部显示 "今天平均每个 session 等了 X 分钟"，作为 efficiency dashboard？

**回答**：不做。理由：
- 增加注意力负担（用户多看一个数字）
- 鼓励用户"优化等待时间"，可能反而让用户不并行（违反 brief）
- 违反 L2 "fade into background"——dashboard 越花哨用户越意识到 app 存在

### D.2 为什么不接受 PR 加 "Snooze"

**有人会问**：能不能加一个 "snooze this session" 让某个 session 不计入 waiting count 5 分钟？

**回答**：不接受。理由：
- 引入 stateful 用户配置（违反 R2）
- snooze 列表本身要管理（违反 R3）
- 真要 snooze，用户可以直接 Cmd+Q app（极端但 work）
- snooze 行为本身意味着用户"觉得这个 session 不重要"——那为啥不直接 cancel 它？

### D.3 跟 Claude Code Hooks 系统的关系（如果未来出）

**情景**：如果 Anthropic 给 Claude Code 加 lifecycle hooks（如 `on_turn_complete`），我们要不要切换？

**当前判断**：可能切，但保留 JSONL fallback。理由：
- Hooks 是 0 延迟，比 polling 好
- 但 hooks 是 Claude Code-version 绑定，老版本用户用不了
- 双数据源容错性好

待 hooks 出现时单开 ADR。

---

## E. 名词选择记录（为什么不叫 X）

### E.1 不叫 "Claude Watcher"

太被动，听起来像 monitoring/surveillance。

### E.2 不叫 "Claude Code Tray"

太技术化，普通用户不知道 "tray" 是什么。

### E.3 不叫 "Sessions"

太通用，搜索发现性差。

### 最终：Claude Code Monitor

直白、SEO 友好（"claude code" 搜得到）、不假装巧妙。

---

## F. 对 PRD 的 "如果未来违反 brief 怎么办"

如果未来某个 PR 想加 R1-R5 砍掉的功能（通知 / 配置 / 标签 / 跳转 / 历史），流程：

1. PR 作者必须先开新 ADR 推翻对应 R 条
2. ADR 必须包含：为什么 brief 错了 / 新证据是什么 / 受影响范围
3. 作者 review，必要时拒绝
4. 接受的话，更新 PRD 移除对应 R / FR-N 条

→ **防 scope creep 的硬约束**。

---

## G. 文档自身的元约定

### G.1 这份 addendum 跟主 PRD 的边界

- **进 PRD**：明确的 functional / non-functional requirement、user segments、release plan
- **进 addendum**：edge case 处理细则、未来 migration、术语、为啥不做某事、命名理由
- **进 decision-log**：技术/架构决策（ADR 格式）

### G.2 这份 addendum 的更新触发

- 实测某个 edge case 后发现 MVP 策略错 → 改这里
- 开始写 spec/jsonl-schema.md 后发现假设错 → 改这里
- 收到用户 issue 是 PRD 没 cover 的小问题 → 进这里
