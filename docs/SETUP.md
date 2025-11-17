# LingCode 环境搭建指南

本文档介绍如何在 macOS 上搭建 LingCode 开发环境。

## 前置要求

- macOS 10.15 或更高版本
- Homebrew 包管理器
- 至少 2GB 可用磁盘空间

## 1. 安装 Rust 工具链

### 方法一：使用官方安装脚本（推荐）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，重启终端或运行：
```bash
source $HOME/.cargo/env
```

### 方法二：使用 Homebrew

```bash
brew install rust
```

### 验证安装

```bash
rustc --version
cargo --version
```

## 2. 克隆项目

```bash
git clone <repository-url>
cd LingCode
```

## 3. 下载 Rime 资源文件

运行资源下载脚本：

```bash
./tools/download_resources.sh
```

这个脚本会自动从 Rime 官方仓库下载所需的资源文件，包括：
- 输入方案配置文件
- 词典文件
- OpenCC 简繁转换配置

## 4. 构建项目

```bash
cargo build --workspace
```

首次构建会下载所有依赖包，可能需要几分钟时间。

## 5. 运行测试

```bash
cargo test --workspace
```

## 6. 构建发布版本

```bash
cargo build --workspace --release
```

发布版本的二进制文件将位于 `target/release/` 目录。

## 常见问题

### Q: cargo 命令找不到

A: 确保已正确安装 Rust，并且 cargo 在 PATH 中。运行以下命令添加到 PATH：
```bash
source $HOME/.cargo/env
```

### Q: 编译错误：缺少某个依赖

A: 运行以下命令更新依赖：
```bash
cargo update
```

### Q: 下载资源文件失败

A: 检查网络连接，或手动从以下地址下载：
- https://github.com/rime/rime-prelude
- https://github.com/rime/rime-luna-pinyin
- https://github.com/rime/rime-double-pinyin

### Q: 在 macOS 上编译 C 依赖失败

A: 安装 Xcode Command Line Tools：
```bash
xcode-select --install
```

## 开发工具推荐

- **IDE**: Visual Studio Code + rust-analyzer 插件
- **调试**: LLDB (macOS 自带)
- **性能分析**: Instruments (Xcode 附带)

## 下一步

- 查看 [docs/RESOURCES.md](RESOURCES.md) 了解资源文件的详细信息
- 查看 [docs/ARCHITECTURE.md](ARCHITECTURE.md) 了解项目架构
- 查看示例代码 `examples/` 目录

## 获取帮助

如果遇到问题，请：
1. 查看本文档的常见问题部分
2. 搜索项目的 Issues
3. 创建新的 Issue 描述问题
