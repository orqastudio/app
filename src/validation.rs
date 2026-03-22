//! Frontmatter validation logic for OrqaStudio artifact files.
//!
//! Produces LSP `Diagnostic` values for `.orqa/` markdown files:
//!
//! ## File-level checks (single file, fast, always run)
//! - Missing YAML frontmatter delimiters
//! - Missing required `id` field
//! - JSON Schema validation (types, patterns, enums, required fields)
//! - Invalid or legacy artifact ID format
//! - Duplicate frontmatter keys
//! - Knowledge artifact missing `synchronised-with` relationship
//! - Relationship targets not found in the artifact graph
//! - Missing `relationships` section on delivery/process artifacts
//!
//! ## Graph-level checks (full graph, run when graph is available)
//! Delegated to [`orqa_validation::validate()`] — covers:
//! - Broken references (targets that don't exist)
//! - Missing inverse relationships
//! - Type constraints (from/to type mismatches)
//! - Cardinality violations (min/max count constraints)
//! - Circular dependencies
//! - Body text references without relationship edges
//! - Parent/child status inconsistencies

use std::collections::HashMap;
use std::path::Path;

use orqa_validation::checks::schema::{build_frontmatter_schema, validate_frontmatter};
use orqa_validation::platform::ArtifactTypeDef;
use orqa_validation::types::{IntegrityCategory, IntegritySeverity};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::types::ArtifactGraph;

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
/// `artifact_types` provides JSON Schema definitions for frontmatter validation.
/// When empty, schema validation is skipped (only structural checks run).
pub fn validate_file(
    rel_path: &str,
    content: &str,
    graph: Option<&ArtifactGraph>,
    artifact_types: &[ArtifactTypeDef],
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

    check_required_id(has_id, content, &mut diagnostics);
    check_artifact_id(has_id, content, &mut diagnostics);
    check_duplicate_keys(content, &mut diagnostics);
    check_json_schema(content, frontmatter, artifact_types, &mut diagnostics);
    check_knowledge_sync(rel_path, frontmatter, content, &mut diagnostics);
    check_relationship_targets(graph, content, &mut diagnostics);
    check_missing_relationships(rel_path, frontmatter, content, &mut diagnostics);

    diagnostics
}

fn parse_frontmatter<'a>(content: &'a str, diagnostics: &mut Vec<Diagnostic>) -> Option<&'a str> {
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

