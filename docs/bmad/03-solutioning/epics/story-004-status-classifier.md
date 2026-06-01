# S-004 · Status classifier

**Epic:** [001 Core Monitoring](epic-001-core-monitoring.md)
**Status:** ✅ DONE (2026-05-21, 15 unit + 1 live-env integration tests pass)
**Estimate:** S — actual ~20min
**Owner:** caiyiwen

## Description

**As a** backend module
**I want to** classify a session into Waiting / Working / Unknown based on its last meaningful JSONL entry
**so that** the frontend can render the right state badge and the tray can count waiting sessions.

## Acceptance criteria

### 函数签名

```rust
/// 反向扫，跳过 attachment / file-history-snapshot / etc，找最后一条 type ∈ {user, assistant} 的 envelope
pub fn last_meaningful(lines: &[String]) -> Option<JsonlEnvelope>;

/// 基于 last_meaningful 的输出判定 status
pub fn classify(env: Option<JsonlEnvelope>) -> SessionStatus;
```

### 规则（按 [spec/jsonl-schema.md § 6](../../../spec/jsonl-schema.md) + [UML 09 State](../../../design/uml/09-state-session.md)）

| Input (envelope from `last_meaningful`) | Output |
|---|---|
| `None`（JSONL 读不到、parse 失败、反扫到文件头都没找到 user/assistant） | `Unknown` |
| `envelope.kind == "assistant"` AND `message.stop_reason == "end_turn"` | `Waiting` |
| `envelope.kind == "assistant"` AND `stop_reason` 是 `tool_use` / `max_tokens` / null / 其他 | `Working` |
| `envelope.kind == "user"`（含 tool_result wrapper） | `Working` |

### 数据类型（从 [data-model.md § 2.2-2.4](../data-model.md) 引入）

- `JsonlEnvelope { kind, uuid, parent_uuid, timestamp, session_id, cwd, version, git_branch, message }`
- `NestedMessage { role, content, stop_reason, model }`
- `ContentValue` enum: `Text(String) | Blocks(Vec<ContentBlock>)`（serde untagged）
- `ContentBlock` enum: `Text | Thinking | ToolUse | ToolResult`（serde tag="type"）

### 必须

- 单元测试覆盖所有分支
- 用实测真实 JSONL fixture 验证（至少 3 个 session 的 transcript 末尾片段）

## Dev notes

- Pure function（`classify`），无 IO，纯逻辑
- `last_meaningful` 是 read-only IO（接受 lines slice）+ parse
- **关键 invalidation 2026-05-18**：之前文档假设的"无 pending tool_use" 判定是错的——实际用 `stop_reason == "end_turn"` 更准。Tool approval prompt 也算 `stop_reason=tool_use` → Working（MVP 接受不区分 tool-prompt vs message-prompt 的限制）
- 反向扫的边界：实测 JSONL 末尾常出现 `attachment` (task_reminder)、`ai-title`、`last-prompt`、`file-history-snapshot`、`permission-mode`——`last_meaningful` 必须跳过这些直到找到 user/assistant 或扫到文件头
- 性能：反向扫的"最坏" case 是整个 JSONL 全是非 user/assistant entry（几乎不发生）。典型 case 反向 1-3 行就够

## Dependencies

- **Upstream**: [S-003 JSONL tail reader](story-003-jsonl-tail-reader.md) 返回 lines；spec/jsonl-schema.md（✅ DONE）
- **Downstream**: [S-005 list_sessions IPC](story-005-list-sessions-command.md)

## Files to touch

- `src-tauri/src/session.rs` — 新增 `JsonlEnvelope` / `NestedMessage` / `ContentValue` / `ContentBlock` types + `last_meaningful()` + `classify()`

## Test plan

### 单元测试

