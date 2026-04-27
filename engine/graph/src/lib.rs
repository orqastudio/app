//! orqa-graph: Artifact graph construction and traversal for OrqaStudio.
//!
//! This crate is the canonical home of all graph logic. The engine/validation crate
//! depends on this crate for graph operations; the dependency flows one way only
//! (engine/graph → engine/types, engine/validation → engine/graph).
//!
//! Public modules:
//! - `build`: graph construction, scanning, node assembly, type inference
//! - `metrics`: graph health metrics and traceability queries
//! - `surreal`: SurrealDB connection wrapper and schema initialisation
//! - `surreal_queries`: SurrealQL-based graph query functions
//! - `sync`: incremental and bulk sync of `.orqa/` markdown files into SurrealDB
//! - `error`: `GraphError` type for I/O and YAML parse failures

pub mod build;
pub mod error;
pub mod merge;
pub mod metrics;
pub mod surreal;
pub mod surreal_queries;
pub mod sync;
pub mod writers;

#[cfg(test)]
mod metrics_tests;

#[cfg(test)]
mod build_tests;

// Re-export graph data types and metric types from engine/types so consumers can access
// them via this crate without depending on engine/types directly.
pub use orqa_engine_types::{
    AncestryChain, AncestryNode, ArtifactGraph, ArtifactNode, ArtifactRef, GraphHealth, GraphStats,
    OutlierAgeDistribution, TraceabilityResult, TracedArtifact,
};

// Re-export the most commonly used public API at crate root.
pub use build::{
    build_artifact_graph, build_graph_from_entries, build_valid_relationship_types,
    extract_frontmatter, graph_stats, humanize_stem, infer_artifact_type, load_project_config,
    TypeRegistry,
};
pub use error::GraphError;
pub use merge::{three_way_merge, value_to_map, FieldMerge, MergeResult};
pub use metrics::{
    compute_health, compute_traceability, find_siblings, trace_descendants, trace_to_pillars,
    PipelineCategories,
};
pub use surreal::GraphDb;
pub use surreal_queries::{
    avg_degree, count_by_status, count_by_type, find_orphans as surreal_find_orphans,
    find_siblings as surreal_find_siblings, list_artifacts, search_artifacts, total_artifacts,
    total_edges, trace_descendants as surreal_trace_descendants,
    trace_to_pillars as surreal_trace_to_pillars, ArtifactRecord, GroupCount, OrphanArtifact,
    TraceStep,
};
pub use sync::{
    bulk_sync, delete_artifact, delete_artifact_by_path, delete_plugin_artifacts,
    delete_plugin_enforcement_rules, sync_file, sync_plugin_file,
    upsert_enforcement_rules_from_plugin, BulkSyncSummary, EnforcementRuleSource, SyncResult,
};
pub use writers::{
    bump_version, check_plugin_drift, count_edges_from, count_edges_involving, create_artifact,
    delete_artifact_with_edges, delete_plugin_installation, import_merge_write, import_upsert,
    list_all_plugin_installations, list_plugin_installations, read_artifact,
    read_plugin_installation, update_artifact_fields, upsert_plugin_installation,
    upsert_plugin_installation_with_timestamp, upsert_relates_to_edge, BumpError, DriftedFile,
    DriftedPlugin, PluginDriftReport, PluginFileEntry, PluginInstallStatus, RelationshipEdge,
    StoredArtifact,
};
