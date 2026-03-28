//! Trait module declarations for the orqa-engine-types crate.
//!
//! This module exposes abstract interfaces that each access layer implements.
//! The engine defines what operations storage, sidecars, tool execution, and
//! event emission must support; the app, daemon, and CLI each provide their
//! own concrete implementations.

/// Tool executor abstraction: submit tool calls and receive results.
pub mod executor;
/// Sidecar process abstraction: spawning and communicating with sidecars.
pub mod sidecar;
/// Persistent storage abstraction: session, event, and artifact storage.
pub mod storage;
/// Transport abstraction: sending/receiving messages between components.
pub mod transport;
