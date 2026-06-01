# Logging Spec

> **Status:** TBD (待写)
> **Cross-ref:** [F1 acceptance](../product/user-stories.md#f1--jsonl-损坏读不到), [BMAD S-012](../bmad/03-solutioning/epics/story-012-logging.md), [Architecture § 9](../bmad/03-solutioning/architecture.md)

---

## Why this exists

定义 log 文件的精确格式 / 路径 / 级别 / rotation 策略，作为 implementation 的契约 + 用户 debug 时的预期。

---

## Initial design (待 implementation 验证)

### Path

```
~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log
```

### Format

```
{ISO 8601 timestamp Z} [{LEVEL}] {message}
```

例：
```
2026-05-18T14:08:30Z [INFO] startup: tray icon registered, polling started
2026-05-18T14:08:32Z [WARN] session pid=12345: JSONL parse failed, marking Unknown
2026-05-18T14:08:34Z [ERROR] sysinfo refresh failed: <details>
```

### Levels

| Level | When |
|---|---|
| ERROR | JSONL parse 失败、sysinfo error、tray init 失败 |
| WARN | session 配对启发式 fallback、JSONL 文件不存在 |
| INFO | app 启动 / 退出、tray click |
| DEBUG | 每 refresh tick（默认关，`RUST_LOG=debug` 开） |

### Rotation

- **MVP**: 不 rotate（单文件无限增长）
- **v0.2+**: 按 size (10MB) rotate

---

## To fill when implementing S-012

- [ ] 实际 flexi_logger 的 format string
- [ ] 实测路径在 macOS 12 / 14 / 15 的权限差异
- [ ] 实测 log 文件 7d / 30d 大小（推算 rotation 策略）
- [ ] 决定是否需要 structured logging (JSON lines? key-value?) for future parsing

---

## Forward compatibility

- 改 log format = breaking change for log parser（如果有第三方写）
- MVP 假设无第三方 parser
- v1.0+ 若开放 telemetry-opt-in，可能切结构化格式

→ 写到 [ADR](../bmad/02-planning/decision-log.md) 当决定时。
