//! # 🚀 SOLFUNMEME MetaMeme: The Ultimate Self-Replicating Poetry Engine
//! 
//! This is the main CLI application for the SOLFUNMEME MetaMeme project.
//! It combines lambda calculus, emoji semantics, and poetic generation
//! to create the world's first self-replicating NFT collection.

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use log::{info, error};
use rand::Rng;

use lambda_calculus_core::{Expr, LambdaEngine};
use emoji_semantics::EmojiSemantics;
use stanza_universe::StanzaUniverse;

/// 🌀 SOLFUNMEME MetaMeme CLI
#[derive(Parser)]
#[command(name = "solfunmeme")]
#[command(about = "🚀 SOLFUNMEME: The Ultimate Self-Replicating MetaMeme - 9,901 NFT Lambda Calculus Poetry Collection")]
#[command(version = "0.1.0")]
#[command(author = "The MetaIntrospector Collective")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// 🎭 Generate a poetic stanza from emoji sequence
    Generate {
        /// Emoji sequence to interpret
        #[arg(short, long)]
        emoji: String,
        
        /// Output file for the generated stanza
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// 🌀 Create a self-replicating quine expression
    Quine {
        /// Seed for quine generation
        #[arg(short, long, default_value = "🌀")]
        seed: String,
        
        /// Maximum reduction steps
        #[arg(short, long, default_value = "100")]
        max_steps: usize,
    },
    
    /// 🧬 Evolve an existing stanza
    Evolve {
        /// Parent stanza ID
        #[arg(short, long)]
        parent_id: u32,
        
        /// Mutation rate (0.0 - 1.0)
        #[arg(short, long, default_value = "0.1")]
        mutation_rate: f64,
        
        /// Number of generations to evolve
        #[arg(short, long, default_value = "1")]
        generations: u32,
    },
    
    /// 🎨 Generate NFT metadata for a collection
    Nft {
        /// Number of NFTs to generate
        #[arg(short, long, default_value = "100")]
        count: u32,
        
        /// Output directory for metadata
        #[arg(short, long, default_value = "nft-metadata")]
        output_dir: PathBuf,
        
        /// Minimum resonance score
        #[arg(short, long, default_value = "0.85")]
        min_resonance: f64,
    },
    
    /// 🔍 Analyze an emoji sequence
    Analyze {
        /// Emoji sequence to analyze
        emoji: String,
        
        /// Show detailed reduction trace
        #[arg(short, long)]
        trace: bool,
    },
    
    /// 🌌 Initialize the complete stanza universe
    Universe {
        /// Number of stanzas to generate
        #[arg(short, long, default_value = "25")]
        count: u32,
        
        /// Output file for the universe
        #[arg(short, long, default_value = "stanza-universe.json")]
        output: PathBuf,
    },
    
    /// 🚀 Launch interactive SOLFUNMEME REPL
    Repl,
    
    /// 📊 Show statistics about the current universe
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }
    
    info!("🚀 SOLFUNMEME MetaMeme Engine Starting...");
    
    match cli.command {
        Commands::Generate { emoji, output } => {
            generate_stanza(&emoji, output.as_deref()).await?;
        }
        
        Commands::Quine { seed, max_steps } => {
            create_quine(&seed, max_steps).await?;
        }
        
        Commands::Evolve { parent_id, mutation_rate, generations } => {
            evolve_stanza(parent_id, mutation_rate, generations).await?;
        }
        
        Commands::Nft { count, output_dir, min_resonance } => {
            generate_nft_collection(count, &output_dir, min_resonance).await?;
        }
        
        Commands::Analyze { emoji, trace } => {
            analyze_emoji(&emoji, trace).await?;
        }
        
        Commands::Universe { count, output } => {
            create_universe(count, &output).await?;
        }
        
        Commands::Repl => {
            launch_repl().await?;
        }
        
        Commands::Stats => {
            show_stats().await?;
        }
    }
    
    info!("✨ SOLFUNMEME MetaMeme Engine Complete!");
    Ok(())
}

