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

    Ok(graph)
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
    NullTarget,
    ResearchGap,
    PlanningPlacement,
    DependencyViolation,
    CircularDependency,
    SupersessionSymmetry,
    MilestoneGate,
    IdeaPromotionValidity,
    IdeaDeliveryTracking,
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

/// A fix that was applied to resolve an integrity issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    pub artifact_id: String,
    pub description: String,
    pub file_path: String,
}

/// Run integrity checks on the artifact graph and return all findings.
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
) -> Vec<IntegrityCheck> {
    let mut checks = Vec::new();

    check_broken_refs(graph, &mut checks);
    check_missing_inverses(graph, project_relationships, &mut checks);
    check_research_gaps(graph, &mut checks);
    check_planning_placement(graph, &mut checks);
    check_dependency_violations(graph, &mut checks);
    check_circular_dependencies(graph, &mut checks);
    check_supersession_symmetry(graph, &mut checks);
    check_milestone_gate(graph, delivery, &mut checks);
    check_idea_promotion_validity(graph, &mut checks);
    check_idea_delivery_tracking(graph, &mut checks);
    check_body_text_refs_without_relationships(graph, &mut checks);
    if !valid_statuses.is_empty() {
        check_invalid_statuses(graph, valid_statuses, &mut checks);
        check_parent_child_consistency(graph, valid_statuses, delivery, &mut checks);
    }
    if !delivery.types.is_empty() {
        check_delivery_paths(graph, delivery, &mut checks);
    }

    checks
}

/// Validate delivery artifacts against the `DeliveryConfig`.
///
/// Warns when:
/// - An artifact under `.orqa/delivery/` is not covered by any configured delivery type.
/// - An artifact's inferred type doesn't match the delivery type key for its path.
/// - A delivery type has a parent config but the artifact is missing the parent field.
/// - The parent field points to an artifact of the wrong type.
fn check_delivery_paths(
    graph: &ArtifactGraph,
    delivery: &DeliveryConfig,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph
        .nodes
        .values()
        .filter(|n| n.path.starts_with(".orqa/delivery/"))
    {
        let matched = delivery
            .types
            .iter()
            .find(|dt| node.path.starts_with(dt.path.trim_end_matches('/')));

        let Some(dtype) = matched else {
            push_orphaned_from_config(node, checks);
            continue;
        };

        check_delivery_node_type(node, dtype, checks);
        check_delivery_node_parent(node, dtype, graph, checks);
    }
}

/// Warn when no configured delivery type covers an artifact's path.
fn push_orphaned_from_config(node: &ArtifactNode, checks: &mut Vec<IntegrityCheck>) {
    checks.push(IntegrityCheck {
        category: IntegrityCategory::DeliveryPathMismatch,
        severity: IntegritySeverity::Warning,
        artifact_id: node.id.clone(),
        message: format!(
            "{} is under '{}' but no delivery type in the config covers that path",
            node.id, node.path
        ),
        auto_fixable: false,
        fix_description: Some(
            "Add a delivery type entry to project.json covering this path, \
             or move the artifact to a configured path"
                .to_owned(),
        ),
    });
}

/// Warn when an artifact's inferred type does not match its delivery type key.
fn check_delivery_node_type(
    node: &ArtifactNode,
    dtype: &crate::domain::project_settings::DeliveryTypeConfig,
    checks: &mut Vec<IntegrityCheck>,
) {
    if node.artifact_type == dtype.key {
        return;
    }
    checks.push(IntegrityCheck {
        category: IntegrityCategory::DeliveryPathMismatch,
        severity: IntegritySeverity::Warning,
        artifact_id: node.id.clone(),
        message: format!(
            "{} is under path '{}' (delivery type '{}') but has artifact_type '{}'",
            node.id, dtype.path, dtype.key, node.artifact_type
        ),
        auto_fixable: false,
        fix_description: Some(format!(
            "Move the artifact to the correct directory, \
             or update the delivery type key in project.json to '{}'",
            node.artifact_type
        )),
    });
}

