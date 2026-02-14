use serde::{Deserialize, Serialize};

/// Scalar snapshot for HPCC/ERG/TECR context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalarContext {
    // HPCC via RoH traces
    pub roh_before: f32,   // 0.0–0.3 in CapControlledHuman
    pub roh_peak: f32,     // max RoH during window
    pub roh_after: f32,    // 0.0–0.3

    // TECR via DECAY/LIFEFORCE over window
    pub decay_min: f32,    // 0.0–1.0
    pub decay_max: f32,    // 0.0–1.0
    pub lifeforce_min: f32,// 0.0–1.0
    pub lifeforce_max: f32,// 0.0–1.0

    // ERG via NATURE window counts
    pub calm_stable_epochs: u32,
    pub overloaded_epochs: u32,
    pub recovery_epochs: u32,

    // Event density (TECR: TIME/NANO)
    pub nano_events: u32,  // number of evolve events in window
}

/// Qualitative BIOTREE/NATURE/GOAL summary for the window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitativeContext {
    pub biotree: serde_json::Value, // e.g., { "fear_level": "high", ... }
    pub nature: serde_json::Value,  // e.g., { "label_main": "...", ... }
    pub goal: serde_json::Value,    // e.g., { "intent": "...", ... }
}

/// A single discipline contribution record for .evolve.jsonl.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisciplineContribution {
    pub timestamp_ms_start: u64,
    pub timestamp_ms_end: u64,
    pub subject_id: String,
    pub discipline_window_id: String,

    // Safety and resilience context
    pub scalar: ScalarContext,

    // FEAR/PAIN trajectories (simple aggregates)
    pub fear_avg: f32,   // 0.0–1.0
    pub fear_max: f32,   // 0.0–1.0
    pub pain_avg: f32,   // 0.0–1.0
    pub pain_max: f32,   // 0.0–1.0,

    // Qualitative BIOTREE/NATURE/GOAL
    pub qualitative: Option<QualitativeContext>,

    // Subject-stated purpose, for respecting contribution intent.
    pub subject_purpose: Option<String>, // e.g., "improve medicine"
}

/// Pure helper to serialize a contribution as one JSONL line.
/// This is intentionally IO-free; caller is responsible for appending.
pub fn discipline_contribution_to_jsonl_line(
    contrib: &DisciplineContribution,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(contrib).map(|s| {
        let mut line = s;
        line.push('\n');
        line
    })
}
