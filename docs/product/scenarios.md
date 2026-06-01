# User Scenarios

> **Status:** Draft
>
> 5 个剧本，覆盖典型 / 重负载 / 出错 / onboarding / 稳定使用 五种使用日。
>
> **结构**：每个剧本含 `上下文 → 时间线 → 关键观察 → 引用的 user stories → 验证标准`。
>
> **跟 [user-stories.md](user-stories.md) 的关系**：stories 是原子单元，scenarios 是它们的具体组合。剧本里的每个动作都能追溯到一个或多个 story ID（H1/E2/F1 等）。如果剧本里出现某个动作无法对应任何 story，说明 stories 漏了，要回头补。

---

## S1 — 典型周二下午（3 session 并行）

### 上下文

下午 2 点，你（user）正在 3 个项目上并行工作：
- **A**：在 `~/work/api-server` 写一个新 endpoint（你正在 IDE 里打字）
- **B**：在 `~/work/api-server-tests` 跑回归测试（claude 自己在跑）
- **C**：在 `~/personal/blog` 改一篇博客的 markdown（claude 在改格式）

监控 app 已经常驻菜单栏 2 周，你已经把扫菜单栏的动作内化为肌肉记忆。

### 时间线

**14:00**
你在 A 终端里敲完一段代码，停下来想下一步。
扫一眼菜单栏：`👁 0`（没有等你的）。
继续打字。

**14:08**
余光感觉菜单栏有变化。
瞄一眼：`👁 1`。
（→ H1）

**14:08:30**
左键点击 tray，popup 弹出：

```
api-server-tests  ·  waiting · just now   "All 142 tests passed. Want me to commit
                                            with message 'fix: token validation edge case'?"
blog              ·  working              "Reformatting headings to ATX style..."
```

（A 项目里你只在 IDE 里手写代码，没启动 claude 进程，所以 A 不出现在列表里——只有运行中的 claude 进程才进列表）

（→ H2：分诊；只有 1 个 waiting 不需要排序决策）

你瞬间明白：B 完了在等 commit 决策，C 还在跑。
你权衡：要不要现在切去？B 的"commit yes/no"很轻，30 秒搞定。

**14:08:45**
Cmd+Tab → B 终端 → 输入 `y` → 回车 → claude 提交。
菜单栏：`👁 0`，B 从列表消失（claude 提交完会退出 session）。
（→ E6：平滑移除）

**14:09**
点 popup 外区域关闭。回 A 继续写代码。

**14:32**
完成 A 的一段功能，正想跑测试。
扫一眼菜单栏：`👁 1`。
弹开：C · waiting · 4min · "Reformatted 8 headings. Diff is +12/-12. Review the changes?"
你判断：不急，C 是个人项目，晚点再说。
点 popup 外关闭，继续 A 工作。
（→ H3 的关键例子：读了关键信息但选择**不**切走）

**14:33**
你为 A 启动新一轮测试，开第四个 claude session：
- **D**：在 `~/work/api-server` 跑测试

菜单栏：`👁 0`（D 在跑还没完成）。

**14:45**
浏览器查了下文档。回头扫菜单栏：`👁 2`。
弹开：

```
api-server  ·  waiting · 2min    "All tests passed including the new endpoint.
                                   16 new test cases. Want to commit?"
blog        ·  waiting · 17min   "Reformatted 8 headings. Diff is +12/-12..."
```

你 Cmd+Tab → D（api-server 那个） → 确认 commit。
（→ H2 又一次，现在 2 个 waiting 需要选——D 先）

**15:30**
工作完结。一天里你扫菜单栏约 12 次，弹开列表 4 次。
弹开后实际切走的次数：3 次。
真正被 app "救" 的次数：1 次（14:08 的 B commit，如果没看见可能漏 20 分钟）。

### 关键观察

- 整天你扫菜单栏 12 次，但只在状态变化时才弹开（约 1/3 的扫视会引发动作）——这就是"被动感知"的工作方式
- app 的存在感是"几乎为 0"——你不会想它，但需要时它在
- 没有任何通知打断你
- D 在跑时你完全不需要查看 D，因为你知道"等你 = 弹出来"
- 切回 B 之后 B 就自然退出，列表自然变化，无需手动管理

