//! 拼音字典管理（stub）
use lingcode_core::error::Result;
pub struct PinyinDict {}
impl PinyinDict { pub fn new() -> Self { Self {} } }
impl Default for PinyinDict { fn default() -> Self { Self::new() } }
impl PinyinDict {
    pub fn load_from_file(&mut self, _path: &str) -> Result<()> { Ok(()) }
}