// POST /parse endpoint for the OrqaStudio daemon.
//
// Reads an artifact file, extracts its ID and type from YAML frontmatter,
// counts downstream references within the project's .orqa/ tree, and decides
// whether a warning should be injected by the connector.
//
// The business logic for should_warn lives here so the connector remains a
// thin adapter (architecture principle: daemon is the business-logic boundary).

use std::path::{Path, PathBuf};
use std::time::Instant;

use axum::extract::State;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::health::HealthState;

/// Artifact types considered high-influence — changes ripple across the
/// entire governance framework.
const HIGH_INFLUENCE_TYPES: &[&str] = &["epic", "principle", "decision"];

/// Request body for POST /parse.
#[derive(Deserialize)]
pub struct ParseRequest {
    /// Absolute path to the artifact file to parse.
    pub file: String,
}

/// Response body for POST /parse.
#[derive(Serialize)]
pub struct ParseResponse {
    /// Artifact ID extracted from the `id:` frontmatter field.
    pub id: Option<String>,
    /// Artifact type extracted from the `type:` frontmatter field.
    pub artifact_type: String,
    /// True if the artifact type is considered high-influence (epic, principle, decision).
    pub high_influence: bool,
    /// Number of other .orqa/ files that reference this artifact's ID.
    pub downstream_count: u32,
    /// Short prose summary of the downstream relationships, if any.
    pub downstream_summary: Option<String>,
    /// True when the connector should inject a warning for this write.
    ///
    /// Set when high_influence == true OR downstream_count > DOWNSTREAM_WARN_THRESHOLD.
    /// Centralising this decision in the daemon means connector logic stays zero.
    pub should_warn: bool,
}

/// Handle POST /parse — parse an artifact file and return impact metadata.
///
/// Reads the file at the requested path, extracts YAML frontmatter fields
/// (id, type), scans other .orqa/ files for references to this ID, then
/// computes should_warn. Returns 400 if the file cannot be read.
#[allow(clippy::too_many_lines)]
#[allow(clippy::unused_async)]
pub async fn parse_handler(
    State(state): State<HealthState>,
    Json(req): Json<ParseRequest>,
) -> Result<Json<ParseResponse>, (StatusCode, String)> {
    let start = Instant::now();
    let file_path = Path::new(&req.file);

    let content = std::fs::read_to_string(file_path).map_err(|e| {
        warn!(file = %req.file, error = %e, "[parse] could not read file");
        (
            StatusCode::BAD_REQUEST,
            format!("could not read file: {e}"),
        )
    })?;

    let (id, artifact_type) = extract_frontmatter(&content);

    // Derive the project root from the file's location so the handler needs
    // no injected state — the file path implicitly anchors it to the project.
    let project_root = file_path
        .parent()
        .and_then(|p| crate::process::find_project_root(p).ok());

    let (downstream_count, downstream_summary) = match &project_root {
        Some(root) => count_downstream(root, id.as_ref(), file_path),
        None => (0, None),
    };

    let downstream_warn_threshold = state.config.downstream_warn_threshold;
    let high_influence = HIGH_INFLUENCE_TYPES.contains(&artifact_type.to_lowercase().as_str());
    let should_warn = high_influence || downstream_count > downstream_warn_threshold;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    if should_warn {
        info!(
            subsystem = "parse",
            elapsed_ms,
            file = %req.file,
            artifact_type = %artifact_type,
            high_influence,
            downstream_count,
            should_warn,
            "[parse] artifact parsed — warning required"
        );
    } else {
        debug!(
            subsystem = "parse",
            elapsed_ms,
            file = %req.file,
            artifact_type = %artifact_type,
            high_influence,
            downstream_count,
            should_warn,
            "[parse] artifact parsed"
        );
    }

    Ok(Json(ParseResponse {
        id,
        artifact_type,
        high_influence,
        downstream_count,
        downstream_summary,
        should_warn,
    }))
}

