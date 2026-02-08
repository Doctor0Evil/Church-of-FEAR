use regex::Regex;
use serde_json::Value;
use ac_aln_rt::errors::AlnError;

pub struct RegexBranch;

impl RegexBranch {
    pub fn integrate_regex(pattern: &str, target: &str) -> Result<Value, AlnError> {
        let re = Regex::new(pattern).map_err(|e| AlnError::InvalidInput(e.to_string()))?;
        let matched = re.is_match(target);
        Ok(serde_json::json!({
            "status": "success",
            "matched": matched
        }))
    }
}
