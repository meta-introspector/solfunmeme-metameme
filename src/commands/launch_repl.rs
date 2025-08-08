use anyhow::Result;
use std::io::{self, Write};
use lambda_calculus_core::LambdaEngine;
use emoji_semantics::EmojiSemantics;

pub async fn launch_repl() -> Result<()> {
    println!("🚀 SOLFUNMEME Interactive REPL");
    println!("==============================");
    println!("Enter emoji sequences to see their lambda calculus interpretations!");
    println!("Commands: :quit, :help, :stats");
    println!();
    
    let mut emoji_engine = EmojiSemantics::new();
    let mut lambda_engine = LambdaEngine::new();
    
    loop {
        print!("🌀 > ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input {
            ":quit" | ":q" => {
                println!("👋 Farewell from the MetaMeme universe!");
                break;
            }
            ":help" | ":h" => {
                println!("🎭 SOLFUNMEME REPL Help:");
                println!("  Enter emoji sequences to interpret them");
                println!("  :quit or :q - Exit the REPL");
                println!("  :help or :h - Show this help");
                continue;
            }
            ":stats" | ":s" => {
                println!("📊 Current session statistics:");
                println!("  Emoji semantics loaded: {}", emoji_engine.semantics.len());
                println!("  Lambda engine max steps: {}", lambda_engine.max_steps);
                continue;
            }
            _ => {
                match emoji_engine.interpret_emoji_poem(input) {
                    Ok((expr, resonance)) => {
                        match lambda_engine.normalize(expr.clone()) {
                            Ok(trace) => {
                                println!("  Expression: {}", expr);
                                println!("  Reduced: {}", trace.final_form);
                                println!("  Resonance: {:.3}", resonance);
                                println!("  Steps: {}", trace.step_count);
                                
                                let output_emoji = emoji_engine.expr_to_emoji(&trace.final_form);
                                println!("  Output: {}", output_emoji);
                                
                                if output_emoji == input {
                                    println!("  🌀 QUINE!");
                                }
                            }
                            Err(e) => println!("  ❌ Reduction error: {}", e),
                        }
                    }
                    Err(e) => println!("  ❌ Interpretation error: {}", e),
                }
            }
        }
        println!();
    }
    
    Ok(())
}