use crate::ledger::deed_event::DeedEvent;
use crate::ledger::metrics::BioloadMetrics;

pub fn mint_church(event: &DeedEvent, metrics: &BioloadMetrics) -> u64 {
    event.compute_church_reward(metrics.bioload_delta)
}
