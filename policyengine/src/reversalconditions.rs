use serde::{Deserialize, Serialize};

use crate::biosafe::BiosafePolytope;        // RoH, DECAY, lifeforce, unfairdrain.[file:2]
use crate::capability::CapabilityState;     // Capability lattice; includes CHURCH/POWER roles.[file:5]
use crate::envelope::EnvelopeSnapshot;      // Biophysical envelopes, minsafe/maxsafe bands.[file:2]
use crate::evidence::EvidenceBundle;        // 10-tag ALN evidence object.[file:2]
use crate::sovereign::SovereignMultisig;    // Neuromorph-GOD / jurisdiction attestation.[file:1]

/// Evidence flags derived from diagnostics (Tree-of-FEAR, FateWindow, NATURE).
/// These are coarse, DIAGNOSTIC-ONLY derived booleans, never raw predicates.[file:1][file:2]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct EvidenceFlags {
    /// True if all biosafe corridor checks pass (RoH ≤ 0.3, no UNFAIRDRAIN, lifeforce floor).[file:1][file:2]
    pub corridor_safe: bool,
    /// True if the FateWindow for this decision is valid (no RoH ceiling or envelope breach).[file:2]
    pub window_valid: bool,
    /// True if independent diagnostics show that no safer alternative policy exists.[file:1]
    pub no_safer_alternative: bool,
    /// True if overload / harm has been detected in the relevant window; used only as evidence.[file:1]
    pub overload_present: bool,
}

/// The kind of reversal request being *evaluated*.
/// Note: the kernel NEVER performs the reversal itself.[file:1]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReversalKind {
    /// A proposal to *further tighten* or maintain an already-tight capability bound.
    CapabilityTightening,
    /// A proposal to shrink biophysical envelopes (more protection), never expand.[file:2]
    EnvelopeTightening,
    /// A last-resort request to enter a safe-halt / repair corridor.[file:1]
    EmergencySafeHalt,
}

/// Inputs required to decide whether a proposed change is ethically admissible.
/// All fields are readonly views; this kernel has no authority to mutate host state.[file:1][file:2]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalContext {
    pub current_capability: CapabilityState,
    pub proposed_capability: CapabilityState,

    /// RoH snapshot before and after the *proposal*; “after” is hypothetical.[file:2]
    pub roh_before: f32,
    pub roh_after: f32,

    pub polytope_before: BiosafePolytope,
    pub polytope_after: BiosafePolytope,

    pub envelope_before: EnvelopeSnapshot,
    pub envelope_after: EnvelopeSnapshot,

    pub evidence: EvidenceBundle,      // Must be complete, shard-anchored, 10 tags.[file:2]
    pub evidence_flags: EvidenceFlags, // Derived diagnostics only.[file:1][file:2]
    pub sovereign: SovereignMultisig,  // Sovereignty / roles consensus.[file:1]

    pub reversal_kind: ReversalKind,
}

/// Canonical decision reasons. These are *judgements*, not commands.[file:1][file:2]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionReason {
    /// The proposal is ethically admissible (non-predatory, non-savage) but still non-actuating.
    /// Higher layers may choose to *consider* it; this kernel does not apply it.[file:1]
    AdmissibleTightening,

    /// The system must enter a repair / safe-halt corridor; no predatory rollback allowed.[file:1]
    RequireRepairSafeHalt,

    /// Biosafe corridor (RoH, DECAY, lifeforce) would be violated by the proposed path.[file:2]
    DeniedRoHViolation,
    /// Biophysical envelopes would be relaxed or minsafe/maxsafe widened in a harmful way.[file:2]
    DeniedEnvelopeViolation,
    /// Proposal implies or relies on unfair drain across roles or species.[file:1]
    DeniedUnfairDrain,
    /// Proposal breaks monotonicity of safety (would increase RoH or loosen constraints).[file:1]
    DeniedMonotonicityViolation,
    /// Evidence bundle is incomplete, inconsistent, or not anchored as required.[file:2]
    DeniedEvidenceFailure,
    /// Sovereign multi-sig (Neuromorph-GOD, roles, jurisdictions) is not satisfied.[file:1]
    DeniedSovereigntyFailure,
    /// Proposal would effectively upgrade capability or relax envelopes (“reverse to exploit”).[file:1]
    DeniedUnauthorizedUpgrade,
    /// Proposal attempts to use reversal to re-open harmful or predatory corridors.[file:1]
    DeniedPredatoryReversal,
}

