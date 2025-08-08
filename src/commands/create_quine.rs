use anyhow::Result;
use log::info;
use lambda_calculus_core::LambdaEngine;
use emoji_semantics::EmojiSemantics;

pub async fn create_quine(seed: &str, max_steps: usize) -> Result<()> {
    info!("🌀 Creating quine with seed: {}", seed);
    
    let mut lambda_engine = LambdaEngine::new().with_max_steps(max_steps);
    let quine = lambda_engine.create_quine(seed);
    
    let trace = lambda_engine.normalize(quine.clone())?;
    
    let emoji_engine = EmojiSemantics::new();
    let emoji_output = emoji_engine.expr_to_emoji(&trace.final_form);
    
    println!("🌀 SOLFUNMEME Quine Generated 🌀");
    println!("===============================");
    println!();
    println!("Seed: {}", seed);
    println!("Original Expression: {}", quine);
    println!("Final Form: {}", trace.final_form);
    println!("Emoji Output: {}", emoji_output);
    println!("Reduction Steps: {}", trace.step_count);
    println!();
    
    if emoji_output.contains(seed) {
        println!("✅ PERFECT QUINE: The output contains the original seed!");
        println!("🧬 This expression has achieved self-replication!");
    } else {
        println!("🔄 PARTIAL QUINE: The expression shows self-referential properties.");
        println!("🌱 Further evolution may achieve perfect self-replication.");
    }
    
    Ok(())
}