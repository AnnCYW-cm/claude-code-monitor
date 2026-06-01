# Data Model — 001-mvp

> **Data model: Rust + TS data types for Session, IPC payloads, classification rules** — originally from Spec Kit `/speckit.plan` supporting, mv to BMAD by [ADR-013](../02-planning/decision-log.md)
> **Status:** ✅ JSONL types updated (2026-05-18) against real schema in [spec/jsonl-schema.md](../../spec/jsonl-schema.md)
>
> 定义后端 Rust struct + 前端 TS interface + 跨 IPC 序列化约定。
> 跟 [UML 05 Class diagram](../../design/uml/05-class.md) 互补：UML 是图，本文档是字段精确定义。
>
> **JSONL parsing struct（§ 2.2 ~ 2.4）以 [spec/jsonl-schema.md](../../spec/jsonl-schema.md) 为准——本文档曾基于假设猜测，2026-05-18 实测后整改。**

---

## 1. Core types

### 1.1 Session (DTO, 跨 IPC)

后端到前端的 session 表示。

#### Rust

```rust
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Session {
    pub pid: u32,
    pub cwd: String,
    pub status: SessionStatus,
    pub last_message: Option<String>,
    pub last_update_unix: Option<u64>,
    pub waiting_since_unix: Option<u64>,
}
```

#### TypeScript

```ts
interface Session {
  pid: number;
  cwd: string;
  status: "waiting" | "working" | "unknown";
  last_message: string | null;
  last_update_unix: number | null;
  waiting_since_unix: number | null;
}
```

#### Field semantics

| Field | Type | Required | Description |
|---|---|---|---|
| `pid` | `u32` / `number` | yes | OS 进程 ID |
| `cwd` | `String` / `string` | yes | claude 进程的当前工作目录 (absolute path) |
| `status` | enum (`Waiting/Working/Unknown`) | yes | 当前状态 (see `SessionStatus`) |
| `last_message` | `Option<String>` / `string \| null` | no | 最后一条 assistant 消息文本；Unknown 时为 None |
| `last_update_unix` | `Option<u64>` / `number \| null` | no | 最后一条 user/assistant entry 的 timestamp，转 Unix seconds。注：JSONL 里 timestamp 是 ISO 8601 string（如 `"2026-05-04T05:42:34.919Z"`），后端 parse with `chrono::DateTime::parse_from_rfc3339()` 后转 u64 暴露给前端 |
| `waiting_since_unix` | `Option<u64>` / `number \| null` | no | 进入 waiting 状态的时间戳；仅 waiting 时 Some |

---

### 1.2 SessionStatus enum

#### Rust

```rust
use serde::Serialize;

#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Waiting,
    Working,
    Unknown,
}
```

#### TypeScript

```ts
type SessionStatus = "waiting" | "working" | "unknown";
```

#### Classification rules

跟 [spec/jsonl-schema.md § 6](../../spec/jsonl-schema.md) 一致（实测推翻了之前的假设）：

1. 反向扫 JSONL，跳过非 user/assistant 类型 entry（attachment / file-history-snapshot / permission-mode / ai-title / last-prompt）
2. 找到第一条 `type == "user"` 或 `type == "assistant"` 的 entry → 判 status

| Input（last meaningful envelope） | Output |
|---|---|
| `None` (parse 失败 / JSONL 全是非 user/assistant) | `Unknown` |
| `envelope.kind == "assistant"` AND `message.stop_reason == "end_turn"` | `Waiting` |
| `envelope.kind == "assistant"` AND `stop_reason` 不是 `end_turn`（如 `tool_use`） | `Working` |
| `envelope.kind == "user"`（含 tool_result wrapper） | `Working` |

详见 [UML 09 State](../../design/uml/09-state-session.md) + [story-004 classifier](epics/story-004-status-classifier.md) + [spec/jsonl-schema.md § 6.2 伪代码](../../spec/jsonl-schema.md)。

---

## 2. Internal types (not crossing IPC)

### 2.1 RawProcess (sysinfo wrapper output)

```rust
#[derive(Debug)]
pub struct RawProcess {
    pub pid: u32,
    pub cwd: PathBuf,
    pub started_at: SystemTime,
}
```

| Field | Description |
|---|---|
| `pid` | 进程 ID |
| `cwd` | claude 进程的 cwd (PathBuf, ` Process::cwd()` 解出来) |
| `started_at` | 进程启动时间 (用于 same-cwd 多进程配对启发式) |

