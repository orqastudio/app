//! Artifact domain types for the OrqaStudio engine.
//!
//! Defines the core structs used to represent governance artifacts (.orqa/ files)
//! and their navigation tree structure. Artifact types are opaque strings — the
//! engine does not enumerate them; plugins declare artifact types via their manifests.
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
    /// Opaque artifact type key declared by a plugin (e.g. `"task"`, `"epic"`).
    /// The engine treats this as an opaque string; plugins define what values are valid.
    pub artifact_type: String,
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
    /// Opaque artifact type key declared by a plugin.
    pub artifact_type: String,
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

/// Generic YAML frontmatter parsed from any artifact file.
///
/// The engine treats frontmatter as an opaque key-value map. Plugins declare
/// what fields are valid for their artifact types via JSON Schema. The engine
/// does not interpret specific field names except for universal fields like
/// `id`, `title`, and `status` used for navigation and display.
pub type GenericFrontmatter = HashMap<String, serde_json::Value>;

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
