//! Artifact domain types for the OrqaStudio engine.
//!
//! Defines the core structs and enums used to represent governance artifacts (.orqa/ files),
//! their navigation tree structure, and frontmatter shapes for each artifact kind.
//! These types are serialized over the Tauri IPC boundary and used by the frontend.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A fully-loaded governance artifact with content and compliance metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Database row ID.
    pub id: i64,
    /// ID of the project this artifact belongs to.
    pub project_id: i64,
    /// Type category of the artifact (agent, rule, knowledge, doc).
    pub artifact_type: ArtifactType,
    /// Relative path from the project root (e.g. `.orqa/rules/RULE-abc.md`).
    pub rel_path: String,
    /// Display name of the artifact.
    pub name: String,
    /// Short description from frontmatter.
    pub description: Option<String>,
    /// Full raw content of the artifact file.
    pub content: String,
    /// SHA-256 hash of the file content for change detection.
    pub file_hash: Option<String>,
    /// Size of the artifact file in bytes.
    pub file_size: Option<i64>,
    /// ISO-8601 timestamp of the file's last modification.
    pub file_modified_at: Option<String>,
    /// Whether this artifact complies with current enforcement rules.
    pub compliance_status: ComplianceStatus,
    /// Typed relationships declared in this artifact's frontmatter.
    pub relationships: Option<Vec<ArtifactRelationship>>,
    /// Arbitrary metadata for extension by plugins.
    pub metadata: Option<serde_json::Value>,
    /// ISO-8601 timestamp when this artifact record was created.
    pub created_at: String,
    /// ISO-8601 timestamp of the last update to this artifact record.
    pub updated_at: String,
}

/// A lightweight summary of a governance artifact without content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSummary {
    /// Database row ID.
    pub id: i64,
    /// Type category of the artifact.
    pub artifact_type: ArtifactType,
    /// Relative path from the project root.
    pub rel_path: String,
    /// Display name of the artifact.
    pub name: String,
    /// Short description from frontmatter.
    pub description: Option<String>,
    /// Whether this artifact complies with current enforcement rules.
    pub compliance_status: ComplianceStatus,
    /// ISO-8601 timestamp of the file's last modification.
    pub file_modified_at: Option<String>,
}

/// The type of a governance artifact — determines its storage location and schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    /// An agent definition (`.orqa/agents/`).
    Agent,
    /// An enforcement rule (`.orqa/rules/`).
    Rule,
    /// A knowledge artifact (`.orqa/knowledge/`).
    Knowledge,
    /// A documentation file (`docs/` or other markdown).
    Doc,
}

/// Whether an artifact is currently compliant with enforcement rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    /// All enforcement checks passed.
    Compliant,
    /// One or more enforcement checks failed.
    NonCompliant,
    /// Compliance has not yet been evaluated.
    Unknown,
    /// An error occurred during compliance evaluation.
    Error,
}

/// A typed relationship from one artifact to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRelationship {
    /// The relationship type (e.g. "references", "delivers", "depends-on").
    #[serde(rename = "type")]
    pub relationship_type: String,
    /// Target artifact path or ID.
    pub target: String,
}

/// YAML frontmatter metadata extracted from a documentation file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocFrontmatter {
    /// Document title.
    pub title: Option<String>,
    /// Categorization label (e.g. `"architecture"`, `"process"`).
    pub category: Option<String>,
    /// Tags for filtering and discovery.
    #[serde(default)]
    pub tags: Vec<String>,
    /// ISO-8601 date string when the document was created.
    pub created: Option<String>,
    /// ISO-8601 date string when the document was last updated.
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
    pub children: Option<Vec<Self>>,
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
    /// Field name as it appears in the JSON Schema.
    pub name: String,
    /// All allowed values for this field, in schema-defined order.
    pub values: Vec<String>,
}

/// A sortable field derived from a JSON Schema date or string property.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortableField {
    /// Field name as it appears in the JSON Schema.
    pub name: String,
    /// `"date"` or `"string"`
    pub field_type: String,
}

/// Default sort configuration for a navigation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    /// Field name to sort by.
    pub field: String,
    /// Sort direction: `"asc"` or `"desc"`.
    pub direction: String,
}

/// A labelled section in a layout-based navigation view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSection {
    /// Human-readable section label.
    pub label: String,
    /// Optional description of what this section contains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Artifact IDs or keys displayed in this section, in order.
    pub items: Vec<String>,
}

/// Layout configuration for a navigation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationLayout {
    /// Ordered list of layout sections.
    pub sections: Vec<LayoutSection>,
    /// Label to show for items not assigned to any section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized: Option<String>,
}

/// Default navigation behaviour for a type (sort, group, filters).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationDefaults {
    /// Default sort field and direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortConfig>,
    /// Default group-by field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Custom ordering for group values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_order: Option<HashMap<String, Vec<String>>>,
    /// Default filter selections: field name → selected values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, Vec<String>>>,
    /// Group labels that should be collapsed by default in the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed_groups: Option<Vec<String>>,
}

