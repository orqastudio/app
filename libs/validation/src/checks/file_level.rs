//! File-level validation checks for individual artifact files.
//!
//! These checks operate on raw file content (the editor buffer or a file read
//! from disk) rather than the full artifact graph. They are fast enough to run
//! on every keystroke in an editor.
//!
//! Both the LSP server and the CLI `orqa check` command call into this module.
//! The LSP wraps findings as `Diagnostic`; the CLI formats them as text.
//!
//! ## Checks
//!
//! - Missing or unclosed YAML frontmatter delimiters
//! - Invalid or legacy artifact ID format
//! - Duplicate frontmatter keys
//! - JSON Schema validation (types, patterns, enums, required fields)
//! - Knowledge artifact missing `synchronised-with` relationship
//! - Relationship targets not found in the artifact graph
//! - Missing `relationships` section on delivery/process artifacts

use std::collections::HashMap;

use crate::checks::schema::{build_frontmatter_schema, validate_frontmatter};
use crate::graph::ArtifactGraph;
use crate::platform::ArtifactTypeDef;

// ---------------------------------------------------------------------------
// Finding types
// ---------------------------------------------------------------------------

/// Severity of a file-level finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSeverity {
    Error,
    Warning,
    Info,
}

/// A single finding from file-level validation.
///
/// Carries line/column information for editor positioning. Both the LSP and
/// CLI consume these — the LSP converts them to `Diagnostic`, the CLI to text.
#[derive(Debug, Clone)]
pub struct FileFinding {
    /// Start line (0-based).
    pub line: u32,
    /// Start column (0-based).
    pub col_start: u32,
    /// End column (0-based, exclusive).
    pub col_end: u32,
    /// Finding severity.
    pub severity: FileSeverity,
    /// Human-readable message. Prefixed with a category tag (e.g. `[schema]`).
    pub message: String,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Validate a single artifact file and return findings.
///
/// Only `.orqa/` markdown files are validated. Other files receive no findings.
///
/// - `rel_path`: path relative to the project root, with forward slashes.
/// - `content`: the full file text (may be an unsaved editor buffer).
/// - `graph`: when `Some`, relationship target checks run. `None` skips them.
/// - `artifact_types`: plugin-contributed type definitions for schema validation.
///   When empty, schema validation is skipped.
pub fn validate_file(
    rel_path: &str,
    content: &str,
    graph: Option<&ArtifactGraph>,
    artifact_types: &[ArtifactTypeDef],
) -> Vec<FileFinding> {
    let mut findings = Vec::new();

    if !rel_path.starts_with(".orqa/") || !rel_path.to_ascii_lowercase().ends_with(".md") {
        return findings;
    }

    let Some(frontmatter) = parse_frontmatter(content, &mut findings) else {
        return findings;
    };

    check_artifact_id(frontmatter, content, &mut findings);
    check_duplicate_keys(content, &mut findings);
    check_json_schema(content, frontmatter, artifact_types, &mut findings);
    check_knowledge_sync(rel_path, frontmatter, content, &mut findings);
    check_relationship_targets(graph, content, &mut findings);
    check_missing_relationships(rel_path, frontmatter, content, &mut findings);

    findings
}

// ---------------------------------------------------------------------------
// ID validation helpers (public for reuse)
// ---------------------------------------------------------------------------

/// Validate that an artifact ID matches the expected format.
///
/// Accepts both legacy sequential IDs (`TYPE-NNN`) and new hex IDs (`TYPE-XXXXXXXX`).
pub fn is_valid_artifact_id(id: &str) -> bool {
    let Some((prefix, suffix)) = id.split_once('-') else {
        return false;
    };
    if prefix.is_empty() || !prefix.chars().all(|c| c.is_ascii_uppercase()) {
        return id.rmatch_indices('-').next().is_some_and(|(i, _)| {
            let final_suffix = &id[i + 1..];
            let prefix_part = &id[..i];
            !prefix_part.is_empty()
                && prefix_part
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c == '-')
                && (final_suffix.chars().all(|c| c.is_ascii_digit())
                    || (final_suffix.len() == 8
                        && final_suffix.chars().all(|c| c.is_ascii_hexdigit())))
        });
    }
    suffix.chars().all(|c| c.is_ascii_digit())
        || (suffix.len() == 8 && suffix.chars().all(|c| c.is_ascii_hexdigit()))
}

