//! 候选词相关定义

use serde::{Deserialize, Serialize};

/// 候选词
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    /// 候选文字
    pub text: String,
    /// 注音（拼音等）
    pub comment: Option<String>,
    /// 权重/频率
    pub weight: u32,
    /// 候选词来源（词库名）
    pub source: Option<String>,
}

impl Candidate {
    pub fn new(text: String) -> Self {
        Self {
            text,
            comment: None,
            weight: 0,
            source: None,
        }
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
}

/// 候选词列表
#[derive(Debug, Clone, Default)]
pub struct Candidates {
    items: Vec<Candidate>,
}

impl Candidates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, candidate: Candidate) {
        self.items.push(candidate);
    }

    pub fn get(&self, index: usize) -> Option<&Candidate> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Candidate> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_builder() {
        let c = Candidate::new("中".to_string())
            .with_comment("zhong".to_string())
            .with_weight(100);
        
        assert_eq!(c.text, "中");
        assert_eq!(c.weight, 100);
    }

    #[test]
    fn test_candidates_collection() {
        let mut candidates = Candidates::new();
        candidates.add(Candidate::new("中".to_string()));
        candidates.add(Candidate::new("众".to_string()));
        
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates.get(0).unwrap().text, "中");
    }
}
