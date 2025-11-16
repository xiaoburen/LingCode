//! 繁体拼音引擎（stub）
use crate::PinyinEngine;
use lingcode_core::{candidate::Candidates, error::Result, types::SchemeType};

pub struct TraditionalPinyinEngine {}
impl TraditionalPinyinEngine { pub fn new() -> Self { Self {} } }
impl Default for TraditionalPinyinEngine { fn default() -> Self { Self::new() } }

impl PinyinEngine for TraditionalPinyinEngine {
    fn scheme_type(&self) -> SchemeType { SchemeType::PinyinTraditional }
    fn get_candidates(&self, _pinyin: &str) -> Result<Candidates> { Ok(Candidates::new()) }
    fn is_valid_pinyin(&self, pinyin: &str) -> bool { !pinyin.is_empty() }
    fn get_pinyin_completion(&self, _p: &str) -> Result<Vec<String>> { Ok(vec![]) }
}