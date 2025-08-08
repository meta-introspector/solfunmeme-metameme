use anyhow::Result;
use log::info;
use crate::MetaMemeEngine;

pub async fn instrumented_run() -> Result<()> {
    info!("🧪 Running instrumented methods and monitoring memory...");

    let mut engine = MetaMemeEngine::new();

    // Test generate_poem
    engine.memory_monitor.start_tracking("generate_poem");
    let poem = engine.generate_poem("🌀🎭🧬").await?;
    engine.memory_monitor.stop_tracking("generate_poem");
    info!("Generated Poem: {:?}", poem);

    // Test create_quine
    engine.memory_monitor.start_tracking("create_quine");
    let quine = engine.create_quine("🌀").await?;
    engine.memory_monitor.stop_tracking("create_quine");
    info!("Created Quine: {:?}", quine);

    // Test generate_nft_collection
    engine.memory_monitor.start_tracking("generate_nft_collection");
    let nfts = engine.generate_nft_collection(5).await?;
    engine.memory_monitor.stop_tracking("generate_nft_collection");
    info!("Generated NFTs: {:?}", nfts);

    // Test evolve_universe
    engine.memory_monitor.start_tracking("evolve_universe");
    let evolution_result = engine.evolve_universe(2, 0.5).await?;
    engine.memory_monitor.stop_tracking("evolve_universe");
    info!("Evolution Result: {:?}", evolution_result);

    engine.memory_monitor.print_summary();

    Ok(())
}