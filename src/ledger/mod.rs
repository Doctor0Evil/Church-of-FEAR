mod deed_event;
mod account;

pub use deed_event::DeedEvent;
pub use account::ChurchAccountState;

use std::collections::HashMap;

pub struct Ledger {
    events: Vec<DeedEvent>,
    last_hash: String,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            events: Vec::new(),
            last_hash: String::new(),
        }
    }

    pub fn append(&mut self, event: DeedEvent) {
        if event.prev_hash != self.last_hash {
            panic!("Invalid prev_hash");
        }
        self.events.push(event.clone());
        self.last_hash = event.self_hash;
    }

    pub fn last_hash(&self) -> &str {
        &self.last_hash
    }

    pub fn events_for_actor(&self, actor_id: &str) -> Vec<&DeedEvent> {
        self.events.iter().filter(|e| e.actor_id == actor_id).collect()
    }
}
