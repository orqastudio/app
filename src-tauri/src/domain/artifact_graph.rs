use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::OrqaError;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

/// A bidirectional graph of all governance artifacts in `.orqa/`.
///
/// Built by scanning every `.md` file under the project root that carries a
/// YAML `id` field. References between artifacts are extracted from well-known
/// frontmatter fields and inverted in a second pass to produce `references_in`
/// backlinks on each node.
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
    /// Frontmatter `id` field (e.g. "EPIC-048").
    pub id: String,
    /// Relative path from the project root (e.g. ".orqa/planning/epics/EPIC-048.md").
    pub path: String,
    /// Inferred category string (e.g. "epic", "task", "milestone", "idea", "decision").
    pub artifact_type: String,
    /// Frontmatter `title` field, or a humanized fallback from the filename.
    pub title: String,
    /// Frontmatter `description` field.
    pub description: Option<String>,
    /// Frontmatter `status` field.
    pub status: Option<String>,
    /// Full YAML frontmatter parsed into JSON for generic access.
    pub frontmatter: serde_json::Value,
    /// Forward references declared in this node's frontmatter.
    pub references_out: Vec<ArtifactRef>,
    /// Backlinks computed from other nodes' `references_out` during pass 2.
    pub references_in: Vec<ArtifactRef>,
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
}

/// Summary statistics about the artifact graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    /// Total number of nodes (artifacts with an `id` field).
    pub node_count: usize,
    /// Total number of directed edges (sum of all `references_out` lengths).
    pub edge_count: usize,
    /// Nodes that have no `references_out` and no `references_in`.
    pub orphan_count: usize,
    /// References whose `target_id` does not exist in the graph.
    pub broken_ref_count: usize,
}

// ---------------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------------

/// Frontmatter fields that hold a single artifact ID reference.
const SINGLE_REF_FIELDS: &[&str] = &[
    "milestone",
    "epic",
    "promoted-to",
    "supersedes",
    "superseded-by",
    "surpassed-by",
    "promoted-from",
    "assignee",
];

/// Frontmatter fields that hold an array of artifact ID references.
const ARRAY_REF_FIELDS: &[&str] = &[
    "depends-on",
    "blocks",
    "pillars",
    "research-refs",
    "docs-required",
    "docs-produced",
    "skills",
];

/// Build an `ArtifactGraph` by scanning all `.md` files under `orqa_dir`.
///
/// Two-pass algorithm:
/// 1. Walk every `.md` file, parse frontmatter, collect nodes and forward refs.
/// 2. Invert every forward ref into a backlink on the target node.
///
/// Files without an `id` frontmatter field are silently skipped — they are
/// documentation pages, not typed governance artifacts.
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, OrqaError> {
    let orqa_dir = project_path.join(".orqa");

    let mut graph = ArtifactGraph::default();

    // Pass 1: walk all .md files and collect nodes + forward refs.
    walk_directory(&orqa_dir, project_path, &mut graph)?;

    // Pass 2: invert references — add backlinks to target nodes.
    let forward_refs: Vec<ArtifactRef> = graph
        .nodes
        .values()
        .flat_map(|n| n.references_out.iter().cloned())
        .collect();

    for ref_entry in forward_refs {
        if let Some(target_node) = graph.nodes.get_mut(&ref_entry.target_id) {
            target_node.references_in.push(ref_entry);
        }
        // Broken references (target not in nodes) are silently counted via GraphStats.
    }

    Ok(graph)
}

/// Recursively walk a directory, collecting `ArtifactNode` entries into `graph`.
fn walk_directory(
    dir: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
) -> Result<(), OrqaError> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        // Directory doesn't exist — skip silently (some sub-paths may be optional).
        Err(_) => return Ok(()),
    };

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Skip hidden and private entries.
        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        let ft = entry.file_type()?;

        if ft.is_dir() {
            walk_directory(&entry.path(), project_root, graph)?;
        } else if ft.is_file() && name.ends_with(".md") {
            // README files carry navigation metadata, not artifact identity.
            if name.eq_ignore_ascii_case("README.md") {
                continue;
            }
            collect_node(&entry.path(), project_root, graph)?;
        }
    }

    Ok(())
}

