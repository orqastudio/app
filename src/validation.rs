//! Frontmatter validation logic for OrqaStudio artifact files.
//!
//! Produces LSP `Diagnostic` values for `.orqa/` markdown files:
//! - Missing YAML frontmatter delimiters
//! - Missing required `id` field
//! - Invalid `status` value (against 12 canonical statuses)
//! - Invalid or legacy artifact ID format
//! - Duplicate frontmatter keys
//! - Knowledge artifact missing `synchronised-with` relationship
//! - Relationship targets not found in the artifact graph
//! - Missing `relationships` section on delivery/process artifacts

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::types::ArtifactGraph;

/// The 12 canonical artifact statuses.
pub const VALID_STATUSES: &[&str] = &[
    "captured",
    "exploring",
    "ready",
    "prioritised",
    "active",
    "hold",
    "blocked",
    "review",
    "completed",
    "surpassed",
    "archived",
    "recurring",
];

/// Validate that an artifact ID matches the expected format.
///
/// Accepts both legacy sequential IDs (`TYPE-NNN`) and new hex IDs (`TYPE-XXXXXXXX`).
pub fn is_valid_artifact_id(id: &str) -> bool {
    let Some((prefix, suffix)) = id.split_once('-') else {
        return false;
    };
    if prefix.is_empty() || !prefix.chars().all(|c| c.is_ascii_uppercase()) {
        return id
            .rmatch_indices('-')
            .next()
            .is_some_and(|(i, _)| {
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

/// Validate a single artifact file and return LSP diagnostics.
///
/// Only `.orqa/` markdown files are validated. Other files receive no diagnostics.
///
/// `rel_path` must be the path relative to the project root, with forward slashes.
/// `content` is the full file text.
/// `graph` is `None` when the graph hasn't been built yet (relationship target
/// checks are skipped in that case).
pub fn validate_file(
    rel_path: &str,
    content: &str,
    graph: Option<&ArtifactGraph>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if !rel_path.starts_with(".orqa/") || !rel_path.to_ascii_lowercase().ends_with(".md") {
        return diagnostics;
    }

    let Some(frontmatter) = parse_frontmatter(content, &mut diagnostics) else {
        return diagnostics;
    };

    let lines: Vec<&str> = frontmatter.lines().collect();
    let has_id = lines.iter().any(|l| l.starts_with("id:"));
    let has_status = lines.iter().any(|l| l.starts_with("status:"));

    check_required_id(has_id, content, &mut diagnostics);
    check_status(has_status, content, &mut diagnostics);
    check_artifact_id(has_id, content, &mut diagnostics);
    check_duplicate_keys(content, &mut diagnostics);
    check_knowledge_sync(rel_path, frontmatter, content, &mut diagnostics);
    check_relationship_targets(graph, content, &mut diagnostics);
    check_missing_relationships(rel_path, frontmatter, content, &mut diagnostics);

    diagnostics
}

fn parse_frontmatter<'a>(
    content: &'a str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<&'a str> {
    if content.find("---\n") != Some(0) {
        diagnostics.push(Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 3)),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("orqastudio".into()),
            message: "Missing YAML frontmatter (must start with ---)".into(),
            ..Default::default()
        });
        return None;
    }
    let Some(fm_end) = content[4..].find("\n---") else {
        diagnostics.push(Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 3)),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("orqastudio".into()),
            message: "Unclosed YAML frontmatter (missing closing ---)".into(),
            ..Default::default()
        });
        return None;
    };
    Some(&content[4..fm_end + 4])
}

fn check_required_id(has_id: bool, content: &str, diagnostics: &mut Vec<Diagnostic>) {
    if !has_id {
        let line_num = find_frontmatter_end_line(content);
        diagnostics.push(Diagnostic {
            range: Range::new(Position::new(line_num, 0), Position::new(line_num, 3)),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("orqastudio".into()),
            message: "Missing required frontmatter field: id".into(),
            ..Default::default()
        });
    }
}

fn check_status(has_status: bool, content: &str, diagnostics: &mut Vec<Diagnostic>) {
    if !has_status {
        return;
    }
    for (i, line) in content.lines().enumerate() {
        if line.starts_with("status:") {
            let status = line.trim_start_matches("status:").trim().trim_matches('"');
            if !VALID_STATUSES.contains(&status) {
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(i as u32, 0),
                        Position::new(i as u32, line.len() as u32),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("orqastudio".into()),
                    message: format!(
                        "Invalid status \"{status}\" — must be one of: {}",
                        VALID_STATUSES.join(", ")
                    ),
                    ..Default::default()
                });
            }
            break;
        }
    }
}

