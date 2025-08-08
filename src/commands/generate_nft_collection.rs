use anyhow::Result;
use log::info;
use std::path::Path;
use emoji_semantics::EmojiSemantics;
use rand::Rng;
use emoji_semantics::NFTMetadata;

pub async fn generate_nft_collection(count: u32, output_dir: &Path, min_resonance: f64) -> Result<()> {
    info!("🎨 Generating {} NFTs with min resonance {:.3}", count, min_resonance);
    
    std::fs::create_dir_all(output_dir)?;
    
    let mut emoji_engine = EmojiSemantics::new();
    
    for token_id in 1..=count {
        // Generate random high-resonance emoji sequence
        let emoji_length = rand::thread_rng().gen_range(3..=8);
        let emoji_sequence = emoji_engine.generate_random_poem(emoji_length, min_resonance);
        
        let metadata = emoji_engine.generate_nft_metadata(&emoji_sequence, token_id)?;
        
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        let filename = format!("{}.json", token_id);
        let filepath = output_dir.join(filename);
        
        std::fs::write(&filepath, metadata_json)?;
        
        if token_id % 100 == 0 {
            info!("📝 Generated {} NFT metadata files", token_id);
        }
    }
    
    info!("✅ Generated {} NFT metadata files in {}", count, output_dir.display());
    Ok(())
}