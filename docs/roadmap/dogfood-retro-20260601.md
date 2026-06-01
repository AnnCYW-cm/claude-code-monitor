# Dogfood Retro · 2026-06-01 (1-2 day smoke round)

> 这是 Epic 1 (S-001..S-005) 完结后的**短周期** dogfood，目的是先验证管道在真实 workflow 下不崩、数据对、性能可以接受。
> 不是 14 天 closed-alpha gate retro——完整 retro 走 [dogfood-retrospective-template.md](../guides/dogfood-retrospective-template.md)。
> 1-2 天能可靠捕捉的：准确性 bug、明显性能问题、边缘 case、UI 鲜痛点。
> 1-2 天**不能**可靠捕捉的：T0 success metric、电池影响、workflow 真集成、长尾稳定性——这些留给 Epic 2 后的 14 天 retro。

---

## 0. Meta

| 字段 | 值 |
|---|---|
| Dogfood 起止 | 2026-06-01 起（第一轮 smoke 当天结束） |
| App 版本 | start: `bc4eb15`（Epic 1 完结 + CI 绿） → end of day: `11e4d94`（含 2 个 P0 fix） |
| 运行方式 | `npm run tauri:dev`（开发版本，未 release） |
| 机器 | MacBook Pro 16 / M-series Apple Silicon / macOS Tahoe 26.3.1 |
| 期间作者状态 | 当时 3 个真实 claude session 并行（home cwd 都从 `/Users/caiyiwen` 启动） |

---

## 1. 启动顺利度

| 步骤 | OK? | 备注 / 报错 |
|---|---|---|
| `npm install` 成功 | ✅ (第 2 次) | 第 1 次撞 esbuild errno -88 (ENOEXEC)；`rm -rf node_modules package-lock.json && npm install` 修好。node arch=arm64 ✓ |
| `npm run tauri:dev` 第一次起来 | ❌→✅ | 第 1 次撞 `feature edition2024 is required` (cargo 1.83.0)。Homebrew Rust 跟 rustup 双装冲突——`brew uninstall rust` 后 cargo 切到 rustup stable，过 |
| Tray icon 出现在菜单栏 | ✅ | template 渲染，菜单栏右上角 |
| 点 tray 弹出窗口 | ✅ | popup 正常弹出 |
| 窗口里能看到 session list | ✅（修完两个 P0 后） | 首次启动显示 "no claude sessions running" ← 实际有 4 个；详见 § 6 finding #1 + #2 |
| 终端 `console.log("[list_sessions]", sessions)` 有输出 | ⬜ | 当时没开 DevTools 验，后补 |

---

## 2. Tray title 准确性（最关键观察项）

每次扫一眼 tray，对照你的真实感知 vs app 显示。记 mismatch：

```
- HH:MM: 我实际有 N 个 waiting session，tray 显示 M。差异原因 ???
  - 真 waiting 但 app 没认出？(stop_reason 不是 end_turn? jsonl 没新写?)
  - app 认成 waiting 但其实在做事？(我刚回完没看见 working 转换?)
- ...
```

**目标**：mismatch == 0 才算 classify 算法对。任一条都需要回查 JSONL 实际内容。

---

## 3. Polling 节奏感受

`main.ts` 现在固定 2s poll。1-2 天后回答：

- 2s 间隔感觉 □ 太快（电池/CPU 焦虑） □ 刚好 □ 太慢（错过快速转换）
- 切完一轮（你按完 enter → tray 数字变化）感觉延迟 ___ 秒
- 如果改可配置（违反 zero-config 红线），你会改成 ___ 秒

---

## 4. 性能感受

| 项 | 实际体验 |
|---|---|
| App 启动到 tray 出现 | __ 秒 |
| 弹 popup 时延 | 顺滑 / 偶尔卡 / 经常卡 |
| 持续运行后 macOS 系统监控里这 app 的 CPU | __% |
| 持续运行后内存 | __ MB |
| Battery impact 感知 | 无感 / 偶尔察觉 / 烦 |

> 比对参考: list_processes 实测 23ms (S-001 集成测试), list() 8ms 零 session (S-005)。理论上单次 invoke <50ms，每 2s 一次，CPU 应该 << 1%。

---

## 5. Crash / panic / 不响应

第一轮 smoke 期间：0 crash, 0 panic, 0 hang ✅

但**逻辑层 2 个 P0 bug**（不是 crash，是"显示数据完全错"，比 panic 还隐蔽）——详见 § 6。

---

## 6. 让我"啊原来这种 case 没考虑"的边缘场景

第一轮 dogfood 在 30 分钟内撞出 **2 个 P0 production bug** + **1 个测试套盲区**。这是开源 release 前抓到的最值钱的反馈。

---

### Finding #1 · sysinfo 在 macOS 不能识别 Node-based CLI + 拿不到别人 cwd

**症状**: tray title 空、popup 显示 "no claude sessions running"，但 user 实际有 3+ 个 Claude Code session 在跑。

**根因**:
1. `sysinfo::Process::name()` 在 macOS 上返回 underlying exe basename（`"node"`），不读 `process.title` 设置的 comm（`"claude.exe"`）。我们的 `is_claude_name()` 只认 "claude"/"claude-code"，看到 "node" 全 reject。
2. `sysinfo::Process::cwd()` 在 macOS 非 root 下对其他 user 进程返回 `None`（哪怕同 user）。`extract_raw()` 看到 None 直接跳过。

