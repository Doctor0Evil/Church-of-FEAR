use serde_json::Value;
use ac_aln_rt::errors::AlnError;

pub struct LanguageBranch;

impl LanguageBranch {
    pub fn define_syntax(rule: &str, semantic_action: &str) -> Result<Value, AlnError> {
        if rule.trim().is_empty() {
            return Err(AlnError::InvalidInput("empty rule".into()));
        }
        Ok(serde_json::json!({
            "status": "syntax_defined",
            "rule": rule,
            "semantic_action": semantic_action
        }))
    }
}
