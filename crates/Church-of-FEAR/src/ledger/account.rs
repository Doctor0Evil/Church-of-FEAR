use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub owner: String,
    pub balance_church: u64,
    pub balance_pwr: u64,
}

impl Account {
    pub fn new(id: String, owner: String) -> Self {
        Self {
            id,
            owner,
            balance_church: 0,
            balance_pwr: 0,
        }
    }

    pub fn credit_church(&mut self, amount: u64) {
        self.balance_church = self.balance_church.saturating_add(amount);
    }

    pub fn debit_church(&mut self, amount: u64) {
        self.balance_church = self.balance_church.saturating_sub(amount);
    }

    pub fn credit_pwr(&mut self, amount: u64) {
        self.balance_pwr = self.balance_pwr.saturating_add(amount);
    }
}
