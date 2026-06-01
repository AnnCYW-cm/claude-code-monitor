# JSONL Schema — Claude Code Transcript

> **Status:** ✅ DONE (实测 2026-05-18 against Claude Code 2.1.126 on macOS 26.3.1)
> **Source:** 真实 cat 用户机器 `~/.claude/projects/-Users-caiyiwen/*.jsonl` 多个文件
> **Cross-ref:** [BMAD S-002 JSONL locator](../bmad/03-solutioning/epics/story-002-jsonl-locator.md), [BMAD S-004 classifier](../bmad/03-solutioning/epics/story-004-status-classifier.md), [tasks T002/T004/T008/T009](../bmad/03-solutioning/tasks.md), [data-model.md](../bmad/03-solutioning/data-model.md)
>
> **重大 invalidation**：我们之前在 [data-model.md](../bmad/03-solutioning/data-model.md) / [UML 09](../design/uml/09-state-session.md) / [classifier S-004](../bmad/03-solutioning/epics/story-004-status-classifier.md) 假设的 `JsonlMessage { role, content, timestamp }` 是**错的**——实际 schema 比这复杂。本文档是 source of truth，下游需要更新。

---

## 1. 文件路径规则

### 1.1 Path encoding

- Root: `~/.claude/projects/`
- Per session: `<root>/<encoded-cwd>/<session-uuid>.jsonl`
- Encoding: cwd 的 `/` 替换为 `-`，含前导 `/`
- 例：
  - cwd `/Users/caiyiwen` → dir `~/.claude/projects/-Users-caiyiwen/`
  - cwd `/Users/caiyiwen/my-project` → dir `~/.claude/projects/-Users-caiyiwen-my-project/`

### 1.2 同 cwd 多 session 的目录结构

实测 `-Users-caiyiwen/` 下有：
- `<uuid>.jsonl` 文件（transcript 主文件）
- `<uuid>/` 子目录（缓存/metadata，monitor 不读）

```
~/.claude/projects/-Users-caiyiwen/
├── 0ad8807a-2636-435e-9b50-a778231bbca7/        ← 子目录 (skip)
├── 12c38bd2-2ac0-4506-aadb-55ffd844079e/
├── 179b8e5d-0235-4215-89a4-7aac8126ccbf/
├── 179b8e5d-0235-4215-89a4-7aac8126ccbf.jsonl   ← transcript file
├── 2123db4f-abf1-4afa-81ed-4b0c8f821d44/
├── 2123db4f-abf1-4afa-81ed-4b0c8f821d44.jsonl
└── ...
```

每个 active session 对应一个 `<uuid>.jsonl` 文件，**uuid 跟 `sessionId` 字段一致**。

### 1.3 ⚠️ 重要：session cwd 可在 transcript 内变化

实测同一 JSONL 文件内，不同 entry 的 `cwd` 字段可能不同——用户用 `cd` 切换目录后 Claude Code 继续记到同一 transcript：

```json
// 第一条 (开始)
{"type":"user", "cwd":"/Users/caiyiwen", ...}
// 后面某条
{"type":"attachment", "cwd":"/Users/caiyiwen/ai-product-column-research/drafts", ...}
```

但**文件路径**仍由 session 启动时的 cwd 决定（`-Users-caiyiwen/`）——不会因为内部 cd 而 mv。

**对 monitor 的影响**：
- 进程当前 cwd（`sysinfo::Process::cwd()`）可能跟 JSONL 文件归属 cwd **不同步**
- 我们的 [S-002 JSONL locator](../bmad/03-solutioning/epics/story-002-jsonl-locator.md) 用进程 cwd 推 JSONL 路径——可能找不到（如果用户 cd 后启动新 transcript）
- 缓解：进程的 cwd 不变（cwd 是进程属性，cd 改变的是用户 shell 子进程的 cwd 不影响 claude 父进程）。需要 verify。

---

