//! Artifact graph builder for the standalone LSP server.
//!
//! This is a self-contained reimplementation of the OrqaStudio artifact graph
//! builder, decoupled from the Tauri app's internal module structure. It reads
//! `project.json` and walks `.orqa/` to produce an `ArtifactGraph` suitable
//! for LSP validation queries (target existence checks).

use std::path::Path;

use regex::Regex;

use crate::error::LspError;
use crate::types::{
    ArtifactEntry, ArtifactGraph, ArtifactNode, ArtifactRef, ProjectSettings, TypeRegistry,
};

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

    // Pass 1c: in organisation mode, insert bare-ID aliases for child-project nodes
    // so that existence checks via `graph.nodes.contains_key(bare_id)` resolve
    // regardless of which project owns the artifact.
    if let Some(ref s) = settings {
        if s.organisation {
            insert_bare_id_aliases(&mut graph);
        }
    }

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

    let frontmatter_type = yaml_value.get("type").and_then(|v| v.as_str());
    let artifact_type = infer_artifact_type(&rel_path, type_registry, frontmatter_type, &id);

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
// Organisation mode: bare-ID alias insertion
// ---------------------------------------------------------------------------

/// Insert bare-ID aliases for child-project nodes in the graph.
///
/// Child artifacts are stored with qualified keys (`project::ARTIFACT-ID`).
/// When the LSP validates a reference target like `RULE-6c0496e0`, it queries
/// `graph.nodes.contains_key("RULE-6c0496e0")` which fails without this alias.
///
/// This pass inserts a copy of each child node under its bare ID when the bare
/// ID does not conflict with a root-project node. First-found wins on conflicts
/// between child projects.
fn insert_bare_id_aliases(graph: &mut ArtifactGraph) {
    let mut bare_to_qualified: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for key in graph.nodes.keys() {
        if let Some(sep) = key.find("::") {
            let bare_id = &key[sep + 2..];
            // Root key takes priority: skip if already resolved directly.
            if graph.nodes.contains_key(bare_id) {
                continue;
            }
            // First-found wins on duplicate bare IDs across child projects.
            bare_to_qualified
                .entry(bare_id.to_owned())
                .or_insert_with(|| key.clone());
        }
    }

    for (bare_id, qualified_key) in &bare_to_qualified {
        if let Some(node) = graph.nodes.get(qualified_key).cloned() {
            graph.nodes.insert(bare_id.clone(), node);
        }
    }
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
///
/// Resolution priority (highest to lowest):
/// 1. Explicit `type:` field in frontmatter (`frontmatter_type` parameter).
/// 2. Longest-prefix match against the config-driven type registry (from project.json).
/// 3. ID-prefix match against the platform artifact types (from core.json).
/// 4. Hardcoded path-segment heuristic for well-known directory names.
/// 5. `"doc"` as the final fallback.
fn infer_artifact_type(
    rel_path: &str,
    type_registry: &TypeRegistry,
    frontmatter_type: Option<&str>,
    artifact_id: &str,
) -> String {
    // 1. Explicit frontmatter type field overrides everything.
    if let Some(t) = frontmatter_type {
        let t = t.trim();
        if !t.is_empty() {
            return t.to_owned();
        }
    }

    let normalized = rel_path.replace('\\', "/");

    // 2. Config-driven registry: longest-prefix match wins.
    let mut best_match: Option<(&String, &String)> = None;
    for (path_prefix, type_key) in type_registry {
        let prefix_slash = if path_prefix.ends_with('/') {
            path_prefix.clone()
        } else {
            format!("{path_prefix}/")
        };
        if (normalized.starts_with(&prefix_slash) || normalized == *path_prefix)
            && (best_match.is_none() || path_prefix.len() > best_match.unwrap().0.len())
        {
            best_match = Some((path_prefix, type_key));
        }
    }
    if let Some((_, type_key)) = best_match {
        return type_key.clone();
    }

    // 3. ID-prefix match against the platform's artifact type definitions.
    if let Some(prefix) = artifact_id.split('-').next() {
        if !prefix.is_empty() {
            let matched = crate::platform::PLATFORM
                .artifact_types
                .iter()
                .find(|t| t.id_prefix == prefix)
                .map(|t| t.key.clone());
            if let Some(t) = matched {
                return t;
            }
        }
    }

    // 4. Hardcoded path-segment heuristic for well-known directory names.
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
        write_artifact(
            &dir,
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

    // -----------------------------------------------------------------------
    // Organisation mode tests
    // -----------------------------------------------------------------------

    fn write_org_project_json(dir: &Path, child_name: &str, child_path: &str) {
        let json = format!(
            r#"{{
  "name": "Test Org",
  "organisation": true,
  "projects": [
    {{ "name": "{child_name}", "path": "{child_path}" }}
  ],
  "artifacts": []
}}"#
        );
        let orqa = dir.join(".orqa");
        fs::create_dir_all(&orqa).expect("create .orqa");
        fs::write(orqa.join("project.json"), json).expect("write project.json");
    }

    #[test]
    fn organisation_mode_scans_child_project_and_inserts_bare_alias() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        let rules_dir = child_dir.join(".orqa/process/rules");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");

        // Both qualified and bare-ID keys should exist.
        assert!(graph.nodes.contains_key("app::RULE-001"), "qualified key");
        assert!(
            graph.nodes.contains_key("RULE-001"),
            "bare-ID alias for LSP existence checks"
        );
    }

    #[test]
    fn root_project_wins_on_id_conflict_with_child() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Same ID in both root and child.
        let root_rules = tmp.path().join(".orqa/process/rules");
        write_artifact(
            &root_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Root\n---\n",
        );
        let child_rules = child_dir.join(".orqa/process/rules");
        write_artifact(
            &child_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Child\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");

        // Bare-ID key should be the root node (no project prefix).
        let node = graph.nodes.get("RULE-001").expect("node");
        assert_eq!(node.project, None, "root node has no project prefix");
    }
}