### 引用到的 stories

H1, H2, H3, E6

### 验证标准

- 这是 product MVP 的"正常运行"基准——所有相关 stories 在这一天都被自然触发
- 如果未来某次 implement 后此剧本走不通（例如：弹出 popup 要 > 1s），就是 regression
- 关键 SLA：H2 的"< 200ms 弹出"、H3 的"完整消息可读"必须在这一天每次都成立

---

## S2 — 重负载日：8 个 session 并发

### 上下文

周四，你接手了一个老项目的大重构，同时还有几条次要工作线：
- **A**：主项目的重构主干（你在 IDE 写，不开 claude）
- **B**：A 项目的单元测试
- **C**：A 项目的集成测试
- **D**：文档同步（claude 改 README）
- **E**：兄弟项目的小 bugfix
- **F**：依赖升级的 changelog 分析（claude 在分析 156 个包）
- **G**：旧文档归档（claude 在 mv 文件）
- **H**：博客文章润色（个人项目，低优先级）

7 个 claude session + 你自己的 IDE 工作 = 8 条并发轨道。你处于压力状态。

### 时间线

**10:00**
扫一眼菜单栏：`👁 3`。
心想"好戏开始"。

**10:00:30**
弹开列表（8 项，需要滚动看到第 7-8 个）：

```
[按 waiting 时长降序排序]

B  ·  waiting · 3min   "127 tests pass, 3 fail. Want full failure output?"
D  ·  waiting · 2min   "README updated. The diff touches 23 sections.
                        Should I split into smaller PRs?"
E  ·  waiting · 1min   "Bug fixed in commit a4b2. Want me to push?"
C  ·  working          "Running integration suite (16 of 42 complete)..."
F  ·  working          "Analyzing 156 dependency updates..."
G  ·  working          "Moving 14 archived posts..."
H  ·  working          "Polishing intro paragraph..."
```

（→ H2：分诊；→ E4：列表压力；→ L3：从 3 session 成长为 8 session 的承压点）

你的分诊逻辑：
1. **B** 的 3 fail 可能是阻塞 → 最先看
2. **E** 的 push 是低风险确认 → 第二
3. **D** 的"是否分拆 PR"是策略性的，要思考一会儿 → 第三

**10:01-10:05**
- Cmd+Tab → B → "yes, full output" → 看到 3 个 failures
  - 1 个是真 bug，2 个是 brittle assertion
  - 让 claude 修真 bug、删 brittle 的两个 → B 重新跑 → working
- 回菜单栏，扫一眼

**10:05:30** 菜单栏：`👁 2`（D 和 E 还在等；B 已变 working）
**10:06** 切到 E → "yes push" → E 完成 → 退出
**10:07** 菜单栏：`👁 1`（D；E 从列表消失，B 重测中）
**10:08** D 的"是否分拆 PR"需要 10 分钟思考，先放着
**10:09** 回 A，继续重构

**10:25**
扫菜单栏：`👁 4`。 心想"草。"
弹开：

```
G  ·  waiting · 8min    "Archived 14 posts. Listed paths."
F  ·  waiting · 5min    "156 deps analyzed. 12 breaking changes flagged.
                         Top concern: react-router v6 → v7."
C  ·  waiting · 3min    "Integration tests complete: 38 pass, 4 fail.
                         Categorized: 2 timeout, 1 race, 1 logic. See details?"
B  ·  waiting · 1min    "Bug fixed, tests pass. Diff in commit b9d1."
D  ·  working           "..."  (D 你之前没回，它没继续——D 真的在等你)
H  ·  working           "Polishing conclusion now..."
```

你的反应：
- 心智上有点超载——4 个 waiting + 1 个 needs decision (D)
- 决定批处理：5 分钟"分诊冲刺"模式，每个 waiting 处理到"下一步明确"为止，不一定完成
- 顺序：C 的 4 fail 优先（新信息）；F 的 breaking changes 阅读性任务，可以晚一点；G 和 B 是 acknowledge & move on

（→ H2 在压力下的展现；→ L3）

**10:25-10:45** 你逐个处理 8 个 session 的状态：

