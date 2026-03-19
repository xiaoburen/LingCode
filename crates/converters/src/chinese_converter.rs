//! 简繁转换模块
//! 
//! 提供简体中文和繁体中文之间的转换功能

/// 转换模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionMode {
    /// 简体转繁体（台湾标准）
    S2T,
    /// 简体转繁体（香港标准）
    S2HK,
    /// 简体转繁体（台湾标准，带常用词汇）
    S2TW,
    /// 繁体转简体
    T2S,
    /// 台湾繁体转简体
    TW2S,
    /// 香港繁体转简体
    HK2S,
}

impl ConversionMode {
    /// 获取 OpenCC 配置名称
    fn as_config(&self) -> &'static str {
        match self {
            ConversionMode::S2T => "s2t.json",
            ConversionMode::S2HK => "s2hk.json",
            ConversionMode::S2TW => "s2tw.json",
            ConversionMode::T2S => "t2s.json",
            ConversionMode::TW2S => "tw2s.json",
            ConversionMode::HK2S => "hk2s.json",
        }
    }
}

/// 简繁转换器
pub struct ChineseConverter {
    s2t: opencc::OpenCC,
    s2hk: opencc::OpenCC,
    s2tw: opencc::OpenCC,
    t2s: opencc::OpenCC,
    tw2s: opencc::OpenCC,
    hk2s: opencc::OpenCC,
}

impl ChineseConverter {
    /// 创建新的转换器
    pub fn new() -> Self {
        Self {
            s2t: opencc::OpenCC::new(ConversionMode::S2T.as_config()),
            s2hk: opencc::OpenCC::new(ConversionMode::S2HK.as_config()),
            s2tw: opencc::OpenCC::new(ConversionMode::S2TW.as_config()),
            t2s: opencc::OpenCC::new(ConversionMode::T2S.as_config()),
            tw2s: opencc::OpenCC::new(ConversionMode::TW2S.as_config()),
            hk2s: opencc::OpenCC::new(ConversionMode::HK2S.as_config()),
        }
    }

    /// 转换文本
    /// 
    /// # Arguments
    /// 
    /// * `text` - 要转换的文本
    /// * `mode` - 转换模式
    /// 
    /// # Returns
    /// 
    /// 转换后的文本
    pub fn convert(&self, text: &str, mode: ConversionMode) -> String {
        match mode {
            ConversionMode::S2T => self.s2t.convert(text),
            ConversionMode::S2HK => self.s2hk.convert(text),
            ConversionMode::S2TW => self.s2tw.convert(text),
            ConversionMode::T2S => self.t2s.convert(text),
            ConversionMode::TW2S => self.tw2s.convert(text),
            ConversionMode::HK2S => self.hk2s.convert(text),
        }
    }

    /// 简体转繁体（台湾标准）
    pub fn s2t(&self, text: &str) -> String {
        self.s2t.convert(text)
    }

    /// 繁体转简体
    pub fn t2s(&self, text: &str) -> String {
        self.t2s.convert(text)
    }
}

impl Default for ChineseConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s2t() {
        let converter = ChineseConverter::new();
        assert_eq!(converter.s2t("简体中文"), "簡體中文");
        assert_eq!(converter.s2t("鼠标"), "鼠標");
        assert_eq!(converter.s2t("软件"), "軟件");
    }

    #[test]
    fn test_t2s() {
        let converter = ChineseConverter::new();
        assert_eq!(converter.t2s("簡體中文"), "简体中文");
        assert_eq!(converter.t2s("鼠標"), "鼠标");
        assert_eq!(converter.t2s("軟件"), "软件");
    }

    #[test]
    fn test_roundtrip() {
        let converter = ChineseConverter::new();
        let original = "简体中文测试";
        let traditional = converter.s2t(original);
        let simplified = converter.t2s(&traditional);
        assert_eq!(simplified, original);
    }
}