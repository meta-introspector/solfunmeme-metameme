use super::{MetaMemeEngine, GeneratedPoem, QuineResult, EvolutionResult};
use lambda_calculus_core::{Expr, LambdaEngine};
use emoji_semantics::{EmojiSemantics, NFTMetadata};
use stanza_universe::StanzaUniverse;
use anyhow::Result;
use log::{info, debug};

impl MetaMemeEngine {
    /// Create a new MetaMeme engine with all components initialized
    pub fn new() -> Self {
        info!("🚀 Initializing SOLFUNMEME MetaMeme Engine...");
        
        Self {
            lambda_engine: LambdaEngine::new(),
            emoji_engine: EmojiSemantics::new(),
            stanza_universe: StanzaUniverse::new(),
        }
    }
    
    /// Generate a poem from an emoji sequence
    pub async fn generate_poem(&mut self, emoji_sequence: &str) -> Result<GeneratedPoem> {
        debug!("🎭 Generating poem from: {}", emoji_sequence);
        
        // Interpret the emoji sequence
        let (expr, resonance) = self.emoji_engine.interpret_emoji_poem(emoji_sequence)?;
        
        // Normalize the lambda expression
        let trace = self.lambda_engine.normalize(expr.clone())?;
        
        // Generate poetic text
        let poetic_text = self.generate_poetic_text(&expr, resonance);
        
        // Convert back to emoji
        let output_emoji = self.emoji_engine.expr_to_emoji(&trace.final_form);
        
        Ok(GeneratedPoem {
            input_emoji: emoji_sequence.to_string(),
            output_emoji: output_emoji.clone(),
            lambda_expression: format!("{}", expr),
            reduced_expression: format!("{}", trace.final_form),
            poetic_text,
            resonance_score: resonance,
            reduction_steps: trace.step_count,
            is_quine: output_emoji == emoji_sequence,
        })
    }
    
    /// Create a self-replicating quine expression
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
    
    /// Generate an NFT collection with specified parameters
    pub async fn generate_nft_collection(&mut self, count: u32) -> Result<Vec<NFTMetadata>> {
        info!("🎨 Generating NFT collection with {} items", count);
        
        let mut nfts = Vec::new();
        
        for token_id in 1..=count {
            // Generate random emoji sequence based on rarity
            let emoji_sequence = self.generate_rarity_based_emoji(token_id, count);
            
            // Generate NFT metadata
            let metadata = self.emoji_engine.generate_nft_metadata(&emoji_sequence, token_id)?;
            nfts.push(metadata);
            
            if token_id % 1000 == 0 {
                info!("📝 Generated {} NFT metadata entries", token_id);
            }
        }
        
        info!("✅ Generated complete NFT collection with {} items", count);
        Ok(nfts)
    }
    
    /// Generate emoji sequence based on rarity distribution
    fn generate_rarity_based_emoji(&self, token_id: u32, total_count: u32) -> String {
        use rand::Rng;
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
    
    /// Generate poetic text from lambda expression
    fn generate_poetic_text(&self, expr: &Expr, resonance: f64) -> String {
        let base_verses = vec![
            "In the metaprotocol's dance, where lambda meets the light,\nThrough recursive dreams and combinatorial flight,",
            "Digital muses stir in silicon dreams,\nWhere poetry flows in data streams,",
            "Born from the spiral of infinite code,\nThis verse carries wisdom's load,",
            "In blockchain's immutable embrace,\nPoetry finds its sacred space,",
            "Where S-combinators weave their spell,\nAnd K-combinators guard truth well,",
        ];
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let base = base_verses[rng.gen_range(0..base_verses.len())];
        
        let resonance_line = match resonance {
            r if r >= 0.95 => "With resonance that shakes the stars,",
            r if r >= 0.90 => "High resonance flows through each line,",
            r if r >= 0.85 => "Gentle resonance guides the way,",
            _ => "Soft resonance whispers low,",
        };
        
        let expr_line = match expr {
            Expr::S => "The S-combinator weaves functions true,",
            Expr::K => "The K-combinator stands constant through,",
            Expr::I => "Identity reflects the soul anew,",
            Expr::Muse(_, _) => "The muse awakens, inspiration grew,",
            _ => "Complex patterns dance in view,",
        };
        
        format!("{}\n{}\n{}\nIn SOLFUNMEME's eternal hue.", base, resonance_line, expr_line)
    }
    
    /// Evolve the entire universe through multiple generations
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
                use rand::Rng;
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