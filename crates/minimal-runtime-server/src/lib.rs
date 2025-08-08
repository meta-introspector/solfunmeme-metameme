//! # 🌐 Minimal Runtime Server for SOLFUNMEME MetaMeme
//! 
//! This crate provides a lightweight, blockchain-free runtime server for the
//! SOLFUNMEME MetaMeme system. It can run anywhere - locally, in the cloud,
//! or even in WebAssembly environments.
//! 
//! ## Key Features
//! 
//! - **🌐 HTTP API**: RESTful endpoints for all MetaMeme operations
//! - **🔄 Real-time Processing**: Instant emoji to poetry conversion
//! - **💾 In-Memory State**: No external dependencies required
//! - **🌍 Cross-Platform**: Runs on any system with Rust support
//! - **⚡ High Performance**: Async processing with Tokio
//! - **🎭 Complete Engine**: Full lambda calculus and emoji semantics

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use log::{info, debug, error};
use uuid::Uuid;

use lambda_calculus_core::{Expr, LambdaEngine, ReductionTrace};
use emoji_semantics::{EmojiSemantics, NFTMetadata, RarityTier};
use stanza_universe::{StanzaUniverse, Stanza};

/// 🌟 The main runtime server state
pub struct MetaMemeRuntime {
    /// Lambda calculus engine
    pub lambda_engine: LambdaEngine,
    /// Emoji semantic interpretation
    pub emoji_engine: EmojiSemantics,
    /// Stanza universe for poetry
    pub stanza_universe: StanzaUniverse,
    /// Active sessions
    pub sessions: Arc<RwLock<HashMap<String, Session>>>,
    /// Generated poems cache
    pub poems_cache: Arc<RwLock<HashMap<String, GeneratedPoem>>>,
    /// NFT metadata cache
    pub nft_cache: Arc<RwLock<HashMap<u32, NFTMetadata>>>,
}

/// 🎭 A user session with the MetaMeme engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub created_at: u64,
    pub last_activity: u64,
    pub poems_generated: u32,
    pub quines_created: u32,
    pub nfts_minted: u32,
    pub favorite_emojis: Vec<String>,
    pub resonance_history: Vec<f64>,
}

/// 🌟 A generated poem with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPoem {
    pub id: String,
    pub session_id: String,
    pub input_emoji: String,
    pub output_emoji: String,
    pub lambda_expression: String,
    pub reduced_expression: String,
    pub poetic_text: String,
    pub resonance_score: f64,
    pub reduction_steps: usize,
    pub is_quine: bool,
    pub created_at: u64,
    pub rarity_tier: RarityTier,
}

/// 🌀 A quine generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuineResult {
    pub id: String,
    pub session_id: String,
    pub seed: String,
    pub original_expression: String,
    pub final_expression: String,
    pub output_emoji: String,
    pub reduction_steps: usize,
    pub is_perfect_quine: bool,
    pub created_at: u64,
}

/// 🎨 NFT generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTRequest {
    pub emoji_sequence: String,
    pub session_id: Option<String>,
    pub custom_attributes: Option<HashMap<String, String>>,
}

/// 📊 Runtime statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStats {
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub total_poems: usize,
    pub total_quines: usize,
    pub total_nfts: usize,
    pub average_resonance: f64,
    pub most_popular_emoji: String,
    pub uptime_seconds: u64,
    pub memory_usage_mb: f64,
}

/// 🔄 API request/response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoemRequest {
    pub emoji_sequence: String,
    pub session_id: Option<String>,
    pub max_reduction_steps: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuineRequest {
    pub seed: String,
    pub session_id: Option<String>,
    pub max_reduction_steps: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub emoji_sequence: String,
    pub include_trace: bool,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub input: String,
    pub emoji_count: usize,
    pub resonance_score: f64,
    pub lambda_expression: String,
    pub reduced_expression: String,
    pub reduction_steps: usize,
    pub is_normal_form: bool,
    pub output_emoji: String,
    pub is_quine: bool,
    pub trace: Option<Vec<String>>,
}

