# 灵码输入法 - 开发日志

## 当前状态
- **阶段**: Phase 3 前端实现
- **开始时间**: 2026-03-19 15:02
- **最后更新**: 2026-03-19 21:40

## 马拉松任务清单

### ✅ 已完成

**Phase 1 - 核心基础**
- [x] 核心数据类型（InputState、KeyEvent、Candidate）
- [x] 引擎状态机（Idle → Composing → Selecting）
- [x] 基础拼音匹配器（全拼转汉字）
- [x] CLI demo 能正常输入中文
- [x] 核心库单元测试

**Phase 2 - 功能完善**
- [x] 多词库加载（8105/base/ext/tencent）
- [x] 双拼映射（小鹤/自然码/搜狗）
- [x] 词频学习（UserDict + 30天半衰期）
- [x] Rime YAML 方案解析
- [x] **简繁转换（OpenCC 集成）**

**Phase 3 - macOS 前端（部分）**
- [x] 基础输入法框架（InputMethodKit）
- [x] 候选词窗口 UI（圆角、阴影、选中高亮）
- [x] 按键处理（字母/数字/空格/回车/退格/上下箭头）
- [x] 安装脚本

---

### 📋 待完成任务

**Phase 3 macOS 前端**
- [ ] FFI 绑定（Rust ↔ Swift）
- [ ] 连接真实 Rust 引擎（替换 mock 数据）
- [ ] 用户词库持久化（macOS 端保存）
- [ ] 设置面板

**Phase 3 其他平台**
- [ ] Windows 前端（TSF）
- [ ] Linux 前端（IBus/Fcitx）

**其他**
- [ ] 更新 README 进度表
- [ ] 集成测试

---

## 下一步任务（按优先级）

1. **FFI 绑定** - 让 macOS 前端连接 Rust 引擎
2. **连接真实引擎** - macOS 前端调用 Rust 拼音引擎
3. **用户词库持久化** - macOS 端保存学习数据
4. **Windows 前端** - 跨平台支持

## 关键决策
- 使用 OpenCC 进行简繁转换 ✅
- FFI 使用 cbindgen + Swift 桥接

## 参考资料
- Anthropic 论文: Long-Running Claude for Scientific Research
