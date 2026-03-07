use serde::{Deserialize, Serialize};

use crate::error::OrqaError;

/// Parse a string into an `ArtifactType`, returning a validation error for unknown types.
pub fn parse_artifact_type(s: &str) -> Result<ArtifactType, OrqaError> {
    match s {
        "agent" => Ok(ArtifactType::Agent),
        "rule" => Ok(ArtifactType::Rule),
        "skill" => Ok(ArtifactType::Skill),
        "hook" => Ok(ArtifactType::Hook),
        "doc" => Ok(ArtifactType::Doc),
        other => Err(OrqaError::Validation(format!(
            "unknown artifact type: {other} (valid: agent, rule, skill, hook, doc)"
        ))),
    }
}

/// Derive the relative path for an artifact based on its type and name.
pub fn derive_rel_path(artifact_type: &ArtifactType, name: &str) -> String {
    let sanitized = name.replace(' ', "-").to_lowercase();

    match artifact_type {
        ArtifactType::Agent => format!(".claude/agents/{sanitized}.md"),
        ArtifactType::Rule => format!(".claude/rules/{sanitized}.md"),
        ArtifactType::Skill => format!(".claude/skills/{sanitized}/SKILL.md"),
        ArtifactType::Hook => format!(".claude/hooks/{sanitized}.sh"),
        ArtifactType::Doc => format!("docs/{sanitized}.md"),
    }
}

/// Infer an `ArtifactType` from a `.claude/` relative path prefix.
pub fn infer_artifact_type_from_path(rel_path: &str) -> ArtifactType {
    if rel_path.starts_with(".claude/agents") {
        ArtifactType::Agent
    } else if rel_path.starts_with(".claude/rules") {
        ArtifactType::Rule
    } else if rel_path.starts_with(".claude/skills") {
        ArtifactType::Skill
    } else if rel_path.starts_with(".claude/hooks") {
        ArtifactType::Hook
    } else {
        ArtifactType::Doc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: i64,
    pub project_id: i64,
    pub artifact_type: ArtifactType,
    pub rel_path: String,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub file_hash: Option<String>,
    pub file_size: Option<i64>,
    pub file_modified_at: Option<String>,
    pub compliance_status: ComplianceStatus,
    pub relationships: Option<Vec<ArtifactRelationship>>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSummary {
    pub id: i64,
    pub artifact_type: ArtifactType,
    pub rel_path: String,
    pub name: String,
    pub description: Option<String>,
    pub compliance_status: ComplianceStatus,
    pub file_modified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    Agent,
    Rule,
    Skill,
    Hook,
    Doc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Unknown,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRelationship {
    #[serde(rename = "type")]
    pub relationship_type: String,
    pub target: String,
}

/// A node in the documentation tree. Directories have children; markdown files have a path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocNode {
    /// Display name: filename without `.md`, hyphens replaced with spaces, title-cased.
    pub label: String,
    /// Relative path from `docs/` without `.md` extension (e.g. `"product/vision"`). `None` for directories.
    pub path: Option<String>,
    /// Child nodes for directories. `None` for leaf files.
    pub children: Option<Vec<DocNode>>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
            parse_artifact_type("skill"),
            Ok(ArtifactType::Skill)
        ));
        assert!(matches!(
            parse_artifact_type("hook"),
            Ok(ArtifactType::Hook)
        ));
        assert!(matches!(parse_artifact_type("doc"), Ok(ArtifactType::Doc)));
    }

    #[test]
    fn parse_artifact_type_invalid() {
        let result = parse_artifact_type("unknown");
        assert!(matches!(result, Err(OrqaError::Validation(_))));
    }

    #[test]
    fn derive_rel_path_agent() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Agent, "backend-engineer"),
            ".claude/agents/backend-engineer.md"
        );
    }

    #[test]
    fn derive_rel_path_skill() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Skill, "chunkhound"),
            ".claude/skills/chunkhound/SKILL.md"
        );
    }

    #[test]
    fn derive_rel_path_sanitizes_spaces() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Rule, "No Stubs Rule"),
            ".claude/rules/no-stubs-rule.md"
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
            serde_json::to_value(ArtifactType::Skill)
                .expect("serialization should succeed")
                .as_str(),
            Some("skill")
        );
        assert_eq!(
            serde_json::to_value(ArtifactType::Hook)
                .expect("serialization should succeed")
                .as_str(),
            Some("hook")
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
            target: ".claude/rules/coding-standards.md".to_string(),
        };

        let json = serde_json::to_value(&rel).expect("serialization should succeed");
        // Serde renames relationship_type -> "type" in JSON
        assert_eq!(json["type"], "references");
        assert_eq!(json["target"], ".claude/rules/coding-standards.md");
    }

    #[test]
    fn artifact_roundtrip() {
        let artifact = Artifact {
            id: 1,
            project_id: 1,
            artifact_type: ArtifactType::Rule,
            rel_path: ".claude/rules/no-stubs.md".to_string(),
            name: "no-stubs".to_string(),
            description: Some("No stubs or placeholders".to_string()),
            content: "# No Stubs\n\nContent here.".to_string(),
            file_hash: Some("abc123".to_string()),
            file_size: Some(1024),
            file_modified_at: Some("2026-03-03T00:00:00Z".to_string()),
            compliance_status: ComplianceStatus::Compliant,
            relationships: Some(vec![ArtifactRelationship {
                relationship_type: "references".to_string(),
                target: ".claude/rules/error-ownership.md".to_string(),
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
}
