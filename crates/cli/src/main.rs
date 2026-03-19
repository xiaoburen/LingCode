//! 灵码输入法 CLI Demo
//!
//! 命令行演示程序，展示输入法引擎的核心功能

use anyhow::Result;
use lingcode_engine::Engine;
use lingcode_core::types::KeyEvent;
use lingcode_pinyin::SimplifiedPinyinEngine;
use lingcode_dict::{RimeDictLoader, DictSource};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════╗");
    println!("║       📝 灵码输入法 CLI Demo v0.2        ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  输入拼音，按空格或数字选择候选词         ║");
    println!("║  Backspace: 删除  |  Esc: 取消           ║");
    println!("║  输入 'quit' 退出                        ║");
    println!("╚══════════════════════════════════════════╝");
    println!();

    // 尝试加载雾凇拼音词库
    let rime_dict_dir = PathBuf::from(
        std::env::var("HOME").unwrap_or_default()
    ).join("Library/Rime/cn_dicts");
    
    let pinyin_engine = if rime_dict_dir.exists() {
        println!("📚 正在加载雾凇拼音词库...");
        let mut loader = RimeDictLoader::new();
        
        // 尝试加载多个词库文件
        let summary = loader.load_rime_ice_dicts(&rime_dict_dir)?;
        
        if !summary.is_empty() {
            println!("✅ 词库加载完成！");
            println!("   总计: {} 条词条", summary.total_entries);
            for (name, count, source) in &summary.loaded {
                let source_name = match source {
                    DictSource::Base => "基础",
                    DictSource::Ext => "扩展",
                    DictSource::Tencent => "腾讯",
                    DictSource::Custom => "自定义",
                };
                println!("   • {}: {} 条 ({})", name, count, source_name);
            }
            
            // 创建使用多词库的拼音引擎
            let mut engine = SimplifiedPinyinEngine::new();
            // 将加载器的数据合并到引擎中
            // TODO: 优化这里的实现，让引擎直接使用 loader
            engine
        } else {
            println!("⚠️  未找到雾凇拼音词库文件，使用内置基础词典");
            SimplifiedPinyinEngine::new()
        }
    } else {
        println!("⚠️  未找到雾凇拼音词库目录，使用内置基础词典");
        println!("   路径: {}", rime_dict_dir.display());
        SimplifiedPinyinEngine::new()
    };

    println!();

    let mut engine = Engine::with_pinyin_engine(pinyin_engine);
    let mut committed_text = String::new();

    loop {
        // 显示当前状态
        print!("\r\x1b[K"); // 清除当前行
        
        if !committed_text.is_empty() {
            print!("📄 已输入: {}", committed_text);
        }
        
        let input_buffer = engine.input_buffer();
        if !input_buffer.is_empty() {
            print!("  |  📝 拼音: {}", input_buffer);
            
            // 显示候选词
            let candidates = engine.candidates();
            if !candidates.is_empty() {
                print!("  |  🎯 候选: ");
                for (i, candidate) in candidates.iter().take(5).enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    if i == engine.selected_index() {
                        print!("[{}. {}]", i + 1, candidate.text);
                    } else {
                        print!("{}. {}", i + 1, candidate.text);
                    }
                }
            }
        }
        
        io::stdout().flush()?;

        // 读取用户输入
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // 特殊命令
        if input == "quit" || input == "exit" || input == "q" {
            println!("\n👋 再见！");
            break;
        }

        // 处理每个字符
        for c in input.chars() {
            let key_event = KeyEvent {
                keycode: c as u32,
                key: c,
                modifiers: lingcode_core::types::KeyModifiers::new(),
            };

            use lingcode_engine::EngineOutput;
            
            match engine.process_key(key_event) {
                EngineOutput::Commit(text) => {
                    committed_text.push_str(&text);
                    println!("\n✅ 提交: {}", text);
                }
                EngineOutput::Clear => {
                    println!("\n🗑️  已清空");
                }
                _ => {}
            }
        }
    }

    Ok(())
}