## 2. JSONL 文件 format

### 2.1 文件级

- Encoding: UTF-8
- Line separator: `\n`
- 每行一个 JSON 对象（一条 entry）
- Append-only（claude 进程持续 append 不 rewrite）
- 文件大小可达数 MB（实测 7.5MB / 1794 行 = 长会话）

### 2.2 顶层 envelope

每条 entry 都是带"信封"的对象。**顶层 `type` 决定它是什么**——这是 monitor 分类的关键。

```json
{
  "type": "user" | "assistant" | "file-history-snapshot" | "permission-mode" | "attachment" | "ai-title" | "last-prompt",
  "uuid": "...",
  "parentUuid": "..." | null,
  "timestamp": "2026-05-04T05:42:34.919Z",   // ISO 8601 with millis + Z
  "sessionId": "...",                          // 跟文件名 uuid 一致
  "cwd": "/Users/...",                         // ⚠️ 可在 transcript 内变化
  "version": "2.1.126",                        // Claude Code 版本
  "gitBranch": "master",
  "userType": "external",
  "entrypoint": "cli",
  "isSidechain": false,
  "message": { ... },                          // 仅在 type=user/assistant
  "attachment": { ... },                       // 仅在 type=attachment
  "snapshot": { ... },                         // 仅在 type=file-history-snapshot
  // ...其他 optional fields
}
```

### 2.3 顶层 type 字段值（实测枚举）

| `type` 值 | 含义 | monitor 是否关心 |
|---|---|---|
| `user` | 用户输入（含 tool_result） | ✅ 关心（决定 working state） |
| `assistant` | Claude 回复（含 text / thinking / tool_use） | ✅ 关心（决定 waiting state） |
| `file-history-snapshot` | Claude Code 内部跟踪 user file changes | ❌ 跳过 |
| `permission-mode` | session 起始的权限模式 | ❌ 跳过 |
| `attachment` | 附件（如 task_reminder / image） | ❌ 跳过 |
| `ai-title` | 自动生成的对话标题 | ❌ 跳过 |
| `last-prompt` | 最后 prompt 记录（resume 用） | ❌ 跳过 |

→ **classify 逻辑必须先 filter top-level type ∈ {user, assistant}**，再看 nested message.role。

---

## 3. Message structure (type=user)

### 3.1 User 输入（人类打字）

```json
{
  "type": "user",
  "uuid": "fb708732-8f6d-4ba0-915e-15ae7a61e5ec",
  "parentUuid": null,
  "timestamp": "2026-05-04T05:42:34.919Z",
  "promptId": "f5b892fe-6168-4535-b59a-2873a71fdb0f",
  "message": {
    "role": "user",
    "content": "纯字符串：用户输入的文本"   // ⚠️ string OR array
  },
  // ... 顶层 envelope 其他字段
}
```

### 3.2 User 包装的 tool_result

Claude Code 把 tool_result 包装成 user message：

```json
{
  "type": "user",
  "promptId": "f5b892fe-6168-4535-b59a-2873a71fdb0f",
  "message": {
    "role": "user",
    "content": [                              // ⚠️ array，跟 3.1 不同
      {
        "type": "tool_result",
        "tool_use_id": "toolu_0176rGGDuQDFaz2dRdaPT291",
        "content": "Launching skill: 调研规范"
      }
    ]
  },
  "toolUseResult": {                          // 顶层补一份冗余 metadata
    "success": true,
    "commandName": "调研规范"
  },
  "sourceToolAssistantUUID": "dc444397-..."
}
```

→ **`message.content` 是 `string` OR `array`**（discriminated by 数组首元素 `type`）

---

## 4. Message structure (type=assistant)