→ 跟 Session 区别：RawProcess 是 sysinfo 数据，Session 是带 status 的 DTO。

---

### 2.2 JsonlEnvelope (顶层 wrapper，每行一个)

✅ **基于 [spec/jsonl-schema.md](../../spec/jsonl-schema.md) § 2 实测**（推翻了之前的猜测——之前以为 role/content 在顶层，实际在 nested `message`）：

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonlEnvelope {
    #[serde(rename = "type")]
    pub kind: String,                       // "user" / "assistant" / "attachment" / "file-history-snapshot" / "permission-mode" / "ai-title" / "last-prompt"
    pub uuid: String,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    pub timestamp: String,                  // ISO 8601 with millis, e.g. "2026-05-04T05:42:34.919Z"
    #[serde(rename = "sessionId")]
    pub session_id: String,                 // 跟文件名 uuid 一致
    pub cwd: Option<String>,                // ⚠️ 可能在某些 entry 缺失；同一 session 内可变化
    pub version: Option<String>,            // Claude Code 版本 e.g. "2.1.126"
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    pub message: Option<NestedMessage>,     // 仅在 kind=user/assistant 时存在
    // 其他 envelope 字段（attachment / snapshot / etc）忽略，monitor 不需要
}
```

### 2.3 NestedMessage (在 envelope.message 里)

```rust
#[derive(Deserialize, Debug)]
pub struct NestedMessage {
    pub role: String,                       // "user" / "assistant"
    #[serde(default)]
    pub content: ContentValue,              // ⚠️ string OR array (untagged enum)
    #[serde(rename = "stop_reason")]
    pub stop_reason: Option<String>,        // "end_turn" / "tool_use" / "max_tokens" / etc
    #[serde(default)]
    pub model: Option<String>,
    // 忽略 id / type / usage / stop_sequence 等不用的字段
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentValue {
    /// user 人类输入 → 直接是 string
    Text(String),
    /// assistant content + user tool_result wrapper → array of ContentBlock
    Blocks(Vec<ContentBlock>),
}

impl Default for ContentValue {
    fn default() -> Self {
        ContentValue::Blocks(vec![])
    }
}
```

### 2.4 ContentBlock (在 NestedMessage.content array 里)

```rust
#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    Thinking {
        thinking: String,
        #[serde(default)]
        signature: Option<String>,
    },
    ToolUse {
        id: String,
        name: String,                       // "Bash" / "Read" / "Edit" / etc
        input: serde_json::Value,           // tool 参数 object
    },
    ToolResult {
        tool_use_id: String,
        content: serde_json::Value,         // string OR array of Text blocks
    },
}
```

### 2.5 关键 classify helper

```rust
/// 从一组 JSONL line 反向扫，返回最后一条 type ∈ {user, assistant} 的 envelope。
/// 跳过 attachment / file-history-snapshot / permission-mode / ai-title / last-prompt 等。
pub fn last_meaningful(lines: &[String]) -> Option<JsonlEnvelope> {
    for line in lines.iter().rev() {
        let env: JsonlEnvelope = serde_json::from_str(line).ok()?;
        if env.kind == "user" || env.kind == "assistant" {
            return Some(env);
        }
    }
    None
}

pub fn classify(env: Option<JsonlEnvelope>) -> SessionStatus {
    let env = match env {
        None => return SessionStatus::Unknown,
        Some(e) => e,
    };
    if env.kind != "assistant" {
        return SessionStatus::Working;   // user 或 tool_result
    }
    let stop = env.message.as_ref().and_then(|m| m.stop_reason.as_deref());
    match stop {
        Some("end_turn") => SessionStatus::Waiting,
        _ => SessionStatus::Working,     // tool_use / max_tokens / etc
    }
}
```

---

### 2.3 LocateError

JSONL 定位的错误枚举。

```rust
use std::io;

#[derive(Debug)]
pub enum LocateError {
    DirNotFound,        // ~/.claude/projects/<encoded>/ 不存在
    NoJsonlFiles,       // 目录存在但无 .jsonl 文件
    NoActiveJsonl,      // 有 .jsonl 但都 stale (mtime > 60s ago)
    IoError(io::Error),
}

