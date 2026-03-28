// Trait module declarations for the orqa-engine-types crate.
//
// This module exposes abstract interfaces that each access layer implements.
// The engine defines what operations storage, sidecars, tool execution, and
// event emission must support; the app, daemon, and CLI each provide their
// own concrete implementations.

pub mod executor;
pub mod sidecar;
pub mod storage;
pub mod transport;
