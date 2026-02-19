#![forbid(unsafe_code)]

use std::time::SystemTime;

use crate::deed_log::{DeedEvent, DeedEventKind};
use crate::ids::{UpgradeId, MicrospaceId, JurisdictionId};
use crate::policy::{ReversalPolicy, RoleId, RoleSet};
use crate::proofs::{ProofClass, ProofHandle};
use crate::risk::{IncidentStats, RiskBand};

/// High-level autonomy tier of a behavior or upgrade.
///
/// Every UpgradeDescriptor / behavior must declare its tier; moving upward
/// requires additional evidence and governance multi-sig.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutonomyTier {
    /// Simulation-only; neuromorph-sim and non-actuating tests.
    SimulationOnly,
    /// Host-local; acts only on the augmented host's own body/BCI.
    HostLocal,
    /// Corridor-bound; may touch nanoswarm / eco-nodes, but only within
    /// proven-safe envelopes and without cross-jurisdiction impact.
    CorridorBound,
    /// Eco-node; can actuate shared infrastructure (e.g., Phoenix water nodes).
    EcoNode,
    /// Global network; can influence multi-region systems across jurisdictions.
    GlobalNet,
}

/// Whether a behavior still requires a runtime rollback path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonRollbackStatus {
    /// Full rollback is required for safety (default for all new behaviors).
    Experimental,
    /// Rollback still expected, but behavior has passed initial field tests.
    Provisional,
    /// Behavior is proven-safe for normal operation without routine rollback;
    /// only emergency detox / hard kill remain as safety nets.
    Settled,
}

/// Evidence required to settle a behavior into non-rollback status.
///
/// This aggregates CEIM/CPVM proofs, field statistics, and biophysical tags.
#[derive(Clone, Debug)]
pub struct NonRollbackEvidence {
    /// Proof that mass-balance constraints (e.g., CEIM) hold over horizon H.
    pub ceim_proof: ProofHandle,
    /// Proof that viability kernels (e.g., CPVM) are respected by this behavior.
    pub cpvm_proof: ProofHandle,
    /// Minimum observation horizon in days.
    pub observation_horizon_days: u32,
    /// Maximum allowed incident rate per 1000 sessions over the horizon.
    pub max_incident_rate_per_1k_sessions: f32,
    /// Empirical incident statistics gathered from field telemetry.
    pub incident_stats: IncidentStats,
    /// Ten short-hex tags grounding the evidence in your global registry.
    /// These must be registered and documented elsewhere.
    pub biophys_hex: [&'static str; 10],
}

impl NonRollbackEvidence {
    /// Convenience constructor with a standard 10-tag chain.
    pub fn new_standard(
        ceim_proof: ProofHandle,
        cpvm_proof: ProofHandle,
        observation_horizon_days: u32,
        max_incident_rate_per_1k_sessions: f32,
        incident_stats: IncidentStats,
    ) -> Self {
        Self {
            ceim_proof,
            cpvm_proof,
            observation_horizon_days,
            max_incident_rate_per_1k_sessions,
            incident_stats,
            biophys_hex: [
                "71ac02d1", // CEIM mass-balance corridor for Phoenix MAR basins.
                "4be29c03", // CPVM viability kernel residual bounds for pumps/valves.
                "a1f3c9b2", // Host ATP / Blood-token mapping under mixed BCI+eco duty.
                "2f8c6b44", // Thermodynamic envelope (host core/local ΔT) evidence.
                "7e1da2ff", // Neurovascular / HRV coupling under neuromorphic load.
                "5b93e0c3", // Nitrate / PFAS detox and nanoswarm corridor safety.
                "d0174aac", // Duty-cycle envelopes for BCI + eco actuators.
                "6ac2f9d9", // Neuromorphic hardware energy / latency characterization.
                "c4e61b20", // Pain/inflammation rollback thresholds in field studies.
                "8f09d5ee", // Jurisdictional and microspace ALN compliance audits.
            ],
        }
    }

    /// Quick check that incident statistics are within the declared ceiling.
    pub fn incidents_within_ceiling(&self) -> bool {
        if self.incident_stats.sessions_observed == 0 {
            return false;
        }
        let rate = (self.incident_stats.total_incidents as f32
            / self.incident_stats.sessions_observed as f32)
            * 1000.0;
        rate <= self.max_incident_rate_per_1k_sessions
    }
}

/// Request to move an upgrade/behavior into a higher autonomy tier and/or
/// a more permissive NonRollbackStatus.
#[derive(Clone, Debug)]
pub struct SettlementRequest {
    pub upgrade_id: UpgradeId,
    pub current_tier: AutonomyTier,
    pub requested_tier: AutonomyTier,
    pub current_nonrollback: NonRollbackStatus,
    pub requested_nonrollback: NonRollbackStatus,
    /// Microspaces that this behavior touches (tissues, aquifer cells, nodes).
    pub microspaces: Vec<MicrospaceId>,
    /// Jurisdictions that must co-approve (e.g., PHX, GVA, BRU).
    pub jurisdictions: Vec<JurisdictionId>,
    /// Roles participating in the settlement multi-sig.
    pub roles: RoleSet,
    /// CEIM/CPVM and field evidence bundle.
    pub evidence: NonRollbackEvidence,
    /// Linked proofs (e.g., Googolswarm transactions, ceim/cpvm theorems).
    pub proofs: Vec<ProofHandle>,
    /// Current reversal policy (must remain non-empty even for Settled).
    pub reversal_policy: ReversalPolicy,
    /// Timestamp at which the request was assembled.
    pub assembled_at: SystemTime,
}

/// Result of evaluating a settlement request.
#[derive(Clone, Debug)]
pub struct SettlementDecision {
    pub approved: bool,
    pub reason: Option<String>,
    pub new_tier: AutonomyTier,
    pub new_nonrollback: NonRollbackStatus,
}

impl SettlementDecision {
    pub fn denied(reason: impl Into<String>) -> Self {
        Self {
            approved: false,
            reason: Some(reason.into()),
            new_tier: AutonomyTier::SimulationOnly,
            new_nonrollback: NonRollbackStatus::Experimental,
        }
    }

