//! Artifact graph — scanning, building, and querying `.orqa/` artifacts.
//!
//! This is a self-contained copy of the artifact graph logic from
//! `app/backend/src-tauri/src/domain/artifact_graph.rs`, adapted for
//! standalone use without Tauri dependencies.

use std::collections::HashMap;
use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::error::McpError;
use crate::settings::{ArtifactEntry, DeliveryConfig, ProjectSettings};

// ---------------------------------------------------------------------------
// Domain types
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
    /// Frontmatter `id` field (e.g. "EPIC-048").
    pub id: String,
    /// Source project name in organisation mode, or `None` for single-project mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Relative path from the project root (e.g. ".orqa/delivery/epics/EPIC-048.md").
    pub path: String,
    /// Inferred category string (e.g. "epic", "task", "milestone", "idea", "decision").
    pub artifact_type: String,
    /// Frontmatter `title` field, or a humanized fallback from the filename.
    pub title: String,
    /// Frontmatter `description` field.
    pub description: Option<String>,
    /// Frontmatter `status` field.
    pub status: Option<String>,
    /// Frontmatter `priority` field (e.g. "P1", "P2", "P3").
    pub priority: Option<String>,
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
    /// Semantic relationship type (e.g. "enforced-by", "grounded-by", "delivers").
    pub relationship_type: Option<String>,
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
// Type registry
// ---------------------------------------------------------------------------

/// A mapping from directory path segments to artifact type keys.
pub type TypeRegistry = Vec<(String, String)>;

fn build_type_registry(settings: &ProjectSettings) -> TypeRegistry {
    let mut registry = Vec::new();
    for entry in &settings.artifacts {
        match entry {
            ArtifactEntry::Group { children, .. } => {
                for child in children {
                    registry.push((child.path.replace('\\', "/"), child.key.clone()));
                }
            }
            ArtifactEntry::Type(type_config) => {
                registry.push((type_config.path.replace('\\', "/"), type_config.key.clone()));
            }
        }
    }
    registry
}

/// Load project settings from `{project_path}/.orqa/project.json`.
///
/// Returns `None` if the file does not exist.
fn load_settings(project_path: &Path) -> Option<ProjectSettings> {
    let settings_file = project_path.join(".orqa/project.json");
    if !settings_file.exists() {
        return None;
    }
    let contents = std::fs::read_to_string(&settings_file).ok()?;
    serde_json::from_str(&contents).ok()
}

// ---------------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------------

/// Build an `ArtifactGraph` by scanning all `.md` files under the project's `.orqa/` directory.
///
/// Two-pass algorithm:
/// 1. Walk every `.md` file, parse frontmatter, collect nodes and forward refs.
/// 2. Invert every forward ref into a backlink on the target node.
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, McpError> {
    let orqa_dir = project_path.join(".orqa");

    let settings = load_settings(project_path);
    let type_registry = settings
        .as_ref()
        .map(build_type_registry)
        .unwrap_or_default();

    let mut graph = ArtifactGraph::default();

    // Pass 1a: walk the project's own .orqa/ with project: None.
    walk_directory(&orqa_dir, project_path, &mut graph, &type_registry, None)?;

    // Pass 1b: if organisation mode, scan each child project.
    if let Some(ref settings) = settings {
        if settings.organisation {
            for child in &settings.projects {
                let child_path = if Path::new(&child.path).is_absolute() {
                    std::path::PathBuf::from(&child.path)
                } else {
                    project_path.join(&child.path)
                };
                let child_path = child_path.canonicalize().unwrap_or(child_path);
                let child_orqa = child_path.join(".orqa");
                if child_orqa.exists() {
                    let child_settings = load_settings(&child_path);
                    let child_registry = child_settings
                        .as_ref()
                        .map(build_type_registry)
                        .unwrap_or_default();
                    walk_directory(
                        &child_orqa,
                        &child_path,
                        &mut graph,
                        &child_registry,
                        Some(&child.name),
                    )?;
                    qualify_intra_project_refs(&mut graph, &child.name);
                }
            }
        }
    }

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
    }

    Ok(graph)
}

/// Qualify intra-project relationship targets for a child project.
fn qualify_intra_project_refs(graph: &mut ArtifactGraph, project_name: &str) {
    let prefix = format!("{project_name}::");
    let all_keys: std::collections::HashSet<String> = graph.nodes.keys().cloned().collect();

    let child_keys: Vec<String> = all_keys
        .iter()
        .filter(|k| k.starts_with(&prefix))
        .cloned()
        .collect();

    for key in child_keys {
        if let Some(node) = graph.nodes.get_mut(&key) {
            for ref_entry in &mut node.references_out {
                if !ref_entry.target_id.contains("::") {
                    let qualified = format!("{project_name}::{}", ref_entry.target_id);
                    if all_keys.contains(&qualified) {
                        ref_entry.target_id = qualified;
                    }
                }
            }
        }
    }
}

