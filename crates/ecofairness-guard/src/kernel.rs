use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityBounds {
    pub min_share: f32,
    pub max_share: f32,
    /// Optional human-readable note.
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityClassSpec {
    pub name: String,
    pub min_share: f32,
    pub max_share: f32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteEnvelope {
    pub route: String,
    pub max_power_fraction: f32,
    pub max_compute_fraction: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraceEquityKernelSpec {
    pub resource_kind: String,
    pub normalization: String,
    pub classes: Vec<EquityClassSpec>,
    pub node_routes: Vec<RouteEnvelope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraceEquityKernel {
    /// Map from EquityClass name → bounds.
    pub classes: HashMap<String, EquityBounds>,
    pub resource_kind: String,
    pub normalization: String,
    pub node_routes: HashMap<String, RouteEnvelope>,
}

#[derive(thiserror::Error, Debug)]
pub enum EquityKernelError {
    #[error("I/O error loading .eco-fairness.aln: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error in .eco-fairness.aln: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Invalid equity kernel invariant: {0}")]
    Invariant(String),
}

impl GraceEquityKernel {
    /// Load and validate from a JSON-compatible `.eco-fairness.aln` file.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, EquityKernelError> {
        let raw = fs::read_to_string(path)?;
        let spec: GraceEquityKernelSpec = serde_json::from_str(&raw)?;

        if spec.classes.is_empty() {
            return Err(EquityKernelError::Invariant(
                "grace_equity_kernel.classes must not be empty".into(),
            ));
        }

        let mut sum_min = 0.0_f32;
        let mut classes = HashMap::new();

        for c in &spec.classes {
            if !(0.0..=1.0).contains(&c.min_share) {
                return Err(EquityKernelError::Invariant(format!(
                    "min_share for class '{}' must be in [0.0, 1.0], got {}",
                    c.name, c.min_share
                )));
            }
            if !(0.0..=1.0).contains(&c.max_share) {
                return Err(EquityKernelError::Invariant(format!(
                    "max_share for class '{}' must be in [0.0, 1.0], got {}",
                    c.name, c.max_share
                )));
            }
            if c.min_share > c.max_share {
                return Err(EquityKernelError::Invariant(format!(
                    "min_share > max_share for class '{}'",
                    c.name
                )));
            }
            sum_min += c.min_share;
            if classes
                .insert(
                    c.name.clone(),
                    EquityBounds {
                        min_share: c.min_share,
                        max_share: c.max_share,
                        description: c.description.clone(),
                    },
                )
                .is_some()
            {
                return Err(EquityKernelError::Invariant(format!(
                    "Duplicate EquityClass name '{}'",
                    c.name
                )));
            }
        }

        if sum_min > 1.0 + 1e-6 {
            return Err(EquityKernelError::Invariant(format!(
                "sum(min_share) must be ≤ 1.0, got {}",
                sum_min
            )));
        }

        let mut node_routes = HashMap::new();
        for env in &spec.node_routes {
            if env.max_power_fraction < 0.0
                || env.max_power_fraction > 1.0
                || env.max_compute_fraction < 0.0
                || env.max_compute_fraction > 1.0
            {
                return Err(EquityKernelError::Invariant(format!(
                    "Route '{}' envelopes must be in [0.0, 1.0]",
                    env.route
                )));
            }
            node_routes.insert(env.route.clone(), env.clone());
        }

        Ok(Self {
            classes,
            resource_kind: spec.resource_kind,
            normalization: spec.normalization,
            node_routes,
        })
    }

    pub fn bounds_for_class(&self, class: &str) -> Option<&EquityBounds> {
        self.classes.get(class)
    }

    pub fn route_envelope(&self, route: &str) -> Option<&RouteEnvelope> {
        self.node_routes.get(route)
    }
}
