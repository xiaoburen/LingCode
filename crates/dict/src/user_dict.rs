//! 用户词频学习模块
//!
//! 记录用户输入习惯，动态调整候选词排序

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// 词条使用记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordRecord {
    /// 文本内容
    pub text: String,
    /// 对应的拼音
    pub pinyin: String,
    /// 使用次数
    pub count: u32,
    /// 最后使用时间（Unix时间戳）
    pub last_used: u64,
    /// 首次使用时间
    pub first_used: u64,
}

impl WordRecord {
    /// 创建新记录
    pub fn new(text: String, pinyin: String) -> Self {
        let now = current_timestamp();
        Self {
            text,
            pinyin,
            count: 1,
            last_used: now,
            first_used: now,
        }
    }

    /// 更新使用记录
    pub fn update(&mut self) {
        self.count += 1;
        self.last_used = current_timestamp();
    }

    /// 计算词频分数（考虑使用次数和时间衰减）
    pub fn score(&self) -> f64 {
        let now = current_timestamp();
        let days_since_last = (now - self.last_used) as f64 / 86400.0;
        let days_since_first = (now - self.first_used) as f64 / 86400.0;

        // 基础分数：使用次数
        let count_score = self.count as f64;

        // 时间衰减因子：最近使用过的词分数更高
        // 使用指数衰减：e^(-days/30)，30天半衰期
        let time_decay = (-days_since_last / 30.0).exp();

        // 活跃度因子：经常使用的词分数更高
        // 计算平均每天使用次数
        let daily_avg = if days_since_first > 0.0 {
            self.count as f64 / days_since_first
        } else {
            self.count as f64
        };

        count_score * time_decay * (1.0 + daily_avg.ln_1p())
    }
}

/// 词频数据库
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDict {
    /// 用户词库：拼音 -> 词条列表
    words: HashMap<String, Vec<WordRecord>>,
    /// 全局词条索引：文本 -> (拼音, 索引)
    text_index: HashMap<String, (String, usize)>,
    /// 总记录数
    total_records: usize,
    /// 最后保存时间
    last_saved: u64,
}

impl UserDict {
    /// 创建空的用户词库
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            text_index: HashMap::new(),
            total_records: 0,
            last_saved: 0,
        }
    }

    /// 从文件加载用户词库
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let dict: UserDict = serde_json::from_reader(reader)?;
        Ok(dict)
    }

    /// 保存到文件
    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        // 确保父目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        
        Ok(())
    }

    /// 记录用户使用某个词条
    pub fn record_usage(&mut self, text: &str, pinyin: &str) {
        // 检查是否已存在
        if let Some((existing_pinyin, index)) = self.text_index.get(text).cloned() {
            // 更新现有记录
            if let Some(records) = self.words.get_mut(&existing_pinyin) {
                if let Some(record) = records.get_mut(index) {
                    record.update();
                    return;
                }
            }
        }

        // 创建新记录
        let record = WordRecord::new(text.to_string(), pinyin.to_string());
        let pinyin_key = pinyin.to_string();
        
        let records = self.words.entry(pinyin_key.clone()).or_insert_with(Vec::new);
        let index = records.len();
        records.push(record);
        
        self.text_index.insert(text.to_string(), (pinyin_key, index));
        self.total_records += 1;
    }

    /// 根据拼音查询词条，返回按词频排序的结果
    pub fn lookup(&self, pinyin: &str) -> Vec<(&WordRecord, f64)> {
        let mut results: Vec<(&WordRecord, f64)> = self
            .words
            .get(pinyin)
            .map(|records| {
                records.iter()
                    .map(|r| (r, r.score()))
                    .collect()
            })
            .unwrap_or_default();

        // 按分数降序排序
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results
    }

    /// 获取词条的使用次数
    pub fn get_count(&self, text: &str) -> u32 {
        self.text_index
            .get(text)
            .and_then(|(pinyin, index)| {
                self.words.get(pinyin)?.get(*index).map(|r| r.count)
            })
            .unwrap_or(0)
    }

    /// 清理低频词条（保留最近使用或高频的词条）
    pub fn cleanup(&mut self, max_entries: usize) {
        if self.total_records <= max_entries {
            return;
        }

        // 收集所有记录及其分数
        let mut all_records: Vec<(f64, String, usize)> = Vec::new();
        for (pinyin, records) in &self.words {
            for (index, record) in records.iter().enumerate() {
                all_records.push((record.score(), pinyin.clone(), index));
            }
        }

        // 按分数排序
        all_records.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // 保留前 max_entries 个
        let to_keep: std::collections::HashSet<(String, usize)> = all_records
            .into_iter()
            .take(max_entries)
            .map(|(_, pinyin, index)| (pinyin, index))
            .collect();

        // 重建数据结构
        let mut new_words: HashMap<String, Vec<WordRecord>> = HashMap::new();
        let mut new_index: HashMap<String, (String, usize)> = HashMap::new();
        let mut new_total = 0;

        for (pinyin, records) in &self.words {
            let mut new_records: Vec<WordRecord> = Vec::new();
            for (old_index, record) in records.iter().enumerate() {
                if to_keep.contains(&(pinyin.clone(), old_index)) {
                    let new_index_val = new_records.len();
                    new_index.insert(record.text.clone(), (pinyin.clone(), new_index_val));
                    new_records.push(record.clone());
                    new_total += 1;
                }
            }
            if !new_records.is_empty() {
                new_words.insert(pinyin.clone(), new_records);
            }
        }

        self.words = new_words;
        self.text_index = new_index;
        self.total_records = new_total;
    }

    /// 合并另一个用户词库
    pub fn merge(&mut self, other: &UserDict) {
        for (pinyin, records) in &other.words {
            for record in records {
                self.record_usage(&record.text, pinyin);
            }
        }
    }

    /// 获取统计信息
    pub fn stats(&self) -> UserDictStats {
        UserDictStats {
            total_records: self.total_records,
            unique_pinyin: self.words.len(),
            unique_words: self.text_index.len(),
        }
    }
}

