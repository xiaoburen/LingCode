//! Schema parser for Rime-compatible YAML schema files
//! This module handles parsing of schema configuration files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Rime-compatible schema configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Schema metadata
    pub schema: SchemaInfo,
    
    /// Switches for various features
    #[serde(default)]
    pub switches: Vec<Switch>,
    
    /// Engine configuration
    #[serde(default)]
    pub engine: Engine,
    
    /// Speller configuration
    #[serde(default)]
    pub speller: Option<Speller>,
    
    /// Translator configuration
    #[serde(default)]
    pub translator: Option<Translator>,
    
    /// Additional custom fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// Schema metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    /// Schema ID
    pub schema_id: String,
    
    /// Schema name
    pub name: String,
    
    /// Schema version
    #[serde(default)]
    pub version: Option<String>,
    
    /// Schema author
    #[serde(default)]
    pub author: Option<String>,
    
    /// Schema description
    #[serde(default)]
    pub description: Option<String>,
    
    /// Dependencies
    #[serde(default)]
    pub dependencies: Vec<String>,
}

/// Feature switch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Switch {
    /// Switch name
    pub name: String,
    
    /// Reset state
    #[serde(default)]
    pub reset: u8,
    
    /// Available states
    #[serde(default)]
    pub states: Vec<String>,
}

/// Engine configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Engine {
    /// Processors
    #[serde(default)]
    pub processors: Vec<String>,
    
    /// Segmentors
    #[serde(default)]
    pub segmentors: Vec<String>,
    
    /// Translators
    #[serde(default)]
    pub translators: Vec<String>,
    
    /// Filters
    #[serde(default)]
    pub filters: Vec<String>,
}

/// Speller configuration for pinyin input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speller {
    /// Alphabet
    #[serde(default)]
    pub alphabet: Option<String>,
    
    /// Delimiter
    #[serde(default)]
    pub delimiter: Option<String>,
    
    /// Algebra rules
    #[serde(default)]
    pub algebra: Vec<String>,
}

/// Translator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translator {
    /// Dictionary name
    #[serde(default)]
    pub dictionary: Option<String>,
    
    /// Prism name
    #[serde(default)]
    pub prism: Option<String>,
}

impl Schema {
    /// Parse a schema from YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
    
    /// Convert schema to YAML string
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_schema() {
        let yaml = r#"
schema:
  schema_id: luna_pinyin
  name: 朙月拼音
  version: "0.1"
  author: "Test"
  description: "Test schema"
"#;
        let schema = Schema::from_yaml(yaml);
        assert!(schema.is_ok());
        let schema = schema.unwrap();
        assert_eq!(schema.schema.schema_id, "luna_pinyin");
        assert_eq!(schema.schema.name, "朙月拼音");
    }
}
