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
use crate::settings::{ArtifactEntry, ProjectSettings};

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

    // Merge static platform types with plugin-contributed types for ID-prefix inference.
    let mut platform_types: Vec<crate::platform::ArtifactTypeDef> =
        crate::platform::PLATFORM.artifact_types.clone();
    platform_types.extend(
        crate::platform::scan_plugin_manifests(project_path)
            .artifact_types
            .into_iter(),
    );

    let mut graph = ArtifactGraph::default();

    // Pass 1a: walk the project's own .orqa/ with project: None.
    walk_directory(
        &orqa_dir,
        project_path,
        &mut graph,
        &type_registry,
        &platform_types,
        None,
    )?;

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
                        &platform_types,
                        Some(&child.name),
                    )?;
                    qualify_intra_project_refs(&mut graph, &child.name);
                }
            }
        }
    }

    // Pass 1c: in organisation mode, rewrite cross-project target IDs before Pass 2.
    // Root nodes reference child artifacts by bare ID (e.g. RULE-6c0496e0) but
    // those artifacts are stored with a qualified key (e.g. app::RULE-6c0496e0).
    // Rewriting here ensures Pass 2 can find targets and insert backlinks correctly.
    let org_mode = settings.as_ref().is_some_and(|s| s.organisation);
    if org_mode {
        rewrite_cross_project_refs(&mut graph);
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

    // Pass 3: in organisation mode, insert bare-ID aliases AFTER backlinks are computed.
    // This ensures the alias node already contains its backlinks when inserted.
    if org_mode {
        insert_bare_id_aliases(&mut graph);
    }

    Ok(graph)
}

