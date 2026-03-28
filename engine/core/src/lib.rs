// orqa-engine: Facade crate for the OrqaStudio engine.
//
// This crate re-exports the public API from all engine domain crates so that
// consumers can import everything through `orqa_engine::*` without depending on
// individual domain crates directly. No business logic lives here — every module
// is a thin re-export from its canonical domain crate.

/// Agent role taxonomy, agent specification, and task-agent assembly types.
pub mod agent {
    pub use orqa_agent::types;
    pub use orqa_agent::{AgentSpec, BaseRole, TaskAgent};
}

/// Artifact logic: ID generation/validation, type parsing, path derivation,
/// frontmatter extraction and parsing.
pub mod artifact {
    pub use orqa_artifact::*;

    pub mod fs {
        pub use orqa_artifact::fs::*;
    }

    pub mod reader {
        pub use orqa_artifact::reader::*;
    }
}

/// Centralised project configuration loader.
pub mod config {
    pub use orqa_engine_types::config::*;
}

/// Rule parsing, compiled-regex evaluation, and project scanning.
pub mod enforcement {
    pub mod engine {
        pub use orqa_enforcement::engine::*;
    }

    pub mod parser {
        pub use orqa_enforcement::parser::*;
    }

    pub mod store {
        pub use orqa_enforcement::store::*;
    }

    pub mod scanner {
        pub use orqa_enforcement::scanner::*;
    }
}

/// Engine-level error type for I/O, serialization, and validation failures.
pub mod error {
    pub use orqa_engine_types::error::*;
}

/// Artifact graph construction and query functions.
pub mod graph {
    pub use orqa_graph::*;
    pub use orqa_validation::graph::{
        build_artifact_graph, graph_stats, ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats,
    };
}

/// Lesson parse/render logic and file-backed lesson store.
pub mod lesson {
    pub use orqa_lesson::*;

    pub mod store {
        pub use orqa_lesson::store::*;
    }
}

/// Graph-theoretic metric types and computation functions.
pub mod metrics {
    pub use orqa_validation::compute_health;
    pub use orqa_validation::metrics::{
        compute_traceability, find_siblings, trace_descendants, trace_to_pillars, AncestryChain,
        AncestryNode, TraceabilityResult, TracedArtifact,
    };
    pub use orqa_validation::types::GraphHealth;
}

/// Path constants and config-driven path resolution.
pub mod paths {
    pub use orqa_engine_types::paths::*;
}

/// Platform configuration.
pub mod platform {
    pub use orqa_engine_types::platform::*;
}

/// Plugin lifecycle: manifest reading, discovery, collision detection,
/// installation, lockfile management, registry browsing, hook dispatcher generation.
pub mod plugin {
    pub use orqa_plugin::*;
}

/// Project scanning, settings types, and file-backed settings store.
pub mod project {
    pub use orqa_project::*;

    pub mod scanner {
        pub use orqa_project::scanner::*;
    }

    pub mod store {
        pub use orqa_project::store::*;
    }
}

/// System prompt builder, session title generator, knowledge injector.
pub mod prompt {
    pub mod builder {
        pub use orqa_prompt::builder::*;
    }

    pub mod knowledge {
        pub use orqa_prompt::knowledge::*;
    }

    pub mod session_title {
        pub use orqa_prompt::session_title::*;
    }

    pub use orqa_prompt::{
        build_system_prompt, collect_plugin_agent_definitions, list_knowledge_catalog,
        read_governance_file, read_rules, resolve_system_prompt,
    };
}

/// Semantic code search API.
pub mod search {
    pub use orqa_search::chunker;
    pub use orqa_search::embedder;
    pub use orqa_search::store;
    pub use orqa_search::types;
    pub use orqa_search::SearchEngine;
    pub use orqa_search::SearchError;
    pub use orqa_search::{ChunkInfo, IndexStatus, SearchResult};
}

/// Streaming protocol, stream loop, and tool execution.
pub mod streaming {
    pub use orqa_streaming::*;

    pub mod protocol {
        pub use orqa_streaming::protocol::*;
    }

    pub mod stream_loop {
        pub use orqa_streaming::stream_loop::*;
    }

    pub mod tools {
        pub use orqa_streaming::tools::*;
    }
}

/// Abstract storage interfaces that each access layer implements.
pub mod traits {
    pub mod executor {
        pub use orqa_engine_types::traits::executor::*;
    }

    pub mod sidecar {
        pub use orqa_engine_types::traits::sidecar::*;
    }

    pub mod storage {
        pub use orqa_engine_types::traits::storage::*;
    }

    pub mod transport {
        pub use orqa_engine_types::traits::transport::*;
    }
}

/// Shared struct and enum definitions (no business logic, no I/O).
pub mod types {
    pub mod artifact {
        pub use orqa_engine_types::types::artifact::*;
    }

    pub mod enforcement {
        pub use orqa_engine_types::types::enforcement::*;
    }

    pub mod governance {
        pub use orqa_engine_types::types::governance::*;
    }

    pub mod health {
        pub use orqa_engine_types::types::health::*;
    }

    pub mod knowledge {
        pub use orqa_engine_types::types::knowledge::*;
    }

    pub mod lesson {
        pub use orqa_engine_types::types::lesson::*;
    }

    pub mod message {
        pub use orqa_engine_types::types::message::*;
    }

    pub mod project {
        pub use orqa_engine_types::types::project::*;
    }

    pub mod session {
        pub use orqa_engine_types::types::session::*;
    }

    pub mod settings {
        pub use orqa_engine_types::types::settings::*;
    }

    pub mod streaming {
        pub use orqa_engine_types::types::streaming::*;
    }

    pub mod workflow {
        pub use orqa_engine_types::types::workflow::*;
    }
}

/// Utility functions (time, etc.).
pub mod utils {
    pub mod time {
        pub use orqa_engine_types::utils::time::*;
    }

    pub use time::*;
}

/// Integrity check types, context-building functions, and validation.
pub mod validation {
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
    pub use orqa_validation::checks;
    pub use orqa_validation::platform;
    pub use orqa_validation::types;
}

/// Status transition evaluation, process state tracking, session activity tracking.
pub mod workflow {
    pub mod gates {
        pub use orqa_workflow::gates::*;
    }

    pub mod state {
        pub use orqa_workflow::state::*;
    }

    pub mod tracker {
        pub use orqa_workflow::tracker::*;
    }

    pub mod transitions {
        pub use orqa_workflow::transitions::*;
    }
}
