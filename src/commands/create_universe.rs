use anyhow::Result;
use log::info;
use std::path::Path;
use stanza_universe::StanzaUniverse;
use emoji_semantics::EmojiSemantics;
use rand::Rng;

pub async fn create_universe(count: u32, output: &Path) -> Result<()> {
    info!("🌌 Creating universe with {} stanzas", count);
    
    let mut universe = StanzaUniverse::new();
    
    // Generate additional stanzas beyond the core ones
    let mut emoji_engine = EmojiSemantics::new();
    
    for i in 4..=count {
        let emoji_length = rand::thread_rng().gen_range(3..=7);
        let emoji_sequence = emoji_engine.generate_random_poem(emoji_length, 0.80);
        
        let poetic_text = format!(
            "Stanza {} emerges from the void,\nWhere {} dances unalloyed,\nIn recursive loops of pure delight,\nBringing darkness into light.",
            i, emoji_sequence
        );
        
        let resonance = rand::thread_rng().gen_range(0.80..0.98);
        let recursion_depth = rand::thread_rng().gen_range(1..=4);
        let is_quine = rand::thread_rng().gen_bool(0.1); // 10% chance of being a quine
        
        universe.create_stanza(&poetic_text, &emoji_sequence, resonance, is_quine, recursion_depth)?;
    }
    
    // Serialize the universe
    let universe_data = serde_json::to_string_pretty(&universe.stanzas)?;
    std::fs::write(output, universe_data)?;
    
    info!("✅ Universe with {} stanzas written to {}", count, output.display());
    Ok(())
}
