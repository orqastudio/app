// orqa-artifact: Artifact business logic for the OrqaStudio engine.
//
// This crate provides the business logic for working with governance artifacts:
// ID generation and validation, type parsing, path derivation, frontmatter
// extraction/parsing, filesystem I/O, and navigation tree scanning.
// All functions operate on types defined in `orqa_engine_types::types::artifact`.
//
// Type definitions (structs/enums) live in `orqa-engine-types`. This crate
// contains only the behaviour -- the functions that act on those types.
//
// Submodules:
//   `fs`     -- filesystem helpers: write, read, scan directories
//   `reader` -- navigation tree scanner driven by project.json artifact config

pub mod fs;
pub mod reader;

use rand::Rng;

use orqa_engine_types::error::EngineError;
use orqa_engine_types::types::artifact::{
    ArtifactType, DecisionFrontmatter, DocFrontmatter, EpicFrontmatter, IdeaFrontmatter,
    LessonFrontmatter, MilestoneFrontmatter, TaskFrontmatter,
};

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

/// Parse a string into an `ArtifactType`, returning a validation error for unknown types.
pub fn parse_artifact_type(s: &str) -> Result<ArtifactType, EngineError> {
    match s {
        "agent" => Ok(ArtifactType::Agent),
        "rule" => Ok(ArtifactType::Rule),
        "knowledge" => Ok(ArtifactType::Knowledge),
        "doc" => Ok(ArtifactType::Doc),
        other => Err(EngineError::Validation(format!(
            "unknown artifact type: {other} (valid: agent, rule, knowledge, doc)"
        ))),
    }
}

/// Derive the relative path for an artifact based on its type and name.
///
/// The name is sanitized (spaces become hyphens, lowercased) before being
/// embedded in the path. Rules live in `.orqa/learning/rules/`. Knowledge
/// files live under `.orqa/documentation/<topic>/knowledge/` but because the
/// topic is not known at derivation time, a flat fallback path is used. Agents
/// are ephemeral and have no canonical `.orqa/` location.
pub fn derive_rel_path(artifact_type: &ArtifactType, name: &str) -> String {
    let sanitized = name.replace(' ', "-").to_lowercase();

    match artifact_type {
        ArtifactType::Agent => format!(".claude/agents/{sanitized}.md"),
        ArtifactType::Rule => format!(".orqa/learning/rules/{sanitized}.md"),
        ArtifactType::Knowledge => format!(".orqa/documentation/knowledge/{sanitized}.md"),
        ArtifactType::Doc => format!("docs/{sanitized}.md"),
    }
}

/// Infer an `ArtifactType` from a `.orqa/` relative path prefix.
///
/// Defaults to `ArtifactType::Doc` for paths that do not match any known prefix.
pub fn infer_artifact_type_from_path(rel_path: &str) -> ArtifactType {
    if rel_path.starts_with(".claude/agents") {
        ArtifactType::Agent
    } else if rel_path.starts_with(".orqa/learning/rules") {
        ArtifactType::Rule
    } else if rel_path.contains("/knowledge/") {
        ArtifactType::Knowledge
    } else {
        ArtifactType::Doc
    }
}

/// Extract the YAML text between `---` delimiters from a markdown file.
///
/// Returns `(yaml_text, body)`. If no frontmatter is present, returns `(None, full_content)`.
pub fn extract_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_string());
    }

    let after_open = &trimmed[3..];
    let Some(close_pos) = after_open.find("\n---") else {
        return (None, content.to_string());
    };

    let fm_text = after_open[..close_pos].to_string();
    let body = after_open[close_pos + 4..]
        .trim_start_matches('\n')
        .to_string();
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

/// Parse doc frontmatter from markdown content.
pub fn parse_doc_frontmatter(content: &str) -> (DocFrontmatter, String) {
    parse_frontmatter::<DocFrontmatter>(content)
}

/// Parse milestone frontmatter from markdown content.
pub fn parse_milestone_frontmatter(content: &str) -> (MilestoneFrontmatter, String) {
    parse_frontmatter::<MilestoneFrontmatter>(content)
}

/// Parse epic frontmatter from markdown content.
pub fn parse_epic_frontmatter(content: &str) -> (EpicFrontmatter, String) {
    parse_frontmatter::<EpicFrontmatter>(content)
}

/// Parse task frontmatter from markdown content.
pub fn parse_task_frontmatter(content: &str) -> (TaskFrontmatter, String) {
    parse_frontmatter::<TaskFrontmatter>(content)
}

/// Parse idea frontmatter from markdown content.
pub fn parse_idea_frontmatter(content: &str) -> (IdeaFrontmatter, String) {
    parse_frontmatter::<IdeaFrontmatter>(content)
}

/// Parse decision frontmatter from markdown content.
pub fn parse_decision_frontmatter(content: &str) -> (DecisionFrontmatter, String) {
    parse_frontmatter::<DecisionFrontmatter>(content)
}

/// Parse lesson frontmatter from markdown content.
pub fn parse_lesson_frontmatter(content: &str) -> (LessonFrontmatter, String) {
    parse_frontmatter::<LessonFrontmatter>(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_engine_types::types::artifact::{
        Artifact, ArtifactRelationship, ArtifactSummary, ComplianceStatus,
    };

    #[test]
    fn parse_artifact_type_valid() {
        assert!(matches!(
            parse_artifact_type("agent"),
            Ok(ArtifactType::Agent)
        ));
        assert!(matches!(
            parse_artifact_type("rule"),
            Ok(ArtifactType::Rule)
        ));
        assert!(matches!(
            parse_artifact_type("knowledge"),
            Ok(ArtifactType::Knowledge)
        ));
        assert!(matches!(parse_artifact_type("doc"), Ok(ArtifactType::Doc)));
    }

    #[test]
    fn parse_artifact_type_invalid() {
        let result = parse_artifact_type("unknown");
        assert!(matches!(result, Err(EngineError::Validation(_))));
    }

    #[test]
    fn derive_rel_path_agent() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Agent, "backend-engineer"),
            ".claude/agents/backend-engineer.md"
        );
    }

    #[test]
    fn derive_rel_path_knowledge() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Knowledge, "chunkhound"),
            ".orqa/documentation/knowledge/chunkhound.md"
        );
    }

    #[test]
    fn derive_rel_path_sanitizes_spaces() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Rule, "No Stubs Rule"),
            ".orqa/learning/rules/no-stubs-rule.md"
        );
    }

    #[test]
    fn infer_artifact_type_agents() {
        assert_eq!(
            infer_artifact_type_from_path(".claude/agents/foo.md"),
            ArtifactType::Agent
        );
    }

    #[test]
    fn infer_artifact_type_doc_fallback() {
        assert_eq!(
            infer_artifact_type_from_path("docs/something.md"),
            ArtifactType::Doc
        );
    }

    #[test]
    fn artifact_type_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(ArtifactType::Agent)
                .expect("serialization should succeed")
                .as_str(),
            Some("agent")
        );
        assert_eq!(
            serde_json::to_value(ArtifactType::Rule)
                .expect("serialization should succeed")
                .as_str(),
            Some("rule")
        );
        assert_eq!(
            serde_json::to_value(ArtifactType::Knowledge)
                .expect("serialization should succeed")
                .as_str(),
            Some("knowledge")
        );
        assert_eq!(
            serde_json::to_value(ArtifactType::Doc)
                .expect("serialization should succeed")
                .as_str(),
            Some("doc")
        );
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
            artifact_type: ArtifactType::Rule,
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
        assert_eq!(deserialized.artifact_type, ArtifactType::Rule);
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
            artifact_type: ArtifactType::Agent,
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
