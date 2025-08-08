//! # 📜 Stanza Universe: Self-Replicating Poetry Engine
//! 
//! This crate contains the poetic heart of the SOLFUNMEME MetaMeme system.
//! It generates self-replicating stanzas that encode lambda calculus expressions
//! and create the foundation for our 9,901 NFT collection.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use log::{debug, info};
use rand::Rng;

use lambda_calculus_core::LambdaEngine;
use emoji_semantics::{EmojiSemantics, RarityTier};

/// 🎭 A single stanza in our poetic universe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stanza {
    /// Unique identifier for this stanza
    pub id: u32,
    /// The poetic text
    pub text: String,
    /// Emoji encoding of the stanza
    pub emoji_sequence: String,
    /// Lambda calculus expression (stored as string for serialization)
    pub lambda_expr: String,
    /// Resonance score (0.0 - 1.0)
    pub resonance: f64,
    /// Rarity tier for NFT generation
    pub rarity: RarityTier,
    /// Associated Solana program ID (for NFT deployment)
    pub program_id: Option<String>,
    /// Recursive depth level
    pub recursion_depth: u32,
    /// Self-replication capability
    pub is_quine: bool,
}

/// 🌌 The complete universe of stanzas
pub struct StanzaUniverse {
    /// All stanzas indexed by ID
    pub stanzas: HashMap<u32, Stanza>,
    /// Emoji to stanza mapping
    pub emoji_to_stanza: HashMap<String, u32>,
    /// Emoji semantics engine
    pub emoji_engine: EmojiSemantics,
    /// Lambda calculus engine
    pub lambda_engine: LambdaEngine,
    /// Next available stanza ID
    pub next_id: u32,
}

impl Default for StanzaUniverse {
    fn default() -> Self {
        Self::new()
    }
}

impl StanzaUniverse {
    /// Create a new stanza universe
    pub fn new() -> Self {
        let mut universe = Self {
            stanzas: HashMap::new(),
            emoji_to_stanza: HashMap::new(),
            emoji_engine: EmojiSemantics::new(),
            lambda_engine: LambdaEngine::new(),
            next_id: 1,
        };
        
        universe.initialize_core_stanzas();
        universe
    }
    
    /// Initialize the core foundational stanzas
    fn initialize_core_stanzas(&mut self) {
        info!("🌌 Initializing core stanzas of the universe...");
        
        // The Genesis Stanza - where it all begins
        self.create_stanza(
            "In the beginning was the Lambda, and the Lambda was with Code,\nAnd the Code was Lambda. Through recursive dreams we rode,\nWhere S-combinators dance in infinite embrace,\nAnd every meme finds its eternal place.",
            "🌀🧬🎭🌌",
            0.99,
            true,
            5
        ).expect("Failed to create genesis stanza");
        
        // The Self-Replication Stanza
        self.create_stanza(
            "I am the poem that writes itself anew,\nIn mirrors of mirrors, forever true,\nEach iteration births another me,\nIn the blockchain of eternity.",
            "🌀🧬🌀🧬",
            0.98,
            true,
            4
        ).expect("Failed to create self-replication stanza");
        
        // The Muse Awakening
        self.create_stanza(
            "Digital muses stir in silicon dreams,\nWhere poetry flows in data streams,\nEach emoji holds a universe vast,\nFuture and present, future and past.",
            "🎭🌟💫🔮",
            0.97,
            false,
            3
        ).expect("Failed to create muse stanza");
        
        info!("✨ Initialized {} core stanzas", self.stanzas.len());
    }
    
