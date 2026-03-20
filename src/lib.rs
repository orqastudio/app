//! `orqa-search` — standalone ONNX code search engine for OrqaStudio.
//!
//! Provides:
//! - [`SearchEngine`] — the main entry point for indexing and querying a codebase
//! - [`embedder`] — ONNX runtime wrapper for BGE-small-en-v1.5 embeddings
//! - [`chunker`] — language-aware code chunking
//! - [`store`] — DuckDB persistence for chunks and embeddings
//! - [`types`] — shared data structures
//! - [`error`] — unified error type
//!
//! # Example
//!
//! ```no_run
//! use orqa_search::SearchEngine;
//! use std::path::Path;
//!
//! let mut engine = SearchEngine::new(Path::new("/tmp/index.duckdb")).unwrap();
//! engine.index(Path::new("/my/project"), &[]).unwrap();
//! let results = engine.search_regex("fn main", None, 10).unwrap();
//! ```

pub mod chunker;
pub mod embedder;
pub mod engine;
pub mod error;
pub mod store;
pub mod types;

// Re-export the primary public API at the crate root.
pub use engine::SearchEngine;
pub use error::SearchError;
pub use types::{ChunkInfo, IndexStatus, SearchResult};
