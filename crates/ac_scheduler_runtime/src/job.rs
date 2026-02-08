use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobKind {
    GitMaintenance,
    EcoScan,
    AuditLineage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobId(pub String);

impl JobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub kind: JobKind,
    pub payload: serde_json::Value,
}

impl Job {
    pub fn new(kind: JobKind, payload: serde_json::Value) -> Self {
        Self {
            id: JobId::new(),
            kind,
            payload,
        }
    }
}
