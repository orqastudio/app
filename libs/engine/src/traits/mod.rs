// Trait module declarations for the orqa-engine crate.
//
// This module exposes abstract interfaces that each access layer implements.
// The engine defines what operations storage and sidecars must support;
// the app, daemon, and CLI each provide their own concrete implementations.

pub mod sidecar;
pub mod storage;
