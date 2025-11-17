//! Resource loader module for loading schemas, dictionaries and other resources
//! This module handles loading resources from Rime-compatible formats

use std::path::{Path, PathBuf};
use std::fs;

/// Resource loader for managing input method resources
pub struct ResourceLoader {
    resource_dir: PathBuf,
}

impl ResourceLoader {
    /// Create a new resource loader with the specified resource directory
    pub fn new(resource_dir: impl AsRef<Path>) -> Self {
        Self {
            resource_dir: resource_dir.as_ref().to_path_buf(),
        }
    }

    /// Get the path to the schemas directory
    pub fn schemas_dir(&self) -> PathBuf {
        self.resource_dir.join("schemas")
    }

    /// Get the path to the dictionaries directory
    pub fn dicts_dir(&self) -> PathBuf {
        self.resource_dir.join("dicts")
    }

    /// Get the path to the OpenCC directory
    pub fn opencc_dir(&self) -> PathBuf {
        self.resource_dir.join("opencc")
    }

    /// Load a schema file by name
    pub fn load_schema(&self, schema_name: &str) -> Result<String, std::io::Error> {
        let schema_path = self.schemas_dir().join(format!("{}.schema.yaml", schema_name));
        fs::read_to_string(schema_path)
    }

    /// Load a dictionary file by name
    pub fn load_dict(&self, dict_name: &str) -> Result<String, std::io::Error> {
        let dict_path = self.dicts_dir().join(format!("{}.dict.yaml", dict_name));
        fs::read_to_string(dict_path)
    }

    /// List all available schemas
    pub fn list_schemas(&self) -> Result<Vec<String>, std::io::Error> {
        let mut schemas = Vec::new();
        if let Ok(entries) = fs::read_dir(self.schemas_dir()) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".schema.yaml") {
                        let schema_name = file_name.trim_end_matches(".schema.yaml").to_string();
                        schemas.push(schema_name);
                    }
                }
            }
        }
        Ok(schemas)
    }

    /// List all available dictionaries
    pub fn list_dicts(&self) -> Result<Vec<String>, std::io::Error> {
        let mut dicts = Vec::new();
        if let Ok(entries) = fs::read_dir(self.dicts_dir()) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".dict.yaml") {
                        let dict_name = file_name.trim_end_matches(".dict.yaml").to_string();
                        dicts.push(dict_name);
                    }
                }
            }
        }
        Ok(dicts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_loader_paths() {
        let loader = ResourceLoader::new("/path/to/resources");
        assert_eq!(loader.schemas_dir(), PathBuf::from("/path/to/resources/schemas"));
        assert_eq!(loader.dicts_dir(), PathBuf::from("/path/to/resources/dicts"));
        assert_eq!(loader.opencc_dir(), PathBuf::from("/path/to/resources/opencc"));
    }
}
