use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricId(pub String);

impl MetricId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricKind {
    Throughput,
    Latency,
    ErrorRate,
    EcoCost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub id: MetricId,
    pub name: String,
    pub kind: MetricKind,
    pub value: f64,
    pub unit: String,
}

impl Metric {
    pub fn new(name: &str, kind: MetricKind, value: f64, unit: &str) -> Self {
        Self {
            id: MetricId::new(),
            name: name.to_string(),
            kind,
            value,
            unit: unit.to_string(),
        }
    }
}
