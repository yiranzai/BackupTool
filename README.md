# 数据备份工具

本项目是一个基于 Tauri 框架的桌面应用程序，旨在提供文件管理和任务调度功能。它允许用户复制目录，保存和加载 JSON 数据，以及在文件管理器中显示特定路径。此外，该应用还支持通过图形用户界面进行交互，提供了一套完整的功能来管理文件和任务。

## 主要功能

目录复制：异步复制源目录到目标目录，包括递归复制所有子目录和文件。复制过程中会根据当前时间创建新的目标子目录，以便组织和管理复制的内容。
JSON 数据管理：允许用户将 JSON 数据保存到指定文件中，或从文件中加载 JSON 数据。这可以用于配置管理或数据持久化。
文件管理器集成：提供在文件管理器中显示指定路径的功能，方便用户直接访问和查看文件或目录。
任务调度界面：虽然具体的界面代码未在提供的代码段中展示，但从上下文中可以推断出，该应用提供了一个任务管理器界面，允许用户添加、配置和查看任务。这可能包括设置任务名称、源目录、目标目录、执行频率等。

## 特性

- 使用 Vue 3 和 TypeScript 开发前端界面，提供了丰富的交互体验。
- 集成了 Tauri，一个安全、轻量级的框架，用于构建跨平台的桌面应用。
- 支持多种数据备份和恢复策略，包括但不限于文件、数据库等。
- 配置了 ESLint 和 Prettier，确保代码质量和风格一致性。
- 使用 Tailwind CSS 进行样式设计，快速开发美观的界面。
- 集成了 Vite 作为前端构建工具，提供快速的热重载支持。
- 使用 Vitest 进行单元测试，确保代码的稳定性。
- 配置了 GitHub Actions，自动化测试和构建流程。

## 安装

1. 确保已安装 [Tauri 的前置条件](https://tauri.studio/v1/guides/getting-started/prerequisites)。
2. 克隆仓库并安装依赖项（本模板默认使用 `pnpm`）：

```sh
pnpm i
```

## 使用

### 前端（TS, PnPM）

#### 运行开发服务器

使用以下命令同时启动前后端：

```sh
pnpm dev
```

#### 测试

运行以下命令进行测试：

```sh
pnpm test
```

### 后端（Rust, Cargo）

后端代码位于 [`src-tauri/`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fyiranzai%2Fwork%2Frust%2Ftauri-test%2Fsrc-tauri%2F%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "/Users/yiranzai/work/rust/tauri-test/src-tauri/") 目录（以下命令需在该目录下运行）。

#### 查找过时的 Rust 依赖

如果已安装 [cargo-outdated](https://github.com/kbknapp/cargo-outdated)：

```sh
cargo outdated
```

#### 升级 Rust 依赖

如果已安装 [cargo-edit](https://github.com/killercup/cargo-edit)：

```sh
cargo upgrade
```

## 构建和发布

### 构建

项目配置了 GitHub Actions，每次推送和 PR 都会自动测试和构建应用。手动构建：

```sh
pnpm build
```

### 发布新版本

1. 在 `package.json`、`src-tauri/Cargo.toml` 和 `src-tauri/tauri.conf.json` 中更新版本号。
2. 运行 `pnpm check` 更新 `Cargo.lock`。
3. 用 `vX.Y.Z` 标记要发布的提交。
4. 编辑发布说明并推送（包括标签）。
5. GitHub 工作流将自动为此版本构建一个新的草稿发布。准备好后发布 🎉。

## 贡献

如果您想为这个项目贡献代码，请查看 [贡献指南](TEMPLATE_README.md#Contributing)。

## 许可证

本项目采用 MIT 许可证。详情请见 [LICENSE](LICENSE) 文件。
