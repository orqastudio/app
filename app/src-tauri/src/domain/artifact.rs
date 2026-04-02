// Artifact domain types — re-exported from orqa-engine-types.
//
// Type definitions (NavTree, NavGroup, NavType, DocNode, ArtifactEntry, etc.)
// live in orqa-engine-types and are re-exported here for use throughout the app.
// All artifact scanning and tree-building is delegated to the daemon.

pub use orqa_engine_types::types::artifact::*;