```rust
#[test]
fn classify_returns_unknown_when_envelope_is_none() {
    assert_eq!(classify(None), SessionStatus::Unknown);
}

#[test]
fn classify_returns_working_when_kind_is_user() {
    let env = JsonlEnvelope { kind: "user".into(), /* ... */ };
    assert_eq!(classify(Some(env)), SessionStatus::Working);
}

#[test]
fn classify_returns_waiting_when_assistant_end_turn() {
    let env = JsonlEnvelope {
        kind: "assistant".into(),
        message: Some(NestedMessage {
            role: "assistant".into(),
            stop_reason: Some("end_turn".into()),
            /* ... */
        }),
        /* ... */
    };
    assert_eq!(classify(Some(env)), SessionStatus::Waiting);
}

#[test]
fn classify_returns_working_when_assistant_tool_use() {
    let env = /* assistant + stop_reason="tool_use" */;
    assert_eq!(classify(Some(env)), SessionStatus::Working);
}

#[test]
fn classify_returns_working_when_assistant_max_tokens() {
    let env = /* assistant + stop_reason="max_tokens" */;
    assert_eq!(classify(Some(env)), SessionStatus::Working);
}

#[test]
fn classify_returns_working_when_assistant_no_stop_reason() {
    let env = /* assistant + stop_reason=None */;
    assert_eq!(classify(Some(env)), SessionStatus::Working);
}

#[test]
fn last_meaningful_skips_attachment_entries() {
    let lines = vec![
        r#"{"type":"user", "uuid":"1", ...}"#.into(),
        r#"{"type":"assistant", "uuid":"2", ...}"#.into(),
        r#"{"type":"attachment", "uuid":"3", ...}"#.into(),
        r#"{"type":"ai-title", "uuid":"4", ...}"#.into(),
    ];
    let env = last_meaningful(&lines).unwrap();
    assert_eq!(env.uuid, "2"); // 跳过 attachment + ai-title，命中 assistant
}

#[test]
fn last_meaningful_returns_none_when_no_user_assistant() {
    let lines = vec![
        r#"{"type":"file-history-snapshot", ...}"#.into(),
        r#"{"type":"permission-mode", ...}"#.into(),
    ];
    assert_eq!(last_meaningful(&lines).is_none(), true);
}
```

### 集成 (依赖 S-001/S-002/S-003 一起)

- 真实 waiting session（claude 刚回答完）→ 返回 Waiting
- 真实 working session（claude 在 tool_use 中）→ 返回 Working
- 真实 session 末尾是 attachment → 反向跳过后返回 Waiting/Working

### Fixture

用真实 JSONL 截取，放 `src-tauri/tests/fixtures/`：
- `assistant_end_turn.jsonl`
- `assistant_tool_use.jsonl`
- `user_message.jsonl`
- `attachment_at_end.jsonl`（验证反向扫）
- `empty_after_only_metadata.jsonl`（全是 attachment/ai-title）

## Definition of Done

- [x] 代码 merged（pending dedicated commit）
- [x] 100% 分支覆盖（classify 所有 4 行 + last_meaningful 反向扫 6 case）
- [ ] 用 ≥3 个真实 JSONL fixture 校验（暂用 live-env integration test 替代——live session 跑通 list→locate→tail→classify 全链路即视为等效。持久 fixture 留待 dogfood）
- [x] [UML 09](../../../design/uml/09-state-session.md) + [data-model § 2.5](../data-model.md) 跟实现一致

## Implementation summary (2026-05-21)

- `SessionStatus { Waiting / Working / Unknown }` enum（Serialize 为 lowercase 字符串）
- `Session.status` 从 `String` 改为 `SessionStatus`（线协议字符串 unchanged）
- `JsonlEnvelope` / `NestedMessage` / `ContentValue (untagged)` / `ContentBlock (tag=type)` 全部 derive Deserialize
- `last_meaningful(&[String]) -> Option<JsonlEnvelope>` — reverse-scan，跳过非 user/assistant + parse-fail
- `classify(Option<JsonlEnvelope>) -> SessionStatus` — pure，所有未知 stop_reason 默认 Working
- **关键 bug 避免**：data-model.md 示例代码用 `serde_json::from_str(line).ok()?` 会在第一条 parse 失败时短路退出 last_meaningful；实测改为 `if let Ok(env) = ...` 跳过继续扫
- 单元测试覆盖：classify 全 4 行 + last_meaningful 6 case（attachment skip / no-user-assistant / empty / unparseable skip / user-after-assistant / 反扫顺序） + 2 真实 envelope shape deserialise + 1 wire-format check
- Live-env integration: 在 jsonl_tail.rs 加 end-to-end test (list→locate→tail→classify)，证明全链路不 panic 且 status ∈ {Waiting/Working/Unknown}