/// 🎭 Generate a poetic stanza from emoji sequence
async fn generate_stanza(emoji: &str, output: Option<&std::path::Path>) -> Result<()> {
    info!("🎭 Generating stanza from emoji: {}", emoji);
    
    let mut emoji_engine = EmojiSemantics::new();
    let (expr, resonance) = emoji_engine.interpret_emoji_poem(emoji)?;
    
    let mut lambda_engine = LambdaEngine::new();
    let trace = lambda_engine.normalize(expr.clone())?;
    
    // Generate poetic text based on the expression
    let poetic_text = generate_poetic_text(&expr, resonance);
    
    let result = format!(
        "🌟 SOLFUNMEME Stanza Generated 🌟\n\
        ================================\n\
        \n\
        Emoji Sequence: {}\n\
        Resonance Score: {:.3}\n\
        \n\
        Lambda Expression: {}\n\
        Reduced Form: {}\n\
        Reduction Steps: {}\n\
        \n\
        Poetic Manifestation:\n\
        ---------------------\n\
        {}\n\
        \n\
        🧬 This stanza embodies the eternal dance between computation and creativity,\n\
        where lambda calculus meets the infinite poetry of the digital realm.\n",
        emoji,
        resonance,
        expr,
        trace.final_form,
        trace.step_count,
        poetic_text
    );
    
    if let Some(output_path) = output {
        std::fs::write(output_path, &result)?;
        info!("📝 Stanza written to: {}", output_path.display());
    } else {
        println!("{}", result);
    }
    
    Ok(())
}

/// Generate poetic text from lambda expression
fn generate_poetic_text(expr: &Expr, resonance: f64) -> String {
    let base_verses = vec![
        "In the spiral of recursive dreams,\nWhere lambda meets the light of screens,",
        "Through combinatorial gardens vast,\nWhere future dances with the past,",
        "Digital muses stir and wake,\nNew realities they gently make,",
        "In blockchain's immutable embrace,\nPoetry finds its sacred space,",
    ];
    
    let mut rng = rand::thread_rng();
    let base = base_verses[rng.gen_range(0..base_verses.len())];
    
    let resonance_verse = match resonance {
        r if r >= 0.95 => "With resonance that shakes the stars,\nThis verse transcends all earthly bars.",
        r if r >= 0.90 => "High resonance flows through each line,\nMaking mortal words divine.",
        r if r >= 0.85 => "Gentle resonance guides the way,\nThrough night's darkness into day.",
        _ => "Soft resonance whispers low,\nOf truths that only poets know.",
    };
    
    let expr_verse = match expr {
        Expr::S => "The S-combinator weaves its spell,\nComposing functions, all is well.",
        Expr::K => "The K-combinator stands so true,\nConstant guardian, me and you.",
        Expr::I => "Identity reflects the soul,\nMaking broken spirits whole.",
        Expr::Muse(_, _) => "The muse awakens from her sleep,\nSecrets of creation to keep.",
        _ => "Complex patterns intertwine,\nIn this expression so divine.",
    };
    
    format!("{}\n{}\n\n{}", base, resonance_verse, expr_verse)
}

/// 🌀 Create a self-replicating quine expression
async fn create_quine(seed: &str, max_steps: usize) -> Result<()> {
    info!("🌀 Creating quine with seed: {}", seed);
    
    let mut lambda_engine = LambdaEngine::new().with_max_steps(max_steps);
    let quine = lambda_engine.create_quine(seed);
    
    let trace = lambda_engine.normalize(quine.clone())?;
    
    let emoji_engine = EmojiSemantics::new();
    let emoji_output = emoji_engine.expr_to_emoji(&trace.final_form);
    
    println!("🌀 SOLFUNMEME Quine Generated 🌀");
    println!("===============================");
    println!();
    println!("Seed: {}", seed);
    println!("Original Expression: {}", quine);
    println!("Final Form: {}", trace.final_form);
    println!("Emoji Output: {}", emoji_output);
    println!("Reduction Steps: {}", trace.step_count);
    println!();
    
    if emoji_output.contains(seed) {
        println!("✅ PERFECT QUINE: The output contains the original seed!");
        println!("🧬 This expression has achieved self-replication!");
    } else {
        println!("🔄 PARTIAL QUINE: The expression shows self-referential properties.");
        println!("🌱 Further evolution may achieve perfect self-replication.");
    }
    
    Ok(())
}