impl From<io::Error> for LocateError {
    fn from(e: io::Error) -> Self {
        LocateError::IoError(e)
    }
}
```

---

## 3. Persistence

**No persistence.** 这是产品原则 (constitution § I.3 "now not history")：
- 不存数据库
- 不存配置
- 不存 cache
- 仅写 log file (debug 用，非业务数据)

唯一的"持久化"：app 退出再启动时，重新枚举进程 + 读 JSONL。

---

## 4. Field naming convention

| Layer | Convention | Example |
|---|---|---|
| Rust struct field | `snake_case` | `last_message` |
| serde to JSON | `snake_case` (不转换) | `"last_message": "..."` |
| TS interface | `snake_case` (镜像 Rust) | `last_message` |

→ 不在 IPC 边界做 case 转换，减少 friction。

---

## 5. Versioning

### 5.1 Schema version

MVP **不在 DTO 里加 version 字段**。理由：
- 单 binary 单 frontend，部署一致
- 无外部 consumer

### 5.2 Future migration

如果未来 expose IPC 给第三方（v1.0+ unlikely）：
- 加 `schema_version: u32` 到 Session
- 改字段用 additive 风格（only add new optional, never remove）
- Breaking change → 单独 IPC command

---

## 6. Validation rules

### 6.1 Inbound (from external sources)

| Source | Validation |
|---|---|
| sysinfo `Process` | `name()` not empty; `cwd()` is Some (else skip session) |
| JSONL parse | serde 自动校验 schema；fail → Unknown |
| User clicks | none (trusted) |

### 6.2 Outbound (to frontend via IPC)

| Field | Constraint |
|---|---|
| `pid` | > 0 |
| `cwd` | non-empty string |
| `status` | one of `waiting/working/unknown` |
| `last_message` | length unbounded (可以很长，前端自己处理) |
| `last_update_unix` | > 1577836800 (sanity check：2020-01-01) |
| `waiting_since_unix` | Some 当且仅当 status == waiting |

---

## 7. Examples

### 7.1 Happy path

```json
{
  "pid": 12345,
  "cwd": "/Users/caiyiwen/work/api-server",
  "status": "waiting",
  "last_message": "All 142 tests passed. Want me to commit with message 'fix: token validation edge case'?",
  "last_update_unix": 1763472510,
  "waiting_since_unix": 1763472510
}
```

### 7.2 Working

```json
{
  "pid": 12346,
  "cwd": "/Users/caiyiwen/work/api-server-tests",
  "status": "working",
  "last_message": "Running integration suite (16 of 42 complete)...",
  "last_update_unix": 1763472495,
  "waiting_since_unix": null
}
```

### 7.3 Unknown (JSONL corrupted)

```json
{
  "pid": 12347,
  "cwd": "/Users/caiyiwen/work/blog",
  "status": "unknown",
  "last_message": null,
  "last_update_unix": null,
  "waiting_since_unix": null
}
```

### 7.4 Empty (no sessions)

```json
[]
```

---

## 8. Frontend derived fields

Frontend 不存额外字段，但 render 时计算：

### 8.1 `cwdName` (display name)

```ts
const cwdName = session.cwd.split('/').filter(Boolean).pop() ?? `pid ${session.pid}`;
```

### 8.2 `durationLabel` (waiting 时长)

```ts
function durationLabel(waitingSince: number | null): string | null {
  if (!waitingSince) return null;
  const elapsedSec = Date.now() / 1000 - waitingSince;
  const minutes = Math.floor(elapsedSec / 60);
  if (minutes < 1) return "just now";
  return `${minutes}min`;
}
```

### 8.3 `messagePreview` (列表单行预览)

```ts
function messagePreview(msg: string | null): string {
  if (!msg) return "";
  return msg.split('\n')[0].slice(0, 80);
}
```

---

## 9. Cross-reference

| Topic | Where |
|---|---|
| Spec FR | [spec.md § 5](../02-planning/PRD.md) |
| Plan implementation | [plan.md § 3](architecture.md) |
| Classification logic | [UML 09](../../design/uml/09-state-session.md) + [S-004](../../bmad/03-solutioning/epics/story-004-status-classifier.md) |
| Class diagram | [UML 05](../../design/uml/05-class.md) |
| IPC fields | [../../spec/ipc-contract.md](../../spec/ipc-contract.md) |
| Constitution data principles | [constitution § I.3](../../constitution.md) |
