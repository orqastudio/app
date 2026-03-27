// Engine-level error type for I/O, serialization, and validation failures.
//
// This error type is used by the engine crate's config, paths, artifact, enforcement,
// and other modules. It is intentionally minimal — access layers (app, CLI, daemon)
// convert this to their own error types as needed.

/// Errors that can occur in engine-level operations.
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    /// A file system I/O operation failed.
    #[error("file system error: {0}")]
    FileSystem(#[from] std::io::Error),

    /// JSON serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// A value failed domain validation (e.g. unknown artifact type).
    #[error("validation error: {0}")]
    Validation(String),

    /// A YAML parse or serialization error (e.g. invalid rule frontmatter).
    #[error("yaml error: {0}")]
    Yaml(String),

    /// A governance scan operation failed (e.g. invalid glob pattern).
    #[error("scan error: {0}")]
    Scan(String),
}