/// Recursively walk a directory, collecting `ArtifactNode` entries into `graph`.
fn walk_directory(
    dir: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
    type_registry: &TypeRegistry,
    project_name: Option<&str>,
) -> Result<(), McpError> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(());
    };

    for entry in entries {
        let entry = entry.map_err(|e| McpError::FileSystem(e.to_string()))?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        let ft = entry
            .file_type()
            .map_err(|e| McpError::FileSystem(e.to_string()))?;

        if ft.is_dir() {
            walk_directory(
                &entry.path(),
                project_root,
                graph,
                type_registry,
                project_name,
            )?;
        } else if ft.is_file() && name.ends_with(".md") {
            if name.eq_ignore_ascii_case("README.md") {
                continue;
            }
            collect_node(
                &entry.path(),
                project_root,
                graph,
                type_registry,
                project_name,
            )?;
        }
    }

    Ok(())
}

/// Parse a single `.md` file and add an `ArtifactNode` to the graph if it has a YAML `id` field.
fn collect_node(
    file_path: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
    type_registry: &TypeRegistry,
    project_name: Option<&str>,
) -> Result<(), McpError> {
    let content =
        std::fs::read_to_string(file_path).map_err(|e| McpError::FileSystem(e.to_string()))?;
    let (fm_text, body) = extract_frontmatter(&content);
    let Some(fm_text) = fm_text else {
        return Ok(());
    };
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&fm_text).unwrap_or(serde_yaml::Value::Null);
    let id = match yaml_value.get("id").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.to_owned(),
        _ => return Ok(()),
    };
    let rel_path = file_path
        .strip_prefix(project_root)
        .unwrap_or(file_path)
        .to_string_lossy()
        .replace('\\', "/");
    let node = build_node(
        id.clone(),
        rel_path.clone(),
        file_path,
        &yaml_value,
        &body,
        type_registry,
        project_name,
    );

    let graph_key = match project_name {
        Some(proj) => format!("{proj}::{id}"),
        None => id.clone(),
    };
    let path_key = match project_name {
        Some(proj) => format!("{proj}::{rel_path}"),
        None => rel_path,
    };

    graph.nodes.insert(graph_key, node);
    graph.path_index.insert(path_key, id);
    Ok(())
}

fn build_node(
    id: String,
    rel_path: String,
    file_path: &Path,
    yaml_value: &serde_yaml::Value,
    body: &str,
    type_registry: &TypeRegistry,
    project_name: Option<&str>,
) -> ArtifactNode {
    let title = yaml_value
        .get("title")
        .and_then(|v| v.as_str())
        .map_or_else(|| humanize_stem(file_path), str::to_owned);
    let description = yaml_value
        .get("description")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let status = yaml_value
        .get("status")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let priority = yaml_value
        .get("priority")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let artifact_type = infer_artifact_type(&rel_path, type_registry);
    let frontmatter = yaml_to_json(yaml_value);
    let mut references_out = collect_forward_refs(yaml_value, &id);
    references_out.extend(collect_body_refs(body, &id));
    ArtifactNode {
        id,
        project: project_name.map(str::to_owned),
        path: rel_path,
        artifact_type,
        title,
        description,
        status,
        priority,
        frontmatter,
        references_out,
        references_in: Vec::new(),
    }
}

fn collect_forward_refs(yaml_value: &serde_yaml::Value, source_id: &str) -> Vec<ArtifactRef> {
    collect_relationship_refs(yaml_value, source_id)
}

fn collect_relationship_refs(yaml_value: &serde_yaml::Value, source_id: &str) -> Vec<ArtifactRef> {
    let Some(seq) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    else {
        return Vec::new();
    };
    let mut refs = Vec::new();
    for item in seq {
        let target = item
            .get("target")
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_owned());
        let rel_type = item
            .get("type")
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_owned());
        if let Some(target_id) = target {
            if !target_id.is_empty() {
                refs.push(ArtifactRef {
                    target_id,
                    field: "relationships".to_owned(),
                    source_id: source_id.to_owned(),
                    relationship_type: rel_type,
                });
            }
        }
    }
    refs
}

