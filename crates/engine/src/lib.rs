//! 灵码输入法引擎 - 核心状态机实现
//!
//! 实现输入状态流转：Idle -> Composing -> Selecting

use lingcode_core::types::{InputState, KeyEvent, SchemeType};
use lingcode_core::candidate::Candidate;
use lingcode_pinyin::{PinyinEngine, SimplifiedPinyinEngine};

pub mod candidate;
pub mod input_state;

/// 引擎输出类型
#[derive(Debug, Clone)]
pub enum EngineOutput {
    /// 无输出（继续输入）
    None,
    /// 提交文本（输入完成）
    Commit(String),
    /// 更新候选词列表
    Candidates(Vec<Candidate>),
    /// 清空输入
    Clear,
}

/// 输入法引擎
pub struct Engine {
    /// 当前状态
    state: InputState,
    /// 输入缓冲区（拼音字符串）
    input_buffer: String,
    /// 候选词列表
    candidates: Vec<Candidate>,
    /// 选中的候选词索引
    selected_index: usize,
    /// 拼音引擎
    pinyin_engine: SimplifiedPinyinEngine,
}

impl Engine {
    /// 创建新引擎实例
    pub fn new() -> Self {
        Self {
            state: InputState::Idle,
            input_buffer: String::new(),
            candidates: Vec::new(),
            selected_index: 0,
            pinyin_engine: SimplifiedPinyinEngine::new(),
        }
    }

    /// 获取当前状态
    pub fn state(&self) -> InputState {
        self.state
    }

    /// 获取输入缓冲区
    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    /// 获取候选词列表
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    /// 获取选中的候选词索引
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// 处理按键事件，返回引擎输出
    pub fn process_key(&mut self, key: KeyEvent) -> EngineOutput {
        match self.state {
            InputState::Idle => self.handle_idle(key),
            InputState::Composing => self.handle_composing(key),
            InputState::Selecting => self.handle_selecting(key),
        }
    }

    /// 处理空闲状态下的按键
    fn handle_idle(&mut self, key: KeyEvent) -> EngineOutput {
        // 只响应字母键（开始输入）
        if key.key.is_ascii_alphabetic() && key.modifiers.is_empty() {
            self.input_buffer.push(key.key);
            self.state = InputState::Composing;
            self.update_candidates();
            EngineOutput::Candidates(self.candidates.clone())
        } else {
            EngineOutput::None
        }
    }

    /// 处理编辑状态下的按键
    fn handle_composing(&mut self, key: KeyEvent) -> EngineOutput {
        match key.key {
            // 字母：继续输入
            c if c.is_ascii_alphabetic() => {
                if key.modifiers.is_empty() {
                    self.input_buffer.push(c);
                    self.update_candidates();
                    EngineOutput::Candidates(self.candidates.clone())
                } else {
                    EngineOutput::None
                }
            }
            // 空格或数字 1-9：进入选择状态
            ' ' | '1'..='9' => {
                if !self.candidates.is_empty() {
                    self.state = InputState::Selecting;
                    if key.key == ' ' {
                        self.selected_index = 0; // 空格选择第一个
                    } else {
                        self.selected_index = (key.key as usize - '1' as usize)
                            .min(self.candidates.len() - 1);
                    }
                    // 立即提交选中的候选词
                    self.commit_selected()
                } else {
                    EngineOutput::None
                }
            }
            // Backspace：删除最后一个字符
            '\u{8}' | '\u{7f}' => {
                self.input_buffer.pop();
                if self.input_buffer.is_empty() {
                    self.state = InputState::Idle;
                    self.candidates.clear();
                    EngineOutput::Clear
                } else {
                    self.update_candidates();
                    EngineOutput::Candidates(self.candidates.clone())
                }
            }
            // Esc：取消输入
            '\u{1b}' => {
                self.reset();
                EngineOutput::Clear
            }
            // 其他键：忽略
            _ => EngineOutput::None,
        }
    }

