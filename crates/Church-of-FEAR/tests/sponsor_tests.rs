use church_of_fear::sponsor::grant::Grant;
use church_of_fear::sponsor::recipient::Recipient;

#[test]
fn grant_creation() {
    let recipient = Recipient::new("r1".into(), "Eco Lab".into(), "XR-grid sim".into());
    let grant = Grant::new("g1".into(), recipient.id.clone(), 100, "Support sim".into());
    assert_eq!(grant.recipient_id, "r1");
    assert_eq!(grant.amount_pwr, 100);
}
