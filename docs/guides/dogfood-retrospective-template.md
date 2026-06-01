# Dogfood Retrospective Template

> **Purpose**：14 天 dogfood 期结束后填这个 template，作为决定是否进入 closed alpha 的依据。
> **Output**：每次 dogfood 周期产出一份 `dogfood-retro-YYYYMMDD.md`（按日期命名，放 `docs/roadmap/` 下）。

---

**填表说明**：每节都列了 prompt 问题，按顺序回答即可。诚实记录——dogfood 的价值在"你愿意承认 app 实际不够好"。

---

## 0. Meta

| 字段 | 值 |
|---|---|
| Dogfood 起止 | YYYY-MM-DD ~ YYYY-MM-DD（14 天）|
| App 版本 | v0.1.0-alpha.X (or commit sha) |
| 跑的 macOS 版本 | sw_vers ProductVersion |
| 跑在哪台机器 | M1 / M2 / Intel + 型号 |
| 期间作者状态 | 全职 dev / 兼职 / 其他项目并行 |

---

## 1. T0 Personal metric 实测

按 [success-metrics § 2](../product/success-metrics.md)：

| Metric | Target | 实测 | Pass? |
|---|---|---|---|
| 连续使用天数 | ≥ 14 | __/14 | ⬜ |
| 每天打开 popup 次数 | ≥ 5/day | 平均 __ | ⬜ |
| "靠它救我" 事件 | ≥ 3 | __ 次 | ⬜ |
| Crash 次数 | 0 | __ | ⬜ |
| 24h RSS 增长 | < 50MB | __ MB | ⬜ |
| 空闲 CPU avg | < 0.5% | __% | ⬜ |

**总体 pass / fail**：⬜ pass | ⬜ fail

---

## 2. "救我" 事件清单

具体记下 dogfood 期间 app 实际帮你的 case（按时间序）：

```
- 2026-MM-DD HH:MM: 在 X session 写代码时，瞄到菜单栏 Y session 已等 N 分钟。
  没看 app 的话可能要多等 M 分钟。
- ...
```

如果不到 3 次：app 可能跟你的 workflow 不匹配，或者你目前 session 密度不够。reflect: 是产品错还是 trial 条件错？

---

## 3. 没救我的事件 / app 失效场景

记下 app **应该 work 但没 work** 的 case：

```
- 2026-MM-DD: 我以为 X session 在 waiting，但 tray 显示 working。
  原因：??? (parser bug? / classification 边缘 case? / Claude Code 行为变化?)
- ...
```

每条对应开 GitHub issue 或修代码。

---

## 4. UI 不爽点

dogfood 14 天每次扫菜单栏 / 弹 popup，记录 friction：

| 问题 | 频率（高/中/低） | 严重度 | 已开 issue? |
|---|---|---|---|
| 例：popup 弹出位置在屏幕中央，太远 | 高 | 中 | #N |
| 例：列表项 hover 状态变色太显眼 | 中 | 低 | — |
| ... | | | |

→ 对照 [ux-design.md](../bmad/02-planning/ux-design.md) 看是不是 spec 错而不是 implementation 错。

---

## 5. 性能感受

| 项 | 实际体验 |
|---|---|
| popup 弹出速度 | 顺滑 / 偶尔卡 / 经常卡 |
| 切换 session 时列表更新 | 实时 / 延迟 / 不更新 |
| App 占用感知（电池/风扇）| 无感 / 偶尔察觉 / 烦 |
| 24h 后内存占用 | 数字 + 跟启动时对比 |

---

## 6. 产品哲学 verify

[constitution](../constitution.md) 5 个红线，14 天后回答：

| 红线 | 14 天后还坚持吗？ | 反思 |
|---|---|---|
| I.1 永不通知 | yes / no | 有没有"加通知就好了"的瞬间？ |
| I.2 零配置 | yes / no | 有没有"加个偏好就完美了"的想法？ |
| I.3 此刻不是历史 | yes / no | 有没有想看历史 session 的瞬间？ |
| I.4 跨终端中立 | yes / no | 有没有想要"跳到对应 tab"的瞬间？ |
| I.5 稳定胜过功能光鲜 | yes / no | 有没有改 UI 的冲动？ |

→ 哪条想改：写新 ADR 推翻原则（[constitution V.2 changing process](../constitution.md)）。
→ 全部坚持：说明哲学 valid，继续。

---

## 7. 漏掉的需求

dogfood 14 天有没有"如果 app 还能做 X 就好了" 的瞬间？记录：

```
- "如果能 X，可以解决 Y"
  - 是否属于 R1-R5 红线？是 → 写到 retro 备查不实现
  - 不属于 → 加入 [v0.2.md candidates](../roadmap/v0.2.md)
```

---

## 8. 文档准确性

dogfood 时有没有发现文档跟实际 app 行为不一致？

```
- docs/X.md § Y 说 "...", 但实际 app 做了 "..."
  - fix in same PR
```

---

## 9. 决定

基于上述 1-8 节，回答：

### 9.1 是否进入 closed alpha？

⬜ **是**——T0 metric 全 pass，UI 不爽点都开了 issue，准备好让 5-10 个朋友测

⬜ **否**——下面任一项 true：
- T0 metric 不达
- 发现 P0 bug 未修
- 文档跟实际偏差太大
- 产品哲学某条 14 天后想改

### 9.2 如果否，下一步

具体行动：

- [ ] 修 issue #X, #Y, #Z
- [ ] 文档 update
- [ ] 14 天再 dogfood 一轮
- [ ] 或者：重大反思（写新 ADR、重新 review brief）

### 9.3 如果是，下一步

按 [v0.1.md § 3.2 closed alpha entry](../roadmap/v0.1.md)：

- [ ] 找 5-10 个 Heavy CC user 朋友
- [ ] 准备 DMG + install.md 推给他们
- [ ] 安排每人 30min 1-on-1 follow-up

---

## 10. 14 天最大学习

不超过 3 句话总结：

```
1. ...
2. ...
3. ...
```

---

## 11. 给未来 dogfood 的建议

下次再跑 dogfood (v0.2 / v1.0 等) 时，本次经验告诉自己什么？

```
- 提前准备 X
- 不要 Y
- 重点关注 Z
```

---

## 文件位置

填好后放 `docs/roadmap/dogfood-retro-YYYYMMDD.md`，commit + push 到 git，作为 history。
