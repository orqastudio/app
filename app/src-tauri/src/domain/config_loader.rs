// Project configuration loader — re-exported from the engine crate.
//
// The canonical implementation lives in `orqa_engine::config`. All project
// settings loading MUST go through `load_project_settings` so there is a
// single file I/O and deserialization code path.
//
// Note: `load_project_settings` returns `orqa_validation::settings::ProjectSettings`
// (the minimal, sufficient representation for path resolution and plugin discovery).
// App code that needs the full settings (including GovernanceCounts, DetectedStack,
// status auto-rules, etc.) must use `project_settings_repo::read` which deserializes
// directly into `domain::project_settings::ProjectSettings`.

pub use orqa_engine::config::*;
