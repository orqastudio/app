//! Error types for the orqa-graph crate.
//!
//! `GraphError` covers I/O failures encountered during artifact scanning and graph construction.
//! It is distinct from `ValidationError` (engine/validation) to keep engine/graph free of
//! any dependency on the validation library.

use thiserror::Error;

/// Error type for graph construction and traversal operations.
#[derive(Debug, Error)]
pub enum GraphError {
    /// A filesystem I/O error occurred while scanning or reading artifact files.
    #[error("filesystem error: {0}")]
    FileSystem(String),
    /// A YAML parse error was encountered in a frontmatter block.
    #[error("YAML parse error: {0}")]
    YamlParse(String),
}
