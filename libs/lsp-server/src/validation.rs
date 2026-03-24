//! LSP adapter for OrqaStudio file validation.
//!
//! This module is a **thin protocol adapter**. All validation logic lives in
//! `orqa_validation::checks::file_level` (the shared engine). This module:
//!
//! 1. Calls the shared engine's `validate_file` to get `FileFinding` values.
//! 2. Converts each `FileFinding` to an LSP `Diagnostic`.
//! 3. Provides graph-level checks via the validation daemon (LSP-specific).
//!
//! The CLI (`orqa check`) calls the same shared engine directly and formats
//! findings as text — no LSP `Diagnostic` conversion needed there.

use std::path::Path;

use orqa_validation::checks::file_level::{self, FileFinding, FileSeverity};
use orqa_validation::graph::ArtifactGraph;
use orqa_validation::platform::ArtifactTypeDef;
use orqa_validation::types::{IntegrityCategory, IntegritySeverity};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

// ---------------------------------------------------------------------------
// File-level adapter (shared engine → LSP Diagnostic)
// ---------------------------------------------------------------------------

/// Validate a single artifact file and return LSP diagnostics.
///
/// Delegates to `orqa_validation::checks::file_level::validate_file` and
/// converts each [`FileFinding`] to a [`Diagnostic`].
pub fn validate_file(
    rel_path: &str,
    content: &str,
    graph: Option<&ArtifactGraph>,
    artifact_types: &[ArtifactTypeDef],
) -> Vec<Diagnostic> {
    file_level::validate_file(rel_path, content, graph, artifact_types)
        .into_iter()
        .map(finding_to_diagnostic)
        .collect()
}

/// Convert a [`FileFinding`] from the shared engine to an LSP [`Diagnostic`].
fn finding_to_diagnostic(f: FileFinding) -> Diagnostic {
    Diagnostic {
        range: Range::new(
            Position::new(f.line, f.col_start),
            Position::new(f.line, f.col_end),
        ),
        severity: Some(match f.severity {
            FileSeverity::Error => DiagnosticSeverity::ERROR,
            FileSeverity::Warning => DiagnosticSeverity::WARNING,
            FileSeverity::Info => DiagnosticSeverity::INFORMATION,
        }),
        source: Some("orqastudio".into()),
        message: f.message,
        ..Default::default()
    }
}

// Re-export ID helpers from the shared engine for backwards compatibility.
pub use file_level::{is_hex_artifact_id, is_valid_artifact_id};

// ---------------------------------------------------------------------------
// Graph-level checks (delegated to orqa-validation)
// ---------------------------------------------------------------------------

/// Run comprehensive graph-level integrity checks via `orqa_validation`.
///
/// Builds the full artifact graph from disk and runs all schema-driven
/// integrity checks (broken refs, missing inverses, type constraints,
/// cardinality, cycles). Findings that reference `artifact_id` are converted
/// to LSP `Diagnostic` values anchored to line 1 of the file.
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

    let Ok(graph) = orqa_validation::build_artifact_graph(project_root) else {
        return Vec::new();
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
fn integrity_check_to_diagnostic(check: orqa_validation::types::IntegrityCheck) -> Diagnostic {
    let severity = match check.severity {
        IntegritySeverity::Error => DiagnosticSeverity::ERROR,
        IntegritySeverity::Warning => DiagnosticSeverity::WARNING,
        IntegritySeverity::Info => DiagnosticSeverity::INFORMATION,
    };

    let category_label = match check.category {
        IntegrityCategory::BrokenLink => "[broken-link]",
        IntegrityCategory::MissingInverse => "[missing-inverse]",
        IntegrityCategory::TypeConstraintViolation => "[type-constraint]",
        IntegrityCategory::RequiredRelationshipMissing => "[required-relationship]",
        IntegrityCategory::CardinalityViolation => "[cardinality]",
        IntegrityCategory::CircularDependency => "[circular-dep]",
        IntegrityCategory::InvalidStatus => "[invalid-status]",
        IntegrityCategory::TypePrefixMismatch => "[type-prefix-mismatch]",
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
        use std::fmt::Write;
        let _ = write!(message, " (auto-fix: {fix_desc})");
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
// Tests — adapter-level tests only. Shared engine tests live in
// orqa_validation::checks::file_level.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_to_diagnostic_error() {
        let f = FileFinding {
            line: 5,
            col_start: 0,
            col_end: 10,
            severity: FileSeverity::Error,
            message: "test error".into(),
        };
        let d = finding_to_diagnostic(f);
        assert_eq!(d.range.start.line, 5);
        assert_eq!(d.range.start.character, 0);
        assert_eq!(d.range.end.character, 10);
        assert_eq!(d.severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(d.message, "test error");
        assert_eq!(d.source, Some("orqastudio".into()));
    }

    #[test]
    fn finding_to_diagnostic_warning() {
        let f = FileFinding {
            line: 3,
            col_start: 2,
            col_end: 8,
            severity: FileSeverity::Warning,
            message: "test warning".into(),
        };
        let d = finding_to_diagnostic(f);
        assert_eq!(d.severity, Some(DiagnosticSeverity::WARNING));
    }

    #[test]
    fn finding_to_diagnostic_info() {
        let f = FileFinding {
            line: 0,
            col_start: 0,
            col_end: 3,
            severity: FileSeverity::Info,
            message: "test info".into(),
        };
        let d = finding_to_diagnostic(f);
        assert_eq!(d.severity, Some(DiagnosticSeverity::INFORMATION));
    }

    #[test]
    fn validate_file_delegates_to_shared_engine() {
        // Non-.orqa file → no diagnostics (engine returns empty, adapter passes through)
        let diagnostics = validate_file("src/main.rs", "fn main() {}", None, &[]);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn validate_file_converts_findings() {
        // Missing frontmatter → engine returns a finding, adapter converts to Diagnostic
        let content = "# No frontmatter\n";
        let diagnostics = validate_file(".orqa/delivery/epics/EPIC-001.md", content, None, &[]);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("frontmatter"));
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    }
}
