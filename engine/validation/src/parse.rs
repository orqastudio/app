//! Artifact parsing: converts a `.md` file into a [`ParsedArtifact`] with
//! structured frontmatter, body content, type inference, and schema validation.

use std::path::Path;

use std::collections::HashSet;

use crate::checks::schema::{build_frontmatter_schema, validate_frontmatter};
use crate::error::ValidationError;
use crate::graph::{
    build_valid_relationship_types, extract_frontmatter, humanize_stem, infer_artifact_type,
    ArtifactGraph,
};
use crate::platform::{scan_plugin_manifests, ArtifactTypeDef};
use crate::types::{ParsedArtifact, ValidationResult};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse a single `.md` artifact file into a [`ParsedArtifact`].
///
/// Thin runtime wrapper: reads the file and loads plugin schemas, then delegates
/// all parsing logic to the pure [`parse_artifact_content`] function.
///
/// Returns an error only on I/O or fatal parse failures. Schema validation
/// errors are embedded in the returned [`ParsedArtifact::validation`] field.
pub fn parse_artifact(
    file_path: &Path,
    project_path: &Path,
) -> Result<ParsedArtifact, ValidationError> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
    let rel_path = relative_path(file_path, project_path);
    let plugin_contributions = scan_plugin_manifests(project_path);
    let valid_rel_types = build_valid_relationship_types(project_path);
    parse_artifact_content(
        &content,
        &rel_path,
        &plugin_contributions.artifact_types,
        &valid_rel_types,
    )
}

/// Parse artifact content from an in-memory string with no I/O.
///
/// Pure function: takes raw markdown content and metadata, returns a [`ParsedArtifact`]
/// or a fatal [`ValidationError`]. Schema validation errors are embedded in the
/// returned artifact's `validation` field rather than returned as `Err`.
///
/// `content` — full file contents including YAML frontmatter.
/// `rel_path` — relative path string used for type inference and error context.
/// `artifact_types` — plugin-provided type definitions for schema validation.
/// `valid_rel_types` — valid relationship types from project and plugin schemas.
#[allow(clippy::implicit_hasher)]
pub fn parse_artifact_content(
    content: &str,
    rel_path: &str,
    artifact_types: &[ArtifactTypeDef],
    valid_rel_types: &HashSet<String>,
) -> Result<ParsedArtifact, ValidationError> {
    // Use a dummy path for error messages — rel_path provides context.
    let dummy_path = Path::new(rel_path);

    let (yaml_value, body) = parse_frontmatter(dummy_path, content)?;
    let (id, title, status) = extract_scalar_fields(&yaml_value, dummy_path)?;
    let frontmatter = serde_json::to_value(&yaml_value).unwrap_or(serde_json::Value::Null);

    let frontmatter_type = yaml_value.get("type").and_then(|v| v.as_str());
    // Path-based registry requires project.json; we rely on ID prefix instead.
    let type_registry = Vec::new();
    let artifact_type = infer_artifact_type(
        rel_path,
        &type_registry,
        frontmatter_type,
        &id,
        artifact_types,
    );

    let mut validation = run_validation(&frontmatter, &artifact_type, artifact_types);
    validate_relationship_types(&yaml_value, valid_rel_types, &mut validation);

    Ok(ParsedArtifact {
        id,
        artifact_type,
        status,
        title,
        frontmatter,
        content: body,
        validation,
    })
}

/// Read and parse frontmatter YAML from raw file content.
///
/// Returns the parsed YAML value and body text, or an error if frontmatter is
/// missing or malformed.
fn parse_frontmatter(
    file_path: &Path,
    content: &str,
) -> Result<(serde_yaml::Value, String), ValidationError> {
    let (fm_text, body) = extract_frontmatter(content);
    let fm_text = fm_text.ok_or_else(|| {
        ValidationError::FileSystem(format!(
            "No YAML frontmatter found in {}",
            file_path.display()
        ))
    })?;
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text)
        .map_err(|e| ValidationError::FileSystem(format!("YAML parse error: {e}")))?;
    Ok((yaml_value, body))
}

