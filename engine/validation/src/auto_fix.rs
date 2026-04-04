//! Auto-fix engine for objective integrity issues.
//!
//! Supports:
//! - `InvalidStatus`: rewrites the `status` field to the suggested canonical value.
//! - `TypePrefixMismatch`: corrects the `type` field to match the ID prefix.
//! - `MissingType`: infers the artifact type from the path registry or ID prefix and adds it.
//! - `MissingStatus`: adds `status: captured` when the field is absent.
//! - `DuplicateRelationship`: deduplicates relationship entries with the same target + type.

use std::fmt::Write as FmtWrite;
use std::path::Path;

use crate::error::ValidationError;
use crate::graph::{extract_frontmatter, ArtifactGraph};
use crate::types::{AppliedFix, IntegrityCategory, IntegrityCheck};

/// Find a node by artifact ID, checking both bare and qualified keys.
///
/// In organisation mode, graph keys are `project::ID` but check artifact_ids
/// use the bare `ID`. This helper finds the node regardless.
fn find_node<'a>(
    graph: &'a ArtifactGraph,
    artifact_id: &str,
) -> Option<&'a crate::graph::ArtifactNode> {
    // Try bare ID first
    if let Some(node) = graph.nodes.get(artifact_id) {
        return Some(node);
    }
    // Try qualified keys (project::id)
    graph.nodes.values().find(|n| n.id == artifact_id)
}

/// Resolve the absolute file path for a node, handling child project paths.
///
/// In organisation mode, `node.path` is relative to the child project root.
/// The child project root is looked up from `.orqa/project.json`.
fn resolve_node_path(node: &crate::graph::ArtifactNode, project_path: &Path) -> std::path::PathBuf {
    if let Some(ref proj_name) = node.project {
        // Try to find the child project path from project.json
        let project_json_path = project_path.join(".orqa/project.json");
        if let Ok(content) = std::fs::read_to_string(&project_json_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(projects) = json.get("projects").and_then(|v| v.as_array()) {
                    for proj in projects {
                        let name = proj.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        if name == proj_name {
                            if let Some(child_path) = proj.get("path").and_then(|v| v.as_str()) {
                                return project_path.join(child_path).join(&node.path);
                            }
                        }
                    }
                }
            }
        }
    }
    project_path.join(&node.path)
}

