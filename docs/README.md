# Church-of-FEAR Ledger

A Rust implementation of the Moral Ledger schema for Church-of-FEAR governance.
This crate builds an immutable audit trail of DeedEvents, computes account states,
and mints symbolic CHURCH tokens for good deeds, promoting ecological sustainability
and forgiveness-seeking without financial involvement.

## Features
- Append-only ledger with hash-chaining for tamper-evidence.
- Derived ChurchAccountState with bounded metrics (eco_score in [0,1]).
- Time-discounted good deeds for fair token minting.
- Harm penalties and quorum-based forgiveness simulation.
- CLI demo for adding deeds and querying states.

## Usage
cargo run -- --add-deed "ecological_sustainability" --actor "user1" --tags "tree_planting"
cargo run -- --compute-state "user1"

This contributes to TECH by providing expandable moral accounting tools,
NANO by minimizing computational risks, and POWER by enabling safe simulations
of eco-grants for earth-saving projects.

# File: src/main.rs
use church_of_fear_ledger::ledger::{DeedEvent, Ledger, ChurchAccountState};
use church_of_fear_ledger::utils::crypto::compute_sha256_hash;
use clap::{Parser, Subcommand};
use serde_json::json;
use std::error::Error;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "church_ledger")]
#[command(about = "CLI for Church-of-FEAR Moral Ledger")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddDeed {
        deed_type: String,
        actor: String,
        #[arg(long)]
        tags: Option<String>,
    },
    ComputeState {
        actor: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut ledger = Ledger::new();

    match cli.command {
        Commands::AddDeed { deed_type, actor, tags } => {
            let tags_vec = tags.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
            let event_id = Uuid::new_v4().to_string();
            let timestamp = chrono::Utc::now().timestamp() as u64;
            let prev_hash = ledger.last_hash().unwrap_or_default();
            let context_json = json!({ "evidence": "Sample evidence" });
            let mut deed = DeedEvent {
                event_id,
                timestamp,
                prev_hash,
                self_hash: String::new(), // Computed later
                actor_id: actor.clone(),
                target_ids: vec![],
                deed_type,
                tags: tags_vec,
                context_json,
                ethics_flags: vec![],
                life_harm_flag: false,
            };
            deed.self_hash = deed.compute_self_hash();
            ledger.append(deed);
            println!("Deed added for actor {}", actor);
        }
        Commands::ComputeState { actor } => {
            if let Some(state) = ChurchAccountState::compute_from_ledger(&ledger, &actor) {
                println!("ChurchAccountState for {}: {:?}", actor, state);
                if state.can_mint_church() {
                    println!("Minting CHURCH tokens: {}", state.compute_mint_amount());
                }
            } else {
                println!("No state for actor {}", actor);
            }
        }
    }
    Ok(())
}
