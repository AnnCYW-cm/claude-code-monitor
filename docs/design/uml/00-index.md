# UML 文档索引

按以下顺序阅读，从"这是什么"到"它怎么实现"逐层下钻。

## 用户视角（先看）

| # | 图 | 回答 |
|---|---|---|
| [01](01-use-case.md) | Use Case | 谁用、能做什么 |
| [02](02-activity-end-to-end.md) | Activity | 用户一次完整使用的流程 |

## 系统结构（再看）

| # | 图 | 回答 |
|---|---|---|
| [03](03-component.md) | Component | 系统由哪些模块组成、谁跟谁说话 |
| [04](04-package.md) | Package | 代码包的组织 |
| [05](05-class.md) | Class | 关键数据结构 |

## 动态行为（最后看）

| # | 图 | 回答 |
|---|---|---|
| [06](06-sequence-startup.md) | Sequence | 启动时发生了什么 |
| [07](07-sequence-refresh.md) | Sequence | 每次刷新轮询的时序 |
| [08](08-sequence-tray-click.md) | Sequence | 用户点击 tray 时的响应 |
| [09](09-state-session.md) | State Machine | 单个 session 的状态机 |
| [10](10-deployment.md) | Deployment | 部署在用户机器上的拓扑 |

## 工具

所有图用 **Mermaid** 写成。GitHub 网页自动渲染；VS Code 装 "Markdown Preview Mermaid Support" 扩展可在编辑器内预览。

## 范围

覆盖的是 MVP（v0.1）。后续扩展（多设备同步、跨平台、historical sessions、跳转到终端 tab）不在此处建模。产品定义见 `docs/product-definition-v0.2.md`（待补）。
