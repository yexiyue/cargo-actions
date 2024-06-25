# cargo-actions cli

[![GitHub Stars](https://img.shields.io/github/stars/yexiyue/cargo-actions?style=flat-square)](https://github.com/yexiyue/cargo-actions) [![Crates.io](https://img.shields.io/crates/v/cargo-actions?style=flat-square)](https://crates.io/crates/cargo-actions)

Cargo Actions 是一个基于 Rust 语言开发的命令行工具，旨在为 GitHub Actions 提供高效的工作流模板管理和部署功能。它通过与 GitHub 账号的集成，允许用户方便地上传、搜索和应用工作流模板，从而简化了持续集成和持续部署（CI/CD）的流程。

## 功能特性

- **用户认证登录**：通过 OAuth 2.0 协议与 GitHub 进行安全集成，用户可以使用他们的 GitHub 账号登录到 Cargo Actions。
- **工作流初始化**：支持从 GitHub 仓库或模板 ID 初始化工作流，提供了灵活的方式来集成 GitHub Actions 工作流。
- **模板上传与分享**：用户可以将自己创建的工作流模板上传到 [Cargo Actions](https://yexiyue.github.io/actions-workflow/) 平台，并与其他用户分享。
- **个性化模板管理**：允许用户管理自己上传和收藏的模板，方便快速启动熟悉或常用的工作流配置。

## 安装

在终端中运行以下命令：

```bash
cargo install cargo-actions
```

## 使用

### 初始化

使用 github 仓库 url 创建项目，可以省略<https://github.com/前缀。默认会使用https://github.com/yexiyue/cargo-actions里的工作流模版。>

使用省略形式，规则（User/Repo）

```bash
cargo actions init yexiyue/cargo-actions
```

使用 url 形式

```bash
cargo actions init https://github.com/yexiyue/cargo-actions.git
```

使用 ssh 形式

```bash
cargo actions init git@github.com:yexiyue/cargo-actions.git
```

![init命令](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/init命令.gif)

同时也可以使用[Cargo Actions 平台](https://yexiyue.github.io/actions-workflow)上的工作流，

![](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/image.webp)

复制喜欢的工作流模版到终端

示例：

```bash
cargo actions init 1 -i
```

![](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/id_init.gif)

### 上传模版

如果你想上传自己的工作流到 cargo actions 平台的话，请先登陆。

```bash
cargo actions login
```

然后准备一个工作流模版

一个标准的工作流模版应该具有下面文件

![](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/template-dir.webp)

- cargo-action.json：配置文件，用于提示用户输入
- 模版名.yaml.hbs：模版文件
- README.md(可选)

`cargo-action.json` 配置字段说明

| 字段名          | 类型     | 描述                                    |
| --------------- | -------- | --------------------------------------- |
| name            | string   | 模板名称                                |
| description     | string   | 模板简短描述                            |
| path            | string   | 模板文件路径，默认为 `${name}.yaml.hbs` |
| prompts         | Prompt[] | 定义命令行交互输入项                    |
| success_message | string   | 模板创建成功后的提示信息                |

Prompt 配置说明

prompt 有以下 4 种类型

1. type:"input"

| 字段名  | 类型   | 描述                             |
| ------- | ------ | -------------------------------- |
| field   | string | 字段名称（与模版中的变量名对应） |
| prompt  | string | 输入提示                         |
| default | string | 默认值                           |

2. type:"confirm"

| 字段名  | 类型   | 描述                             |
| :------ | ------ | -------------------------------- |
| field   | string | 字段名称（与模版中的变量名对应） |
| prompt  | string | 输入提示                         |
| default | bool   | 默认值                           |

3. type:"select"

| 字段名  | 类型                       | 描述                                                     |
| ------- | -------------------------- | -------------------------------------------------------- |
| field   | string                     | 字段名称（与模版中的变量名对应）                         |
| prompt  | string                     | 输入提示                                                 |
| default | number                     | 默认选项对应的索引值                                     |
| options | {value:any,label:string}[] | 选项列表，label 是提示的值，value 是最后在模版中使用的值 |

4. type:"multiselect"

| 字段名  | 类型                       | 描述                                                     |
| ------- | -------------------------- | -------------------------------------------------------- |
| field   | string                     | 字段名称（与模版中的变量名对应）                         |
| prompt  | string                     | 输入提示                                                 |
| default | number[]                   | 默认选项对应的索引值数组                                 |
| options | {value:any,label:string}[] | 选项列表，label 是提示的值，value 是最后在模版中使用的值 |

示例：

```json
{
  "name": "web-deploy",
  "description": "构建web 应用到Github Pages",
  "prompts": [
    {
      "type": "select",
      "field": "toolchain",
      "prompt": "请选择包管理工具",
      "default": 0,
      "options": [
        {
          "label": "npm",
          "value": "npm"
        },
        {
          "label": "yarn",
          "value": "yarn"
        },
        {
          "label": "pnpm",
          "value": "pnpm"
        }
      ]
    },
    {
      "type": "confirm",
      "field": "enable_cache",
      "prompt": "是否启用缓存",
      "default": true
    },
    {
      "type": "input",
      "field": "node_version",
      "prompt": "请输入node版本号",
      "default": "node"
    },
    {
      "type": "input",
      "field": "folder",
      "prompt": "web 项目路径",
      "default": "."
    },
    {
      "type": "input",
      "prompt": "构建产物目录(相对于web 项目路径)",
      "field": "target_dir",
      "default": "dist"
    },
    {
      "type": "confirm",
      "prompt": "是否需要复制index.html 为404.html 以支持spa",
      "field": "copy_index",
      "default": false
    }
  ]
}
```

模版文件是使用[handlebars](https://docs.rs/handlebars/latest/handlebars/)渲染的，模版语法可以参考[Handlebars (handlebarsjs.com)](https://handlebarsjs.com/guide/)

模版文件示例：

```handlebars
name: web on: push: branches: - "master" workflow_dispatch: jobs: deploy:
runs-on: ubuntu-latest permissions: contents: write concurrency: group:
{{#raw}}$\{{ github.workflow }}-$\{{ github.ref }}{{/raw}}
steps: - name: Checkout repository uses: actions/checkout@v4
{{#if (eq toolchain "pnpm")}}
  - name: Install pnpm run: npm install -g pnpm
{{/if}}
- name: Sync node version and setup cache uses: actions/setup-node@v4 with:
node-version: "{{node_version}}"
{{#if enable_cache}}
  {{#if (eq toolchain "pnpm")}}
    cache: "{{folder}}/pnpm-lock.yaml"
  {{/if}}
  {{#if (eq toolchain "npm")}}
    cache: "{{folder}}/package-lock.json"
  {{/if}}
  {{#if (eq toolchain "yarn")}}
    cache: "{{folder}}/yarn.lock"
  {{/if}}
{{/if}}
- name: Install dependencies run: | cd
{{folder}}
{{toolchain}}
install - name: Build run: | cd
{{folder}}
{{toolchain}}
build
{{#if copy_index}}
  cp
  {{target_dir}}/index.html
  {{target_dir}}/404.html
{{/if}}
- name: Deploy uses: peaceiris/actions-gh-pages@v4 with:
{{#raw}}github_token: $\{{ secrets.GITHUB_TOKEN }}{{/raw}}
publish_dir:
{{folder}}/{{target_dir}}
```

注意:

`{{{{raw}}}}    {{{{/raw}}}}`中的表达式不会被转义。

在上传之前，可以使用`check`命令验证工作流模版是否正常工作。

```bash
cargo actions check
```

![](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/check.gif)

然后使用`upload`命令上传工作流模版

```bash
cargo actions upload
```

![](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/upload.webp)

### 使用创建的模版

使用以下命令可以快速使用自己创建的工作流模版，注意需要登陆。

```bash
cargo actions mine
```

同时您也可以登陆 Cargo Actions 平台[个人中心](https://yexiyue.github.io/actions-workflow/user-center/mine)里查看自己创建的工作流模版。

![](https://yexiyue.github.io/picx-images-hosting/cargo-actions-readme/mine.webp)

### 使用收藏的模版

```bash
cargo actions favorite
```

该命令使用与`mine`命令类似，从您在 Cargo Actions 平台收藏的模版中进行选择工作流，来初始化。

### 更多用法使用下面命令查看

```bash
cargo actions --help
```

## 技术栈

- **Rust**：作为核心编程语言，Rust 确保了应用程序的高效率和内存安全。
- **Cynic**：作为 GraphQL 客户端，它使得与后端服务的数据交互变得高效和灵活。
- **git2**：该库支持对 GitHub 仓库进行克隆和高级操作。
- **Clap**：此库用于创建功能丰富且用户友好的命令行界面。
- **Dialogue-macro**：它优化了命令行交互，提供清晰直观的提示和反馈。

## 贡献与反馈

如果你对 Cargo Actions 有任何贡献或反馈，欢迎通过 GitHub 仓库进行提交。欢迎任何形式的贡献，包括代码改进、文档更新、新功能建议等。

## 许可证

Cargo Actions 遵循 MIT 许可证。
