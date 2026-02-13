use crate::deed::DeedEvent;
use crate::validator::{LedgerValidator, ValidationError};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/// Append-only, hash-chained moral ledger (exactly .evolve.jsonl + .donutloop.aln pattern)
#[derive(Debug)]
pub struct MoralLedger {
    path: PathBuf,
    last_hash: String,
}

impl MoralLedger {
    pub fn open_or_create(path: PathBuf) -> Result<Self, std::io::Error> {
        let mut file = OpenOptions::new().read(true).append(true).create(true).open(&path)?;
        let mut last_hash = "0".repeat(64); // genesis

        if path.exists() {
            let reader = BufReader::new(File::open(&path)?);
            for line in reader.lines() {
                let line = line?;
                if line.trim().is_empty() { continue; }
                let event: DeedEvent = serde_json::from_str(&line).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                last_hash = event.self_hash.clone();
            }
        }

        Ok(Self { path, last_hash })
    }

    /// Append a new deed – performs full validation + hash chaining
    pub fn append(&mut self, mut event: DeedEvent) -> Result<Uuid, ValidationError> {
        LedgerValidator::validate_new_event(&event, &self.last_hash)?;
        event = event.finalize_hash_chain(self.last_hash.clone());

        let serialized = serde_json::to_string(&event).map_err(ValidationError::Serialization)?;
        let mut file = OpenOptions::new().append(true).open(&self.path)
            .map_err(ValidationError::Io)?;
        writeln!(file, "{}", serialized).map_err(ValidationError::Io)?;
        self.last_hash = event.self_hash.clone();

        // CHURCH recommendation (advisory logging only)
        let recommendation = event.church_recommendation();
        if recommendation > 0 {
            log::info!("CHURCH recommendation +{} for deed {} by {}", recommendation, event.event_id, event.actor_id);
        }

        Ok(event.event_id)
    }
}
