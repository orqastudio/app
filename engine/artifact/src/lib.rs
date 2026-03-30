//! Artifact business logic for the OrqaStudio engine.
//!
//! This crate provides the business logic for working with governance artifacts:
//! ID generation and validation, frontmatter extraction/parsing, filesystem I/O,
//! and navigation tree scanning. All functions operate on types defined in
//! `orqa_engine_types::types::artifact`.
//!
//! Artifact types are opaque strings — the engine does not enumerate specific types
//! like "agent" or "rule". Plugins declare what artifact types exist and where they
//! live. The engine reads from plugin-configured paths.
//!
//! Type definitions (structs) live in `orqa-engine-types`. This crate contains
//! only the behaviour — the functions that act on those types.
//!
//! Submodules:
//!   `fs`     -- filesystem helpers: write, read, scan directories
//!   `reader` -- navigation tree scanner driven by schema.composed.json

/// Filesystem helpers: write, read, and scan artifact directories.
pub mod fs;
/// Navigation tree scanner driven by schema.composed.json artifact config.
pub mod reader;

use std::path::Path;

use rand::Rng;

use orqa_engine_types::config::{ArtifactEntry, ArtifactTypeConfig};
use orqa_engine_types::error::EngineError;

/// Build artifact scanner entries from `schema.composed.json`.
///
/// Reads `.orqa/schema.composed.json` and maps each `artifactTypes` entry to an
/// `ArtifactEntry::Type` using the schema's `label`, `icon`, and `default_path` fields.
/// This is the single source of truth for which artifact types exist and where they live —
/// replacing the old `artifacts` array in `project.json` (P1: Plugin-Composed Everything).
///
/// Returns an empty vec when the schema file is absent, unreadable, or has no artifact types.
/// Each failure path emits a `tracing::warn` so callers can diagnose empty nav trees.
pub fn artifact_entries_from_schema(project_path: &Path) -> Vec<ArtifactEntry> {
    let schema_path = project_path.join(".orqa").join("schema.composed.json");
    let Ok(content) = std::fs::read_to_string(&schema_path) else {
        tracing::warn!(
            path = %schema_path.display(),
            "schema.composed.json not found — returning empty nav tree"
        );
        return Vec::new();
    };
    let Ok(schema) = serde_json::from_str::<serde_json::Value>(&content) else {
        tracing::warn!(
            path = %schema_path.display(),
            "schema.composed.json could not be parsed as JSON — returning empty nav tree"
        );
        return Vec::new();
    };
    let Some(artifact_types) = schema
        .get("artifactTypes")
        .and_then(|v| v.as_object())
    else {
        tracing::warn!(
            path = %schema_path.display(),
            "schema.composed.json has no 'artifactTypes' object — returning empty nav tree"
        );
        return Vec::new();
    };

    artifact_types
        .iter()
        .map(|(key, def)| {
            ArtifactEntry::Type(ArtifactTypeConfig {
                key: key.clone(),
                label: def
                    .get("label")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned),
                icon: def
                    .get("icon")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned),
                path: def
                    .get("default_path")
                    .and_then(|v| v.as_str())
                    // Strip trailing slash so path joins work consistently.
                    .map_or_else(|| format!(".orqa/{key}"), |p| p.trim_end_matches('/').to_owned()),
            })
        })
        .collect()
}

/// Generate a new artifact ID in `TYPE-XXXXXXXX` format (8 lowercase hex chars).
///
/// The prefix should be the artifact type in uppercase (e.g. "KNOW", "TASK", "EPIC").
/// The hex portion is randomly generated using the system RNG.
pub fn generate_artifact_id(prefix: &str) -> String {
    let hex: u32 = rand::thread_rng().gen();
    format!("{}-{hex:08x}", prefix.to_uppercase())
}

