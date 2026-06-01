# Guides — Operational documentation

> **Purpose**: 给**用户**和**贡献者**看的操作指南——按步骤动手。

---

## 已规划

| 文件 | Status | 受众 |
|---|---|---|
| [install.md](install.md) | **TBD** (作者待办) | 用户 / 装 app 时 |
| dev-setup.md | TBD | 贡献者 / 第一次 build |
| CONTRIBUTING.md | TBD | 贡献者 / 提 PR 前 |
| release.md | TBD | 维护者 / 发版时 |

---

## 跟 docs/ 其他目录的边界

| 进 guides | 进别的目录 |
|---|---|
| 按步骤的 how-to | 设计思路 (→ design/) |
| 用户面向的安装/使用文档 | 产品定义 (→ product/) |
| 贡献者面向的工程文档 | 数据规格 (→ spec/) |
| Gatekeeper bypass 实测步骤 | 为什么不签名 (→ ADR) |

---

## 风格约定

- 命令直接给可复制：```bash 块
- 截图放 `docs/guides/assets/<topic>/`
- 多平台说明用 tab 或并列段
- 失败 path 也写（"如果 X 你会看到 Y，做 Z"）
