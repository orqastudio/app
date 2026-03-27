// Artifact domain types for the OrqaStudio engine.
//
// Defines the core structs and enums used to represent governance artifacts (.orqa/ files),
// their navigation tree structure, and frontmatter shapes for each artifact kind.
// These types are serialized over the Tauri IPC boundary and used by the frontend.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A fully-loaded governance artifact with content and compliance metadata.
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

/// A lightweight summary of a governance artifact without content.
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

/// The type of a governance artifact — determines its storage location and schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    Agent,
    Rule,
    Knowledge,
    Doc,
}

/// Whether an artifact is currently compliant with enforcement rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Unknown,
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
    /// Relative path to the group folder (e.g. ".orqa/delivery").
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
    /// Relative path to the type folder (e.g. ".orqa/delivery/milestones").
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