/// Parse a single `.md` file and add an `ArtifactNode` to the graph if it has
/// a YAML `id` field.
fn collect_node(
    file_path: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
) -> Result<(), OrqaError> {
    let content = std::fs::read_to_string(file_path)?;

    // Extract the raw YAML frontmatter text.
    let (fm_text, _body) = crate::domain::artifact::extract_frontmatter(&content);
    let Some(fm_text) = fm_text else {
        return Ok(());
    };

    // Parse into a generic serde_yaml::Value first so we can extract any field.
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text)
        .unwrap_or(serde_yaml::Value::Null);

    // Require an `id` field — files without one are not typed artifacts.
    let id = match yaml_value.get("id").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.to_owned(),
        _ => return Ok(()),
    };

    // Compute the relative path from the project root.
    let rel_path = file_path
        .strip_prefix(project_root)
        .unwrap_or(file_path)
        .to_string_lossy()
        // Normalise Windows path separators.
        .replace('\\', "/");

    let title = yaml_value
        .get("title")
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .unwrap_or_else(|| humanize_stem(file_path));

    let description = yaml_value
        .get("description")
        .and_then(|v| v.as_str())
        .map(str::to_owned);

    let status = yaml_value
        .get("status")
        .and_then(|v| v.as_str())
        .map(str::to_owned);

    let artifact_type = infer_artifact_type(&rel_path);

    // Convert frontmatter to serde_json::Value for generic storage.
    let frontmatter_json = yaml_to_json(&yaml_value);

    // Collect forward references from well-known fields.
    let references_out = collect_forward_refs(&yaml_value, &id);

    let node = ArtifactNode {
        id: id.clone(),
        path: rel_path.clone(),
        artifact_type,
        title,
        description,
        status,
        frontmatter: frontmatter_json,
        references_out,
        references_in: Vec::new(),
    };

    graph.nodes.insert(id.clone(), node);
    graph.path_index.insert(rel_path, id);

    Ok(())
}

/// Extract forward references from well-known frontmatter fields.
fn collect_forward_refs(yaml_value: &serde_yaml::Value, source_id: &str) -> Vec<ArtifactRef> {
    let mut refs = Vec::new();

    for &field in SINGLE_REF_FIELDS {
        if let Some(target_id) = yaml_value.get(field).and_then(|v| v.as_str()) {
            let target_id = target_id.trim().to_owned();
            if !target_id.is_empty() {
                refs.push(ArtifactRef {
                    target_id,
                    field: field.to_owned(),
                    source_id: source_id.to_owned(),
                });
            }
        }
    }

    for &field in ARRAY_REF_FIELDS {
        if let Some(seq) = yaml_value.get(field).and_then(|v| v.as_sequence()) {
            for item in seq {
                if let Some(target_id) = item.as_str() {
                    let target_id = target_id.trim().to_owned();
                    if !target_id.is_empty() {
                        refs.push(ArtifactRef {
                            target_id,
                            field: field.to_owned(),
                            source_id: source_id.to_owned(),
                        });
                    }
                }
            }
        }
    }

    refs
}