impl Default for UserDict {
    fn default() -> Self {
        Self::new()
    }
}

/// 用户词库统计信息
#[derive(Debug, Clone)]
pub struct UserDictStats {
    pub total_records: usize,
    pub unique_pinyin: usize,
    pub unique_words: usize,
}

/// 获取当前 Unix 时间戳
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_word_record() {
        let mut record = WordRecord::new("测试".to_string(), "ceshi".to_string());
        assert_eq!(record.count, 1);
        
        record.update();
        assert_eq!(record.count, 2);
        assert!(record.score() > 0.0);
    }

    #[test]
    fn test_user_dict_record() {
        let mut dict = UserDict::new();
        
        dict.record_usage("中国", "zhongguo");
        dict.record_usage("中国", "zhongguo");
        dict.record_usage("中文", "zhongwen");
        
        assert_eq!(dict.get_count("中国"), 2);
        assert_eq!(dict.get_count("中文"), 1);
        assert_eq!(dict.get_count("不存在"), 0);
        
        let results = dict.lookup("zhongguo");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0.text, "中国");
    }

    #[test]
    fn test_user_dict_save_load() {
        let mut dict = UserDict::new();
        dict.record_usage("测试", "ceshi");
        dict.record_usage("词频", "cipin");
        
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        
        dict.save_to_file(path).unwrap();
        
        let loaded = UserDict::load_from_file(path).unwrap();
        assert_eq!(loaded.get_count("测试"), 1);
        assert_eq!(loaded.get_count("词频"), 1);
    }

    #[test]
    fn test_cleanup() {
        let mut dict = UserDict::new();
        
        // 添加多个词条
        for i in 0..100 {
            dict.record_usage(&format!("词{}", i), &format!("ci{}", i));
        }
        
        assert_eq!(dict.total_records, 100);
        
        // 清理到只剩50个
        dict.cleanup(50);
        assert_eq!(dict.total_records, 50);
    }

    #[test]
    fn test_merge() {
        let mut dict1 = UserDict::new();
        dict1.record_usage("中国", "zhongguo");
        
        let mut dict2 = UserDict::new();
        dict2.record_usage("中文", "zhongwen");
        dict2.record_usage("中国", "zhongguo");
        
        dict1.merge(&dict2);
        
        assert_eq!(dict1.get_count("中国"), 2); // 1 + 1
        assert_eq!(dict1.get_count("中文"), 1);
    }
}
