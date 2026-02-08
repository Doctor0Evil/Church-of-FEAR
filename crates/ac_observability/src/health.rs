use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub ok: bool,
    pub message: String,
}

impl HealthStatus {
    pub fn ok(msg: &str) -> Self {
        Self {
            ok: true,
            message: msg.to_string(),
        }
    }

    pub fn degraded(msg: &str) -> Self {
        Self {
            ok: false,
            message: msg.to_string(),
        }
    }
}
