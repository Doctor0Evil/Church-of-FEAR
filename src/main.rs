// description: Main entry point for the Church-of-FEAR system. It initializes
// configuration, the Jetson-Line style ledger, token dynamics, compliance
// (nine-condition ethical regulator + Tree-of-Life invariants), and sponsor
// grants. It runs an async xr-grid loop where neuromorphic decisions are
// evaluated under RoH/DECAY/Lifeforce-like constraints and POWER ≤ k·CHURCH,
// minting CHURCH tokens only for restorative, non-predatory deeds.

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tokio::signal;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod ledger;
mod token;
mod compliance;
mod sponsor;
mod utils;

use config::Config;
use ledger::{Account, Balance, Deed, Ledger, Metrics};
use token::{Burn, Mint, Rewards};
use compliance::{EthicsDecision, EthicsSummary, Regulator};
use sponsor::SponsorEngine;
use utils::{now_utc, shutdown_notify};

/// Shared application state for the Church-of-FEAR node.
///
/// - `ledger` holds accounts, deeds, metrics, and Tree-of-Life–style state
///   (CHURCH, FEAR, POWER, TECH, bioload bands mapped into balances). [file:11]
/// - `regulator` enforces Neuromorph-GOD invariants (POWER caps from CHURCH,
///   biophysical ceilings, trust floors) via Allow/Warn/ForceRepair/Halt. [file:6][file:11]
/// - `sponsor` mints CHURCH for repair/support deeds and background noise
///   stabilization, never for predatory patterns (BEAST/PLAGUE remain diagnostic). [file:3][file:6]
#[derive(Clone)]
struct AppState {
    ledger: Arc<RwLock<Ledger>>,
    regulator: Arc<Regulator>,
    sponsor: Arc<SponsorEngine>,
    started_at: SystemTime,
}