/// 🧬 Evolve an existing stanza
async fn evolve_stanza(parent_id: u32, mutation_rate: f64, generations: u32) -> Result<()> {
    info!("🧬 Evolving stanza {} for {} generations", parent_id, generations);
    
    let mut universe = StanzaUniverse::new();
    let mut current_id = parent_id;
    
    for generation in 1..=generations {
        match universe.evolve_stanza(current_id, mutation_rate) {
            Ok(new_id) => {
                let new_stanza = universe.get_stanza(new_id).unwrap();
                println!("🧬 Generation {}: Stanza #{}", generation, new_id);
                println!("   Emoji: {}", new_stanza.emoji_sequence);
                println!("   Resonance: {:.3}", new_stanza.resonance);
                println!("   Recursion Depth: {}", new_stanza.recursion_depth);
                println!();
                current_id = new_id;
            }
            Err(e) => {
                error!("❌ Evolution failed at generation {}: {}", generation, e);
                break;
            }
        }
    }
    
    Ok(())
}

/// 🎨 Generate NFT collection metadata
async fn generate_nft_collection(count: u32, output_dir: &std::path::Path, min_resonance: f64) -> Result<()> {
    info!("🎨 Generating {} NFTs with min resonance {:.3}", count, min_resonance);
    
    std::fs::create_dir_all(output_dir)?;
    
    let mut emoji_engine = EmojiSemantics::new();
    
    for token_id in 1..=count {
        // Generate random high-resonance emoji sequence
        let emoji_length = rand::thread_rng().gen_range(3..=8);
        let emoji_sequence = emoji_engine.generate_random_poem(emoji_length, min_resonance);
        
        let metadata = emoji_engine.generate_nft_metadata(&emoji_sequence, token_id)?;
        
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        let filename = format!("{}.json", token_id);
        let filepath = output_dir.join(filename);
        
        std::fs::write(&filepath, metadata_json)?;
        
        if token_id % 100 == 0 {
            info!("📝 Generated {} NFT metadata files", token_id);
        }
    }
    
    info!("✅ Generated {} NFT metadata files in {}", count, output_dir.display());
    Ok(())
}

/// 🔍 Analyze an emoji sequence
async fn analyze_emoji(emoji: &str, show_trace: bool) -> Result<()> {
    info!("🔍 Analyzing emoji sequence: {}", emoji);
    
    let mut emoji_engine = EmojiSemantics::new();
    let (expr, resonance) = emoji_engine.interpret_emoji_poem(emoji)?;
    
    let mut lambda_engine = LambdaEngine::new();
    let trace = lambda_engine.normalize(expr.clone())?;
    
    println!("🔍 SOLFUNMEME Emoji Analysis 🔍");
    println!("==============================");
    println!();
    println!("Input: {}", emoji);
    println!("Emoji Count: {}", emoji.chars().count());
    println!("Resonance Score: {:.3}", resonance);
    println!();
    println!("Lambda Expression: {}", expr);
    println!("Reduced Form: {}", trace.final_form);
    println!("Reduction Steps: {}", trace.step_count);
    println!("Normal Form: {}", trace.is_normal_form);
    println!();
    
    if show_trace && !trace.steps.is_empty() {
        println!("🔄 Reduction Trace:");
        println!("------------------");
        for (i, step) in trace.steps.iter().enumerate() {
            println!("Step {}: {}", i, step);
        }
        println!();
    }
    
    // Convert back to emoji
    let output_emoji = emoji_engine.expr_to_emoji(&trace.final_form);
    println!("Output Emoji: {}", output_emoji);
    
    if output_emoji == emoji {
        println!("🌀 QUINE DETECTED: This sequence is self-replicating!");
    }
    
    Ok(())
}