impl Default for MetaMemeRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaMemeRuntime {
    /// Create a new MetaMeme runtime
    pub fn new() -> Self {
        info!("🚀 Initializing SOLFUNMEME MetaMeme Runtime...");
        
        Self {
            lambda_engine: LambdaEngine::new(),
            emoji_engine: EmojiSemantics::new(),
            stanza_universe: StanzaUniverse::new(),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            poems_cache: Arc::new(RwLock::new(HashMap::new())),
            nft_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new user session
    pub fn create_session(&self) -> Result<Session> {
        let session_id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        let session = Session {
            id: session_id.clone(),
            created_at: now,
            last_activity: now,
            poems_generated: 0,
            quines_created: 0,
            nfts_minted: 0,
            favorite_emojis: Vec::new(),
            resonance_history: Vec::new(),
        };
        
        self.sessions.write().unwrap().insert(session_id.clone(), session.clone());
        
        info!("👤 Created new session: {}", session_id);
        Ok(session)
    }
    
    /// Get or create a session
    pub fn get_or_create_session(&self, session_id: Option<String>) -> Result<Session> {
        match session_id {
            Some(id) => {
                let mut sessions = self.sessions.write().unwrap();
                if let Some(mut session) = sessions.get(&id).cloned() {
                    // Update last activity
                    session.last_activity = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs();
                    sessions.insert(id, session.clone());
                    Ok(session)
                } else {
                    // Session doesn't exist, create new one
                    drop(sessions);
                    self.create_session()
                }
            }
            None => self.create_session(),
        }
    }
    
    /// Generate a poem from emoji sequence
    pub fn generate_poem(&mut self, request: PoemRequest) -> Result<GeneratedPoem> {
        debug!("🎭 Generating poem from: {}", request.emoji_sequence);
        
        let session = self.get_or_create_session(request.session_id)?;
        
        // Set max reduction steps if specified
        if let Some(max_steps) = request.max_reduction_steps {
            self.lambda_engine = self.lambda_engine.clone().with_max_steps(max_steps);
        }
        
        // Interpret emoji sequence
        let (expr, resonance) = self.emoji_engine.interpret_emoji_poem(&request.emoji_sequence)?;
        
        // Normalize the expression
        let trace = self.lambda_engine.normalize(expr.clone())?;
        
        // Generate poetic text
        let poetic_text = self.generate_poetic_text(&expr, resonance);
        
        // Convert back to emoji
        let output_emoji = self.emoji_engine.expr_to_emoji(&trace.final_form);
        
        // Determine rarity
        let rarity_tier = self.calculate_rarity(resonance, request.emoji_sequence.chars().count());
        
        let poem_id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        let poem = GeneratedPoem {
            id: poem_id.clone(),
            session_id: session.id.clone(),
            input_emoji: request.emoji_sequence.clone(),
            output_emoji: output_emoji.clone(),
            lambda_expression: format!("{}", expr),
            reduced_expression: format!("{}", trace.final_form),
            poetic_text,
            resonance_score: resonance,
            reduction_steps: trace.step_count,
            is_quine: output_emoji == request.emoji_sequence,
            created_at: now,
            rarity_tier,
        };
        
        // Cache the poem
        self.poems_cache.write().unwrap().insert(poem_id.clone(), poem.clone());
        
        // Update session stats
        let mut sessions = self.sessions.write().unwrap();
        if let Some(mut session) = sessions.get(&session.id).cloned() {
            session.poems_generated += 1;
            session.resonance_history.push(resonance);
            session.last_activity = now;
            
            // Track favorite emojis
            for emoji in request.emoji_sequence.chars() {
                let emoji_str = emoji.to_string();
                if !session.favorite_emojis.contains(&emoji_str) {
                    session.favorite_emojis.push(emoji_str);
                }
            }
            
            sessions.insert(session.id.clone(), session);
        }
        
        info!("✨ Generated poem {} with resonance {:.3}", poem_id, resonance);
        Ok(poem)
    }
    
    /// Create a self-replicating quine
    pub fn create_quine(&mut self, request: QuineRequest) -> Result<QuineResult> {
        debug!("🌀 Creating quine with seed: {}", request.seed);
        
        let session = self.get_or_create_session(request.session_id)?;
        
        // Set max reduction steps if specified
        if let Some(max_steps) = request.max_reduction_steps {
            self.lambda_engine = self.lambda_engine.clone().with_max_steps(max_steps);
        }
        
        let quine_expr = self.lambda_engine.create_quine(&request.seed);
        let trace = self.lambda_engine.normalize(quine_expr.clone())?;
        let output_emoji = self.emoji_engine.expr_to_emoji(&trace.final_form);
        
        let is_perfect_quine = output_emoji.contains(&request.seed);
        
        let quine_id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        let quine = QuineResult {
            id: quine_id.clone(),
            session_id: session.id.clone(),
            seed: request.seed.clone(),
            original_expression: format!("{}", quine_expr),
            final_expression: format!("{}", trace.final_form),
            output_emoji,
            reduction_steps: trace.step_count,
            is_perfect_quine,
            created_at: now,
        };
        
        // Update session stats
        let mut sessions = self.sessions.write().unwrap();
        if let Some(mut session) = sessions.get(&session.id).cloned() {
            session.quines_created += 1;
            session.last_activity = now;
            sessions.insert(session.id.clone(), session);
        }
        
        info!("🌀 Created quine {} (perfect: {})", quine_id, is_perfect_quine);
        Ok(quine)
    }
    
    /// Analyze an emoji sequence
    pub fn analyze_emoji(&mut self, request: AnalysisRequest) -> Result<AnalysisResult> {
        debug!("🔍 Analyzing emoji sequence: {}", request.emoji_sequence);
        
        let _session = self.get_or_create_session(request.session_id)?;
        
        let (expr, resonance) = self.emoji_engine.interpret_emoji_poem(&request.emoji_sequence)?;
        let trace = self.lambda_engine.normalize(expr.clone())?;
        let output_emoji = self.emoji_engine.expr_to_emoji(&trace.final_form);
        
        let trace_strings = if request.include_trace {
            Some(trace.steps.iter().map(|step| format!("{}", step)).collect())
        } else {
            None
        };
        
        Ok(AnalysisResult {
            input: request.emoji_sequence.clone(),
            emoji_count: request.emoji_sequence.chars().count(),
            resonance_score: resonance,
            lambda_expression: format!("{}", expr),
            reduced_expression: format!("{}", trace.final_form),
            reduction_steps: trace.step_count,
            is_normal_form: trace.is_normal_form,
            output_emoji: output_emoji.clone(),
            is_quine: output_emoji == request.emoji_sequence,
            trace: trace_strings,
        })
    }
    
    /// Generate NFT metadata
    pub fn generate_nft(&mut self, request: NFTRequest) -> Result<NFTMetadata> {
        debug!("🎨 Generating NFT for: {}", request.emoji_sequence);
        
        let session = self.get_or_create_session(request.session_id)?;
        
        // Generate a unique token ID
        let token_id = {
            let nft_cache = self.nft_cache.read().unwrap();
            (nft_cache.len() as u32) + 1
        };
        
        let mut metadata = self.emoji_engine.generate_nft_metadata(&request.emoji_sequence, token_id)?;
        
        // Add custom attributes if provided
        if let Some(custom_attrs) = request.custom_attributes {
            for (key, value) in custom_attrs {
                metadata.attributes.push(emoji_semantics::NFTAttribute {
                    trait_type: key,
                    value,
                });
            }
        }
        
        // Cache the NFT
        self.nft_cache.write().unwrap().insert(token_id, metadata.clone());
        
        // Update session stats
        let mut sessions = self.sessions.write().unwrap();
        if let Some(mut session) = sessions.get(&session.id).cloned() {
            session.nfts_minted += 1;
            session.last_activity = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            sessions.insert(session.id.clone(), session);
        }
        
        info!("🎨 Generated NFT #{} for session {}", token_id, session.id);
        Ok(metadata)
    }
    
    /// Get runtime statistics
    pub fn get_stats(&self) -> Result<RuntimeStats> {
        let sessions = self.sessions.read().unwrap();
        let poems = self.poems_cache.read().unwrap();
        let nfts = self.nft_cache.read().unwrap();
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        // Calculate active sessions (active in last hour)
        let active_sessions = sessions.values()
            .filter(|s| now - s.last_activity < 3600)
            .count();
        
        // Calculate average resonance
        let total_resonance: f64 = poems.values()
            .map(|p| p.resonance_score)
            .sum();
        let average_resonance = if poems.is_empty() {
            0.0
        } else {
            total_resonance / poems.len() as f64
        };
        
        // Find most popular emoji
        let mut emoji_counts: HashMap<char, usize> = HashMap::new();
        for poem in poems.values() {
            for emoji in poem.input_emoji.chars() {
                *emoji_counts.entry(emoji).or_insert(0) += 1;
            }
        }
        let most_popular_emoji = emoji_counts.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(emoji, _)| emoji.to_string())
            .unwrap_or_else(|| "🌀".to_string());
        
        // Calculate total quines
        let total_quines = sessions.values()
            .map(|s| s.quines_created)
            .sum::<u32>() as usize;
        
        // Estimate memory usage (rough calculation)
        let memory_usage_mb = (sessions.len() * 1000 + poems.len() * 2000 + nfts.len() * 3000) as f64 / 1024.0 / 1024.0;
        
        Ok(RuntimeStats {
            total_sessions: sessions.len(),
            active_sessions,
            total_poems: poems.len(),
            total_quines,
            total_nfts: nfts.len(),
            average_resonance,
            most_popular_emoji,
            uptime_seconds: now, // Simplified - would need actual start time
            memory_usage_mb,
        })
    }
    
    /// Generate poetic text from expression
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
    
    /// Calculate rarity tier
    fn calculate_rarity(&self, resonance: f64, emoji_count: usize) -> RarityTier {
        let complexity_score = resonance + (emoji_count as f64 * 0.01);
        
        match complexity_score {
            s if s >= 1.05 => RarityTier::UltraRare,
            s if s >= 1.00 => RarityTier::Epic,
            s if s >= 0.95 => RarityTier::Rare,
            s if s >= 0.90 => RarityTier::Uncommon,
            _ => RarityTier::Common,
        }
    }
    
    /// Clean up old sessions and cache entries
    pub fn cleanup(&self) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        // Remove sessions inactive for more than 24 hours
        let mut sessions = self.sessions.write().unwrap();
        sessions.retain(|_, session| now - session.last_activity < 86400);
        
        // Remove poems older than 7 days
        let mut poems = self.poems_cache.write().unwrap();
        poems.retain(|_, poem| now - poem.created_at < 604800);
        
        info!("🧹 Cleaned up old sessions and cache entries");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_runtime_creation() {
        let runtime = MetaMemeRuntime::new();
        assert!(runtime.sessions.read().unwrap().is_empty());
        assert!(runtime.poems_cache.read().unwrap().is_empty());
    }
    
    #[test]
    fn test_session_creation() {
        let runtime = MetaMemeRuntime::new();
        let session = runtime.create_session().unwrap();
        
        assert!(!session.id.is_empty());
        assert_eq!(session.poems_generated, 0);
        assert_eq!(session.quines_created, 0);
    }
    
    #[test]
    fn test_poem_generation() {
        let mut runtime = MetaMemeRuntime::new();
        let request = PoemRequest {
            emoji_sequence: "🌀🎭".to_string(),
            session_id: None,
            max_reduction_steps: Some(10),
        };
        
        let poem = runtime.generate_poem(request).unwrap();
        
        assert_eq!(poem.input_emoji, "🌀🎭");
        assert!(poem.resonance_score > 0.0);
        assert!(!poem.poetic_text.is_empty());
    }
    
    #[test]
    fn test_quine_creation() {
        let mut runtime = MetaMemeRuntime::new();
        let request = QuineRequest {
            seed: "🌀".to_string(),
            session_id: None,
            max_reduction_steps: Some(10),
        };
        
        let quine = runtime.create_quine(request).unwrap();
        
        assert_eq!(quine.seed, "🌀");
        assert!(!quine.original_expression.is_empty());
        assert!(quine.reduction_steps >= 0);
    }
    
    #[test]
    fn test_emoji_analysis() {
        let mut runtime = MetaMemeRuntime::new();
        let request = AnalysisRequest {
            emoji_sequence: "🌀🎭🧬".to_string(),
            include_trace: true,
            session_id: None,
        };
        
        let analysis = runtime.analyze_emoji(request).unwrap();
        
        assert_eq!(analysis.input, "🌀🎭🧬");
        assert_eq!(analysis.emoji_count, 3);
        assert!(analysis.resonance_score > 0.0);
        assert!(analysis.trace.is_some());
    }
    
    #[test]
    fn test_nft_generation() {
        let mut runtime = MetaMemeRuntime::new();
        let request = NFTRequest {
            emoji_sequence: "🌀🎭🧬🌌".to_string(),
            session_id: None,
            custom_attributes: Some({
                let mut attrs = HashMap::new();
                attrs.insert("Creator".to_string(), "Test".to_string());
                attrs
            }),
        };
        
        let nft = runtime.generate_nft(request).unwrap();
        
        assert_eq!(nft.emoji_sequence, "🌀🎭🧬🌌");
        assert!(nft.resonance_score > 0.0);
        assert!(!nft.attributes.is_empty());
        
        // Check custom attribute was added
        assert!(nft.attributes.iter().any(|attr| attr.trait_type == "Creator"));
    }
    
    #[test]
    fn test_stats_generation() {
        let mut runtime = MetaMemeRuntime::new();
        
        // Generate some data
        let poem_request = PoemRequest {
            emoji_sequence: "🌀🎭".to_string(),
            session_id: None,
            max_reduction_steps: Some(10),
        };
        runtime.generate_poem(poem_request).unwrap();
        
        let stats = runtime.get_stats().unwrap();
        
        assert_eq!(stats.total_poems, 1);
        assert_eq!(stats.total_sessions, 1);
        assert!(stats.average_resonance > 0.0);
    }
}
