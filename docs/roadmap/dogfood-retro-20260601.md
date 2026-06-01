# Dogfood Retro · 2026-06-01 (1-2 day smoke round)

> 这是 Epic 1 (S-001..S-005) 完结后的**短周期** dogfood，目的是先验证管道在真实 workflow 下不崩、数据对、性能可以接受。
> 不是 14 天 closed-alpha gate retro——完整 retro 走 [dogfood-retrospective-template.md](../guides/dogfood-retrospective-template.md)。
> 1-2 天能可靠捕捉的：准确性 bug、明显性能问题、边缘 case、UI 鲜痛点。
> 1-2 天**不能**可靠捕捉的：T0 success metric、电池影响、workflow 真集成、长尾稳定性——这些留给 Epic 2 后的 14 天 retro。

---

## 0. Meta

| 字段 | 值 |
|---|---|
| Dogfood 起止 | 2026-06-01 ~ 2026-06-__ |
| App 版本 | commit `bc4eb15`（Epic 1 完结 + CI 绿） |
| 运行方式 | `npm run tauri:dev`（开发版本，未 release） |
| 机器 | M1/M2 + macOS Tahoe 26.3.1 |
| 期间作者状态 | __ |

---

## 1. 启动顺利度

| 步骤 | OK? | 备注 / 报错 |
|---|---|---|
| `npm install` 成功 | ⬜ | |
| `npm run tauri:dev` 第一次起来 | ⬜ | 首跑 cargo 编译 ~5min 正常 |
| Tray icon 出现在菜单栏 | ⬜ | template-rendered，深浅模式都该看见 |
| 点 tray 弹出窗口 | ⬜ | |
| 窗口里能看到 session list | ⬜ | UI 是 Epic 2 前的占位，丑但有数据 |
| 终端 `console.log("[list_sessions]", sessions)` 有输出 | ⬜ | DevTools console 打开 |

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

记录所有不正常事件：

```
- HH:MM: <现象> <复现条件> <终端报错>
  - 后续: 开 issue # / 修 in-line / 加 retest
- ...
```

> 期待 = 0 次。Epic 1 的 catch_unwind 应该兜住单 session 异常但全 app 不挂。

---

## 6. 让我"啊原来这种 case 没考虑"的边缘场景

1-2 天大概率会撞上至少 1-2 个测试套没覆盖的 case。例如：
- 同一 cwd 同时开 3 个 claude session（rare 但你可能干过）
- claude 内 `cd` 到子目录又继续聊
- 同时有 claude 进程是 `sudo claude` 跑的
- VPN / DNS 中断时 claude 卡死状态
- macOS 睡眠唤醒后 sysinfo 行为
- ...你撞上的

```
- 边缘场景：___
  - app 表现：___
  - 预期表现：___
  - 是 spec 问题 / impl 问题 / 文档缺失？
```

---

## 7. 1-2 天后的判断

⬜ **管道扎实，可以推 Epic 2 (UI)** — tray title 准、polling 够用、不崩
⬜ **数据/性能有 P0 bug** — 先开 issue 修，再 dogfood 一轮
⬜ **管道 OK 但产品定位需复核** — 先回看 constitution / brief

具体下一步：
- [ ] ___

---

## 8. 1-2 天最大 1 个学习

```
___
```

（不超过 1 句话。这周期不抓多教训，抓最戳的那个。）
