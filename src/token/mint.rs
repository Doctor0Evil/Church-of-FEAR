use crate::ledger::account::UserAccount;
use uuid::Uuid;

pub struct Token {
    pub id: String,
    pub amount: u64,
    pub deed_ref: String,
}

pub struct MintEngine;

impl MintEngine {
    pub fn mint(account: &UserAccount, total_impact: u64) -> Vec<Token> {
        account.deeds.iter()
            .map(|deed| Token {
                id: Uuid::new_v4().to_string(),
                amount: (deed.impact as u64 * total_impact) / 100,
                deed_ref: deed.description.clone(),
            })
            .collect()
    }
}
