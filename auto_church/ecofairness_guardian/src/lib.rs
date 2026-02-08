#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tracing::{info, warn};

pub use rohmodel::RohModel;
pub use tsafe::{SovereignAction, PolicyEngine, RequestRoute};
pub use vkernel::ViabilityKernel;

/// Global lazy-loaded .eco-fairness.aln shard (JSON for maximum interoperability)
static ECO_FAIRNESS_SPEC: Lazy<RwLock<EcoFairnessSpec>> = Lazy::new(|| {
    let spec = EcoFairnessSpec::load("config/.eco-fairness.aln")
        .expect("Failed to load .eco-fairness.aln – this invariant must exist");
    RwLock::new(spec)
});

/// Per-subject & per-route live usage tracking (concurrent, sharded, zero-cost reads)
static CURRENT_USAGE: Lazy<DashMap<String, EcoEnvelope>> = Lazy::new(DashMap::new);

#[derive(Error, Debug)]
pub enum GuardError {
    #[error("Eco budget exceeded on route {route}: {resource} demand {demand} > limit {limit}")]
    BudgetExceeded { route: String, resource: String, demand: f64, limit: f64 },

    #[error("Equity violation for subject {subject}: below guaranteed minimum")]
    BelowMinimum { subject: String },

    #[error("RoH ceiling breach (current {current_roh} > {ceiling})")]
    RohCeilingBreach { current_roh: f64, ceiling: f64 },

    #[error("Viability kernel rejection: {reason}")]
    ViabilityFailure { reason: String },

    #[error("Altar route treated as governed compute – requires EVOLVE token")]
    AltarRequiresEvolve,
}

/// ALN/JSON friendly – direct mapping for .eco-fairness.aln shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoEnvelope {
    pub max_power_watts: f64,
    pub max_emissions_gco2eq: f64,
    pub max_compute_cycles: u64,
    pub priority_uplift_if_eco_positive: bool, // true for earth-restoring tasks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoFairnessSpec {
    pub global_roh_ceiling: f64,                   // 0.3 immutable unless EVOLVE+multisig
    pub global_eco_budget: EcoEnvelope,
    pub per_route_budgets: HashMap<String, EcoEnvelope>,
    pub per_subject_minimums: HashMap<String, EcoEnvelope>,
    pub altar_routes: Vec<String>,                 // donation/lesson scheduling routes
}

impl EcoFairnessSpec {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let spec: EcoFairnessSpec = serde_json::from_reader(file)?;
        info!("Loaded EcoFairnessSpec from {path}");
        Ok(spec)
    }
}

/// Core kernel – pure, stateless math + shared state queries
pub struct GraceEquityKernel {
    roh: RohModel,
    vkernel: ViabilityKernel,
}

impl GraceEquityKernel {
    pub fn new(roh: RohModel, vkernel: ViabilityKernel) -> Self {
        Self { roh, vkernel }
    }

    /// Short-abbreviation real-world fast path
    #[inline(always)]
    pub fn gek_check(&self, subject: &str, route: &str, demand: &EcoEnvelope) -> Result<(), GuardError> {
        self.check_route(subject, route, demand)
    }

    /// Full invariant check – called on every Auto_Church governed action
    pub fn check_route(&self, subject: &str, route: &str, demand: &EcoEnvelope) -> Result<(), GuardError> {
        let spec = ECO_FAIRNESS_SPEC.read();

        // 1. RoH ceiling (0.3) – hard invariant
        if self.roh.current_value() > spec.global_roh_ceiling {
            return Err(GuardError::RohCeilingBreach {
                current_roh: self.roh.current_value(),
                ceiling: spec.global_roh_ceiling,
            });
        }

        // 2. Per-route budgets
        if let Some(budget) = spec.per_route_budgets.get(route) {
            if demand.max_power_watts > budget.max_power_watts {
                return Err(GuardError::BudgetExceeded {
                    route: route.to_string(),
                    resource: "power".into(),
                    demand: demand.max_power_watts,
                    limit: budget.max_power_watts,
                });
            }
            // …repeat for emissions & cycles
        }

        // 3. Altar routes are NEVER free throughput
        if spec.altar_routes.contains(&route.to_string()) {
            return Err(GuardError::AltarRequiresEvolve);
        }

        // 4. Per-subject minimum service guarantee (equity floor)
        let usage = CURRENT_USAGE.entry(subject.to_string()).or_default();
        if let Some(minimum) = spec.per_subject_minimums.get(subject) {
            if usage.max_compute_cycles + demand.max_compute_cycles < minimum.max_compute_cycles {
                return Err(GuardError::BelowMinimum { subject: subject.into() });
            }
        }

        // 5. Viability kernel cross-check
        if !self.vkernel.is_viable(demand) {
            return Err(GuardError::ViabilityFailure {
                reason: "Demand outside Tsafe viability envelope".into(),
            });
        }

        // Success → atomically update live usage (dashmap is lock-free sharded)
        let mut entry = CURRENT_USAGE.entry(subject.to_string()).or_default();
        entry.max_power_watts += demand.max_power_watts;
        entry.max_emissions_gco2eq += demand.max_emissions_gco2eq;
        entry.max_compute_cycles += demand.max_compute_cycles;

        Ok(())
    }
}

/// Mandatory guardian – single point of truth for Eco+Equity
pub struct EcoFairnessGuard {
    kernel: GraceEquityKernel,
}

impl EcoFairnessGuard {
    pub fn new(roh: RohModel, vkernel: ViabilityKernel) -> Self {
        Self {
            kernel: GraceEquityKernel::new(roh, vkernel),
        }
    }

    /// Public API used by Tsafe Cortex Gate
    pub fn check(&self, action: &SovereignAction, route: RequestRoute) -> Result<(), GuardError> {
        let demand = EcoEnvelope::from_action(action); // mapping defined elsewhere
        self.kernel.gek_check(&action.subject_id, route.as_str(), &demand)
    }
}

/// Example integration into existing Tsafe Cortex Gate (drop into tsafe/src/cortex_gate.rs)
/// This makes EcoFairnessGuard MANDATORY for all Auto_Church routes
/*
use ecofairness_guardian::{EcoFairnessGuard, GuardError};

impl PolicyEngine {
    pub async fn authorize_request(&self, req: SovereignAction, route: RequestRoute) -> Result<(), Box<dyn std::error::Error>> {
        // …existing guards (AuraBoundaryGuard, SoulNonTradeableShield, etc.)

        // ← NEW MANDATORY ECO+EQUITY GUARD
        self.eco_fairness_guard
            .check(&req, route)
            .map_err(|e| {
                warn!("EcoFairnessGuard rejected {route:?} for {}: {e}", req.subject_id);
                e
            })?;

        // continue with actuation…
        Ok(())
    }
}
