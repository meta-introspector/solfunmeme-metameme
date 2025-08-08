use lambda_calculus_core::LambdaEngine;
use emoji_semantics::EmojiSemantics;
use stanza_universe::StanzaUniverse;
use ragit_memory_monitor::MemoryMonitor;

/// 🌟 The main MetaMeme engine that orchestrates all components
pub struct MetaMemeEngine {
    /// Lambda calculus computation engine
    pub lambda_engine: LambdaEngine,
    /// Emoji semantic interpretation engine
    pub emoji_engine: EmojiSemantics,
    /// Stanza universe for poetry generation
    pub stanza_universe: StanzaUniverse,
    /// Memory monitor for tracking memory usage
    pub memory_monitor: MemoryMonitor,
}