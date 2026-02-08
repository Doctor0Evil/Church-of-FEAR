use serde::{Serialize, Deserialize};
use crate::ledger::deed::Deed;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    pub id: String,
    pub deeds: Vec<Deed>,
    pub score: u32,
}

impl UserAccount {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            deeds: Vec::new(),
            score: 0,
        }
    }

    pub fn add_deed(&mut self, description: &str, impact: u32) {
        let deed = Deed::new(description, impact);
        self.score += deed.impact;
        self.deeds.push(deed);
    }
}
