#[derive(Debug, Clone)]
pub struct EcoRegEnvelope {
    pub roh_max: f64,
    pub decay_max: f64,
}

impl Default for EcoRegEnvelope {
    fn default() -> Self {
        Self {
            roh_max: 0.3,
            decay_max: 1.0,
        }
    }
}

impl EcoRegEnvelope {
    pub fn within_bounds(&self, roh: f64, decay: f64) -> bool {
        roh <= self.roh_max && decay <= self.decay_max
    }
}
