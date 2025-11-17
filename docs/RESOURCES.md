# 资源使用说明

LingCode 项目使用来自 Rime 开源社区的资源文件，包括输入方案、词典和配置文件。

## 资源来源

### 1. rime-prelude
- **项目地址**: https://github.com/rime/rime-prelude
- **用途**: 基础配置和默认设置
- **许可证**: LGPL v3
- **包含内容**:
  - 默认配置文件
  - 基础符号表
  - 标点符号映射

### 2. rime-luna-pinyin
- **项目地址**: https://github.com/rime/rime-luna-pinyin
- **用途**: 朙月拼音词典和方案
- **许可证**: LGPL v3
- **包含内容**:
  - luna_pinyin.schema.yaml - 全拼方案配置
  - luna_pinyin.dict.yaml - 基础词典
  - luna_pinyin.extended.dict.yaml - 扩展词典

### 3. rime-double-pinyin
- **项目地址**: https://github.com/rime/rime-double-pinyin
- **用途**: 双拼输入方案
- **许可证**: LGPL v3
- **包含内容**:
  - double_pinyin.schema.yaml - 自然码双拼
  - double_pinyin_flypy.schema.yaml - 小鹤双拼
  - double_pinyin_mspy.schema.yaml - 微软双拼

### 4. rime-opencc
- **项目地址**: https://github.com/rime/rime-opencc
- **用途**: OpenCC 简繁转换数据
- **许可证**: Apache License 2.0
- **包含内容**:
  - 简繁转换配置文件
  - 简繁转换词典

## 使用方式

### 1. 下载资源文件

```bash
# 进入资源目录
cd resources

# 克隆或下载所需的资源仓库
git clone https://github.com/rime/rime-prelude.git
git clone https://github.com/rime/rime-luna-pinyin.git
git clone https://github.com/rime/rime-double-pinyin.git
git clone https://github.com/rime/rime-opencc.git

# 将所需文件复制到对应目录
cp rime-prelude/*.yaml schemas/
cp rime-luna-pinyin/*.yaml schemas/
cp rime-luna-pinyin/*.yaml dicts/
cp rime-double-pinyin/*.yaml schemas/
cp -r rime-opencc/* opencc/
```

### 2. 在代码中加载资源

```rust
use lingcode_core::ResourceLoader;

// 创建资源加载器
let loader = ResourceLoader::new("resources");

// 加载方案
let schema = loader.load_schema("luna_pinyin")?;

// 列出所有可用方案
let schemas = loader.list_schemas()?;
```

### 3. 解析方案配置

```rust
use lingcode_core::Schema;

// 解析 YAML 方案
let schema = Schema::from_yaml(&yaml_content)?;
println!("Schema: {}", schema.schema.name);
```

## 目录结构

```
resources/
├── schemas/          # 输入方案配置文件
│   ├── luna_pinyin.schema.yaml
│   ├── double_pinyin.schema.yaml
│   └── ...
├── dicts/            # 词典文件
│   ├── luna_pinyin.dict.yaml
│   └── ...
└── opencc/           # OpenCC 简繁转换配置
    ├── s2t.json
    ├── t2s.json
    └── ...
```

## 许可协议说明

所有从 Rime 项目引用的资源文件均遵循其原始许可协议：
- **rime-prelude**: LGPL v3
- **rime-luna-pinyin**: LGPL v3
- **rime-double-pinyin**: LGPL v3
- **rime-opencc**: Apache License 2.0

在使用这些资源时，请确保遵守相应的许可证要求。

## 贡献资源

如果您想为 LingCode 贡献新的输入方案或词典资源，请：

1. 确保资源符合 Rime 的配置格式
2. 在 `resources/` 目录下添加相应文件
3. 更新本文档，说明资源来源和用途
4. 确保遵守原始资源的许可证要求

## 参考资料

- [Rime 官方文档](https://github.com/rime/home/wiki)
- [Rime 输入方案设计书](https://github.com/rime/home/wiki/RimeWithSchemata)
- [Rime 定制指南](https://github.com/rime/home/wiki/CustomizationGuide)
