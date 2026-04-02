//! Error types for the validation library.

/// Unified error type for validation operations.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// File system operation failed.
    #[error("file system error: {0}")]
    FileSystem(String),

    /// YAML deserialization error.
    #[error("YAML parse error: {0}")]
    Yaml(String),

    /// JSON serialization or deserialization error.
    #[error("JSON error: {0}")]
    Json(String),

    /// Artifact graph construction failed.
    #[error("graph build error: {0}")]
    GraphBuild(String),

    /// A validation rule raised an error.
    #[error("validation error: {0}")]
    Validation(String),

    /// Underlying I/O error from the OS.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<serde_json::Error> for ValidationError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e.to_string())
    }
}

impl From<serde_yaml::Error> for ValidationError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Yaml(e.to_string())
    }
}

/// Convert a graph construction error into a validation error.
///
/// Graph errors bubble up through validation callers that return ValidationError.
/// This impl lets callers use `?` directly instead of `.map_err(|e| ...)`.
impl From<orqa_graph::GraphError> for ValidationError {
    fn from(e: orqa_graph::GraphError) -> Self {
        Self::GraphBuild(e.to_string())
    }
}
