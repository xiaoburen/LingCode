//! 雾凇拼音词库加载器
//!
//! 从 Rime 格式的 YAML 词库加载词条，支持多词库合并

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

/// 词库来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DictSource {
    Base,      // 基础词库 8105
    Ext,       // 扩展词库
    Tencent,   // 腾讯词库
    Custom,    // 自定义词库
}

impl DictSource {
    pub fn priority(&self) -> u32 {
        match self {
            DictSource::Base => 100,
            DictSource::Ext => 80,
            DictSource::Tencent => 60,
            DictSource::Custom => 40,
        }
    }
}

/// 带来源的词库条目
#[derive(Debug, Clone)]
pub struct SourcedEntry {
    pub entry: DictEntry,
    pub source: DictSource,
}

/// 雾凇拼音词库加载器（支持多词库）
pub struct RimeDictLoader {
    entries: Vec<SourcedEntry>,
    pinyin_index: HashMap<String, Vec<SourcedEntry>>,
    loaded_sources: Vec<DictSource>,
}

impl RimeDictLoader {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            pinyin_index: HashMap::new(),
            loaded_sources: Vec::new(),
        }
    }

    /// 从文件加载词库（指定来源）
    pub fn load_from_file_with_source(&mut self, path: &Path, source: DictSource) -> Result<usize> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut in_header = true;
        let mut loaded_count = 0;
        
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
                let sourced = SourcedEntry { entry, source };
                self.entries.push(sourced.clone());
                self.pinyin_index
                    .entry(pinyin)
                    .or_insert_with(Vec::new)
                    .push(sourced);
                loaded_count += 1;
            }
        }
        
        // 对每个拼音的词条按权重排序
        for entries in self.pinyin_index.values_mut() {
            entries.sort_by(|a, b| {
                // 先按来源优先级排序，再按权重排序
                let priority_cmp = b.source.priority().cmp(&a.source.priority());
                if priority_cmp != std::cmp::Ordering::Equal {
                    priority_cmp
                } else {
                    b.entry.weight.cmp(&a.entry.weight)
                }
            });
        }
        
        if loaded_count > 0 {
            self.loaded_sources.push(source);
        }
        
        Ok(loaded_count)
    }

    /// 从文件加载词库（默认 Custom 来源）
    pub fn load_from_file(&mut self, path: &Path) -> Result<usize> {
        self.load_from_file_with_source(path, DictSource::Custom)
    }

    /// 加载多个雾凇拼音词库文件
    /// 
    /// 标准加载顺序：
    /// 1. 8105.dict.yaml - 基础词库
    /// 2. base.dict.yaml - 基础扩展
    /// 3. ext.dict.yaml - 扩展词库
    /// 4. tencent.dict.yaml - 腾讯词库
    pub fn load_rime_ice_dicts(&mut self, rime_dict_dir: &Path) -> Result<DictLoadSummary> {
        let mut summary = DictLoadSummary::new();
        
        // 定义要加载的词库文件及其来源
        let dict_files: Vec<(&str, DictSource)> = vec![
            ("8105.dict.yaml", DictSource::Base),
            ("base.dict.yaml", DictSource::Ext),
            ("ext.dict.yaml", DictSource::Ext),
            ("tencent.dict.yaml", DictSource::Tencent),
        ];
        
        for (filename, source) in dict_files {
            let path = rime_dict_dir.join(filename);
            if path.exists() {
                match self.load_from_file_with_source(&path, source) {
                    Ok(count) => {
                        log::info!("已加载 {}: {} 条词条", filename, count);
                        summary.add_loaded(filename, count, source);
                    }
                    Err(e) => {
                        log::warn!("加载 {} 失败: {}", filename, e);
                        summary.add_failed(filename, e.to_string());
                    }
                }
            } else {
                log::debug!("词库文件不存在: {}", path.display());
                summary.add_missing(filename);
            }
        }
        
        Ok(summary)
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
            .map(|v| v.iter().map(|s| &s.entry).collect())
            .unwrap_or_default()
    }

    /// 获取所有词条
    pub fn entries(&self) -> &[SourcedEntry] {
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

    /// 获取已加载的词库来源
    pub fn loaded_sources(&self) -> &[DictSource] {
        &self.loaded_sources
    }

    /// 获取统计信息
    pub fn stats(&self) -> DictStats {
        let mut by_source: HashMap<DictSource, usize> = HashMap::new();
        for entry in &self.entries {
            *by_source.entry(entry.source).or_insert(0) += 1;
        }
        
        DictStats {
            total_entries: self.entries.len(),
            unique_pinyin: self.pinyin_index.len(),
            by_source,
        }
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

/// 词库加载摘要
#[derive(Debug, Clone)]
pub struct DictLoadSummary {
    pub loaded: Vec<(String, usize, DictSource)>,
    pub failed: Vec<(String, String)>,
    pub missing: Vec<String>,
    pub total_entries: usize,
}

impl DictLoadSummary {
    pub fn new() -> Self {
        Self {
            loaded: Vec::new(),
            failed: Vec::new(),
            missing: Vec::new(),
            total_entries: 0,
        }
    }

    pub fn add_loaded(&mut self, name: &str, count: usize, source: DictSource) {
        self.loaded.push((name.to_string(), count, source));
        self.total_entries += count;
    }

    pub fn add_failed(&mut self, name: &str, error: String) {
        self.failed.push((name.to_string(), error));
    }

    pub fn add_missing(&mut self, name: &str) {
        self.missing.push(name.to_string());
    }

    pub fn is_empty(&self) -> bool {
        self.loaded.is_empty()
    }
}

/// 词库统计信息
#[derive(Debug, Clone)]
pub struct DictStats {
    pub total_entries: usize,
    pub unique_pinyin: usize,
    pub by_source: HashMap<DictSource, usize>,
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
