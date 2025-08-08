use crate::{engine::MetaMemeEngine, engine::QuineResult};
use anyhow::Result;
use log::debug;

impl MetaMemeEngine {
    pub async fn create_quine(&mut self, seed: &str) -> Result<QuineResult> {
        debug!("🌀 Creating quine with seed: {}", seed);
        
        let quine_expr = self.lambda_engine.create_quine(seed);
        let trace = self.lambda_engine.normalize(quine_expr.clone())?;
        let output_emoji = self.emoji_engine.expr_to_emoji(&trace.final_form);
        
        let is_perfect_quine = output_emoji.contains(seed);
        
        Ok(QuineResult {
            seed: seed.to_string(),
            original_expression: format!("{}", quine_expr),
            final_expression: format!("{}", trace.final_form),
            output_emoji,
            reduction_steps: trace.step_count,
            is_perfect_quine,
        })
    }
}