/// Apply auto-fixable integrity checks by modifying artifact files on disk.
///
/// Returns the list of fixes that were successfully applied.
pub fn apply_fixes(
    graph: &ArtifactGraph,
    checks: &[IntegrityCheck],
    project_path: &Path,
) -> Result<Vec<AppliedFix>, ValidationError> {
    let mut applied = Vec::new();

    for check in checks {
        if !check.auto_fixable {
            continue;
        }

        match &check.category {
            IntegrityCategory::InvalidStatus => {
                if let Some(fix) = apply_invalid_status_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            IntegrityCategory::TypePrefixMismatch => {
                if let Some(fix) = apply_type_prefix_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            IntegrityCategory::MissingType => {
                if let Some(fix) = apply_missing_type_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            IntegrityCategory::MissingStatus => {
                if let Some(fix) = apply_missing_status_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            IntegrityCategory::DuplicateRelationship => {
                if let Some(fix) = apply_duplicate_relationship_fix(graph, check, project_path)? {
                    applied.push(fix);
                }
            }
            _ => {
                // No auto-fix defined for this category. New IntegrityCategory variants
                // should be reviewed here to determine whether an auto-fix is warranted.
                tracing::debug!(
                    "[auto_fix] no auto-fix handler for category {:?} on artifact '{}'",
                    check.category,
                    check.artifact_id
                );
            }
        }
    }

    Ok(applied)
}

/// Update a single scalar frontmatter field in an artifact file.
///
/// Reads the file, finds the field in the YAML block, replaces its value,
/// and writes the file back to disk. The YAML frontmatter must be delimited
/// by `---` markers.
///
/// Only simple `key: value` scalar fields are supported. The field must already
/// exist in the frontmatter — this function does not add new fields.
pub fn update_artifact_field(
    full_path: &Path,
    field: &str,
    value: &str,
) -> Result<(), ValidationError> {
    let content = std::fs::read_to_string(full_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    let (fm_opt, body) = extract_frontmatter(&content);
    let fm_text = fm_opt.ok_or_else(|| {
        ValidationError::Validation(format!("no frontmatter block in '{}'", full_path.display()))
    })?;

    let field_prefix = format!("{field}:");
    let mut found = false;
    let new_fm = fm_text
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if let Some(_rest) = trimmed.strip_prefix(field_prefix.as_str()) {
                found = true;
                let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
                return format!("{indent}{field}: {value}");
            }
            line.to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n");

    if !found {
        return Err(ValidationError::Validation(format!(
            "field '{field}' not found in frontmatter of '{}'",
            full_path.display()
        )));
    }

    let new_content = format!("---\n{new_fm}\n---\n{body}");
    std::fs::write(full_path, new_content)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Internal fix implementations
// ---------------------------------------------------------------------------

/// Fix an invalid status by rewriting the `status` field to the suggested replacement.
fn apply_invalid_status_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, ValidationError> {
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

    let Some(node) = find_node(graph, &check.artifact_id) else {
        return Ok(None);
    };

    let file_path = resolve_node_path(node, project_path);
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

/// Fix a type/prefix mismatch by rewriting the `type:` field to match the ID prefix.
fn apply_type_prefix_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, ValidationError> {
    // Extract the correct type from the fix description: "Change type: X to type: Y"
    let correct_type = check.fix_description.as_deref().and_then(|desc| {
        desc.strip_prefix("Change type: ")
            .and_then(|rest| rest.split(" to type: ").nth(1))
            .map(str::to_owned)
    });

    let Some(correct_type) = correct_type else {
        return Ok(None);
    };

    let Some(node) = find_node(graph, &check.artifact_id) else {
        return Ok(None);
    };

    let file_path = resolve_node_path(node, project_path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let (fm_text, body) = extract_frontmatter(&content);
    let Some(fm_text) = fm_text else {
        return Ok(None);
    };

    // Replace the type field in the frontmatter
    let mut new_fm = String::new();
    for line in fm_text.lines() {
        if line.starts_with("type:") {
            // Write to a String never fails, so the error is unreachable.
            let _ = write!(new_fm, "type: {correct_type}");
        } else {
            new_fm.push_str(line);
        }
        new_fm.push('\n');
    }

    let new_content = format!("---\n{new_fm}---\n{body}");
    std::fs::write(&file_path, new_content)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!("Changed type to '{correct_type}' to match ID prefix"),
        file_path: node.path.clone(),
    }))
}

/// Fix a missing `type:` field by inferring the type from the artifact's path and ID.
///
/// The inferred type is taken directly from the node's `artifact_type` field, which
/// was already computed by the graph builder using the path registry + ID prefix heuristic.
fn apply_missing_type_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, ValidationError> {
    let Some(node) = find_node(graph, &check.artifact_id) else {
        return Ok(None);
    };

    let inferred_type = node.artifact_type.clone();
    if inferred_type.is_empty() || inferred_type == "doc" {
        return Ok(None);
    }

    let file_path = resolve_node_path(node, project_path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    // Guard: don't add if `type:` already present as a top-level frontmatter key.
    // Must NOT match indented `type:` in relationship entries (e.g., `    type: grounded-by`).
    if content.lines().any(|l| l.starts_with("type:")) {
        return Ok(None);
    }

    // Insert `type:` after the `id:` line, operating on the raw file.
    let Some(new_content) =
        insert_field_in_file(&content, "id:", &format!("type: {inferred_type}"))
    else {
        return Ok(None);
    };

    std::fs::write(&file_path, new_content)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!("Added type: {inferred_type} to {}", node.path),
        file_path: node.path.clone(),
    }))
}

/// Fix a missing `status:` field by adding `status: captured`.
fn apply_missing_status_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, ValidationError> {
    let Some(node) = find_node(graph, &check.artifact_id) else {
        return Ok(None);
    };

    let file_path = resolve_node_path(node, project_path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    // Guard: don't add if `status:` already present as a top-level frontmatter key.
    if content.lines().any(|l| l.starts_with("status:")) {
        return Ok(None);
    }

    // Insert `status:` after `type:` if present, otherwise after `id:`.
    let anchor = if content.lines().any(|l| l.trim_start().starts_with("type:")) {
        "type:"
    } else {
        "id:"
    };
    let Some(new_content) = insert_field_in_file(&content, anchor, "status: captured") else {
        return Ok(None);
    };

    std::fs::write(&file_path, new_content)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!("Added status: captured to {}", node.path),
        file_path: node.path.clone(),
    }))
}

/// Fix duplicate relationships by deduplicating entries with the same target + type.
///
/// Keeps the first occurrence of each (target, type) pair and removes subsequent duplicates.
fn apply_duplicate_relationship_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, ValidationError> {
    let Some(node) = find_node(graph, &check.artifact_id) else {
        return Ok(None);
    };

    let file_path = resolve_node_path(node, project_path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let (fm_opt, body) = extract_frontmatter(&content);
    let Some(fm_text) = fm_opt else {
        return Ok(None);
    };

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text).map_err(|e| {
        ValidationError::Validation(format!("YAML parse error in {}: {e}", node.path))
    })?;

    let (deduped, removed) = dedup_relationships(&yaml_value);
    if removed == 0 {
        return Ok(None);
    }

    write_deduped_relationships(&yaml_value, deduped, &body, &file_path)?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!(
            "Removed {removed} duplicate relationship entries from {}",
            node.path
        ),
        file_path: node.path.clone(),
    }))
}

