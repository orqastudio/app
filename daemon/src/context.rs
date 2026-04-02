// POST /context endpoint for the OrqaStudio daemon.
//
// Returns active project context that the connector uses when generating
// CLAUDE.md: active rule titles (from .orqa/learning/rules/*.md frontmatter)
// and active workflow names (from .orqa/workflows/*.resolved.json filenames).
//
// The connector should not parse .orqa/ frontmatter or list .orqa/ directories
// directly — all .orqa/ reads are business logic that belongs in the daemon.

use std::path::Path;
use std::time::Instant;

use axum::Json;
use serde::{Deserialize, Serialize};

/// Request body for POST /context.
#[derive(Deserialize)]
pub struct ContextRequest {
    /// Absolute path to the project root.
    pub project_path: String,
}

/// Response body for POST /context.
#[derive(Serialize)]
pub struct ContextResponse {
    /// Rule titles extracted from .orqa/learning/rules/*.md frontmatter `title:` fields.
    /// Empty when the rules directory does not exist or no rules have titles.
    pub rule_titles: Vec<String>,
    /// Workflow names from .orqa/workflows/*.resolved.json filenames (stem only).
    /// Empty when the workflows directory does not exist.
    pub workflow_names: Vec<String>,
}

/// Handle POST /context.
///
/// Reads rule titles from rule frontmatter and workflow names from resolved
/// workflow filenames. Both reads degrade gracefully — missing directories
/// return empty vecs rather than errors.
pub fn context_handler(Json(req): Json<ContextRequest>) -> Json<ContextResponse> {
    let start = Instant::now();
    let project_path = Path::new(&req.project_path);
    let rule_titles = read_rule_titles(project_path);
    let workflow_names = read_workflow_names(project_path);

    tracing::debug!(
        subsystem = "context",
        elapsed_ms = start.elapsed().as_millis() as u64,
        rule_count = rule_titles.len(),
        workflow_count = workflow_names.len(),
        "[context] context_handler completed"
    );

    Json(ContextResponse { rule_titles, workflow_names })
}

/// Read `title:` values from frontmatter of all *.md files in .orqa/learning/rules/.
///
/// Skips files with no frontmatter, no title field, or unreadable content.
/// Returns titles in filesystem order (not guaranteed stable across runs).
fn read_rule_titles(project_path: &Path) -> Vec<String> {
    let rules_dir = project_path.join(".orqa").join("learning").join("rules");
    if !rules_dir.exists() {
        return Vec::new();
    }

    let mut titles = Vec::new();

    let Ok(entries) = std::fs::read_dir(&rules_dir) else {
        return Vec::new();
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        if let Some(title) = extract_frontmatter_title(&content) {
            titles.push(title);
        }
    }

    titles
}

/// Read workflow names from *.resolved.json filenames in .orqa/workflows/.
///
/// Returns the file stem (name without the `.resolved.json` suffix).
fn read_workflow_names(project_path: &Path) -> Vec<String> {
    let workflows_dir = project_path.join(".orqa").join("workflows");
    if !workflows_dir.exists() {
        return Vec::new();
    }

    let mut names = Vec::new();

    let Ok(entries) = std::fs::read_dir(&workflows_dir) else {
        return Vec::new();
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_owned(),
            None => continue,
        };
        if let Some(stem) = file_name.strip_suffix(".resolved.json") {
            names.push(stem.to_owned());
        }
    }

    names
}

/// Extract the `title:` value from YAML frontmatter.
///
/// Expects frontmatter delimited by `---` lines at the start of the file.
/// Returns None when the file has no frontmatter or no title field.
fn extract_frontmatter_title(content: &str) -> Option<String> {
    let mut lines = content.lines();

    if lines.next()?.trim() != "---" {
        return None;
    }

    for line in lines {
        if line.trim() == "---" {
            break;
        }
        if let Some(rest) = line.strip_prefix("title:") {
            let title = rest.trim().trim_matches('"').trim_matches('\'');
            if !title.is_empty() {
                return Some(title.to_owned());
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_temp_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    // ── extract_frontmatter_title ──

    #[test]
    fn extracts_title_from_well_formed_frontmatter() {
        let content = "---\ntitle: My Rule\nid: RULE-abc\n---\nBody text.\n";
        assert_eq!(extract_frontmatter_title(content), Some("My Rule".to_string()));
    }

    #[test]
    fn returns_none_when_no_frontmatter() {
        let content = "# Just a heading\n\nNo frontmatter here.\n";
        assert!(extract_frontmatter_title(content).is_none());
    }

    #[test]
    fn returns_none_when_no_title_field() {
        let content = "---\nid: RULE-abc\ntype: rule\n---\nBody.\n";
        assert!(extract_frontmatter_title(content).is_none());
    }

    #[test]
    fn strips_quotes_from_title() {
        let content = "---\ntitle: \"Quoted Title\"\n---\nBody.\n";
        assert_eq!(extract_frontmatter_title(content), Some("Quoted Title".to_string()));
    }

    // ── read_rule_titles ──

    #[test]
    fn returns_empty_when_rules_dir_absent() {
        let dir = make_temp_project();
        let result = read_rule_titles(dir.path());
        assert!(result.is_empty());
    }

    #[test]
    fn reads_titles_from_rule_files() {
        let dir = make_temp_project();
        let rules_dir = dir.path().join(".orqa").join("learning").join("rules");
        fs::create_dir_all(&rules_dir).unwrap();
        fs::write(
            rules_dir.join("RULE-abc123.md"),
            "---\ntitle: Always Test\nid: RULE-abc123\n---\nContent.\n",
        )
        .unwrap();
        fs::write(
            rules_dir.join("RULE-def456.md"),
            "---\ntitle: Never Defer\nid: RULE-def456\n---\nContent.\n",
        )
        .unwrap();

        let mut titles = read_rule_titles(dir.path());
        titles.sort(); // filesystem order is not stable
        assert_eq!(titles, vec!["Always Test", "Never Defer"]);
    }

    #[test]
    fn skips_non_md_files_in_rules_dir() {
        let dir = make_temp_project();
        let rules_dir = dir.path().join(".orqa").join("learning").join("rules");
        fs::create_dir_all(&rules_dir).unwrap();
        fs::write(rules_dir.join("not-a-rule.txt"), "title: Ignored\n").unwrap();

        let titles = read_rule_titles(dir.path());
        assert!(titles.is_empty());
    }

    // ── read_workflow_names ──

    #[test]
    fn returns_empty_when_workflows_dir_absent() {
        let dir = make_temp_project();
        let result = read_workflow_names(dir.path());
        assert!(result.is_empty());
    }

    #[test]
    fn reads_workflow_names_from_resolved_json_files() {
        let dir = make_temp_project();
        let workflows_dir = dir.path().join(".orqa").join("workflows");
        fs::create_dir_all(&workflows_dir).unwrap();
        fs::write(workflows_dir.join("agile.resolved.json"), "{}").unwrap();
        fs::write(workflows_dir.join("delivery.resolved.json"), "{}").unwrap();

        let mut names = read_workflow_names(dir.path());
        names.sort();
        assert_eq!(names, vec!["agile", "delivery"]);
    }

    #[test]
    fn skips_non_resolved_json_files() {
        let dir = make_temp_project();
        let workflows_dir = dir.path().join(".orqa").join("workflows");
        fs::create_dir_all(&workflows_dir).unwrap();
        fs::write(workflows_dir.join("agile.yaml"), "").unwrap();
        fs::write(workflows_dir.join("raw.md"), "").unwrap();

        let names = read_workflow_names(dir.path());
        assert!(names.is_empty());
    }
}
