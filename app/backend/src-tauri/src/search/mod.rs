//! Search module — thin re-export of the `orqa-search` standalone library.
//!
//! All implementation lives in `libs/search`. This module exists only to
//! preserve the `crate::search` path used by `state.rs`, `lib.rs`, and
//! the MCP/IPC servers.

pub use orqa_search::chunker;
pub use orqa_search::embedder;
pub use orqa_search::store;
pub use orqa_search::types;
pub use orqa_search::SearchEngine;