/// Extract and deduplicate the `relationships` sequence from a YAML value.
///
/// Returns the deduplicated sequence and the count of entries removed.
/// Returns an empty vec and zero if `relationships` is absent.
fn dedup_relationships(yaml_value: &serde_yaml::Value) -> (Vec<serde_yaml::Value>, usize) {
    let Some(rels) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    else {
        return (Vec::new(), 0);
    };
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut removed = 0usize;
    let deduped: Vec<serde_yaml::Value> = rels
        .iter()
        .filter(|rel| {
            let target = rel
                .get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let rel_type = rel
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let key = (target, rel_type);
            if seen.contains(&key) {
                removed += 1;
                false
            } else {
                seen.insert(key);
                true
            }
        })
        .cloned()
        .collect();
    (deduped, removed)
}

/// Rebuild the file with the deduplicated relationship list and write it to disk.
fn write_deduped_relationships(
    yaml_value: &serde_yaml::Value,
    deduped: Vec<serde_yaml::Value>,
    body: &str,
    file_path: &Path,
) -> Result<(), ValidationError> {
    let mut new_yaml = yaml_value.clone();
    if let Some(map) = new_yaml.as_mapping_mut() {
        map.insert(
            serde_yaml::Value::String("relationships".to_owned()),
            serde_yaml::Value::Sequence(deduped),
        );
    }
    let new_fm = serde_yaml::to_string(&new_yaml)
        .map_err(|e| ValidationError::Validation(format!("YAML serialization error: {e}")))?;
    let new_fm = new_fm
        .trim_start_matches("---\n")
        .trim_end_matches('\n')
        .to_owned();
    let new_content = format!("---\n{new_fm}\n---\n{body}");
    std::fs::write(file_path, new_content).map_err(|e| ValidationError::FileSystem(e.to_string()))
}

