// orqa-engine-types: Foundation crate for the OrqaStudio engine.
//
// Contains shared type definitions, error types, abstract traits, and utility
// functions used by all engine domain crates (agent, enforcement, workflow,
// prompt, plugin) and re-exported through the engine facade.
//
// This crate has no business logic — only data shapes, contracts, and pure helpers.
// The config, paths, and platform modules are included here (not in the engine facade)
// because they form the foundational layer that all domain crates depend on.

pub mod config;
pub mod error;
pub mod paths;
pub mod platform;
pub mod traits;
pub mod types;
pub mod utils;
