// Error types for the orqa-storage crate.
//
// StorageError is the single error type returned by all storage operations.
// It wraps rusqlite errors, serde_json errors, and not-found conditions.

use thiserror::Error;

/// Errors that can occur in storage operations.
#[derive(Debug, Error)]
pub enum StorageError {
    /// A SQLite operation failed.
    #[error("database error: {0}")]
    Database(String),

    /// A JSON serialization or deserialization error.
    #[error("serialization error: {0}")]
    Serialization(String),

    /// The requested record does not exist.
    #[error("not found: {0}")]
    NotFound(String),

    /// The database path could not be resolved or created.
    #[error("path error: {0}")]
    Path(String),
}

impl From<rusqlite::Error> for StorageError {
    fn from(e: rusqlite::Error) -> Self {
        match e {
            rusqlite::Error::QueryReturnedNoRows => {
                Self::NotFound("query returned no rows".to_owned())
            }
            other => Self::Database(other.to_string()),
        }
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}
