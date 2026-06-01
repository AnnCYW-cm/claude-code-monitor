---
name: Bug report
about: 报告一个 bug
title: '[BUG] '
labels: bug
assignees: ''
---

<!-- 谢谢报告 bug！请尽量填完下面所有字段，缺失的字段会让 debug 困难得多。 -->

## 环境

- **App 版本**：(查看右键 tray → About，未来加；或者 git tag)
- **macOS 版本**：(`sw_vers` 输出的 ProductVersion)
- **硬件**：(M1 / M2 / M3 / Intel + 具体型号)
- **Claude Code 版本**：(`claude --version`)
- **同时跑的 claude session 数**：(大概几个)
- **终端 emulator**：(Terminal.app / iTerm2 / Ghostty / Warp / Alacritty / 其他)

## Bug 描述

<!-- 一句话说 bug 是什么 -->



## 重现步骤

<!-- 越具体越好，让我能在我的机器上跑 -->

1. 
2. 
3. 

## 期望行为

<!-- 你认为应该发生什么 -->



## 实际行为

<!-- 实际发生了什么 -->



## 截图 (如适用)

<!-- 拖拽截图到这里 -->



## Log 文件相关片段

<!--
打开 ~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log
找到 bug 发生时间附近的 ERROR / WARN 行，粘贴到下面（脱敏后）。
注意：log 可能含 cwd 路径，请检查是否敏感。
-->

```
<paste here>
```

## Additional context

<!-- 任何其他可能有用的信息 -->



---

<!--
报告前 quick check（节省你和维护者时间）：
- [ ] 已搜索 existing issues 确认这个 bug 没被报过
- [ ] 已确认这不是 spec 设计如此 (查 docs/bmad/02-planning/PRD.md § 14 Out of scope)
- [ ] 已尝试 quit app + 重新打开
- [ ] 已确认 ~/.claude/projects/ 下有 .jsonl 文件
-->
