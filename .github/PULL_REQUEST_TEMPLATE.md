<!--
谢谢提 PR！请填完下面 sections，缺失的会让 review 慢。

提交前必读：[docs/guides/CONTRIBUTING.md](../docs/guides/CONTRIBUTING.md)
-->

## Story

<!-- Closes 哪个 dev story？link 到 docs/bmad/03-solutioning/epics/story-NNN-XXX.md -->

Closes #N (story `S-NNN`)

## What changed

<!-- 简要描述这个 PR 做了什么 -->

- 

## Why

<!-- 解释 WHY 不仅 what。如果是 fixing bug，描述 root cause。 -->



## How tested

<!-- 你怎么 verify 这个 PR work 的？ -->

- [ ] 跑了相关 `cargo test`
- [ ] 跑了 `cargo bench` (如适用)
- [ ] 手动测了对应 user story acceptance
- [ ] 14 天 dogfood 中观察过 (如适用)

具体步骤：

1. 
2. 
3. 

## Checklist

<!-- PR 进 review 前必须全过 -->

- [ ] **All acceptance criteria of related story 已 met**
- [ ] Unit tests 加了 (新功能) 或更新了 (修复)
- [ ] **没违反 [constitution](../docs/constitution.md) 红线**（如有，先开 ADR）
- [ ] **没引入 [forbidden dependency](../docs/bmad/03-solutioning/project-context.md#33-不允许的依赖)** (CI grep 会查)
- [ ] **Performance budget verified** (如适用，对照 [PRD § 6.1](../docs/bmad/02-planning/PRD.md))
- [ ] 相关文档更新了 (如 PR 改了 spec/API/UX 行为)
- [ ] `cargo clippy` no new warnings
- [ ] `cargo fmt` 跑过
- [ ] **commit message 第一行 ≤ 70 字符**

## Screenshot / Recording

<!-- UI 改动：放截图或录屏 -->



## Anything else

<!-- 评审者要注意的 -->



---

<!-- Maintainer review 节奏：每周末 1 次 office hour（见 docs/constitution.md V.3）。-->