- B: 看 diff → "yes commit" → 完成
- C: 看 4 failures → 让 claude 重跑 timeout 类（可能是机器负载）→ working
- F: "yes show the react-router migration guide" → working
- G: "yes, push the archive commit" → 完成
- D: 决定分拆 → "split by section: structure first, then API docs, then examples" → working
- E 已经早就退出

**10:45**
菜单栏：`👁 0`。 你深呼吸，回 A。
心想"还好有这个 app，否则我可能漏掉 G 或 F。"

### 关键观察

- **列表滚动**：8 session 时 popup 需要滚动 1 屏——E4 的承压验证
- **分诊行为**：用户在 stress 时按"风险 × 影响 / 操作成本"排序，不是 first-in-first-out
- **waiting 数字动态**：3 → 2 → 1 → 0 → 4 → 0，是用户压力释放的可视化
- **app 不打断**：即使在 stress 下，app 仍然只是"被动陈列"，不主动 push——这是它能在压力日里"保持有用"而不是"成为新压力源"的原因
- **隐性救援**：用户提到"否则我可能漏掉 G"——这是 app 的真实价值，但用户大部分时间不会意识到

### 引用到的 stories

H1, H2, H3, E4, E6, L3

### 验证标准

- E4 的"超过 6 条滚动" + "< 100ms invoke" 在此剧本被实测（8 session 下滚动 + 响应都正常）
- 关键 UX：分诊期间用户扫一眼能看出"哪个等的时间最长"（默认按 waiting 时长降序排序）
- 失败信号：如果用户在 stress 下抱怨"找不到 X session"，说明排序逻辑或列表密度需要 review

---

## S3 — 出错日：JSONL 损坏（附录：app 自崩）

### 上下文

周五下午 4 点，磁盘空间告警你忽略了 2 小时。3 个 session 在跑：A（你写代码）、B（跑测试）、C（写 docs）。

### 时间线

**16:00** 正常工作。菜单栏：`👁 0`。

**16:15** disk full event triggered。
- B 在写 JSONL 时遇到 ENOSPC（no space left），JSONL 写入截断
- 监控 app 下一轮 refresh（2s 后）：
  - 列表里 B · **Unknown** · "(unable to read transcript)"
- 你扫一眼菜单栏，发现少了一个数字（之前 `👁 1` 现在 `👁 0`）
- 弹开看：B 显示 Unknown
- 你心想"奇怪，B 应该在跑啊"

（→ F1：JSONL 损坏；→ E2 反向：状态不一致但不消失）

**16:16**
你切到 B 终端看：claude 进程还在，但显示 "Error writing transcript: no space left"。
你清理磁盘空间：`rm -rf ~/Downloads/old-stuff` 释放 50GB。
（→ F1 acceptance "自动重试不需用户干预"，但你也手动看了一眼——多一个验证点）

**16:16:30**
下一轮 refresh：B · waiting · "Encountered disk error, partial output above. Should I retry the test run?"
菜单栏：`👁 1`。
你切到 B：yes retry。

（→ F1 acceptance "下一轮 refresh 自动重试" 验证）

**16:25** （**附录场景：假设 app 自崩**）监控 app 自己 panic 了
- 假设原因：sysinfo 某次系统调用上 SIGSEGV（**虚构例子，非已知 bug**——用来验证 F3 的恢复 UX）
- 菜单栏 tray icon 消失
- 你的第一反应（5 秒后）："咦，菜单栏怎么少了个东西？"
- 第二反应："哦，app 崩了"
- 你在 Finder 找到 ClaudeCodeMonitor.app → 双击 → tray icon 回来
- 你心想"好歹我能注意到，但下次最好别崩"

（→ F3：app 自崩 + 重启）

**16:30**
一切恢复正常，3 session 都在列表里。你继续工作。
你在脑子里记下：F2（卡死检测）和 F3 自动守护可能是 v0.2 的候选。

### 关键观察

- **F1 的关键 UX**：B 变 Unknown 而不是消失——如果直接消失，你会以为它正常退出，错过 retry 时机
- **F3 的信号 UX**：图标消失作为"app 死了"的信号在重度用户身上工作得还行（视线会扫菜单栏），但成本是用户偶尔有"app 还活着吗"的疑虑
- **两种 failure 都不需要用户做复杂操作**——清磁盘空间 + 重新打开 .app，都是已有的系统级技能
- **app 不试图"自救"**：JSONL 损坏不假装数据完好；app 崩了不假装还活着。透明 > 智能。

