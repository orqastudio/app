use crate::chunker::ChunkError;
use crate::embedder::EmbedError;
use crate::store::StoreError;

/// Top-level error type for the orqa-search library.
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    /// DuckDB persistence layer error.
    #[error("store error: {0}")]
    Store(#[from] StoreError),

    /// ONNX embedding error.
    #[error("embed error: {0}")]
    Embed(#[from] EmbedError),

    /// Code chunking error.
    #[error("chunk error: {0}")]
    Chunk(#[from] ChunkError),

    /// Generic search error.
    #[error("search error: {0}")]
    Search(String),
}