**为啥 84 个测试全没抓到**:
- 单元测试 stub `is_claude_name("claude")` — 手写字符串，永远绿
- 集成测试 `list_processes_returns_only_running_claude_or_empty` 用 **"or empty"** 容错路径 — 0 个 process 也 pass
- live-env 集成测试 `or_skips` 路径 — 0 个 skip 不报错
- 实测验证: 0 个 claude 时 84 全绿；3 个 claude 时仍然 84 全绿。完全测不出"应该找到但没找到"

**修复** (commit `32134e9`):
- 砍 sysinfo dep，换 libproc（Activity Monitor 同款 API）做 enumerate + name + uid + start_time
- cwd 没有现成 wrapper（libproc 0.14.11 `pidcwd()` 显式返回 "not implemented for macos"），自写 raw FFI 调 `proc_pidinfo(PROC_PIDVNODEPATHINFO)`。手算 struct layout（vinfo_stat 136 / vnode_info 152 / proc_vnodepathinfo 2352 bytes）
- 加新进程名 `claude.exe` 到白名单
- 新加 fail-loud 测试 `list_finds_claude_when_ps_does` — shell out 到 `ps -axo uid,comm` 取 ground truth，**ps 看到 claude 但 list_processes 返 0 = test panic**。这是消灭测试假阳性的核心动作

**实测验证**: ps 看到 4 个 claude，list_processes() 找到 3 个（race condition 漏 1 个可接受）；2.26ms 耗时（25ms NFR budget 1/10）

→ **属性**: impl 问题（架构层面 — sysinfo 在 macOS 不适合做 process introspection）+ 测试盲区（or-skip 容错 = 假阳性）

---

### Finding #2 · 60s ACTIVE_JSONL_WINDOW 太狠，正常 idle session 被误杀

**症状**（fix #1 之后）: 3 个 session 都显示 "unknown" 状态，但其中至少 1 个明确在 waiting（user 心里知道）。

**根因**:
- S-002 的 `ACTIVE_JSONL_WINDOW = 60s` — jsonl mtime > 60s 前就判 `NoActiveJsonl` → status = Unknown
- 实测 user 的 jsonl mtime 是 6 分钟前（刚跟 claude 聊完一段，回 popup 这边查看）
- 60s 窗口的设计原意是"防 same-cwd 多 process 误配 jsonl"，但真实工作流 idle > 60s 完全正常（claude 思考、user 走开、session 切换）
- 正在 Waiting 的 session 是 tray 应该高亮的，悄悄降级 Unknown = 产品功能失效

**修复** (commit `11e4d94`):
- 窗口从 60s 提到 7 天 (`7 * 24 * 60 * 60`)
- 保留 sanity check 防止真死 jsonl（abandoned test runs），但接受所有 realistic idle
- 单元测试 `locate_returns_no_active_jsonl_when_all_stale` 同步从 120s 改成 8 天

**实测验证**: 3 个 session 全显示真状态（"看到 working 和 waiting 了"）

→ **属性**: spec 问题 — S-002 acceptance 中"60s 启发式"是工程师拍脑袋的常量，没基于真实用户行为校准。开 issue 留底以防未来 v0.x 撞同款。

---

### 测试套盲区（贯穿 finding #1 / #2）

**教训**：对外行为有信任的测试只有两种：
1. 跟外部 ground truth 比对（ps / 真 JSONL / dbus / etc）— 强信号
2. 在 process 边界做 end-to-end 验证 — 中信号

**不能信**的测试模式：
- "or empty" / "or skip" 容错 — 0 case 也 pass，等同没测
- 手写 fake input 单测 — 测了字符串匹配，没测真实数据流

这次新加的 `list_finds_claude_when_ps_does` 是模板：**ground-truth-anchored fail-loud 测试**。未来 S-002 ~ S-005 也该补一个类似的 — "claude session 在跑且 jsonl 有内容时，list_sessions 必须返回非空"。

---

## 7. 1-2 天后的判断

✅ **管道扎实，可以推 Epic 2 (UI)** — 修完 finding #1 + #2 后，3 个真 session 正确显示 working/waiting；tray title 准；不崩

具体下一步：
- [x] commit `32134e9` libproc 重写 process enum
- [x] commit `11e4d94` 放宽 active window 7 天
- [ ] **Epic 2 真 UI**（S-006~S-010）— popup 当前是 raw list，需 status icon / waiting duration / last_message preview 排版
- [ ] **补 ground-truth 测试** 给 S-002 ~ S-005，复用本 retro 里的 `list_finds_claude_when_ps_does` 模板
- [ ] 14 天周期 dogfood（用 [dogfood-retrospective-template.md](../guides/dogfood-retrospective-template.md)）— 评估 T0 success metric / 电池 / workflow 真集成

---

## 8. 1-2 天最大 1 个学习

```
84 个测试全绿 + Epic 1 backend 在生产环境完全没工作，因为整个测试套是"or-empty / or-skip 容错"，
没一个跟 ground truth 比对——dogfood 30 分钟内捅破了"测试覆盖度"这层纸糊的安全感。
```

→ 推论：未来每个 story 的 Definition of Done 必须包含**至少 1 个 ground-truth-anchored 测试**（外部 ps / 真 JSONL / 真 IPC），不能只靠 stub。