### 引用到的 stories

F1, F3, E2（反向）

### 验证标准

- F1 的 "Unknown 状态 + 自动恢复" 必须工作——否则用户对 app 失去信任
- F3 的 "重启即可" 简单足够——但要在 README 和 install.md 里说清楚
- 如果未来加了 launchd 守护，要新开一份剧本（按下一个可用编号，如 S6）验证自动重启的 UX 是否真比手动重启好

---

## S4 — 首次 30 分钟：装到第一次"啊有用"

### 上下文

你是 author 的朋友，轻度 Claude Code 用户（平时同时跑 2 session 左右），从没用过这种菜单栏 utility。author 把 GitHub repo 推给你试。

### 时间线

**T+0:00** 你点开 GitHub README。看到 "menubar app that shows which Claude Code sessions are waiting for you"。
你心想"哦，监控用的"。

**T+0:01** README 的 Quick Start：
```
brew install --cask claude-code-monitor   # TBD, not yet on Homebrew
# or build from source:
git clone <repo>
cd claude-code-monitor
npm install
npm run tauri:build
```
brew 还没上，你 OK 从 source build。

**T+0:02 - T+0:14** 你执行：
- `git clone` (5s)
- `npm install` (3 min)
- `npm run tauri:build` (10 min，第一次 cargo build 下载 ~500MB crates)

期间你刷推文等。

**T+0:14** build 完成。`.app` 在 `src-tauri/target/release/bundle/macos/`。你双击。

```
"无法打开 ClaudeCodeMonitor.app，因为它来自身份不明的开发者。"
[OK]
```

你心想"什么鬼"。
（→ F4：Gatekeeper）

**T+0:14:30** 两个路径：
- **路径 A**：你看 install.md 的 Gatekeeper 小节，看到"右键 → Open → Open Anyway"或"系统设置 → 隐私与安全"，照做，成功打开
- **路径 B**：install.md 没说，你不知道，3 分钟内放弃，关闭浏览器 tab

**这一步是 onboarding 流失最高点**。
（→ F4 acceptance：install.md 必须写清楚）

假设你走了路径 A，1 分钟内打开成功。

**T+0:15** app 启动。菜单栏出现一个小图标。
你点击。
弹出窗口：

```
┌─────────────────────────────────┐
│  Claude Code Monitor   refresh  │
├─────────────────────────────────┤
│                                 │
│   no claude sessions running    │
│                                 │
└─────────────────────────────────┘
```

你心想"哦，我现在确实没开 session。等等，所以这就是它做的事？"
（→ E1：empty state；→ R2：零配置）

**T+0:16** 你打开终端，cd 到一个项目，运行 `claude`。
你给它一个任务："repackage this folder structure into separate `src/` and `tests/` directories"（一个 1-2 分钟的活）。
你切回菜单栏，点 app。

```
┌─────────────────────────────────────────────┐
│  Claude Code Monitor          refresh       │
├─────────────────────────────────────────────┤
│  my-project   working                       │
│  Reading folder structure...                │
└─────────────────────────────────────────────┘
```

你心想"哦，它真的看到了。"

**T+0:17** 你回终端等 claude 干活。
你专门盯着菜单栏看（因为这是你第一次试）。
约 90 秒后，菜单栏 tray 变成 `👁 1`。
你点。

```
┌──────────────────────────────────────────────────┐
│  Claude Code Monitor               refresh       │
├──────────────────────────────────────────────────┤
│  my-project    waiting                           │
│  Restructured into ./src and ./tests. Want me   │
│  to also update the imports in main.py?         │
└──────────────────────────────────────────────────┘
```

你 Cmd+Tab → 终端 → 回复 yes。
菜单栏：`👁 0`。

**这是"啊有用"瞬间。**
（→ H1, H2, H3 完整循环；→ L1：第一次价值体验）

**T+0:20** 你心想"OK 这玩意儿真的省心。但只 1 个 session 我也用不上多少。"
你又开了第二个 claude session 在另一个项目。
现在你脑子里有了模型：tray = 等你数；点 = 看具体；状态变化 = 自然涌现。

