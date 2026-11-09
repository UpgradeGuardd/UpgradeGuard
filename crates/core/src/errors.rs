//! Shared error types for the UpdrageGuard crates.

use thiserror::Error;

/// Errors surfaced by core operations.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid contract version: {0}")]
    InvalidVersion(String),

    #[error("analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CoreError>;
