# 灵码输入法 - 开发日志

## 当前状态
- **阶段**: Phase 3 前端实现 - 已完成并暂停
- **开始时间**: 2026-03-19 15:02
- **最后更新**: 2026-03-19 22:00
- **状态**: ⏸️ 已暂停

## 马拉松任务完成清单

### ✅ Phase 1 - 核心基础（全部完成）
- [x] 核心数据类型（InputState、KeyEvent、Candidate）
- [x] 引擎状态机（Idle → Composing → Selecting）
- [x] 基础拼音匹配器（全拼转汉字）
- [x] CLI demo 能正常输入中文
- [x] 核心库单元测试

### ✅ Phase 2 - 功能完善（全部完成）
- [x] 多词库加载（8105/base/ext/tencent）
- [x] 双拼映射（小鹤/自然码/搜狗）
- [x] 词频学习（UserDict + 30天半衰期时间衰减）
- [x] Rime YAML 方案解析
- [x] 简繁转换（OpenCC 集成，支持6种模式）

### ✅ Phase 3 - 跨平台前端（macOS 完成）
- [x] FFI 绑定（C API）
- [x] macOS 输入法框架（InputMethodKit）
- [x] 候选词窗口 UI（圆角、阴影、选中高亮）
- [x] 连接真实 Rust 引擎（通过 FFI）
- [x] 按键处理（字母/数字/空格/回车/退格/上下箭头）
- [x] 安装脚本（install.sh）

### 📋 待完成任务（暂停）
- [ ] Windows 前端（TSF）
- [ ] Linux 前端（IBus/Fcitx）
- [ ] 用户词库持久化（macOS 端保存）
- [ ] 设置面板

---

## 关键文件位置

```
LingCode/
├── crates/
│   ├── core/          # 核心类型
│   ├── engine/        # 状态机
│   ├── pinyin/        # 拼音引擎（含词频学习）
│   ├── double-pinyin/ # 双拼支持
│   ├── dict/          # 词库管理
│   ├── converters/    # 简繁转换（OpenCC）
│   └── ffi/           # FFI 绑定（C API）
│       └── include/lingcode.h  # C 头文件
├── frontends/
│   └── desktop/macos/ # macOS 输入法（已完成）
│       └── LingCodeIME/
├── README.md          # 项目文档
└── CHANGELOG.md       # 本文件
```

## GitHub 仓库
https://github.com/xiaoburen/LingCode

## 暂停说明
项目已达到可用状态：
- Rust 核心库完整（拼音/双拼/词频/简繁转换）
- CLI demo 可正常使用
- macOS 输入法前端框架完成，支持真实 Rust 引擎

暂停原因：需要用户测试和反馈后再决定后续方向。

## 恢复开发时的建议
1. 完善 macOS 输入法（用户词库持久化、设置面板）
2. 实现 Windows 前端（TSF）
3. 实现 Linux 前端（IBus/Fcitx）
4. 添加更多输入方案（五笔、笔画等）
