use std::collections::HashMap;

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
        ArtifactType::Agent => format!(".orqa/team/agents/{sanitized}.md"),
        ArtifactType::Rule => format!(".orqa/governance/rules/{sanitized}.md"),
        ArtifactType::Skill => format!(".orqa/team/skills/{sanitized}/SKILL.md"),
        ArtifactType::Hook => format!(".orqa/governance/hooks/{sanitized}.sh"),
        ArtifactType::Doc => format!("docs/{sanitized}.md"),
    }
}

/// Infer an `ArtifactType` from a `.orqa/` relative path prefix.
pub fn infer_artifact_type_from_path(rel_path: &str) -> ArtifactType {
    if rel_path.starts_with(".orqa/team/agents") {
        ArtifactType::Agent
    } else if rel_path.starts_with(".orqa/governance/rules") {
        ArtifactType::Rule
    } else if rel_path.starts_with(".orqa/team/skills") {
        ArtifactType::Skill
    } else if rel_path.starts_with(".orqa/governance/hooks") {
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

/// YAML frontmatter metadata extracted from a documentation file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocFrontmatter {
    pub title: Option<String>,
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
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
    /// All scalar YAML frontmatter fields for filtering and sorting. `None` for directories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontmatter: Option<HashMap<String, serde_json::Value>>,
    /// Status value from YAML frontmatter (e.g. `"draft"`, `"in-progress"`, `"done"`). `None` for
    /// directories and files without a `status` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Short description for leaf nodes: YAML `description` field if present, otherwise the
    /// first paragraph of the body. `None` for directories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Lucide icon name sourced from the directory's README.md frontmatter. `None` for leaf files
    /// and directories without a README.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// A filterable field derived from a JSON Schema enum property.
///
/// The `values` array preserves the original array order from the schema, which
/// is intentional (e.g. lifecycle ordering for status fields).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterableField {
    pub name: String,
    pub values: Vec<String>,
}

/// A sortable field derived from a JSON Schema date or string property.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortableField {
    pub name: String,
    /// `"date"` or `"string"`
    pub field_type: String,
}

/// Default sort configuration for a navigation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    pub field: String,
    pub direction: String,
}

/// A labelled section in a layout-based navigation view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSection {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub items: Vec<String>,
}

/// Layout configuration for a navigation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationLayout {
    pub sections: Vec<LayoutSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized: Option<String>,
}

/// Default navigation behaviour for a type (sort, group, filters).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_order: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, Vec<String>>>,
    /// Group labels that should be collapsed by default in the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed_groups: Option<Vec<String>>,
}

/// Navigation configuration loaded from `_navigation.json` in a type directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<NavigationDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<NavigationLayout>,
}

/// README frontmatter for navigation discovery.
///
/// Each group and type folder in `.orqa/` has a `README.md` with this frontmatter.
/// The `role` field distinguishes group folders ("group") from artifact-list folders ("artifacts").
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NavReadme {
    /// "group" or "artifacts"
    pub role: Option<String>,
    /// Human-readable display label (e.g. "Planning", "Milestones")
    pub label: Option<String>,
    /// Short description of the folder's contents
    pub description: Option<String>,
    /// Lucide icon name (e.g. "clipboard-list", "target")
    pub icon: Option<String>,
    /// Numeric sort order within the parent
    pub sort: Option<i64>,
}

/// A group folder in the navigation tree (e.g. Planning, Governance).
///
/// Groups contain one or more `NavType` folders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavGroup {
    /// Human-readable display label.
    pub label: String,
    /// Short description of the group's purpose.
    pub description: String,
    /// Lucide icon name.
    pub icon: String,
    /// Numeric sort order (lower = first).
    pub sort: i64,
    /// Relative path to the group folder (e.g. ".orqa/planning").
    pub path: String,
    /// Raw content of the group's README.md.
    pub readme_content: String,
    /// Artifact type folders nested within this group.
    pub types: Vec<NavType>,
}

/// An artifact type folder within a group (e.g. Milestones, Rules).
///
/// Types contain a flat list of `DocNode` artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavType {
    /// Human-readable display label.
    pub label: String,
    /// Short description of the type's purpose.
    pub description: String,
    /// Lucide icon name.
    pub icon: String,
    /// Numeric sort order (lower = first).
    pub sort: i64,
    /// Relative path to the type folder (e.g. ".orqa/planning/milestones").
    pub path: String,
    /// Raw content of the type's README.md.
    pub readme_content: String,
    /// Artifact nodes within this type folder.
    pub nodes: Vec<DocNode>,
    /// Enum-valued properties from this type's `schema.json`, suitable for filtering.
    pub filterable_fields: Vec<FilterableField>,
    /// Date and string properties from this type's `schema.json`, suitable for sorting.
    pub sortable_fields: Vec<SortableField>,
    /// Navigation defaults and layout loaded from `_navigation.json`, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation_config: Option<NavigationConfig>,
}

