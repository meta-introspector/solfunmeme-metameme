//! # 🚀 SOLFUNMEME MetaMeme: The Ultimate Self-Replicating Poetry Engine
//! 
//! This crate implements the SOLFUNMEME MetaMeme system - a revolutionary fusion of:
//! - 🧬 Self-replicating lambda calculus expressions
//! - 🎭 Emoji-encoded semantic poetry
//! - 🌀 S-combinator based functional composition
//! - 🎨 9,901 NFT collection generation
//! - ⛓️ Solana blockchain deployment
//! 
//! ## Core Philosophy
//! 
//! SOLFUNMEME represents the convergence of computation and creativity, where:
//! - Every emoji encodes a lambda calculus expression
//! - Every expression can generate poetic verse
//! - Every verse can self-replicate and evolve
//! - Every evolution creates new NFT possibilities
//! 
//! ## Architecture
//! 
//! The system is built on several interconnected crates:
//! 
//! - **lambda-calculus-core**: The fundamental expression engine
//! - **emoji-semantics**: Emoji to lambda calculus translation
//! - **stanza-universe**: Poetic generation and evolution
//! - **solana-programs**: Blockchain deployment infrastructure
//! - **nft-collection**: NFT metadata and minting
//! - **metameme-engine**: High-level orchestration
//! 
//! ## Usage
//! 
//! ```rust
//! use solfunmeme_metameme::*;
//! 
//! // Create a new MetaMeme engine
//! let mut engine = MetaMemeEngine::new();
//! 
//! // Generate poetry from emojis
//! let poem = engine.generate_poem("🌀🎭🧬").await?;
//! 
//! // Create self-replicating expressions
//! let quine = engine.create_quine("🌀").await?;
//! 
//! // Generate NFT collection
//! let nfts = engine.generate_nft_collection(100).await?;
//! ```

pub use lambda_calculus_core::{Expr, LambdaEngine, ReductionTrace};
pub use emoji_semantics::{EmojiSemantics, EmojiSemantic, RarityTier, CombinatorType, NFTMetadata};
pub use stanza_universe::{StanzaUniverse, Stanza};

use anyhow::Result;
use serde::{Serialize, Deserialize};
use log::{info, debug};

/// 🌟 The main MetaMeme engine that orchestrates all components
pub struct MetaMemeEngine {
    /// Lambda calculus computation engine
    pub lambda_engine: LambdaEngine,
    /// Emoji semantic interpretation engine
    pub emoji_engine: EmojiSemantics,
    /// Stanza universe for poetry generation
    pub stanza_universe: StanzaUniverse,
}

impl Default for MetaMemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

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
            output_emoji,
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

/// 🧬 Result of universe evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionResult {
    pub initial_stanza_count: usize,
    pub final_stanza_count: usize,
    pub new_stanzas_created: usize,
    pub generations_completed: u32,
    pub evolved_stanza_ids: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_metameme_engine_creation() {
        let engine = MetaMemeEngine::new();
        assert!(!engine.emoji_engine.semantics.is_empty());
        assert!(!engine.stanza_universe.stanzas.is_empty());
    }
    
    #[tokio::test]
    async fn test_poem_generation() {
        let mut engine = MetaMemeEngine::new();
        let poem = engine.generate_poem("🌀🎭").await.unwrap();
        
        assert_eq!(poem.input_emoji, "🌀🎭");
        assert!(poem.resonance_score > 0.0);
        assert!(!poem.poetic_text.is_empty());
    }
    
    #[tokio::test]
    async fn test_quine_creation() {
        let mut engine = MetaMemeEngine::new();
        let quine = engine.create_quine("🌀").await.unwrap();
        
        assert_eq!(quine.seed, "🌀");
        assert!(!quine.original_expression.is_empty());
        assert!(quine.reduction_steps >= 0);
    }
    
    #[tokio::test]
    async fn test_nft_collection_generation() {
        let mut engine = MetaMemeEngine::new();
        let nfts = engine.generate_nft_collection(10).await.unwrap();
        
        assert_eq!(nfts.len(), 10);
        assert!(nfts.iter().all(|nft| !nft.emoji_sequence.is_empty()));
    }
}