/// Navigation configuration loaded from `_navigation.json` in a type directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationConfig {
    /// Default sort, group, and filter settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<NavigationDefaults>,
    /// Fixed layout sections for this type.
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
    /// Relative path to the group folder (e.g. ".orqa/implementation").
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
    /// Relative path to the type folder (e.g. ".orqa/implementation/milestones").
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

/// YAML frontmatter metadata extracted from a milestone file (`.orqa/milestones/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MilestoneFrontmatter {
    /// Unique milestone ID.
    pub id: Option<String>,
    /// Milestone title.
    pub title: Option<String>,
    /// Current status value.
    pub status: Option<String>,
    /// ISO-8601 creation date.
    pub created: Option<String>,
    /// ISO-8601 last-updated date.
    pub updated: Option<String>,
    /// ISO-8601 target deadline date.
    pub deadline: Option<String>,
    /// Short description of the milestone.
    pub description: Option<String>,
    /// Tags for filtering and discovery.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// YAML frontmatter metadata extracted from an epic file (`.orqa/epics/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpicFrontmatter {
    /// Unique epic ID.
    pub id: Option<String>,
    /// Epic title.
    pub title: Option<String>,
    /// Current status value.
    pub status: Option<String>,
    /// Priority level (e.g. `"P1"`, `"P2"`).
    pub priority: Option<String>,
    /// Parent milestone ID.
    pub milestone: Option<String>,
    /// ISO-8601 creation date.
    pub created: Option<String>,
    /// ISO-8601 last-updated date.
    pub updated: Option<String>,
    /// ISO-8601 target deadline date.
    pub deadline: Option<String>,
    /// Short description of the epic.
    pub description: Option<String>,
    /// Assigned team member.
    pub assignee: Option<String>,
    /// Pillar keys this epic contributes to.
    #[serde(default)]
    pub pillar: Vec<String>,
    /// Tags for filtering and discovery.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// YAML frontmatter metadata extracted from a task file (`.orqa/tasks/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskFrontmatter {
    /// Unique task ID.
    pub id: Option<String>,
    /// Task title.
    pub title: Option<String>,
    /// Current status value.
    pub status: Option<String>,
    /// Parent epic ID.
    pub epic: Option<String>,
    /// ISO-8601 creation date.
    pub created: Option<String>,
    /// ISO-8601 last-updated date.
    pub updated: Option<String>,
    /// Assigned team member.
    pub assignee: Option<String>,
    /// Short description of the task.
    pub description: Option<String>,
    /// Tags for filtering and discovery.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// YAML frontmatter metadata extracted from an idea file (`.orqa/ideas/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdeaFrontmatter {
    /// Unique idea ID.
    pub id: Option<String>,
    /// Idea title.
    pub title: Option<String>,
    /// Current status value.
    pub status: Option<String>,
    /// ISO-8601 creation date.
    pub created: Option<String>,
    /// ISO-8601 last-updated date.
    pub updated: Option<String>,
    /// Short description of the idea.
    pub description: Option<String>,
    /// Path to the epic or milestone this idea was promoted to.
    #[serde(rename = "promoted-to")]
    pub promoted_to: Option<String>,
    /// Pillar keys this idea contributes to.
    #[serde(default)]
    pub pillar: Vec<String>,
    /// Tags for filtering and discovery.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// YAML frontmatter metadata extracted from a decision record file (`.orqa/decisions/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecisionFrontmatter {
    /// Unique decision record ID.
    pub id: Option<String>,
    /// Decision title.
    pub title: Option<String>,
    /// Current status value (e.g. `"proposed"`, `"accepted"`, `"superseded"`).
    pub status: Option<String>,
    /// Category (e.g. `"architecture"`, `"process"`, `"tooling"`).
    pub category: Option<String>,
    /// ISO-8601 creation date.
    pub created: Option<String>,
    /// ISO-8601 last-updated date.
    pub updated: Option<String>,
    /// Short description of the decision.
    pub description: Option<String>,
    /// Tags for filtering and discovery.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// YAML frontmatter metadata extracted from a lesson file (`.orqa/lessons/`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LessonFrontmatter {
    /// Unique lesson ID.
    pub id: Option<String>,
    /// Lesson title.
    pub title: Option<String>,
    /// Category: `"process"`, `"coding"`, or `"architecture"`.
    pub category: Option<String>,
    /// Current status (e.g. `"active"`, `"promoted"`, `"resolved"`).
    pub status: Option<String>,
    /// Number of times this pattern has recurred.
    pub recurrence: Option<i64>,
    /// Path to the rule or standard this lesson was promoted to.
    #[serde(rename = "promoted-to")]
    pub promoted_to: Option<String>,
    /// ISO-8601 creation date.
    pub created: Option<String>,
    /// ISO-8601 last-updated date.
    pub updated: Option<String>,
    /// Short description of the lesson.
    pub description: Option<String>,
    /// Tags for filtering and discovery.
    #[serde(default)]
    pub tags: Vec<String>,
}
