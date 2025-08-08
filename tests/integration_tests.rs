use solfunmeme_metameme::*;

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