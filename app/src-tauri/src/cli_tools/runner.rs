//! CLI tool runner — re-export from the orqa-engine plugin crate.
//!
//! The runner implementation lives in `libs/engine/src/plugin/cli_runner.rs`.
//! This module re-exports the public API so existing command code continues
//! to use the `crate::cli_tools::runner` path without changes.

pub use orqa_engine::plugin::cli_runner::{
    CliToolResult, CliToolRunner, CliToolStatus, RegisteredCliTool,
};
