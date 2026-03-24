//! OrqaStudio validation library.
//!
//! A schema-driven integrity engine for the artifact graph. Provides:
//!
//! - Graph construction from a `.orqa/` directory tree
//! - Context building from `core.json` platform schema + project settings
//! - Integrity checks: structural, status, delivery, cycles, cardinality, body refs
//! - Graph-theoretic health metrics
//! - Auto-fix engine for fixable issues
//!
//! # Quick start
//!
//! ```no_run
//! use orqa_validation::{validate, compute_health, auto_fix};
//! use orqa_validation::graph::build_artifact_graph;
//! use orqa_validation::context::build_validation_context;
//! use orqa_validation::settings::DeliveryConfig;
//! use std::path::Path;
//!
//! let project_path = Path::new("/path/to/project");
//! let graph = build_artifact_graph(project_path).unwrap();
//! let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[], &[]);
//! let checks = validate(&graph, &ctx);
//! let health = compute_health(&graph);
//! ```

pub mod auto_fix;
pub mod checks;
pub mod content;
pub mod context;
pub mod daemon;
pub mod error;
pub mod generated;
pub mod graph;
pub mod hooks;
pub mod metrics;
pub mod parse;
pub mod platform;
pub mod settings;
pub mod types;

pub use auto_fix::{apply_fixes, update_artifact_field};
pub use content::{
    extract_behavioral_messages, find_agent, find_agent_in_graph, find_knowledge, AgentContent,
    BehavioralMessages, KnowledgeContent,
};
pub use context::{build_validation_context, build_validation_context_with_types};
pub use error::ValidationError;
pub use graph::{
    build_artifact_graph, graph_stats, ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats,
};
pub use hooks::evaluate_hook;
pub use metrics::{
    compute_health, compute_traceability, find_siblings, trace_descendants, trace_to_pillars,
    AncestryChain, AncestryNode, TraceabilityResult, TracedArtifact,
};
pub use checks::file_level::{
    is_hex_artifact_id, is_valid_artifact_id, validate_file, FileFinding, FileSeverity,
};
pub use parse::{artifact_from_graph_node, parse_artifact, query_artifacts};
pub use types::{
    AppliedFix, EnforcementEvent, EnforcementResult, GraphHealth, HookContext, HookResult,
    HookViolation, IntegrityCategory, IntegrityCheck, IntegritySeverity, ParsedArtifact,
    RelationshipConstraints, RelationshipSchema, StatusRule, ValidationContext, ValidationResult,
};

/// Run all integrity checks on the artifact graph.
///
/// Returns a list of [`IntegrityCheck`] findings (errors and warnings). Pass
/// the result to [`apply_fixes`] to auto-remediate fixable issues, or to
/// [`compute_health`] for graph-theoretic metrics.
pub fn validate(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck> {
    checks::run_all(graph, ctx)
}

/// Apply auto-fixes for integrity issues and return a summary of what was changed.
///
/// Only checks with `auto_fixable: true` are processed. Files on disk are
/// modified in place.
pub fn auto_fix(
    graph: &ArtifactGraph,
    checks: &[IntegrityCheck],
    project_path: &std::path::Path,
) -> Result<Vec<AppliedFix>, ValidationError> {
    apply_fixes(graph, checks, project_path)
}
