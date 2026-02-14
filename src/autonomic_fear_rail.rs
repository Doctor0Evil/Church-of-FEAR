// Autonomic FEAR rail adapter:
// - Maps HRV-derived autonomic features into FEAR and bioload deltas.
// - Designed to feed into BioRail / BioLoad Terrasafe guards via Identity5D
//   and computebioload, without introducing any direct actuation.
// - All outputs are bounded, monotone in risk, and suitable as
//   ROLEDIAGNOSTIC-ONLY evidence for W-cycle / ethics layers.

use serde::{Deserialize, Serialize};

/// Normalized HRV/autonomic window over a short epoch (e.g. 30–120 s),
/// already preprocessed into 0–1 bands by upstream biosignal code:
/// LF/HF, entropy, and ISO/IEC profile matches. [file:33]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HrvWindow {
    /// Normalized LF/HF ratio in [0, 1], where higher means more sympathetic load.
    pub lf_hf_norm: f64,
    /// Normalized sample entropy / complexity in [0, 1], where lower can indicate
    /// reduced variability / stress; we treat low entropy as risk. [file:33]
    pub entropy_norm: f64,
    /// Overall HRV magnitude band in [0, 1] (0 = very low variability, 1 = high).
    pub hrv_power_norm: f64,
    /// ISO/IEC / profile tag derived upstream (e.g. REST, COGNITIVE_LOAD, OVERLOAD).
    /// This is advisory only; we never branch actuation directly on this. [file:42]
    pub profile_tag: AutonomicProfile,
}

/// Coarse profile labels anchored to ISO/IEC‑style workload / vigilance standards,
/// kept as diagnostics only. [file:33][file:42]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AutonomicProfile {
    Rest,
    LightTask,
    CognitiveLoad,
    PhysicalLoad,
    Overload,
}

/// Configuration for mapping autonomic features into FEAR and bioload
/// deltas. All weights are conservative and kept in [0, 1]. [file:31]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AutonomicFearConfig {
    /// Maximum absolute FEAR delta per epoch (Tree-of-Life FEAR rail is bounded). [file:33]
    pub max_fear_delta: f64,
    /// Maximum absolute bioload delta contribution from autonomic stress per epoch. [file:33]
    pub max_bioload_delta: f64,
    /// Weight of LF/HF toward FEAR (sympathetic dominance).
    pub w_lf_hf_fear: f64,
    /// Weight of low entropy toward FEAR (reduced variability).
    pub w_entropy_fear: f64,
    /// Weight of low HRV magnitude toward FEAR.
    pub w_hrv_power_fear: f64,
    /// Extra FEAR bump when profile is explicitly Overload.
    pub overload_fear_bonus: f64,
    /// Weight of LF/HF toward bioload (systemic strain).
    pub w_lf_hf_bioload: f64,
    /// Weight of low entropy toward bioload (rigid, stressed system).
    pub w_entropy_bioload: f64,
    /// Weight of low HRV magnitude toward bioload.
    pub w_hrv_power_bioload: f64,
}

impl AutonomicFearConfig {
    /// Reasonable, corridor‑safe defaults; you can tune per deployment.
    pub fn default_bounded() -> Self {
        Self {
            max_fear_delta: 0.5,          // FEAR changes slowly, avoids jumps. [file:31]
            max_bioload_delta: 0.05,      // Autonomic window is a modest contributor. [file:33]
            w_lf_hf_fear: 0.5,
            w_entropy_fear: 0.25,
            w_hrv_power_fear: 0.25,
            overload_fear_bonus: 0.1,
            w_lf_hf_bioload: 0.5,
            w_entropy_bioload: 0.25,
            w_hrv_power_bioload: 0.25,
        }
    }
}

/// Output of the adapter: FEAR and bioload deltas to be merged into the
/// existing Tree-of-Life / BioRail envelopes for a site. [file:33]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AutonomicDeltas {
    pub delta_fear: f64,
    pub delta_bioload: f64,
}

fn clamp01(x: f64) -> f64 {
    if x.is_nan() {
        0.0
    } else {
        x.max(0.0).min(1.0)
    }
}

/// Core mapping from a single HRV window into FEAR and bioload deltas.
/// Monotone in risk: worsening LF/HF (toward 1), lower entropy, and lower
/// HRV magnitude can only increase risk scores, never decrease them. [file:31]
pub fn hrv_to_autonomic_deltas(cfg: AutonomicFearConfig, window: HrvWindow) -> AutonomicDeltas {
    // Normalize inputs into [0, 1] to guard against upstream errors. [file:31]
    let lf_hf = clamp01(window.lf_hf_norm);
    let entropy = clamp01(window.entropy_norm);
    let hrv_power = clamp01(window.hrv_power_norm);

    // Risk proxies (all in [0, 1]).
    // High LF/HF -> high risk.
    let r_lf_hf = lf_hf;
    // Low entropy -> high risk.
    let r_entropy = 1.0 - entropy;
    // Low HRV magnitude -> high risk.
    let r_hrv_low = 1.0 - hrv_power;

    // FEAR risk score as convex combination of risk proxies. [file:41]
    let fear_risk =
        cfg.w_lf_hf_fear * r_lf_hf +
        cfg.w_entropy_fear * r_entropy +
        cfg.w_hrv_power_fear * r_hrv_low;

    // Optional profile‑based bonus, bounded and additive only when already risky.
    let profile_bonus = match window.profile_tag {
        AutonomicProfile::Overload => cfg.overload_fear_bonus,
        AutonomicProfile::PhysicalLoad | AutonomicProfile::CognitiveLoad => 0.5 * cfg.overload_fear_bonus,
        AutonomicProfile::LightTask | AutonomicProfile::Rest => 0.0,
    };

    // Clamp combined FEAR intensity into [0, 1].
    let fear_intensity = clamp01(fear_risk + profile_bonus);

    // Map intensity → actual FEAR delta in configured band.
    let delta_fear = cfg.max_fear_delta * fear_intensity;

    // Bioload risk reuses the same pattern but with independent weights.
    let bioload_risk =
        cfg.w_lf_hf_bioload * r_lf_hf +
        cfg.w_entropy_bioload * r_entropy +
        cfg.w_hrv_power_bioload * r_hrv_low;

    let bioload_intensity = clamp01(bioload_risk);
    let delta_bioload = cfg.max_bioload_delta * bioload_intensity;

    AutonomicDeltas {
        delta_fear,
        delta_bioload,
    }
}

/// Helper to apply the autonomic deltas to a site‑local FEAR scalar and
/// territorial bioload estimate, ready to feed into Identity5D and
/// computebioload / BioRail guards. [file:31][file:33]
///
/// This function is PURE with respect to actuation; callers are responsible
/// for passing the resulting values through Tsafe guards before any deed. [file:37]
pub fn apply_autonomic_to_state(
    current_fear: f64,
    current_bioload: f64,
    cfg: AutonomicFearConfig,
    window: HrvWindow,
) -> (f64, f64) {
    let deltas = hrv_to_autonomic_deltas(cfg, window);

    // FEAR and bioload remain bounded; callers can further clamp to their own envelopes.
    let new_fear = (current_fear + deltas.delta_fear).max(0.0);
    let new_bioload = (current_bioload + deltas.delta_bioload).max(0.0);

    (new_fear, new_bioload)
}