/// Pure decision kernel: evaluates ethical admissibility of *proposed* changes.
/// It can only deny or demand repair/safe-halt; it never enacts a reversal.[file:1][file:2]
pub fn evaluate_reversal(ctx: &ReversalContext) -> DecisionReason {
    // 1. Hard biosafe corridor: RoH, DECAY, lifeforce within legal ranges.[file:2]
    if !ctx.polytope_before.is_legal_corridor() || !ctx.polytope_after.is_legal_corridor() {
        return DecisionReason::DeniedRoHViolation;
    }

    // 2. Envelopes MUST be non-expansive: no relaxation, no extra room for harm.[file:2]
    if !ctx.envelope_after.is_nonexpansive_vs(&ctx.envelope_before) {
        return DecisionReason::DeniedEnvelopeViolation;
    }

    // 3. UNFAIRDRAIN must remain false (no asymmetric biophysical exploitation).[file:1]
    if ctx.polytope_after.unfairdrain {
        return DecisionReason::DeniedUnfairDrain;
    }

    // 4. Safety monotonicity: no increase in RoH, even during “reversal”.
    //    Any attempt to raise RoH is structurally forbidden.[file:1]
    if ctx.roh_after > ctx.roh_before + f32::EPSILON {
        return DecisionReason::DeniedMonotonicityViolation;
    }

    // 5. Evidence integrity: full 10-tag bundle, valid ALN shard linkage, corridor-safe flags.[file:2]
    if !ctx.evidence.is_complete_and_valid() {
        return DecisionReason::DeniedEvidenceFailure;
    }
    if !ctx.evidence_flags.corridor_safe || !ctx.evidence_flags.window_valid {
        return DecisionReason::DeniedEvidenceFailure;
    }

    // 6. Sovereignty: Neuromorph-GOD invariants and multi-role consent must be satisfied.[file:1]
    if !ctx.sovereign.is_fully_attested_for_reversal() {
        return DecisionReason::DeniedSovereigntyFailure;
    }

    // 7. Structural prohibition: no upgrade or relaxation via “reversal”.
    //    Capability must be ≤ current in the lattice; envelopes already checked above.[file:1]
    if !ctx
        .proposed_capability
        .is_nonexpansive_vs(&ctx.current_capability)
    {
        return DecisionReason::DeniedUnauthorizedUpgrade;
    }

    // 8. Explicit anti-predation check: reversal cannot be used to re-open corridors
    //    that diagnostics mark as harmful (BEAST/PLAGUE, persistent UNFAIRDRAIN, etc.).[file:1]
    if ctx.evidence_flags.overload_present && !ctx.evidence_flags.no_safer_alternative {
        // Someone is trying to “reverse” while a safer, less harmful alternative exists.[file:1]
        return DecisionReason::DeniedPredatoryReversal;
    }

    // 9. Emergency safe-halt: permitted only when no safer alternative exists.[file:1]
    match ctx.reversal_kind {
        ReversalKind::EmergencySafeHalt => {
            if !ctx.evidence_flags.no_safer_alternative {
                // Safe-halt cannot be used as an excuse to abandon fair repair paths.[file:1]
                DecisionReason::DeniedPredatoryReversal
            } else {
                // Demand entry into repair/safe-halt corridor; higher layers implement it.[file:1]
                DecisionReason::RequireRepairSafeHalt
            }
        }
        ReversalKind::CapabilityTightening | ReversalKind::EnvelopeTightening => {
            // Pure tightening that passes all fairness and biosafe checks is admissible.[file:1][file:2]
            DecisionReason::AdmissibleTightening
        }
    }
}
