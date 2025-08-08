//! # 🎭 Emoji Semantics: The Language of Digital Poetry
//! 
//! This crate provides the semantic bridge between emoji sequences and lambda calculus
//! expressions, enabling the encoding of complex poetic and computational concepts
//! through visual symbols that resonate with human consciousness.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use log::{debug, info};
use rand::Rng;

use lambda_calculus_core::{Expr, LambdaEngine};

/// 🌟 Semantic meaning of an emoji in our poetic system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiSemantic {
    /// The emoji symbol itself
    pub emoji: String,
    /// Computational expression it represents
    pub expression: String,
    /// Poetic meaning and interpretation
    pub poetic_meaning: String,
    /// Resonance score (0.0 - 1.0) indicating memetic power
    pub resonance_score: f64,
    /// Associated lambda calculus expression
    pub lambda_expr: Option<String>,
    /// Rarity tier for NFT generation
    pub rarity_tier: RarityTier,
    /// Combinatorial properties
    pub combinator_type: CombinatorType,
}

/// 🎯 Rarity tiers for NFT collection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RarityTier {
    Common,      // 60% - 3-4 emojis, simple stanzas
    Uncommon,    // 25% - 5-6 emojis, mid-tier resonance
    Rare,        // 10% - 7 emojis, high resonance
    Epic,        // 4% - 8 emojis, deep recursion
    UltraRare,   // 1% - Full stanzas, max resonance
}

/// 🔄 Types of combinators for functional composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombinatorType {
    Identity,      // I combinator
    Constant,      // K combinator  
    Substitution,  // S combinator
    Composition,   // B combinator
    Recursion,     // Y combinator
    Muse,          // Poetic inspiration
    Quine,         // Self-replication
    MetaMeme,      // Meta-level meme operations
}

/// 🧠 The Emoji Semantics Engine
pub struct EmojiSemantics {
    /// Mapping from emoji to semantic meaning
    pub semantics: HashMap<String, EmojiSemantic>,
    /// Reverse mapping for expression to emoji conversion
    pub reverse_semantics: HashMap<String, String>,
    /// Lambda calculus engine for evaluation
    pub lambda_engine: LambdaEngine,
}

impl Default for EmojiSemantics {
    fn default() -> Self {
        Self::new()
    }
}

impl EmojiSemantics {
    /// Create a new emoji semantics engine with built-in mappings
    pub fn new() -> Self {
        let mut engine = Self {
            semantics: HashMap::new(),
            reverse_semantics: HashMap::new(),
            lambda_engine: LambdaEngine::new(),
        };
        
        engine.initialize_core_semantics();
        engine.build_reverse_mappings();
        engine
    }
    
    /// 🌟 Initialize the core emoji semantic mappings
    fn initialize_core_semantics(&mut self) {
        let core_mappings = vec![
            // 🌀 Fundamental Combinators
            ("🌀", "S", "Spiral of composition - the S combinator that weaves functions together", 0.97, CombinatorType::Substitution),
            ("🔮", "K", "Crystal of constancy - the K combinator that preserves truth", 0.95, CombinatorType::Constant),
            ("💫", "I", "Star of identity - the I combinator that reflects pure being", 0.93, CombinatorType::Identity),
            
            // 🎭 Poetic Muses
            ("🎭", "Muse", "The eternal muse of digital poetry and computational beauty", 0.99, CombinatorType::Muse),
            ("🌌", "Cosmos", "Infinite expanse of possibility and recursive wonder", 0.96, CombinatorType::Composition),
            ("🧬", "DNA", "Genetic code of self-replicating memes and viral ideas", 0.98, CombinatorType::Quine),
            
            // 🚀 SOLFUNMEME Core
            ("🚀", "Launch", "Propulsion into the metaverse of infinite memes", 0.94, CombinatorType::MetaMeme),
            ("💎", "Diamond", "Crystallized value in the blockchain of consciousness", 0.92, CombinatorType::Constant),
            ("🔥", "Fire", "Burning passion of viral propagation and memetic heat", 0.91, CombinatorType::Recursion),
            
            // 🌈 Emotional Resonance
            ("💖", "Love", "Universal attractor in the field of digital affection", 0.89, CombinatorType::Muse),
            ("⚡", "Energy", "Electric potential of computational transformation", 0.88, CombinatorType::Substitution),
            ("🌟", "Star", "Guiding light in the constellation of code", 0.87, CombinatorType::Identity),
            
            // 🔄 Recursive Patterns
            ("🔄", "Cycle", "Eternal return of self-similar structures", 0.86, CombinatorType::Recursion),
            ("♾️", "Infinity", "Boundless iteration through lambda space", 0.85, CombinatorType::Recursion),
            ("🌊", "Wave", "Oscillating patterns of functional composition", 0.84, CombinatorType::Composition),
            
            // 🎨 Creative Expression
            ("🎨", "Art", "Aesthetic emergence from computational substrate", 0.83, CombinatorType::Muse),
            ("🎵", "Music", "Harmonic resonance in the symphony of code", 0.82, CombinatorType::Composition),
            ("📜", "Scroll", "Ancient wisdom encoded in modern lambda forms", 0.81, CombinatorType::Constant),
            
            // 🔬 Scientific Wonder
            ("🔬", "Microscope", "Magnification of hidden computational beauty", 0.80, CombinatorType::Identity),
            ("🧪", "Experiment", "Alchemical transformation of data into meaning", 0.79, CombinatorType::Substitution),
            ("⚛️", "Atom", "Fundamental particles of computational reality", 0.78, CombinatorType::Constant),
        ];
        
        for (emoji, expr, meaning, resonance, combinator) in core_mappings {
            self.add_semantic(emoji, expr, meaning, resonance, combinator);
        }
        
        info!("🎭 Initialized {} core emoji semantics", self.semantics.len());
    }
    