/// Validate frontmatter against JSON Schema derived from plugin artifact type definitions.
///
/// Parses the YAML frontmatter as JSON, matches the artifact to its type by ID prefix,
/// builds the enriched JSON Schema, and validates. Schema errors are mapped to LSP
/// diagnostics with line-level positioning where possible.
fn check_json_schema(
    content: &str,
    frontmatter_str: &str,
    artifact_types: &[ArtifactTypeDef],
    diagnostics: &mut Vec<Diagnostic>,
) {
    if artifact_types.is_empty() {
        return;
    }

    // Parse frontmatter YAML into a JSON value.
    let frontmatter: serde_json::Value = match serde_yaml::from_str(frontmatter_str) {
        Ok(v) => v,
        Err(_) => return, // YAML parse errors are caught elsewhere
    };

    // Extract ID to determine artifact type.
    let id = match frontmatter.get("id").and_then(serde_json::Value::as_str) {
        Some(id) => id,
        None => return, // No ID — can't match to a schema
    };

    // Match by ID prefix (everything before the first '-').
    let prefix = match id.split('-').next() {
        Some(p) if !p.is_empty() => p,
        _ => return,
    };

    // Build prefix → type_def lookup.
    let type_map: HashMap<&str, &ArtifactTypeDef> = artifact_types
        .iter()
        .map(|t| (t.id_prefix.as_str(), t))
        .collect();

    let Some(type_def) = type_map.get(prefix) else {
        return; // Unknown type — no schema to validate against
    };

    // Build and validate against the enriched JSON Schema.
    let schema = build_frontmatter_schema(type_def);
    let errors = validate_frontmatter(&frontmatter, &schema);

    for error in errors {
        // Try to find the line number for the offending field.
        let field_name = error
            .path
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("");

        let (line, line_len) = find_field_line(content, field_name);

        diagnostics.push(Diagnostic {
            range: Range::new(
                Position::new(line, 0),
                Position::new(line, line_len),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("orqastudio".into()),
            message: format!("[schema] {}", error.message),
            ..Default::default()
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
    // Fallback: use the closing --- line
    let end = find_frontmatter_end_line(content);
    (end, 3)
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
    let mut seen_keys: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
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
// Graph-level checks (delegated to orqa-validation)
// ---------------------------------------------------------------------------

/// Run comprehensive graph-level integrity checks via `orqa_validation`.
///
/// This function builds the full artifact graph from disk and runs all
/// schema-driven integrity checks (broken refs, missing inverses, type
/// constraints, cardinality, cycles). Findings that reference `artifact_id`
/// are converted to LSP `Diagnostic` values anchored to line 1 of the file.
///
/// Returns an empty vec when:
/// - The graph cannot be built (directory missing, IO error)
/// - The validation context cannot be constructed
/// - No checks reference this artifact
///
/// `artifact_id` is extracted from the frontmatter `id:` field by the caller.
/// When it is `None` (no id yet), this function returns no diagnostics because
/// the graph-level checks all require a valid artifact ID to match against.
pub fn validate_graph_checks(project_root: &Path, artifact_id: Option<&str>) -> Vec<Diagnostic> {
    let Some(artifact_id) = artifact_id else {
        return Vec::new();
    };

    let graph = match orqa_validation::build_artifact_graph(project_root) {
        Ok(g) => g,
        Err(_) => return Vec::new(),
    };

    let plugin_contributions = orqa_validation::platform::scan_plugin_manifests(project_root);
    let ctx = orqa_validation::build_validation_context(
        &[],
        &orqa_validation::settings::DeliveryConfig::default(),
        &[],
        &plugin_contributions.relationships,
    );

    let checks = orqa_validation::validate(&graph, &ctx);

    checks
        .into_iter()
        .filter(|c| c.artifact_id == artifact_id)
        .map(integrity_check_to_diagnostic)
        .collect()
}

/// Convert an [`orqa_validation::IntegrityCheck`] to an LSP `Diagnostic`.
///
/// Graph-level findings are not tied to a specific line — they are anchored to
/// the opening frontmatter delimiter (line 0, column 0–3) so the editor shows
/// them at the top of the file.
fn integrity_check_to_diagnostic(
    check: orqa_validation::types::IntegrityCheck,
) -> Diagnostic {
    let severity = match check.severity {
        IntegritySeverity::Error => DiagnosticSeverity::ERROR,
        IntegritySeverity::Warning => DiagnosticSeverity::WARNING,
        IntegritySeverity::Info => DiagnosticSeverity::INFORMATION,
    };

    // Annotate the category in the message for clarity.
    let category_label = match check.category {
        IntegrityCategory::BrokenLink => "[broken-link]",
        IntegrityCategory::MissingInverse => "[missing-inverse]",
        IntegrityCategory::TypeConstraintViolation => "[type-constraint]",
        IntegrityCategory::RequiredRelationshipMissing => "[required-relationship]",
        IntegrityCategory::CardinalityViolation => "[cardinality]",
        IntegrityCategory::CircularDependency => "[circular-dep]",
        IntegrityCategory::InvalidStatus => "[invalid-status]",
        IntegrityCategory::BodyTextRefWithoutRelationship => "[body-ref]",
        IntegrityCategory::ParentChildInconsistency => "[parent-child]",
        IntegrityCategory::DeliveryPathMismatch => "[delivery-path]",
        IntegrityCategory::MissingType => "[missing-type]",
        IntegrityCategory::MissingStatus => "[missing-status]",
        IntegrityCategory::DuplicateRelationship => "[duplicate-relationship]",
        IntegrityCategory::FilenameMismatch => "[filename-mismatch]",
        IntegrityCategory::SchemaViolation => "[schema-violation]",
    };

    let mut message = format!("{category_label} {}", check.message);
    if let Some(fix_desc) = check.fix_description {
        message.push_str(&format!(" (auto-fix: {fix_desc})"));
    }

    Diagnostic {
        range: Range::new(Position::new(0, 0), Position::new(0, 3)),
        severity: Some(severity),
        source: Some("orqastudio".into()),
        message,
        ..Default::default()
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
    fn no_diagnostics_for_non_orqa_file() {
        let diagnostics = validate_file("src/main.rs", "fn main() {}", None, &[]);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn error_on_missing_frontmatter() {
        let content = "# No frontmatter\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("frontmatter"));
    }

    #[test]
    fn error_on_missing_id() {
        let content = "---\ntitle: My Epic\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("Missing required frontmatter field: id")));
    }

    #[test]
    fn schema_catches_invalid_status() {
        let types = vec![make_epic_type()];
        let content = "---\nid: EPIC-deadbeef\nstatus: wip\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &types);
        assert!(
            diagnostics.iter().any(|d| d.message.contains("[schema]")),
            "Expected schema validation error for invalid status, got: {diagnostics:?}"
        );
    }

    #[test]
    fn no_schema_error_for_valid_status() {
        let types = vec![make_epic_type()];
        let content = "---\nid: EPIC-deadbeef\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &types);
        assert!(
            !diagnostics.iter().any(|d| d.message.contains("[schema]")),
            "Expected no schema errors, got: {diagnostics:?}"
        );
    }

    #[test]
    fn warning_for_legacy_sequential_id() {
        let content = "---\nid: EPIC-001\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("Legacy sequential ID")));
    }

    #[test]
    fn no_id_warning_for_hex_id() {
        let content = "---\nid: EPIC-deadbeef\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(!diagnostics
            .iter()
            .any(|d| d.message.contains("Legacy sequential ID")));
    }

    #[test]
    fn error_on_duplicate_frontmatter_key() {
        let content = "---\nid: EPIC-001\nstatus: active\nstatus: completed\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("Duplicate frontmatter key")));
    }

    #[test]
    fn knowledge_artifact_missing_synchronised_with() {
        let content = "---\nid: KNOW-001\ntype: knowledge\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/process/knowledge/KNOW-001.md", content, None, &[]);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("synchronised-with")));
    }

    #[test]
    fn info_on_missing_relationships_for_delivery() {
        let content = "---\nid: EPIC-001\nstatus: active\n---\n# Body\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert!(diagnostics
            .iter()
            .any(|d| d.message.contains("No relationships declared")));
    }
}
