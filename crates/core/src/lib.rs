//! LingCode 核心库
//!
//! 提供输入法引擎的基础数据结构和接口定义
//! 支持多种输入方案（拼音、双拼等）和简繁转换

pub mod types;
pub mod candidate;
pub mod segment;
pub mod config;
pub mod error;
pub mod resource_loader;
pub mod schema_parser;

pub use types::*;
pub use candidate::*;
pub use segment::*;
pub use config::*;
pub use error::*;
pub use resource_loader::*;
pub use schema_parser::*;

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