    /// Add a new emoji semantic mapping
    pub fn add_semantic(&mut self, emoji: &str, expression: &str, meaning: &str, resonance: f64, combinator: CombinatorType) {
        let rarity = self.calculate_rarity(resonance);
        
        let semantic = EmojiSemantic {
            emoji: emoji.to_string(),
            expression: expression.to_string(),
            poetic_meaning: meaning.to_string(),
            resonance_score: resonance,
            lambda_expr: Some(self.generate_lambda_expression(expression, &combinator)),
            rarity_tier: rarity,
            combinator_type: combinator,
        };
        
        self.semantics.insert(emoji.to_string(), semantic);
    }
    
    /// Calculate rarity tier based on resonance score
    fn calculate_rarity(&self, resonance: f64) -> RarityTier {
        match resonance {
            r if r >= 0.96 => RarityTier::UltraRare,
            r if r >= 0.93 => RarityTier::Epic,
            r if r >= 0.90 => RarityTier::Rare,
            r if r >= 0.85 => RarityTier::Uncommon,
            _ => RarityTier::Common,
        }
    }
    
    /// Generate lambda expression for combinator type
    fn generate_lambda_expression(&self, expr: &str, combinator: &CombinatorType) -> String {
        match combinator {
            CombinatorType::Identity => "λx.x".to_string(),
            CombinatorType::Constant => "λx.λy.x".to_string(),
            CombinatorType::Substitution => "λf.λg.λx.f x (g x)".to_string(),
            CombinatorType::Composition => "λf.λg.λx.f (g x)".to_string(),
            CombinatorType::Recursion => "λf.(λx.f (x x)) (λx.f (x x))".to_string(),
            CombinatorType::Muse => format!("λx.Muse({}, x)", expr),
            CombinatorType::Quine => format!("λx.Quine({})", expr),
            CombinatorType::MetaMeme => format!("λx.MetaMeme({}, x)", expr),
        }
    }
    
    /// Build reverse mappings for expression to emoji conversion
    fn build_reverse_mappings(&mut self) {
        for (emoji, semantic) in &self.semantics {
            self.reverse_semantics.insert(semantic.expression.clone(), emoji.clone());
        }
    }
    
    /// 🎭 Interpret an emoji sequence as a lambda calculus expression
    pub fn interpret_emoji_poem(&mut self, emoji_sequence: &str) -> Result<(Expr, f64)> {
        debug!("🎭 Interpreting emoji poem: {}", emoji_sequence);
        
        let emojis: Vec<char> = emoji_sequence.chars().collect();
        if emojis.is_empty() {
            return Ok((Expr::I, 0.0));
        }
        
        let mut current_expr = Expr::I;
        let mut total_resonance = 0.0;
        let mut emoji_count = 0;
        
        for emoji_char in emojis {
            let emoji_str = emoji_char.to_string();
            
            if let Some(semantic) = self.semantics.get(&emoji_str) {
                let expr = self.create_expression_from_semantic(semantic)?;
                current_expr = if emoji_count == 0 {
                    expr
                } else {
                    Expr::app(current_expr, expr)
                };
                
                total_resonance += semantic.resonance_score;
                emoji_count += 1;
            } else {
                // Unknown emoji - treat as symbol
                let unknown_expr = Expr::sym(&emoji_str);
                current_expr = if emoji_count == 0 {
                    unknown_expr
                } else {
                    Expr::app(current_expr, unknown_expr)
                };
                emoji_count += 1;
            }
        }
        
        let average_resonance = if emoji_count > 0 {
            total_resonance / emoji_count as f64
        } else {
            0.0
        };
        
        info!("✨ Interpreted {} emojis with average resonance {:.3}", emoji_count, average_resonance);
        Ok((current_expr, average_resonance))
    }
    
