use crate::{engine::MetaMemeEngine, engine::EvolutionResult};
use anyhow::Result;
use log::{info, debug};
use rand::Rng;

impl MetaMemeEngine {
    pub async fn evolve_universe(&mut self, generations: u32, mutation_rate: f64) -> Result<EvolutionResult> {
        info!("🧬 Evolving universe for {} generations", generations);
        
        let initial_count = self.stanza_universe.stanzas.len();
        let mut evolved_stanzas = Vec::new();
        
        for generation in 1..=generations {
            // Get all current stanza IDs
            let current_ids: Vec<u32> = self.stanza_universe.stanzas.keys().cloned().collect();
            
            // Evolve a random selection of stanzas
            let evolution_count = (current_ids.len() as f64 * mutation_rate) as usize;
            
            for _ in 0..evolution_count {
                let parent_id = current_ids[rand::thread_rng().gen_range(0..current_ids.len())];
                
                match self.stanza_universe.evolve_stanza(parent_id, mutation_rate) {
                    Ok(new_id) => {
                        evolved_stanzas.push(new_id);
                        debug!("🧬 Generation {}: Evolved stanza #{}", generation, new_id);
                    }
                    Err(e) => {
                        debug!("⚠️ Evolution failed for stanza {}: {}", parent_id, e);
                    }
                }
            }
        }
        
        let final_count = self.stanza_universe.stanzas.len();
        
        Ok(EvolutionResult {
            initial_stanza_count: initial_count,
            final_stanza_count: final_count,
            new_stanzas_created: evolved_stanzas.len(),
            generations_completed: generations,
            evolved_stanza_ids: evolved_stanzas,
        })
    }
}