/// Build a bare-ID → qualified-graph-key index for all child-project nodes.
///
/// Root nodes (no `::` in their key) are excluded because they already resolve
/// directly. When a bare ID appears in multiple child projects, first-found wins.
fn build_child_id_index(graph: &ArtifactGraph) -> HashMap<String, String> {
    let mut bare_to_qualified: HashMap<String, String> = HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();

    for key in graph.nodes.keys() {
        if let Some(sep) = key.find("::") {
            let bare_id = &key[sep + 2..];
            if graph.nodes.contains_key(bare_id) {
                // Root key takes priority — skip.
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

/// Rewrite unresolvable bare-ID `target_id` values in `references_out` to their
/// qualified equivalents.
///
/// Must run before Pass 2 (backlink computation) so that the qualified target IDs
/// are present when backlinks are inserted.
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

/// Insert bare-ID aliases for child-project nodes so that direct `graph.nodes.get(bare_id)`
/// lookups resolve without the caller needing to know the project prefix.
///
/// Must run **after** Pass 2 (backlink computation) so that the cloned alias nodes
/// already contain their `references_in` backlinks.
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
    platform_types: &[crate::platform::ArtifactTypeDef],
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
                platform_types,
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
                platform_types,
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
    platform_types: &[crate::platform::ArtifactTypeDef],
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
        platform_types,
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
    platform_types: &[crate::platform::ArtifactTypeDef],
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
    let frontmatter_type = yaml_value.get("type").and_then(|v| v.as_str());
    let artifact_type =
        infer_artifact_type(&rel_path, type_registry, platform_types, frontmatter_type, &id);
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
///
/// In organisation mode, bare-ID alias nodes (added by `insert_bare_id_aliases`) are
/// excluded from counts to avoid double-counting. An alias node is identified by its
/// graph key equalling its `id` while also having a `project` field (meaning it belongs
/// to a child project but was aliased into the root namespace for resolution convenience).
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats {
    // Primary nodes: root nodes (project: None) OR child nodes accessed by their qualified key.
    // Alias nodes: child nodes accessed by their bare ID (key == id, project: Some(...)).
    let primary_nodes: Vec<&ArtifactNode> = graph
        .nodes
        .iter()
        .filter(|(key, node)| {
            // Root node: graph key is just the bare ID, project is None.
            // Child primary: graph key is "project::id", so key != node.id.
            // Alias: key == node.id AND project is Some — exclude these.
            !(key.as_str() == node.id && node.project.is_some())
        })
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

// ---------------------------------------------------------------------------
// Integrity checks — delegated to orqa-validation
// ---------------------------------------------------------------------------

// Re-export the validation crate's public integrity types so existing callers
// (e.g. tools/graph.rs) do not need to change their import paths.
pub use orqa_validation::IntegrityCategory;
pub use orqa_validation::IntegrityCheck;
pub use orqa_validation::IntegritySeverity;

/// Convert the MCP server's `ArtifactGraph` into the validation crate's graph type.
///
/// The two graph types are structurally identical. A serde JSON round-trip is the
/// cleanest conversion without coupling the crates at the type level.
fn to_validation_graph(
    graph: &ArtifactGraph,
) -> Result<orqa_validation::ArtifactGraph, serde_json::Error> {
    let json = serde_json::to_value(graph)?;
    serde_json::from_value(json)
}

/// Run integrity checks on the artifact graph using the `orqa-validation` library.
///
/// Loads statuses and delivery config from project settings when available, falling
/// back to empty defaults (headless mode) when project.json is not found.
/// Plugin-contributed relationships from `plugins/*/orqa-plugin.json` are merged in.
pub fn check_integrity_headless(
    graph: &ArtifactGraph,
    project_root: &Path,
) -> Vec<IntegrityCheck> {
    let val_graph = match to_validation_graph(graph) {
        Ok(g) => g,
        Err(e) => {
            tracing::error!("failed to convert graph for validation: {e}");
            return Vec::new();
        }
    };
    let plugin_contributions = crate::platform::scan_plugin_manifests(project_root);
    let plugin_relationships = contributions_to_validation_schemas(plugin_contributions);
    let ctx =
        orqa_validation::build_validation_context(&[], &Default::default(), &[], &plugin_relationships);
    orqa_validation::validate(&val_graph, &ctx)
}

/// Run integrity checks with full project settings context.
///
/// Uses statuses and delivery config from the project settings, giving richer
/// validation results than `check_integrity_headless`.
/// Plugin-contributed relationships are merged in alongside project relationships.
pub fn check_integrity_with_settings(
    graph: &ArtifactGraph,
    settings: &crate::settings::ProjectSettings,
    project_root: &Path,
) -> Vec<IntegrityCheck> {
    let val_graph = match to_validation_graph(graph) {
        Ok(g) => g,
        Err(e) => {
            tracing::error!("failed to convert graph for validation: {e}");
            return Vec::new();
        }
    };
    let statuses: Vec<String> = settings.statuses.iter().map(|s| s.key.clone()).collect();
    let mut project_relationships: Vec<orqa_validation::types::RelationshipSchema> = settings
        .relationships
        .iter()
        .map(|r| orqa_validation::types::RelationshipSchema {
            key: r.key.clone(),
            inverse: r.inverse.clone(),
            description: String::new(),
            from: vec![],
            to: vec![],
            semantic: None,
            constraints: None,
        })
        .collect();
    let plugin_contributions = crate::platform::scan_plugin_manifests(project_root);
    project_relationships.extend(contributions_to_validation_schemas(plugin_contributions));
    let ctx = orqa_validation::build_validation_context(
        &statuses,
        &Default::default(),
        &[],
        &project_relationships,
    );
    orqa_validation::validate(&val_graph, &ctx)
}

/// Convert plugin `RelationshipDef` contributions to the `orqa_validation` schema type.
fn contributions_to_validation_schemas(
    contributions: crate::platform::PluginContributions,
) -> Vec<orqa_validation::types::RelationshipSchema> {
    contributions
        .relationships
        .into_iter()
        .map(|r| orqa_validation::types::RelationshipSchema {
            key: r.key,
            inverse: r.inverse,
            description: r.description,
            from: r.from,
            to: r.to,
            semantic: r.semantic,
            constraints: r.constraints.map(|c| orqa_validation::types::RelationshipConstraints {
                required: c.required,
                min_count: c.min_count,
                max_count: c.max_count,
                require_inverse: c.require_inverse,
                status_rules: c
                    .status_rules
                    .into_iter()
                    .map(|sr| orqa_validation::types::StatusRule {
                        evaluate: sr.evaluate,
                        condition: sr.condition,
                        statuses: sr.statuses,
                        proposed_status: sr.proposed_status,
                        description: sr.description,
                    })
                    .collect(),
            }),
        })
        .collect()
}

/// Compute graph health metrics using the `orqa-validation` library.
pub fn compute_health(graph: &ArtifactGraph) -> orqa_validation::GraphHealth {
    match to_validation_graph(graph) {
        Ok(val_graph) => orqa_validation::compute_health(&val_graph),
        Err(e) => {
            tracing::error!("failed to convert graph for health computation: {e}");
            orqa_validation::GraphHealth {
                component_count: 0,
                orphan_count: 0,
                orphan_percentage: 0.0,
                avg_degree: 0.0,
                graph_density: 0.0,
                largest_component_ratio: 0.0,
                total_nodes: 0,
                total_edges: 0,
                pillar_traceability: 0.0,
                bidirectionality_ratio: 0.0,
                broken_ref_count: 0,
            }
        }
    }
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

fn infer_artifact_type(
    rel_path: &str,
    type_registry: &TypeRegistry,
    platform_types: &[crate::platform::ArtifactTypeDef],
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

    // 3. ID-prefix match against platform + plugin artifact type definitions.
    if let Some(prefix) = artifact_id.split('-').next() {
        if !prefix.is_empty() {
            let matched = platform_types
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

    // -----------------------------------------------------------------------
    // Organisation mode tests
    // -----------------------------------------------------------------------

    /// Helper: write a minimal project.json with organisation mode enabled.
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
    fn organisation_mode_scans_child_project() {
        let tmp = make_project();
        // Root project with one child sub-project "app" at ./app
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Write a rule in the child project's .orqa/
        let rules_dir = child_dir.join(".orqa/process/rules");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Test Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");

        // Should be resolvable by both qualified and bare ID.
        assert!(
            graph.nodes.contains_key("app::RULE-001"),
            "qualified key must exist"
        );
        assert!(
            graph.nodes.contains_key("RULE-001"),
            "bare-ID alias must exist for cross-project resolution"
        );
        let node = graph.nodes.get("RULE-001").expect("bare-ID lookup");
        assert_eq!(node.id, "RULE-001");
        assert_eq!(node.project.as_deref(), Some("app"));
    }

    #[test]
    fn cross_project_ref_from_root_resolves_without_broken_link() {
        let tmp = make_project();
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Root epic references RULE-001 which lives in the child project.
        let root_epics = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &root_epics,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Root Epic\nrelationships:\n  - target: RULE-001\n    type: enforced-by\n---\n",
        );

        // RULE-001 is in the child project only.
        let child_rules = child_dir.join(".orqa/process/rules");
        write_artifact(
            &child_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Child Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);

        assert_eq!(
            stats.broken_ref_count, 0,
            "cross-project ref should not be a broken link"
        );
    }

    #[test]
    fn root_project_takes_priority_over_child_on_id_conflict() {
        let tmp = make_project();
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Same ID in both root and child — root wins.
        let root_rules = tmp.path().join(".orqa/process/rules");
        write_artifact(
            &root_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Root Rule\n---\n",
        );
        let child_rules = child_dir.join(".orqa/process/rules");
        write_artifact(
            &child_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Child Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");

        // Bare-ID key should resolve to the root node.
        let node = graph.nodes.get("RULE-001").expect("node");
        assert_eq!(node.title, "Root Rule", "root project node must win");
        // Qualified key still accessible for child.
        let child_node = graph.nodes.get("app::RULE-001").expect("child node");
        assert_eq!(child_node.title, "Child Rule");
    }

    #[test]
    fn child_without_orqa_dir_is_silently_skipped() {
        let tmp = make_project();
        write_org_project_json(tmp.path(), "no-orqa-project", "no-orqa");
        // "no-orqa/" directory exists but has no .orqa/ inside.
        fs::create_dir_all(tmp.path().join("no-orqa")).expect("create dir");

        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }
}
