use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipient {
    pub id: String,
    pub name: String,
    pub project: String,
}

impl Recipient {
    pub fn new(id: String, name: String, project: String) -> Self {
        Self { id, name, project }
    }
}
