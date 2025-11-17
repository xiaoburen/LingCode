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
