use serde::{Serialize, Deserialize};

/// 🎭 Result of poem generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPoem {
    pub input_emoji: String,
    pub output_emoji: String,
    pub lambda_expression: String,
    pub reduced_expression: String,
    pub poetic_text: String,
    pub resonance_score: f64,
    pub reduction_steps: usize,
    pub is_quine: bool,
}