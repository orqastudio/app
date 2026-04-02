//! Artifact graph construction — scanning `.orqa/` and building the bidirectional graph.
//!
//! Implements the two-pass scan algorithm:
//! 1. Walk every `.md` file under `.orqa/`, parse YAML frontmatter, collect nodes and forward refs.
//! 2. Invert every forward ref into a backlink on the target node.
//!
//! Also provides `extract_frontmatter` (shared with engine/validation for file rewriting),
//! `graph_stats`, and `load_project_config` (project settings loader).

use std::collections::HashMap;
use std::path::Path;

use regex::Regex;

use orqa_engine_types::config::{
    ArtifactEntry, DeliveryConfig, ProjectRelationshipConfig, ProjectSettings,
};
use orqa_engine_types::platform::ArtifactTypeDef;
use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats};

use crate::error::GraphError;

/// A mapping from directory path segments to artifact type keys.
///
/// Built from `project.json` artifact configuration. Used by `infer_artifact_type`
/// to resolve artifact types from file paths.
pub type TypeRegistry = Vec<(String, String)>;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Build an `ArtifactGraph` by scanning all `.md` files under the project's `.orqa/` directory.
///
/// Two-pass algorithm:
/// 1. Walk every `.md` file, parse frontmatter, collect nodes and forward refs.
/// 2. Invert every forward ref into a backlink on the target node.
///
/// Invalid relationship types (not declared in project.json or plugin manifests) are excluded
/// from the graph and logged as warnings. They don't represent valid knowledge flow.
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, GraphError> {
    use std::time::Instant;
    let start = Instant::now();

    let settings = load_settings(project_path);
    let type_registry = settings
        .as_ref()
        .map(build_type_registry)
        .unwrap_or_default();
    let valid_rel_types = build_valid_relationship_types(project_path);
    let org_mode = settings.as_ref().is_some_and(|s| s.organisation);

    let mut graph = ArtifactGraph::default();

    // Pass 1a: walk the project's own .orqa/.
    walk_directory(
        &project_path.join(".orqa"),
        project_path,
        &mut graph,
        &type_registry,
        None,
        &valid_rel_types,
    )?;

    // Pass 1b: if organisation mode, scan each child project.
    if org_mode {
        if let Some(ref s) = settings {
            scan_child_projects(s, project_path, &mut graph, &valid_rel_types)?;
        }
    }

    // Pass 1c: rewrite cross-project target IDs before computing backlinks.
    if org_mode {
        rewrite_cross_project_refs(&mut graph);
    }

    // Pass 2: invert references — add backlinks to target nodes.
    invert_references(&mut graph);

    // Pass 3: insert bare-ID aliases after backlinks are computed.
    if org_mode {
        insert_bare_id_aliases(&mut graph);
    }

    tracing::info!(
        subsystem = "engine",
        elapsed_ms = start.elapsed().as_millis() as u64,
        node_count = graph.nodes.len(),
        "[engine] build_artifact_graph completed"
    );

    Ok(graph)
}

/// Compute summary statistics for the graph.
///
/// In organisation mode, bare-ID alias nodes are excluded from counts to avoid
/// double-counting. An alias node is identified by its graph key equalling its
/// `id` while also having a `project` field.
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats {
    let primary_nodes: Vec<&ArtifactNode> = graph
        .nodes
        .iter()
        .filter(|(key, node)| !(key.as_str() == node.id && node.project.is_some()))
        .map(|(_, node)| node)
        .collect();

    let node_count = primary_nodes.len();
    let edge_count: usize = primary_nodes.iter().map(|n| n.references_out.len()).sum();
    let orphan_count = primary_nodes
        .iter()
        .filter(|n| {
            n.artifact_type != "doc" && n.references_out.is_empty() && n.references_in.is_empty()
        })
        .count();
    let broken_ref_count: usize = primary_nodes
        .iter()
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

/// Load project settings from a project path, returning statuses, delivery config, and
/// relationship configs on success, or empty defaults on failure.
pub fn load_project_config(
    project_path: &Path,
) -> (Vec<String>, DeliveryConfig, Vec<ProjectRelationshipConfig>) {
    match load_settings(project_path) {
        Some(settings) => {
            let statuses = settings.statuses.iter().map(|s| s.key.clone()).collect();
            (statuses, settings.delivery, settings.relationships)
        }
        None => (Vec::new(), DeliveryConfig::default(), Vec::new()),
    }
}

/// Extract YAML frontmatter and body from a markdown string.
///
/// Returns `(Some(yaml_text), body)` where `yaml_text` is the YAML content
/// between `---` markers (without the markers themselves, trimmed of leading/trailing
/// newlines). `body` is everything after the closing `---` marker.
///
/// # Contract for callers that rewrite files
///
/// To reconstruct the file: `format!("---\n{yaml_text}\n---\n{body}")`
/// This produces a clean file with `---` on its own line, YAML content, `---`,
/// then body content.
pub fn extract_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_owned());
    }

    let after_open = &trimmed[3..];
    let Some(close_pos) = after_open.find("\n---") else {
        return (None, content.to_owned());
    };

    // Trim leading/trailing whitespace from the YAML block so callers
    // can reliably reconstruct with `format!("---\n{fm}\n---\n{body}")`.
    let fm_text = after_open[..close_pos].trim().to_owned();
    let body = after_open[close_pos + 4..]
        .trim_start_matches('\n')
        .to_owned();
    (Some(fm_text), body)
}

