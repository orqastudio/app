// Artifact domain types and logic — re-exported from orqa-engine.
//
// All type definitions (structs/enums) and business logic (ID generation, type parsing,
// path derivation, frontmatter extraction) live in the engine crate at
// `orqa_engine::artifact` and `orqa_engine::types::artifact`. This module re-exports
// them so the rest of the app can import from `crate::domain::artifact`.

pub use orqa_engine::artifact::*;
pub use orqa_engine::types::artifact::*;
