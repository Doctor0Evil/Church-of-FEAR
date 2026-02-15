// Module: Core manifest struct and traits for inner/outer domains. Implements serialization,
// verification, and RAF accumulation. Ensures non-actuation: reads only physical stressors,
// never neural data.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use nalgebra::{DMatrix, DVector};  // For A_eco x <= b_eco polytopes
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey};
use hex::{encode, decode};
use zeroize::Zeroize;

pub mod inner_domain;
pub mod outer_domain;
pub mod extensions;
pub mod signaling;

pub use inner_domain::{NeurorightInvariant, InnerEnvelope};
pub use outer_domain::{EcoAdmissible, KarmaAdmissible, SafetyPolytope};
pub use extensions::{RafAccumulator, BeeWeightedOp, ErrorityEvent};
pub use signaling::{WordMathScore, DutyHeader, LiveDelta};

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Invalid DID signature")]
    InvalidSignature,
    #[error("Polytope violation: {0}")]
    PolytopeViolation(String),
    #[error("RAF accumulation failed: {0}")]
    RafError(String),
    #[error("Hex-stamp mismatch")]
    HexMismatch,
}

/// Core NeuroEcoIdentityManifest: DID-bound, layered governance object.
/// Static anchors: rights flags, evidence bundles.
/// Real-time signals: RAF deltas, duty headers.
/// Interoperable with CEIM (M_j = C_u,j (C_in - C_out) Q t), NanoKarma (K_i = λ_i β_i M_i).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NeuroEcoIdentityManifest {
    #[serde(rename = "@context")]
    context: Vec<String>,  // JSON-LD: ["https://www.w3.org/ns/credentials/v2", "ceim://v1.2", "nanokarma://op"]
    id: String,  // DID: "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"
    r#type: String,  // "NeuroEcoIdentityManifest"
    issuer: String,  // Self-issued DID
    issuance_date: DateTime<Utc>,
    inner_domain: InnerEnvelope,
    outer_domain: OuterDomainConfig,
    extensions: Vec<Extension>,
    evidence_bundles: Vec<HexStampedBundle>,
    signatures: Vec<DidSignature>,
    exclusions: Exclusions,
    live_metrics: Option<LiveMetrics>,  // Real-time: RAF, deltas
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OuterDomainConfig {
    ceim_ref: String,  // URI to CEIM engine
    nanokarma_op: NanoKarmaOp,
    polytopes: Vec<SafetyPolytope>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NanoKarmaOp {
    lambda: DVector<f64>,  // Hazard weights (bee-elevated for VOCs/PM2.5)
    beta: DVector<f64>,    // Normalization (jurisdictional LCIA)
    k_person_current: f64, // Cumulative ∑ K_i
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Extension {
    r#type: String,  // e.g., "RafAccumulator", "BeeWeightedPolytope"
    depends_on: Vec<String>,  // ["ceim", "nanokarma"]
    params: serde_json::Value,  // RAF formula, HB-rating 9.7/10
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HexStampedBundle {
    id: String,  // Hex hash of bundle contents
    bundle_type: String,  // "CEIMModel", "BeeSensitivityStudy"
    uri: String,  // IPFS/HTTPS
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DidSignature {
    key_id: String,
    signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Exclusions {
    allows_neural_intrusion: bool,  // false
    standalone_normative: bool,     // false
    interoperability: Vec<String>,  // ["W3C DID v2", "CEIM v1.2", "NanoKarma"]
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiveMetrics {
    raf_global: f64,
    raf_bee: f64,
    k_deltas: KarmaDeltas,
    word_math: WordMathScore,
    duty_header: DutyHeader,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KarmaDeltas {
    day: f64,  // ΔK over 24h
    week: f64, // ΔK over 7d
}

impl NeuroEcoIdentityManifest {
    /// RAF_delta: Short-abbrev fn for CHURCH earning. Computes pos/neg mass impacts via CEIM -> NanoKarma.
    /// Earns TECH/NANO by simulating restorative actions (e.g., +0.15 for Cybo-Air toxin removal).
    pub fn raf_delta(&self, m_pos: DVector<f64>, m_neg: DVector<f64>) -> Result<f64, ManifestError> {
        let sigma = DVector::from_element(m_pos.len(), 10.0);  // Normalization: 10 kg/person/year baseline
        let delta_r = (self.outer_domain.nanokarma_op.lambda.component_mul(&m_pos)
                       - self.outer_domain.nanokarma_op.lambda.component_mul(&m_neg))
                      .component_div(&sigma)
                      .sum();
        if delta_r < -0.3 {  // Threshold for Errority trigger
            Err(ManifestError::RafError("High negative delta; log Errority".to_string()))
        } else {
            Ok(delta_r)  // Positive/zero: earns eco-grant simulation
        }
    }

    /// ECO_ADMISS: Polytope check for action x_proj. Zero-harm: rejects if violates P_eco or P_bee.
    pub fn eco_admissible(&self, x_proj: &DVector<f64>) -> bool {
        self.outer_domain.polytopes.iter().all(|p| {
            let residual = &p.a * x_proj - &p.b;
            residual.max() <= 0.0  // A x <= b
        })
    }

    /// BEE_WEIGHT: Scales λ_i for pollinators (1.5x human for VOCs/PM2.5). HB-rating 9.7/10 sim.
    pub fn bee_weight(&self, stressor_idx: usize) -> f64 {
        let base_lambda = self.outer_domain.nanokarma_op.lambda[stressor_idx];
        if stressor_idx == 3 || stressor_idx == 4 {  // VOCs, PM2.5 indices
            base_lambda * 1.5
        } else {
            base_lambda
        }
    }

    /// ERR_LOG: Emits Errority event for refinement. Non-punitive: feeds polytope updates, earns WISE via learning.
    pub fn err_log(&mut self, event: ErrorityEvent) -> HexStampedBundle {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&event).unwrap().as_bytes());
        let hash = encode(hasher.finalize());
        self.evidence_bundles.push(HexStampedBundle {
            id: hash.clone(),
            bundle_type: "ErrorityEvent".to_string(),
            uri: format!("ipfs://{}", hash),  // Placeholder for actual IPFS
            timestamp: Utc::now(),
        });
        HexStampedBundle { id: hash, ..Default::default() }  // Returns stamped bundle
    }

    /// HEX_STAMP: Bundles evidence for verification. Ensures tamper-evidence for good-deed ledgers.
    pub fn hex_stamp(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        encode(hasher.finalize())
    }

    /// Verify signature: Ensures DID-bound integrity for non-reversal rights.
    pub fn verify_signature(&self, verifying_key: &VerifyingKey, data: &[u8], sig: &[u8]) -> Result<(), ManifestError> {
        verifying_key.verify(data, &ed25519_dalek::Signature::from_bytes(sig).map_err(|_| ManifestError::InvalidSignature)?)
            .map_err(|_| ManifestError::InvalidSignature)
    }
}

/// System-object: Default manifest for Phoenix, AZ baseline (user loc). Initializes with r0=0.5, bee-focus.
impl Default for NeuroEcoIdentityManifest {
    fn default() -> Self {
        Self {
            context: vec!["https://www.w3.org/ns/credentials/v2".to_string(), "ceim://v1.2".to_string()],
            id: "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            r#type: "NeuroEcoIdentityManifest".to_string(),
            issuer: "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            issuance_date: Utc::now(),
            inner_domain: InnerEnvelope::default(),  // Absolute: noNeuralInputs=true
            outer_domain: OuterDomainConfig {
                ceim_ref: "ceim://v1.2".to_string(),
                nanokarma_op: NanoKarmaOp {
                    lambda: DVector::from_vec(vec![1.0, 1.2, 1.5, 2.25, 2.25]),  // Bee-weighted VOC/PM2.5
                    beta: DVector::from_element(5, 1.0),
                    k_person_current: 0.0,
                },
                polytopes: vec![SafetyPolytope::default()],  // P_eco baseline
            },
            extensions: vec![Extension {
                r#type: "RafAccumulator".to_string(),
                depends_on: vec!["nanokarma".to_string()],
                params: serde_json::json!({ "initial_r": 0.5, "hb_rating": 9.7 }),
            }],
            evidence_bundles: vec![],
            signatures: vec![],
            exclusions: Exclusions {
                allows_neural_intrusion: false,
                standalone_normative: false,
                interoperability: vec!["W3C DID v2".to_string(), "CEIM v1.2".to_string()],
            },
            live_metrics: None,
        }
    }
}

// Tests: Ensure monotonic RAF (good-deed earning), polytope zero-harm, Errority non-punitive.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raf_delta_positive_eco_grant() {
        let manifest = NeuroEcoIdentityManifest::default();
        let m_pos = DVector::from_vec(vec![5.0, 0.0]);  // 5kg CO2 removed
        let m_neg = DVector::from_vec(vec![0.0, 0.0]);
        let delta = manifest.raf_delta(m_pos, m_neg).unwrap();
        assert!(delta > 0.0);  // Earns +TECH for restoration
    }

    #[test]
    fn test_eco_admissible_bee_safe() {
        let manifest = NeuroEcoIdentityManifest::default();
        let x_proj = DVector::from_vec(vec![0.1, 0.05]);  // Low PM2.5/VOC
        assert!(manifest.eco_admissible(&x_proj));  // Passes, earns NANO sim
    }

    #[test]
    fn test_err_log_refinement() {
        let mut manifest = NeuroEcoIdentityManifest::default();
        let event = ErrorityEvent { description: "Polytope edge-case".to_string(), delta_r: -0.1 };
        let bundle = manifest.err_log(event);
        assert!(!bundle.id.is_empty());  // Stamped, feeds WISE learning
    }
}