fn collect_body_refs(body: &str, source_id: &str) -> Vec<ArtifactRef> {
    thread_local! {
        static BODY_REF_RE: Regex =
            Regex::new(r"\[([^\]]*)\]\(([A-Z]+-\d+)\)").expect("body ref regex is valid");
    }

    let mut refs = Vec::new();
    let mut seen = std::collections::HashSet::new();

    BODY_REF_RE.with(|re| {
        for cap in re.captures_iter(body) {
            let target_id = cap[2].to_owned();
            if target_id == source_id || !seen.insert(target_id.clone()) {
                continue;
            }
            refs.push(ArtifactRef {
                target_id,
                field: "body".to_owned(),
                source_id: source_id.to_owned(),
                relationship_type: None,
            });
        }
    });

    refs
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

/// Compute summary statistics for the graph.
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats {
    let node_count = graph.nodes.len();
    let edge_count: usize = graph.nodes.values().map(|n| n.references_out.len()).sum();
    let orphan_count = graph
        .nodes
        .values()
        .filter(|n| {
            n.artifact_type != "doc" && n.references_out.is_empty() && n.references_in.is_empty()
        })
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
// Integrity checks
// ---------------------------------------------------------------------------

/// Category of integrity issue found in the artifact graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrityCategory {
    BrokenLink,
    MissingInverse,
    TypeConstraintViolation,
    RequiredRelationshipMissing,
    CardinalityViolation,
    CircularDependency,
    InvalidStatus,
    BodyTextRefWithoutRelationship,
    ParentChildInconsistency,
    DeliveryPathMismatch,
}

/// Severity of an integrity finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegritySeverity {
    Error,
    Warning,
}

/// A single integrity finding from the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCheck {
    pub category: IntegrityCategory,
    pub severity: IntegritySeverity,
    pub artifact_id: String,
    pub message: String,
    pub auto_fixable: bool,
    pub fix_description: Option<String>,
}

/// Run integrity checks on the artifact graph, using the platform's embedded relationship schema.
///
/// Passes empty valid_statuses and project_relationships (headless mode — no project settings loaded).
pub fn check_integrity_headless(graph: &ArtifactGraph) -> Vec<IntegrityCheck> {
    crate::integrity::run_checks(graph, &[], &DeliveryConfig::default(), &[])
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract YAML frontmatter and body from a markdown string.
///
/// Returns `(Some(frontmatter_text), body)` if the file starts with `---`,
/// or `(None, full_content)` if no frontmatter block is found.
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

fn infer_artifact_type(rel_path: &str, type_registry: &TypeRegistry) -> String {
    let normalized = rel_path.replace('\\', "/");

    for (path_prefix, type_key) in type_registry {
        if normalized.starts_with(path_prefix) {
            return type_key.clone();
        }
    }

    if normalized.contains("/epics/") {
        "epic"
    } else if normalized.contains("/tasks/") {
        "task"
    } else if normalized.contains("/milestones/") {
        "milestone"
    } else if normalized.contains("/ideas/") {
        "idea"
    } else if normalized.contains("/decisions/") {
        "decision"
    } else if normalized.contains("/research/") {
        "research"
    } else if normalized.contains("/lessons/") {
        "lesson"
    } else if normalized.contains("/rules/") {
        "rule"
    } else if normalized.contains("/agents/") {
        "agent"
    } else if normalized.contains("/knowledge/") {
        "knowledge"
    } else if normalized.contains("/hooks/") {
        "hook"
    } else if normalized.contains("/pillars/") {
        "pillar"
    } else {
        "doc"
    }
    .to_owned()
}

fn yaml_to_json(value: &serde_yaml::Value) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

fn humanize_stem(file_path: &Path) -> String {
    let stem = file_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

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
                    s.extend(chars.flat_map(char::to_lowercase));
                    s
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

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
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\ntitle: No ID\n---\n# Body\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn file_with_id_creates_node() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
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
    fn relationship_creates_forward_ref() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nrelationships:\n  - target: EPIC-001\n    type: delivers\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let node = graph.nodes.get("TASK-001").expect("node");
        assert_eq!(node.references_out.len(), 1);
        assert_eq!(node.references_out[0].target_id, "EPIC-001");
        assert_eq!(node.references_out[0].field, "relationships");
    }

    #[test]
    fn backlinks_are_computed_in_pass_two() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\n---\n",
        );
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nrelationships:\n  - target: EPIC-001\n    type: delivers\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let epic = graph.nodes.get("EPIC-001").expect("epic node");
        assert_eq!(epic.references_in.len(), 1);
        assert_eq!(epic.references_in[0].source_id, "TASK-001");
    }

    #[test]
    fn broken_refs_counted_in_stats() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Task\nrelationships:\n  - target: EPIC-MISSING\n    type: delivers\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.broken_ref_count, 1);
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.edge_count, 1);
    }

    #[test]
    fn readme_files_are_skipped() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "README.md",
            "---\nid: SHOULD-SKIP\ntitle: Nav\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn extract_frontmatter_parses_correctly() {
        let content = "---\nid: EPIC-001\ntitle: Test\n---\n# Body content";
        let (fm, body) = extract_frontmatter(content);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("id: EPIC-001"));
        assert!(body.contains("# Body content"));
    }

    #[test]
    fn extract_frontmatter_returns_none_without_delimiters() {
        let content = "# Just a markdown file\nNo frontmatter";
        let (fm, body) = extract_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(body, content);
    }
}