/// Validate that an artifact ID matches the expected format.
///
/// Accepts:
/// - Simple IDs: `TYPE-NNN` (legacy digits) or `TYPE-XXXXXXXX` (8 hex chars)
/// - Compound prefix IDs: `TYPE-SUB-NNN` or `TYPE-SUB-XXXXXXXX` where all prefix
///   segments are uppercase alpha and the final segment is digits or 8 hex chars.
///
/// Returns `true` if the ID is valid.
pub fn is_valid_artifact_id(id: &str) -> bool {
    // Split on the last '-' to extract the final suffix and the prefix part.
    let Some(last_dash) = id.rfind('-') else {
        return false;
    };
    let final_suffix = &id[last_dash + 1..];
    let prefix_part = &id[..last_dash];

    // Prefix part must be non-empty and contain only uppercase letters and hyphens.
    if prefix_part.is_empty()
        || !prefix_part
            .chars()
            .all(|c| c.is_ascii_uppercase() || c == '-')
    {
        return false;
    }
    // No prefix segment may be empty (catches leading/trailing/consecutive hyphens).
    if prefix_part.split('-').any(str::is_empty) {
        return false;
    }
    // Final suffix is either all digits (legacy) or exactly 8 hex chars (new format).
    final_suffix.chars().all(|c| c.is_ascii_digit())
        || (final_suffix.len() == 8 && final_suffix.chars().all(|c| c.is_ascii_hexdigit()))
}

/// Check if an artifact ID uses the new hex format (TYPE-XXXXXXXX).
pub fn is_hex_artifact_id(id: &str) -> bool {
    let Some((_prefix, suffix)) = id.split_once('-') else {
        return false;
    };
    suffix.len() == 8 && suffix.chars().all(|c| c.is_ascii_hexdigit())
}

/// Extract the YAML text between `---` delimiters from a markdown file.
///
/// Returns `(yaml_text, body)`. If no frontmatter is present, returns `(None, full_content)`.
pub fn extract_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_owned());
    }

    let after_open = &trimmed[3..];
    let Some(close_pos) = after_open.find("\n---") else {
        return (None, content.to_owned());
    };

    let fm_text = after_open[..close_pos].to_owned();
    let body = after_open[close_pos + 4..]
        .trim_start_matches('\n')
        .to_owned();
    (Some(fm_text), body)
}

/// Parse YAML frontmatter into any deserializable type.
///
/// Returns `(parsed_frontmatter, body)`. If no frontmatter is present or parsing fails,
/// returns `(Default::default(), full_content)`.
pub fn parse_frontmatter<T: serde::de::DeserializeOwned + Default>(content: &str) -> (T, String) {
    let (fm_text, body) = extract_frontmatter(content);
    let frontmatter = fm_text
        .and_then(|text| serde_yaml::from_str::<T>(&text).ok())
        .unwrap_or_default();
    (frontmatter, body)
}

