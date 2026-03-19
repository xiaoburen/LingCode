//! LingCode 双拼模块
//!
//! 提供双拼输入支持，包括多种双拼方案的映射和转换

pub mod mappings;
pub mod engine;
pub mod schemas;

pub use mappings::DoublePinyinScheme;
pub use engine::DoublePinyinEngine;

/// 创建小鹤双拼引擎的便捷函数
pub fn xiaohe_engine(inner: Box<dyn lingcode_pinyin::PinyinEngine>) -> DoublePinyinEngine {
    DoublePinyinEngine::with_xiaohe(inner)
}

/// 创建自然码引擎的便捷函数
pub fn ziranma_engine(inner: Box<dyn lingcode_pinyin::PinyinEngine>) -> DoublePinyinEngine {
    DoublePinyinEngine::with_ziranma(inner)
}

/// 创建搜狗双拼引擎的便捷函数
pub fn sougou_engine(inner: Box<dyn lingcode_pinyin::PinyinEngine>) -> DoublePinyinEngine {
    DoublePinyinEngine::with_sougou(inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loads() {
        // 测试模块能正常加载
        assert_eq!(DoublePinyinScheme::XiaoHe.name(), "小鹤双拼");
    }
}
