use crate::chunker::ChunkError;
use crate::embedder::EmbedError;
use crate::store::StoreError;

/// Top-level error type for the orqa-search library.
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("store error: {0}")]
    Store(#[from] StoreError),

    #[error("embed error: {0}")]
    Embed(#[from] EmbedError),

    #[error("chunk error: {0}")]
    Chunk(#[from] ChunkError),

    #[error("search error: {0}")]
    Search(String),
}
