# Install Guide

> **Status:** Draft（macOS 26 路径基于 Apple 公开文档推断，未实测；其他版本 TBD —— **作者待办**）
> **Audience:** End users
> **Cross-ref:** [F4 Gatekeeper](../product/user-stories.md#f4--gatekeeper-拦截), [quickstart](../bmad/03-solutioning/quickstart.md)

---

## Three ways to install

### A. Homebrew Cask (待上线)

```bash
brew install --cask claude-code-monitor
```

**Status**: 待 v0.1 release 后提交 cask formula。

### B. DMG download (待 release)

1. 下载 `ClaudeCodeMonitor_0.1.0_universal.dmg` from GitHub Releases（URL 待 v0.1 release 后填入）
2. 双击 DMG → 拖 `ClaudeCodeMonitor.app` 到 `Applications/`
3. **首次打开**：见下面"Gatekeeper bypass"

### C. Build from source

需要 Node 18+ / Rust 1.77+ / macOS 12+。

```bash
git clone https://github.com/<owner>/claude-code-monitor.git
cd claude-code-monitor
npm install
npm run tauri:build
# 输出在 src-tauri/target/release/bundle/macos/
```

第一次 `cargo build` 会下载 ~500MB Rust crates（10-15 分钟），后续 build 几秒。

---

## Gatekeeper bypass (首次打开未签名 .app)

我们 v0.1 **不申请 Apple Developer Program / 不签名**（成本 $99/年 + notarization 复杂度）。所以首次打开 .app 会触发 macOS Gatekeeper 警告。

### 支持的 macOS 版本

| macOS 版本 | 状态 | 步骤实测 |
|---|---|---|
| 12 (Monterey) | 假定支持 | ❓ 未实测 |
| 13 (Ventura) | 假定支持 | ❓ 未实测 |
| 14 (Sonoma) | 假定支持 | ❓ 未实测 |
| 15 (Sequoia) | 假定支持 | ❓ 未实测 |
| 26 (Tahoe / 2026) | 假定支持 | ⚠️ 基于公开文档推断，**未实测**（需要 build .app 跑一遍才能确认） |
| 26+ | 假定向前兼容 | ❓ |

→ 作者只有 1 台 macOS 26 机器，没有 12-15 物理机器/VM。**跨版本验证依靠 beta 用户反馈**。看到 Gatekeeper 步骤跟下面不一样请开 GitHub issue。

### 步骤（macOS 26 推断版，待实测验证）

首次双击 .app 会看到：

```
"ClaudeCodeMonitor.app" 无法打开，
因为 Apple 无法检查它是否包含恶意软件。

[移到废纸篓]   [完成]
```

**绕过方法**：

#### 方法 1（推荐）：系统设置

1. 双击 .app 让 Gatekeeper 警告弹出，点击 **完成**
2. 打开 **系统设置 → 隐私与安全性**
3. 滚动到底部 "安全性" 一节
4. 找到 `已阻止 ClaudeCodeMonitor 因为来自身份不明的开发者`
5. 点击 **仍要打开**
6. 再次双击 .app → 现在弹窗会有 **打开** 选项
7. 点 **打开**

#### 方法 2（macOS 14 及更早）：右键 Open Anyway

1. 在 `/Applications/` 找到 `ClaudeCodeMonitor.app`
2. **按住 Control 键 + 点击** .app（或右键）
3. 选 **打开**
4. 弹窗里点 **打开**

⚠️ macOS 15 (Sequoia) 起，Apple 加强了 Gatekeeper，右键 Open Anyway 路径**可能**不再有效。如果你在 15+ 上看到右键菜单没有"打开" 选项，走方法 1。

#### 方法 3（开发者命令行）：`xattr -d`

```bash
xattr -d com.apple.quarantine /Applications/ClaudeCodeMonitor.app
```

清掉文件的 quarantine 属性后双击就可打开。**仅推荐给信任本项目源码的开发者**。

### 实测 todo

**作者待办**：

- [ ] macOS 26 实测一遍上述 3 个方法，截图 fix 描述（需要先 build .app）
- [ ] 找 macOS 12/14/15 用户帮测，或者用 VM
- [ ] 收集 beta 用户报告的版本差异

实测后本节会更新到 **Status: ✅ DONE**。

---

## First run

成功打开后：

1. menubar 出现一个小图标 (黑色实心圆，macOS auto-tints to dark mode)
2. 点击图标 → popup 显示
3. 如果没开任何 claude session，会显示 "no claude sessions running"
4. 在 terminal 任意目录运行 `claude`
5. 切回 menubar 点 popup → 看到 1 个 session

---

## Uninstall

```bash
# 1. Quit app (右键 tray icon → Quit)
# 2. 删 app
rm -rf /Applications/ClaudeCodeMonitor.app
# 3. 删 log (可选)
rm -rf ~/Library/Logs/com.caiyiwen.claude-code-monitor
```

App 不创建任何其他文件。**Zero residue uninstall**（[constitution I.2 零配置](../constitution.md) 的延伸——不留垃圾）。

---

## Troubleshooting

### Tray icon 不出现

- 检查 Activity Monitor 是否有 `ClaudeCodeMonitor` 进程
- 如果有但 tray 不显示：可能被 Bartender / Hidden Bar 等管理工具隐藏
- 没进程：见 [F3 app 崩溃](../product/user-stories.md#f3--app-自己崩了)，重新打开

### tray icon 但点击无反应

- 已知 issue（v0.1 alpha）：第一次启动后 ~1s 内点击 tray 可能没反应（webview 还在初始化）
- 等 2-3 秒再点

### Popup 显示 session 但 status 永远 unknown

- 可能 JSONL 格式跟我们 spec ([../spec/jsonl-schema.md](../spec/jsonl-schema.md)) 假设的不一致
- 报 issue 附上你的 Claude Code 版本（`claude --version`）+ macOS 版本

### 更多

收集 alpha 用户反馈后持续追加。

---

## License

MIT — see [LICENSE](../../LICENSE) at project root.
