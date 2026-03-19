//! 双拼引擎实现
//!
//! 提供双拼到全拼的转换，支持多种双拼方案

use crate::mappings::DoublePinyinScheme;
use lingcode_core::{
    candidate::{Candidate, Candidates},
    error::Result,
    types::SchemeType,
};
use lingcode_pinyin::PinyinEngine;

/// 双拼引擎
pub struct DoublePinyinEngine {
    /// 使用的双拼方案
    scheme: DoublePinyinScheme,
    /// 底层拼音引擎（用于查询候选词）
    inner_engine: Box<dyn PinyinEngine>,
    /// 当前输入缓冲区
    input_buffer: String,
}

impl DoublePinyinEngine {
    /// 创建新的双拼引擎
    pub fn new(scheme: DoublePinyinScheme, inner_engine: Box<dyn PinyinEngine>) -> Self {
        Self {
            scheme,
            inner_engine,
            input_buffer: String::new(),
        }
    }

    /// 使用小鹤双拼方案
    pub fn with_xiaohe(inner_engine: Box<dyn PinyinEngine>) -> Self {
        Self::new(DoublePinyinScheme::XiaoHe, inner_engine)
    }

    /// 使用自然码方案
    pub fn with_ziranma(inner_engine: Box<dyn PinyinEngine>) -> Self {
        Self::new(DoublePinyinScheme::ZiranMa, inner_engine)
    }

    /// 使用搜狗双拼方案
    pub fn with_sougou(inner_engine: Box<dyn PinyinEngine>) -> Self {
        Self::new(DoublePinyinScheme::Sougou, inner_engine)
    }

    /// 获取当前方案
    pub fn scheme(&self) -> DoublePinyinScheme {
        self.scheme
    }

    /// 切换双拼方案
    pub fn set_scheme(&mut self, scheme: DoublePinyinScheme) {
        self.scheme = scheme;
    }

    /// 处理双拼输入
    /// 
    /// 将双拼编码转换为全拼，然后查询候选词
    pub fn process_shuangpin(&self, shuangpin: &str) -> Result<Candidates> {
        // 将双拼转换为全拼
        let full_pinyin = self.convert_to_full(shuangpin)?;
        
        // 使用底层引擎查询候选词
        self.inner_engine.get_candidates(&full_pinyin)
    }

    /// 将双拼字符串转换为全拼
    /// 
    /// 输入如 "nknk" 转换为 "ninnin"
    pub fn convert_to_full(&self, shuangpin: &str) -> Result<String> {
        let mut result = String::new();
        let chars: Vec<char> = shuangpin.chars().collect();
        
        // 每两个字符为一组进行转换
        let mut i = 0;
        while i + 1 < chars.len() {
            let pair = format!("{}{}", chars[i], chars[i + 1]);
            
            match self.scheme.to_full_pinyin(&pair) {
                Some(full) => {
                    result.push_str(&full);
                    i += 2;
                }
                None => {
                    // 如果转换失败，保留原字符
                    result.push(chars[i]);
                    i += 1;
                }
            }
        }
        
        // 处理剩余的单个字符
        if i < chars.len() {
            result.push(chars[i]);
        }
        
        Ok(result)
    }

    /// 检查是否是有效的双拼编码
    pub fn is_valid_shuangpin(&self, input: &str) -> bool {
        if input.len() % 2 != 0 {
            return false;
        }

        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        while i + 1 < chars.len() {
            let pair = format!("{}{}", chars[i], chars[i + 1]);
            if !self.scheme.is_valid(&pair) {
                return false;
            }
            i += 2;
        }

        true
    }

    /// 获取当前输入缓冲区
    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    /// 添加输入
    pub fn push_input(&mut self, ch: char) {
        self.input_buffer.push(ch);
    }

    /// 删除最后一个输入
    pub fn pop_input(&mut self) {
        self.input_buffer.pop();
    }

