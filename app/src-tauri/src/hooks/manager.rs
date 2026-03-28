//! Hook manager — re-export from the orqa-engine plugin crate.
//!
//! The hook manager implementation lives in `engine/core/src/plugin/hooks.rs`.
//! This module re-exports the public API so existing command code continues
//! to use the `crate::hooks::manager` path without changes.

pub use orqa_engine::plugin::hooks::{
    generate_dispatchers, read_hook_registry, HookGenerationResult, RegisteredHook,
};
