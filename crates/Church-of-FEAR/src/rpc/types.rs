use serde::{Deserialize, Serialize};
use crate::ledger::deed_event::DeedEvent;
use crate::ledger::metrics::BioloadMetrics;

/// Generic JSON-RPC 2.0 envelope.

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
    #[serde(default)]
    pub id: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

// ---- Auto_Church specific payloads ----

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoChurchMintParams {
    pub prev_hash: String,
    pub actor_id: String,
    pub target_ids: Vec<String>,
    pub deed_type: String,
    pub tags: Vec<String>,
    pub context_json: serde_json::Value,
    pub ethics_flags: Vec<String>,
    pub life_harm_flag: bool,
    pub bioload_delta: f64,
    pub roh: f64,
    pub decay: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoChurchMintResult {
    pub deed: DeedEvent,
    pub metrics: BioloadMetrics,
    pub church_minted: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoChurchValidateParams {
    pub deed: DeedEvent,
    pub roh: f64,
    pub decay: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoChurchValidateResult {
    pub valid: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoChurchVisualizeParams {
    pub events: Vec<DeedEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoChurchVisualizeResult {
    /// Placeholder: in-process visualizations do not return a serializable App,
    /// so the RPC just acknowledges that the visualization was launched.
    pub launched: bool,
}
