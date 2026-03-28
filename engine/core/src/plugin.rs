// Plugin module for the orqa-engine crate.
//
// Re-exports the plugin system from the orqa-plugin crate so that consumers
// can continue to use `orqa_engine::plugin::*` without changing their imports.

pub use orqa_plugin::*;
