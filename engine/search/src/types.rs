use serde::{Deserialize, Serialize};

/// A single code chunk from the indexed codebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub file_path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub content: String,
    pub language: Option<String>,
}

/// A search result with relevance score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub content: String,
    pub language: Option<String>,
    pub score: f64,
    pub match_context: String,
}

/// Status of the search index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStatus {
    pub is_indexed: bool,
    pub chunk_count: u32,
    pub has_embeddings: bool,
    pub last_indexed: Option<String>,
    pub index_path: String,
}
