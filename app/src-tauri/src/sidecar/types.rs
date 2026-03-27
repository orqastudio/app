// Sidecar protocol type re-exports for the orqa-studio app.
//
// SidecarRequest, SidecarResponse, and MessageSummary are defined in the
// orqa-engine crate so they can be shared with the daemon, CLI, and other
// access layers. This module re-exports them for use within the app without
// changing any existing import paths.

pub use orqa_engine::streaming::protocol::{MessageSummary, SidecarRequest, SidecarResponse};
