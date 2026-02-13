use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerConfig {
    pub roh_max: f64,
    pub decay_max: f64,
    pub token_reward_factor: u64,
    pub repair_pwr_threshold: f64,
}

impl Default for LedgerConfig {
    fn default() -> Self {
        Self {
            roh_max: 0.3,
            decay_max: 1.0,
            token_reward_factor: 100,
            repair_pwr_threshold: 0.8,
        }
    }
}