    pub fn approved(
        tier: AutonomyTier,
        nonrollback: NonRollbackStatus,
    ) -> Self {
        Self {
            approved: true,
            reason: None,
            new_tier: tier,
            new_nonrollback: nonrollback,
        }
    }
}

/// Core governance check to decide whether a behavior can be treated as
/// "settled" and thus operate without routine rollback, within its corridors.
///
/// This does *not* remove emergency detox/kill; it only allows the runtime
/// scheduler to stop carrying per-session rollback bookkeeping once safety
/// and ethics are proven by policy and usage.
pub fn can_settle_to_nonrollback(
    req: &SettlementRequest,
) -> SettlementDecision {
    // 1. NonRollbackStatus must only move forward, never backward here.
    if matches!(
        (req.current_nonrollback, req.requested_nonrollback),
        (NonRollbackStatus::Settled, NonRollbackStatus::Experimental)
            | (NonRollbackStatus::Settled, NonRollbackStatus::Provisional)
            | (NonRollbackStatus::Provisional, NonRollbackStatus::Experimental)
    ) {
        return SettlementDecision::denied(
            "NonRollbackStatus can only progress, not regress, in this path.",
        );
    }

    // 2. Require at least HostLocal tier before considering non-rollback.
    if req.current_tier == AutonomyTier::SimulationOnly {
        return SettlementDecision::denied(
            "Simulation-only behaviors cannot be settled; deploy to HostLocal first.",
        );
    }

    // 3. Ensure required roles are present (host, ethics, regulator, eco-node).
    let required_roles: [RoleId; 4] = [
        RoleId::HostConsent,
        RoleId::EthicsBoard,
        RoleId::RegulatorQuorum,
        RoleId::EcoNodeOperator,
    ];
    if !req.roles.contains_all(&required_roles) {
        return SettlementDecision::denied(
            "Missing required multi-sig roles (Host, Ethics, Regulator, EcoNode).",
        );
    }

    // 4. Check that CEIM and CPVM proofs exist and are in the right classes.
    let ceim_ok = req
        .proofs
        .iter()
        .any(|p| p.class == ProofClass::CeimMassBalance
            && p.id == req.evidence.ceim_proof.id);
    let cpvm_ok = req
        .proofs
        .iter()
        .any(|p| p.class == ProofClass::CpvmViability
            && p.id == req.evidence.cpvm_proof.id);
    if !ceim_ok || !cpvm_ok {
        return SettlementDecision::denied(
            "Missing CEIM/CPVM proofs for requested settlement.",
        );
    }

    // 5. Require sufficient observation horizon and low incident rate.
    if req.evidence.observation_horizon_days < 90 {
        return SettlementDecision::denied(
            "Observation horizon too short; require ≥ 90 days of field data.",
        );
    }
    if !req.evidence.incidents_within_ceiling() {
        return SettlementDecision::denied(
            "Incident rate exceeds allowed ceiling for non-rollback settlement.",
        );
    }

    // 6. Reversal policy must remain present even when Settled (emergency use).
    if req.reversal_policy.is_empty() {
        return SettlementDecision::denied(
            "ReversalPolicy must never be empty; keep emergency detox/kill.",
        );
    }

    // 7. Jurisdiction and microspace alignment: all microspaces touched by this
    // behavior must be explicitly listed, and all relevant jurisdictions must
    // be part of the ALN shard validated by external auditors (checked in ALN).
    if req.microspaces.is_empty() || req.jurisdictions.is_empty() {
        return SettlementDecision::denied(
            "Microspaces and jurisdictions must be explicitly enumerated.",
        );
    }

    // If all checks pass, allow the requested tier and non-rollback status.
    let decision =
        SettlementDecision::approved(req.requested_tier, req.requested_nonrollback);

    // Emit a DeedEvent for the audit log.
    let _deed = DeedEvent::new(
        DeedEventKind::NonRollbackSettlementApproved,
        req.upgrade_id,
        decision.new_tier,
        decision.new_nonrollback,
        req.roles.clone(),
        req.proofs.clone(),
        req.assembled_at,
    );
    // In a full implementation, this DeedEvent would be persisted and may be
    // anchored to Googolswarm / Cybernet as an immutable audit record.

    decision
}
