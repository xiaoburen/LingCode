//! 输入分段相关定义

use crate::candidate::{Candidate, Candidates};
use serde::{Deserialize, Serialize};

/// 输入段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// 拼音输入（原始）
    pub input: String,
    /// 已确认的文字
    pub confirmed: String,
    /// 候选词
    #[serde(skip)]
    pub candidates: Candidates,
    /// 当前选中的候选词索引
    pub selected_index: usize,
}

impl Segment {
    pub fn new(input: String) -> Self {
        Self {
            input,
            confirmed: String::new(),
            candidates: Candidates::new(),
            selected_index: 0,
        }
    }

    /// 获取当前选中的候选词
    pub fn current_candidate(&self) -> Option<&Candidate> {
        self.candidates.get(self.selected_index)
    }

    /// 移至下一个候选词
    pub fn next_candidate(&mut self) {
        if self.selected_index + 1 < self.candidates.len() {
            self.selected_index += 1;
        }
    }

    /// 移至上一个候选词
    pub fn prev_candidate(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// 确认当前候选词
    pub fn commit(&mut self) {
        if let Some(candidate) = self.current_candidate() {
            self.confirmed = candidate.text.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_creation() {
        let seg = Segment::new("zhong".to_string());
        assert_eq!(seg.input, "zhong");
        assert_eq!(seg.confirmed, "");
    }

    #[test]
    fn test_segment_navigation() {
        let mut seg = Segment::new("zhong".to_string());
        seg.candidates.add(Candidate::new("中".to_string()));
        seg.candidates.add(Candidate::new("众".to_string()));

        assert_eq!(seg.selected_index, 0);
        seg.next_candidate();
        assert_eq!(seg.selected_index, 1);
        seg.prev_candidate();
        assert_eq!(seg.selected_index, 0);
    }
}
