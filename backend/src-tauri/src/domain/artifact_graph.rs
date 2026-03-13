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
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&fm_text).unwrap_or(serde_yaml::Value::Null);

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
        .map_or_else(|| humanize_stem(file_path), str::to_owned);

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
    if let Some(seq) = yaml_value.get("relationships").and_then(|v| v.as_sequence()) {
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
    }

    refs
}

/// Compute summary statistics for the graph.
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats {
    let node_count = graph.nodes.len();

    let edge_count: usize = graph.nodes.values().map(|n| n.references_out.len()).sum();

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
// Integrity checks
// ---------------------------------------------------------------------------

/// Category of integrity issue found in the artifact graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrityCategory {
    BrokenLink,
    MissingInverse,
    NullTarget,
    ResearchGap,
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
pub fn check_integrity(graph: &ArtifactGraph) -> Vec<IntegrityCheck> {
    let mut checks = Vec::new();

    // 1. Broken references — target_id doesn't exist in the graph
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

    // 2. Missing bidirectional inverses for relationship edges
    let inverse_map: &[(&str, &str)] = &[
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
    ];

    for node in graph.nodes.values() {
        for ref_entry in &node.references_out {
            let rel_type = match &ref_entry.relationship_type {
                Some(t) => t.as_str(),
                None => continue,
            };

            let expected_inverse = match inverse_map.iter().find(|(t, _)| *t == rel_type) {
                Some((_, inv)) => *inv,
                None => continue,
            };

            // Check if target has the inverse pointing back
            let target = match graph.nodes.get(&ref_entry.target_id) {
                Some(t) => t,
                None => continue, // broken ref, already caught above
            };

            let has_inverse = target.references_out.iter().any(|r| {
                r.relationship_type.as_deref() == Some(expected_inverse)
                    && r.target_id == node.id
            });

            if !has_inverse {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::MissingInverse,
                    severity: IntegritySeverity::Warning,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} --{}--> {} but {} has no {} edge back to {}",
                        node.id, rel_type, ref_entry.target_id,
                        ref_entry.target_id, expected_inverse, node.id
                    ),
                    auto_fixable: true,
                    fix_description: Some(format!(
                        "Add {{ target: \"{}\", type: \"{}\" }} to {}'s relationships array",
                        node.id, expected_inverse, ref_entry.target_id
                    )),
                });
            }
        }
    }

    // 3. Research gaps — delivered/partially-delivered ideas with unresolved research-needed
    check_research_gaps(graph, &mut checks);

    checks
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

        // Check if any tasks reference this idea (by ID appearing in task descriptions
        // or via backlinks from tasks)
        let has_related_tasks = node.references_in.iter().any(|r| {
            graph
                .nodes
                .get(&r.source_id)
                .map(|n| n.artifact_type == "task")
                .unwrap_or(false)
        });

        if !has_related_tasks {
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

// ---------------------------------------------------------------------------
// Auto-fix engine
// ---------------------------------------------------------------------------

/// Apply auto-fixable integrity checks by modifying artifact files on disk.
///
/// Currently supports:
/// - `MissingInverse`: adds the inverse relationship entry to the target artifact's
///   frontmatter `relationships` array.
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

        match check.category {
            IntegrityCategory::MissingInverse => {
                if let Some(fix) = apply_missing_inverse_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            _ => {}
        }
    }

    Ok(applied)
}