```json
{
  "type": "assistant",
  "uuid": "dc444397-725c-41ed-a902-446b4857d5be",
  "parentUuid": "...",
  "timestamp": "2026-05-04T05:42:40.374Z",
  "requestId": "req_011CagzfzzQBxhM9dboVYNGK",
  "message": {
    "model": "claude-opus-4-7",
    "id": "msg_01HprtD7iBPCiHQ5QKhcZmR6",
    "type": "message",                        // nested 也有 type
    "role": "assistant",
    "content": [                              // 始终 array
      // ContentBlock[]
    ],
    "stop_reason": "tool_use" | "end_turn" | ...,
    "stop_sequence": null,
    "usage": {
      "input_tokens": 6,
      "output_tokens": 279,
      "cache_creation_input_tokens": 13134,
      "cache_read_input_tokens": 14816,
      // ... 详细 usage stats
    }
  },
  // ... envelope 其他字段
}
```

### 4.1 stop_reason 值

实测见到：
- `tool_use` — claude 调用工具后停顿（等 tool_result）
- `end_turn` — claude 完成一轮回答，等用户输入（**waiting 信号**）
- 其他（`max_tokens` / `stop_sequence` / `pause_turn` 等理论可能）

→ **`stop_reason == "end_turn"` 是 waiting 的强信号**（比"无 pending tool_use" 更精确）

---

## 5. ContentBlock types

### 5.1 `text`
```json
{ "type": "text", "text": "..." }
```

### 5.2 `thinking` (Claude 的内部思考)
```json
{ "type": "thinking", "thinking": "...", "signature": "long base64..." }
```

### 5.3 `tool_use`
```json
{
  "type": "tool_use",
  "id": "toolu_0176rGGDuQDFaz2dRdaPT291",
  "name": "Skill",                    // tool 名（Bash / Read / Edit / etc）
  "input": { "skill": "调研规范" },    // tool 参数（object）
  "caller": { "type": "direct" }      // optional
}
```

### 5.4 `tool_result`（仅在 user message 里）
```json
{
  "type": "tool_result",
  "tool_use_id": "toolu_...",
  "content": "string OR array of text blocks"
}
```

---

## 6. Classification logic (用于 monitor)

### 6.1 找"最后一条有意义的 message"

**伪代码**：

```rust
fn last_meaningful_message(jsonl_lines: &[String]) -> Option<MessageEnvelope> {
    // 反向扫描，找第一个 type ∈ {user, assistant} 的 entry
    for line in jsonl_lines.iter().rev() {
        let env: MessageEnvelope = serde_json::from_str(line).ok()?;
        if env.r#type == "user" || env.r#type == "assistant" {
            return Some(env);
        }
        // 跳过 attachment / file-history-snapshot / permission-mode / ai-title / last-prompt
    }
    None
}
```

### 6.2 决定 status

```rust
fn classify(envelope: Option<MessageEnvelope>) -> SessionStatus {
    let env = match envelope {
        None => return SessionStatus::Unknown,    // JSONL 空 / 全是非 user/assistant entries
        Some(e) => e,
    };

    // 必须是 assistant 才考虑 waiting
    if env.r#type != "assistant" {
        return SessionStatus::Working;             // user 或 tool_result 中 = claude 在处理
    }

    // assistant + end_turn → waiting (清晰信号)
    if env.message.stop_reason.as_deref() == Some("end_turn") {
        return SessionStatus::Waiting;
    }

    // assistant + tool_use → 在等工具执行 (working)
    // 或其他 stop_reason → working
    SessionStatus::Working
}
```

→ **比文档之前假设的"无 pending tool_use" 判定更准**（用 `stop_reason` 是 official 信号）

### 6.3 关键 invalidation 对比

| 之前文档假设 | 实际正确 |
|---|---|
| `last_msg.role == "assistant"` | 应该是 `envelope.type == "assistant"` |
| "no pending tool_use" 判 waiting | 用 `message.stop_reason == "end_turn"` 更准 |
| 最后一行就是判断对象 | 必须先 filter type ∈ {user, assistant} |
| JsonlMessage struct 顶层有 role/content | role/content 在 nested `message` |
| timestamp 是 Unix u64 | timestamp 是 ISO 8601 string |

