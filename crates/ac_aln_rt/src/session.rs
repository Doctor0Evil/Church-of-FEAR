use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub bot_id: String,
    pub state: String,
    pub data: HashMap<String, serde_json::Value>,
}

impl Session {
    pub fn new(user_id: String, bot_id: String, state: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            bot_id,
            state: state.to_string(),
            data: HashMap::new(),
        }
    }
}