/// Fix a missing inverse relationship by adding the inverse entry to the target file.
fn apply_missing_inverse_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, OrqaError> {
    // Parse the message to extract source_id, target_id, and inverse_type.
    // Format: "RULE-001 --enforces--> AD-001 but AD-001 has no enforced-by edge back to RULE-001"
    let message = &check.message;

    let parts: Vec<&str> = message.split(" --").collect();
    if parts.len() < 2 {
        return Ok(None);
    }
    let source_id = parts[0].trim();

    let rest = parts[1];
    let arrow_parts: Vec<&str> = rest.split("--> ").collect();
    if arrow_parts.len() < 2 {
        return Ok(None);
    }

    let after_arrow = arrow_parts[1];
    // "AD-001 but AD-001 has no enforced-by edge back to RULE-001"
    let but_parts: Vec<&str> = after_arrow.split(" but ").collect();
    if but_parts.len() < 2 {
        return Ok(None);
    }
    let target_id = but_parts[0].trim();

    // Extract inverse type: "AD-001 has no enforced-by edge back to RULE-001"
    let has_no_parts: Vec<&str> = but_parts[1].split(" has no ").collect();
    if has_no_parts.len() < 2 {
        return Ok(None);
    }
    let edge_parts: Vec<&str> = has_no_parts[1].split(" edge back to ").collect();
    if edge_parts.is_empty() {
        return Ok(None);
    }
    let inverse_type = edge_parts[0].trim();

    // Find the target node's file path.
    let target_node = match graph.nodes.get(target_id) {
        Some(n) => n,
        None => return Ok(None),
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

    // Parse the YAML to check if the inverse relationship already exists.
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text)
        .map_err(|e| OrqaError::Validation(format!("YAML parse error in {}: {e}", target_node.path)))?;

    if let Some(rels) = yaml_value.get("relationships").and_then(|v| v.as_sequence()) {
        for rel in rels {
            let existing_target = rel.get("target").and_then(|v| v.as_str());
            let existing_type = rel.get("type").and_then(|v| v.as_str());
            if existing_target == Some(source_id) && existing_type == Some(inverse_type) {
                // Already exists — skip.
                return Ok(None);
            }
        }
    }

    // Build the new relationship entry to append.
    let new_entry = format!(
        "  - target: {}\n    type: {}\n    rationale: \"Auto-generated inverse of {} relationship from {}\"",
        source_id, inverse_type, inverse_type, check.artifact_id
    );

    // Reconstruct the file content with the new relationship added.
    let new_content = if fm_text.contains("relationships:") {
        // Append to existing relationships array.
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
            // relationships key exists but is empty — add after it.
            let new_fm = fm_text.replace(
                "relationships:",
                &format!("relationships:\n{new_entry}"),
            );
            format!("---\n{new_fm}\n---\n{body}")
        }
    } else {
        // No relationships key — add one at the end of frontmatter.
        let new_fm = format!("{}\nrelationships:\n{new_entry}", fm_text.trim_end());
        format!("---\n{new_fm}\n---\n{body}")
    };

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
        let checks = check_integrity(&graph);
        assert_eq!(checks.len(), 1);
        assert!(matches!(checks[0].category, IntegrityCategory::BrokenLink));
        assert!(matches!(checks[0].severity, IntegritySeverity::Error));
        assert_eq!(checks[0].artifact_id, "TASK-001");
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
        let checks = check_integrity(&graph);
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
        let checks = check_integrity(&graph);
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
        let checks = check_integrity(&graph);

        let missing: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, IntegrityCategory::MissingInverse))
            .collect();
        assert!(!missing.is_empty(), "should find missing inverse");

        let applied = apply_fixes(&graph, &checks, tmp.path()).expect("apply");
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].artifact_id, "AD-001");

        // Verify the file was updated
        let updated_content =
            fs::read_to_string(decisions_dir.join("AD-001.md")).expect("read");
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
        let checks2 = check_integrity(&graph2);
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
        let checks = check_integrity(&graph);
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
        let checks = check_integrity(&graph);
        let applied = apply_fixes(&graph, &checks, tmp.path()).expect("apply");
        assert_eq!(applied.len(), 1);

        let updated = fs::read_to_string(pillars_dir.join("PILLAR-001.md")).expect("read");
        assert!(updated.contains("relationships:"), "should have relationships key");
        assert!(updated.contains("grounded-by"), "should have inverse type");
        assert!(updated.contains("RULE-001"), "should reference source");
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