/// The full navigation tree returned by `artifact_scan_tree`.
///
/// Groups are sorted by their `sort` field. Within each group, types are sorted
/// by their `sort` field. Within each type, nodes are sorted alphabetically by label.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavTree {
    /// All top-level groups discovered from `.orqa/` and `docs/`.
    pub groups: Vec<NavGroup>,
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
/// returns `(Default, full_content)`.
pub fn parse_frontmatter<T: serde::de::DeserializeOwned + Default>(content: &str) -> (T, String) {
    let (fm_text, body) = extract_frontmatter(content);
    let frontmatter = fm_text
        .and_then(|text| serde_yaml::from_str::<T>(&text).ok())
        .unwrap_or_default();
    (frontmatter, body)
}

/// Convenience alias: parse doc frontmatter.
pub fn parse_doc_frontmatter(content: &str) -> (DocFrontmatter, String) {
    parse_frontmatter::<DocFrontmatter>(content)
}

/// YAML frontmatter metadata extracted from a milestone file (`.orqa/milestones/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MilestoneFrontmatter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub deadline: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Convenience alias: parse milestone frontmatter.
pub fn parse_milestone_frontmatter(content: &str) -> (MilestoneFrontmatter, String) {
    parse_frontmatter::<MilestoneFrontmatter>(content)
}

/// YAML frontmatter metadata extracted from an epic file (`.orqa/epics/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpicFrontmatter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub milestone: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub deadline: Option<String>,
    pub description: Option<String>,
    pub assignee: Option<String>,
    #[serde(default)]
    pub pillar: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Convenience alias: parse epic frontmatter.
pub fn parse_epic_frontmatter(content: &str) -> (EpicFrontmatter, String) {
    parse_frontmatter::<EpicFrontmatter>(content)
}

/// YAML frontmatter metadata extracted from a task file (`.orqa/tasks/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskFrontmatter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub epic: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub assignee: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Convenience alias: parse task frontmatter.
pub fn parse_task_frontmatter(content: &str) -> (TaskFrontmatter, String) {
    parse_frontmatter::<TaskFrontmatter>(content)
}

/// YAML frontmatter metadata extracted from an idea file (`.orqa/ideas/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdeaFrontmatter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "promoted-to")]
    pub promoted_to: Option<String>,
    #[serde(default)]
    pub pillar: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Convenience alias: parse idea frontmatter.
pub fn parse_idea_frontmatter(content: &str) -> (IdeaFrontmatter, String) {
    parse_frontmatter::<IdeaFrontmatter>(content)
}

/// YAML frontmatter metadata extracted from a decision record file (`.orqa/decisions/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecisionFrontmatter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Convenience alias: parse decision frontmatter.
pub fn parse_decision_frontmatter(content: &str) -> (DecisionFrontmatter, String) {
    parse_frontmatter::<DecisionFrontmatter>(content)
}

/// YAML frontmatter metadata extracted from a lesson file (`.orqa/lessons/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LessonFrontmatter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub recurrence: Option<i64>,
    #[serde(rename = "promoted-to")]
    pub promoted_to: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Convenience alias: parse lesson frontmatter.
pub fn parse_lesson_frontmatter(content: &str) -> (LessonFrontmatter, String) {
    parse_frontmatter::<LessonFrontmatter>(content)
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
            ".orqa/team/agents/backend-engineer.md"
        );
    }

    #[test]
    fn derive_rel_path_skill() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Skill, "chunkhound"),
            ".orqa/team/skills/chunkhound/SKILL.md"
        );
    }

    #[test]
    fn derive_rel_path_sanitizes_spaces() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Rule, "No Stubs Rule"),
            ".orqa/governance/rules/no-stubs-rule.md"
        );
    }

    #[test]
    fn infer_artifact_type_agents() {
        assert_eq!(
            infer_artifact_type_from_path(".orqa/team/agents/foo.md"),
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
            target: ".orqa/governance/rules/coding-standards.md".to_string(),
        };

        let json = serde_json::to_value(&rel).expect("serialization should succeed");
        // Serde renames relationship_type -> "type" in JSON
        assert_eq!(json["type"], "references");
        assert_eq!(json["target"], ".orqa/governance/rules/coding-standards.md");
    }

    #[test]
    fn artifact_roundtrip() {
        let artifact = Artifact {
            id: 1,
            project_id: 1,
            artifact_type: ArtifactType::Rule,
            rel_path: ".orqa/governance/rules/no-stubs.md".to_string(),
            name: "no-stubs".to_string(),
            description: Some("No stubs or placeholders".to_string()),
            content: "# No Stubs\n\nContent here.".to_string(),
            file_hash: Some("abc123".to_string()),
            file_size: Some(1024),
            file_modified_at: Some("2026-03-03T00:00:00Z".to_string()),
            compliance_status: ComplianceStatus::Compliant,
            relationships: Some(vec![ArtifactRelationship {
                relationship_type: "references".to_string(),
                target: ".orqa/governance/rules/error-ownership.md".to_string(),
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
            rel_path: ".orqa/team/agents/backend-engineer.md".to_string(),
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
