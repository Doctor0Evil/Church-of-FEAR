use serde::{Deserialize, Serialize};

pub type Scalar = f64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TreeOfLifeState {
    pub church: Scalar,
    pub fear: Scalar,
    pub power: Scalar,
    pub tech: Scalar,
    pub bioload: Scalar,
    pub lifeforce: Scalar,
    pub decay: Scalar,
    pub roh: Scalar,
    pub oxygen: Scalar,
    pub blood: Scalar,
    pub hpcc: Scalar,
    pub erg: Scalar,
    pub tecl: Scalar, // TECR
    pub biosignature1d: Scalar,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Envelope {
    pub roh_max: Scalar,
    pub decay_max: Scalar,
    pub lifeforce_min: Scalar,
    pub bioload_max: Scalar,
    pub fear_min: Scalar,
    pub fear_max: Scalar,
    pub power_church_k: Scalar,
    pub hpcc_max: Scalar,
    pub erg_max: Scalar,
    pub tecl_max: Scalar,
    pub biosig_min: Scalar,
    pub biosig_max: Scalar,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            roh_max: 0.3,
            decay_max: 1.0,
            lifeforce_min: 0.0,
            bioload_max: 1.0,
            fear_min: 0.0,
            fear_max: 1.0,
            power_church_k: 1.0,
            hpcc_max: 1.0,
            erg_max: 1.0,
            tecl_max: 1.0,
            biosig_min: 0.0,
            biosig_max: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GodLikeStatus {
    pub corridor_safe: bool,
    pub neurorights_safe: bool,
    pub justice_safe: bool,
    pub power_steward_safe: bool,
}

pub fn is_corridor_safe(state: &TreeOfLifeState, env: &Envelope) -> bool {
    state.roh <= env.roh_max
        && state.decay <= env.decay_max
        && state.lifeforce >= env.lifeforce_min
        && state.bioload <= env.bioload_max
        && state.fear >= env.fear_min
        && state.fear <= env.fear_max
}

pub fn is_power_steward_safe(state: &TreeOfLifeState, env: &Envelope) -> bool {
    if state.church <= 0.0 {
        return state.power <= 0.0;
    }
    state.power <= env.power_church_k * state.church
}

pub fn is_justice_safe(state: &TreeOfLifeState, env: &Envelope) -> bool {
    state.hpcc <= env.hpcc_max
        && state.erg <= env.erg_max
        && state.tecl <= env.tecl_max
}

pub fn is_neurorights_safe(state: &TreeOfLifeState, env: &Envelope) -> bool {
    state.biosignature1d >= env.biosig_min && state.biosignature1d <= env.biosig_max
}

pub fn evaluate_god_like(state: &TreeOfLifeState, env: &Envelope) -> GodLikeStatus {
    GodLikeStatus {
        corridor_safe: is_corridor_safe(state, env),
        neurorights_safe: is_neurorights_safe(state, env),
        justice_safe: is_justice_safe(state, env),
        power_steward_safe: is_power_steward_safe(state, env),
    }
}

pub fn is_god_like(state: &TreeOfLifeState, env: &Envelope) -> bool {
    let s = evaluate_god_like(state, env);
    s.corridor_safe && s.neurorights_safe && s.justice_safe && s.power_steward_safe
}