    /// 清空输入
    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
    }

    /// 获取双拼提示（用于显示当前输入对应的全拼）
    pub fn get_pinyin_hint(&self) -> Option<String> {
        if self.input_buffer.is_empty() {
            return None;
        }

        self.convert_to_full(&self.input_buffer).ok()
    }
}

impl PinyinEngine for DoublePinyinEngine {
    fn scheme_type(&self) -> SchemeType {
        match self.scheme {
            DoublePinyinScheme::XiaoHe => SchemeType::DoublePinyinXiaoHe,
            DoublePinyinScheme::ZiranMa => SchemeType::DoublePinyinZiranma,
            DoublePinyinScheme::Sougou => SchemeType::DoublePinyinSougou,
        }
    }

    fn get_candidates(&self, input: &str) -> Result<Candidates> {
        // 如果输入长度是偶数，尝试作为双拼处理
        if input.len() % 2 == 0 && self.is_valid_shuangpin(input) {
            self.process_shuangpin(input)
        } else {
            // 否则直接透传给底层引擎
            self.inner_engine.get_candidates(input)
        }
    }

    fn is_valid_pinyin(&self, input: &str) -> bool {
        // 检查是否是有效的双拼或全拼
        self.is_valid_shuangpin(input) || self.inner_engine.is_valid_pinyin(input)
    }

    fn get_pinyin_completion(&self, prefix: &str) -> Result<Vec<String>> {
        // 对于双拼，提供双拼键的补全提示
        if prefix.len() % 2 == 1 {
            // 奇数长度，提供韵母补全
            let last_char = prefix.chars().last().unwrap();
            let completions: Vec<String> = self.scheme.final_keys()
                .iter()
                .map(|final_char| format!("{}{}", prefix, final_char))
                .filter(|s| self.is_valid_shuangpin(s))
                .take(5)
                .collect();
            
            if !completions.is_empty() {
                return Ok(completions);
            }
        }
        
        // 默认使用底层引擎的补全
        self.inner_engine.get_pinyin_completion(prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lingcode_pinyin::SimplifiedPinyinEngine;

    #[test]
    fn test_double_pinyin_engine() {
        let inner = Box::new(SimplifiedPinyinEngine::new());
        let engine = DoublePinyinEngine::with_xiaohe(inner);
        
        assert_eq!(engine.scheme(), DoublePinyinScheme::XiaoHe);
    }

    #[test]
    fn test_convert_to_full() {
        let inner = Box::new(SimplifiedPinyinEngine::new());
        let engine = DoublePinyinEngine::with_xiaohe(inner);
        
        // nk = nin
        assert_eq!(engine.convert_to_full("nk").unwrap(), "nin");
        
        // ul = shuai
        assert_eq!(engine.convert_to_full("ul").unwrap(), "shuai");
        
        // nknk = ninnin
        assert_eq!(engine.convert_to_full("nknk").unwrap(), "ninnin");
    }

    #[test]
    fn test_valid_shuangpin() {
        let inner = Box::new(SimplifiedPinyinEngine::new());
        let engine = DoublePinyinEngine::with_xiaohe(inner);
        
        assert!(engine.is_valid_shuangpin("nk"));
        assert!(engine.is_valid_shuangpin("nknk"));
        assert!(!engine.is_valid_shuangpin("n"));
        assert!(!engine.is_valid_shuangpin("nkn"));
    }

    #[test]
    fn test_input_buffer() {
        let inner = Box::new(SimplifiedPinyinEngine::new());
        let mut engine = DoublePinyinEngine::with_xiaohe(inner);
        
        engine.push_input('n');
        engine.push_input('k');
        assert_eq!(engine.input_buffer(), "nk");
        
        let hint = engine.get_pinyin_hint();
        assert_eq!(hint, Some("nin".to_string()));
        
        engine.pop_input();
        assert_eq!(engine.input_buffer(), "n");
        
        engine.clear_input();
        assert_eq!(engine.input_buffer(), "");
    }
}