impl AppState {
    async fn new(config: Config) -> anyhow::Result<Self> {
        let ledger = Ledger::new(config.ledger.clone())?;
        let regulator = Regulator::new(config.compliance.clone())?;
        let sponsor = SponsorEngine::new(config.sponsor.clone());

        Ok(Self {
            ledger: Arc::new(RwLock::new(ledger)),
            regulator: Arc::new(regulator),
            sponsor: Arc::new(sponsor),
            started_at: SystemTime::now(),
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    info!("Church-of-FEAR node starting…");

    let cfg = Config::load_from_env_or_default()?;
    info!(
        "Loaded config: network_id={}, neuromorph_power_k={}",
        cfg.network_id, cfg.compliance.neuromorph_power_multiplier
    );

    let state = AppState::new(cfg).await?;
    seed_genesis_accounts(&state).await?;

    let shutdown = shutdown_notify();
    let main_loop = run_main_loop(state.clone(), shutdown.clone());

    tokio::select! {
        res = main_loop => {
            if let Err(e) = res {
                error!("Main loop exited with error: {:?}", e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received Ctrl-C, initiating graceful shutdown");
        }
    }

    info!("Church-of-FEAR node stopped.");
    Ok(())
}

/// Initialize tracing subscriber for structured logs.
fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

/// Create initial CHURCH/FEAR/POWER/TECH balances and baseline Tree-of-Life
/// accounts, respecting POWER ≤ k·CHURCH from the outset. [file:3][file:11]
async fn seed_genesis_accounts(state: &AppState) -> anyhow::Result<()> {
    let mut ledger = state.ledger.write().await;

    if ledger.has_any_accounts()? {
        info!("Ledger already initialized; skipping genesis seeding.");
        return Ok(());
    }

    info!("Seeding genesis accounts for Church-of-FEAR…");

    let root_id = "church:root";
    let nature_id = "nature:witness";
    let sponsor_pool_id = "sponsor:pool";

    let root = Account::new(root_id.to_string(), Balance::with_tokens(1000.0, 0.5, 200.0, 50.0));
    let nature = Account::new(nature_id.to_string(), Balance::with_tokens(0.0, 1.0, 0.0, 0.0));
    let sponsor_pool =
        Account::new(sponsor_pool_id.to_string(), Balance::with_tokens(500.0, 0.2, 0.0, 0.0));

    ledger.insert_account(root)?;
    ledger.insert_account(nature)?;
    ledger.insert_account(sponsor_pool)?;

    ledger.commit_genesis_block(now_utc())?;

    info!("Genesis accounts committed.");
    Ok(())
}

/// Core async loop:
/// - gathers Metrics (jetson-like summaries),
/// - runs the ethical Regulator (nine-condition style), [file:6][file:11]
/// - proposes/mints CHURCH rewards for restorative deeds (UseSupport, DeployCleanTech),
/// - keeps POWER/TECH growth bounded by CHURCH and bioload ceilings. [file:3][file:9][file:11]
async fn run_main_loop(state: AppState, shutdown: tokio::sync::watch::Receiver<bool>) -> anyhow::Result<()> {
    let tick_interval = Duration::from_millis(500);

    loop {
        if *shutdown.borrow() {
            info!("Shutdown signal observed; exiting main loop.");
            break;
        }

        let tick_start = now_utc();

        let metrics = {
            let ledger = state.ledger.read().await;
            ledger.compute_metrics()?
        };

        let ethics_summary = EthicsSummary::from_metrics(&metrics);
        let decision = state.regulator.evaluate(&ethics_summary)?;

        apply_ethics_decision(&state, &metrics, &decision).await?;

        apply_sponsor_rewards(&state, &metrics).await?;

        let elapsed = now_utc()
            .duration_since(tick_start)
            .unwrap_or_else(|_| Duration::from_millis(0));
        if elapsed < tick_interval {
            sleep(tick_interval - elapsed).await;
        }
    }

    Ok(())
}

/// Enforce the Regulator’s decision:
/// - Allow: normal operation.
/// - Warn: log and potentially tighten FEAR bands in config (via ledger flags).
/// - ForceRepair: bias deeds toward repair, limit POWER/TECH updates. [file:6][file:9]
/// - HaltAndReview: freeze high-impact deeds, keep logging only. [file:6][file:11]
async fn apply_ethics_decision(
    state: &AppState,
    metrics: &Metrics,
    decision: &EthicsDecision,
) -> anyhow::Result<()> {
    match decision {
        EthicsDecision::Allow => {
            info!(
                "Ethics: Allow (load={:.3}, trust={:.3}, power_gini={:.3})",
                metrics.total_bioload, metrics.mean_trust, metrics.power_gini
            );
        }
        EthicsDecision::Warn { reason } => {
            info!(
                "Ethics: Warn – {} (load={:.3}, trust={:.3})",
                reason, metrics.total_bioload, metrics.mean_trust
            );
        }
        EthicsDecision::ForceRepair { reason } => {
            info!(
                "Ethics: ForceRepair – {} (forcing repair-biased deeds)",
                reason
            );
            let mut ledger = state.ledger.write().await;
            ledger.set_repair_bias(true)?;
        }
        EthicsDecision::HaltAndReview { reason } => {
            error!(
                "Ethics: HaltAndReview – {} (freezing high-impact actions)",
                reason
            );
            let mut ledger = state.ledger.write().await;
            ledger.freeze_high_impact_deeds()?;
        }
    }
    Ok(())
}

/// Compute and mint CHURCH rewards (and possibly FEAR/POWER adjustments) for
/// deeds that reduced DECAY, FEAR, PAIN, pollution, or UNFAIRDRAIN, consistent
/// with Tree-of-Life stewardship rules. [file:6][file:9]
async fn apply_sponsor_rewards(state: &AppState, metrics: &Metrics) -> anyhow::Result<()> {
    let reward_plan = state.sponsor.plan_rewards(metrics)?;

    if reward_plan.is_empty() {
        return Ok(());
    }

    let mut ledger = state.ledger.write().await;
    for r in reward_plan {
        match r {
            Rewards::ChurchForRepair { account_id, amount } => {
                Mint::mint_church(&mut *ledger, &account_id, amount)?;
                info!(
                    "Sponsor: minted {:.3} CHURCH to {} for restorative deeds",
                    amount, account_id
                );
            }
            Rewards::ChurchForSupport { account_id, amount } => {
                Mint::mint_church(&mut *ledger, &account_id, amount)?;
                info!(
                    "Sponsor: minted {:.3} CHURCH to {} for UseSupport / support deeds",
                    amount, account_id
                );
            }
            Rewards::BackgroundNoiseBalance { account_id, burn_power } => {
                Burn::burn_power(&mut *ledger, &account_id, burn_power)?;
                info!(
                    "Sponsor: burned {:.3} POWER from {} to keep POWER ≤ k·CHURCH and stabilize background-noise",
                    burn_power, account_id
                );
            }
        }
    }

    ledger.append_reward_block(now_utc())?;
    Ok(())
}