/// Validate the parent relationship for a delivery node that has a parent config.
#[allow(clippy::too_many_lines)]
fn check_delivery_node_parent(
    node: &ArtifactNode,
    dtype: &crate::domain::project_settings::DeliveryTypeConfig,
    graph: &ArtifactGraph,
    checks: &mut Vec<IntegrityCheck>,
) {
    let Some(parent_cfg) = &dtype.parent else {
        return;
    };

    // Find the parent via relationship graph: look for an outgoing edge whose
    // relationship_type matches parent_cfg.relationship and whose target is the
    // expected parent type.
    let parent_ref = node.references_out.iter().find(|r| {
        r.relationship_type.as_deref() == Some(&parent_cfg.relationship)
            && graph
                .nodes
                .get(&r.target_id)
                .is_some_and(|n| n.artifact_type == parent_cfg.parent_type)
    });

    let Some(parent_ref) = parent_ref else {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::DeliveryPathMismatch,
            severity: IntegritySeverity::Warning,
            artifact_id: node.id.clone(),
            message: format!(
                "{} (delivery type '{}') is missing required '{}' relationship to a {} artifact",
                node.id, dtype.key, parent_cfg.relationship, parent_cfg.parent_type
            ),
            auto_fixable: false,
            fix_description: Some(format!(
                "Add a '{}' relationship targeting a {} artifact",
                parent_cfg.relationship, parent_cfg.parent_type
            )),
        });
        return;
    };

    // Broken link is already caught by check_broken_refs — avoid noise.
    let Some(parent_node) = graph.nodes.get(&parent_ref.target_id) else {
        return;
    };

    if parent_node.artifact_type != parent_cfg.parent_type {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::DeliveryPathMismatch,
            severity: IntegritySeverity::Warning,
            artifact_id: node.id.clone(),
            message: format!(
                "{} has {} relationship to '{}' but {} is a '{}', expected '{}'",
                node.id,
                parent_cfg.relationship,
                parent_ref.target_id,
                parent_ref.target_id,
                parent_node.artifact_type,
                parent_cfg.parent_type
            ),
            auto_fixable: false,
            fix_description: Some(format!(
                "Update '{}' relationship to target a valid {} artifact",
                parent_cfg.relationship, parent_cfg.parent_type
            )),
        });
    }
}

/// Check for broken references — target_id doesn't exist in the graph.
fn check_broken_refs(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        for ref_entry in &node.references_out {
            if !graph.nodes.contains_key(&ref_entry.target_id) {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::BrokenLink,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "Reference to {} (field: {}) does not resolve to any artifact",
                        ref_entry.target_id, ref_entry.field
                    ),
                    auto_fixable: false,
                    fix_description: None,
                });
            }
        }
    }
}

/// Canonical bidirectional inverse relationship pairs.
///
/// 10 canonical pairs + self-inverse `synchronised-with`.
/// Project relationships (e.g. `depends-on`/`depended-on-by`) are loaded at
/// runtime from `project.json` and merged in `check_integrity`.
const INVERSE_MAP: &[(&str, &str)] = &[
    ("informs", "informed-by"),
    ("informed-by", "informs"),
    ("evolves-into", "evolves-from"),
    ("evolves-from", "evolves-into"),
    ("drives", "driven-by"),
    ("driven-by", "drives"),
    ("governs", "governed-by"),
    ("governed-by", "governs"),
    ("delivers", "delivered-by"),
    ("delivered-by", "delivers"),
    ("enforces", "enforced-by"),
    ("enforced-by", "enforces"),
    ("grounded", "grounded-by"),
    ("grounded-by", "grounded"),
    ("observes", "observed-by"),
    ("observed-by", "observes"),
    ("merged-into", "merged-from"),
    ("merged-from", "merged-into"),
    ("synchronised-with", "synchronised-with"),
];

/// Check for missing bidirectional inverses on relationship edges.
///
/// Builds a combined inverse map from canonical pairs + project relationships.
fn check_missing_inverses(
    graph: &ArtifactGraph,
    project_relationships: &[crate::domain::project_settings::ProjectRelationshipConfig],
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        for ref_entry in &node.references_out {
            check_ref_inverse_with_project(node, ref_entry, graph, project_relationships, checks);
        }
    }
}

/// Check a single outgoing reference for a missing bidirectional inverse,
/// considering both canonical and project relationship types.
fn check_ref_inverse_with_project(
    node: &ArtifactNode,
    ref_entry: &ArtifactRef,
    graph: &ArtifactGraph,
    project_relationships: &[crate::domain::project_settings::ProjectRelationshipConfig],
    checks: &mut Vec<IntegrityCheck>,
) {
    let rel_type = match &ref_entry.relationship_type {
        Some(t) => t.as_str(),
        None => return,
    };

    // Look up inverse: first in canonical map, then in project relationships.
    let expected_inverse = INVERSE_MAP
        .iter()
        .find(|(t, _)| *t == rel_type)
        .map(|(_, inv)| *inv)
        .or_else(|| {
            project_relationships.iter().find_map(|pr| {
                if pr.key == rel_type {
                    Some(pr.inverse.as_str())
                } else if pr.inverse == rel_type {
                    Some(pr.key.as_str())
                } else {
                    None
                }
            })
        });

    let Some(expected_inverse) = expected_inverse else {
        return;
    };

    let Some(target) = graph.nodes.get(&ref_entry.target_id) else {
        return;
    };

    let has_inverse = target.references_out.iter().any(|r| {
        r.relationship_type.as_deref() == Some(expected_inverse) && r.target_id == node.id
    });

    if !has_inverse {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::MissingInverse,
            severity: IntegritySeverity::Warning,
            artifact_id: node.id.clone(),
            message: format!(
                "{} --{}--> {} but {} has no {} edge back to {}",
                node.id,
                rel_type,
                ref_entry.target_id,
                ref_entry.target_id,
                expected_inverse,
                node.id
            ),
            auto_fixable: true,
            fix_description: Some(format!(
                "Add {{ target: \"{}\", type: \"{}\" }} to {}'s relationships array",
                node.id, expected_inverse, ref_entry.target_id
            )),
        });
    }
}