fn check_artifact_id(has_id: bool, content: &str, diagnostics: &mut Vec<Diagnostic>) {
    if !has_id {
        return;
    }
    for (i, line) in content.lines().enumerate() {
        if line.starts_with("id:") {
            let id = line.trim_start_matches("id:").trim().trim_matches('"');
            if !is_valid_artifact_id(id) {
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(i as u32, 0),
                        Position::new(i as u32, line.len() as u32),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("orqastudio".into()),
                    message: format!(
                        "Invalid artifact ID \"{id}\" — must be TYPE-XXXXXXXX (8 hex chars)"
                    ),
                    ..Default::default()
                });
            } else if !is_hex_artifact_id(id) {
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(i as u32, 0),
                        Position::new(i as u32, line.len() as u32),
                    ),
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("orqastudio".into()),
                    message: format!(
                        "Legacy sequential ID \"{id}\" — new artifacts should use TYPE-XXXXXXXX hex format (AD-057)"
                    ),
                    ..Default::default()
                });
            }
            break;
        }
    }
}

fn check_duplicate_keys(content: &str, diagnostics: &mut Vec<Diagnostic>) {
    let mut seen_keys: std::collections::HashMap<String, u32> =
        std::collections::HashMap::new();
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
                    diagnostics.push(Diagnostic {
                        range: Range::new(
                            Position::new(i as u32, 0),
                            Position::new(i as u32, line.len() as u32),
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("orqastudio".into()),
                        message: format!(
                            "Duplicate frontmatter key \"{key}\" (first seen on line {})",
                            first_line + 1
                        ),
                        ..Default::default()
                    });
                } else {
                    seen_keys.insert(key, i as u32);
                }
            }
        }
    }
}

fn check_knowledge_sync(
    rel_path: &str,
    frontmatter: &str,
    content: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let is_knowledge = frontmatter
        .lines()
        .any(|l| l.trim().starts_with("type:") && l.contains("knowledge"))
        || rel_path.contains("/knowledge/");

    if is_knowledge && !frontmatter.contains("synchronised-with") {
        let line_num = find_frontmatter_end_line(content);
        diagnostics.push(Diagnostic {
            range: Range::new(
                Position::new(line_num, 0),
                Position::new(line_num, 3),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("orqastudio".into()),
            message: "Knowledge artifacts must have at least one synchronised-with relationship to a human-facing doc (AD-058)".into(),
            ..Default::default()
        });
    }
}

fn check_relationship_targets(
    graph: Option<&ArtifactGraph>,
    content: &str,
    diagnostics: &mut Vec<Diagnostic>,
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
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(i as u32, 0),
                        Position::new(i as u32, line.len() as u32),
                    ),
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("orqastudio".into()),
                    message: format!("Relationship target \"{target}\" not found in graph"),
                    ..Default::default()
                });
            }
        }
    }
}

fn check_missing_relationships(
    rel_path: &str,
    frontmatter: &str,
    content: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if (rel_path.starts_with(".orqa/delivery/") || rel_path.starts_with(".orqa/process/"))
        && !frontmatter.contains("relationships:")
    {
        let line_num = find_frontmatter_end_line(content);
        diagnostics.push(Diagnostic {
            range: Range::new(Position::new(line_num, 0), Position::new(line_num, 3)),
            severity: Some(DiagnosticSeverity::INFORMATION),
            source: Some("orqastudio".into()),
            message: "No relationships declared — most delivery/process artifacts should have at least one".into(),
            ..Default::default()
        });
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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
    fn no_diagnostics_for_non_orqa_file() {
        let diagnostics = validate_file("src/main.rs", "fn main() {}", None);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn error_on_missing_frontmatter() {
        let content = "# No frontmatter\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("frontmatter"));
    }

    #[test]
    fn error_on_missing_id() {
        let content = "---\ntitle: My Epic\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("Missing required frontmatter field: id")));
    }

    #[test]
    fn error_on_invalid_status() {
        let content = "---\nid: EPIC-001\nstatus: wip\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("Invalid status")));
    }

    #[test]
    fn no_error_for_valid_status() {
        let content = "---\nid: EPIC-001\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert!(!diagnostics
            .iter()
            .any(|d| d.message.contains("Invalid status")));
    }

    #[test]
    fn warning_for_legacy_sequential_id() {
        let content = "---\nid: EPIC-001\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("Legacy sequential ID")));
    }

    #[test]
    fn no_id_warning_for_hex_id() {
        let content = "---\nid: EPIC-deadbeef\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert!(!diagnostics
            .iter()
            .any(|d| d.message.contains("Legacy sequential ID")));
    }

    #[test]
    fn error_on_duplicate_frontmatter_key() {
        let content = "---\nid: EPIC-001\nstatus: active\nstatus: completed\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("Duplicate frontmatter key")));
    }

    #[test]
    fn knowledge_artifact_missing_synchronised_with() {
        let content = "---\nid: KNOW-001\ntype: knowledge\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/process/knowledge/KNOW-001.md", content, None);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("synchronised-with")));
    }

    #[test]
    fn info_on_missing_relationships_for_delivery() {
        let content = "---\nid: EPIC-001\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("No relationships declared")));
    }
}
