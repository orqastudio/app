//! Shared data structures returned by the search API.

use serde::{Deserialize, Serialize};

/// A single code chunk from the indexed codebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    /// Relative path to the source file.
    pub file_path: String,
    /// First line of the chunk (1-based).
    pub start_line: u32,
    /// Last line of the chunk (1-based, inclusive).
    pub end_line: u32,
    /// Raw source text of this chunk.
    pub content: String,
    /// Detected programming language, if known.
    pub language: Option<String>,
}

/// A search result with relevance score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Relative path to the source file.
    pub file_path: String,
    /// First line of the matching chunk (1-based).
    pub start_line: u32,
    /// Last line of the matching chunk (1-based, inclusive).
    pub end_line: u32,
    /// Raw source text of the matching chunk.
    pub content: String,
    /// Detected programming language, if known.
    pub language: Option<String>,
    /// Relevance score (higher is more relevant; range depends on search mode).
    pub score: f64,
    /// Surrounding context snippet for display.
    pub match_context: String,
}

/// Status of the search index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStatus {
    /// Whether the index has been built for the current project.
    pub is_indexed: bool,
    /// Number of chunks currently stored in the index.
    pub chunk_count: u32,
    /// Whether embedding vectors have been generated for the stored chunks.
    pub has_embeddings: bool,
    /// ISO-8601 timestamp of the last successful index run, if available.
    pub last_indexed: Option<String>,
    /// Filesystem path to the DuckDB index file.
    pub index_path: String,
}