/// Check that delivered ideas have their research-needed items tracked as tasks.
fn check_research_gaps(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "idea" {
            continue;
        }

        let status = match &node.status {
            Some(s) => s.as_str(),
            None => continue,
        };

        if status != "delivered" && status != "partially-delivered" {
            continue;
        }

        let research_needed = match node.frontmatter.get("research-needed") {
            Some(serde_json::Value::Array(arr)) if !arr.is_empty() => arr,
            _ => continue,
        };

        // Check if any artifacts reference this idea via relationships —
        // tasks, research docs, rules, decisions, or epics all count as evidence
        // that the research questions were addressed.
        let has_related_artifacts = node
            .references_in
            .iter()
            .any(|r| r.field == "relationships" && graph.nodes.contains_key(&r.source_id));

        if !has_related_artifacts {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::ResearchGap,
                severity: IntegritySeverity::Warning,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} is {} with {} research-needed items but no tasks reference it — \
                     research questions may be unresolved",
                    node.id,
                    status,
                    research_needed.len()
                ),
                auto_fixable: false,
                fix_description: Some(
                    "Create tasks to resolve the remaining research questions, \
                     or document answers in the idea body"
                        .to_string(),
                ),
            });
        }
    }
}

/// Check if an artifact has an indirect milestone through a relationship chain.
///
/// Tasks: follow `delivers` → epic, then check if epic has `delivers` → milestone.
/// Ideas: follow `evolves-into` → epic, then check if epic has `delivers` → milestone.
fn has_indirect_milestone(graph: &ArtifactGraph, node: &ArtifactNode) -> bool {
    let rel_type = if node.artifact_type == "task" {
        "delivers"
    } else if node.artifact_type == "idea" {
        "evolves-into"
    } else {
        return false;
    };
    node.references_out
        .iter()
        .filter(|r| r.relationship_type.as_deref() == Some(rel_type))
        .any(|r| {
            graph.nodes.get(&r.target_id).is_some_and(|parent| {
                parent.references_out.iter().any(|pr| {
                    pr.relationship_type.as_deref() == Some("delivers")
                        && graph
                            .nodes
                            .get(&pr.target_id)
                            .is_some_and(|n| n.artifact_type == "milestone")
                })
            })
        })
}

/// Check that planning artifacts are placed — reachable from a milestone or have a horizon.
fn check_planning_placement(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    let planning_types = ["idea", "epic", "task"];

    for node in graph.nodes.values() {
        if !planning_types.contains(&node.artifact_type.as_str()) {
            continue;
        }

        // Skip terminal statuses — delivered/archived/discarded ideas don't need placement
        if let Some(status) = &node.status {
            let s = status.as_str();
            if s == "delivered" || s == "archived" || s == "discarded" || s == "done" {
                continue;
            }
        }

        let has_horizon = matches!(
            node.frontmatter.get("horizon"),
            Some(v) if !v.is_null() && v.as_str() != Some("null")
        );
        // Check for direct `delivers` relationship to a milestone.
        let has_direct_milestone = node.references_out.iter().any(|r| {
            r.relationship_type.as_deref() == Some("delivers")
                && graph
                    .nodes
                    .get(&r.target_id)
                    .is_some_and(|n| n.artifact_type == "milestone")
        });

        if !has_horizon && !has_direct_milestone && !has_indirect_milestone(graph, node) {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::PlanningPlacement,
                severity: IntegritySeverity::Warning,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} ({}) has no milestone and no planning horizon — untriaged",
                    node.id, node.artifact_type
                ),
                auto_fixable: false,
                fix_description: Some(
                    "Set a horizon (active/next/later/someday) or assign a milestone".to_string(),
                ),
            });
        }
    }
}

