// Search module for the orqa-engine crate.
//
// Re-exports the public search API from orqa-search so that consumers can
// import everything through orqa_engine::search instead of depending on
// orqa_search directly. This keeps the dependency surface narrow and lets
// the engine act as the single import point for all access layers.
//
// Submodules (chunker, embedder, store, types) are re-exported so that the
// Tauri app's crate::search thin-wrapper can resolve all paths through
// orqa_engine::search without a direct orqa-search dependency.

pub use orqa_search::chunker;
pub use orqa_search::embedder;
pub use orqa_search::store;
pub use orqa_search::types;
pub use orqa_search::SearchEngine;
pub use orqa_search::SearchError;
pub use orqa_search::{ChunkInfo, IndexStatus, SearchResult};
