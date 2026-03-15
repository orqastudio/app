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
    /// Semantic relationship type (e.g. "enforced-by", "grounded", "practices").
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

/// Build an `ArtifactNode` from parsed frontmatter YAML and markdown body.
fn build_node(
    id: String,
    rel_path: String,
    file_path: &Path,
    yaml_value: &serde_yaml::Value,
    body: &str,
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
    let artifact_type = infer_artifact_type(&rel_path);
    let frontmatter = yaml_to_json(yaml_value);
    let mut references_out = collect_forward_refs(yaml_value, &id);
    references_out.extend(collect_body_refs(body, &id));
    ArtifactNode {
        id,
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
fn collect_node(
    file_path: &Path,
    project_root: &Path,
    graph: &mut ArtifactGraph,
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
    let node = build_node(id.clone(), rel_path.clone(), file_path, &yaml_value, &body);
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
                    relationship_type: None,
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
                            relationship_type: None,
                        });
                    }
                }
            }
        }
    }

    // Process `relationships` array — typed semantic edges.
    refs.extend(collect_relationship_refs(yaml_value, source_id));

    refs
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
pub fn check_integrity(
    graph: &ArtifactGraph,
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
) -> Vec<IntegrityCheck> {
    let mut checks = Vec::new();

    check_broken_refs(graph, &mut checks);
    check_missing_inverses(graph, &mut checks);
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

    checks
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

/// Bidirectional inverse relationship pairs.
const INVERSE_MAP: &[(&str, &str)] = &[
    ("observes", "observed-by"),
    ("observed-by", "observes"),
    ("grounded", "grounded-by"),
    ("grounded-by", "grounded"),
    ("practices", "practiced-by"),
    ("practiced-by", "practices"),
    ("enforces", "enforced-by"),
    ("enforced-by", "enforces"),
    ("verifies", "verified-by"),
    ("verified-by", "verifies"),
    ("informs", "informed-by"),
    ("informed-by", "informs"),
    ("scoped-to", "scoped-by"),
    ("scoped-by", "scoped-to"),
    ("documents", "documented-by"),
    ("documented-by", "documents"),
    ("belongs-to", "contains"),
    ("contains", "belongs-to"),
    ("delivers", "delivered-by"),
    ("delivered-by", "delivers"),
];

/// Check a single outgoing reference for a missing bidirectional inverse.
///
/// Appends an `IntegrityCheck` to `checks` when the inverse is absent.
fn check_ref_inverse(
    node: &ArtifactNode,
    ref_entry: &ArtifactRef,
    graph: &ArtifactGraph,
    checks: &mut Vec<IntegrityCheck>,
) {
    let rel_type = match &ref_entry.relationship_type {
        Some(t) => t.as_str(),
        None => return,
    };
    let expected_inverse = match INVERSE_MAP.iter().find(|(t, _)| *t == rel_type) {
        Some((_, inv)) => *inv,
        None => return,
    };
    let Some(target) = graph.nodes.get(&ref_entry.target_id) else {
        return; // broken ref, already caught above
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

/// Check for missing bidirectional inverses on relationship edges.
fn check_missing_inverses(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        for ref_entry in &node.references_out {
            check_ref_inverse(node, ref_entry, graph, checks);
        }
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

/// Check if a frontmatter field has a non-null, non-empty string value.
fn has_non_null_string(node: &ArtifactNode, field: &str) -> bool {
    matches!(
        node.frontmatter.get(field),
        Some(v) if !v.is_null() && v.as_str().is_some_and(|s| !s.is_empty())
    )
}

/// Check if an artifact has an indirect milestone through an epic reference.
fn has_indirect_milestone(graph: &ArtifactGraph, node: &ArtifactNode) -> bool {
    let field = if node.artifact_type == "task" {
        "epic"
    } else if node.artifact_type == "idea" {
        "promoted-to"
    } else {
        return false;
    };
    node.frontmatter
        .get(field)
        .and_then(|v| v.as_str())
        .and_then(|epic_id| graph.nodes.get(epic_id))
        .is_some_and(|epic| has_non_null_string(epic, "milestone"))
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
        let has_direct_milestone = has_non_null_string(node, "milestone");

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
fn check_dependency_violations(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "task" {
            continue;
        }

        let status = match &node.status {
            Some(s) if s == "in-progress" => s.clone(),
            _ => continue,
        };

        let Some(serde_json::Value::Array(deps)) = node.frontmatter.get("depends-on") else {
            continue;
        };

        for dep in deps {
            let Some(dep_id) = dep.as_str() else {
                continue;
            };

            let dep_done = graph
                .nodes
                .get(dep_id)
                .and_then(|n| n.status.as_deref())
                .is_some_and(|s| s == "done");

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

/// Run DFS cycle detection starting from a single node's dependency list.
fn detect_cycles_from(
    graph: &ArtifactGraph,
    start_id: &str,
    initial_deps: &[serde_json::Value],
    reported: &mut std::collections::HashSet<String>,
    checks: &mut Vec<IntegrityCheck>,
) {
    let mut visited = std::collections::HashSet::new();
    let mut stack = Vec::new();

    for dep in initial_deps {
        if let Some(dep_id) = dep.as_str() {
            stack.push((dep_id.to_string(), vec![start_id.to_string()]));
        }
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
            if let Some(serde_json::Value::Array(next_deps)) =
                dep_node.frontmatter.get("depends-on")
            {
                for next_dep in next_deps {
                    if let Some(next_id) = next_dep.as_str() {
                        let mut new_path = path.clone();
                        new_path.push(current_id.clone());
                        stack.push((next_id.to_string(), new_path));
                    }
                }
            }
        }
    }
}

/// Detect circular dependencies in depends-on chains.
fn check_circular_dependencies(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    let mut reported = std::collections::HashSet::new();

    for node in graph.nodes.values() {
        let deps = match node.frontmatter.get("depends-on") {
            Some(serde_json::Value::Array(arr)) if !arr.is_empty() => arr,
            _ => continue,
        };

        detect_cycles_from(graph, &node.id, deps, &mut reported, checks);
    }
}

/// Extract supersession target IDs from a frontmatter field (handles both string and array).
fn extract_supersession_targets(frontmatter: &serde_json::Value, field: &str) -> Vec<String> {
    match frontmatter.get(field) {
        Some(serde_json::Value::String(s)) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                vec![]
            } else {
                vec![trimmed.to_owned()]
            }
        }
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.trim().to_owned()))
            .filter(|s| !s.is_empty())
            .collect(),
        _ => vec![],
    }
}

/// Check one direction of a supersession relationship and report if the back-reference is missing.
fn check_one_supersession_direction(
    node: &ArtifactNode,
    graph: &ArtifactGraph,
    forward_field: &str,
    backward_field: &str,
    checks: &mut Vec<IntegrityCheck>,
) {
    let target_ids = extract_supersession_targets(&node.frontmatter, forward_field);
    for target_id in target_ids {
        if let Some(target) = graph.nodes.get(&target_id) {
            let back_targets = extract_supersession_targets(&target.frontmatter, backward_field);
            if !back_targets.iter().any(|s| s == &node.id) {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::SupersessionSymmetry,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} {} {} but {} does not have {}: {}",
                        node.id, forward_field, target_id, target_id, backward_field, node.id
                    ),
                    auto_fixable: false,
                    fix_description: Some(format!(
                        "Add {}: {} to {}'s frontmatter",
                        backward_field, node.id, target_id
                    )),
                });
            }
        }
    }
}

