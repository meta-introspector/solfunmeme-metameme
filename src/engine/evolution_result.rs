use serde::{Serialize, Deserialize};

/// 🧬 Result of universe evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionResult {
    pub initial_stanza_count: usize,
    pub final_stanza_count: usize,
    pub new_stanzas_created: usize,
    pub generations_completed: u32,
    pub evolved_stanza_ids: Vec<u32>,
}