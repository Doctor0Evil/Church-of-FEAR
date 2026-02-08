use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlnError {
    #[error("Session not found")]
    SessionNotFound,
    #[error("Redis error: {0}")]
    Redis(String),
    #[error("Postgres error: {0}")]
    Postgres(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Command failed: {0}")]
    CommandFailed(String),
}
