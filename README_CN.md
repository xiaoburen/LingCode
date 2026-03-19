# 灵码输入法 (LingCode)

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

> 一个基于 Rust 的 Rime 兼容输入法框架，支持多平台、多方案。

## ✨ 特性

- 🦀 **纯 Rust 实现** - 高性能、内存安全
- 🔌 **Rime 兼容** - 支持 Rime 方案、词典、OpenCC 转换
- 🎯 **多方案支持** - 全拼、双拼（自然码、小鹤、微软等）
- 💻 **多平台** - macOS、Windows、Linux、Android、iOS
- 🧩 **模块化设计** - 核心、引擎、前端分离，易于扩展

## 🚀 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/xiaoburen/LingCode.git
cd LingCode

# 构建 CLI 演示程序
cargo build -p lingcode-cli --release

# 运行
./target/release/lingcode
```

### 使用演示

```
╔══════════════════════════════════════════╗
║       📝 灵码输入法 CLI Demo v0.1        ║
╠══════════════════════════════════════════╣
║  输入拼音，按空格或数字选择候选词         ║
║  Backspace: 删除  |  Esc: 取消           ║
╚══════════════════════════════════════════╝

输入: zhongwen
候选: [1. 中文] 2. 中问 3. 中闻
按 1 提交: 中文

输入: nihao
候选: [1. 你好] 2. 您好 3. 尼好
按 1 提交: 你好
```

## 📁 项目结构

```
LingCode/
├── crates/
│   ├── core/          # 核心类型定义
│   ├── engine/        # 输入法引擎（状态机）
│   ├── pinyin/        # 拼音引擎
│   ├── double-pinyin/ # 双拼支持
│   ├── dict/          # 词典管理
│   ├── converters/    # 简繁转换（OpenCC）
│   ├── ffi/           # FFI 绑定
│   └── cli/           # 命令行演示
├── frontends/
│   └── desktop/
│       ├── macos/     # macOS 输入法
│       ├── windows/   # Windows 输入法
│       └── linux/     # Linux 输入法
└── resources/
    └── dict/          # 词典文件
```

## 🛠️ 开发状态

| 组件 | 状态 | 说明 |
|------|------|------|
| Core 类型 | ✅ 完成 | SchemeType、KeyEvent、Candidate 等 |
| Engine | ✅ 完成 | Idle→Composing→Selecting 状态机 |
| 拼音引擎 | ✅ 完成 | 简体拼音，基础词典 |
| CLI Demo | ✅ 完成 | 可运行演示 |
| 双拼支持 | 🚧 进行中 | 自然码、小鹤方案 |
| 词典系统 | 🚧 进行中 | 雾凇拼音词库集成 |
| macOS 前端 | 📋 计划 | InputMethodKit 绑定 |
| Windows 前端 | 📋 计划 | TSF 实现 |

## 📚 词库

灵码输入法使用**雾凇拼音**作为默认词库：

- 海量词条，精准输入
- 定期更新，与时俱进
- 支持自定义扩展

词库位置：`resources/dict/rime-ice/`

## 🤝 贡献

欢迎提交 Issue 和 PR！

## 📄 许可证

Apache License 2.0

---

*灵码输入法 - 让中文输入更流畅*
