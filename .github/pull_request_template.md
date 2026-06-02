<!--
  提交 PR 前请逐项勾选确认。Please check all boxes before submitting.
  选择性填写，没有就不写。Fill in what's relevant; leave blank if not applicable.
-->

## Summary / 概要

<!-- 简要描述此 PR 做了什么。Briefly describe what this PR does. -->

## Related Issue / 关联 Issue

<!-- Fixes #123, Closes #456, or related to #789 -->

## Type of Change / 变更类型

<!-- 勾选相关项。Check relevant items. -->

- [ ] Bug fix / 修 bug
- [ ] New feature / 新功能
- [ ] UI/UX improvement / 界面与体验优化
- [ ] Localization / 本地化
- [ ] Performance / 性能优化
- [ ] Build, packaging, or tooling / 构建、打包或工具链
- [ ] Documentation / 文档
- [ ] Refactor / 重构
- [ ] Test / 测试

## Affected Platforms / 影响平台

<!-- 勾选受影响或已验证的平台。Check platforms affected or verified. -->

- [ ] Windows
- [ ] macOS
- [ ] Linux
- [ ] Platform-agnostic / 平台无关

## Scope / 改动范围

<!-- 勾选涉及的模块。Check modules touched. -->

- [ ] `src/components/` — UI 组件（MainWindow, SettingsPanel, BackgroundLayer 等）
- [ ] `src/features/markdown/` — Markdown 编辑与预览
- [ ] `src/features/notes/` — 笔记增删改查、分类、排序
- [ ] `src/features/settings/` — 配置、主题、快捷方式、便签颜色
- [ ] `src/features/windows/` — 窗口管理、磁贴、便签窗
- [ ] `src/features/importExport/` — .md 导入导出
- [ ] `src/locales/` — i18n 翻译文件（zh-CN, zh-HK, en-US）
- [ ] `src-tauri/src/lib.rs` — Tauri 命令（Rust 侧 API）
- [ ] `src-tauri/src/services/notes.rs` — 笔记持久化存储
- [ ] `src-tauri/src/locales.rs` — Rust 侧本地化
- [ ] `src-tauri/src/desktop.rs` — 桌面集成（托盘、开机启动）
- [ ] `src-tauri/tauri.conf.json` — Tauri 配置（窗口、打包、签名）
- [ ] CI / workflow / tooling — GitHub Actions、lint、fmt 等工具链

## Screenshots or Recordings / 截图或录屏

<!-- 前端变更请附截图或录屏。Attach screenshots or recordings for frontend changes. -->

## Testing / 测试

<!-- 提供复现和验证步骤。Provide steps to verify the change. -->

1.
2.
3.

## Checklist / 检查清单

### Code quality / 代码质量

- [ ] `npx oxfmt --check` 通过（前端格式化）
- [ ] `npm run lint` 通过（oxlint）
- [ ] `cargo fmt --check` 通过（Rust 格式化，在 `src-tauri/` 下执行）
- [ ] `cargo clippy -- -D warnings` 通过（Rust lint）
- [ ] 无新增 TypeScript 编译错误（`tsc`）
- [ ] 行尾统一为 LF（项目 `.gitattributes` 已配置）

### Functionality / 功能验证

- [ ] 已在本地构建并运行验证（`npm run tauri dev`）
- [ ] 窗口无装饰 + 透明背景正常显示
- [ ] 系统托盘与全局快捷键未受影响
- [ ] i18n 翻译已覆盖涉及的语言文案
- [ ] 如需变更文档，已同步更新对应 README 或 `Docs/` 中内容

### For bug fixes / Bug 修复

- [ ] 已描述根本原因
- [ ] 已验证修复解决该问题，且未引入回归

### For features / 功能开发

- [ ] 已考虑跨平台差异（Windows/macOS/Linux 行为一致性）

---

<!-- 感谢你对花笺 Floral Notepaper 的贡献！Thank you for your contribution! -->
