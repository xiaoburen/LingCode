//! LingCode 拼音模块 - 顶层入口，模块划分
pub mod simplified;
pub mod traditional;
pub mod dict;
pub mod matcher;

pub use simplified::SimplifiedPinyinEngine;
pub use traditional::TraditionalPinyinEngine;
pub use matcher::PinyinMatcher;
pub use dict::PinyinDict;

use lingcode_core::{
    candidate::Candidates,
    error::Result,
    types::SchemeType,
};

/// 拼音引擎特征
pub trait PinyinEngine {
    fn scheme_type(&self) -> SchemeType;
    fn get_candidates(&self, pinyin: &str) -> Result<Candidates>;
    fn is_valid_pinyin(&self, pinyin: &str) -> bool;
    fn get_pinyin_completion(&self, pinyin_prefix: &str) -> Result<Vec<String>>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_loads() { 
        assert!(true); 
    }
}
