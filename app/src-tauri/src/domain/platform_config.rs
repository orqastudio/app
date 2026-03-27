//! Platform configuration — re-exported from the engine crate.
//!
//! All types, functions, and the static PLATFORM are now defined in
//! `orqa_engine::platform`. This module exists solely to preserve the
//! existing import paths within the app.
pub use orqa_engine::platform::*;