/// Insert a new YAML field line into a raw file, immediately after the first
/// frontmatter line that starts with `anchor_prefix`.
///
/// Operates on the raw file content (with `---` delimiters) so no
/// parse/reconstruct round-trip is needed. Returns the full file content.
fn insert_field_in_file(raw_content: &str, anchor_prefix: &str, new_field: &str) -> Option<String> {
    let lines: Vec<&str> = raw_content.lines().collect();

    // Find the opening `---`
    let open = lines.iter().position(|l| l.trim() == "---")?;

    // Find the anchor line within frontmatter
    let anchor_pos = lines[open + 1..]
        .iter()
        .position(|l| l.trim_start().starts_with(anchor_prefix))
        .map(|p| p + open + 1)?;

    // Insert the new field line after the anchor
    let mut result: Vec<&str> = Vec::with_capacity(lines.len() + 1);
    result.extend_from_slice(&lines[..=anchor_pos]);
    result.push(new_field);
    result.extend_from_slice(&lines[anchor_pos + 1..]);

    Some(result.join("\n"))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_engine_types::{ArtifactGraph, ArtifactNode};
    use std::fs;
    use tempfile::TempDir;

    fn write_temp_artifact(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
        let path = dir.path().join(name);
        fs::write(&path, content).expect("write temp artifact");
        path
    }

    fn make_node(id: &str, rel_path: &str) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: rel_path.to_owned(),
            artifact_type: "task".to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({}),
            body: None,
            references_out: vec![],
            references_in: vec![],
        }
    }

    fn make_graph_with_node(id: &str, rel_path: &str) -> ArtifactGraph {
        let mut graph = ArtifactGraph::default();
        graph.nodes.insert(id.to_owned(), make_node(id, rel_path));
        graph
    }

    // -------------------------------------------------------------------------
    // update_artifact_field tests
    // -------------------------------------------------------------------------

    #[test]
    fn update_artifact_field_replaces_status() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_temp_artifact(
            &dir,
            "task.md",
            "---\nid: TASK-001\nstatus: invalid-status\n---\nBody.\n",
        );
        update_artifact_field(&path, "status", "active").expect("update should succeed");
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("status: active"));
        assert!(!contents.contains("invalid-status"));
    }

    #[test]
    fn update_artifact_field_preserves_body() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_temp_artifact(
            &dir,
            "task.md",
            "---\nid: TASK-001\nstatus: old\n---\n## My Body\n\nContent here.\n",
        );
        update_artifact_field(&path, "status", "active").expect("update should succeed");
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("## My Body"));
        assert!(contents.contains("Content here."));
    }

    #[test]
    fn update_artifact_field_errors_on_missing_field() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_temp_artifact(
            &dir,
            "task.md",
            "---\nid: TASK-001\n---\nBody.\n",
        );
        let result = update_artifact_field(&path, "status", "active");
        assert!(result.is_err(), "should fail when field is absent");
    }

    #[test]
    fn update_artifact_field_errors_without_frontmatter() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_temp_artifact(&dir, "task.md", "no frontmatter here\n");
        let result = update_artifact_field(&path, "status", "active");
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // apply_fixes — skip non-fixable checks
    // -------------------------------------------------------------------------

    #[test]
    fn apply_fixes_skips_non_fixable_checks() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        let graph = ArtifactGraph::default();
        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::BrokenLink,
            severity: IntegritySeverity::Error,
            artifact_id: "TASK-001".to_owned(),
            message: "broken link".to_owned(),
            auto_fixable: false,
            fix_description: None,
        }];
        let result = apply_fixes(&graph, &checks, dir.path()).expect("apply_fixes should not error");
        assert!(result.is_empty());
    }

    // -------------------------------------------------------------------------
    // apply_fixes — InvalidStatus fix
    // -------------------------------------------------------------------------

    #[test]
    fn apply_fixes_invalid_status_rewrites_field() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        let artifact_content = "---\nid: TASK-001\nstatus: done\n---\nBody.\n";
        fs::write(dir.path().join("task.md"), artifact_content).unwrap();

        let graph = make_graph_with_node("TASK-001", "task.md");
        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::InvalidStatus,
            severity: IntegritySeverity::Error,
            artifact_id: "TASK-001".to_owned(),
            message: "invalid status".to_owned(),
            auto_fixable: true,
            fix_description: Some("Change status to 'completed'".to_owned()),
        }];

        let applied = apply_fixes(&graph, &checks, dir.path()).expect("apply");
        assert_eq!(applied.len(), 1);
        let contents = fs::read_to_string(dir.path().join("task.md")).unwrap();
        assert!(contents.contains("status: completed"));
    }

    // -------------------------------------------------------------------------
    // apply_fixes — MissingStatus fix
    // -------------------------------------------------------------------------

    #[test]
    fn apply_fixes_missing_status_adds_captured() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        // No status field
        let artifact_content = "---\nid: TASK-001\ntype: task\n---\nBody.\n";
        fs::write(dir.path().join("task.md"), artifact_content).unwrap();

        let graph = make_graph_with_node("TASK-001", "task.md");
        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::MissingStatus,
            severity: IntegritySeverity::Error,
            artifact_id: "TASK-001".to_owned(),
            message: "missing status".to_owned(),
            auto_fixable: true,
            fix_description: Some("Add status: captured".to_owned()),
        }];

        let applied = apply_fixes(&graph, &checks, dir.path()).expect("apply");
        assert_eq!(applied.len(), 1);
        let contents = fs::read_to_string(dir.path().join("task.md")).unwrap();
        assert!(contents.contains("status: captured"));
    }

    // -------------------------------------------------------------------------
    // apply_fixes — artifact not in graph is skipped gracefully
    // -------------------------------------------------------------------------

    #[test]
    fn apply_fixes_skips_unknown_artifact_id() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        let graph = ArtifactGraph::default(); // empty graph
        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::InvalidStatus,
            severity: IntegritySeverity::Error,
            artifact_id: "TASK-UNKNOWN".to_owned(),
            message: "invalid status".to_owned(),
            auto_fixable: true,
            fix_description: Some("Change status to 'active'".to_owned()),
        }];

        let applied = apply_fixes(&graph, &checks, dir.path()).expect("apply");
        assert!(applied.is_empty());
    }

    // -------------------------------------------------------------------------
    // insert_field_in_file tests
    // -------------------------------------------------------------------------

    #[test]
    fn insert_field_after_id_line() {
        let raw = "---\nid: TASK-001\nstatus: active\n---\nBody.\n";
        let result = insert_field_in_file(raw, "id:", "type: task");
        let result = result.expect("should succeed");
        // "type: task" should appear after the id line
        let lines: Vec<&str> = result.lines().collect();
        let id_pos = lines.iter().position(|l| l.starts_with("id:")).unwrap();
        assert_eq!(lines[id_pos + 1], "type: task");
    }

    #[test]
    fn insert_field_returns_none_when_anchor_missing() {
        let raw = "---\nstatus: active\n---\nBody.\n";
        // "id:" is not present — should return None
        let result = insert_field_in_file(raw, "id:", "type: task");
        assert!(result.is_none());
    }

    #[test]
    fn insert_field_returns_none_when_no_frontmatter() {
        let raw = "No frontmatter here at all.\n";
        let result = insert_field_in_file(raw, "id:", "type: task");
        assert!(result.is_none());
    }

    // -------------------------------------------------------------------------
    // apply_fixes — TypePrefixMismatch fix
    // -------------------------------------------------------------------------

    #[test]
    fn apply_fixes_type_prefix_mismatch_rewrites_type_field() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        let content = "---\nid: RULE-001\ntype: task\nstatus: active\n---\nBody.\n";
        fs::write(dir.path().join("rule.md"), content).unwrap();

        let graph = make_graph_with_node("RULE-001", "rule.md");
        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::TypePrefixMismatch,
            severity: IntegritySeverity::Error,
            artifact_id: "RULE-001".to_owned(),
            message: "type mismatch".to_owned(),
            auto_fixable: true,
            fix_description: Some("Change type: task to type: rule".to_owned()),
        }];

        let applied = apply_fixes(&graph, &checks, dir.path()).expect("apply");
        assert_eq!(applied.len(), 1);
        let written = fs::read_to_string(dir.path().join("rule.md")).unwrap();
        assert!(written.contains("type: rule"));
        assert!(!written.contains("type: task"));
    }

    #[test]
    fn apply_fixes_type_prefix_mismatch_missing_file_skips_gracefully() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        // Node references a file that doesn't exist
        let graph = make_graph_with_node("RULE-001", "nonexistent.md");
        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::TypePrefixMismatch,
            severity: IntegritySeverity::Error,
            artifact_id: "RULE-001".to_owned(),
            message: "type mismatch".to_owned(),
            auto_fixable: true,
            fix_description: Some("Change type: task to type: rule".to_owned()),
        }];

        let applied = apply_fixes(&graph, &checks, dir.path()).expect("apply");
        assert!(applied.is_empty());
    }

    // -------------------------------------------------------------------------
    // apply_fixes — MissingType fix
    // -------------------------------------------------------------------------

    #[test]
    fn apply_fixes_missing_type_inserts_type_field() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        let content = "---\nid: TASK-001\nstatus: active\n---\nBody.\n";
        fs::write(dir.path().join("task.md"), content).unwrap();

        let mut graph = make_graph_with_node("TASK-001", "task.md");
        // Set the artifact_type so the fix knows what type to add
        if let Some(node) = graph.nodes.get_mut("TASK-001") {
            node.artifact_type = "task".to_owned();
        }

        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::MissingType,
            severity: IntegritySeverity::Error,
            artifact_id: "TASK-001".to_owned(),
            message: "missing type".to_owned(),
            auto_fixable: true,
            fix_description: Some("Add type: task".to_owned()),
        }];

        let applied = apply_fixes(&graph, &checks, dir.path()).expect("apply");
        assert_eq!(applied.len(), 1);
        let written = fs::read_to_string(dir.path().join("task.md")).unwrap();
        assert!(written.contains("type: task"));
    }

    #[test]
    fn apply_fixes_missing_type_skips_when_type_already_present() {
        use orqa_engine_types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

        let dir = tempfile::tempdir().unwrap();
        // Already has type: task
        let content = "---\nid: TASK-001\ntype: task\nstatus: active\n---\nBody.\n";
        fs::write(dir.path().join("task.md"), content).unwrap();

        let mut graph = make_graph_with_node("TASK-001", "task.md");
        if let Some(node) = graph.nodes.get_mut("TASK-001") {
            node.artifact_type = "task".to_owned();
        }

        let checks = vec![IntegrityCheck {
            category: IntegrityCategory::MissingType,
            severity: IntegritySeverity::Error,
            artifact_id: "TASK-001".to_owned(),
            message: "missing type".to_owned(),
            auto_fixable: true,
            fix_description: Some("Add type: task".to_owned()),
        }];

        // Guard should prevent re-adding when type: is already present
        let applied = apply_fixes(&graph, &checks, dir.path()).expect("apply");
        assert!(applied.is_empty());
    }

    // -------------------------------------------------------------------------
    // update_artifact_field — indented fields
    // -------------------------------------------------------------------------

    #[test]
    fn update_artifact_field_preserves_indented_sibling_fields() {
        let dir = tempfile::tempdir().unwrap();
        let content = "---\nid: TASK-001\nstatus: pending\npriority: P1\n---\nBody.\n";
        let path = write_temp_artifact(&dir, "task.md", content);
        update_artifact_field(&path, "status", "active").expect("update");
        let written = fs::read_to_string(&path).unwrap();
        assert!(written.contains("priority: P1"));
        assert!(written.contains("status: active"));
        assert!(!written.contains("status: pending"));
    }

    // -------------------------------------------------------------------------
    // find_node — qualified key lookup
    // -------------------------------------------------------------------------

    #[test]
    fn find_node_finds_by_bare_id() {
        let graph = make_graph_with_node("TASK-001", "task.md");
        let node = find_node(&graph, "TASK-001");
        assert!(node.is_some());
    }

    #[test]
    fn find_node_returns_none_for_unknown_id() {
        let graph = ArtifactGraph::default();
        let node = find_node(&graph, "TASK-UNKNOWN");
        assert!(node.is_none());
    }
}