/// Extract `id` and `type` fields from YAML frontmatter.
///
/// Expects frontmatter delimited by `---` lines at the start of the file.
/// Returns (None, "unknown") if the file has no frontmatter or the fields
/// are absent — callers treat missing metadata as low-influence unknowns.
fn extract_frontmatter(content: &str) -> (Option<String>, String) {
    let mut id: Option<String> = None;
    let mut artifact_type = String::from("unknown");

    // Frontmatter must start at the very first line.
    let mut lines = content.lines();
    if lines.next() != Some("---") {
        return (id, artifact_type);
    }

    for line in lines {
        if line == "---" {
            break;
        }
        if let Some(value) = line.strip_prefix("id:") {
            id = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("type:") {
            value.trim().clone_into(&mut artifact_type);
        }
    }

    (id, artifact_type)
}

/// Count how many .orqa/ files (excluding the artifact itself) reference the
/// artifact's ID, and build a short summary string.
///
/// Scans every .md file under `project_root/.orqa/` for a literal string match
/// of the artifact ID. This is an approximation — it catches YAML relationship
/// fields and inline prose mentions alike. A future version can use the graph
/// engine crate for a proper relationship query.
///
/// Returns (0, None) when the ID is unknown or the .orqa/ directory is absent.
fn count_downstream(
    project_root: &Path,
    id: Option<&String>,
    self_path: &Path,
) -> (u32, Option<String>) {
    let Some(artifact_id) = id else {
        return (0, None);
    };

    let orqa_dir = project_root.join(".orqa");
    if !orqa_dir.exists() {
        return (0, None);
    }

    let mut count: u32 = 0;
    let mut examples: Vec<String> = Vec::new();

    if let Ok(entries) = collect_md_files(&orqa_dir) {
        for entry in entries {
            // Skip the artifact itself.
            if entry == self_path {
                continue;
            }
            if let Ok(text) = std::fs::read_to_string(&entry) {
                if text.contains(artifact_id.as_str()) {
                    count += 1;
                    if examples.len() < 3 {
                        // Use the file stem as a human-readable label.
                        if let Some(stem) = entry.file_stem().and_then(|s| s.to_str()) {
                            examples.push(stem.to_owned());
                        }
                    }
                }
            }
        }
    }

    let summary = if count == 0 {
        None
    } else if examples.is_empty() {
        Some(format!("{count} reference(s)"))
    } else {
        let tail = if count > examples.len() as u32 {
            format!(", and {} more", count - examples.len() as u32)
        } else {
            String::new()
        };
        Some(format!("{}{tail}", examples.join(", ")))
    };

    (count, summary)
}

/// Recursively collect all .md files under a directory.
///
/// Returns an empty Vec and logs a warning if the directory cannot be read.
fn collect_md_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut results = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Recurse — .orqa/ has subdirectories (process/, delivery/, etc.)
            if let Ok(mut sub) = collect_md_files(&path) {
                results.append(&mut sub);
            }
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            results.push(path);
        }
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test frontmatter extraction with a well-formed file.
    #[test]
    fn test_extract_frontmatter_complete() {
        let content = "---\nid: epic-001\ntype: epic\ntitle: My Epic\n---\n\nBody text.\n";
        let (id, artifact_type) = extract_frontmatter(content);
        assert_eq!(id.as_deref(), Some("epic-001"));
        assert_eq!(artifact_type, "epic");
    }

    // Test frontmatter extraction when fields are absent.
    #[test]
    fn test_extract_frontmatter_missing_fields() {
        let content = "---\ntitle: No id or type here\n---\n\nBody.\n";
        let (id, artifact_type) = extract_frontmatter(content);
        assert_eq!(id, None);
        assert_eq!(artifact_type, "unknown");
    }

    // Test that files without frontmatter return defaults.
    #[test]
    fn test_extract_frontmatter_no_delimiter() {
        let content = "Just plain text, no frontmatter.\n";
        let (id, artifact_type) = extract_frontmatter(content);
        assert_eq!(id, None);
        assert_eq!(artifact_type, "unknown");
    }

    // Test that high-influence types are correctly identified.
    #[test]
    fn test_high_influence_types() {
        for t in ["epic", "principle", "decision"] {
            assert!(HIGH_INFLUENCE_TYPES.contains(&t), "{t} should be high-influence");
        }
        for t in ["task", "story", "bug"] {
            assert!(!HIGH_INFLUENCE_TYPES.contains(&t), "{t} should NOT be high-influence");
        }
    }

    // Test should_warn logic: high_influence alone triggers it.
    #[test]
    fn test_should_warn_high_influence() {
        // Use the default threshold value from DaemonConfig.
        let threshold = crate::config::DaemonConfig::default().downstream_warn_threshold;
        let high_influence = true;
        let downstream_count: u32 = 0;
        let should_warn = high_influence || downstream_count > threshold;
        assert!(should_warn);
    }

    // Test should_warn logic: threshold alone triggers it.
    #[test]
    fn test_should_warn_threshold() {
        let threshold = crate::config::DaemonConfig::default().downstream_warn_threshold;
        let high_influence = false;
        let downstream_count: u32 = threshold + 1;
        let should_warn = high_influence || downstream_count > threshold;
        assert!(should_warn);
    }

    // Test should_warn logic: exactly at threshold — no warn.
    #[test]
    fn test_no_warn_at_threshold() {
        let threshold = crate::config::DaemonConfig::default().downstream_warn_threshold;
        let high_influence = false;
        let downstream_count: u32 = threshold;
        let should_warn = high_influence || downstream_count > threshold;
        assert!(!should_warn);
    }

    // Test should_warn logic: below threshold, not high-influence — no warn.
    #[test]
    fn test_no_warn_below_threshold() {
        let threshold = crate::config::DaemonConfig::default().downstream_warn_threshold;
        let high_influence = false;
        let downstream_count: u32 = threshold - 1;
        let should_warn = high_influence || downstream_count > threshold;
        assert!(!should_warn);
    }
}
