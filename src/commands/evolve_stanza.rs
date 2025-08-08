use anyhow::Result;
use log::{info, error};
use stanza_universe::StanzaUniverse;

pub async fn evolve_stanza(parent_id: u32, mutation_rate: f64, generations: u32) -> Result<()> {
    info!("🧬 Evolving stanza {} for {} generations", parent_id, generations);
    
    let mut universe = StanzaUniverse::new();
    let mut current_id = parent_id;
    
    for generation in 1..=generations {
        match universe.evolve_stanza(current_id, mutation_rate) {
            Ok(new_id) => {
                let new_stanza = universe.get_stanza(new_id).unwrap();
                println!("🧬 Generation {}: Stanza #{}", generation, new_id);
                println!("   Emoji: {}", new_stanza.emoji_sequence);
                println!("   Resonance: {:.3}", new_stanza.resonance);
                println!("   Recursion Depth: {}", new_stanza.recursion_depth);
                println!();
                current_id = new_id;
            }
            Err(e) => {
                error!("❌ Evolution failed at generation {}: {}", generation, e);
                break;
            }
        }
    }
    
    Ok(())
}