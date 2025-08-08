pub mod engine;
pub mod methods;
pub mod generated_poem;
pub mod quine_result;
pub mod evolution_result;

pub use engine::MetaMemeEngine;
pub use generated_poem::GeneratedPoem;
pub use quine_result::QuineResult;
pub use evolution_result::EvolutionResult;

pub use methods::new::*;
pub use methods::generate_poem::*;
pub use methods::create_quine::*;
pub use methods::generate_nft_collection::*;
pub use methods::generate_rarity_based_emoji::*;
pub use methods::generate_poetic_text::*;
pub use methods::evolve_universe::*;