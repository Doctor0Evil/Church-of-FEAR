use sha2::{Digest, Sha256};
/// Utility for SHA-256 hashing.
pub fn sha256(input: &str) -> String {
let mut hasher = Sha256::new();
hasher.update(input.as_bytes());
format!("{:x}", hasher.finalize())
}
