//! LSP adapter for OrqaStudio file validation.
//!
//! This module is a **thin protocol adapter**. All validation logic lives in
//! the shared engine (`orqa_engine::validation::checks::file_level`). This module:
//!
//! 1. Calls the shared engine's `validate_file` to get `FileFinding` values.
//! 2. Converts each `FileFinding` to an LSP `Diagnostic`.
//!
//! Graph-level checks are delegated to the orqa-daemon HTTP API by `server.rs`.
//! The CLI (`orqa check`) calls the same shared engine directly and formats
//! findings as text — no LSP `Diagnostic` conversion needed there.

use orqa_engine::validation::checks::file_level::{self, FileFinding, FileSeverity};
use orqa_engine::graph::ArtifactGraph;
use orqa_engine::validation::ArtifactTypeDef;
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

// Re-export ID helpers from the shared engine.
pub use file_level::{is_hex_artifact_id, is_valid_artifact_id};

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
        let diagnostics = validate_file(".orqa/implementation/epics/EPIC-001.md", content, None, &[]);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("frontmatter"));
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    }
}
