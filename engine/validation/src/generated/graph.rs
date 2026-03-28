// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: cargo build -p orqa-validation

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

/// A directed reference from one artifact to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    /// The artifact ID that is referenced (the link target).
    pub target_id: String,
    /// Name of the frontmatter field that contains this reference.
    pub field: String,
    /// ID of the artifact that declares this reference (the link source).
    pub source_id: String,
    /// Semantic relationship type (e.g. 'enforced-by', 'grounded'). Only set for refs from the relationships array.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub relationship_type: Option<String>,
}

/// A single artifact node in the bidirectional graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactNode {
    /// Frontmatter 'id' field (e.g. 'EPIC-048').
    pub id: String,
    /// Source project name in organisation mode, or null for single-project mode.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub project: Option<String>,
    /// Relative path from the project root (e.g. '.orqa/implementation/epics/EPIC-048.md').
    pub path: String,
    /// Inferred category string (e.g. 'epic', 'task', 'milestone', 'idea', 'decision').
    pub artifact_type: String,
    /// Frontmatter 'title' field, or a humanized fallback from the filename.
    pub title: String,
    /// Frontmatter 'description' field.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    /// Frontmatter 'status' field.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    /// Frontmatter 'priority' field (e.g. 'P1', 'P2', 'P3').
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub priority: Option<String>,
    /// Full YAML frontmatter parsed into a generic JSON object.
    pub frontmatter: serde_json::Value,
    /// Forward references declared in this node's frontmatter.
    pub references_out: Vec<ArtifactRef>,
    /// Backlinks computed from other nodes' references_out during graph construction.
    pub references_in: Vec<ArtifactRef>,
}

/// A bidirectional graph of all governance artifacts in .orqa/. Built by scanning every .md file that carries a YAML 'id' field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactGraph {
    /// All artifact nodes, keyed by their 'id' frontmatter value (e.g. 'EPIC-048').
    pub nodes: std::collections::HashMap<String, ArtifactNode>,
    /// Reverse-lookup index: relative file path → artifact ID.
    pub path_index: std::collections::HashMap<String, String>,
}

/// Summary statistics about the artifact graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    /// Total number of nodes (artifacts with an 'id' field).
    pub node_count: usize,
    /// Total number of directed edges (sum of all references_out lengths).
    pub edge_count: usize,
    /// Nodes that have no references_out and no references_in.
    pub orphan_count: usize,
    /// References whose target_id does not exist in the graph.
    pub broken_ref_count: usize,
}