/// Check for in-progress tasks whose dependencies are not done.
///
/// Uses `depends-on` relationship edges instead of frontmatter fields.
fn check_dependency_violations(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "task" {
            continue;
        }

        let status = match &node.status {
            Some(s) if s == "in-progress" => s.clone(),
            _ => continue,
        };

        let deps: Vec<&str> = node
            .references_out
            .iter()
            .filter(|r| r.relationship_type.as_deref() == Some("depends-on"))
            .map(|r| r.target_id.as_str())
            .collect();

        for dep_id in deps {
            let dep_done = graph
                .nodes
                .get(dep_id)
                .and_then(|n| n.status.as_deref())
                .is_some_and(|s| s == "done" || s == "completed");

            if !dep_done {
                let dep_status = graph
                    .nodes
                    .get(dep_id)
                    .and_then(|n| n.status.clone())
                    .unwrap_or_else(|| "unknown".to_string());

                checks.push(IntegrityCheck {
                    category: IntegrityCategory::DependencyViolation,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} is {} but dependency {} is {} — dependency gate violated",
                        node.id, status, dep_id, dep_status
                    ),
                    auto_fixable: false,
                    fix_description: Some(format!(
                        "Complete {} before {} can be in-progress",
                        dep_id, node.id
                    )),
                });
            }
        }
    }
}

/// Run DFS cycle detection starting from a single node's dependency relationships.
fn detect_cycles_from(
    graph: &ArtifactGraph,
    start_id: &str,
    initial_dep_ids: &[String],
    reported: &mut std::collections::HashSet<String>,
    checks: &mut Vec<IntegrityCheck>,
) {
    let mut visited = std::collections::HashSet::new();
    let mut stack = Vec::new();

    for dep_id in initial_dep_ids {
        stack.push((dep_id.clone(), vec![start_id.to_string()]));
    }

    while let Some((current_id, path)) = stack.pop() {
        if current_id == start_id {
            let mut cycle_parts = path.clone();
            cycle_parts.sort();
            let cycle_key = cycle_parts.join(",");
            if !reported.contains(&cycle_key) {
                reported.insert(cycle_key);
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::CircularDependency,
                    severity: IntegritySeverity::Error,
                    artifact_id: start_id.to_string(),
                    message: format!(
                        "Circular dependency: {} → {} → {}",
                        start_id,
                        path[1..].join(" → "),
                        start_id
                    ),
                    auto_fixable: false,
                    fix_description: Some(
                        "Break the dependency cycle by removing one edge".to_string(),
                    ),
                });
            }
            continue;
        }

        if !visited.insert(current_id.clone()) {
            continue;
        }

        if let Some(dep_node) = graph.nodes.get(&current_id) {
            let next_deps: Vec<String> = dep_node
                .references_out
                .iter()
                .filter(|r| r.relationship_type.as_deref() == Some("depends-on"))
                .map(|r| r.target_id.clone())
                .collect();
            for next_id in next_deps {
                let mut new_path = path.clone();
                new_path.push(current_id.clone());
                stack.push((next_id, new_path));
            }
        }
    }
}

/// Detect circular dependencies in depends-on relationship chains.
fn check_circular_dependencies(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    let mut reported = std::collections::HashSet::new();

    for node in graph.nodes.values() {
        let deps: Vec<String> = node
            .references_out
            .iter()
            .filter(|r| r.relationship_type.as_deref() == Some("depends-on"))
            .map(|r| r.target_id.clone())
            .collect();

        if deps.is_empty() {
            continue;
        }

        detect_cycles_from(graph, &node.id, &deps, &mut reported, checks);
    }
}

/// Check decision supersession symmetry — both sides must be updated.
///
/// Now uses `evolves-into`/`evolves-from` canonical relationships.
fn check_supersession_symmetry(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "decision" {
            continue;
        }

        // Check evolves-into / evolves-from symmetry via relationship edges
        for ref_entry in &node.references_out {
            let (rel_type, expected_inverse) = match ref_entry.relationship_type.as_deref() {
                Some("evolves-into") => ("evolves-into", "evolves-from"),
                Some("evolves-from") => ("evolves-from", "evolves-into"),
                _ => continue,
            };
            let Some(target) = graph.nodes.get(&ref_entry.target_id) else {
                continue;
            };
            let has_inverse = target.references_out.iter().any(|r| {
                r.relationship_type.as_deref() == Some(expected_inverse) && r.target_id == node.id
            });
            if !has_inverse {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::SupersessionSymmetry,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} {} {} but {} does not have {} back to {}",
                        node.id,
                        rel_type,
                        ref_entry.target_id,
                        ref_entry.target_id,
                        expected_inverse,
                        node.id
                    ),
                    auto_fixable: false,
                    fix_description: Some(format!(
                        "Add {{ target: \"{}\", type: \"{}\" }} to {}'s relationships",
                        node.id, expected_inverse, ref_entry.target_id
                    )),
                });
            }
        }
    }
}

