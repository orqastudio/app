use std::collections::HashMap;
use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::domain::project_settings::DeliveryConfig;
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
    /// Only populated for refs from the `relationships` frontmatter array.
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

/// Extended structural health metrics derived from the artifact graph topology.
///
/// Computed by `compute_graph_health` which performs a full structural analysis
/// including connected-component decomposition, traceability, and bidirectionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHealth {
    /// Total number of nodes in the graph (excluding alias nodes).
    pub total_nodes: usize,
    /// Total number of directed edges.
    pub total_edges: usize,
    /// Number of weakly-connected components (disconnected subgraphs).
    /// A healthy graph has 1 component — all artifacts reachable from each other.
    pub component_count: usize,
    /// Largest component size divided by total nodes (0.0–1.0).
    /// A value of 1.0 means all artifacts are in a single connected component.
    pub largest_component_ratio: f64,
    /// Nodes with zero incoming references (nothing points to them).
    /// Excludes `doc` artifacts which are intentionally standalone.
    pub orphan_count: usize,
    /// `orphan_count / total_nodes * 100`, rounded to 1 decimal place.
    pub orphan_percentage: f64,
    /// Average number of relationships per node (edges * 2 / nodes).
    pub avg_degree: f64,
    /// Edge density: `edges / (nodes * (nodes - 1))`, clamped to 0.0–1.0.
    pub graph_density: f64,
    /// Percentage of rules that have at least one `grounded-by` relationship to a pillar.
    /// 0.0–100.0, or 100.0 when there are no rules.
    pub pillar_traceability: f64,
    /// Ratio of relationship edges that have their inverse edge present.
    /// 0.0–1.0, or 1.0 when there are no typed relationships.
    pub bidirectionality_ratio: f64,
    /// Number of broken references (target not in graph).
    pub broken_ref_count: usize,
}

// ---------------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------------

// NOTE: SINGLE_REF_FIELDS and ARRAY_REF_FIELDS have been removed as part of
// the graph-first architecture migration. All artifact connections now use the
// `relationships` frontmatter array exclusively. The `collect_forward_refs`
// function only processes `collect_relationship_refs` and `collect_body_refs`.

/// Build an `ArtifactGraph` by scanning all `.md` files under `orqa_dir`.
///
/// Two-pass algorithm:
/// 1. Walk every `.md` file, parse frontmatter, collect nodes and forward refs.
/// 2. Invert every forward ref into a backlink on the target node.
///
/// A mapping from directory path segments to artifact type keys.
///
/// Built from `project.json` artifacts config. Each entry maps a path
/// (e.g. ".orqa/delivery/epics") to its type key (e.g. "epic").
/// Used by `infer_artifact_type` to determine types from paths without
/// hardcoding.
pub type TypeRegistry = Vec<(String, String)>;

