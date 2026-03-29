//! Error types for the OrqaStudio LSP server crate.

use thiserror::Error;

/// Errors produced by the LSP server and artifact graph builder.
#[derive(Debug, Error)]
pub enum LspError {
    /// A filesystem operation failed.
    #[error("filesystem error: {0}")]
    FileSystem(#[from] std::io::Error),

    /// JSON deserialisation failed (e.g. malformed `project.json`).
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// YAML deserialisation failed (e.g. malformed frontmatter).
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// A general validation error.
    #[error("validation error: {0}")]
    Validation(String),
}