    /// 处理选择状态下的按键
    fn handle_selecting(&mut self, key: KeyEvent) -> EngineOutput {
        match key.key {
            // 数字 1-9：选择候选词
            '1'..='9' => {
                let index = (key.key as usize - '1' as usize)
                    .min(self.candidates.len() - 1);
                self.selected_index = index;
                self.commit_selected()
            }
            // 空格：选择当前高亮的候选词
            ' ' => {
                self.commit_selected()
            }
            // 方向键上/下：切换选择（简化版用 j/k 代替）
            'j' => {
                if self.selected_index + 1 < self.candidates.len() {
                    self.selected_index += 1;
                }
                EngineOutput::Candidates(self.candidates.clone())
            }
            'k' => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                EngineOutput::Candidates(self.candidates.clone())
            }
            // Esc：取消选择，返回编辑状态
            '\u{1b}' => {
                self.state = InputState::Composing;
                EngineOutput::Candidates(self.candidates.clone())
            }
            // 其他键：提交当前选择并处理新按键
            c => {
                let output = self.commit_selected();
                // 如果新按键是字母，继续处理
                if c.is_ascii_alphabetic() {
                    self.input_buffer.push(c);
                    self.state = InputState::Composing;
                    self.update_candidates();
                }
                output
            }
        }
    }

    /// 提交选中的候选词
    fn commit_selected(&mut self) -> EngineOutput {
        if let Some(candidate) = self.candidates.get(self.selected_index) {
            let text = candidate.text.clone();
            self.reset();
            EngineOutput::Commit(text)
        } else {
            EngineOutput::None
        }
    }

    /// 更新候选词列表
    fn update_candidates(&mut self) {
        // 使用真实的拼音引擎获取候选词
        match self.pinyin_engine.get_candidates(&self.input_buffer) {
            Ok(candidates_list) => {
                self.candidates = candidates_list.iter().cloned().collect();
            }
            Err(_) => {
                self.candidates.clear();
            }
        }
        self.selected_index = 0;
    }

    /// 检查是否是有效的拼音输入
    pub fn is_valid_input(&self, input: &str) -> bool {
        self.pinyin_engine.is_valid_pinyin(input)
    }

    /// 重置引擎状态
    fn reset(&mut self) {
        self.state = InputState::Idle;
        self.input_buffer.clear();
        self.candidates.clear();
        self.selected_index = 0;
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lingcode_core::types::KeyModifiers;

    #[test]
    fn test_engine_new() {
        let engine = Engine::new();
        assert_eq!(engine.state(), InputState::Idle);
        assert!(engine.input_buffer().is_empty());
    }

    #[test]
    fn test_idle_to_composing() {
        let mut engine = Engine::new();
        let output = engine.process_key(KeyEvent {
            keycode: 97, // 'a'
            key: 'a',
            modifiers: KeyModifiers::new(),
        });
        
        assert_eq!(engine.state(), InputState::Composing);
        assert_eq!(engine.input_buffer(), "a");
        assert!(matches!(output, EngineOutput::Candidates(_)));
    }

    #[test]
    fn test_composing_backspace() {
        let mut engine = Engine::new();
        
        // 输入 'a'
        engine.process_key(KeyEvent {
            keycode: 97,
            key: 'a',
            modifiers: KeyModifiers::new(),
        });
        
        // 删除
        let output = engine.process_key(KeyEvent {
            keycode: 8, // Backspace
            key: '\u{8}',
            modifiers: KeyModifiers::new(),
        });
        
        assert_eq!(engine.state(), InputState::Idle);
        assert!(matches!(output, EngineOutput::Clear));
    }

    #[test]
    fn test_composing_to_commit() {
        let mut engine = Engine::new();
        
        // 输入 'zhongwen'
        for c in "zhongwen".chars() {
            engine.process_key(KeyEvent {
                keycode: c as u32,
                key: c,
                modifiers: KeyModifiers::new(),
            });
        }
        
        // 按空格提交
        let output = engine.process_key(KeyEvent {
            keycode: 32,
            key: ' ',
            modifiers: KeyModifiers::new(),
        });
        
        assert!(matches!(output, EngineOutput::Commit(_)));
        assert_eq!(engine.state(), InputState::Idle);
    }
}
