use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use log::{info, error};
use rand::Rng;

use solfunmeme_metameme::MetaMemeEngine;
use ragit_memory_monitor::MemoryMonitor;

use lambda_calculus_core::{Expr, LambdaEngine};
use emoji_semantics::EmojiSemantics;
use stanza_universe::StanzaUniverse;

use crate::commands;

use crate::commands::instrumented_run::instrumented_run;
use crate::commands::generate_stanza::generate_stanza;
use crate::commands::create_quine::create_quine;
use crate::commands::evolve_stanza::evolve_stanza;
use crate::commands::generate_nft_collection::generate_nft_collection;
use crate::commands::analyze_emoji::analyze_emoji;
use crate::commands::create_universe::create_universe;
use crate::commands::launch_repl::launch_repl;
use crate::commands::show_stats::show_stats;


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
        #[arg(short, long)]
        output_dir: PathBuf,
        
        /// Minimum resonance score
        #[arg(short, long, default_value = "0.85")]
        min_resonance: f64,
    },
    
    /// 🔍 Analyze an emoji sequence
    Analyze {
        /// Emoji sequence to analyze
        #[arg(short, long)]
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

    /// 🧪 Run instrumented methods and monitor memory
    InstrumentedRun,
}

pub async fn main() -> Result<()> {
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
            let mut engine = MetaMemeEngine::new();
            generate_stanza(&mut engine, &emoji, output.as_deref()).await?;
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

        Commands::InstrumentedRun => {
            instrumented_run().await?;
        }
    }
    
    info!("✨ SOLFUNMEME MetaMeme Engine Complete!");
    Ok(())
}
