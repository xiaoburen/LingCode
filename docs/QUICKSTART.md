# LingCode 快速开始

快速上手 LingCode 开发的简明指南。

## 5 分钟快速开始

### 1. 安装 Rust（如果还没安装）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. 下载并设置项目

```bash
# 进入项目目录
cd LingCode

# 下载 Rime 资源
./tools/download_resources.sh

# 构建项目
cargo build --workspace
```

### 3. 运行测试

```bash
cargo test --workspace
```

## 使用示例

### 加载输入方案

```rust
use lingcode_core::{ResourceLoader, Schema};

// 创建资源加载器
let loader = ResourceLoader::new("resources");

// 加载双拼方案
let schema_yaml = loader.load_schema("double_pinyin_sample")?;
let schema = Schema::from_yaml(&schema_yaml)?;

println!("方案名称: {}", schema.schema.name);
println!("方案 ID: {}", schema.schema.schema_id);
```

### 列出可用方案

```rust
use lingcode_core::ResourceLoader;

let loader = ResourceLoader::new("resources");
let schemas = loader.list_schemas()?;

for schema_name in schemas {
    println!("- {}", schema_name);
}
```

### 加载词典

```rust
use lingcode_core::ResourceLoader;

let loader = ResourceLoader::new("resources");
let dict_content = loader.load_dict("luna_pinyin")?;

// 解析词典内容...
```

## 项目结构速览

```
LingCode/
├── crates/           # Rust 库
│   ├── core/         # 核心功能（资源加载、方案解析等）
│   ├── engine/       # 输入引擎
│   ├── pinyin/       # 拼音处理
│   └── ...
├── resources/        # Rime 资源文件
│   ├── schemas/      # 输入方案
│   ├── dicts/        # 词典
│   └── opencc/       # 简繁转换
├── tools/            # 工具脚本
└── docs/             # 文档

```

## 常用命令

```bash
# 构建所有组件
cargo build --workspace

# 构建发布版本
cargo build --workspace --release

# 运行测试
cargo test --workspace

# 运行特定测试
cargo test -p lingcode-core

# 检查代码格式
cargo fmt --check

# 运行 linter
cargo clippy --workspace

# 生成文档
cargo doc --workspace --open

# 清理构建产物
cargo clean
```

## 下一步

- 📖 详细设置指南: [docs/SETUP.md](SETUP.md)
- 🔧 资源使用说明: [docs/RESOURCES.md](RESOURCES.md)
- 🏗️ 项目架构: [docs/ARCHITECTURE.md](ARCHITECTURE.md)
- 💡 示例代码: [examples/](../examples/)

## 开发工作流

1. **创建功能分支**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **进行开发**
   - 编写代码
   - 添加测试
   - 运行测试确保通过

3. **提交代码**
   ```bash
   cargo fmt
   cargo clippy --workspace
   cargo test --workspace
   git add .
   git commit -m "描述你的改动"
   ```

4. **推送并创建 Pull Request**
   ```bash
   git push origin feature/my-feature
   ```

## 获取帮助

遇到问题？
- 📚 查看完整文档 [docs/](.)
- 🐛 报告问题 [Issues](../../issues)
- 💬 加入讨论 [Discussions](../../discussions)