**T+0:25** 你已经不再"盯着" app 了，回到正常工作节奏。但你的注意力分配开始包含"周期性扫菜单栏"——尽管你自己还没意识到。

**T+0:25:30** 你跟 author 回消息："试用了下，挺好。"

（→ L1：第 1 天的转化触发达成）

**T+0:30** 30 分钟节点。回顾这 30 分钟：
- 你扫了菜单栏约 8-9 次（多数是因为你在试用，正常使用不会这么频繁）
- 你弹开 popup 约 3 次
- 你的工作流已经悄悄被改了——但你不知道

### 关键观察

- **"啊有用"瞬间是 T+0:17**——第一次完整 H1→H2→H3 循环。在这之前你都是"在试这个 app"，之后你是"在用这个 app"
- **onboarding 没有 UI 引导**——靠的是 empty state + 自然交互。这是 R2 的代价（零配置 = 零引导）
- **F4 Gatekeeper 是最大摩擦点**——install.md 写不好这里就是流失漏斗
- **build from source 用 13 分钟**——开源项目早期是不可避免的成本；上线 Homebrew cask 后能压到 < 2 分钟

### 引用到的 stories

E1, R2, F4, H1, H2, H3, L1

### 验证标准

- 如果新用户在 T+0:17 之前流失，说明 install / first-run 路径有问题（多半是 F4）
- 如果新用户用完 30 分钟不会再用，说明价值传递有问题（H1/H2/H3 没跑通 / 跑通了但不动人）
- "T+0:17 之前总耗时 < 20 分钟" 是 onboarding SLA——超过这个，转化率会陡降

---

## S5 — 第 14 天稳定使用日

### 上下文

你用这个 app 已经 14 天。今天和典型的任何一天都没区别。

### 时间线

**全天**
- 你扫菜单栏的频率：每 5-10 分钟自然扫一眼（已成肌肉记忆）
- 你弹开 popup 的次数：约 15-20 次/天
- 你停下来"看 app 在干嘛"的次数：0
- 你想到"这个 app"的次数：0（在它正常工作时）

**仅有一次特别**

17:23，你扫菜单栏看到 `👁 4`，心想"今天有点忙"，弹开看了下，分诊处理。

这次操作和第 1 天的差别：

| 项 | 第 1 天 | 第 14 天 |
|---|---|---|
| 看到 `👁 4` 到点开 popup 的时间 | 2-3 秒（决策） | < 0.5 秒（自动） |
| 看 popup 到确定先处理谁的时间 | 5-8 秒 | < 2 秒 |
| 切到目标 session 的时间 | 主动判断走哪条 Cmd+Tab | 视觉记忆直接命中 |
| 操作期间对 app 本身的关注 | "这 app 排序是？" | 0，完全 invisible |

### 关键观察

- **核心指标**：app 的"存在感" → 0，但价值持续输出
- 用户从来不"管理" app、不"配置" app、不"想着" app
- 第 14 天和第 1 天的功能完全一样——这正是稳定性的胜利
- 这一天没有 dramatic 时刻，但每一次"先扫菜单栏"都是 app 在工作

### 引用到的 stories

L2（核心）；H1, H2, H3（背景循环）

### 验证标准

- 如果用户在 14 天后开始抱怨"功能太少"，说明产品定位错了——应该坚持简单，不是加功能
- 如果用户停止使用，复盘原因：是停止写 Claude Code 了？还是 app 失去价值？前者无解，后者要 fix
- "用户 14 天后还在用" 是核心留存指标——比 DAU 更重要的是 D14 retention

---

## 引用其他文档

- 故事原子单元：[user-stories.md](user-stories.md)
- 产品定义：`product/definition.md`（待写）
- UML 设计图索引：[design/uml/00-index.md](../design/uml/00-index.md)

## 后续可能加的剧本（不强求）

- **S6 升级日**：v0.1 → v0.2 时用户的体验路径（验证 L2 的"不打扰"）
- **S7 团队推广日**：一个用户给同事演示这个 app（验证产品传达性）
- **S8 用了 90 天的回归日**：长尾用户的状态（验证 D90 留存）