/// Infer a human-readable artifact type category from a relative file path.
///
/// Resolution priority (highest to lowest):
/// 1. Explicit `type:` field in frontmatter.
/// 2. Longest-prefix match against the config-driven type registry.
/// 3. ID-prefix match against `plugin_types` (caller-supplied plugin contributions).
/// 4. Hardcoded path-segment heuristic for well-known directory names.
/// 5. `"doc"` as the final fallback.
pub fn infer_artifact_type(
    rel_path: &str,
    type_registry: &TypeRegistry,
    frontmatter_type: Option<&str>,
    artifact_id: &str,
    plugin_types: &[ArtifactTypeDef],
) -> String {
    if let Some(t) = frontmatter_type.map(str::trim).filter(|t| !t.is_empty()) {
        return t.to_owned();
    }

    let normalized = rel_path.replace('\\', "/");

    let mut best_match: Option<(&String, &String)> = None;
    for (path_prefix, type_key) in type_registry {
        let prefix_slash = if path_prefix.ends_with('/') {
            path_prefix.clone()
        } else {
            format!("{path_prefix}/")
        };
        if (normalized.starts_with(&prefix_slash) || normalized == *path_prefix)
            && best_match.is_none_or(|(prev, _)| path_prefix.len() > prev.len())
        {
            best_match = Some((path_prefix, type_key));
        }
    }
    if let Some((_, type_key)) = best_match {
        return type_key.clone();
    }

    if let Some(prefix) = artifact_id.split('-').next().filter(|p| !p.is_empty()) {
        let matched = plugin_types
            .iter()
            .find(|t| t.id_prefix == prefix)
            .map(|t| t.key.clone());
        if let Some(t) = matched {
            return t;
        }
    }

    type_from_path_heuristic(&normalized)
}

// ---------------------------------------------------------------------------
// Internal: settings loading
// ---------------------------------------------------------------------------

/// Load project settings from `{project_path}/.orqa/project.json`.
///
/// Returns `None` if the file does not exist or fails to parse.
fn load_settings(project_path: &Path) -> Option<ProjectSettings> {
    let settings_file = project_path.join(".orqa/project.json");
    if !settings_file.exists() {
        return None;
    }
    let contents = std::fs::read_to_string(&settings_file).ok()?;
    serde_json::from_str(&contents).ok()
}

/// Build a `TypeRegistry` from project settings artifact entries.
///
/// Maps directory path segments to artifact type keys, enabling path-based type inference.
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

/// Build a set of valid relationship type keys from core.json + plugin manifests.
///
/// Reads from project.json relationships and from plugin/connector manifests under
/// `project_path`. Used during graph construction to filter invalid relationship types.
pub fn build_valid_relationship_types(
    project_path: &Path,
) -> std::collections::HashSet<String> {
    let mut valid = std::collections::HashSet::new();

    // Project-level relationships (from project.json).
    if let Some(settings) = load_settings(project_path) {
        for rel in &settings.relationships {
            valid.insert(rel.key.clone());
            valid.insert(rel.inverse.clone());
        }
    }

    // Plugin-provided relationships — scan plugin and connector manifests.
    for dir_name in &["plugins", "connectors"] {
        let scan_dir = project_path.join(dir_name);
        if !scan_dir.exists() {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&scan_dir) else {
            continue;
        };
        for entry in entries.flatten() {
            if !entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                continue;
            }
            let manifest = entry.path().join("orqa-plugin.json");
            if !manifest.exists() {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&manifest) else {
                continue;
            };
            let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
                continue;
            };
            if let Some(rels) = json
                .pointer("/provides/relationships")
                .and_then(|v| v.as_array())
            {
                for rel in rels {
                    if let Some(key) = rel.get("key").and_then(|v| v.as_str()) {
                        valid.insert(key.to_owned());
                    }
                    if let Some(inv) = rel.get("inverse").and_then(|v| v.as_str()) {
                        valid.insert(inv.to_owned());
                    }
                }
            }
        }
    }

    valid
}

