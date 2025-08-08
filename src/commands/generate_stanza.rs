use anyhow::Result;
use log::info;
use std::path::Path;
use lambda_calculus_core::{Expr, LambdaEngine};
use emoji_semantics::EmojiSemantics;
use crate::MetaMemeEngine;


pub async fn generate_stanza(engine: &mut MetaMemeEngine, emoji: &str, output: Option<&Path>) -> Result<()> {
    info!("🎭 Generating stanza from emoji: {}", emoji);
    
    let (expr, resonance) = engine.emoji_engine.interpret_emoji_poem(emoji)?;
    
    let trace = engine.lambda_engine.normalize(expr.clone())?;
    
    // Generate poetic text based on the expression
    let poetic_text = generate_poetic_text::generate_poetic_text(&expr, resonance);
    
    let result = format!(
        "🌟 SOLFUNMEME Stanza Generated 🌟\n================================\n\nEmoji Sequence: {}\nResonance Score: {:.3}\n\nLambda Expression: {}\nReduced Form: {}\nReduction Steps: {}\n\nPoetic Manifestation:\n---------------------\n{}\n\n🧬 This stanza embodies the eternal dance between computation and creativity,\nwhere lambda calculus meets the infinite poetry of the digital realm.\n",
        emoji,
        resonance,
        expr,
        trace.final_form,
        trace.step_count,
        poetic_text
    );
    
    if let Some(output_path) = output {
        std::fs::write(output_path, &result)?;
        info!("📝 Stanza written to: {}", output_path.display());
    } else {
        println!("{}", result);
    }
    
    Ok(())
}