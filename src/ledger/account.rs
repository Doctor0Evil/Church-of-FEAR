use crate::ledger::Ledger;
use crate::utils::time::time_discount_factor;
use chrono::Utc;

#[derive(Debug)]
pub struct ChurchAccountState {
    pub cumulative_good_deeds: f64, // Time-discounted sum
    pub cumulative_harm_flags: u32,
    pub eco_score: f64, // Convex combo: 0.7 * good_deeds_norm + 0.3 * (1 - harm_norm)
    pub debt_ceiling: f64, // Reduced by harm
    pub church_balance: f64, // Minted tokens
}

impl ChurchAccountState {
    pub fn compute_from_ledger(ledger: &Ledger, actor_id: &str) -> Option<Self> {
        let events = ledger.events_for_actor(actor_id);
        if events.is_empty() {
            return None;
        }

        let now = Utc::now().timestamp() as u64;
        let mut good_deeds = 0.0;
        let mut harm_flags = 0;

        for event in events {
            let age = now - event.timestamp;
            let discount = time_discount_factor(age);
            if event.is_good_deed() {
                good_deeds += 1.0 * discount;
            }
            if event.life_harm_flag {
                harm_flags += 1;
            }
        }

        let good_deeds_norm = good_deeds.min(1.0);
        let harm_norm = (harm_flags as f64 / 10.0).min(1.0); // Cap at 10 harms
        let eco_score = 0.7 * good_deeds_norm + 0.3 * (1.0 - harm_norm);
        let debt_ceiling = 1.0 - harm_norm;
        let church_balance = good_deeds * 0.1; // Symbolic mint per good deed

        Some(Self {
            cumulative_good_deeds: good_deeds,
            cumulative_harm_flags: harm_flags,
            eco_score,
            debt_ceiling,
            church_balance,
        })
    }

    pub fn can_mint_church(&self) -> bool {
        self.cumulative_harm_flags == 0 && self.eco_score > 0.5
    }

    pub fn compute_mint_amount(&self) -> f64 {
        self.eco_score * 10.0 // Symbolic CHURCH tokens
    }

    // Rare-item: Simulates NEUROMORPH-GOD quorum for forgiveness
    pub fn forgiveness_quorum(roles: &[String], required_quorum: usize) -> bool {
        let required = vec!["Host", "OrganicCPUOwner", "Regulator", "SovereignKernel"];
        roles.iter().filter(|r| required.contains(&r.as_str())).count() >= required_quorum
    }
}
