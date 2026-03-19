//! 简体拼音引擎实现
//!
//! 提供基于拼音的汉字输入支持

use crate::PinyinEngine;
use lingcode_core::{
    candidate::{Candidate, Candidates},
    error::{Result},
    types::SchemeType,
};
use lingcode_dict::{RimeDictLoader, DictStats};
use std::collections::HashMap;
use std::path::Path;

/// 简体拼音引擎
pub struct SimplifiedPinyinEngine {
    /// 拼音到汉字的映射表（内置基础词典）
    pinyin_dict: HashMap<String, Vec<(Candidate, u32)>>,
    /// 雾凇拼音词库加载器（支持多词库）
    rime_loader: Option<RimeDictLoader>,
    /// 词库路径
    dict_path: Option<String>,
    /// 是否已加载外部词库
    has_external_dict: bool,
}

impl SimplifiedPinyinEngine {
    /// 创建新引擎实例，加载内置基础词典
    pub fn new() -> Self {
        let mut engine = Self {
            pinyin_dict: HashMap::new(),
            rime_loader: None,
            dict_path: None,
            has_external_dict: false,
        };
        engine.load_builtin_dict();
        engine
    }

    /// 从雾凇拼音词库目录创建引擎（加载多词库）
    pub fn with_rime_dicts(dict_dir: &str) -> Self {
        let mut engine = Self::new();
        engine.load_rime_dicts(dict_dir);
        engine
    }

    /// 加载雾凇拼音多词库
    pub fn load_rime_dicts(&mut self, dict_dir: &str) {
        let mut loader = RimeDictLoader::new();
        let path = Path::new(dict_dir);

        if path.exists() && path.is_dir() {
            match loader.load_rime_ice_dicts(path) {
                Ok(summary) => {
                    if summary.total_entries > 0 {
                        log::info!("已加载 {} 条词条", summary.total_entries);
                        for (name, count, source) in &summary.loaded {
                            let source_name = match source {
                                lingcode_dict::DictSource::Base => "基础",
                                lingcode_dict::DictSource::Ext => "扩展",
                                lingcode_dict::DictSource::Tencent => "腾讯",
                                lingcode_dict::DictSource::Custom => "自定义",
                            };
                            log::info!("  • {}: {} 条 ({})", name, count, source_name);
                        }
                        self.rime_loader = Some(loader);
                        self.dict_path = Some(dict_dir.to_string());
                        self.has_external_dict = true;
                    } else {
                        log::warn!("未找到词库文件");
                    }
                }
                Err(e) => {
                    log::warn!("加载词库失败: {}, 使用内置词典", e);
                }
            }
        } else {
            log::warn!("词库目录不存在: {}, 使用内置词典", dict_dir);
        }
    }

    /// 加载雾凇拼音词库（兼容旧接口，加载单个文件）
    pub fn load_rime_dict(&mut self, dict_path: &str) {
        let path = Path::new(dict_path);
        if path.is_dir() {
            // 如果是目录，使用新的多词库加载
            self.load_rime_dicts(dict_path);
        } else if path.exists() {
            // 如果是文件，加载单个文件
            let mut loader = RimeDictLoader::new();
            match loader.load_from_file(path) {
                Ok(_) => {
                    log::info!("已加载词库: {}", dict_path);
                    self.rime_loader = Some(loader);
                    self.dict_path = Some(dict_path.to_string());
                    self.has_external_dict = true;
                }
                Err(e) => {
                    log::warn!("加载词库失败: {}, 使用内置词典", e);
                }
            }
        } else {
            log::warn!("词库路径不存在: {}, 使用内置词典", dict_path);
        }
    }