/// Check milestone gates — completed gate-type artifacts should not have incomplete P1 children.
///
/// Uses `delivery.types` to find types with a `gate_field` configured. Falls back to
/// the hardcoded milestone → epic check when `delivery.types` is empty.
fn check_milestone_gate(
    graph: &ArtifactGraph,
    delivery: &DeliveryConfig,
    checks: &mut Vec<IntegrityCheck>,
) {
    if delivery.types.is_empty() {
        check_milestone_gate_hardcoded(graph, checks);
        return;
    }

    for gate_type in delivery.types.iter().filter(|t| t.gate_field.is_some()) {
        let child_type_keys: Vec<&str> = delivery
            .types
            .iter()
            .filter(|t| {
                t.parent
                    .as_ref()
                    .is_some_and(|p| p.parent_type == gate_type.key)
            })
            .map(|t| t.key.as_str())
            .collect();

        // The relationship type on child types that connects to this gate type.
        let parent_relationship = delivery
            .types
            .iter()
            .find(|t| {
                t.parent
                    .as_ref()
                    .is_some_and(|p| p.parent_type == gate_type.key)
            })
            .and_then(|t| t.parent.as_ref())
            .map_or("delivers", |p| p.relationship.as_str());

        check_gate_type_nodes(
            graph,
            gate_type,
            &child_type_keys,
            parent_relationship,
            checks,
        );
    }
}

/// Check all complete nodes of a gate type for incomplete P1 children.
fn check_gate_type_nodes(
    graph: &ArtifactGraph,
    gate_type: &crate::domain::project_settings::DeliveryTypeConfig,
    child_type_keys: &[&str],
    parent_relationship: &str,
    checks: &mut Vec<IntegrityCheck>,
) {
    let child_label = if child_type_keys.len() == 1 {
        child_type_keys[0].to_owned()
    } else {
        "child".to_owned()
    };

    for node in graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == gate_type.key)
    {
        let status = match &node.status {
            Some(s) if s == "complete" => s.as_str(),
            _ => continue,
        };

        let incomplete_p1 =
            collect_incomplete_p1_children(graph, node, child_type_keys, parent_relationship);

        if !incomplete_p1.is_empty() {
            push_milestone_gate_finding(
                checks,
                &node.id,
                status,
                &incomplete_p1,
                &child_label,
                &gate_type.key,
            );
        }
    }
}

/// Collect IDs of P1 child artifacts that are not yet `done`.
///
/// Uses the relationship graph: a child references its parent via `parent_relationship`
/// (e.g. "delivers") in `references_out`.
fn collect_incomplete_p1_children<'g>(
    graph: &'g ArtifactGraph,
    parent_node: &ArtifactNode,
    child_type_keys: &[&str],
    parent_relationship: &str,
) -> Vec<&'g str> {
    graph
        .nodes
        .values()
        .filter(|n| {
            child_type_keys.contains(&n.artifact_type.as_str())
                && n.references_out.iter().any(|r| {
                    r.relationship_type.as_deref() == Some(parent_relationship)
                        && r.target_id == parent_node.id
                })
                && n.frontmatter
                    .get("priority")
                    .and_then(|v| v.as_str())
                    .is_some_and(|p| p == "P1")
                && n.status.as_deref() != Some("done")
        })
        .map(|n| n.id.as_str())
        .collect()
}

/// Push a `MilestoneGate` finding for a complete parent with incomplete P1 children.
fn push_milestone_gate_finding(
    checks: &mut Vec<IntegrityCheck>,
    parent_id: &str,
    parent_status: &str,
    incomplete_p1: &[&str],
    child_label: &str,
    gate_type_key: &str,
) {
    checks.push(IntegrityCheck {
        category: IntegrityCategory::MilestoneGate,
        severity: IntegritySeverity::Error,
        artifact_id: parent_id.to_owned(),
        message: format!(
            "{parent_id} is {parent_status} but has {} incomplete P1 {child_label}(s): {}",
            incomplete_p1.len(),
            incomplete_p1.join(", ")
        ),
        auto_fixable: false,
        fix_description: Some(format!(
            "Complete all P1 {child_label}s before marking {gate_type_key} as complete",
        )),
    });
}

/// Hardcoded fallback for `check_milestone_gate` when no delivery config is present.
fn check_milestone_gate_hardcoded(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "milestone" {
            continue;
        }

        let status = match &node.status {
            Some(s) if s == "complete" => s.as_str(),
            _ => continue,
        };

        let incomplete_p1: Vec<&str> = graph
            .nodes
            .values()
            .filter(|n| {
                n.artifact_type == "epic"
                    && n.frontmatter
                        .get("milestone")
                        .and_then(|v| v.as_str())
                        .is_some_and(|m| m == node.id)
                    && n.frontmatter
                        .get("priority")
                        .and_then(|v| v.as_str())
                        .is_some_and(|p| p == "P1")
                    && n.status.as_deref() != Some("done")
            })
            .map(|n| n.id.as_str())
            .collect();

        if !incomplete_p1.is_empty() {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::MilestoneGate,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} is {} but has {} incomplete P1 epic(s): {}",
                    node.id,
                    status,
                    incomplete_p1.len(),
                    incomplete_p1.join(", ")
                ),
                auto_fixable: false,
                fix_description: Some(
                    "Complete all P1 epics before marking milestone as complete".to_string(),
                ),
            });
        }
    }
}

