use crate::ledger::deed_event::DeedEvent;
use crate::ledger::metrics::BioloadMetrics;

pub fn compute_tech_reward(event: &DeedEvent, metrics: &BioloadMetrics) -> u64 {
    if !event.life_harm_flag && event.ethics_flags.is_empty() && metrics.roh <= 0.3 {
        10
    } else {
        0
    }
}
