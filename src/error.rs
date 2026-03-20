//! Error types for the MCP server.

/// Unified error type for MCP server operations.
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("file system error: {0}")]
    FileSystem(String),

    #[error("YAML parse error: {0}")]
    Yaml(String),

    #[error("JSON error: {0}")]
    Json(String),

    #[error("graph build error: {0}")]
    GraphBuild(String),

    #[error("search error: {0}")]
    Search(String),

    #[error("validation error: {0}")]
    Validation(String),

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

impl From<orqa_search::SearchError> for McpError {
    fn from(e: orqa_search::SearchError) -> Self {
        Self::Search(e.to_string())
    }
}
