<div align="center">

<img src="./assets/logo.svg" width="120" height="120" alt="灵码输入法 Logo">

# 灵码输入法 LingCode

**基于 Rust 的 Rime 兼容输入法框架**

[<img src="https://img.shields.io/badge/English-README-blue?style=flat-square">](./README.md)
[<img src="https://img.shields.io/badge/中文-README-red?style=flat-square">](./README_CN.md)

[<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/License-Apache%202.0-green?style=for-the-badge">](./LICENSE)
[<img src="https://img.shields.io/badge/Rime-Compatible-orange?style=for-the-badge">](https://rime.im/)

</div>

---

## ✨ 特性

| 功能 | 状态 | 说明 |
|------|------|------|
| 🎯 **拼音输入** | ✅ 完成 | 简体/繁体拼音 |
| ⌨️ **双拼支持** | 🚧 进行中 | 自然码、小鹤、微软双拼 |
| 💻 **多平台** | 🚧 进行中 | macOS、Windows、Linux、iOS、Android |
| 📚 **Rime 兼容** | ✅ 完成 | 使用 Rime 方案与词库 |
| 🧠 **智能学习** | 📋 计划中 | 词频学习 |
| 🔄 **简繁转换** | 📋 计划中 | OpenCC 简繁转换 |

---

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
```

---

## 📁 项目结构

```
LingCode/
├── crates/
│   ├── core/          # 核心类型定义
│   ├── engine/        # 输入法引擎（状态机）
│   ├── pinyin/        # 拼音引擎
│   ├── double-pinyin/ # 双拼支持
│   ├── dict/          # 词典管理（雾凇拼音）
│   ├── converters/    # OpenCC 集成
│   ├── ffi/           # FFI 绑定
│   └── cli/           # 命令行演示
├── frontends/
│   └── desktop/
│       ├── macos/     # macOS 输入法
│       ├── windows/   # Windows 输入法
│       └── linux/     # Linux 输入法
└── resources/
    └── dict/          # 词典文件（雾凇拼音）
```

---

## 🛠️ 开发状态

### 第一阶段：核心基础 ✅
- [x] 核心类型定义
- [x] 引擎状态机（Idle→Composing→Selecting）
- [x] 简体拼音引擎
- [x] CLI 演示程序

### 第二阶段：功能增强 🚧
- [x] 加载雾凇拼音词库（8105 字表）
- [ ] 加载完整雾凇拼音词库
- [ ] 双拼支持
- [ ] 词频学习

### 第三阶段：前端开发 📋
- [ ] macOS 输入法
- [ ] Windows 输入法
- [ ] Linux 输入法

---

## 📚 词库

灵码输入法使用 **[雾凇拼音](https://github.com/iDvel/rime-ice)** 作为默认词库：

- **8105.dict.yaml** - 8105 通用规范汉字表
- **base.dict.yaml** - 基础词库
- **ext.dict.yaml** - 扩展词库
- **tencent.dict.yaml** - 腾讯词库

词库位置：`~/Library/Rime/cn_dicts/`（macOS）

---

## 🤝 贡献

欢迎提交 [Issue](https://github.com/xiaoburen/LingCode/issues) 和 [Pull Request](https://github.com/xiaoburen/LingCode/pulls)！

---

## 🙏 致谢

本项目受以下项目启发并与其兼容：

- [Rime 输入法](https://rime.im/) - 最棒的输入法框架
- [雾凇拼音](https://github.com/iDvel/rime-ice) - 一个全面的 Rime 词库

本项目使用 [OpenClaw](https://github.com/openclaw/openclaw) 开发 - 一个用于长期任务管理的个人 AI 助手框架。

---

## 📄 许可证

基于 [Apache License 2.0](./LICENSE) 开源。

---

<div align="center">

<img src="./assets/logo.svg" width="40" height="40" alt="灵码输入法 Logo">

**灵码输入法 - 让中文输入更流畅**

[<img src="https://img.shields.io/github/stars/xiaoburen/LingCode?style=social">](https://github.com/xiaoburen/LingCode)
[<img src="https://img.shields.io/github/forks/xiaoburen/LingCode?style=social">](https://github.com/xiaoburen/LingCode)

</div>
