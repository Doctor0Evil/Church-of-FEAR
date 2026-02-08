use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyPolicy {
    pub no_harm_to_life: bool,
    pub eco_priority: bool,
    pub transparency_required: bool,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self {
            no_harm_to_life: true,
            eco_priority: true,
            transparency_required: true,
        }
    }
}
