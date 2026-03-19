//! 雾凇拼音词库加载器
//!
//! 从 Rime 格式的 YAML 词库加载词条

use lingcode_core::candidate::Candidate;
use lingcode_core::error::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// 词库条目
#[derive(Debug, Clone)]
pub struct DictEntry {
    pub text: String,
    pub pinyin: String,
    pub weight: u32,
}

/// 雾凇拼音词库加载器
pub struct RimeDictLoader {
    entries: Vec<DictEntry>,
    pinyin_index: HashMap<String, Vec<DictEntry>>,
}

impl RimeDictLoader {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            pinyin_index: HashMap::new(),
        }
    }

    /// 从文件加载词库
    pub fn load_from_file(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut in_header = true;
        
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            
            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // 跳过 YAML 头部
            if in_header {
                if line == "..." {
                    in_header = false;
                }
                continue;
            }
            
            // 解析词条: 文字\t拼音\t权重
            if let Some(entry) = self.parse_entry(line) {
                let pinyin = entry.pinyin.clone();
                self.entries.push(entry.clone());
                self.pinyin_index
                    .entry(pinyin)
                    .or_insert_with(Vec::new)
                    .push(entry);
            }
        }
        
        // 对每个拼音的词条按权重排序
        for entries in self.pinyin_index.values_mut() {
            entries.sort_by(|a, b| b.weight.cmp(&a.weight));
        }
        
        Ok(())
    }

    /// 解析单行词条
    fn parse_entry(&self, line: &str) -> Option<DictEntry> {
        let parts: Vec<&str> = line.split('\t').collect();
        
        match parts.len() {
            2 => {
                // 文字\t拼音（无权重，默认100）
                Some(DictEntry {
                    text: parts[0].to_string(),
                    pinyin: parts[1].to_string(),
                    weight: 100,
                })
            }
            3 => {
                // 文字\t拼音\t权重
                let weight = parts[2].parse().unwrap_or(100);
                Some(DictEntry {
                    text: parts[0].to_string(),
                    pinyin: parts[1].to_string(),
                    weight,
                })
            }
            _ => None,
        }
    }

    /// 根据拼音查询词条
    pub fn lookup(&self, pinyin: &str) -> Vec<&DictEntry> {
        self.pinyin_index
            .get(pinyin)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// 获取所有词条
    pub fn entries(&self) -> &[DictEntry] {
        &self.entries
    }

    /// 获取词条数量
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 转换为 Candidate 列表
    pub fn to_candidates(&self, pinyin: &str) -> Vec<Candidate> {
        self.lookup(pinyin)
            .into_iter()
            .map(|entry| {
                Candidate::new(entry.text.clone())
                    .with_comment(entry.pinyin.clone())
                    .with_weight(entry.weight)
            })
            .collect()
    }
}

impl Default for RimeDictLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_entry() {
        let loader = RimeDictLoader::new();
        
        // 测试带权重的词条
        let entry = loader.parse_entry("中\tzhong\t100").unwrap();
        assert_eq!(entry.text, "中");
        assert_eq!(entry.pinyin, "zhong");
        assert_eq!(entry.weight, 100);
        
        // 测试不带权重的词条
        let entry = loader.parse_entry("文\twen").unwrap();
        assert_eq!(entry.text, "文");
        assert_eq!(entry.pinyin, "wen");
        assert_eq!(entry.weight, 100);
    }

    #[test]
    fn test_load_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# 测试词库").unwrap();
        writeln!(temp_file, "---").unwrap();
        writeln!(temp_file, "name: test").unwrap();
        writeln!(temp_file, "...").unwrap();
        writeln!(temp_file, "中\tzhong\t1000").unwrap();
        writeln!(temp_file, "种\tzhong\t900").unwrap();
        writeln!(temp_file, "文\twen\t800").unwrap();
        
        let mut loader = RimeDictLoader::new();
        loader.load_from_file(temp_file.path()).unwrap();
        
        assert_eq!(loader.len(), 3);
        
        let zhong_entries = loader.lookup("zhong");
        assert_eq!(zhong_entries.len(), 2);
        assert_eq!(zhong_entries[0].text, "中"); // 权重高的在前
        assert_eq!(zhong_entries[1].text, "种");
    }

    #[test]
    fn test_to_candidates() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "---").unwrap();
        writeln!(temp_file, "...").unwrap();
        writeln!(temp_file, "中\tzhong\t1000").unwrap();
        writeln!(temp_file, "种\tzhong\t900").unwrap();
        
        let mut loader = RimeDictLoader::new();
        loader.load_from_file(temp_file.path()).unwrap();
        
        let candidates = loader.to_candidates("zhong");
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].text, "中");
        assert_eq!(candidates[1].text, "种");
    }
}
