<!--
 * @Author: yiranzai wuqingdzx@gmail.com
 * @Date: 2024-07-13 20:56:56
 * @LastEditors: yiranzai wuqingdzx@gmail.com
 * @LastEditTime: 2024-07-14 01:23:36
 * @FilePath: /BackupTool/CHANGES.md
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
# Changelog

## v0.0.4

- **更换存储组件**：从自己维护的json文件改为了官方[store插件](https://github.com/tauri-apps/tauri-plugin-store)

## v0.0.3

- **开机启动**: 增加了开机启动功能

## v0.0.2

- **处理流水线**: 处理优化流水线

## v0.0.1

- **目录复制**：异步复制源目录到目标目录，包括递归复制所有子目录和文件。复制过程中会根据当前时间创建新的目标子目录，以便组织和管理复制的内容。
- **JSON 数据管理**：允许用户将 JSON 数据保存到指定文件中，或从文件中加载 JSON 数据。这可以用于配置管理或数据持久化。
- **文件管理器集成**：提供在文件管理器中显示指定路径的功能，方便用户直接访问和查看文件或目录。
- **任务调度界面**：虽然具体的界面代码未在提供的代码段中展示，但从上下文中可以推断出，该应用提供了一个任务管理器界面，允许用户添加、配置和查看任务。这可能包括设置任务名称、源目录、目标目录、执行频率等。
- **项目初始化**：完成了项目的基础搭建，包括框架选择和环境配置。
- **前端改进**：使用 Vue 3 和 TypeScript 重构了前端界面，提升了用户交互体验。
- **样式更新**：引入 Tailwind CSS，优化了应用的整体样式和布局。
- **构建工具升级**：集成了 Vite 作为前端构建工具，显著提高了开发效率和热重载速度。
- **代码质量控制**：配置了 ESLint 和 Prettier，确保了代码风格的一致性和质量。
- **测试框架集成**：使用 Vitest 对项目进行单元测试，保障了代码的稳定性。
- **自动化流程**：通过 GitHub Actions 实现了自动化的测试和构建流程。
- **依赖管理**：提供了 Rust 依赖的查找和升级指令，帮助维护项目的依赖关系
