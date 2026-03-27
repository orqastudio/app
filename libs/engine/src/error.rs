// Engine-level error type for I/O and serialization failures.
//
// This error type is used by the engine crate's config, paths, and other
// modules that perform file I/O or JSON deserialization. It is intentionally
// minimal — access layers (app, CLI, daemon) convert this to their own error
// types as needed.

/// Errors that can occur in engine-level operations.
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    /// A file system I/O operation failed.
    #[error("file system error: {0}")]
    FileSystem(#[from] std::io::Error),

    /// JSON serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
