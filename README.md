# LingCode 输入法

**LingCode ─ A fast, safe, and truly i18n-friendly RIME reimplementation in Rust.**

<img src="https://socialify.git.ci/你的用户名/lingcode/image?description=1&font=Inter&forks=1&issues=1&language=1&name=1&owner=1&pattern=Plus&pulls=1&stargazers=1&theme=Auto" alt="LingCode" width="100%"/>

> 一个代码库，五个平台，同一种丝滑的中文输入体验。  
> 简体、繁体、英文用户都平等被温柔以待。

## 支持平台（按字母顺序）

| 平台       | 状态       | 技术栈                    |
|------------|------------|---------------------------|
| Windows    | 🚀 开发中  | Tauri 2 + egui            |
| macOS      | 🚀 开发中  | Tauri 2 + egui + rime-rs  |
| Linux      | ⏳ 规划中  | Tauri 2 + egui            |
| iOS        | ⏳ 规划中  | Rust + Swift              |
| Android    | ⏳ 规划中  | Rust + Flutter            |


## 我们承诺

- 100% Rust 编写，内存安全，无段错误  
- 简体与繁体用户同等优先级，所有内置方案默认同时包含简繁词库  
- 方案热切换，无需重启  
- 完全开源 MIT，欢迎所有平台贡献者

## 快速开始（macOS & Windows & Linux）

```bash
git clone https://github.com/你的用户名/lingcode.git
cd lingcode
cargo tauri dev