/// Check decision supersession symmetry — both sides must be updated.
fn check_supersession_symmetry(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        if node.artifact_type != "decision" {
            continue;
        }

        check_one_supersession_direction(node, graph, "supersedes", "superseded-by", checks);
        check_one_supersession_direction(node, graph, "superseded-by", "supersedes", checks);
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

        // The frontmatter field on child types that points to this gate type.
        let parent_field = delivery
            .types
            .iter()
            .find(|t| {
                t.parent
                    .as_ref()
                    .is_some_and(|p| p.parent_type == gate_type.key)
            })
            .and_then(|t| t.parent.as_ref())
            .map_or("milestone", |p| p.field.as_str());

        check_gate_type_nodes(graph, gate_type, &child_type_keys, parent_field, checks);
    }
}

/// Check all complete nodes of a gate type for incomplete P1 children.
fn check_gate_type_nodes(
    graph: &ArtifactGraph,
    gate_type: &crate::domain::project_settings::DeliveryTypeConfig,
    child_type_keys: &[&str],
    parent_field: &str,
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
            collect_incomplete_p1_children(graph, node, child_type_keys, parent_field);

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
fn collect_incomplete_p1_children<'g>(
    graph: &'g ArtifactGraph,
    parent_node: &ArtifactNode,
    child_type_keys: &[&str],
    parent_field: &str,
) -> Vec<&'g str> {
    graph
        .nodes
        .values()
        .filter(|n| {
            child_type_keys.contains(&n.artifact_type.as_str())
                && n.frontmatter
                    .get(parent_field)
                    .and_then(|v| v.as_str())
                    .is_some_and(|m| m == parent_node.id)
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

        let status = match &node.status {
            Some(s) if s == "promoted" => s.as_str(),
            _ => continue,
        };

        // A promoted idea should have promoted-to set
        let has_promoted_to = node
            .frontmatter
            .get("promoted-to")
            .and_then(|v| v.as_str())
            .is_some_and(|s| !s.is_empty());

        if !has_promoted_to {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::IdeaPromotionValidity,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} has status {} but promoted-to is not set",
                    node.id, status
                ),
                auto_fixable: false,
                fix_description: Some(
                    "Set promoted-to to the epic ID, or revert status to shaped".to_string(),
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

        if node.status.as_deref() != Some("promoted") {
            continue;
        }

        let epic_id = match node.frontmatter.get("promoted-to").and_then(|v| v.as_str()) {
            Some(id) if !id.is_empty() => id,
            _ => continue,
        };

        let epic_done = graph
            .nodes
            .get(epic_id)
            .and_then(|n| n.status.as_deref())
            .is_some_and(|s| s == "done");

        if epic_done {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::IdeaDeliveryTracking,
                severity: IntegritySeverity::Warning,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} is promoted to {} which is done — idea should be delivered or partially-delivered",
                    node.id, epic_id
                ),
                auto_fixable: false,
                fix_description: Some(format!(
                    "Update {} status to delivered (if fully covered) or partially-delivered",
                    node.id
                )),
            });
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
    // artifacts of that type against their parent field.
    for dtype in &delivery.types {
        let Some(parent_cfg) = &dtype.parent else {
            continue;
        };
        check_child_type_consistency(
            graph,
            &dtype.key,
            &parent_cfg.field,
            &parent_cfg.parent_type,
            &status_pos,
            checks,
        );
    }
}

