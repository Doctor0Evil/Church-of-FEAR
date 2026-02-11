#[cfg(test)]
mod tests {
    use super::super::ledger::{DeedEvent, Ledger, ChurchAccountState};
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_ledger_append_and_hash() {
        let mut ledger = Ledger::new();
        let mut deed = DeedEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: 0,
            prev_hash: String::new(),
            self_hash: String::new(),
            actor_id: "test".to_string(),
            target_ids: vec![],
            deed_type: "test".to_string(),
            tags: vec![],
            context_json: json!({}),
            ethics_flags: vec![],
            life_harm_flag: false,
        };
        deed.self_hash = deed.compute_self_hash();
        ledger.append(deed.clone());
        assert_eq!(ledger.last_hash(), deed.self_hash);
    }

    #[test]
    fn test_account_compute() {
        let mut ledger = Ledger::new();
        let mut deed_good = DeedEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: 0,
            prev_hash: String::new(),
            self_hash: String::new(),
            actor_id: "test".to_string(),
            target_ids: vec![],
            deed_type: "ecological_sustainability".to_string(),
            tags: vec!["tree_planting".to_string()],
            context_json: json!({}),
            ethics_flags: vec![],
            life_harm_flag: false,
        };
        deed_good.self_hash = deed_good.compute_self_hash();
        ledger.append(deed_good);

        let state = ChurchAccountState::compute_from_ledger(&ledger, "test").unwrap();
        assert!(state.can_mint_church());
        assert_eq!(state.compute_mint_amount(), 7.0); // Assuming eco_score=0.7
    }
}