---

## 7. Suggested Rust types

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonlEnvelope {
    #[serde(rename = "type")]
    pub kind: String,                    // "user" / "assistant" / "attachment" / etc
    pub uuid: String,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    pub timestamp: String,               // ISO 8601, parse 到 chrono::DateTime<Utc>
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub cwd: Option<String>,             // 可能在某些 entry 缺失
    pub version: Option<String>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    pub message: Option<NestedMessage>,  // 仅 user/assistant 有
    // 其他 envelope fields 用 #[serde(flatten)] 收 Value
}

#[derive(Deserialize, Debug)]
pub struct NestedMessage {
    pub role: String,                    // "user" / "assistant"
    #[serde(default)]
    pub content: ContentValue,           // string OR array (custom deserializer)
    #[serde(rename = "stop_reason")]
    pub stop_reason: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    // ignore usage / id / type / etc — not needed for classification
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentValue {
    Text(String),                        // user 输入
    Blocks(Vec<ContentBlock>),           // assistant content + user tool_result wrapper
}

impl Default for ContentValue {
    fn default() -> Self {
        ContentValue::Blocks(vec![])
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    Thinking { thinking: String, #[serde(default)] signature: Option<String> },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool_use_id: String,
        content: serde_json::Value,      // 可能 string 或 array
    },
}
```

---

## 8. Sample messages (anonymized, real format)

### 8.1 User 输入

```json
{"parentUuid":null,"isSidechain":false,"promptId":"f5b892fe-...","type":"user","message":{"role":"user","content":"# AI产品付费内容市场调研\n\n## 角色与目标\n你是一名内容产品策略研究员..."},"uuid":"fb708732-8f6d-4ba0-915e-15ae7a61e5ec","timestamp":"2026-05-04T05:42:34.919Z","permissionMode":"default","userType":"external","entrypoint":"cli","cwd":"/Users/caiyiwen","sessionId":"179b8e5d-0235-4215-89a4-7aac8126ccbf","version":"2.1.126","gitBranch":"master"}
```

### 8.2 Assistant tool_use

```json
{"parentUuid":"83b77ac0-...","isSidechain":false,"message":{"model":"claude-opus-4-7","id":"msg_01HprtD7...","type":"message","role":"assistant","content":[{"type":"tool_use","id":"toolu_0176rGGDuQDFaz2dRdaPT291","name":"Skill","input":{"skill":"调研规范"},"caller":{"type":"direct"}}],"stop_reason":"tool_use","stop_sequence":null,"usage":{"input_tokens":6,"output_tokens":279,...}},"requestId":"req_...","type":"assistant","uuid":"dc444397-...","timestamp":"2026-05-04T05:42:40.374Z","userType":"external","entrypoint":"cli","cwd":"/Users/caiyiwen","sessionId":"179b8e5d-...","version":"2.1.126","gitBranch":"master"}
```

### 8.3 User tool_result wrapper

```json
{"parentUuid":"dc444397-...","isSidechain":false,"promptId":"f5b892fe-...","type":"user","message":{"role":"user","content":[{"type":"tool_result","tool_use_id":"toolu_0176rGGDuQDFaz2dRdaPT291","content":"Launching skill: 调研规范"}]},"uuid":"516f9faf-...","timestamp":"2026-05-04T05:42:40.380Z","toolUseResult":{"success":true,"commandName":"调研规范"},"sourceToolAssistantUUID":"dc444397-...","userType":"external","entrypoint":"cli","cwd":"/Users/caiyiwen","sessionId":"179b8e5d-...","version":"2.1.126","gitBranch":"master"}
```

### 8.4 Attachment (non-classification entry)

```json
{"parentUuid":"c993474c-...","isSidechain":false,"attachment":{"type":"task_reminder","content":[],"itemCount":0},"type":"attachment","uuid":"92cf9ba0-...","timestamp":"2026-05-18T11:59:37.075Z","userType":"external","entrypoint":"cli","cwd":"/Users/caiyiwen/ai-product-column-research/drafts","sessionId":"179b8e5d-...","version":"2.1.126","gitBranch":"master","slug":"streamed-enchanting-micali"}
```

→ 末尾 entry **可能是这种** type=attachment，monitor 反扫时必须跳过。

---

## 9. Version compatibility

| Claude Code version | Schema 状态 |
|---|---|
| 2.1.126 (本实测) | ✅ 上述完整正确 |
| 老版本 (2.0.x / 1.x) | 未知，假设兼容（添加字段 OK，删除字段会断） |
| 未来版本 | 持续观察 Anthropic 发布说明 |

### 兼容策略

- serde struct 用 `#[serde(default)]` + `Option<T>` 让缺失字段不 panic
- 严格依赖字段（`type` / `message.role` / `message.stop_reason`）缺失 → status=Unknown + log warn
- 加 CI fixture 测试（取真实 JSONL 样本固定到 `tests/fixtures/`）

---

## 10. Open questions (实测后剩余的)

1. **session cwd 变化时进程 cwd 是否跟变？**
   - 假设：进程 cwd 不变（cwd 是 process attribute，shell cd 不影响 parent process）
   - 待 S-001 实施时 verify
2. **历史 JSONL 文件什么时候清理？**
   - 实测 `-Users-caiyiwen/` 下有几十个 .jsonl，最早从 2 月（3 个月前）的还在
   - Claude Code 不主动清理
   - 对 monitor 影响：只看进程当前的 `sessionId` 对应文件，不会被 history 干扰
3. **如果 sessionId 对应的 .jsonl 文件被用户手动删除？**
   - claude 进程会重建吗？崩溃吗？
   - 未实测——属于 [F1 JSONL corrupted](../product/user-stories.md#f1--jsonl-损坏读不到) 边缘 case
4. **subprocess (skill / agent) 是否单独 sessionId？**
   - 实测 `isSidechain: false` 在所有看到的 entry。`isSidechain: true` 待确认是否 subprocess
   - 假设：subprocess 不独立显示在 monitor 列表（只看主进程 pid）

---

## 11. Downstream invalidation

本实测推翻了多份文档的假设。**需要 update 的下游文档**：

| 文档 | 需要改的 |
|---|---|
| [data-model.md](../bmad/03-solutioning/data-model.md) | JsonlMessage struct 重写（envelope + nested message + ContentValue enum） |
| [UML 09 State](../design/uml/09-state-session.md) | classify 逻辑：先 filter type，再用 stop_reason |
| [S-004 classifier story](../bmad/03-solutioning/epics/story-004-status-classifier.md) | acceptance 更新（filter type，用 stop_reason） |
| [S-002 JSONL locator story](../bmad/03-solutioning/epics/story-002-jsonl-locator.md) | acceptance：path encoding 已 verify ✓；进程 cwd ↔ session cwd 关系待 verify |
| [project-context § Naming convention](../bmad/03-solutioning/project-context.md) | enum 命名建议 (JsonlEnvelope / NestedMessage / ContentValue) |
| [PRD NFR-C4 Claude Code version](../bmad/02-planning/PRD.md) | 当前 baseline 是 2.1.126 |
| [PRD NFR-C1 macOS 版本](../bmad/02-planning/PRD.md) | 假设 macOS 12+，但实测机器 26.3.1——可能需要更新支持范围 |

→ **这些更新留作 S-002/S-004 实施时一并做**，不在 OQ-1 范围。本文档作为权威 reference。

---

## 12. Status

✅ **DONE** — OQ-1 / OQ-2 blocking 解除（仅剩 cwd-跟随 验证 in S-001 实施期）。