/// Validate that a string is a usable artifact type key (non-empty, no path separators).
///
/// The engine does not enumerate valid types — plugins declare them. This validates
/// only the structural requirements: non-empty and containing only safe characters.
pub fn validate_artifact_type_key(key: &str) -> Result<(), EngineError> {
    if key.is_empty() {
        return Err(EngineError::Validation(
            "artifact type key must not be empty".to_owned(),
        ));
    }
    if key.contains('/') || key.contains('\\') {
        return Err(EngineError::Validation(format!(
            "artifact type key must not contain path separators: {key}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_engine_types::types::artifact::{
        Artifact, ArtifactRelationship, ArtifactSummary, ComplianceStatus,
    };

    #[test]
    fn validate_artifact_type_key_valid() {
        assert!(validate_artifact_type_key("task").is_ok());
        assert!(validate_artifact_type_key("epic").is_ok());
        assert!(validate_artifact_type_key("my-type").is_ok());
    }

    #[test]
    fn validate_artifact_type_key_empty() {
        assert!(matches!(
            validate_artifact_type_key(""),
            Err(EngineError::Validation(_))
        ));
    }

    #[test]
    fn validate_artifact_type_key_path_separator() {
        assert!(matches!(
            validate_artifact_type_key("foo/bar"),
            Err(EngineError::Validation(_))
        ));
    }

    #[test]
    fn compliance_status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(ComplianceStatus::Compliant)
                .expect("serialization should succeed")
                .as_str(),
            Some("compliant")
        );
        assert_eq!(
            serde_json::to_value(ComplianceStatus::NonCompliant)
                .expect("serialization should succeed")
                .as_str(),
            Some("non_compliant")
        );
        assert_eq!(
            serde_json::to_value(ComplianceStatus::Unknown)
                .expect("serialization should succeed")
                .as_str(),
            Some("unknown")
        );
        assert_eq!(
            serde_json::to_value(ComplianceStatus::Error)
                .expect("serialization should succeed")
                .as_str(),
            Some("error")
        );
    }

    #[test]
    fn artifact_relationship_uses_type_field() {
        let rel = ArtifactRelationship {
            relationship_type: "references".to_string(),
            target: ".orqa/learning/rules/coding-standards.md".to_string(),
        };

        let json = serde_json::to_value(&rel).expect("serialization should succeed");
        assert_eq!(json["type"], "references");
        assert_eq!(json["target"], ".orqa/learning/rules/coding-standards.md");
    }

    #[test]
    fn artifact_roundtrip() {
        let artifact = Artifact {
            id: 1,
            project_id: 1,
            artifact_type: "rule".to_string(),
            rel_path: ".orqa/learning/rules/no-stubs.md".to_string(),
            name: "no-stubs".to_string(),
            description: Some("No stubs or placeholders".to_string()),
            content: "# No Stubs\n\nContent here.".to_string(),
            file_hash: Some("abc123".to_string()),
            file_size: Some(1024),
            file_modified_at: Some("2026-03-03T00:00:00Z".to_string()),
            compliance_status: ComplianceStatus::Compliant,
            relationships: Some(vec![ArtifactRelationship {
                relationship_type: "references".to_string(),
                target: ".orqa/learning/rules/error-ownership.md".to_string(),
            }]),
            metadata: Some(serde_json::json!({"priority": "high"})),
            created_at: "2026-03-03T00:00:00Z".to_string(),
            updated_at: "2026-03-03T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&artifact).expect("serialization should succeed");
        let deserialized: Artifact =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.id, artifact.id);
        assert_eq!(deserialized.artifact_type, "rule");
        assert_eq!(deserialized.compliance_status, ComplianceStatus::Compliant);
        assert!(deserialized.relationships.is_some());
        assert_eq!(
            deserialized
                .relationships
                .as_ref()
                .expect("should have relationships")
                .len(),
            1
        );
    }

    #[test]
    fn artifact_summary_serialization() {
        let summary = ArtifactSummary {
            id: 1,
            artifact_type: "agent".to_string(),
            rel_path: ".claude/agents/backend-engineer.md".to_string(),
            name: "backend-engineer".to_string(),
            description: Some("Rust backend agent".to_string()),
            compliance_status: ComplianceStatus::Unknown,
            file_modified_at: None,
        };

        let json = serde_json::to_value(&summary).expect("serialization should succeed");
        assert_eq!(json["artifact_type"], "agent");
        assert_eq!(json["compliance_status"], "unknown");
        assert!(json["file_modified_at"].is_null());
    }

    #[test]
    fn generate_artifact_id_format() {
        let id = generate_artifact_id("TASK");
        let parts: Vec<&str> = id.splitn(2, '-').collect();
        assert_eq!(parts[0], "TASK");
        assert_eq!(parts[1].len(), 8);
        assert!(parts[1].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn generate_artifact_id_uppercases_prefix() {
        let id = generate_artifact_id("epic");
        assert!(id.starts_with("EPIC-"));
    }

    #[test]
    fn is_valid_artifact_id_hex() {
        assert!(is_valid_artifact_id("TASK-1a2b3c4d"));
        assert!(is_valid_artifact_id("EPIC-deadbeef"));
    }

    #[test]
    fn is_valid_artifact_id_legacy() {
        assert!(is_valid_artifact_id("TASK-001"));
        assert!(is_valid_artifact_id("EPIC-42"));
    }

    #[test]
    fn is_valid_artifact_id_compound_prefix() {
        assert!(is_valid_artifact_id("KNOW-SVE-001"));
    }

    #[test]
    fn is_valid_artifact_id_rejects_invalid() {
        assert!(!is_valid_artifact_id("notvalid"));
        assert!(!is_valid_artifact_id("-001"));
        assert!(!is_valid_artifact_id("task-001")); // lowercase prefix
    }

    #[test]
    fn is_hex_artifact_id_detects_hex() {
        assert!(is_hex_artifact_id("TASK-1a2b3c4d"));
        assert!(!is_hex_artifact_id("TASK-001"));
    }

    #[test]
    fn extract_frontmatter_with_valid_frontmatter() {
        let content = "---\ntitle: Test\n---\nBody here.";
        let (fm, body) = extract_frontmatter(content);
        assert_eq!(fm.as_deref(), Some("\ntitle: Test"));
        assert_eq!(body, "Body here.");
    }

    #[test]
    fn extract_frontmatter_without_frontmatter() {
        let content = "Just body text.";
        let (fm, body) = extract_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(body, "Just body text.");
    }
}
