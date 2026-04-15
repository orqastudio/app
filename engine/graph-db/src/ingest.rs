//! Artifact ingestion — walks `.orqa/` directories, parses YAML frontmatter,
//! and inserts artifacts + relationship edges into SurrealDB.
//!
//! Adapted from the pattern in `engine/graph/src/build.rs` but self-contained.
//! Uses `walkdir` for recursive directory traversal and manual frontmatter
//! splitting (same `---` delimiter logic as the enforcement parser).

use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::GraphDb;

/// Summary of an ingestion run.
#[derive(Debug, Default)]
pub struct IngestSummary {
    /// Total `.md` files scanned.
    pub files_scanned: usize,
    /// Artifacts successfully inserted (files with a valid `id` field).
    pub artifacts_inserted: usize,
    /// Relationship edges created.
    pub edges_created: usize,
    /// Files skipped due to parse errors or missing `id`.
    pub errors_skipped: usize,
}

/// A relationship entry from YAML frontmatter.
#[derive(Debug, Deserialize)]
struct RawRelationship {
    target: String,
    #[serde(rename = "type")]
    rel_type: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    rationale: Option<String>,
}

/// Walk an `.orqa/` directory recursively and ingest all markdown artifacts.
///
/// For each `.md` file with YAML frontmatter containing an `id` field:
/// 1. INSERT the artifact node
/// 2. For each `relationships:` entry, INSERT a `relates_to` edge
///
/// Returns a summary of files scanned, artifacts inserted, edges created.
pub async fn ingest_directory(db: &GraphDb, orqa_dir: &Path) -> Result<IngestSummary> {
    let mut summary = IngestSummary::default();

    for entry in walkdir::WalkDir::new(orqa_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        // Skip README.md files.
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.eq_ignore_ascii_case("README.md"))
        {
            continue;
        }

        summary.files_scanned += 1;

        match ingest_single_file(db, path, orqa_dir).await {
            Ok(edges) => {
                summary.artifacts_inserted += 1;
                summary.edges_created += edges;
            }
            Err(_) => {
                summary.errors_skipped += 1;
            }
        }
    }

    Ok(summary)
}

/// Parse and upsert a single artifact file and its relationship edges.
///
/// Returns the number of edges created, or an error if the file lacks an `id`
/// or cannot be parsed.
pub async fn ingest_single_file(db: &GraphDb, path: &Path, orqa_root: &Path) -> Result<usize> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;

    let (fm_text, body) = extract_frontmatter(&content);
    let fm_text = fm_text.ok_or_else(|| anyhow::anyhow!("no frontmatter in {}", path.display()))?;

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&fm_text)
        .with_context(|| format!("parsing YAML in {}", path.display()))?;

    let id = yaml_value
        .get("id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("no id field in {}", path.display()))?
        .trim()
        .to_owned();

    let fields = extract_fields(&yaml_value, &id, path, orqa_root);
    upsert_artifact(db, &id, &fields, &body).await?;

    let relationships = parse_relationships(&yaml_value);
    insert_edges(db, &id, &relationships).await
}

/// Extracted fields from YAML frontmatter for a single artifact.
struct ArtifactFields {
    rel_path: String,
    title: String,
    description: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    artifact_type: String,
    created: Option<String>,
    updated: Option<String>,
    frontmatter: serde_json::Value,
}

/// Extract all artifact fields from parsed YAML frontmatter.
fn extract_fields(
    yaml_value: &serde_yaml::Value,
    id: &str,
    path: &Path,
    orqa_root: &Path,
) -> ArtifactFields {
    let rel_path = path
        .strip_prefix(orqa_root.parent().unwrap_or(orqa_root))
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    let title = yaml_str(yaml_value, "title").unwrap_or_else(|| id.to_owned());
    let description = yaml_str(yaml_value, "description");
    let status = yaml_str(yaml_value, "status");
    let priority = yaml_str(yaml_value, "priority");
    let artifact_type =
        yaml_str(yaml_value, "type").unwrap_or_else(|| infer_type_from_path(&rel_path, id));
    let created = yaml_str(yaml_value, "created");
    let updated = yaml_str(yaml_value, "updated");
    let frontmatter = serde_json::to_value(yaml_value).unwrap_or(serde_json::Value::Null);

    ArtifactFields {
        rel_path,
        title,
        description,
        status,
        priority,
        artifact_type,
        created,
        updated,
        frontmatter,
    }
}

/// Extract a string field from YAML frontmatter.
fn yaml_str(yaml: &serde_yaml::Value, key: &str) -> Option<String> {
    yaml.get(key).and_then(|v| v.as_str()).map(str::to_owned)
}