/// Check that promoted ideas were shaped before promotion.
fn check_idea_promotion_validity(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "idea" {
            continue;
        }

        if node.status.as_deref() != Some("surpassed") {
            continue;
        }

        // A surpassed idea should have lineage: evolves-into (single successor)
        // or merged-into (consolidated with others into a larger artifact).
        let has_lineage = node.references_out.iter().any(|r| {
            matches!(
                r.relationship_type.as_deref(),
                Some("evolves-into" | "merged-into")
            )
        });

        if !has_lineage {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::IdeaPromotionValidity,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} has status surpassed but no evolves-into or merged-into relationship",
                    node.id
                ),
                auto_fixable: false,
                fix_description: Some(
                    "Add an evolves-into or merged-into relationship to the artifact this idea became"
                        .to_string(),
                ),
            });
        }
    }
}

/// Check for promoted ideas whose epics are done but idea is still promoted (not delivered).
fn check_idea_delivery_tracking(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "idea" {
            continue;
        }

        // Skip ideas already in terminal states.
        let status = match &node.status {
            Some(s) => s.as_str(),
            None => continue,
        };
        if status == "completed" || status == "surpassed" || status == "archived" {
            continue;
        }

        // Check evolves-into targets — if any are completed, the idea should be terminal.
        let evolves_targets: Vec<&str> = node
            .references_out
            .iter()
            .filter(|r| r.relationship_type.as_deref() == Some("evolves-into"))
            .map(|r| r.target_id.as_str())
            .collect();

        if evolves_targets.is_empty() {
            continue;
        }

        for target_id in evolves_targets {
            let target_completed = graph
                .nodes
                .get(target_id)
                .and_then(|n| n.status.as_deref())
                .is_some_and(|s| s == "completed");

            if target_completed {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::IdeaDeliveryTracking,
                    severity: IntegritySeverity::Warning,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} evolves-into {} which is completed, but idea is still status: {} — should be surpassed or completed",
                        node.id, target_id, status
                    ),
                    auto_fixable: false,
                    fix_description: Some(format!(
                        "Update {} status to surpassed (if fully covered) or completed",
                        node.id
                    )),
                });
            }
        }
    }
}

/// Check that every artifact ID found in body text has a corresponding relationship
/// (either in the `relationships` array or as a frontmatter reference field).
///
/// Body references to IDs that appear only as prose citations with no graph edge are
/// flagged as `BodyTextRefWithoutRelationship` warnings. The suggested auto-fix is to
/// add an `informed-by` relationship, which is the most generic relationship type.
///
/// Implementation note: body text references are already extracted during graph
/// construction by `collect_body_refs` and stored as `references_out` entries with
/// `field == "body"`. All frontmatter field references (including `relationships`,
/// `depends-on`, `epic`, `pillars`, etc.) are stored with their respective field names.
/// A body ref is "covered" when any non-body outgoing edge targets the same artifact ID.
fn check_body_text_refs_without_relationships(
    graph: &ArtifactGraph,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        // Collect body-text refs (field == "body") and check each one for a covering edge.
        for body_ref in node.references_out.iter().filter(|r| r.field == "body") {
            let target_id = &body_ref.target_id;

            // A body ref is "covered" if any non-body outgoing edge points to the same target.
            // This covers: relationships array, all SINGLE_REF_FIELDS, all ARRAY_REF_FIELDS.
            let has_relationship = node
                .references_out
                .iter()
                .any(|r| r.field != "body" && &r.target_id == target_id);

            if !has_relationship {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::BodyTextRefWithoutRelationship,
                    severity: IntegritySeverity::Warning,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} references {} in body text but has no relationship edge to it",
                        node.id, target_id
                    ),
                    auto_fixable: true,
                    fix_description: Some(format!(
                        "Add {{ target: \"{}\", type: \"informed-by\" }} to {}'s relationships array",
                        target_id, node.id
                    )),
                });
            }
        }
    }
}

/// Mapping of commonly seen legacy status values to their canonical replacements.
///
/// Used to produce auto-fix suggestions when a status value is invalid.
const LEGACY_STATUS_MAP: &[(&str, &str)] = &[
    ("draft", "captured"),
    ("todo", "ready"),
    ("done", "completed"),
    ("in-progress", "active"),
    ("wip", "active"),
    ("complete", "completed"),
    ("open", "captured"),
    ("closed", "completed"),
    ("pending", "ready"),
    ("backlog", "captured"),
];