/// Check if an artifact ID uses the new hex format (`TYPE-XXXXXXXX`).
pub fn is_hex_artifact_id(id: &str) -> bool {
    let Some((_prefix, suffix)) = id.split_once('-') else {
        return false;
    };
    suffix.len() == 8 && suffix.chars().all(|c| c.is_ascii_hexdigit())
}

// ---------------------------------------------------------------------------
// Internal checks
// ---------------------------------------------------------------------------

/// Find the line number of the closing `---` of the YAML frontmatter block.
///
/// Returns 1 as a safe fallback if the block cannot be found.
fn find_frontmatter_end_line(content: &str) -> u32 {
    let mut in_fm = false;
    for (count, line) in content.lines().enumerate() {
        if line == "---" {
            if in_fm {
                return count as u32;
            }
            in_fm = true;
        }
    }
    1
}

/// Parse frontmatter delimiters. Returns the frontmatter text slice on success,
/// or pushes a finding and returns `None` if the block is missing or unclosed.
fn parse_frontmatter<'a>(content: &'a str, findings: &mut Vec<FileFinding>) -> Option<&'a str> {
    if content.find("---\n") != Some(0) {
        findings.push(FileFinding {
            line: 0,
            col_start: 0,
            col_end: 3,
            severity: FileSeverity::Error,
            message: "Missing YAML frontmatter (must start with ---)".into(),
        });
        return None;
    }
    let Some(fm_end) = content[4..].find("\n---") else {
        findings.push(FileFinding {
            line: 0,
            col_start: 0,
            col_end: 3,
            severity: FileSeverity::Error,
            message: "Unclosed YAML frontmatter (missing closing ---)".into(),
        });
        return None;
    };
    Some(&content[4..fm_end + 4])
}

/// Validate frontmatter against JSON Schema derived from plugin artifact type definitions.
fn check_json_schema(
    content: &str,
    frontmatter_str: &str,
    artifact_types: &[ArtifactTypeDef],
    findings: &mut Vec<FileFinding>,
) {
    if artifact_types.is_empty() {
        return;
    }

    let frontmatter: serde_json::Value = match serde_yaml::from_str(frontmatter_str) {
        Ok(v) => v,
        Err(_) => return, // YAML parse errors are caught elsewhere
    };

    let Some(id) = frontmatter.get("id").and_then(serde_json::Value::as_str) else {
        return;
    };

    let prefix = match id.split('-').next() {
        Some(p) if !p.is_empty() => p,
        _ => return,
    };

    let type_map: HashMap<&str, &ArtifactTypeDef> = artifact_types
        .iter()
        .map(|t| (t.id_prefix.as_str(), t))
        .collect();

    let Some(type_def) = type_map.get(prefix) else {
        return;
    };

    let schema = build_frontmatter_schema(type_def);
    let errors = validate_frontmatter(&frontmatter, &schema);

    for error in errors {
        let field_name = error
            .path
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("");

        let (line, line_len) = find_field_line(content, field_name);

        findings.push(FileFinding {
            line,
            col_start: 0,
            col_end: line_len,
            severity: FileSeverity::Error,
            message: format!("[schema] {}", error.message),
        });
    }
}

/// Find the line number of a frontmatter field in the file content.
/// Returns (line_number, line_length). Falls back to the closing `---` line.
fn find_field_line(content: &str, field_name: &str) -> (u32, u32) {
    if !field_name.is_empty() {
        let search = format!("{field_name}:");
        for (i, line) in content.lines().enumerate() {
            if line.starts_with(&search) {
                return (i as u32, line.len() as u32);
            }
        }
    }
    let end = find_frontmatter_end_line(content);
    (end, 3)
}