/// Extract id, title, and status scalar fields from a parsed YAML frontmatter value.
///
/// Returns an error if the `id` field is missing or empty.
fn extract_scalar_fields(
    yaml_value: &serde_yaml::Value,
    file_path: &Path,
) -> Result<(String, String, Option<String>), ValidationError> {
    let id = yaml_value
        .get("id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| {
            ValidationError::FileSystem(format!(
                "Artifact at {} has no `id` field in frontmatter",
                file_path.display()
            ))
        })?
        .to_owned();
    let title = yaml_value
        .get("title")
        .and_then(|v| v.as_str())
        .map_or_else(|| humanize_stem(file_path), str::to_owned);
    let status = yaml_value
        .get("status")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    Ok((id, title, status))
}

/// Compute the relative path of `file_path` under `project_path`, normalised to forward slashes.
fn relative_path(file_path: &Path, project_path: &Path) -> String {
    file_path
        .strip_prefix(project_path)
        .unwrap_or(file_path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Convert a graph node (`ArtifactNode`) into a [`ParsedArtifact`].
///
/// Uses the cached body content from the node when available. Falls back to
/// re-reading the file from disk only if the node has no cached body (e.g.
/// nodes from older serialised graphs).
pub fn artifact_from_graph_node(
    node: &crate::graph::ArtifactNode,
    project_path: &Path,
    artifact_types: &[ArtifactTypeDef],
) -> Result<ParsedArtifact, ValidationError> {
    let body = if let Some(ref cached) = node.body {
        cached.clone()
    } else {
        let file_path = project_path.join(&node.path);
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| ValidationError::FileSystem(e.to_string()))?;
        let (_, body) = extract_frontmatter(&content);
        body
    };

    let validation = run_validation(&node.frontmatter, &node.artifact_type, artifact_types);

    Ok(ParsedArtifact {
        id: node.id.clone(),
        artifact_type: node.artifact_type.clone(),
        status: node.status.clone(),
        title: node.title.clone(),
        frontmatter: node.frontmatter.clone(),
        content: body,
        validation,
    })
}

// ---------------------------------------------------------------------------
// Query helpers
// ---------------------------------------------------------------------------

/// Filter and convert graph nodes into [`ParsedArtifact`] values.
///
/// Applies optional `type_filter`, `status_filter`, `id_filter`, and
/// `search_filter`. The search filter is a case-insensitive substring match
/// against title and description. Nodes whose files cannot be read are
/// skipped with a warning rather than failing the whole query.
pub fn query_artifacts(
    graph: &ArtifactGraph,
    project_path: &Path,
    type_filter: Option<&str>,
    status_filter: Option<&str>,
    id_filter: Option<&str>,
    search_filter: Option<&str>,
    artifact_types: &[ArtifactTypeDef],
) -> Vec<ParsedArtifact> {
    // Fast path: when an exact ID is provided, try a direct HashMap lookup
    // first (O(1)) instead of iterating all nodes (O(n)).
    if let Some(idf) = id_filter {
        if let Some(node) = graph.nodes.get(idf) {
            if passes_filters(node, type_filter, status_filter)
                && passes_search(node, search_filter)
            {
                if let Ok(parsed) = artifact_from_graph_node(node, project_path, artifact_types) {
                    return vec![parsed];
                }
            }
        }
        // Also check qualified keys in org mode (e.g., "project::RULE-abc").
        // If the direct lookup didn't match, fall through to the prefix scan below.
    }

    let mut results = Vec::new();

    for (key, node) in &graph.nodes {
        // In organisation mode the graph contains bare-ID alias nodes.
        // Skip them to avoid duplicates — prefer the qualified key entries.
        if key.as_str() == node.id && node.project.is_some() {
            continue;
        }

        if !passes_filters(node, type_filter, status_filter) {
            continue;
        }

        if let Some(idf) = id_filter {
            if node.id != idf && !node.id.starts_with(idf) {
                continue;
            }
        }

        if !passes_search(node, search_filter) {
            continue;
        }

        match artifact_from_graph_node(node, project_path, artifact_types) {
            Ok(parsed) => results.push(parsed),
            Err(e) => {
                tracing::warn!(
                    artifact = %node.id,
                    error = %e,
                    "could not read artifact body — skipping"
                );
            }
        }
    }

    // Stable output order: sort by artifact type then by ID.
    results.sort_by(|a, b| {
        a.artifact_type
            .cmp(&b.artifact_type)
            .then_with(|| a.id.cmp(&b.id))
    });

    results
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Check whether a node passes the optional type and status filters.
pub fn passes_filters(
    node: &crate::graph::ArtifactNode,
    type_filter: Option<&str>,
    status_filter: Option<&str>,
) -> bool {
    if let Some(tf) = type_filter {
        if node.artifact_type != tf {
            return false;
        }
    }
    if let Some(sf) = status_filter {
        match &node.status {
            Some(s) if s == sf => {}
            _ => return false,
        }
    }
    true
}

/// Case-insensitive substring match against the node's title and description.
pub fn passes_search(node: &crate::graph::ArtifactNode, search_filter: Option<&str>) -> bool {
    let Some(query) = search_filter else {
        return true;
    };
    let q = query.to_lowercase();
    if node.title.to_lowercase().contains(&q) {
        return true;
    }
    if let Some(ref desc) = node.description {
        if desc.to_lowercase().contains(&q) {
            return true;
        }
    }
    false
}

fn run_validation(
    frontmatter: &serde_json::Value,
    artifact_type: &str,
    artifact_types: &[ArtifactTypeDef],
) -> ValidationResult {
    let Some(type_def) = artifact_types.iter().find(|t| t.key == artifact_type) else {
        // No schema registered for this type — treat as valid (no constraints to enforce).
        return ValidationResult {
            valid: true,
            errors: Vec::new(),
        };
    };

    let schema = build_frontmatter_schema(type_def);
    let schema_errors = validate_frontmatter(frontmatter, &schema);

    if schema_errors.is_empty() {
        ValidationResult {
            valid: true,
            errors: Vec::new(),
        }
    } else {
        let errors = schema_errors
            .into_iter()
            .map(|e| {
                if e.path.is_empty() {
                    e.message
                } else {
                    format!("{}: {}", e.path, e.message)
                }
            })
            .collect();
        ValidationResult {
            valid: false,
            errors,
        }
    }
}

/// Check that every relationship's `type` field is in the valid vocabulary.
///
/// Invalid types are added as errors to the validation result. This mirrors
/// the graph builder's `tracing::warn` but surfaces findings in the `/parse`
/// response so pre-commit hooks and editors can report them.
fn validate_relationship_types(
    yaml_value: &serde_yaml::Value,
    valid_rel_types: &HashSet<String>,
    validation: &mut ValidationResult,
) {
    if valid_rel_types.is_empty() {
        return; // No vocabulary loaded — skip (avoids false positives)
    }
    let Some(seq) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    else {
        return;
    };
    for item in seq {
        let rel_type = item.get("type").and_then(|v| v.as_str());
        let target = item
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("<unknown>");

        match rel_type {
            Some(rt) if !valid_rel_types.contains(rt) => {
                validation.valid = false;
                validation.errors.push(format!(
                    "Invalid relationship type '{rt}' on target '{target}' — not defined in any plugin or project schema",
                ));
            }
            None => {
                validation.valid = false;
                validation.errors.push(format!(
                    "Relationship to '{target}' is missing a 'type' field",
                ));
            }
            _ => {} // valid
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    fn write_file(dir: &Path, name: &str, content: &str) {
        fs::create_dir_all(dir).expect("create dir");
        fs::write(dir.join(name), content).expect("write file");
    }

    #[test]
    fn parse_minimal_artifact() {
        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/learning/rules");
        let content = "---\nid: RULE-a1b2c3d4\ntitle: My Rule\nstatus: active\n---\n\n## Body\n\nSome content here.\n";
        write_file(&rules_dir, "RULE-a1b2c3d4.md", content);

        let file_path = rules_dir.join("RULE-a1b2c3d4.md");
        let parsed = parse_artifact(&file_path, tmp.path()).expect("parse");

        assert_eq!(parsed.id, "RULE-a1b2c3d4");
        assert_eq!(parsed.status.as_deref(), Some("active"));
        assert_eq!(parsed.title, "My Rule");
        assert!(parsed.content.contains("Some content here."));
        // No plugin schemas registered — validation passes trivially.
        assert!(parsed.validation.valid);
    }

    #[test]
    fn parse_extracts_body_after_frontmatter() {
        let tmp = make_project();
        let dir = tmp.path().join(".orqa/implementation/epics");
        let content =
            "---\nid: EPIC-aabbccdd\ntitle: Test Epic\nstatus: draft\n---\n\nThis is the body.\n";
        write_file(&dir, "EPIC-aabbccdd.md", content);

        let file_path = dir.join("EPIC-aabbccdd.md");
        let parsed = parse_artifact(&file_path, tmp.path()).expect("parse");

        assert_eq!(parsed.content.trim(), "This is the body.");
    }

    #[test]
    fn parse_fails_on_missing_id() {
        let tmp = make_project();
        let dir = tmp.path();
        let content = "---\ntitle: No ID\n---\n\nBody.\n";
        write_file(dir, "no-id.md", content);

        let file_path = dir.join("no-id.md");
        let result = parse_artifact(&file_path, tmp.path());
        assert!(result.is_err());
    }

    #[test]
    fn parse_fails_on_missing_frontmatter() {
        let tmp = make_project();
        let dir = tmp.path();
        let content = "# Just a heading\n\nNo frontmatter.\n";
        write_file(dir, "no-fm.md", content);

        let file_path = dir.join("no-fm.md");
        let result = parse_artifact(&file_path, tmp.path());
        assert!(result.is_err());
    }

    #[test]
    fn query_filters_by_type() {
        use crate::graph::build_artifact_graph;

        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/learning/rules");
        let epics_dir = tmp.path().join(".orqa/implementation/epics");

        write_file(
            &rules_dir,
            "RULE-a1b2c3d4.md",
            "---\nid: RULE-a1b2c3d4\ntitle: A Rule\nstatus: active\n---\nBody.\n",
        );
        write_file(
            &epics_dir,
            "EPIC-11223344.md",
            "---\nid: EPIC-11223344\ntitle: An Epic\nstatus: draft\n---\nBody.\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let results = query_artifacts(&graph, tmp.path(), Some("rule"), None, None, None, &[]);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RULE-a1b2c3d4");
    }

    #[test]
    fn query_filters_by_status() {
        use crate::graph::build_artifact_graph;

        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/learning/rules");

        write_file(
            &rules_dir,
            "RULE-a1b2c3d4.md",
            "---\nid: RULE-a1b2c3d4\ntitle: Active Rule\nstatus: active\n---\nBody.\n",
        );
        write_file(
            &rules_dir,
            "RULE-b2c3d4e5.md",
            "---\nid: RULE-b2c3d4e5\ntitle: Archived Rule\nstatus: archived\n---\nBody.\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let results = query_artifacts(&graph, tmp.path(), None, Some("active"), None, None, &[]);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RULE-a1b2c3d4");
    }

    #[test]
    fn query_filters_by_id_prefix() {
        use crate::graph::build_artifact_graph;

        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/learning/rules");

        write_file(
            &rules_dir,
            "RULE-a1b2c3d4.md",
            "---\nid: RULE-a1b2c3d4\ntitle: Rule A\nstatus: active\n---\nBody.\n",
        );
        write_file(
            &rules_dir,
            "RULE-b2c3d4e5.md",
            "---\nid: RULE-b2c3d4e5\ntitle: Rule B\nstatus: active\n---\nBody.\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let results = query_artifacts(&graph, tmp.path(), None, None, Some("RULE-a1"), None, &[]);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RULE-a1b2c3d4");
    }

    #[test]
    fn validate_relationship_types_flags_invalid() {
        let yaml: serde_yaml::Value = serde_yaml::from_str(
            r"
id: RULE-test0001
title: Test
relationships:
  - type: depends-on
    target: TASK-aaa
  - type: bogus-type
    target: TASK-bbb
  - target: TASK-ccc
",
        )
        .expect("yaml");

        let mut valid_types = HashSet::new();
        valid_types.insert("depends-on".to_owned());
        valid_types.insert("blocks".to_owned());

        let mut validation = ValidationResult {
            valid: true,
            errors: Vec::new(),
        };

        validate_relationship_types(&yaml, &valid_types, &mut validation);

        assert!(!validation.valid, "should be invalid");
        assert_eq!(
            validation.errors.len(),
            2,
            "two errors: bogus type + missing type"
        );
        assert!(
            validation.errors[0].contains("bogus-type"),
            "first error mentions the invalid type"
        );
        assert!(
            validation.errors[1].contains("missing a 'type' field"),
            "second error mentions missing type"
        );
    }

    #[test]
    fn validate_relationship_types_skips_when_vocabulary_empty() {
        let yaml: serde_yaml::Value = serde_yaml::from_str(
            r"
id: RULE-test0002
title: Test
relationships:
  - type: anything-goes
    target: TASK-aaa
",
        )
        .expect("yaml");

        let empty_types = HashSet::new();
        let mut validation = ValidationResult {
            valid: true,
            errors: Vec::new(),
        };

        validate_relationship_types(&yaml, &empty_types, &mut validation);

        assert!(
            validation.valid,
            "empty vocabulary should not produce errors"
        );
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn validate_relationship_types_all_valid() {
        let yaml: serde_yaml::Value = serde_yaml::from_str(
            r"
id: RULE-test0003
title: Test
relationships:
  - type: depends-on
    target: TASK-aaa
  - type: blocks
    target: TASK-bbb
",
        )
        .expect("yaml");

        let mut valid_types = HashSet::new();
        valid_types.insert("depends-on".to_owned());
        valid_types.insert("blocks".to_owned());

        let mut validation = ValidationResult {
            valid: true,
            errors: Vec::new(),
        };

        validate_relationship_types(&yaml, &valid_types, &mut validation);

        assert!(validation.valid, "all types valid — should pass");
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn query_results_are_sorted() {
        use crate::graph::build_artifact_graph;

        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/learning/rules");
        let epics_dir = tmp.path().join(".orqa/implementation/epics");

        write_file(
            &rules_dir,
            "RULE-zzzzzzzz.md",
            "---\nid: RULE-zzzzzzzz\ntitle: Z Rule\nstatus: active\n---\nBody.\n",
        );
        write_file(
            &rules_dir,
            "RULE-aaaaaaaa.md",
            "---\nid: RULE-aaaaaaaa\ntitle: A Rule\nstatus: active\n---\nBody.\n",
        );
        write_file(
            &epics_dir,
            "EPIC-11111111.md",
            "---\nid: EPIC-11111111\ntitle: An Epic\nstatus: draft\n---\nBody.\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let results = query_artifacts(&graph, tmp.path(), None, None, None, None, &[]);

        // epics come before rules alphabetically; within rules, aaaaaaaa before zzzzzzzz
        let ids: Vec<&str> = results.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids[0], "EPIC-11111111");
        assert_eq!(ids[1], "RULE-aaaaaaaa");
        assert_eq!(ids[2], "RULE-zzzzzzzz");
    }

    // -----------------------------------------------------------------------
    // passes_filters unit tests
    // -----------------------------------------------------------------------

    fn make_node(
        id: &str,
        artifact_type: &str,
        status: Option<&str>,
    ) -> crate::graph::ArtifactNode {
        crate::graph::ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: format!(".orqa/test/{id}.md"),
            artifact_type: artifact_type.to_owned(),
            title: id.to_owned(),
            description: None,
            status: status.map(str::to_owned),
            priority: None,
            frontmatter: serde_json::json!({}),
            body: None,
            references_out: vec![],
            references_in: vec![],
        }
    }

    #[test]
    fn passes_filters_no_filters_always_passes() {
        let node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        assert!(passes_filters(&node, None, None));
    }

    #[test]
    fn passes_filters_type_filter_matches() {
        let node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        assert!(passes_filters(&node, Some("rule"), None));
    }

    #[test]
    fn passes_filters_type_filter_rejects() {
        let node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        assert!(!passes_filters(&node, Some("epic"), None));
    }

    #[test]
    fn passes_filters_status_filter_matches() {
        let node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        assert!(passes_filters(&node, None, Some("active")));
    }

    #[test]
    fn passes_filters_status_filter_rejects() {
        let node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        assert!(!passes_filters(&node, None, Some("archived")));
    }

    #[test]
    fn passes_filters_status_filter_rejects_when_node_has_no_status() {
        let node = make_node("RULE-a1b2c3d4", "rule", None);
        assert!(!passes_filters(&node, None, Some("active")));
    }

    // -----------------------------------------------------------------------
    // passes_search unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn passes_search_no_filter_always_passes() {
        let node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        assert!(passes_search(&node, None));
    }

    #[test]
    fn passes_search_matches_title_case_insensitive() {
        let mut node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        node.title = "Agent Delegation".to_owned();
        assert!(passes_search(&node, Some("agent")));
        assert!(passes_search(&node, Some("DELEGATION")));
    }

    #[test]
    fn passes_search_matches_description() {
        let mut node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        node.title = "Some Rule".to_owned();
        node.description = Some("always delegate to background agents".to_owned());
        assert!(passes_search(&node, Some("background")));
    }

    #[test]
    fn passes_search_rejects_when_no_match() {
        let mut node = make_node("RULE-a1b2c3d4", "rule", Some("active"));
        node.title = "Agent Delegation".to_owned();
        node.description = None;
        assert!(!passes_search(&node, Some("planner")));
    }

    // -----------------------------------------------------------------------
    // relative_path unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn relative_path_strips_project_prefix() {
        let project = Path::new("/projects/my-app");
        let file = Path::new("/projects/my-app/.orqa/learning/rules/RULE-a1b2c3d4.md");
        let rel = relative_path(file, project);
        assert_eq!(rel, ".orqa/learning/rules/RULE-a1b2c3d4.md");
    }

    #[test]
    fn relative_path_returns_file_path_when_not_under_project() {
        let project = Path::new("/projects/my-app");
        let file = Path::new("/other/path/RULE-a1b2c3d4.md");
        let rel = relative_path(file, project);
        // When strip_prefix fails, falls back to the full file path
        assert!(rel.ends_with("RULE-a1b2c3d4.md"));
    }

    // -----------------------------------------------------------------------
    // parse_artifact_content unit tests (pure function — no I/O)
    // -----------------------------------------------------------------------

    #[test]
    fn parse_artifact_content_minimal() {
        let content = "---\nid: RULE-a1b2c3d4\ntitle: My Rule\nstatus: active\n---\n\n## Body\n";
        let parsed = parse_artifact_content(
            content,
            ".orqa/learning/rules/RULE-a1b2c3d4.md",
            &[],
            &HashSet::new(),
        )
        .expect("parse");
        assert_eq!(parsed.id, "RULE-a1b2c3d4");
        assert_eq!(parsed.title, "My Rule");
        assert_eq!(parsed.status.as_deref(), Some("active"));
        assert!(parsed.validation.valid);
    }

    #[test]
    fn parse_artifact_content_infers_type_from_frontmatter() {
        // Type field in frontmatter takes precedence over path-based inference.
        let content =
            "---\nid: EPIC-aabbccdd\ntype: epic\ntitle: My Epic\nstatus: draft\n---\n# Body\n";
        let parsed = parse_artifact_content(
            content,
            ".orqa/implementation/epics/EPIC-aabbccdd.md",
            &[],
            &HashSet::new(),
        )
        .expect("parse");
        assert_eq!(parsed.artifact_type, "epic");
    }

    #[test]
    fn parse_artifact_content_fails_on_missing_id() {
        let content = "---\ntitle: No ID\n---\n\nBody.\n";
        let result = parse_artifact_content(content, "no-id.md", &[], &HashSet::new());
        assert!(result.is_err());
    }

    #[test]
    fn parse_artifact_content_fails_on_missing_frontmatter() {
        let content = "# Just a heading\n\nNo frontmatter.\n";
        let result = parse_artifact_content(content, "no-fm.md", &[], &HashSet::new());
        assert!(result.is_err());
    }

    #[test]
    fn parse_artifact_content_validates_relationship_types() {
        let content = "---\nid: RULE-a1b2c3d4\ntitle: My Rule\nstatus: active\nrelationships:\n  - type: invalid-type\n    target: OTHER-001\n---\n";
        let mut valid_types = HashSet::new();
        valid_types.insert("depends-on".to_owned());
        let parsed = parse_artifact_content(
            content,
            ".orqa/learning/rules/RULE-a1b2c3d4.md",
            &[],
            &valid_types,
        )
        .expect("parse");
        // invalid-type is not in valid_types — should produce a validation error
        assert!(!parsed.validation.valid);
        assert!(parsed
            .validation
            .errors
            .iter()
            .any(|e| e.contains("invalid-type")));
    }

    #[test]
    fn parse_artifact_content_clean_when_valid_types() {
        let content = "---\nid: RULE-a1b2c3d4\ntitle: My Rule\nstatus: active\nrelationships:\n  - type: depends-on\n    target: OTHER-001\n---\n";
        let mut valid_types = HashSet::new();
        valid_types.insert("depends-on".to_owned());
        let parsed = parse_artifact_content(
            content,
            ".orqa/learning/rules/RULE-a1b2c3d4.md",
            &[],
            &valid_types,
        )
        .expect("parse");
        assert!(parsed.validation.valid);
        assert!(parsed.validation.errors.is_empty());
    }
}