/// Suggest a canonical replacement for a legacy status value.
///
/// Returns `Some(canonical)` when a well-known mapping exists,
/// `None` when no heuristic applies.
fn suggest_status_fix<'a>(invalid: &str, valid: &'a [String]) -> Option<&'a str> {
    // Direct match in valid list (case-insensitive) — return canonical casing.
    if let Some(v) = valid.iter().find(|s| s.eq_ignore_ascii_case(invalid)) {
        return Some(v.as_str());
    }
    // Check legacy-to-canonical map.
    let canonical_hint = LEGACY_STATUS_MAP
        .iter()
        .find(|(old, _)| old.eq_ignore_ascii_case(invalid))
        .map(|(_, new)| *new);

    let hint = canonical_hint?;
    valid
        .iter()
        .find(|s| s.eq_ignore_ascii_case(hint))
        .map(String::as_str)
}

/// Check that every artifact's `status` field is in the project's valid status list.
fn check_invalid_statuses(
    graph: &ArtifactGraph,
    valid_statuses: &[String],
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        let Some(status) = &node.status else {
            continue;
        };

        if valid_statuses.iter().any(|s| s == status) {
            continue;
        }

        let valid_list = valid_statuses.join(", ");
        let suggestion = suggest_status_fix(status, valid_statuses);
        let (auto_fixable, fix_description) = if let Some(replacement) = suggestion {
            (
                true,
                Some(format!("Change status from '{status}' to '{replacement}'")),
            )
        } else {
            (
                false,
                Some(format!(
                    "Set status to one of the valid values: {valid_list}"
                )),
            )
        };

        checks.push(IntegrityCheck {
            category: IntegrityCategory::InvalidStatus,
            severity: IntegritySeverity::Warning,
            artifact_id: node.id.clone(),
            message: format!(
                "{} has invalid status '{}'. Valid values: {}",
                node.id, status, valid_list
            ),
            auto_fixable,
            fix_description,
        });
    }
}

// ---------------------------------------------------------------------------
// Check: parent-child status consistency
// ---------------------------------------------------------------------------

/// Flags when a child artifact is further along the status progression than
/// its parent. Uses array position in `valid_statuses` as the progression order.
///
/// "Children" are artifacts referencing a parent via `epic` or `milestone`
/// frontmatter fields.
/// Push a `ParentChildInconsistency` finding when a child is further along than its parent.
fn push_parent_child_inconsistency(
    checks: &mut Vec<IntegrityCheck>,
    child_id: &str,
    child_status: &str,
    parent_id: &str,
    parent_status: &str,
    parent_label: &str,
) {
    checks.push(IntegrityCheck {
        artifact_id: child_id.to_owned(),
        category: IntegrityCategory::ParentChildInconsistency,
        severity: IntegritySeverity::Warning,
        message: format!(
            "{child_id} is '{child_status}' but {parent_label} {parent_id} is '{parent_status}' — child is further along than parent",
        ),
        auto_fixable: false,
        fix_description: Some(format!(
            "Either advance {parent_id} to at least '{child_status}', or move {child_id} to a different {parent_label}",
        )),
    });
}

fn check_parent_child_consistency(
    graph: &ArtifactGraph,
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    checks: &mut Vec<IntegrityCheck>,
) {
    // Build status → position map from config order.
    let status_pos: HashMap<&str, usize> = valid_statuses
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();

    if delivery.types.is_empty() {
        // Fallback: hardcoded epic/milestone field checks.
        check_parent_child_consistency_hardcoded(graph, &status_pos, checks);
        return;
    }

    // Config-driven: for each delivery type that declares a parent, check all
    // artifacts of that type against their parent relationship.
    for dtype in &delivery.types {
        let Some(parent_cfg) = &dtype.parent else {
            continue;
        };
        check_child_type_consistency(
            graph,
            &dtype.key,
            &parent_cfg.relationship,
            &parent_cfg.parent_type,
            &status_pos,
            checks,
        );
    }
}

/// Check all artifacts of `child_type` for parent-child status inconsistencies.
///
/// Uses the relationship graph: a child references its parent via `parent_relationship`
/// (e.g. "delivers") in `references_out`.
fn check_child_type_consistency(
    graph: &ArtifactGraph,
    child_type: &str,
    parent_relationship: &str,
    parent_label: &str,
    status_pos: &HashMap<&str, usize>,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == child_type)
    {
        let Some(child_status) = node.status.as_deref() else {
            continue;
        };
        let Some(&child_pos) = status_pos.get(child_status) else {
            continue; // invalid status, caught elsewhere
        };
        // Find parent via relationship edge
        let parent_ref = node
            .references_out
            .iter()
            .find(|r| r.relationship_type.as_deref() == Some(parent_relationship));
        let Some(parent_ref) = parent_ref else {
            continue;
        };
        let Some(parent) = graph.nodes.get(&parent_ref.target_id) else {
            continue; // broken ref, caught by check_broken_refs
        };
        let Some(parent_status) = &parent.status else {
            continue;
        };
        let Some(&parent_pos) = status_pos.get(parent_status.as_str()) else {
            continue;
        };
        if child_pos > parent_pos {
            push_parent_child_inconsistency(
                checks,
                &node.id,
                child_status,
                &parent_ref.target_id,
                parent_status,
                parent_label,
            );
        }
    }
}

