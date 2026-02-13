use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grant {
    pub id: String,
    pub recipient_id: String,
    pub amount_pwr: u64,
    pub description: String,
}

impl Grant {
    pub fn new(id: String, recipient_id: String, amount_pwr: u64, description: String) -> Self {
        Self {
            id,
            recipient_id,
            amount_pwr,
            description,
        }
    }
}
