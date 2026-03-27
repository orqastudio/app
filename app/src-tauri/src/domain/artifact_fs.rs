// Filesystem helpers for artifact I/O — re-exported from orqa-engine.
//
// All logic (write_artifact_file, artifact_from_file, governance_dir, scan_directory,
// now_iso, humanize_name) lives in the engine crate at `orqa_engine::artifact::fs`.
// This module re-exports so the rest of the app can import from `crate::domain::artifact_fs`.

pub use orqa_engine::artifact::fs::*;
