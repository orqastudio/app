// Health snapshot domain types — re-exported from the orqa-engine crate.
//
// HealthSnapshot and NewHealthSnapshot represent point-in-time snapshots of
// artifact graph health metrics. Snapshots are persisted by the daemon and
// surfaced in the governance dashboard.

pub use orqa_engine::types::health::*;
