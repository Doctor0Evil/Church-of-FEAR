mod config;
mod utils;
mod ledger;
mod token;
mod compliance;
mod sponsor;

use crate::ledger::deed_event::{DeedEvent, BioloadReducer, RepairHero};
use crate::ledger::metrics::BioloadMetrics;
use crate::token::mint::mint_church;
use crate::compliance::validator::validate_deed;
use crate::utils::time::now_timestamp;
use log::info;
use serde_json::json;

fn main() {
    env_logger::init();

    info!("Starting Church-of-FEAR ledger node…");

    let genesis = DeedEvent::genesis();
    let context = json!({
        "description": "Tree planting along river bank",
        "location": "Phoenix, AZ",
        "roh": 0.2,
        "decay": 0.7
    });

    let deed = DeedEvent::new(
        genesis.self_hash.clone(),
        "actor:eco-hero".to_string(),
        vec!["target:local-watershed".to_string()],
        "ecological_sustainability".to_string(),
        vec!["tree_planting".to_string()],
        context,
        vec![],
        false,
    );

    let roh = 0.2;
    let decay = 0.7;
    validate_deed(&deed, roh, decay).expect("deed must be compliant");

    let metrics = BioloadMetrics::new(-0.12, roh, decay);
    let church_delta = mint_church(&deed, &metrics);

    info!(
        "Deed {} at {} minted {} CHURCH tokens",
        deed.event_id,
        now_timestamp(),
        church_delta
    );

    let reducer = BioloadReducer::new(metrics.bioload_delta);
    let extra_church = reducer.earn_church();
    info!("BioloadReducer added {} bonus CHURCH", extra_church);

    let hero = RepairHero { impact_score: 0.9 };
    let pwr = hero.grant_pwr();
    info!("RepairHero granted {} PWR", pwr);
}
