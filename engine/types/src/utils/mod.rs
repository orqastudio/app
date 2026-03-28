//! Utility submodules for the orqa-engine-types crate.
//!
//! Contains pure utility functions used across the engine and all access layers.
//! Each submodule covers a single utility domain (e.g. timestamp arithmetic).

/// Timestamp utilities: current time, elapsed time, and ISO-8601 formatting.
pub mod time;

pub use time::*;
