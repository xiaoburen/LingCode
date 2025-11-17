//! 配置管理

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 输入法配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 配置名称
    pub name: String,
    /// 配置版本
    pub version: String,
    /// 输入方案配置
    pub scheme_configs: HashMap<String, SchemeConfig>,
}

/// 单个输入方案配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemeConfig {
    /// 方案名称
    pub name: String,
    /// 方案描述
    pub description: Option<String>,
    /// 键盘布局
    pub keyboard_layout: Option<String>,
    /// 自定义选项
    pub options: HashMap<String, serde_json::Value>,
}

impl Config {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "1.0".to_string(),
            scheme_configs: HashMap::new(),
        }
    }

    pub fn add_scheme(&mut self, scheme_name: String, config: SchemeConfig) {
        self.scheme_configs.insert(scheme_name, config);
    }

    pub fn get_scheme(&self, scheme_name: &str) -> Option<&SchemeConfig> {
        self.scheme_configs.get(scheme_name)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new("default".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new("test".to_string());
        assert_eq!(config.name, "test");
        assert!(config.scheme_configs.is_empty());
    }
}
