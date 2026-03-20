//! Shared types used by the LSP server.
//!
//! This module contains a self-contained subset of the OrqaStudio project
//! settings and artifact graph types needed for LSP validation. These are
//! intentionally decoupled from the Tauri app so this crate can be built
//! independently as a standalone binary.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Project settings (subset required for graph building)
// ---------------------------------------------------------------------------

/// A single artifact type with a filesystem path to scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactTypeConfig {
    pub key: String,
    pub path: String,
}

/// An entry in the artifacts config — either a direct type or a group of types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactEntry {
    Group {
        key: String,
        #[serde(default)]
        label: Option<String>,
        children: Vec<ArtifactTypeConfig>,
    },
    Type(ArtifactTypeConfig),
}

/// A child project reference in an organisation-mode project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildProjectConfig {
    pub name: String,
    pub path: String,
}

/// Minimal project settings needed by the graph builder.
///
/// This is a permissive subset — unknown fields are ignored via `#[serde(default)]`
/// on all fields, so the standalone LSP can parse any valid `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSettings {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub organisation: bool,
    #[serde(default)]
    pub projects: Vec<ChildProjectConfig>,
    #[serde(default)]
    pub artifacts: Vec<ArtifactEntry>,
}

// ---------------------------------------------------------------------------
// Artifact graph types
// ---------------------------------------------------------------------------

/// A bidirectional graph of all governance artifacts in `.orqa/`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtifactGraph {
    /// All artifact nodes, keyed by their `id` frontmatter value (e.g. "EPIC-048").
    pub nodes: HashMap<String, ArtifactNode>,
    /// Reverse-lookup index: relative file path → artifact ID.
    pub path_index: HashMap<String, String>,
}

/// A single artifact node in the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactNode {
    /// Frontmatter `id` field.
    pub id: String,
    /// Source project name in organisation mode, or `None` for single-project mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Relative path from the project root.
    pub path: String,
    /// Inferred category string (e.g. "epic", "task", "milestone").
    pub artifact_type: String,
}

/// A directed reference from one artifact to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    /// The artifact ID that is referenced (the link target).
    pub target_id: String,
    /// Name of the frontmatter field that contains this reference.
    pub field: String,
    /// ID of the artifact that declares this reference (the link source).
    pub source_id: String,
    /// Semantic relationship type (e.g. "enforced-by", "grounded-by", "delivers").
    pub relationship_type: Option<String>,
}

/// Registry mapping path prefixes to artifact type keys.
pub type TypeRegistry = Vec<(String, String)>;
