use serde::{Serialize, Deserialize};

/// 🌀 Result of quine creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuineResult {
    pub seed: String,
    pub original_expression: String,
    pub final_expression: String,
    pub output_emoji: String,
    pub reduction_steps: usize,
    pub is_perfect_quine: bool,
}