# Security Policy

## 报告漏洞

如果发现安全漏洞，**请不要开 public issue**。直接通过以下方式联系：

- 邮件: `caiyiwenann@gmail.com`
- Subject: `[SECURITY] Claude Code Monitor - <短描述>`

我会在 **3 个工作日内** 回复（v0.1 单作者节奏；正式发布后 24h SLA）。

## 此项目的安全保证

Claude Code Monitor 是 **完全本地** 的 macOS utility，安全面很窄：

| 行为 | 状态 |
|---|---|
| 访问网络 | ❌ 不访问任何网络（[constitution NFR-S1](docs/constitution.md)） |
| 读 `~/.claude/` 以外文件 | ❌ 不读 |
| 读 OAuth token / API key | ❌ 不读（只 parse JSONL transcript 字段，不 parse token 字段） |
| 写文件（除 log） | ❌ 仅写 `~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log` |
| 上传 telemetry | ❌ 不上传 |
| 操作其他进程 | ❌ 只 read 进程列表，不 kill/inject |
| 需要 macOS 特殊权限 | ❌ 不需要（沿用 user 进程权限） |

## 已知未做的安全措施

按 [architecture § 7.3](docs/bmad/03-solutioning/architecture.md)，**显式没做**：

- ❌ App 未签名 / 未公证（v0.1 不申请 Apple Developer Program，详 [F4 install guide](docs/guides/install.md)）
- ❌ 不加自我完整性校验
- ❌ 不加 anti-tampering

→ 用户**应该从 GitHub Release 验证 SHA-256 校验和**，不要从其他源下载。

## 攻击面

主要风险：

| 风险 | 严重度 | 缓解 |
|---|---|---|
| JSONL 含恶意构造字段触发 parser panic | 低 | serde 是 safe Rust，最坏返回 Err；panic 被 `catch_unwind` 隔离（[S-011](docs/bmad/03-solutioning/epics/story-011-error-handling.md)） |
| `~/.claude/` 被替换为大文件（DoS） | 极低 | 我们只读最后一行，不读整文件 |
| Supply chain (依赖 crate 被劫持) | 中 | Cargo.lock 锁定 + Dependabot（v0.2+ 加） |
| Gatekeeper bypass 后 .app 被替换 | 用户自负 | 见下方建议 |

## 用户建议

- 从 GitHub Release 下载 DMG，不从其他源（URL 待 v0.1 release 后填入）
- 验证 SHA-256 校验和（每次 release 附）
- 如果在意，可以从 source build（[install § C](docs/guides/install.md)）

## 不属于本项目安全责任的

- Claude Code (Anthropic) 本身的安全
- macOS 系统漏洞
- 用户的 `~/.claude/` 目录权限管理

---

## 公开披露

漏洞修复发布后，会在 [CHANGELOG.md](CHANGELOG.md) `Security` 章节注明（不含敏感细节）。

历史漏洞（如有）：

- （无）
