fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut ledger = MoralLedger::open_or_create("moral_ledger.jsonl".into())?;
    
    // Example good deed → earns CHURCH recommendation
    church::log_ecological_cleanup(
        &mut ledger,
        "user:xboxteejay".to_string(),
        "https://ipfs.io/ipfs/Qm.../reforestation_receipt.pdf".to_string(),
    )?;

    // Example open-source contribution
    church::log_open_source_contribution(
        &mut ledger,
        "user:xboxteejay".to_string(),
        "church_of_fear_ledger".to_string(),
    )?;

    Ok(())
}
