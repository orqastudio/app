// Validation module for the orqa-engine crate.
//
// Re-exports the public integrity check types, context-building functions,
// plugin platform scanning, and settings from orqa_validation so that consumers
// can import everything through orqa_engine::validation without a direct
// dependency on orqa_validation.

pub use orqa_validation::context::{build_validation_context, build_validation_context_with_types};
pub use orqa_validation::error::ValidationError;
pub use orqa_validation::evaluate_hook;
pub use orqa_validation::platform::{
    ArtifactTypeDef, EnforcementMechanism, PluginContributions, SchemaExtension,
    scan_plugin_manifests,
};
pub use orqa_validation::settings::DeliveryConfig;
pub use orqa_validation::types::{
    AppliedFix, EnforcementEvent, EnforcementResult, HookContext, HookResult, HookViolation,
    IntegrityCategory, IntegrityCheck, IntegritySeverity, ParsedArtifact, RelationshipConstraints,
    RelationshipSchema, StatusRule, ValidationContext, ValidationResult,
};
pub use orqa_validation::{
    artifact_from_graph_node, is_hex_artifact_id, is_valid_artifact_id, parse_artifact,
    query_artifacts, validate_file, FileFinding, FileSeverity,
};
// Re-export check modules for callers that need the file_level sub-module directly.
pub use orqa_validation::checks;

// Re-export submodules so consumers can use orqa_engine::validation::platform::* etc.
pub use orqa_validation::platform;
pub use orqa_validation::types;
