//! 拼音匹配器（stub）
pub struct PinyinMatcher {}
impl PinyinMatcher { pub fn new() -> Self { Self {} } }
impl Default for PinyinMatcher { fn default() -> Self { Self::new() } }
impl PinyinMatcher {
    pub fn match_pinyin(&self, _input: &str, _target: &str) -> f32 { 0.0 }
}