# cargo-actions cli

Cargo Actions 是一个基于 Rust 语言开发的命令行工具，旨在为 GitHub Actions 提供高效的工作流模板管理和部署功能。它通过与 GitHub 账号的集成，允许用户方便地上传、搜索和应用工作流模板，从而简化了持续集成和持续部署（CI/CD）的流程。

## 功能特性

- **用户认证登录**：通过 OAuth 2.0 协议与 GitHub 进行安全集成，用户可以使用他们的 GitHub 账号登录到 Cargo Actions。
- **工作流初始化**：支持从 GitHub 仓库或模板 ID 初始化工作流，提供了灵活的方式来集成 GitHub Actions 工作流。
- **模板上传与分享**：用户可以将自己创建的工作流模板上传到 [Cargo Actions](https://yexiyue.github.io/actions-workflow/) 平台，并与其他用户分享。
- **个性化模板管理**：允许用户管理自己上传和收藏的模板，方便快速启动熟悉或常用的工作流配置。



## 安装

Cargo Actions 可以通过以下步骤进行安装和使用：

1. 在终端中运行以下命令：

```bash
cargo install cargo-actions
```



## 基础使用

使用github 仓库url创建项目，可以省略https://github.com/前缀。默认会使用https://github.com/yexiyue/cargo-actions里的工作流模版。

使用省略形式，规则（User/Repo）

```bash
cargo actions init yexiyue/cargo-actions
```

使用url形式

```bash
cargo actions init https://github.com/yexiyue/cargo-actions.git
```

使用ssh形式

```bash
cargo actions init git@github.com:yexiyue/cargo-actions.git
```

![init命令](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/init命令.gif)

同时也可以使用[Cargo Actions平台](https://yexiyue.github.io/actions-workflow)上的工作流，

![](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/image.webp)

复制喜欢的工作流模版到终端

示例：

```bash
cargo actions init 1 -i
```



## 技术实现



## 贡献与反馈

如果你对 Cargo Actions 有任何贡献或反馈，欢迎通过 GitHub 仓库进行提交。我们欢迎任何形式的贡献，包括代码改进、文档更新、新功能建议等。

## 许可证

Cargo Actions 遵循 MIT 许可证。