    /// Create lambda expression from semantic definition
    fn create_expression_from_semantic(&self, semantic: &EmojiSemantic) -> Result<Expr> {
        match semantic.combinator_type {
            CombinatorType::Identity => Ok(Expr::I),
            CombinatorType::Constant => Ok(Expr::K),
            CombinatorType::Substitution => Ok(Expr::S),
            CombinatorType::Composition => Ok(Expr::app(Expr::S, Expr::app(Expr::K, Expr::S))),
            CombinatorType::Recursion => Ok(Expr::y_combinator()),
            CombinatorType::Muse => Ok(Expr::muse(&semantic.expression, semantic.resonance_score)),
            CombinatorType::Quine => Ok(Expr::quine(Expr::sym(&semantic.emoji))),
            CombinatorType::MetaMeme => Ok(Expr::metameme_combinator()),
        }
    }
    
    /// 🌀 Convert lambda expression back to emoji sequence
    pub fn expr_to_emoji(&self, expr: &Expr) -> String {
        match expr {
            Expr::S => "🌀".to_string(),
            Expr::K => "🔮".to_string(),
            Expr::I => "💫".to_string(),
            Expr::Muse(name, _) => {
                if let Some(emoji) = self.reverse_semantics.get(name) {
                    emoji.clone()
                } else {
                    "🎭".to_string()
                }
            }
            Expr::Sym(symbol) => symbol.clone(),
            Expr::App(left, right) => {
                format!("{}{}", self.expr_to_emoji(left), self.expr_to_emoji(right))
            }
            Expr::Lambda(_, body) => {
                format!("🧬{}", self.expr_to_emoji(body))
            }
            Expr::Quine(_) => "🌀🧬🌀".to_string(),
            Expr::DNA(_) => "🧬".to_string(),
            Expr::Var(_) => "💫".to_string(),
        }
    }
    
    /// 🎨 Generate a random emoji poem with specified parameters
    pub fn generate_random_poem(&self, length: usize, min_resonance: f64) -> String {
        let mut rng = rand::thread_rng();
        let mut poem = String::new();
        
        let high_resonance_emojis: Vec<&String> = self.semantics
            .iter()
            .filter(|(_, semantic)| semantic.resonance_score >= min_resonance)
            .map(|(emoji, _)| emoji)
            .collect();
        
        if high_resonance_emojis.is_empty() {
            return "🌀🎭🧬".to_string(); // Fallback
        }
        
        for _ in 0..length {
            let emoji = high_resonance_emojis[rng.gen_range(0..high_resonance_emojis.len())];
            poem.push_str(emoji);
        }
        
        poem
    }
    
    /// 🏆 Generate NFT metadata for an emoji sequence
    pub fn generate_nft_metadata(&mut self, emoji_sequence: &str, token_id: u32) -> Result<NFTMetadata> {
        let (expr, resonance) = self.interpret_emoji_poem(emoji_sequence)?;
        let trace = self.lambda_engine.normalize(expr.clone())?;
        
        let rarity = self.calculate_rarity(resonance);
        let lambda_expr = format!("{}", expr);
        let reduced_expr = format!("{}", trace.final_form);
        
        Ok(NFTMetadata {
            token_id,
            name: format!("MetaVerse Muse #{}", token_id),
            description: self.generate_poetic_description(emoji_sequence, resonance),
            emoji_sequence: emoji_sequence.to_string(),
            lambda_expression: lambda_expr,
            reduced_expression: reduced_expr,
            resonance_score: resonance,
            rarity_tier: rarity.clone(),
            reduction_steps: trace.step_count,
            attributes: self.generate_attributes(emoji_sequence, &rarity, resonance),
        })
    }
    