/// Validate the `id:` field format if present.
fn check_artifact_id(frontmatter: &str, content: &str, findings: &mut Vec<FileFinding>) {
    let has_id = frontmatter.lines().any(|l| l.starts_with("id:"));
    if !has_id {
        return;
    }
    for (i, line) in content.lines().enumerate() {
        if line.starts_with("id:") {
            let id = line.trim_start_matches("id:").trim().trim_matches('"');
            if !is_valid_artifact_id(id) {
                findings.push(FileFinding {
                    line: i as u32,
                    col_start: 0,
                    col_end: line.len() as u32,
                    severity: FileSeverity::Error,
                    message: format!(
                        "Invalid artifact ID \"{id}\" — must be TYPE-XXXXXXXX (8 hex chars)"
                    ),
                });
            } else if !is_hex_artifact_id(id) {
                findings.push(FileFinding {
                    line: i as u32,
                    col_start: 0,
                    col_end: line.len() as u32,
                    severity: FileSeverity::Warning,
                    message: format!(
                        "Legacy sequential ID \"{id}\" — new artifacts should use TYPE-XXXXXXXX hex format (AD-057)"
                    ),
                });
            }
            break;
        }
    }
}

/// Check for duplicate frontmatter keys.
fn check_duplicate_keys(content: &str, findings: &mut Vec<FileFinding>) {
    let mut seen_keys: HashMap<String, u32> = HashMap::new();
    for (i, line) in content.lines().enumerate() {
        if line == "---" {
            if seen_keys.is_empty() {
                continue; // opening ---
            }
            break; // closing ---
        }
        if let Some(key) = line.split(':').next() {
            let key = key.trim().to_string();
            if !key.is_empty() && !key.starts_with('-') && !key.starts_with(' ') {
                if let Some(&first_line) = seen_keys.get(&key) {
                    findings.push(FileFinding {
                        line: i as u32,
                        col_start: 0,
                        col_end: line.len() as u32,
                        severity: FileSeverity::Error,
                        message: format!(
                            "Duplicate frontmatter key \"{key}\" (first seen on line {})",
                            first_line + 1
                        ),
                    });
                } else {
                    seen_keys.insert(key, i as u32);
                }
            }
        }
    }
}

/// Check that knowledge artifacts have a `synchronised-with` relationship.
fn check_knowledge_sync(
    rel_path: &str,
    frontmatter: &str,
    content: &str,
    findings: &mut Vec<FileFinding>,
) {
    let is_knowledge = frontmatter
        .lines()
        .any(|l| l.trim().starts_with("type:") && l.contains("knowledge"))
        || rel_path.contains("/knowledge/");

    if is_knowledge && !frontmatter.contains("synchronised-with") {
        let line_num = find_frontmatter_end_line(content);
        findings.push(FileFinding {
            line: line_num,
            col_start: 0,
            col_end: 3,
            severity: FileSeverity::Error,
            message: "Knowledge artifacts must have at least one synchronised-with relationship to a human-facing doc (AD-058)".into(),
        });
    }
}

/// Check that relationship targets exist in the artifact graph.
fn check_relationship_targets(
    graph: Option<&ArtifactGraph>,
    content: &str,
    findings: &mut Vec<FileFinding>,
) {
    let Some(graph) = graph else { return };
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("- target:") {
            let target = trimmed
                .trim_start_matches("- target:")
                .trim()
                .trim_matches('"');
            if !target.is_empty() && !graph.nodes.contains_key(target) {
                findings.push(FileFinding {
                    line: i as u32,
                    col_start: 0,
                    col_end: line.len() as u32,
                    severity: FileSeverity::Warning,
                    message: format!("Relationship target \"{target}\" not found in graph"),
                });
            }
        }
    }
}

