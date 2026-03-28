//! Search module — thin re-export of the search API through `orqa_engine::search`.
//!
//! All implementation lives in `engine/search`. This module exists only to
//! preserve the `crate::search` path used by `state.rs`, `lib.rs`, and
//! the MCP/IPC servers. It routes through `orqa_engine::search` rather than
//! importing `orqa_search` directly, keeping all direct library dependencies
//! consolidated in the engine crate.

pub use orqa_engine::search::chunker;
pub use orqa_engine::search::embedder;
pub use orqa_engine::search::store;
pub use orqa_engine::search::types;
pub use orqa_engine::search::SearchEngine;
