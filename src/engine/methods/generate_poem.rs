use anyhow::Result;
use log::info;
use ragit_instrumentation_macros::instrument_function;

use crate::engine::{MetaMemeEngine, GeneratedPoem};

impl MetaMemeEngine {
    /// Generates a poetic stanza from an emoji sequence.
    /// 
    /// This function interprets the input emoji sequence as a lambda calculus expression,
    /// then uses the stanza universe to generate a poetic text based on the expression's semantics.
    /// 
    /// # Arguments
    /// 
    /// * `emoji_sequence` - A string slice representing the emoji sequence (e.g., "🌀🎭🧬").
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `GeneratedPoem` struct if successful, or an `anyhow::Error` if an error occurs.
    #[instrument_function]
    pub async fn generate_poem(&mut self, emoji_sequence: &str) -> Result<GeneratedPoem> {
        info!("Generating poem for emoji sequence: {}", emoji_sequence);

        // Interpret the emoji sequence into a lambda calculus expression
        let (expr, resonance) = self.emoji_engine.interpret_emoji_poem(emoji_sequence)?;

        // Generate poetic text based on the expression
        let poetic_text = self.generate_poetic_text(&expr, resonance);

        Ok(GeneratedPoem {
            input_emoji: emoji_sequence.to_string(),
            output_emoji: self.emoji_engine.expr_to_emoji(&expr),
            lambda_expression: format!("{}", expr),
            reduced_expression: format!("{}", expr),
            poetic_text,
            resonance_score: resonance,
            reduction_steps: 0, // This will be updated if we perform reduction here
            is_quine: false, // This will be updated if we perform quine check here
        })
    }
}