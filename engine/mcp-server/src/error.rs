//! Error types for the MCP server.

/// Unified error type for MCP server operations.
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    /// A filesystem I/O error occurred during file operations.
    #[error("file system error: {0}")]
    FileSystem(String),

    /// A YAML parse error occurred while reading a governance artifact.
    #[error("YAML parse error: {0}")]
    Yaml(String),

    /// A JSON serialization or deserialization error occurred.
    #[error("JSON error: {0}")]
    Json(String),

    /// The artifact graph could not be built from the project directory.
    #[error("graph build error: {0}")]
    GraphBuild(String),

    /// A semantic search operation failed.
    #[error("search error: {0}")]
    Search(String),

    /// A validation rule check failed.
    #[error("validation error: {0}")]
    Validation(String),

    /// The daemon process is not reachable on its expected socket or port.
    #[error("daemon unreachable: {0}")]
    DaemonUnreachable(String),

    /// A JSON-RPC protocol error occurred (malformed envelope, unknown method, etc.).
    #[error("protocol error: {0}")]
    Protocol(String),

    /// An underlying OS I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<serde_json::Error> for McpError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e.to_string())
    }
}

impl From<serde_yaml::Error> for McpError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Yaml(e.to_string())
    }
}

impl From<orqa_engine::search::SearchError> for McpError {
    fn from(e: orqa_engine::search::SearchError) -> Self {
        Self::Search(e.to_string())
    }
}
