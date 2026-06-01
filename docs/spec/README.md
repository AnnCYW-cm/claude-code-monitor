# Spec — Interface and format specifications

> **Purpose**: 这个目录放数据格式、协议、API 的**精确定义**——读者在 implement 或集成时不用问任何人就能写对。
>
> 跟 `docs/design/` 的关系：design 说"为什么这么设计"，spec 说"具体字段是这些"。

---

## 已规划

| 文件 | Status | 引用方 |
|---|---|---|
| [jsonl-schema.md](jsonl-schema.md) | **TBD** (作者待办) | user-stories E5/F1, BMAD architecture, research-notes |
| [logging.md](logging.md) | **TBD** | user-stories F1, story-012 |
| [ipc-contract.md](ipc-contract.md) | ✅ DONE | frontend ↔ backend IPC 字段约定 |

---

## 命名约定

- 文件名 = 被规格化的对象（`jsonl-schema.md` / `logging.md` / `ipc-contract.md`）
- 多版本共存时用后缀：`jsonl-schema-v2.md`（少见——通常应该原地 update 加版本表格）

---

## 什么进 spec，什么不进

| 进 spec | 不进 spec |
|---|---|
| JSONL 字段名 / 类型 / 约束 | "为什么用 JSONL 而不是 SQLite"（→ design/ADR） |
| IPC command 签名 + 字段 | "为什么 IPC 设计为同步阻塞"（→ ADR） |
| log file 路径 / 格式 / rotation | "为什么 log 写到 ~/Library/Logs"（→ ADR） |
| 错误码定义 | 错误恢复策略（→ design） |