/// Compute summary statistics for the graph.
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats {
    let node_count = graph.nodes.len();

    let edge_count: usize = graph
        .nodes
        .values()
        .map(|n| n.references_out.len())
        .sum();

    let orphan_count = graph
        .nodes
        .values()
        .filter(|n| n.references_out.is_empty() && n.references_in.is_empty())
        .count();

    let broken_ref_count: usize = graph
        .nodes
        .values()
        .flat_map(|n| n.references_out.iter())
        .filter(|r| !graph.nodes.contains_key(&r.target_id))
        .count();

    GraphStats {
        node_count,
        edge_count,
        orphan_count,
        broken_ref_count,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Infer a human-readable artifact type category from a relative file path.
fn infer_artifact_type(rel_path: &str) -> String {
    if rel_path.contains("/epics/") {
        "epic"
    } else if rel_path.contains("/tasks/") {
        "task"
    } else if rel_path.contains("/milestones/") {
        "milestone"
    } else if rel_path.contains("/ideas/") {
        "idea"
    } else if rel_path.contains("/decisions/") {
        "decision"
    } else if rel_path.contains("/research/") {
        "research"
    } else if rel_path.contains("/lessons/") {
        "lesson"
    } else if rel_path.contains("/rules/") {
        "rule"
    } else if rel_path.contains("/agents/") {
        "agent"
    } else if rel_path.contains("/skills/") {
        "skill"
    } else if rel_path.contains("/hooks/") {
        "hook"
    } else if rel_path.contains("/pillars/") {
        "pillar"
    } else {
        "doc"
    }
    .to_owned()
}

/// Convert a `serde_yaml::Value` to `serde_json::Value`.
///
/// The conversion goes through a JSON round-trip. Any value that cannot be
/// represented in JSON becomes `null`.
fn yaml_to_json(value: &serde_yaml::Value) -> serde_json::Value {
    // serde_yaml Values serialise to JSON via the standard serde pipeline.
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

/// Derive a display title from a file path's stem when no frontmatter title exists.
fn humanize_stem(file_path: &Path) -> String {
    let stem = file_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    // Preserve all-caps IDs like EPIC-001, RULE-023, AD-029.
    let all_caps = stem
        .chars()
        .all(|c| c.is_ascii_uppercase() || c == '-' || c == '_' || c.is_ascii_digit());
    if stem.chars().any(|c| c.is_ascii_uppercase()) && all_caps {
        return stem;
    }

    stem.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut s = first.to_uppercase().to_string();
                    s.extend(chars.flat_map(|c| c.to_lowercase()));
                    s
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    fn write_artifact(dir: &Path, name: &str, content: &str) {
        fs::create_dir_all(dir).expect("create dir");
        fs::write(dir.join(name), content).expect("write file");
    }

    #[test]
    fn empty_orqa_dir_returns_empty_graph() {
        let tmp = make_project();
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
        assert!(graph.path_index.is_empty());
    }

    #[test]
    fn file_without_id_is_skipped() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/planning/epics");
        write_artifact(&epics_dir, "EPIC-001.md", "---\ntitle: No ID\n---\n# Body\n");
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn file_with_id_creates_node() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/planning/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: draft\n---\n# Body\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert_eq!(graph.nodes.len(), 1);
        let node = graph.nodes.get("EPIC-001").expect("node");
        assert_eq!(node.id, "EPIC-001");
        assert_eq!(node.title, "My Epic");
        assert_eq!(node.status.as_deref(), Some("draft"));
        assert_eq!(node.artifact_type, "epic");
    }

    #[test]
    fn single_ref_field_creates_forward_ref() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/planning/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nepic: EPIC-001\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let node = graph.nodes.get("TASK-001").expect("node");
        assert_eq!(node.references_out.len(), 1);
        assert_eq!(node.references_out[0].target_id, "EPIC-001");
        assert_eq!(node.references_out[0].field, "epic");
        assert_eq!(node.references_out[0].source_id, "TASK-001");
    }

    #[test]
    fn backlinks_are_computed_in_pass_two() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/planning/epics");
        let tasks_dir = tmp.path().join(".orqa/planning/tasks");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\n---\n",
        );
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nepic: EPIC-001\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let epic = graph.nodes.get("EPIC-001").expect("epic node");
        assert_eq!(epic.references_in.len(), 1);
        assert_eq!(epic.references_in[0].source_id, "TASK-001");
        assert_eq!(epic.references_in[0].field, "epic");
    }

    #[test]
    fn array_ref_field_creates_multiple_forward_refs() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/planning/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-002.md",
            "---\nid: TASK-002\ntitle: Dependent Task\ndepends-on:\n  - TASK-001\n  - TASK-003\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let node = graph.nodes.get("TASK-002").expect("node");
        assert_eq!(node.references_out.len(), 2);
    }

    #[test]
    fn broken_refs_counted_in_stats() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/planning/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Task\nepic: EPIC-MISSING\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.broken_ref_count, 1);
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.edge_count, 1);
    }

    #[test]
    fn path_index_maps_path_to_id() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/planning/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let key = graph
            .path_index
            .keys()
            .find(|k| k.contains("EPIC-001"))
            .cloned()
            .expect("path key");
        assert_eq!(graph.path_index[&key], "EPIC-001");
    }

    #[test]
    fn readme_files_are_skipped() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/planning/epics");
        write_artifact(&epics_dir, "README.md", "---\nid: SHOULD-SKIP\ntitle: Nav\n---\n");
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn graph_stats_orphan_count() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/planning/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Isolated\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.orphan_count, 1);
    }

    #[test]
    fn infer_artifact_type_variants() {
        assert_eq!(infer_artifact_type(".orqa/planning/epics/EPIC-001.md"), "epic");
        assert_eq!(infer_artifact_type(".orqa/planning/tasks/TASK-001.md"), "task");
        assert_eq!(
            infer_artifact_type(".orqa/planning/milestones/MS-001.md"),
            "milestone"
        );
        assert_eq!(
            infer_artifact_type(".orqa/governance/decisions/AD-001.md"),
            "decision"
        );
        assert_eq!(
            infer_artifact_type(".orqa/governance/lessons/IMPL-001.md"),
            "lesson"
        );
        assert_eq!(
            infer_artifact_type(".orqa/documentation/product/vision.md"),
            "doc"
        );
    }
}