    /// 加载内置基础词典
    fn load_builtin_dict(&mut self) {
        // 基础单字映射
        let builtin_data = vec![
            // 中
            ("zhong", vec![("中", 100), ("种", 90), ("重", 80), ("众", 70), ("钟", 60)]),
            // 文
            ("wen", vec![("文", 100), ("问", 90), ("闻", 80), ("稳", 70)]),
            // 国
            ("guo", vec![("国", 100), ("过", 90), ("果", 80)]),
            // 人
            ("ren", vec![("人", 100), ("任", 90), ("认", 80), ("仁", 70)]),
            // 大
            ("da", vec![("大", 100), ("打", 90), ("达", 80)]),
            // 小
            ("xiao", vec![("小", 100), ("笑", 90), ("校", 80)]),
            // 的
            ("de", vec![("的", 100), ("得", 90), ("地", 80)]),
            // 是
            ("shi", vec![("是", 100), ("时", 90), ("事", 80), ("十", 70)]),
            // 我
            ("wo", vec![("我", 100), ("握", 50)]),
            // 你
            ("ni", vec![("你", 100), ("您", 90), ("尼", 50)]),
            // 好
            ("hao", vec![("好", 100), ("号", 90), ("毫", 50)]),
            // 在
            ("zai", vec![("在", 100), ("再", 90), ("载", 50)]),
            // 有
            ("you", vec![("有", 100), ("又", 90), ("由", 80)]),
            // 和
            ("he", vec![("和", 100), ("合", 90), ("河", 80)]),
            // 了
            ("le", vec![("了", 100), ("乐", 80)]),
            // 不
            ("bu", vec![("不", 100), ("部", 90), ("步", 80)]),
            // 一
            ("yi", vec![("一", 100), ("以", 90), ("已", 80), ("意", 70)]),
            // 个
            ("ge", vec![("个", 100), ("各", 90), ("歌", 80)]),
            // 上
            ("shang", vec![("上", 100), ("商", 90), ("伤", 80)]),
            // 下
            ("xia", vec![("下", 100), ("夏", 80)]),
            // 来
            ("lai", vec![("来", 100), ("赖", 60)]),
            // 去
            ("qu", vec![("去", 100), ("取", 90), ("趣", 60)]),
            // 到
            ("dao", vec![("到", 100), ("道", 90), ("倒", 80)]),
            // 说
            ("shuo", vec![("说", 100), ("硕", 60)]),
            // 要
            ("yao", vec![("要", 100), ("药", 90), ("遥", 60)]),
            // 会
            ("hui", vec![("会", 100), ("回", 90), ("汇", 80)]),
            // 能
            ("neng", vec![("能", 100), ("农", 50)]),
            // 可以
            ("keyi", vec![("可以", 100)]),
            // 中国
            ("zhongguo", vec![("中国", 100)]),
            // 中文
            ("zhongwen", vec![("中文", 100)]),
            // 你好
            ("nihao", vec![("你好", 100)]),
        ];

        for (pinyin, chars) in builtin_data {
            let candidates: Vec<(Candidate, u32)> = chars
                .into_iter()
                .map(|(ch, weight)| {
                    let candidate = Candidate::new(ch.to_string())
                        .with_comment(pinyin.to_string())
                        .with_weight(weight);
                    (candidate, weight)
                })
                .collect();
            self.pinyin_dict.insert(pinyin.to_string(), candidates);
        }
    }

    /// 从文件加载词典（扩展用）
    pub fn load_dict_from_file(&mut self, _path: &str) -> Result<()> {
        // TODO: 实现从文件加载词典
        Ok(())
    }

    /// 添加自定义词条
    pub fn add_entry(&mut self, pinyin: &str, text: &str, weight: u32) {
        let candidate = Candidate::new(text.to_string())
            .with_comment(pinyin.to_string())
            .with_weight(weight);
        
        self.pinyin_dict
            .entry(pinyin.to_string())
            .or_insert_with(Vec::new)
            .push((candidate, weight));
    }

    /// 检查是否已加载外部词库
    pub fn has_external_dict(&self) -> bool {
        self.has_external_dict
    }

    /// 获取词库统计信息
    pub fn dict_stats(&self) -> Option<DictStats> {
        self.rime_loader.as_ref().map(|loader| loader.stats())
    }
}

impl Default for SimplifiedPinyinEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PinyinEngine for SimplifiedPinyinEngine {
    fn scheme_type(&self) -> SchemeType {
        SchemeType::PinyinSimplified
    }