/// Check that delivery/process artifacts have a relationships section.
fn check_missing_relationships(
    rel_path: &str,
    frontmatter: &str,
    content: &str,
    findings: &mut Vec<FileFinding>,
) {
    if (rel_path.starts_with(".orqa/delivery/") || rel_path.starts_with(".orqa/process/"))
        && !frontmatter.contains("relationships:")
    {
        let line_num = find_frontmatter_end_line(content);
        findings.push(FileFinding {
            line: line_num,
            col_start: 0,
            col_end: 3,
            severity: FileSeverity::Info,
            message: "No relationships declared — most delivery/process artifacts should have at least one".into(),
        });
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_epic_type() -> ArtifactTypeDef {
        let mut transitions = std::collections::HashMap::new();
        transitions.insert("captured".to_owned(), vec!["active".to_owned()]);
        transitions.insert("active".to_owned(), vec!["completed".to_owned()]);
        transitions.insert("completed".to_owned(), vec![]);
        ArtifactTypeDef {
            key: "epic".to_owned(),
            label: "Epic".to_owned(),
            icon: "layers".to_owned(),
            id_prefix: "EPIC".to_owned(),
            frontmatter_schema: serde_json::json!({
                "type": "object",
                "required": ["id", "status"],
                "properties": {
                    "id": { "type": "string", "pattern": "^EPIC-[a-f0-9]{8}$" }
                },
                "additionalProperties": true
            }),
            status_transitions: transitions,
        }
    }

    #[test]
    fn valid_artifact_id_hex() {
        assert!(is_valid_artifact_id("EPIC-1a2b3c4d"));
        assert!(is_valid_artifact_id("TASK-deadbeef"));
        assert!(is_valid_artifact_id("RULE-00000000"));
    }

    #[test]
    fn valid_artifact_id_sequential() {
        assert!(is_valid_artifact_id("EPIC-001"));
        assert!(is_valid_artifact_id("TASK-123"));
        assert!(is_valid_artifact_id("RULE-006"));
    }

    #[test]
    fn invalid_artifact_ids() {
        assert!(!is_valid_artifact_id("EPIC"));
        assert!(!is_valid_artifact_id("epic-001"));
        assert!(!is_valid_artifact_id("EPIC-xyz"));
        assert!(!is_valid_artifact_id(""));
    }

    #[test]
    fn hex_artifact_id_detection() {
        assert!(is_hex_artifact_id("EPIC-1a2b3c4d"));
        assert!(!is_hex_artifact_id("EPIC-001"));
        assert!(!is_hex_artifact_id("EPIC-123"));
    }

    #[test]
    fn no_findings_for_non_orqa_file() {
        let findings = validate_file("src/main.rs", "fn main() {}", None, &[]);
        assert!(findings.is_empty());
    }

    #[test]
    fn error_on_missing_frontmatter() {
        let content = "# No frontmatter\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("frontmatter"));
    }

    #[test]
    fn no_missing_id_error_without_schema() {
        let content = "---\ntitle: My Epic\nstatus: active\n---\n# Body\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(!findings
            .iter()
            .any(|f| f.message.contains("Missing required frontmatter field")));
    }

    #[test]
    fn schema_catches_invalid_status() {
        let types = vec![make_epic_type()];
        let content = "---\nid: EPIC-deadbeef\nstatus: wip\n---\n# Body\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &types);
        assert!(
            findings.iter().any(|f| f.message.contains("[schema]")),
            "Expected schema validation error for invalid status, got: {findings:?}"
        );
    }

    #[test]
    fn no_schema_error_for_valid_status() {
        let types = vec![make_epic_type()];
        let content = "---\nid: EPIC-deadbeef\nstatus: active\n---\n# Body\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &types);
        assert!(
            !findings.iter().any(|f| f.message.contains("[schema]")),
            "Expected no schema errors, got: {findings:?}"
        );
    }

    #[test]
    fn warning_for_legacy_sequential_id() {
        let content = "---\nid: EPIC-001\nstatus: active\n---\n# Body\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(findings
            .iter()
            .any(|f| f.message.contains("Legacy sequential ID")));
    }

    #[test]
    fn no_id_warning_for_hex_id() {
        let content = "---\nid: EPIC-deadbeef\nstatus: active\n---\n# Body\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(!findings
            .iter()
            .any(|f| f.message.contains("Legacy sequential ID")));
    }

    #[test]
    fn error_on_duplicate_frontmatter_key() {
        let content = "---\nid: EPIC-001\nstatus: active\nstatus: completed\n---\n# Body\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(findings
            .iter()
            .any(|f| f.message.contains("Duplicate frontmatter key")));
    }

    #[test]
    fn knowledge_artifact_missing_synchronised_with() {
        let content = "---\nid: KNOW-001\ntype: knowledge\n---\n# Body\n";
        let findings =
            validate_file(".orqa/process/knowledge/KNOW-001.md", content, None, &[]);
        assert!(findings
            .iter()
            .any(|f| f.message.contains("synchronised-with")));
    }

    #[test]
    fn info_on_missing_relationships_for_delivery() {
        let content = "---\nid: EPIC-001\nstatus: active\n---\n# Body\n";
        let findings = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(findings
            .iter()
            .any(|f| f.message.contains("No relationships declared")));
    }
}
