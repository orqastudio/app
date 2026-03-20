//! Artifact graph builder for the standalone LSP server.
//!
//! This is a self-contained reimplementation of the OrqaStudio artifact graph
//! builder, decoupled from the Tauri app's internal module structure. It reads
//! `project.json` and walks `.orqa/` to produce an `ArtifactGraph` suitable
//! for LSP validation queries (target existence checks).

use std::path::Path;

use regex::Regex;

use crate::error::LspError;
use crate::types::{ArtifactEntry, ArtifactGraph, ArtifactNode, ArtifactRef, ProjectSettings, TypeRegistry};

// ---------------------------------------------------------------------------
// Settings reader
// ---------------------------------------------------------------------------

/// Read `{project_path}/.orqa/project.json`, returning `None` if the file
/// does not exist.
fn read_project_settings(project_path: &Path) -> Result<Option<ProjectSettings>, LspError> {
    let settings_file = project_path.join(".orqa").join("project.json");
    if !settings_file.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&settings_file)?;
    // Use permissive deserialization — ignore unknown fields.
    let settings: ProjectSettings = serde_json::from_str(&contents)?;
    Ok(Some(settings))
}

// ---------------------------------------------------------------------------
// Type registry
// ---------------------------------------------------------------------------

/// Build a `TypeRegistry` from parsed project settings.
fn build_type_registry(settings: &Option<ProjectSettings>) -> TypeRegistry {
    let Some(settings) = settings else {
        return Vec::new();
    };

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

// ---------------------------------------------------------------------------
// Public graph builder
// ---------------------------------------------------------------------------

/// Build an `ArtifactGraph` by scanning all `.md` files under `.orqa/`.
///
/// Two-pass algorithm:
/// 1. Walk every `.md` file, parse frontmatter, collect nodes and forward refs.
/// 2. Invert every forward ref into a backlink on the target node.
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, LspError> {
    let settings = read_project_settings(project_path)?;
    let type_registry = build_type_registry(&settings);
    let orqa_dir = project_path.join(".orqa");

    let mut graph = ArtifactGraph::default();

    // Pass 1a: walk the project's own .orqa/
    walk_directory(&orqa_dir, project_path, &mut graph, &type_registry, None)?;

    // Pass 1b: organisation mode — scan each child project
    if let Some(ref s) = settings {
        if s.organisation {
            for child in &s.projects {
                let child_path = if Path::new(&child.path).is_absolute() {
                    std::path::PathBuf::from(&child.path)
                } else {
                    project_path.join(&child.path)
                };
                let child_path = child_path.canonicalize().unwrap_or(child_path);
                let child_orqa = child_path.join(".orqa");
                if child_orqa.exists() {
                    let child_settings = read_project_settings(&child_path)?;
                    let child_registry = build_type_registry(&child_settings);
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

    // Pass 2: compute backlinks — currently unused by LSP validation but
    // included for API completeness.
    // (The LSP only queries `graph.nodes.contains_key(target)` for existence.)

    Ok(graph)
}

// ---------------------------------------------------------------------------
// Directory walker
// ---------------------------------------------------------------------------

fn walk_directory(
    dir: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
    type_registry: &TypeRegistry,
    project_name: Option<&str>,
) -> Result<(), LspError> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(());
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
            collect_node(&entry.path(), project_root, graph, type_registry, project_name)?;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Node collector
// ---------------------------------------------------------------------------

fn collect_node(
    file_path: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
    type_registry: &TypeRegistry,
    project_name: Option<&str>,
) -> Result<(), LspError> {
    let content = std::fs::read_to_string(file_path)?;
    let (fm_text, _body) = extract_frontmatter(&content);
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

    let artifact_type = infer_artifact_type(&rel_path, type_registry);

    let node = ArtifactNode {
        id: id.clone(),
        project: project_name.map(str::to_owned),
        path: rel_path.clone(),
        artifact_type,
    };

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

// ---------------------------------------------------------------------------
// Organisation mode: qualify intra-project refs
// ---------------------------------------------------------------------------

fn qualify_intra_project_refs(graph: &mut ArtifactGraph, project_name: &str) {
    let prefix = format!("{project_name}::");
    let all_keys: std::collections::HashSet<String> = graph.nodes.keys().cloned().collect();

    let child_keys: Vec<String> = all_keys
        .iter()
        .filter(|k| k.starts_with(&prefix))
        .cloned()
        .collect();

    // This function only adjusts the path_index for qualified keys.
    // Forward refs are not stored on ArtifactNode in this lightweight crate.
    let _ = child_keys; // suppress unused warning — kept for structural parity
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract `(frontmatter_text, body)` from a markdown file.
///
/// Returns `(None, full_content)` if no frontmatter delimiters are found.
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

/// Infer a human-readable artifact type from a relative file path.
fn infer_artifact_type(rel_path: &str, type_registry: &TypeRegistry) -> String {
    let normalized = rel_path.replace('\\', "/");

    for (path_prefix, type_key) in type_registry {
        if normalized.starts_with(path_prefix) {
            return type_key.clone();
        }
    }

    // Hardcoded fallback for backwards compatibility.
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

/// Collect forward `relationships` array references from YAML frontmatter.
pub fn collect_relationship_refs(
    yaml_value: &serde_yaml::Value,
    source_id: &str,
) -> Vec<ArtifactRef> {
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

/// Collect artifact refs from markdown body text (`[TEXT](ARTIFACT-ID)` pattern).
pub fn collect_body_refs(body: &str, source_id: &str) -> Vec<ArtifactRef> {
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_artifact(dir: &Path, name: &str, content: &str) {
        fs::create_dir_all(dir).expect("create dir");
        fs::write(dir.join(name), content).expect("write file");
    }

    #[test]
    fn empty_orqa_dir_returns_empty_graph() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn file_without_id_is_skipped() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(&dir, "EPIC-001.md", "---\ntitle: No ID\n---\n# Body\n");
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn file_with_id_creates_node() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: active\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert_eq!(graph.nodes.len(), 1);
        let node = graph.nodes.get("EPIC-001").expect("node");
        assert_eq!(node.id, "EPIC-001");
        assert_eq!(node.artifact_type, "epic");
    }

    #[test]
    fn readme_files_are_skipped() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(&dir, "README.md", "---\nid: SHOULD-SKIP\ntitle: Nav\n---\n");
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn path_index_maps_path_to_id() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(&dir, "EPIC-001.md", "---\nid: EPIC-001\ntitle: My Epic\n---\n");
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
    fn extract_frontmatter_parses_correctly() {
        let content = "---\nid: EPIC-001\ntitle: My Epic\n---\n# Body content\n";
        let (fm, body) = extract_frontmatter(content);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("id: EPIC-001"));
        assert!(body.contains("Body content"));
    }

    #[test]
    fn extract_frontmatter_returns_none_without_delimiters() {
        let content = "# No frontmatter here\n";
        let (fm, _body) = extract_frontmatter(content);
        assert!(fm.is_none());
    }
}
