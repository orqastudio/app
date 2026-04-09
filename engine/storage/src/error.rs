// Error types for the orqa-storage crate.
//
// StorageError is the single error type returned by all storage operations.
// It wraps sea-orm errors, serde_json errors, and not-found conditions.

use thiserror::Error;

/// Errors that can occur in storage operations.
#[derive(Debug, Error)]
pub enum StorageError {
    /// A database operation failed (query, insert, update, or connection error).
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

impl From<sea_orm::DbErr> for StorageError {
    /// Convert a SeaORM database error into a StorageError.
    ///
    /// RecordNotFound maps to NotFound; all other errors map to Database.
    fn from(e: sea_orm::DbErr) -> Self {
        match e {
            sea_orm::DbErr::RecordNotFound(msg) => Self::NotFound(msg),
            other => Self::Database(other.to_string()),
        }
    }
}

impl From<serde_json::Error> for StorageError {
    /// Convert a serde_json error into a StorageError.
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}