/// Build a type registry from the project's `project.json` settings.
///
/// Reads the `artifacts` config entries and maps each path to its key.
/// Returns an empty registry if settings are unavailable.
fn build_type_registry(project_path: &Path) -> TypeRegistry {
    use crate::domain::project_settings::ArtifactEntry;

    let settings =
        crate::repo::project_settings_repo::read(&project_path.to_string_lossy()).unwrap_or(None);

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

/// Files without an `id` frontmatter field are silently skipped — they are
/// documentation pages, not typed governance artifacts.
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, OrqaError> {
    let orqa_dir = project_path.join(".orqa");
    let type_registry = build_type_registry(project_path);

    let mut graph = ArtifactGraph::default();

    // Pass 1a: walk the org project's own .orqa/ with project: None.
    walk_directory(&orqa_dir, project_path, &mut graph, &type_registry, None)?;

    // Pass 1b: if organisation mode, scan each child project.
    let settings =
        crate::repo::project_settings_repo::read(&project_path.to_string_lossy()).unwrap_or(None);

    if let Some(ref settings) = settings {
        if settings.organisation {
            for child in &settings.projects {
                let child_path = if Path::new(&child.path).is_absolute() {
                    std::path::PathBuf::from(&child.path)
                } else {
                    project_path.join(&child.path)
                };
                // Canonicalize to resolve `..` segments.
                let child_path = child_path.canonicalize().unwrap_or(child_path);
                let child_orqa = child_path.join(".orqa");
                if child_orqa.exists() {
                    let child_registry = build_type_registry(&child_path);
                    walk_directory(
                        &child_orqa,
                        &child_path,
                        &mut graph,
                        &child_registry,
                        Some(&child.name),
                    )?;
                    // Qualify intra-project refs that lack a `::` separator.
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
        // Try the target_id directly (may already be qualified or match an unqualified key).
        if let Some(target_node) = graph.nodes.get_mut(&ref_entry.target_id) {
            target_node.references_in.push(ref_entry);
        }
        // Broken references (target not in nodes) are silently counted via GraphStats.
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
fn build_child_id_index(graph: &ArtifactGraph) -> std::collections::HashMap<String, String> {
    let mut bare_to_qualified: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for key in graph.nodes.keys() {
        if let Some(sep) = key.find("::") {
            let bare_id = &key[sep + 2..];
            if graph.nodes.contains_key(bare_id) {
                // Root key takes priority — skip.
                continue;
            }
            bare_to_qualified
                .entry(bare_id.to_owned())
                .or_insert_with(|| key.clone());
        }
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
///
/// After scanning a child, any `references_out` target that doesn't contain `::`
/// is prefixed with the child project name so it resolves correctly in the
/// merged graph. Cross-project refs (already containing `::`) are left as-is.
fn qualify_intra_project_refs(graph: &mut ArtifactGraph, project_name: &str) {
    let prefix = format!("{project_name}::");
    // Collect all graph keys so we can check existence without borrow conflicts.
    let all_keys: std::collections::HashSet<String> = graph.nodes.keys().cloned().collect();

    // Collect node keys belonging to this project.
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
                    // Qualify if the qualified key exists; otherwise leave unqualified
                    // (it may be a cross-project ref or a broken link).
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
) -> Result<(), OrqaError> {
    // Directory doesn't exist — skip silently (some sub-paths may be optional).
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
            // README files carry navigation metadata, not artifact identity.
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

/// Build an `ArtifactNode` from parsed frontmatter YAML and markdown body.
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
    let frontmatter_type = yaml_value.get("type").and_then(|v| v.as_str());
    let artifact_type = infer_artifact_type(&rel_path, type_registry, frontmatter_type, &id);
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

/// Parse a single `.md` file and add an `ArtifactNode` to the graph if it has
/// a YAML `id` field.
///
/// When `project_name` is `Some`, the node gets a qualified graph key
/// (`"{project}::{id}"`) and a `project` field. In single-project mode
/// (`None`), keys are unqualified — zero behaviour change.
fn collect_node(
    file_path: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
    type_registry: &TypeRegistry,
    project_name: Option<&str>,
) -> Result<(), OrqaError> {
    let content = std::fs::read_to_string(file_path)?;
    let (fm_text, body) = crate::domain::artifact::extract_frontmatter(&content);
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

    // In org mode, qualify the graph key and path index to prevent ID collisions.
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

/// Extract forward references from the `relationships` frontmatter array.
///
/// After the graph-first migration, all connections use the `relationships`
/// array exclusively. Standalone fields (epic, milestone, depends-on, etc.)
/// are no longer processed.
fn collect_forward_refs(yaml_value: &serde_yaml::Value, source_id: &str) -> Vec<ArtifactRef> {
    collect_relationship_refs(yaml_value, source_id)
}

/// Extract forward references from the `relationships` YAML array.
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

/// Extract artifact references from markdown body text.
///
/// Scans for the pattern `[TEXT](ARTIFACT-ID)` where `ARTIFACT-ID` matches
/// `PREFIX-NNN` (e.g. `EPIC-048`, `RULE-006`). These are informational edges
/// with `field: "body"` and no `relationship_type`.
fn collect_body_refs(body: &str, source_id: &str) -> Vec<ArtifactRef> {
    // Thread-local compiled regex to avoid recompilation on every call.
    thread_local! {
        static BODY_REF_RE: Regex =
            Regex::new(r"\[([^\]]*)\]\(([A-Z]+-\d+)\)").expect("body ref regex is valid");
    }

    let mut refs = Vec::new();
    let mut seen = std::collections::HashSet::new();

    BODY_REF_RE.with(|re| {
        for cap in re.captures_iter(body) {
            let target_id = cap[2].to_owned();
            // Skip self-references and deduplicate.
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
            // Alias: key == node.id AND project is Some — exclude.
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

/// Collect primary (non-alias) nodes from the graph.
///
/// In organisation mode bare-ID alias nodes are excluded to avoid double-counting.
fn primary_nodes(graph: &ArtifactGraph) -> Vec<&ArtifactNode> {
    graph
        .nodes
        .iter()
        .filter(|(key, node)| !(key.as_str() == node.id && node.project.is_some()))
        .map(|(_, node)| node)
        .collect()
}

/// Compute the weakly-connected components of the artifact graph.
///
/// Returns a `Vec` where each element contains the IDs of nodes in that component.
/// Two nodes are in the same component if they are connected by any edge (regardless
/// of direction).
fn compute_components(nodes: &[&ArtifactNode]) -> Vec<Vec<String>> {
    // Build adjacency map (undirected) for the primary nodes.
    let mut adj: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    for node in nodes {
        adj.entry(node.id.as_str()).or_default();
        for r in &node.references_out {
            adj.entry(node.id.as_str())
                .or_default()
                .push(r.target_id.as_str());
            adj.entry(r.target_id.as_str())
                .or_default()
                .push(node.id.as_str());
        }
    }

    let mut visited: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let mut components: Vec<Vec<String>> = Vec::new();

    for node in nodes {
        if visited.contains(node.id.as_str()) {
            continue;
        }
        // BFS from this node.
        let mut component: Vec<String> = Vec::new();
        let mut queue: std::collections::VecDeque<&str> = std::collections::VecDeque::new();
        queue.push_back(node.id.as_str());
        visited.insert(node.id.as_str());
        while let Some(current) = queue.pop_front() {
            component.push(current.to_owned());
            if let Some(neighbours) = adj.get(current) {
                for &nbr in neighbours {
                    if !visited.contains(nbr) {
                        visited.insert(nbr);
                        queue.push_back(nbr);
                    }
                }
            }
        }
        components.push(component);
    }

    components
}

/// Compute extended structural health metrics for the artifact graph.
///
/// This is the authoritative source of structural health data, replacing
/// the client-side Cytoscape analysis in the frontend. All metrics are
/// computed from the Rust artifact graph representation.
pub fn compute_graph_health(graph: &ArtifactGraph) -> GraphHealth {
    let nodes = primary_nodes(graph);
    let total_nodes = nodes.len();

    if total_nodes == 0 {
        return GraphHealth {
            total_nodes: 0,
            total_edges: 0,
            component_count: 0,
            largest_component_ratio: 0.0,
            orphan_count: 0,
            orphan_percentage: 0.0,
            avg_degree: 0.0,
            graph_density: 0.0,
            pillar_traceability: 100.0,
            bidirectionality_ratio: 1.0,
            broken_ref_count: 0,
        };
    }

    let total_edges: usize = nodes.iter().map(|n| n.references_out.len()).sum();

    // Connected components (undirected BFS).
    let components = compute_components(&nodes);
    let component_count = components.len();
    let largest_component_size = components.iter().map(|c| c.len()).max().unwrap_or(0);
    let largest_component_ratio = largest_component_size as f64 / total_nodes as f64;

    // Orphans: no incoming or outgoing edges (docs excluded).
    let orphan_count = nodes
        .iter()
        .filter(|n| {
            n.artifact_type != "doc" && n.references_out.is_empty() && n.references_in.is_empty()
        })
        .count();
    let orphan_percentage = (orphan_count as f64 / total_nodes as f64 * 1000.0).round() / 10.0;

    // Average degree: (total_edges * 2) / total_nodes (undirected convention).
    let avg_degree = ((total_edges as f64 * 2.0) / total_nodes as f64 * 10.0).round() / 10.0;

    // Graph density: edges / (nodes * (nodes - 1)), directed.
    let max_edges = total_nodes * (total_nodes.saturating_sub(1));
    let graph_density = if max_edges == 0 {
        0.0
    } else {
        ((total_edges as f64 / max_edges as f64) * 10000.0).round() / 10000.0
    };

    // Pillar traceability: % of rules with at least one grounded-by → pillar edge.
    let rule_nodes: Vec<&&ArtifactNode> =
        nodes.iter().filter(|n| n.artifact_type == "rule").collect();
    let pillar_traceability = if rule_nodes.is_empty() {
        100.0
    } else {
        let grounded = rule_nodes
            .iter()
            .filter(|n| {
                n.references_out.iter().any(|r| {
                    r.relationship_type.as_deref() == Some("grounded-by")
                        && graph
                            .nodes
                            .get(&r.target_id)
                            .is_some_and(|t| t.artifact_type == "pillar")
                })
            })
            .count();
        (grounded as f64 / rule_nodes.len() as f64 * 1000.0).round() / 10.0
    };

    // Bidirectionality: ratio of typed relationship edges that have their inverse.
    // We look for any typed ref (relationship_type: Some) and check whether the
    // target has the inverse. Since we don't have the registry here, we use a
    // heuristic: if A → B with type T and B → A with any type, count as bidirectional.
    let typed_refs: Vec<(&str, &str)> = nodes
        .iter()
        .flat_map(|n| {
            n.references_out
                .iter()
                .filter(|r| r.relationship_type.is_some())
                .map(|r| (n.id.as_str(), r.target_id.as_str()))
        })
        .collect();
    let bidirectionality_ratio = if typed_refs.is_empty() {
        1.0
    } else {
        // Build a set of (source, target) pairs for quick lookup.
        let ref_set: std::collections::HashSet<(&str, &str)> = nodes
            .iter()
            .flat_map(|n| {
                n.references_out
                    .iter()
                    .map(|r| (n.id.as_str(), r.target_id.as_str()))
            })
            .collect();
        let bidirectional = typed_refs
            .iter()
            .filter(|(src, tgt)| ref_set.contains(&(*tgt, *src)))
            .count();
        (bidirectional as f64 / typed_refs.len() as f64 * 1000.0).round() / 1000.0
    };

    // Broken refs.
    let broken_ref_count: usize = nodes
        .iter()
        .flat_map(|n| n.references_out.iter())
        .filter(|r| !graph.nodes.contains_key(&r.target_id))
        .count();

    GraphHealth {
        total_nodes,
        total_edges,
        component_count,
        largest_component_ratio,
        orphan_count,
        orphan_percentage,
        avg_degree,
        graph_density,
        pillar_traceability,
        bidirectionality_ratio,
        broken_ref_count,
    }
}

// ---------------------------------------------------------------------------
// Integrity checks
// ---------------------------------------------------------------------------

/// Category of integrity issue found in the artifact graph.
///
/// Generic categories derived from schema-driven checks. Domain-specific
/// variants have been replaced by schema-driven equivalents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrityCategory {
    /// Target of a reference does not exist in the graph.
    BrokenLink,
    /// Inverse relationship edge is missing.
    MissingInverse,
    /// From/to type constraints on a relationship are violated.
    TypeConstraintViolation,
    /// A required relationship (constraints.required) is missing or below minCount.
    RequiredRelationshipMissing,
    /// A maxCount cardinality constraint is exceeded.
    CardinalityViolation,
    /// A cycle was detected on a relationship with "dependency" semantic.
    CircularDependency,
    /// Status value is not in the valid status list.
    InvalidStatus,
    /// Body text references an artifact without a corresponding relationship edge.
    BodyTextRefWithoutRelationship,
    /// Child artifact is further along the status progression than its parent.
    ParentChildInconsistency,
    /// Delivery path does not match the delivery config hierarchy.
    DeliveryPathMismatch,
    /// The artifact has no `type:` field in its frontmatter.
    MissingType,
    /// The artifact has no `status:` field in its frontmatter.
    MissingStatus,
    /// The same target + relationship type appears more than once in `relationships`.
    DuplicateRelationship,
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

/// A fix that was applied to resolve an integrity issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    pub artifact_id: String,
    pub description: String,
    pub file_path: String,
}

/// Run integrity checks on the artifact graph and return all findings.
///
/// Delegates to the schema-driven integrity engine. All checks are derived
/// from relationship schemas — no hardcoded relationship keys or artifact types.
///
/// `valid_statuses` is the list of status keys defined in `project.json`.
/// Pass an empty slice to skip status validation (e.g. when settings are unavailable).
///
/// `delivery` is the delivery-type hierarchy from `project.json`. Pass
/// `&DeliveryConfig::default()` to use only hardcoded fallback behaviour.
///
/// `project_relationships` is the list of project-level relationship types from
/// `project.json`. These extend the canonical inverse map for missing-inverse
/// detection. Pass an empty slice when not available.
pub fn check_integrity(
    graph: &ArtifactGraph,
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[crate::domain::project_settings::ProjectRelationshipConfig],
    plugin_relationships: &[crate::domain::integrity_engine::RelationshipSchema],
) -> Vec<IntegrityCheck> {
    let ctx = crate::domain::integrity_engine::build_validation_context(
        valid_statuses,
        delivery,
        project_relationships,
        plugin_relationships,
    );
    crate::domain::integrity_engine::run_schema_checks(graph, &ctx)
}

// ---------------------------------------------------------------------------
// Auto-fix engine
// ---------------------------------------------------------------------------

/// Update a single scalar frontmatter field in an artifact file.
///
/// Reads the file, finds the field in the YAML block, replaces its value,
/// and writes the file back to disk. The YAML frontmatter must be delimited
/// by `---` markers.
///
/// Only simple `key: value` scalar fields are supported. The field must already
/// exist in the frontmatter — this function does not add new fields.
///
/// Returns `OrqaError::Validation` if the field is not found or the file has
/// no valid frontmatter block.
pub fn update_artifact_field(full_path: &Path, field: &str, value: &str) -> Result<(), OrqaError> {
    let content =
        std::fs::read_to_string(full_path).map_err(|e| OrqaError::FileSystem(e.to_string()))?;

    let (fm_opt, body) = crate::domain::artifact::extract_frontmatter(&content);
    let fm_text = fm_opt.ok_or_else(|| {
        OrqaError::Validation(format!("no frontmatter block in '{}'", full_path.display()))
    })?;

    // Replace the line `field: old_value` with `field: new_value`.
    // We match on the field name at the start of a line (with optional leading spaces).
    let field_prefix = format!("{field}:");
    let mut found = false;
    let new_fm = fm_text
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if let Some(_rest) = trimmed.strip_prefix(field_prefix.as_str()) {
                found = true;
                // Preserve leading whitespace from the original line.
                let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
                return format!("{indent}{field}: {value}");
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    if !found {
        return Err(OrqaError::Validation(format!(
            "field '{field}' not found in frontmatter of '{}'",
            full_path.display()
        )));
    }

    let new_content = format!("---\n{new_fm}\n---\n{body}");
    std::fs::write(full_path, new_content).map_err(|e| OrqaError::FileSystem(e.to_string()))?;

    Ok(())
}

/// Apply auto-fixable integrity checks by modifying artifact files on disk.
///
/// Currently supports:
/// - `MissingInverse`: adds the inverse relationship entry to the target artifact's
///   frontmatter `relationships` array.
/// - `InvalidStatus`: rewrites the `status` field to the suggested canonical value.
/// - `BodyTextRefWithoutRelationship`: adds an `informed-by` relationship.
///
/// Returns a list of fixes that were successfully applied.
pub fn apply_fixes(
    graph: &ArtifactGraph,
    checks: &[IntegrityCheck],
    project_path: &Path,
) -> Result<Vec<AppliedFix>, OrqaError> {
    let mut applied = Vec::new();

    for check in checks {
        if !check.auto_fixable {
            continue;
        }

        match &check.category {
            IntegrityCategory::MissingInverse => {
                if let Some(fix) = apply_missing_inverse_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            IntegrityCategory::InvalidStatus => {
                if let Some(fix) = apply_invalid_status_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            IntegrityCategory::BodyTextRefWithoutRelationship => {
                if let Some(fix) = apply_body_text_ref_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            _ => {}
        }
    }

    Ok(applied)
}

/// Fix an invalid status by rewriting the `status` field to its suggested replacement.
///
/// The fix description is expected to contain the replacement value in the form
/// `"Change status from 'old' to 'new'"`, which is parsed to extract the target value.
fn apply_invalid_status_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, OrqaError> {
    let replacement = check
        .fix_description
        .as_deref()
        .and_then(|desc| {
            let after_to = desc.split(" to '").nth(1)?;
            after_to.strip_suffix('\'')
        })
        .map(str::to_owned);

    let Some(replacement) = replacement else {
        return Ok(None);
    };

    let Some(node) = graph.nodes.get(&check.artifact_id) else {
        return Ok(None);
    };

    let file_path = project_path.join(&node.path);
    if !file_path.exists() {
        return Ok(None);
    }

    update_artifact_field(&file_path, "status", &replacement)?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!("Updated status to '{}' in {}", replacement, node.path),
        file_path: node.path.clone(),
    }))
}

/// Parse the missing-inverse check message to extract source_id, target_id, and inverse_type.
///
/// Expected format: "RULE-001 --enforces--> AD-001 but AD-001 has no enforced-by edge back to RULE-001"
fn parse_missing_inverse_message(message: &str) -> Option<(&str, &str, &str)> {
    let parts: Vec<&str> = message.split(" --").collect();
    if parts.len() < 2 {
        return None;
    }
    let source_id = parts[0].trim();

    let arrow_parts: Vec<&str> = parts[1].split("--> ").collect();
    if arrow_parts.len() < 2 {
        return None;
    }

    let but_parts: Vec<&str> = arrow_parts[1].split(" but ").collect();
    if but_parts.len() < 2 {
        return None;
    }
    let target_id = but_parts[0].trim();

    let has_no_parts: Vec<&str> = but_parts[1].split(" has no ").collect();
    if has_no_parts.len() < 2 {
        return None;
    }
    let edge_parts: Vec<&str> = has_no_parts[1].split(" edge back to ").collect();
    if edge_parts.is_empty() {
        return None;
    }
    let inverse_type = edge_parts[0].trim();

    Some((source_id, target_id, inverse_type))
}

/// Fix a body-text ref without relationship by adding an `informed-by` relationship entry
/// to the source artifact's own frontmatter.
fn apply_body_text_ref_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, OrqaError> {
    let target_id = check.fix_description.as_deref().and_then(|desc| {
        let after = desc.strip_prefix("Add { target: \"")?;
        let end = after.find('"')?;
        Some(after[..end].to_owned())
    });

    let Some(target_id) = target_id else {
        return Ok(None);
    };

    let Some(source_node) = graph.nodes.get(&check.artifact_id) else {
        return Ok(None);
    };

    let file_path = project_path.join(&source_node.path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content =
        std::fs::read_to_string(&file_path).map_err(|e| OrqaError::FileSystem(e.to_string()))?;
    let (fm_text, body) = crate::domain::artifact::extract_frontmatter(&content);
    let Some(fm_text) = fm_text else {
        return Ok(None);
    };

    // Guard: don't add a duplicate entry.
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&fm_text).unwrap_or(serde_yaml::Value::Null);
    if let Some(rels) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    {
        let already_present = rels.iter().any(|rel| {
            rel.get("target").and_then(|v| v.as_str()) == Some(&target_id)
                && rel.get("type").and_then(|v| v.as_str()) == Some("informed-by")
        });
        if already_present {
            return Ok(None);
        }
    }

    let new_entry = format!(
        "  - target: {target_id}\n    type: informed-by\n    rationale: \"Auto-generated from body text reference\""
    );

    let new_content = insert_relationship_entry(&fm_text, &body, &new_entry);
    std::fs::write(&file_path, new_content).map_err(|e| OrqaError::FileSystem(e.to_string()))?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!(
            "Added {{ target: \"{}\", type: \"informed-by\" }} to {}'s relationships",
            target_id, check.artifact_id
        ),
        file_path: source_node.path.clone(),
    }))
}

/// Insert a new relationship entry into frontmatter text, returning the full reconstructed file.
fn insert_relationship_entry(fm_text: &str, body: &str, new_entry: &str) -> String {
    if fm_text.contains("relationships:") {
        let lines: Vec<&str> = fm_text.lines().collect();
        let mut insert_pos = None;
        let mut in_relationships = false;

        for (i, line) in lines.iter().enumerate() {
            if line.trim_start().starts_with("relationships:") {
                in_relationships = true;
                continue;
            }
            if in_relationships {
                if line.starts_with("  - ") || line.starts_with("    ") {
                    insert_pos = Some(i + 1);
                } else if !line.trim().is_empty() {
                    break;
                }
            }
        }

        if let Some(pos) = insert_pos {
            let entry_lines: Vec<&str> = new_entry.lines().collect();
            let mut new_lines = lines.clone();
            for (j, entry_line) in entry_lines.iter().enumerate() {
                new_lines.insert(pos + j, entry_line);
            }
            format!("---\n{}\n---\n{}", new_lines.join("\n"), body)
        } else {
            let new_fm = fm_text.replace("relationships:", &format!("relationships:\n{new_entry}"));
            format!("---\n{new_fm}\n---\n{body}")
        }
    } else {
        let new_fm = format!("{}\nrelationships:\n{new_entry}", fm_text.trim_end());
        format!("---\n{new_fm}\n---\n{body}")
    }
}

/// Fix a missing inverse relationship by adding the inverse entry to the target file.
fn apply_missing_inverse_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, OrqaError> {
    let Some((source_id, target_id, inverse_type)) = parse_missing_inverse_message(&check.message)
    else {
        return Ok(None);
    };

    let Some(target_node) = graph.nodes.get(target_id) else {
        return Ok(None);
    };

    let file_path = project_path.join(&target_node.path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)?;
    let (fm_text, body) = crate::domain::artifact::extract_frontmatter(&content);
    let Some(fm_text) = fm_text else {
        return Ok(None);
    };

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text).map_err(|e| {
        OrqaError::Validation(format!("YAML parse error in {}: {e}", target_node.path))
    })?;

    if let Some(rels) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    {
        for rel in rels {
            let existing_target = rel.get("target").and_then(|v| v.as_str());
            let existing_type = rel.get("type").and_then(|v| v.as_str());
            if existing_target == Some(source_id) && existing_type == Some(inverse_type) {
                return Ok(None);
            }
        }
    }

    let new_entry = format!(
        "  - target: {}\n    type: {}\n    rationale: \"Auto-generated inverse of {} relationship from {}\"",
        source_id, inverse_type, inverse_type, check.artifact_id
    );

    let new_content = insert_relationship_entry(&fm_text, &body, &new_entry);
    std::fs::write(&file_path, new_content)?;

    Ok(Some(AppliedFix {
        artifact_id: target_id.to_string(),
        description: format!(
            "Added {{ target: \"{source_id}\", type: \"{inverse_type}\" }} to relationships"
        ),
        file_path: target_node.path.clone(),
    }))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Infer a human-readable artifact type category from a relative file path.
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
    if let Some(t) = frontmatter_type.map(str::trim).filter(|t| !t.is_empty()) {
        return t.to_owned();
    }
    let normalized = rel_path.replace('\\', "/");
    if let Some(t) = type_from_registry(&normalized, type_registry) {
        return t;
    }
    if let Some(t) = type_from_id_prefix(artifact_id) {
        return t;
    }
    type_from_path_heuristic(&normalized)
}

/// Return the type key for the longest-matching path prefix in the registry.
fn type_from_registry(normalized: &str, type_registry: &TypeRegistry) -> Option<String> {
    let mut best: Option<(&String, &String)> = None;
    for (path_prefix, type_key) in type_registry {
        let prefix_slash = if path_prefix.ends_with('/') {
            path_prefix.clone()
        } else {
            format!("{path_prefix}/")
        };
        if (normalized.starts_with(&prefix_slash) || normalized == *path_prefix)
            && (best.is_none() || path_prefix.len() > best.unwrap().0.len())
        {
            best = Some((path_prefix, type_key));
        }
    }
    best.map(|(_, k)| k.clone())
}

/// Return the artifact type key matching the ID prefix against core.json types.
fn type_from_id_prefix(artifact_id: &str) -> Option<String> {
    let prefix = artifact_id.split('-').next().filter(|p| !p.is_empty())?;
    crate::domain::platform_config::PLATFORM
        .artifact_types
        .iter()
        .find(|t| t.id_prefix == prefix)
        .map(|t| t.key.clone())
}

/// Hardcoded path-segment heuristic for well-known directory names.
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
                    s.extend(chars.flat_map(char::to_lowercase));
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
        assert_eq!(node.references_out[0].source_id, "TASK-001");
        assert_eq!(
            node.references_out[0].relationship_type,
            Some("delivers".to_owned())
        );
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
        assert_eq!(epic.references_in[0].field, "relationships");
    }

    #[test]
    fn multiple_relationships_create_forward_refs() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-002.md",
            "---\nid: TASK-002\ntitle: Dependent Task\nrelationships:\n  - target: TASK-001\n    type: depends-on\n  - target: TASK-003\n    type: depends-on\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let node = graph.nodes.get("TASK-002").expect("node");
        assert_eq!(node.references_out.len(), 2);
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
    fn path_index_maps_path_to_id() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
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
    fn graph_stats_orphan_count() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
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
    fn relationships_array_creates_typed_refs() {
        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Agent Delegation\nrelationships:\n  - target: AD-001\n    type: enforces\n    rationale: Enforces the agent delegation principle\n  - target: PILLAR-001\n    type: grounded\n    rationale: Grounded in clarity pillar\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let node = graph.nodes.get("RULE-001").expect("node");

        let rel_refs: Vec<_> = node
            .references_out
            .iter()
            .filter(|r| r.field == "relationships")
            .collect();
        assert_eq!(rel_refs.len(), 2);

        let enforces = rel_refs
            .iter()
            .find(|r| r.target_id == "AD-001")
            .expect("AD-001 ref");
        assert_eq!(enforces.relationship_type.as_deref(), Some("enforces"));

        let grounded = rel_refs
            .iter()
            .find(|r| r.target_id == "PILLAR-001")
            .expect("PILLAR-001 ref");
        assert_eq!(grounded.relationship_type.as_deref(), Some("grounded"));
    }

    #[test]
    fn check_integrity_finds_broken_refs() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Task\nrelationships:\n  - target: EPIC-MISSING\n    type: delivers\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[], &[]);
        let broken: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::BrokenLink))
            .collect();
        assert_eq!(broken.len(), 1);
        assert!(matches!(broken[0].severity, IntegritySeverity::Error));
        assert_eq!(broken[0].artifact_id, "TASK-001");
    }

    #[test]
    fn check_integrity_finds_missing_inverses() {
        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        let decisions_dir = tmp.path().join(".orqa/process/decisions");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Rule\nrelationships:\n  - target: AD-001\n    type: enforces\n---\n",
        );
        write_artifact(
            &decisions_dir,
            "AD-001.md",
            "---\nid: AD-001\ntitle: Decision\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[], &[]);
        let missing = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::MissingInverse))
            .collect::<Vec<_>>();
        assert_eq!(missing.len(), 1);
        assert!(missing[0].auto_fixable);
        assert!(missing[0].message.contains("enforced-by"));
    }

    #[test]
    fn check_integrity_clean_with_matching_inverses() {
        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        let decisions_dir = tmp.path().join(".orqa/process/decisions");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Rule\nrelationships:\n  - target: AD-001\n    type: enforces\n---\n",
        );
        write_artifact(
            &decisions_dir,
            "AD-001.md",
            "---\nid: AD-001\ntitle: Decision\nrelationships:\n  - target: RULE-001\n    type: enforced-by\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[], &[]);
        let missing = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::MissingInverse))
            .collect::<Vec<_>>();
        assert_eq!(missing.len(), 0);
    }

    #[test]
    fn apply_fixes_adds_missing_inverse() {
        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        let decisions_dir = tmp.path().join(".orqa/process/decisions");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Rule\nrelationships:\n  - target: AD-001\n    type: enforces\n    rationale: Test\n---\nBody\n",
        );
        write_artifact(
            &decisions_dir,
            "AD-001.md",
            "---\nid: AD-001\ntitle: Decision\n---\nBody\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[], &[]);

        let missing: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::MissingInverse))
            .collect();
        assert!(!missing.is_empty(), "should find missing inverse");

        let applied = apply_fixes(&graph, &checks, tmp.path()).expect("apply");
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].artifact_id, "AD-001");

        // Verify the file was updated
        let updated_content = fs::read_to_string(decisions_dir.join("AD-001.md")).expect("read");
        assert!(
            updated_content.contains("enforced-by"),
            "should contain inverse relationship type"
        );
        assert!(
            updated_content.contains("RULE-001"),
            "should reference source artifact"
        );

        // Rebuild graph and verify no more missing inverses
        let graph2 = build_artifact_graph(tmp.path()).expect("rebuild");
        let checks2 = check_integrity(&graph2, &[], &DeliveryConfig::default(), &[], &[]);
        let missing2: Vec<_> = checks2
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::MissingInverse))
            .collect();
        assert!(
            missing2.is_empty(),
            "should have no missing inverses after fix"
        );
    }

    #[test]
    fn apply_fixes_skips_existing_inverse() {
        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        let decisions_dir = tmp.path().join(".orqa/process/decisions");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Rule\nrelationships:\n  - target: AD-001\n    type: enforces\n    rationale: Test\n---\nBody\n",
        );
        write_artifact(
            &decisions_dir,
            "AD-001.md",
            "---\nid: AD-001\ntitle: Decision\nrelationships:\n  - target: RULE-001\n    type: enforced-by\n    rationale: Already there\n---\nBody\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[], &[]);
        let applied = apply_fixes(&graph, &checks, tmp.path()).expect("apply");
        assert!(applied.is_empty(), "should not add duplicate inverse");
    }

    #[test]
    fn apply_fixes_adds_relationships_key_when_missing() {
        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        let pillars_dir = tmp.path().join(".orqa/process/pillars");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Rule\nrelationships:\n  - target: PILLAR-001\n    type: grounded\n    rationale: Test\n---\nBody\n",
        );
        write_artifact(
            &pillars_dir,
            "PILLAR-001.md",
            "---\nid: PILLAR-001\ntitle: Pillar\n---\nBody\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[], &[]);
        let applied = apply_fixes(&graph, &checks, tmp.path()).expect("apply");
        assert_eq!(applied.len(), 1);

        let updated = fs::read_to_string(pillars_dir.join("PILLAR-001.md")).expect("read");
        assert!(
            updated.contains("relationships:"),
            "should have relationships key"
        );
        assert!(updated.contains("grounded-by"), "should have inverse type");
        assert!(updated.contains("RULE-001"), "should reference source");
    }

    #[test]
    fn check_integrity_flags_invalid_status() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: wip\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let valid = vec!["active".to_string(), "completed".to_string()];
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[], &[]);
        let invalid: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::InvalidStatus))
            .collect();
        assert_eq!(invalid.len(), 1);
        assert_eq!(invalid[0].artifact_id, "EPIC-001");
        assert!(
            invalid[0].auto_fixable,
            "wip maps to active — should be auto-fixable"
        );
        assert!(invalid[0].message.contains("wip"));
        assert!(invalid[0].message.contains("active"));
    }

    #[test]
    fn check_integrity_no_findings_for_valid_status() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: active\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let valid = vec!["active".to_string(), "completed".to_string()];
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[], &[]);
        let invalid: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::InvalidStatus))
            .collect();
        assert!(
            invalid.is_empty(),
            "valid status should produce no findings"
        );
    }

    #[test]
    fn check_integrity_skips_status_check_when_no_valid_list() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: anything-goes\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        // Empty valid list — status check should be skipped entirely.
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[], &[]);
        let invalid: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::InvalidStatus))
            .collect();
        assert!(
            invalid.is_empty(),
            "empty valid_statuses should suppress status validation"
        );
    }

    #[test]
    fn check_integrity_unknown_status_not_auto_fixable() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: xyzzy\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let valid = vec!["active".to_string(), "completed".to_string()];
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[], &[]);
        let invalid: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::InvalidStatus))
            .collect();
        assert_eq!(invalid.len(), 1);
        assert!(
            !invalid[0].auto_fixable,
            "unknown status without mapping should not be auto-fixable"
        );
    }

    #[test]
    fn apply_fixes_rewrites_invalid_status() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: draft\n---\n# Body\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let valid = vec!["captured".to_string(), "active".to_string()];
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[], &[]);
        let invalid: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::InvalidStatus))
            .collect();
        assert_eq!(invalid.len(), 1, "draft should be flagged");
        assert!(
            invalid[0].auto_fixable,
            "draft->captured should be auto-fixable"
        );

        let applied = apply_fixes(&graph, &checks, tmp.path()).expect("apply fixes");
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].artifact_id, "EPIC-001");

        let updated = fs::read_to_string(epics_dir.join("EPIC-001.md")).expect("read updated");
        assert!(
            updated.contains("status: captured"),
            "status should be rewritten to canonical value"
        );
    }

    #[test]
    fn infer_artifact_type_variants() {
        let empty_registry: TypeRegistry = Vec::new();
        // These fall through to the hardcoded path-segment heuristic.
        assert_eq!(
            infer_artifact_type(
                ".orqa/delivery/epics/EPIC-001.md",
                &empty_registry,
                None,
                "EPIC-001"
            ),
            "epic"
        );
        assert_eq!(
            infer_artifact_type(
                ".orqa/delivery/tasks/TASK-001.md",
                &empty_registry,
                None,
                "TASK-001"
            ),
            "task"
        );
        assert_eq!(
            infer_artifact_type(
                ".orqa/delivery/milestones/MS-001.md",
                &empty_registry,
                None,
                "MS-001"
            ),
            "milestone"
        );
        assert_eq!(
            infer_artifact_type(
                ".orqa/process/decisions/AD-001.md",
                &empty_registry,
                None,
                "AD-001"
            ),
            "decision"
        );
        assert_eq!(
            infer_artifact_type(
                ".orqa/process/lessons/IMPL-001.md",
                &empty_registry,
                None,
                "IMPL-001"
            ),
            "lesson"
        );
        assert_eq!(
            infer_artifact_type(
                ".orqa/documentation/product/vision.md",
                &empty_registry,
                None,
                "DOC-001"
            ),
            "doc"
        );
    }

    #[test]
    fn infer_artifact_type_frontmatter_type_wins() {
        let empty_registry: TypeRegistry = Vec::new();
        // Frontmatter type: field overrides path-based inference.
        assert_eq!(
            infer_artifact_type(
                ".orqa/delivery/epics/EPIC-001.md",
                &empty_registry,
                Some("rule"),
                "EPIC-001"
            ),
            "rule"
        );
    }

    #[test]
    fn infer_artifact_type_id_prefix_fallback() {
        let empty_registry: TypeRegistry = Vec::new();
        // When neither frontmatter type nor path matches, ID prefix is used.
        assert_eq!(
            infer_artifact_type(
                ".orqa/unknown/RULE-006.md",
                &empty_registry,
                None,
                "RULE-006"
            ),
            "rule"
        );
        assert_eq!(
            infer_artifact_type(
                ".orqa/unknown/KNOW-011.md",
                &empty_registry,
                None,
                "KNOW-011"
            ),
            "knowledge"
        );
        assert_eq!(
            infer_artifact_type(
                ".orqa/unknown/AGENT-001.md",
                &empty_registry,
                None,
                "AGENT-001"
            ),
            "agent"
        );
    }

    #[test]
    fn infer_artifact_type_uses_registry() {
        let registry: TypeRegistry =
            vec![(".orqa/custom/widgets".to_string(), "widget".to_string())];
        assert_eq!(
            infer_artifact_type(".orqa/custom/widgets/W-001.md", &registry, None, "W-001"),
            "widget"
        );
        // Falls back to hardcoded when registry doesn't match
        assert_eq!(
            infer_artifact_type(
                ".orqa/delivery/epics/EPIC-001.md",
                &registry,
                None,
                "EPIC-001"
            ),
            "epic"
        );
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
        let tmp = make_project();
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        let rules_dir = child_dir.join(".orqa/process/rules");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Test Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");

        // Both qualified and bare-ID keys must exist.
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

        // Root epic references RULE-001 which only lives in the child project.
        let root_epics = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &root_epics,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Root Epic\nrelationships:\n  - target: RULE-001\n    type: enforced-by\n---\n",
        );

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
            "cross-project ref from root to child must not be a broken link"
        );
    }

    #[test]
    fn cross_project_ref_receives_backlink_from_child() {
        let tmp = make_project();
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        let root_epics = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &root_epics,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Root Epic\nrelationships:\n  - target: RULE-001\n    type: enforced-by\n---\n",
        );

        let child_rules = child_dir.join(".orqa/process/rules");
        write_artifact(
            &child_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Child Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");

        // The child rule should have a backlink from EPIC-001.
        // The bare-ID alias node gets the backlink via Pass 2.
        let rule_node = graph.nodes.get("RULE-001").expect("RULE-001 node");
        assert!(
            !rule_node.references_in.is_empty(),
            "RULE-001 should have a backlink from EPIC-001"
        );
        assert_eq!(rule_node.references_in[0].source_id, "EPIC-001");
    }

    #[test]
    fn root_project_takes_priority_over_child_on_id_conflict() {
        let tmp = make_project();
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Same ID in both root and child — root must win.
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

        let node = graph.nodes.get("RULE-001").expect("node");
        assert_eq!(node.title, "Root Rule", "root project node must win");
        assert_eq!(node.project, None, "root node has no project prefix");

        let child_node = graph.nodes.get("app::RULE-001").expect("child node");
        assert_eq!(child_node.title, "Child Rule");
    }

    #[test]
    fn child_without_orqa_dir_is_silently_skipped() {
        let tmp = make_project();
        write_org_project_json(tmp.path(), "no-orqa-project", "no-orqa");
        // Directory exists but has no .orqa/ inside.
        fs::create_dir_all(tmp.path().join("no-orqa")).expect("create dir");

        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }
}
