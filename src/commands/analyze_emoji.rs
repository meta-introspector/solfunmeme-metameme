use anyhow::Result;
use log::info;
use lambda_calculus_core::LambdaEngine;
use emoji_semantics::EmojiSemantics;

pub async fn analyze_emoji(emoji: &str, show_trace: bool) -> Result<()> {
    info!("🔍 Analyzing emoji sequence: {}", emoji);
    
    let mut emoji_engine = EmojiSemantics::new();
    let (expr, resonance) = emoji_engine.interpret_emoji_poem(emoji)?;
    
    let mut lambda_engine = LambdaEngine::new();
    let trace = lambda_engine.normalize(expr.clone())?;
    
    println!("🔍 SOLFUNMEME Emoji Analysis 🔍");
    println!("==============================");
    println!();
    println!("Input: {}", emoji);
    println!("Emoji Count: {}", emoji.chars().count());
    println!("Resonance Score: {:.3}", resonance);
    println!();
    println!("Lambda Expression: {}", expr);
    println!("Reduced Form: {}", trace.final_form);
    println!("Reduction Steps: {}", trace.step_count);
    println!("Normal Form: {}", trace.is_normal_form);
    println!();
    
    if show_trace && !trace.steps.is_empty() {
        println!("🔄 Reduction Trace:");
        println!("------------------");
        for (i, step) in trace.steps.iter().enumerate() {
            println!("Step {}: {}", i, step);
        }
        println!();
    }
    
    // Convert back to emoji
    let output_emoji = emoji_engine.expr_to_emoji(&trace.final_form);
    println!("Output Emoji: {}", output_emoji);
    
    if output_emoji == emoji {
        println!("🌀 QUINE DETECTED: This sequence is self-replicating!");
    }
    
    Ok(())
}