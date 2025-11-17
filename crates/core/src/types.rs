//! 基础类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 输入方案类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemeType {
    /// 全拼（简体）
    PinyinSimplified,
    /// 全拼（繁体）
    PinyinTraditional,
    /// 双拼
    DoublePinyin,
}

impl std::fmt::Display for SchemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemeType::PinyinSimplified => write!(f, "pinyin_simp"),
            SchemeType::PinyinTraditional => write!(f, "pinyin_trad"),
            SchemeType::DoublePinyin => write!(f, "double_pinyin"),
        }
    }
}

/// 汉字转换类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConvertType {
    /// 简体
    Simplified,
    /// 繁体
    Traditional,
}

/// 双拼方案
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoublePinyinScheme {
    /// 小鹤双拼
    XiaoHe,
    /// 搜狗双拼
    Sogou,
    /// 自然码
    Ziran,
    /// 标准双拼
    Standard,
}

/// 输入法状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputState {
    /// 空闲状态
    Idle,
    /// 编辑中
    Composing,
    /// 选择候选词
    Selecting,
}

/// 按键事件
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// 按键码
    pub keycode: u32,
    /// 按键符号
    pub key: char,
    /// 是否是修饰键组合
    pub modifiers: KeyModifiers,
}

/// 修饰键标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyModifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    pub fn with_ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }
}

/// 配置选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    /// 输入方案
    pub scheme: SchemeType,
    /// 繁简转换
    pub convert_type: ConvertType,
    /// 候选词数量限制
    pub max_candidates: usize,
    /// 自定义配置
    pub custom: HashMap<String, String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            scheme: SchemeType::PinyinSimplified,
            convert_type: ConvertType::Simplified,
            max_candidates: 10,
            custom: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme_display() {
        assert_eq!(SchemeType::PinyinSimplified.to_string(), "pinyin_simp");
    }

    #[test]
    fn test_key_modifiers() {
        let mods = KeyModifiers::new().with_ctrl().with_shift();
        assert!(mods.ctrl && mods.shift);
    }
}