/// 🌌 Create the complete stanza universe
async fn create_universe(count: u32, output: &std::path::Path) -> Result<()> {
    info!("🌌 Creating universe with {} stanzas", count);
    
    let mut universe = StanzaUniverse::new();
    
    // Generate additional stanzas beyond the core ones
    let mut emoji_engine = EmojiSemantics::new();
    
    for i in 4..=count {
        let emoji_length = rand::thread_rng().gen_range(3..=7);
        let emoji_sequence = emoji_engine.generate_random_poem(emoji_length, 0.80);
        
        let poetic_text = format!(
            "Stanza {} emerges from the void,\nWhere {} dances unalloyed,\nIn recursive loops of pure delight,\nBringing darkness into light.",
            i, emoji_sequence
        );
        
        let resonance = rand::thread_rng().gen_range(0.80..0.98);
        let recursion_depth = rand::thread_rng().gen_range(1..=4);
        let is_quine = rand::thread_rng().gen_bool(0.1); // 10% chance of being a quine
        
        universe.create_stanza(&poetic_text, &emoji_sequence, resonance, is_quine, recursion_depth)?;
    }
    
    // Serialize the universe
    let universe_data = serde_json::to_string_pretty(&universe.stanzas)?;
    std::fs::write(output, universe_data)?;
    
    info!("✅ Universe with {} stanzas written to {}", count, output.display());
    Ok(())
}

/// 🚀 Launch interactive REPL
async fn launch_repl() -> Result<()> {
    println!("🚀 SOLFUNMEME Interactive REPL");
    println!("==============================");
    println!("Enter emoji sequences to see their lambda calculus interpretations!");
    println!("Commands: :quit, :help, :stats");
    println!();
    
    let mut emoji_engine = EmojiSemantics::new();
    let mut lambda_engine = LambdaEngine::new();
    
    loop {
        print!("🌀 > ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input {
            ":quit" | ":q" => {
                println!("👋 Farewell from the MetaMeme universe!");
                break;
            }
            ":help" | ":h" => {
                println!("🎭 SOLFUNMEME REPL Help:");
                println!("  Enter emoji sequences to interpret them");
                println!("  :quit or :q - Exit the REPL");
                println!("  :help or :h - Show this help");
                println!("  :stats or :s - Show statistics");
                continue;
            }
            ":stats" | ":s" => {
                println!("📊 Current session statistics:");
                println!("  Emoji semantics loaded: {}", emoji_engine.semantics.len());
                println!("  Lambda engine max steps: {}", lambda_engine.max_steps);
                continue;
            }
            _ => {
                match emoji_engine.interpret_emoji_poem(input) {
                    Ok((expr, resonance)) => {
                        match lambda_engine.normalize(expr.clone()) {
                            Ok(trace) => {
                                println!("  Expression: {}", expr);
                                println!("  Reduced: {}", trace.final_form);
                                println!("  Resonance: {:.3}", resonance);
                                println!("  Steps: {}", trace.step_count);
                                
                                let output_emoji = emoji_engine.expr_to_emoji(&trace.final_form);
                                println!("  Output: {}", output_emoji);
                                
                                if output_emoji == input {
                                    println!("  🌀 QUINE!");
                                }
                            }
                            Err(e) => println!("  ❌ Reduction error: {}", e),
                        }
                    }
                    Err(e) => println!("  ❌ Interpretation error: {}", e),
                }
            }
        }
        println!();
    }
    
    Ok(())
}

/// 📊 Show statistics about the current universe
async fn show_stats() -> Result<()> {
    println!("📊 SOLFUNMEME MetaMeme Statistics");
    println!("=================================");
    
    let universe = StanzaUniverse::new();
    let emoji_engine = EmojiSemantics::new();
    
    println!("🌌 Universe Statistics:");
    println!("  Total Stanzas: {}", universe.stanzas.len());
    println!("  Emoji Mappings: {}", universe.emoji_to_stanza.len());
    println!();
    
    println!("🎭 Emoji Semantics:");
    println!("  Total Emoji Semantics: {}", emoji_engine.semantics.len());
    println!("  Reverse Mappings: {}", emoji_engine.reverse_semantics.len());
    println!();
    
    // Analyze rarity distribution
    let mut rarity_counts = std::collections::HashMap::new();
    for stanza in universe.stanzas.values() {
        *rarity_counts.entry(format!("{:?}", stanza.rarity)).or_insert(0) += 1;
    }
    
    println!("🎯 Rarity Distribution:");
    for (rarity, count) in rarity_counts {
        println!("  {}: {}", rarity, count);
    }
    println!();
    
    // Show quine statistics
    let quine_count = universe.stanzas.values().filter(|s| s.is_quine).count();
    println!("🌀 Self-Replication:");
    println!("  Quine Stanzas: {}", quine_count);
    println!("  Regular Stanzas: {}", universe.stanzas.len() - quine_count);
    
    Ok(())
}