    fn get_candidates(&self, pinyin: &str) -> Result<Candidates> {
        let mut candidates = Candidates::new();
        
        // 优先从雾凇拼音词库查询
        if let Some(ref loader) = self.rime_loader {
            let rime_candidates = loader.to_candidates(pinyin);
            for candidate in rime_candidates {
                candidates.add(candidate);
            }
        }
        
        // 如果词库结果不足，补充内置词典
        if candidates.len() < 5 {
            if let Some(matches) = self.pinyin_dict.get(pinyin) {
                for (candidate, _weight) in matches {
                    // 避免重复
                    let already_exists = candidates.iter().any(|c| c.text == candidate.text);
                    if !already_exists {
                        candidates.add(candidate.clone());
                    }
                }
            }
        }
        
        // 前缀匹配（如果结果太少）
        if candidates.len() < 5 {
            for (key, matches) in &self.pinyin_dict {
                if key.starts_with(pinyin) && key.as_str() != pinyin {
                    for (candidate, _weight) in matches.iter().take(2) {
                        let already_exists = candidates.iter().any(|c| c.text == candidate.text);
                        if !already_exists {
                            candidates.add(candidate.clone());
                        }
                    }
                }
                if candidates.len() >= 10 {
                    break;
                }
            }
        }
        
        Ok(candidates)
    }

    fn is_valid_pinyin(&self, pinyin: &str) -> bool {
        if pinyin.is_empty() {
            return false;
        }
        
        // 检查是否是有效的拼音格式（只包含小写字母）
        if !pinyin.chars().all(|c| c.is_ascii_lowercase()) {
            return false;
        }
        
        // 检查是否在词库中
        if let Some(ref loader) = self.rime_loader {
            if !loader.lookup(pinyin).is_empty() {
                return true;
            }
        }
        
        // 检查是否在词典中或有前缀匹配
        self.pinyin_dict.contains_key(pinyin)
            || self.pinyin_dict.keys().any(|k| k.starts_with(pinyin))
    }

    fn get_pinyin_completion(&self, prefix: &str) -> Result<Vec<String>> {
        let completions: Vec<String> = self
            .pinyin_dict
            .keys()
            .filter(|k| k.starts_with(prefix) && k.as_str() != prefix)
            .cloned()
            .take(10)
            .collect();
        
        Ok(completions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_new() {
        let engine = SimplifiedPinyinEngine::new();
        assert_eq!(engine.scheme_type(), SchemeType::PinyinSimplified);
    }

    #[test]
    fn test_get_candidates_zhong() {
        let engine = SimplifiedPinyinEngine::new();
        let candidates = engine.get_candidates("zhong").unwrap();
        
        assert!(candidates.len() >= 3);
        assert!(candidates.get(0).is_some());
    }

    #[test]
    fn test_get_candidates_nihao() {
        let engine = SimplifiedPinyinEngine::new();
        let candidates = engine.get_candidates("nihao").unwrap();
        
        assert!(candidates.len() >= 1);
        let first = candidates.get(0).unwrap();
        assert_eq!(first.text, "你好");
    }

    #[test]
    fn test_is_valid_pinyin() {
        let engine = SimplifiedPinyinEngine::new();
        
        assert!(engine.is_valid_pinyin("zhong"));
        assert!(engine.is_valid_pinyin("ni"));
        assert!(!engine.is_valid_pinyin(""));
        assert!(!engine.is_valid_pinyin("123"));
    }

    #[test]
    fn test_pinyin_completion() {
        let engine = SimplifiedPinyinEngine::new();
        let completions = engine.get_pinyin_completion("zho").unwrap();
        
        assert!(!completions.is_empty());
        assert!(completions.contains(&"zhong".to_string()));
    }

    #[test]
    fn test_add_custom_entry() {
        let mut engine = SimplifiedPinyinEngine::new();
        engine.add_entry("test", "测试", 100);
        
        let candidates = engine.get_candidates("test").unwrap();
        assert!(candidates.get(0).is_some());
        assert_eq!(candidates.get(0).unwrap().text, "测试");
    }
}
