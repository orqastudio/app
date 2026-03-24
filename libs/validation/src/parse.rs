//! Artifact parsing: converts a `.md` file into a [`ParsedArtifact`] with
//! structured frontmatter, body content, type inference, and schema validation.

use std::path::Path;

use crate::checks::schema::{build_frontmatter_schema, validate_frontmatter};
use crate::error::ValidationError;
use crate::graph::{extract_frontmatter, infer_artifact_type, ArtifactGraph};
use crate::platform::{scan_plugin_manifests, ArtifactTypeDef};
use crate::types::{ParsedArtifact, ValidationResult};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse a single `.md` artifact file into a [`ParsedArtifact`].
///
/// Steps:
/// 1. Read the file and split frontmatter from body.
/// 2. Parse the YAML frontmatter.
/// 3. Infer the artifact type from the ID prefix using plugin schemas.
/// 4. Run frontmatter schema validation against the inferred type.
///
/// Returns an error only on I/O or fatal parse failures. Schema validation
/// errors are embedded in the returned [`ParsedArtifact::validation`] field.
pub fn parse_artifact(
    file_path: &Path,
    project_path: &Path,
) -> Result<ParsedArtifact, ValidationError> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

    let (fm_text, body) = extract_frontmatter(&content);

    let fm_text = fm_text.ok_or_else(|| {
        ValidationError::FileSystem(format!(
            "No YAML frontmatter found in {}",
            file_path.display()
        ))
    })?;

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text)
        .map_err(|e| ValidationError::FileSystem(format!("YAML parse error: {e}")))?;

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

    let frontmatter = serde_json::to_value(&yaml_value)
        .unwrap_or(serde_json::Value::Null);

    let title = yaml_value
        .get("title")
        .and_then(|v| v.as_str())
        .map_or_else(|| humanize_stem(file_path), str::to_owned);

    let status = yaml_value
        .get("status")
        .and_then(|v| v.as_str())
        .map(str::to_owned);

    let frontmatter_type = yaml_value.get("type").and_then(|v| v.as_str());

    // Compute the relative path for type registry lookup.
    let rel_path = file_path
        .strip_prefix(project_path)
        .unwrap_or(file_path)
        .to_string_lossy()
        .replace('\\', "/");

    // Load plugin contributions to get type registry and schemas.
    let plugin_contributions = scan_plugin_manifests(project_path);

    // Build a minimal type registry from plugin artifact types.
    // We use the id_prefix for type inference when the path registry is empty.
    let type_registry = Vec::new(); // path-based registry requires project.json; we rely on ID prefix

    let artifact_type = infer_artifact_type(
        &rel_path,
        &type_registry,
        frontmatter_type,
        &id,
        &plugin_contributions.artifact_types,
    );

    let validation =
        run_validation(&frontmatter, &artifact_type, &plugin_contributions.artifact_types);

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
fn passes_filters(
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
fn passes_search(
    node: &crate::graph::ArtifactNode,
    search_filter: Option<&str>,
) -> bool {
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

fn humanize_stem(file_path: &Path) -> String {
    let stem = file_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let all_caps = stem
        .chars()
        .all(|c| c.is_ascii_uppercase() || c == '-' || c == '_' || c.is_ascii_digit());
    if stem.chars().any(|c| c.is_ascii_uppercase()) && all_caps {
        return stem;
    }

    stem.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut s = first.to_uppercase().to_string();
                    s.extend(chars.flat_map(char::to_lowercase));
                    s
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
        let rules_dir = tmp.path().join(".orqa/process/rules");
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
        let dir = tmp.path().join(".orqa/delivery/epics");
        let content = "---\nid: EPIC-aabbccdd\ntitle: Test Epic\nstatus: draft\n---\n\nThis is the body.\n";
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
        let rules_dir = tmp.path().join(".orqa/process/rules");
        let epics_dir = tmp.path().join(".orqa/delivery/epics");

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
        let rules_dir = tmp.path().join(".orqa/process/rules");

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
        let rules_dir = tmp.path().join(".orqa/process/rules");

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
    fn query_results_are_sorted() {
        use crate::graph::build_artifact_graph;

        let tmp = make_project();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        let epics_dir = tmp.path().join(".orqa/delivery/epics");

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
}