/// Hardcoded fallback for `check_parent_child_consistency` when no delivery config is present.
fn check_parent_child_consistency_hardcoded(
    graph: &ArtifactGraph,
    status_pos: &HashMap<&str, usize>,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        let Some(child_status) = node.status.as_deref() else {
            continue;
        };
        let Some(&child_pos) = status_pos.get(child_status) else {
            continue;
        };

        // Check epic parent.
        if let Some(parent_id) = node.frontmatter.get("epic").and_then(|v| v.as_str()) {
            if let Some(parent) = graph.nodes.get(parent_id) {
                if let Some(parent_status) = &parent.status {
                    if let Some(&parent_pos) = status_pos.get(parent_status.as_str()) {
                        if child_pos > parent_pos {
                            push_parent_child_inconsistency(
                                checks,
                                &node.id,
                                child_status,
                                parent_id,
                                parent_status,
                                "epic",
                            );
                        }
                    }
                }
            }
        }

        // Check milestone parent.
        if let Some(parent_id) = node.frontmatter.get("milestone").and_then(|v| v.as_str()) {
            if let Some(parent) = graph.nodes.get(parent_id) {
                if let Some(parent_status) = &parent.status {
                    if let Some(&parent_pos) = status_pos.get(parent_status.as_str()) {
                        if child_pos > parent_pos {
                            push_parent_child_inconsistency(
                                checks,
                                &node.id,
                                child_status,
                                parent_id,
                                parent_status,
                                "milestone",
                            );
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Auto-fix engine
// ---------------------------------------------------------------------------

/// Apply auto-fixable integrity checks by modifying artifact files on disk.
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

///
/// Currently supports:
/// - `MissingInverse`: adds the inverse relationship entry to the target artifact's
///   frontmatter `relationships` array.
/// - `InvalidStatus`: rewrites the `status` field to the suggested canonical value.
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
    // Extract the replacement value from the fix description.
    let replacement = check
        .fix_description
        .as_deref()
        .and_then(|desc| {
            // Pattern: "Change status from 'old' to 'new'"
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
///
/// The fix description encodes the target ID in the form:
/// `Add { target: "TARGET-ID", type: "informed-by" } to SOURCE-ID's relationships array`
fn apply_body_text_ref_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, OrqaError> {
    // Parse the target ID from the fix description.
    // Pattern: `Add { target: "TARGET-ID", type: "informed-by" } to ...`
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
/// First checks the config-driven type registry (built from project.json).
/// Falls back to the hardcoded path-segment heuristic for backwards compatibility.
fn infer_artifact_type(rel_path: &str, type_registry: &TypeRegistry) -> String {
    let normalized = rel_path.replace('\\', "/");

    // Check the config-driven registry first: if the rel_path starts with a
    // registered path prefix, use its type key.
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
    } else if normalized.contains("/skills/") {
        "skill"
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[]);

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
        let checks2 = check_integrity(&graph2, &[], &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[]);
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default(), &[]);
        let invalid: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::InvalidStatus))
            .collect();
        assert_eq!(invalid.len(), 1, "draft should be flagged");
        assert!(
            invalid[0].auto_fixable,
            "draft→captured should be auto-fixable"
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
        assert_eq!(
            infer_artifact_type(".orqa/delivery/epics/EPIC-001.md", &empty_registry),
            "epic"
        );
        assert_eq!(
            infer_artifact_type(".orqa/delivery/tasks/TASK-001.md", &empty_registry),
            "task"
        );
        assert_eq!(
            infer_artifact_type(".orqa/delivery/milestones/MS-001.md", &empty_registry),
            "milestone"
        );
        assert_eq!(
            infer_artifact_type(".orqa/process/decisions/AD-001.md", &empty_registry),
            "decision"
        );
        assert_eq!(
            infer_artifact_type(".orqa/process/lessons/IMPL-001.md", &empty_registry),
            "lesson"
        );
        assert_eq!(
            infer_artifact_type(".orqa/documentation/product/vision.md", &empty_registry),
            "doc"
        );
    }

    #[test]
    fn infer_artifact_type_uses_registry() {
        let registry: TypeRegistry =
            vec![(".orqa/custom/widgets".to_string(), "widget".to_string())];
        assert_eq!(
            infer_artifact_type(".orqa/custom/widgets/W-001.md", &registry),
            "widget"
        );
        // Falls back to hardcoded when registry doesn't match
        assert_eq!(
            infer_artifact_type(".orqa/delivery/epics/EPIC-001.md", &registry),
            "epic"
        );
    }
}
