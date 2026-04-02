// Path resolution — re-exported from the engine crate.
//
// The canonical implementation lives in `orqa_engine::paths`. All path
// constants (ORQA_DIR, SETTINGS_FILE) and the `ProjectPaths` struct are
// defined there and re-exported here so app code can import from the
// familiar `domain::paths` path without change.

pub use orqa_engine_types::paths::*;
