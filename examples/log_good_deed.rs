use church_ledger::append_deed_event;
fn main() {
    let _ = append_deed_event(
        "data/church-ledger.jsonl",
        "xboxtj-san-tan-valley".into(),
        vec!["homeless-shelter-az".into()],
        "homelessness_relief".into(),
        vec!["civic-duty".into(), "tree-of-life".into()],
        serde_json::json!({"hours": 8, "meals_served": 45, "location": "San Tan Valley"}),
        vec![],
        false,
    );
    // This single deed mints ~28 CHURCH tokens + eco_grant recommendation for real NPO funding routing
}
