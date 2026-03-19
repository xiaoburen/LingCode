//! 双拼映射表定义
//!
//! 支持多种双拼方案：小鹤双拼、自然码、搜狗双拼

/// 双拼方案类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoublePinyinScheme {
    /// 小鹤双拼
    XiaoHe,
    /// 自然码
    ZiranMa,
    /// 搜狗双拼
    Sougou,
}

impl DoublePinyinScheme {
    /// 获取方案名称
    pub fn name(&self) -> &'static str {
        match self {
            DoublePinyinScheme::XiaoHe => "小鹤双拼",
            DoublePinyinScheme::ZiranMa => "自然码",
            DoublePinyinScheme::Sougou => "搜狗双拼",
        }
    }

    /// 获取声母映射表
    pub fn initials_map(&self) -> &[(char, &str)] {
        // 三种方案的声母映射基本相同
        &[
            ('b', "b"),
            ('c', "c"),
            ('d', "d"),
            ('f', "f"),
            ('g', "g"),
            ('h', "h"),
            ('j', "j"),
            ('k', "k"),
            ('l', "l"),
            ('m', "m"),
            ('n', "n"),
            ('p', "p"),
            ('q', "q"),
            ('r', "r"),
            ('s', "s"),
            ('t', "t"),
            ('w', "w"),
            ('x', "x"),
            ('y', "y"),
            ('z', "z"),
            // 零声母
            ('a', ""),
            ('e', ""),
            ('o', ""),
        ]
    }

    /// 获取韵母映射表
    pub fn finals_map(&self) -> &[(char, &str)] {
        match self {
            DoublePinyinScheme::XiaoHe => &XIAOHE_FINALS,
            DoublePinyinScheme::ZiranMa => &ZIRANMA_FINALS,
            DoublePinyinScheme::Sougou => &SOUGOU_FINALS,
        }
    }

    /// 双拼转全拼
    pub fn to_full_pinyin(&self, shuangpin: &str) -> Option<String> {
        if shuangpin.len() != 2 {
            return None;
        }

        let chars: Vec<char> = shuangpin.chars().collect();
        let first = chars[0];
        let second = chars[1];

        // 查找声母
        let initial = self.initials_map()
            .iter()
            .find(|(c, _)| *c == first)
            .map(|(_, p)| *p)?;

        // 查找韵母
        let final_ = self.finals_map()
            .iter()
            .find(|(c, _)| *c == second)
            .map(|(_, p)| *p)?;

        Some(format!("{}{}", initial, final_))
    }

    /// 检查是否是有效的双拼编码
    pub fn is_valid(&self, shuangpin: &str) -> bool {
        if shuangpin.len() != 2 {
            return false;
        }

        let chars: Vec<char> = shuangpin.chars().collect();
        let first = chars[0];
        let second = chars[1];

        let has_initial = self.initials_map().iter().any(|(c, _)| *c == first);
        let has_final = self.finals_map().iter().any(|(c, _)| *c == second);

        has_initial && has_final
    }

    /// 获取所有支持的声母键
    pub fn initial_keys(&self) -> Vec<char> {
        self.initials_map().iter().map(|(c, _)| *c).collect()
    }

    /// 获取所有支持的韵母键
    pub fn final_keys(&self) -> Vec<char> {
        self.finals_map().iter().map(|(c, _)| *c).collect()
    }
}

impl Default for DoublePinyinScheme {
    fn default() -> Self {
        DoublePinyinScheme::XiaoHe
    }
}

/// 小鹤双拼韵母映射表
const XIAOHE_FINALS: &[(char, &str)] = &[
    ('a', "a"),
    ('b', "iao"),
    ('c', "ao"),
    ('d', "iang"),
    ('e', "e"),
    ('f', "en"),
    ('g', "eng"),
    ('h', "ang"),
    ('i', "i"),
    ('j', "an"),
    ('k', "ing"),
    ('l', "ai"),
    ('m', "ian"),
    ('n', "in"),
    ('o', "o"),
    ('p', "ie"),
    ('q', "iu"),
    ('r', "uan"),
    ('s', "ong"),
    ('t', "ue"),
    ('u', "u"),
    ('v', "ui"),
    ('w', "ia"),
    ('x', "ua"),
    ('y', "un"),
    ('z', "ou"),
    // 特殊韵母
    (';', "ing"),
];

/// 自然码韵母映射表
const ZIRANMA_FINALS: &[(char, &str)] = &[
    ('a', "a"),
    ('b', "ou"),
    ('c', "iao"),
    ('d', "uang"),
    ('e', "e"),
    ('f', "en"),
    ('g', "eng"),
    ('h', "ang"),
    ('i', "i"),
    ('j', "an"),
    ('k', "ao"),
    ('l', "ai"),
    ('m', "ian"),
    ('n', "in"),
    ('o', "o"),
    ('p', "un"),
    ('q', "iu"),
    ('r', "uan"),
    ('s', "ong"),
    ('t', "ue"),
    ('u', "u"),
    ('v', "ui"),
    ('w', "ia"),
    ('x', "ua"),
    ('y', "ing"),
    ('z', "ei"),
    // 特殊韵母
    (';', "ing"),
];

/// 搜狗双拼韵母映射表（与自然码类似，略有不同）
const SOUGOU_FINALS: &[(char, &str)] = &[
    ('a', "a"),
    ('b', "ou"),
    ('c', "iao"),
    ('d', "uang"),
    ('e', "e"),
    ('f', "en"),
    ('g', "eng"),
    ('h', "ang"),
    ('i', "i"),
    ('j', "an"),
    ('k', "ao"),
    ('l', "ai"),
    ('m', "ian"),
    ('n', "in"),
    ('o', "o"),
    ('p', "un"),
    ('q', "iu"),
    ('r', "uan"),
    ('s', "ong"),
    ('t', "ue"),
    ('u', "u"),
    ('v', "ui"),
    ('w', "ia"),
    ('x', "ua"),
    ('y', "ing"),
    ('z', "ei"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xiaohe_conversion() {
        let scheme = DoublePinyinScheme::XiaoHe;
        
        // 测试常见转换
        assert_eq!(scheme.to_full_pinyin("nk"), Some("nin".to_string()));
        assert_eq!(scheme.to_full_pinyin("ul"), Some("shuai".to_string()));
        assert_eq!(scheme.to_full_pinyin("ju"), Some("ju".to_string()));
        assert_eq!(scheme.to_full_pinyin("qu"), Some("qu".to_string()));
        assert_eq!(scheme.to_full_pinyin("xu"), Some("xu".to_string()));
    }

    #[test]
    fn test_ziranma_conversion() {
        let scheme = DoublePinyinScheme::ZiranMa;
        
        assert_eq!(scheme.to_full_pinyin("nj"), Some("nin".to_string()));
        assert_eq!(scheme.to_full_pinyin("uk"), Some("shuai".to_string()));
    }

    #[test]
    fn test_valid_check() {
        let scheme = DoublePinyinScheme::XiaoHe;
        
        assert!(scheme.is_valid("nk"));
        assert!(scheme.is_valid("ul"));
        assert!(!scheme.is_valid("n"));
        assert!(!scheme.is_valid("nkk"));
        assert!(!scheme.is_valid("xx")); // xx 不是有效组合
    }

    #[test]
    fn test_scheme_name() {
        assert_eq!(DoublePinyinScheme::XiaoHe.name(), "小鹤双拼");
        assert_eq!(DoublePinyinScheme::ZiranMa.name(), "自然码");
        assert_eq!(DoublePinyinScheme::Sougou.name(), "搜狗双拼");
    }
}
