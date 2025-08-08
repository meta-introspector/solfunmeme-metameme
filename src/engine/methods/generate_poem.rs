use crate::{engine::MetaMemeEngine, engine::GeneratedPoem};
use anyhow::Result;
use log::debug;

use crate::{engine::MetaMemeEngine, engine::GeneratedPoem};
use anyhow::Result;
use log::debug;

impl MetaMemeEngine {
    pub async fn generate_poem(&mut self, emoji_sequence: &str) -> Result<GeneratedPoem> {
        debug!("🎭 Generating poem from: {}", emoji_sequence);
        
        // Interpret the emoji sequence
        let (expr, resonance) = self.emoji_engine.interpret_emoji_poem(emoji_sequence)?;
        
        // Normalize the lambda expression
        let trace = self.lambda_engine.normalize(expr.clone())?;
        
        // Generate poetic text
        let poetic_text = self.generate_poetic_text(&expr, resonance);
        
        // Convert back to emoji
        let output_emoji = self.emoji_engine.expr_to_emoji(&trace.final_form);
        
        Ok(GeneratedPoem {
            input_emoji: emoji_sequence.to_string(),
            output_emoji: output_emoji.clone(),
            lambda_expression: format!("{}", expr),
            reduced_expression: format!("{}", trace.final_form),
            poetic_text,
            resonance_score: resonance,
            reduction_steps: trace.step_count,
            is_quine: output_emoji == emoji_sequence,
        })
    }
}