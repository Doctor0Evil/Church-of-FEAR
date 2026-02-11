use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::crypto::compute_sha256_hash;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeedEvent {
    pub event_id: String,
    pub timestamp: u64,
    pub prev_hash: String,
    #[serde(skip_serializing)]
    pub self_hash: String,
    pub actor_id: String,
    pub target_ids: Vec<String>,
    pub deed_type: String,
    pub tags: Vec<String>,
    pub context_json: Value,
    pub ethics_flags: Vec<String>,
    pub life_harm_flag: bool,
}

impl DeedEvent {
    pub fn compute_self_hash(&self) -> String {
        let serialized = serde_json::to_string(&self).expect("Serialization failed");
        compute_sha256_hash(serialized.as_bytes())
    }

    pub fn is_good_deed(&self) -> bool {
        !self.life_harm_flag && self.ethics_flags.is_empty() &&
        self.tags.iter().any(|t| matches!(t.as_str(), "ecological_sustainability" | "homelessness_relief" | "math_science_education"))
    }
}
