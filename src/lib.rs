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
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
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
//! Ok(())
//! }
//! ```

pub use lambda_calculus_core::{Expr, LambdaEngine, ReductionTrace};
pub use emoji_semantics::{EmojiSemantics, EmojiSemantic, RarityTier, CombinatorType, NFTMetadata};
pub use stanza_universe::{StanzaUniverse, Stanza};

pub mod engine;
pub use engine::*;