//! LingCode 核心库
//!
//! 提供输入法引擎的基础数据结构和接口定义
//! 支持多种输入方案（拼音、双拼等）和简繁转换

pub mod types;
pub mod candidate;
pub mod segment;
pub mod config;
pub mod error;

pub use types::*;
pub use candidate::*;
pub use segment::*;
pub use config::*;
pub use error::*;

/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}

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

//! 候选词相关定义

use serde::{Deserialize, Serialize};

/// 候选词
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    /// 候选文字
    pub text: String,
    /// 注音（拼音等）
    pub comment: Option<String>,
    /// 权重/频率
    pub weight: u32,
    /// 候选词来源（词库名）
    pub source: Option<String>,
}

impl Candidate {
    pub fn new(text: String) -> Self {
        Self {
            text,
            comment: None,
            weight: 0,
            source: None,
        }
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
}

/// 候选词列表
#[derive(Debug, Clone, Default)]
pub struct Candidates {
    items: Vec<Candidate>,
}

impl Candidates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, candidate: Candidate) {
        self.items.push(candidate);
    }

    pub fn get(&self, index: usize) -> Option<&Candidate> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Candidate> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_builder() {
        let c = Candidate::new("中".to_string())
            .with_comment("zhong".to_string())
            .with_weight(100);
        
        assert_eq!(c.text, "中");
        assert_eq!(c.weight, 100);
    }

    #[test]
    fn test_candidates_collection() {
        let mut candidates = Candidates::new();
        candidates.add(Candidate::new("中".to_string()));
        candidates.add(Candidate::new("众".to_string()));
        
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates.get(0).unwrap().text, "中");
    }
}

//! 输入分段相关定义

use crate::candidate::Candidates;
use serde::{Deserialize, Serialize};

/// 输入段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// 拼音输入（原始）
    pub input: String,
    /// 已确认的文字
    pub confirmed: String,
    /// 候选词
    #[serde(skip)]
    pub candidates: Candidates,
    /// 当前选中的候选词索引
    pub selected_index: usize,
}

impl Segment {
    pub fn new(input: String) -> Self {
        Self {
            input,
            confirmed: String::new(),
            candidates: Candidates::new(),
            selected_index: 0,
        }
    }

    /// 获取当前选中的候选词
    pub fn current_candidate(&self) -> Option<&crate::candidate::Candidate> {
        self.candidates.get(self.selected_index)
    }

    /// 移至下一个候选词
    pub fn next_candidate(&mut self) {
        if self.selected_index + 1 < self.candidates.len() {
            self.selected_index += 1;
        }
    }

    /// 移至上一个候选词
    pub fn prev_candidate(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// 确认当前候选词
    pub fn commit(&mut self) {
        if let Some(candidate) = self.current_candidate() {
            self.confirmed = candidate.text.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate::Candidate;

    #[test]
    fn test_segment_creation() {
        let seg = Segment::new("zhong".to_string());
        assert_eq!(seg.input, "zhong");
        assert_eq!(seg.confirmed, "");
    }

    #[test]
    fn test_segment_navigation() {
        let mut seg = Segment::new("zhong".to_string());
        seg.candidates.add(Candidate::new("中".to_string()));
        seg.candidates.add(Candidate::new("众".to_string()));

        assert_eq!(seg.selected_index, 0);
        seg.next_candidate();
        assert_eq!(seg.selected_index, 1);
        seg.prev_candidate();
        assert_eq!(seg.selected_index, 0);
    }
}

//! 配置管理

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 输入法配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 配置名称
    pub name: String,
    /// 配置版本
    pub version: String,
    /// 输入方案配置
    pub scheme_configs: HashMap<String, SchemeConfig>,
}

/// 单个输入方案配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemeConfig {
    /// 方案名称
    pub name: String,
    /// 方案描述
    pub description: Option<String>,
    /// 键盘布局
    pub keyboard_layout: Option<String>,
    /// 自定义选项
    pub options: HashMap<String, serde_json::Value>,
}

impl Config {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "1.0".to_string(),
            scheme_configs: HashMap::new(),
        }
    }

    pub fn add_scheme(&mut self, scheme_name: String, config: SchemeConfig) {
        self.scheme_configs.insert(scheme_name, config);
    }

    pub fn get_scheme(&self, scheme_name: &str) -> Option<&SchemeConfig> {
        self.scheme_configs.get(scheme_name)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new("default".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new("test".to_string());
        assert_eq!(config.name, "test");
        assert!(config.scheme_configs.is_empty());
    }
}

//! 错误类型定义

use thiserror::Error;

/// LingCode 错误类型
#[derive(Debug, Error)]
pub enum LingCodeError {
    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Dictionary error: {0}")]
    DictError(String),

    #[error("Input error: {0}")]
    InputError(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// 结果类型
pub type Result<T> = std::result::Result<T, LingCodeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = LingCodeError::InputError("test".to_string());
        assert_eq!(err.to_string(), "Input error: test");
    }
}