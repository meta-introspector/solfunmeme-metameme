use crate::engine::MetaMemeEngine;
use rand::Rng;

impl MetaMemeEngine {
    fn generate_rarity_based_emoji(&self, token_id: u32, total_count: u32) -> String {
        let mut rng = rand::thread_rng();
        
        // Calculate rarity based on token position
        let rarity_percentile = (token_id as f64) / (total_count as f64);
        
        let (emoji_length, min_resonance) = match rarity_percentile {
            p if p >= 0.99 => (8, 0.96), // Ultra-rare: 1%
            p if p >= 0.96 => (7, 0.93), // Epic: 4%
            p if p >= 0.90 => (6, 0.90), // Rare: 10%
            p if p >= 0.75 => (5, 0.85), // Uncommon: 25%
            _ => (rng.gen_range(3..=4), 0.80), // Common: 60%
        };
        
        self.emoji_engine.generate_random_poem(emoji_length, min_resonance)
    }
}