/// Upsert an artifact node into the database.
async fn upsert_artifact(
    db: &GraphDb,
    id: &str,
    fields: &ArtifactFields,
    body: &str,
) -> Result<()> {
    let safe_id = sanitize_record_id(id);
    let desc_sql = option_to_surql(fields.description.as_deref());
    let status_sql = option_to_surql(fields.status.as_deref());
    let priority_sql = option_to_surql(fields.priority.as_deref());
    let created_sql = option_to_surql(fields.created.as_deref());
    let updated_sql = option_to_surql(fields.updated.as_deref());
    let body_sql = escape_surql_string(body);
    let title_sql = escape_surql_string(&fields.title);
    let type_sql = escape_surql_string(&fields.artifact_type);
    let path_sql = escape_surql_string(&fields.rel_path);
    let fm_json = serde_json::to_string(&fields.frontmatter).unwrap_or_else(|_| "{}".to_owned());

    let query = format!(
        "UPSERT artifact:`{safe_id}` SET \
            artifact_type = '{type_sql}', \
            title = '{title_sql}', \
            description = {desc_sql}, \
            status = {status_sql}, \
            priority = {priority_sql}, \
            path = '{path_sql}', \
            body = '{body_sql}', \
            frontmatter = {fm_json}, \
            source_plugin = NONE, \
            content_hash = NONE, \
            created = {created_sql}, \
            updated = {updated_sql}, \
            updated_at = time::now();"
    );

    db.db
        .query(&query)
        .await
        .with_context(|| format!("upserting artifact {id}"))?;
    Ok(())
}

/// Delete existing edges and insert new relationship edges for an artifact.
async fn insert_edges(db: &GraphDb, id: &str, relationships: &[RawRelationship]) -> Result<usize> {
    let safe_id = sanitize_record_id(id);
    let mut edge_count = 0;

    // Delete existing edges from this artifact to avoid duplicates on re-ingest.
    let delete_query = format!("DELETE relates_to WHERE in = artifact:`{safe_id}`;");
    db.db.query(&delete_query).await?;

    for rel in relationships {
        let target_safe = sanitize_record_id(&rel.target);
        let rel_type = escape_surql_string(rel.rel_type.as_deref().unwrap_or("unknown"));

        let edge_query = format!(
            "RELATE artifact:`{safe_id}`->relates_to->artifact:`{target_safe}` SET \
                relationship_type = '{rel_type}', \
                field = 'relationships';"
        );

        if db.db.query(&edge_query).await.is_ok() {
            edge_count += 1;
        }
    }

    Ok(edge_count)
}

/// Extract YAML frontmatter and body from a markdown string.
///
/// Same logic as `engine/graph/src/build.rs::extract_frontmatter`.
fn extract_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_owned());
    }

    let after_open = &trimmed[3..];
    let Some(close_pos) = after_open.find("\n---") else {
        return (None, content.to_owned());
    };

    let fm_text = after_open[..close_pos].trim().to_owned();
    let body = after_open[close_pos + 4..]
        .trim_start_matches('\n')
        .to_owned();
    (Some(fm_text), body)
}

/// Parse the `relationships` array from YAML frontmatter.
fn parse_relationships(yaml_value: &serde_yaml::Value) -> Vec<RawRelationship> {
    let Some(seq) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    else {
        return Vec::new();
    };

    seq.iter()
        .filter_map(|item| {
            let target = item.get("target")?.as_str()?.trim().to_owned();
            if target.is_empty() {
                return None;
            }
            let rel_type = item
                .get("type")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().to_owned());
            let rationale = item
                .get("rationale")
                .and_then(|v| v.as_str())
                .map(str::to_owned);
            Some(RawRelationship {
                target,
                rel_type,
                rationale,
            })
        })
        .collect()
}

/// Infer artifact type from file path or ID prefix when no `type:` frontmatter field.
fn infer_type_from_path(rel_path: &str, artifact_id: &str) -> String {
    let normalized = rel_path.replace('\\', "/");
    for segment in normalized.split('/').rev() {
        let t = match segment {
            "tasks" => "task",
            "epics" => "epic",
            "milestones" => "milestone",
            "rules" => "rule",
            "lessons" => "lesson",
            "decisions" => "decision",
            "reviews" => "review",
            "agents" => "agent",
            "personas" => "persona",
            "principles" => "principle",
            "pillars" => "pillar",
            "visions" => "vision",
            "discoveries" => "discovery",
            "wireframes" => "wireframe",
            "knowledge" => "knowledge",
            "docs" | "documentation" => "doc",
            _ => continue,
        };
        return t.to_owned();
    }

    // Fallback: infer from ID prefix (e.g. "EPIC-001" -> "epic").
    if let Some(prefix) = artifact_id.split('-').next() {
        return prefix.to_lowercase();
    }

    "doc".to_owned()
}

/// Sanitize an artifact ID for use as a SurrealDB record ID.
fn sanitize_record_id(id: &str) -> String {
    id.replace('`', "")
}

/// Escape a string for embedding in a SurrealQL single-quoted string literal.
fn escape_surql_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

/// Convert an `Option<&str>` to a SurrealQL expression: `'value'` or `NONE`.
fn option_to_surql(opt: Option<&str>) -> String {
    match opt {
        Some(s) => format!("'{}'", escape_surql_string(s)),
        None => "NONE".to_owned(),
    }
}
