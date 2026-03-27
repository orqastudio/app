// Search module for the orqa-engine crate.
//
// Re-exports the public search API from orqa-search so that consumers can
// import everything through orqa_engine::search instead of depending on
// orqa_search directly. This keeps the dependency surface narrow and lets
// the engine act as the single import point for all access layers.

pub use orqa_search::SearchEngine;
pub use orqa_search::SearchError;
pub use orqa_search::{ChunkInfo, IndexStatus, SearchResult};
