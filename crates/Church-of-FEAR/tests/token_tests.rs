use church_of_fear::ledger::deed_event::DeedEvent;
use church_of_fear::ledger::metrics::BioloadMetrics;
use church_of_fear::token::mint::mint_church;

#[test]
fn mint_for_ecological_negative_bioload() {
    let genesis = DeedEvent::genesis();
    let event = DeedEvent::new(
        genesis.self_hash,
        "actor".into(),
        vec![],
        "ecological_sustainability".into(),
        vec![],
        serde_json::json!({}),
        vec![],
        false,
    );
    let metrics = BioloadMetrics::new(-0.5, 0.1, 0.2);
    let amount = mint_church(&event, &metrics);
    assert!(amount > 0);
}
