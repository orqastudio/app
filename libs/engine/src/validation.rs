// Validation module for the orqa-engine crate.
//
// Re-exports the public integrity check types and context-building functions
// from orqa_validation so that consumers can import them through
// orqa_engine::validation without a direct dependency on orqa_validation.

pub use orqa_validation::context::{build_validation_context, build_validation_context_with_types};
pub use orqa_validation::error::ValidationError;
pub use orqa_validation::evaluate_hook;
pub use orqa_validation::types::{
    AppliedFix, EnforcementEvent, EnforcementResult, HookContext, HookResult, HookViolation,
    IntegrityCategory, IntegrityCheck, IntegritySeverity, ParsedArtifact, RelationshipConstraints,
    RelationshipSchema, StatusRule, ValidationContext, ValidationResult,
};
pub use orqa_validation::{
    artifact_from_graph_node, is_hex_artifact_id, is_valid_artifact_id, parse_artifact,
    query_artifacts, validate_file, FileFinding, FileSeverity,
};
