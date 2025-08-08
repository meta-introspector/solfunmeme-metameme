use anyhow::Result;
use stanza_universe::StanzaUniverse;
use emoji_semantics::EmojiSemantics;

pub async fn show_stats() -> Result<()> {
    println!("📊 SOLFUNMEME MetaMeme Statistics");
    println!("=================================");
    
    let universe = StanzaUniverse::new();
    let emoji_engine = EmojiSemantics::new();
    
    println!("🌌 Universe Statistics:");
    println!("  Total Stanzas: {}", universe.stanzas.len());
    println!("  Emoji Mappings: {}", universe.emoji_to_stanza.len());
    println!();
    
    println!("🎭 Emoji Semantics:");
    println!("  Total Emoji Semantics: {}", emoji_engine.semantics.len());
    println!("  Reverse Mappings: {}", emoji_engine.reverse_semantics.len());
    println!();
    
    // Analyze rarity distribution
    let mut rarity_counts = std::collections::HashMap::new();
    for stanza in universe.stanzas.values() {
        *rarity_counts.entry(format!("{:?}", stanza.rarity)).or_insert(0) += 1;
    }
    
    println!("🎯 Rarity Distribution:");
    for (rarity, count) in rarity_counts {
        println!("  {}: {}", rarity, count);
    }
    println!();
    
    // Show quine statistics
    let quine_count = universe.stanzas.values().filter(|s| s.is_quine).count();
    println!("🌀 Self-Replication:");
    println!("  Quine Stanzas: {}", quine_count);
    println!("  Regular Stanzas: {}", universe.stanzas.len() - quine_count);
    
    Ok(())
}