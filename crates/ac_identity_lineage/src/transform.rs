use crate::{error::LineageError, lineage::LineageRecord, pattern::CommandPattern};
use serde_json::Value;

pub fn apply_pattern(
    pattern: &CommandPattern,
    target: &str,
) -> Result<(LineageRecord, Value), LineageError> {
    let re = pattern.compile()?;
    let matched = re.is_match(target);
    if !matched {
        return Err(LineageError::NoMatch);
    }
    let record = LineageRecord::new(&pattern.name, target, true);
    let payload = serde_json::json!({
        "status": "success",
        "matched": true,
        "pattern": pattern.name,
    });
    Ok((record, payload))
}