/// Check all artifacts of `child_type` for parent-child status inconsistencies.
fn check_child_type_consistency(
    graph: &ArtifactGraph,
    child_type: &str,
    parent_field: &str,
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
        let Some(parent_id) = node.frontmatter.get(parent_field).and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(parent) = graph.nodes.get(parent_id) else {
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
                parent_id,
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
    fn single_ref_field_creates_forward_ref() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
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
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
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
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
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
            "---\nid: TASK-001\ntitle: Task\nepic: EPIC-MISSING\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default());

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
        let checks2 = check_integrity(&graph2, &[], &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &[], &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default());
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
        let checks = check_integrity(&graph, &valid, &DeliveryConfig::default());
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
        assert_eq!(
            infer_artifact_type(".orqa/delivery/epics/EPIC-001.md"),
            "epic"
        );
        assert_eq!(
            infer_artifact_type(".orqa/delivery/tasks/TASK-001.md"),
            "task"
        );
        assert_eq!(
            infer_artifact_type(".orqa/delivery/milestones/MS-001.md"),
            "milestone"
        );
        assert_eq!(
            infer_artifact_type(".orqa/process/decisions/AD-001.md"),
            "decision"
        );
        assert_eq!(
            infer_artifact_type(".orqa/process/lessons/IMPL-001.md"),
            "lesson"
        );
        assert_eq!(
            infer_artifact_type(".orqa/documentation/product/vision.md"),
            "doc"
        );
    }
}