// ---------------------------------------------------------------------------
// Internal: graph construction helpers
// ---------------------------------------------------------------------------

/// Walk all child projects in organisation mode and add their nodes to the graph.
///
/// Resolves each child path relative to `project_path` and qualifies intra-project refs.
fn scan_child_projects(
    settings: &ProjectSettings,
    project_path: &Path,
    graph: &mut ArtifactGraph,
    valid_rel_types: &std::collections::HashSet<String>,
) -> Result<(), GraphError> {
    for child in &settings.projects {
        let child_path = resolve_child_path(project_path, &child.path);
        let child_orqa = child_path.join(".orqa");
        if !child_orqa.exists() {
            continue;
        }
        let child_registry = load_settings(&child_path)
            .as_ref()
            .map(build_type_registry)
            .unwrap_or_default();
        walk_directory(
            &child_orqa,
            &child_path,
            graph,
            &child_registry,
            Some(&child.name),
            valid_rel_types,
        )?;
        qualify_intra_project_refs(graph, &child.name);
    }
    Ok(())
}

/// Resolve a child project path: absolute paths are used as-is; relative paths are joined to `base`.
fn resolve_child_path(base: &Path, child_path_str: &str) -> std::path::PathBuf {
    let raw = if Path::new(child_path_str).is_absolute() {
        std::path::PathBuf::from(child_path_str)
    } else {
        base.join(child_path_str)
    };
    raw.canonicalize().unwrap_or(raw)
}

/// Compute backlinks (Pass 2): for every forward reference, add a backlink on the target node.
fn invert_references(graph: &mut ArtifactGraph) {
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
}

/// Build a bare-ID → qualified-graph-key index for all child-project nodes.
fn build_child_id_index(graph: &ArtifactGraph) -> HashMap<String, String> {
    let mut bare_to_qualified: HashMap<String, String> = HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();

    for key in graph.nodes.keys() {
        if let Some(sep) = key.find("::") {
            let bare_id = &key[sep + 2..];
            if graph.nodes.contains_key(bare_id) {
                continue;
            }
            if bare_to_qualified.contains_key(bare_id) {
                duplicates.push(bare_id.to_owned());
            } else {
                bare_to_qualified.insert(bare_id.to_owned(), key.clone());
            }
        }
    }

    for dup in &duplicates {
        tracing::warn!(
            "artifact ID '{}' exists in multiple child projects; first-found wins for ref resolution",
            dup
        );
    }

    bare_to_qualified
}

/// Rewrite unresolvable bare-ID `target_id` values in `references_out` to their qualified equivalents.
fn rewrite_cross_project_refs(graph: &mut ArtifactGraph) {
    let bare_to_qualified = build_child_id_index(graph);
    let all_keys: std::collections::HashSet<String> = graph.nodes.keys().cloned().collect();

    for node in graph.nodes.values_mut() {
        for ref_entry in &mut node.references_out {
            if !all_keys.contains(&ref_entry.target_id) && !ref_entry.target_id.contains("::") {
                if let Some(qualified) = bare_to_qualified.get(&ref_entry.target_id) {
                    ref_entry.target_id = qualified.clone();
                }
            }
        }
    }
}

/// Insert bare-ID aliases for child-project nodes.
fn insert_bare_id_aliases(graph: &mut ArtifactGraph) {
    let bare_to_qualified = build_child_id_index(graph);

    for (bare_id, qualified_key) in &bare_to_qualified {
        if let Some(node) = graph.nodes.get(qualified_key).cloned() {
            graph.nodes.insert(bare_id.clone(), node);
        }
    }
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
    valid_rel_types: &std::collections::HashSet<String>,
) -> Result<(), GraphError> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            tracing::warn!(path = %dir.display(), error = %e, "[engine] failed to read directory");
            return Ok(());
        }
    };

    for entry in entries {
        let entry = entry.map_err(|e| GraphError::FileSystem(e.to_string()))?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        let ft = entry
            .file_type()
            .map_err(|e| GraphError::FileSystem(e.to_string()))?;

        if ft.is_dir() {
            walk_directory(
                &entry.path(),
                project_root,
                graph,
                type_registry,
                project_name,
                valid_rel_types,
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
                valid_rel_types,
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
    valid_rel_types: &std::collections::HashSet<String>,
) -> Result<(), GraphError> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| GraphError::FileSystem(e.to_string()))?;
    let (fm_text, body) = extract_frontmatter(&content);
    let Some(fm_text) = fm_text else {
        return Ok(());
    };
    let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&fm_text) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(path = %file_path.display(), error = %e, "[engine] failed to parse YAML frontmatter");
            serde_yaml::Value::Null
        }
    };
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
        &NodeBuildCtx {
            type_registry,
            project_name,
            valid_rel_types,
        },
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