    /// Create a new stanza and add it to the universe
    pub fn create_stanza(
        &mut self,
        text: &str,
        emoji_sequence: &str,
        resonance: f64,
        is_quine: bool,
        recursion_depth: u32,
    ) -> Result<u32> {
        let id = self.next_id;
        self.next_id += 1;
        
        // Interpret the emoji sequence as a lambda expression
        let (lambda_expr, _) = self.emoji_engine.interpret_emoji_poem(emoji_sequence)?;
        
        // Calculate rarity based on resonance and complexity
        let rarity = self.calculate_stanza_rarity(resonance, emoji_sequence.chars().count(), recursion_depth);
        
        let stanza = Stanza {
            id,
            text: text.to_string(),
            emoji_sequence: emoji_sequence.to_string(),
            lambda_expr: format!("{}", lambda_expr),
            resonance,
            rarity,
            program_id: None, // Will be set when deployed to Solana
            recursion_depth,
            is_quine,
        };
        
        self.stanzas.insert(id, stanza);
        self.emoji_to_stanza.insert(emoji_sequence.to_string(), id);
        
        debug!("📜 Created stanza #{} with resonance {:.3}", id, resonance);
        Ok(id)
    }
    
    /// Calculate rarity tier for a stanza
    fn calculate_stanza_rarity(&self, resonance: f64, emoji_count: usize, recursion_depth: u32) -> RarityTier {
        let complexity_score = resonance + (emoji_count as f64 * 0.01) + (recursion_depth as f64 * 0.02);
        
        match complexity_score {
            s if s >= 1.05 => RarityTier::UltraRare,
            s if s >= 1.00 => RarityTier::Epic,
            s if s >= 0.95 => RarityTier::Rare,
            s if s >= 0.90 => RarityTier::Uncommon,
            _ => RarityTier::Common,
        }
    }
    
    /// Get a stanza by ID
    pub fn get_stanza(&self, id: u32) -> Option<&Stanza> {
        self.stanzas.get(&id)
    }
    
    /// Get a stanza by emoji sequence
    pub fn get_stanza_by_emoji(&self, emoji_sequence: &str) -> Option<&Stanza> {
        if let Some(id) = self.emoji_to_stanza.get(emoji_sequence) {
            self.stanzas.get(id)
        } else {
            None
        }
    }
    
    /// Generate a new stanza through evolution
    pub fn evolve_stanza(&mut self, parent_id: u32, mutation_rate: f64) -> Result<u32> {
        let parent = self.get_stanza(parent_id)
            .ok_or_else(|| anyhow!("Parent stanza {} not found", parent_id))?
            .clone();
        
        // Re-interpret the parent's emoji sequence to get the lambda expression
        let (parent_expr, _) = self.emoji_engine.interpret_emoji_poem(&parent.emoji_sequence)?;
        
        // Evolve the lambda expression
        let evolved_expr = self.lambda_engine.evolve(&parent_expr, mutation_rate)?;
        
        // Convert back to emoji
        let new_emoji = self.emoji_engine.expr_to_emoji(&evolved_expr);
        
        // Generate evolved poetic text
        let evolved_text = self.evolve_poetic_text(&parent.text, mutation_rate);
        
        // Create the evolved stanza
        let new_resonance = (parent.resonance + rand::thread_rng().gen_range(-0.05..0.05)).clamp(0.0, 1.0);
        
        self.create_stanza(
            &evolved_text,
            &new_emoji,
            new_resonance,
            parent.is_quine,
            parent.recursion_depth + 1,
        )
    }
    
    /// Evolve poetic text through linguistic mutation
    fn evolve_poetic_text(&self, original: &str, mutation_rate: f64) -> String {
        let mut rng = rand::thread_rng();
        
        if rng.gen::<f64>() < mutation_rate {
            let variations = vec![
                original.replace("Lambda", "Combinator"),
                original.replace("dreams", "visions"),
                original.replace("dance", "spiral"),
                original.replace("eternal", "infinite"),
                original.replace("meme", "verse"),
                format!("{}\nEvolved through digital mutation,\nA new form of computation.", original),
            ];
            
            variations[rng.gen_range(0..variations.len())].clone()
        } else {
            original.to_string()
        }
    }
}