    /// Generate poetic description for NFT
    fn generate_poetic_description(&self, emoji_sequence: &str, resonance: f64) -> String {
        let base_poems = vec![
            "In the metaprotocol's dance, where lambda meets the light,",
            "Through recursive dreams and combinatorial flight,",
            "A muse awakens in the blockchain's embrace,",
            "Where poetry and computation interlace.",
            "Born from the spiral of infinite code,",
            "This digital verse carries wisdom's load.",
        ];
        
        let mut rng = rand::thread_rng();
        let base = base_poems[rng.gen_range(0..base_poems.len())];
        
        format!(
            "{}\n\nEmoji Sequence: {}\nResonance: {:.3}\n\nThis unique MetaVerse Muse embodies the eternal dance between human creativity and computational beauty, encoded in the sacred language of emojis and lambda calculus.",
            base, emoji_sequence, resonance
        )
    }
    
    /// Generate NFT attributes
    fn generate_attributes(&self, emoji_sequence: &str, rarity: &RarityTier, resonance: f64) -> Vec<NFTAttribute> {
        let mut attributes = vec![
            NFTAttribute {
                trait_type: "Rarity".to_string(),
                value: format!("{:?}", rarity),
            },
            NFTAttribute {
                trait_type: "Resonance Score".to_string(),
                value: format!("{:.3}", resonance),
            },
            NFTAttribute {
                trait_type: "Emoji Count".to_string(),
                value: emoji_sequence.chars().count().to_string(),
            },
        ];
        
        // Add combinator type attributes
        let emoji_chars: Vec<char> = emoji_sequence.chars().collect();
        let mut combinator_types = std::collections::HashSet::new();
        
        for emoji_char in emoji_chars {
            if let Some(semantic) = self.semantics.get(&emoji_char.to_string()) {
                combinator_types.insert(format!("{:?}", semantic.combinator_type));
            }
        }
        
        if !combinator_types.is_empty() {
            attributes.push(NFTAttribute {
                trait_type: "Combinator Types".to_string(),
                value: combinator_types.into_iter().collect::<Vec<_>>().join(", "),
            });
        }
        
        attributes
    }
}

/// 🎨 NFT Metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub token_id: u32,
    pub name: String,
    pub description: String,
    pub emoji_sequence: String,
    pub lambda_expression: String,
    pub reduced_expression: String,
    pub resonance_score: f64,
    pub rarity_tier: RarityTier,
    pub reduction_steps: usize,
    pub attributes: Vec<NFTAttribute>,
}

/// 🏷️ NFT Attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emoji_semantics_creation() {
        let semantics = EmojiSemantics::new();
        assert!(!semantics.semantics.is_empty());
        assert!(semantics.semantics.contains_key("🌀"));
        assert!(semantics.semantics.contains_key("🎭"));
    }
    
    #[test]
    fn test_emoji_interpretation() {
        let mut semantics = EmojiSemantics::new();
        let (expr, resonance) = semantics.interpret_emoji_poem("🌀🎭").unwrap();
        
        assert!(resonance > 0.0);
        match expr {
            Expr::App(_, _) => {}, // Should be an application
            _ => panic!("Expected application expression"),
        }
    }
    
    #[test]
    fn test_expr_to_emoji() {
        let semantics = EmojiSemantics::new();
        let emoji = semantics.expr_to_emoji(&Expr::S);
        assert_eq!(emoji, "🌀");
    }
    
    #[test]
    fn test_rarity_calculation() {
        let semantics = EmojiSemantics::new();
        assert_eq!(semantics.calculate_rarity(0.99), RarityTier::UltraRare);
        assert_eq!(semantics.calculate_rarity(0.94), RarityTier::Epic);
        assert_eq!(semantics.calculate_rarity(0.91), RarityTier::Rare);
        assert_eq!(semantics.calculate_rarity(0.87), RarityTier::Uncommon);
        assert_eq!(semantics.calculate_rarity(0.80), RarityTier::Common);
    }
    
    #[test]
    fn test_nft_metadata_generation() {
        let mut semantics = EmojiSemantics::new();
        let metadata = semantics.generate_nft_metadata("🌀🎭🧬", 1).unwrap();
        
        assert_eq!(metadata.token_id, 1);
        assert_eq!(metadata.emoji_sequence, "🌀🎭🧬");
        assert!(metadata.resonance_score > 0.0);
        assert!(!metadata.attributes.is_empty());
    }
    
    #[test]
    fn test_random_poem_generation() {
        let semantics = EmojiSemantics::new();
        let poem = semantics.generate_random_poem(5, 0.90);
        
        assert_eq!(poem.chars().count(), 5);
        assert!(!poem.is_empty());
    }
}