/// Context shared across all nodes built during a single graph scan.
struct NodeBuildCtx<'a> {
    type_registry: &'a TypeRegistry,
    project_name: Option<&'a str>,
    valid_rel_types: &'a std::collections::HashSet<String>,
}

/// Build an `ArtifactNode` from parsed YAML frontmatter and body.
///
/// Extracts scalar fields, infers type, filters invalid relationship types, and assembles the node.
fn build_node(
    id: String,
    rel_path: String,
    file_path: &Path,
    yaml_value: &serde_yaml::Value,
    body: &str,
    ctx: &NodeBuildCtx<'_>,
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

    let frontmatter_type = yaml_value.get("type").and_then(|v| v.as_str());
    // Graph construction never has plugin types at hand; type inference relies on
    // path-based registry and frontmatter `type` field only.
    let artifact_type = infer_artifact_type(&rel_path, ctx.type_registry, frontmatter_type, &id, &[]);
    let frontmatter = yaml_to_json(yaml_value);

    let mut references_out = filter_valid_refs(
        collect_relationship_refs(yaml_value, &id),
        &id,
        ctx.valid_rel_types,
    );
    references_out.extend(collect_body_refs(body, &id));

    ArtifactNode {
        id,
        project: ctx.project_name.map(str::to_owned),
        path: rel_path,
        artifact_type,
        title,
        description,
        status,
        priority,
        frontmatter,
        body: Some(body.to_owned()),
        references_out,
        references_in: Vec::new(),
    }
}

/// Filter a list of artifact refs to only those with valid (or untyped) relationship types.
///
/// Invalid typed edges are excluded and logged as warnings since they don't represent valid knowledge flow.
fn filter_valid_refs(
    all_refs: Vec<ArtifactRef>,
    source_id: &str,
    valid_rel_types: &std::collections::HashSet<String>,
) -> Vec<ArtifactRef> {
    let mut out = Vec::new();
    for r in all_refs {
        if let Some(ref rel_type) = r.relationship_type {
            if !valid_rel_types.is_empty() && !valid_rel_types.contains(rel_type) {
                tracing::warn!(
                    artifact = %source_id,
                    relationship = %rel_type,
                    target = %r.target_id,
                    "Skipping invalid relationship type '{}' on {} — not defined in project.json or any plugin schema",
                    rel_type, source_id,
                );
                continue;
            }
        }
        out.push(r);
    }
    out
}

/// Collect relationship entries from the `relationships:` frontmatter sequence.
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

/// Collect body-text markdown link references in the form `[text](ARTIFACT-ID)`.
fn collect_body_refs(body: &str, source_id: &str) -> Vec<ArtifactRef> {
    thread_local! {
        static BODY_REF_RE: Regex =
            Regex::new(r"\[([^\]]*)\]\(([A-Z]+-[a-zA-Z0-9]+)\)").expect("body ref regex is valid");
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
// Helpers
// ---------------------------------------------------------------------------

fn type_from_path_heuristic(normalized: &str) -> String {
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
    match serde_json::to_value(value) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(error = %e, "[engine] failed to convert YAML value to JSON");
            serde_json::Value::Null
        }
    }
}

/// Convert a filename stem to a human-readable title.
///
/// All-caps stems (e.g. "EPIC-001") are returned unchanged. Hyphen-separated
/// lowercase words are title-cased (e.g. "my-epic" → "My Epic").
pub fn humanize_stem(file_path: &Path) -> String {
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
        let epics_dir = tmp.path().join(".orqa/implementation/epics");
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
        let epics_dir = tmp.path().join(".orqa/implementation/epics");
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
        let tasks_dir = tmp.path().join(".orqa/implementation/tasks");
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
        assert_eq!(node.references_out[0].source_id, "TASK-001");
    }

    #[test]
    fn extract_frontmatter_splits_yaml_and_body() {
        let content = "---\nid: EPIC-001\ntitle: My Epic\n---\n# Body\nSome text.";
        let (fm, body) = extract_frontmatter(content);
        assert_eq!(fm.as_deref(), Some("id: EPIC-001\ntitle: My Epic"));
        assert_eq!(body, "# Body\nSome text.");
    }

    #[test]
    fn humanize_stem_all_caps_unchanged() {
        use std::path::PathBuf;
        let path = PathBuf::from("EPIC-001.md");
        assert_eq!(humanize_stem(&path), "EPIC-001");
    }

    #[test]
    fn humanize_stem_kebab_case_title_cased() {
        use std::path::PathBuf;
        let path = PathBuf::from("my-epic.md");
        assert_eq!(humanize_stem(&path), "My Epic");
    }
}
