//! Auto-fix engine for objective integrity issues.
//!
//! Supports:
//! - `MissingInverse`: adds the inverse relationship entry to the target artifact's frontmatter.
//! - `InvalidStatus`: rewrites the `status` field to the suggested canonical value.
//! - `BodyTextRefWithoutRelationship`: adds an `informed-by` relationship.
//! - `MissingType`: infers the artifact type from the path registry or ID prefix and adds it.
//! - `MissingStatus`: adds `status: captured` when the field is absent.
//! - `DuplicateRelationship`: deduplicates relationship entries with the same target + type.

use std::path::Path;

use crate::error::ValidationError;
use crate::graph::{extract_frontmatter, ArtifactGraph};
use crate::types::{AppliedFix, IntegrityCategory, IntegrityCheck};

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
            _ => {}
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
            line.to_string()
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
/// Expected format: `"RULE-001 --enforces--> AD-001 but AD-001 has no enforced-by edge back to RULE-001"`
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

/// Fix a missing inverse relationship by adding the inverse entry to the target file.
fn apply_missing_inverse_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, ValidationError> {
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

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let (fm_text, body) = extract_frontmatter(&content);
    let Some(fm_text) = fm_text else {
        return Ok(None);
    };

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text).map_err(|e| {
        ValidationError::Validation(format!("YAML parse error in {}: {e}", target_node.path))
    })?;

    // Guard: don't add a duplicate.
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
    std::fs::write(&file_path, new_content)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    Ok(Some(AppliedFix {
        artifact_id: target_id.to_string(),
        description: format!(
            "Added {{ target: \"{source_id}\", type: \"{inverse_type}\" }} to relationships"
        ),
        file_path: target_node.path.clone(),
    }))
}

/// Fix a body-text ref without relationship by adding an `informed-by` relationship entry.
fn apply_body_text_ref_fix(
    graph: &ArtifactGraph,
    check: &IntegrityCheck,
    project_path: &Path,
) -> Result<Option<AppliedFix>, ValidationError> {
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

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let (fm_text, body) = extract_frontmatter(&content);
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
    std::fs::write(&file_path, new_content)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!(
            "Added {{ target: \"{}\", type: \"informed-by\" }} to {}'s relationships",
            target_id, check.artifact_id
        ),
        file_path: source_node.path.clone(),
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
    let Some(node) = graph.nodes.get(&check.artifact_id) else {
        return Ok(None);
    };

    let inferred_type = node.artifact_type.clone();
    if inferred_type.is_empty() || inferred_type == "doc" {
        // "doc" is a fallback that may not be meaningful — don't add it blindly.
        return Ok(None);
    }

    let file_path = project_path.join(&node.path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let (fm_opt, body) = extract_frontmatter(&content);
    let fm_text = match fm_opt {
        Some(t) => t,
        None => return Ok(None),
    };

    // Guard: don't add if `type:` already present.
    if fm_text.lines().any(|l| l.trim_start().starts_with("type:")) {
        return Ok(None);
    }

    // Insert `type:` after the `id:` line (or at the beginning if id not found).
    let new_fm = insert_field_after(&fm_text, "id:", &format!("type: {inferred_type}"));
    let new_content = format!("---\n{new_fm}\n---\n{body}");
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
    let Some(node) = graph.nodes.get(&check.artifact_id) else {
        return Ok(None);
    };

    let file_path = project_path.join(&node.path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let (fm_opt, body) = extract_frontmatter(&content);
    let fm_text = match fm_opt {
        Some(t) => t,
        None => return Ok(None),
    };

    // Guard: don't add if `status:` already present.
    if fm_text
        .lines()
        .any(|l| l.trim_start().starts_with("status:"))
    {
        return Ok(None);
    }

    // Insert `status:` after `type:` if present, otherwise after `id:`.
    let anchor = if fm_text.lines().any(|l| l.trim_start().starts_with("type:")) {
        "type:"
    } else {
        "id:"
    };
    let new_fm = insert_field_after(&fm_text, anchor, "status: captured");
    let new_content = format!("---\n{new_fm}\n---\n{body}");
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
    let Some(node) = graph.nodes.get(&check.artifact_id) else {
        return Ok(None);
    };

    let file_path = project_path.join(&node.path);
    if !file_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let (fm_opt, body) = extract_frontmatter(&content);
    let fm_text = match fm_opt {
        Some(t) => t,
        None => return Ok(None),
    };

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text).map_err(|e| {
        ValidationError::Validation(format!("YAML parse error in {}: {e}", node.path))
    })?;

    let Some(rels) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    else {
        return Ok(None);
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

    if removed == 0 {
        return Ok(None);
    }

    // Rebuild the YAML with deduplicated relationships.
    let mut new_yaml = yaml_value.clone();
    if let Some(map) = new_yaml.as_mapping_mut() {
        map.insert(
            serde_yaml::Value::String("relationships".to_owned()),
            serde_yaml::Value::Sequence(deduped),
        );
    }

    let new_fm = serde_yaml::to_string(&new_yaml)
        .map_err(|e| ValidationError::Validation(format!("YAML serialization error: {e}")))?;
    // serde_yaml adds a leading `---\n` in some versions; normalise it away.
    let new_fm = new_fm
        .trim_start_matches("---\n")
        .trim_end_matches('\n')
        .to_owned();

    let new_content = format!("---\n{new_fm}\n---\n{body}");
    std::fs::write(&file_path, new_content)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    Ok(Some(AppliedFix {
        artifact_id: check.artifact_id.clone(),
        description: format!(
            "Removed {removed} duplicate relationship entries from {}",
            node.path
        ),
        file_path: node.path.clone(),
    }))
}

/// Insert a new field line immediately after the first line that starts with `anchor_prefix`.
///
/// If no such anchor line is found, the new field is prepended before all other content.
fn insert_field_after(fm_text: &str, anchor_prefix: &str, new_field: &str) -> String {
    let lines: Vec<&str> = fm_text.lines().collect();
    let anchor_pos = lines
        .iter()
        .position(|l| l.trim_start().starts_with(anchor_prefix));

    match anchor_pos {
        Some(pos) => {
            let mut result = lines[..=pos].to_vec();
            result.push(new_field);
            result.extend_from_slice(&lines[pos + 1..]);
            result.join("\n")
        }
        None => format!("{new_field}\n{}", fm_text),
    }
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
