// Artifact routes: CRUD and graph queries against the cached artifact graph.
//
// All handlers read from the shared GraphState. PUT /artifacts/:id modifies
// the file on disk and then triggers a graph reload. Other write operations
// (POST, DELETE) are deferred to a later task.
//
// Endpoints:
//   GET  /artifacts                   — query with optional type/status/search filters
//   GET  /artifacts/tree              — navigation tree derived from schema + disk scan
//   GET  /artifacts/:id               — single artifact node
//   GET  /artifacts/:id/content       — raw markdown file content from disk
//   PUT  /artifacts/:id               — update a frontmatter field
//   GET  /artifacts/:id/traceability  — traceability chain
//   GET  /artifacts/:id/impact        — downstream impact metadata

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use orqa_artifact::artifact_entries_from_schema;
use orqa_artifact::reader::artifact_scan_tree;
use orqa_engine_types::types::artifact::NavTree;
use orqa_validation::auto_fix::update_artifact_field;
use orqa_validation::metrics::compute_traceability;
use orqa_validation::PipelineCategories;
use orqa_validation::{ArtifactNode, TraceabilityResult};

use crate::graph_state::GraphState;

/// Artifact types whose changes have large downstream impact.
const HIGH_INFLUENCE_TYPES: &[&str] = &["epic", "principle", "decision"];

// ---------------------------------------------------------------------------
// Query parameters
// ---------------------------------------------------------------------------

/// Query parameters accepted by GET /artifacts.
#[derive(Debug, Deserialize)]
pub struct ArtifactsQuery {
    /// Filter by artifact type key (e.g. "epic", "task", "rule").
    #[serde(rename = "type")]
    pub artifact_type: Option<String>,
    /// Filter by status string (e.g. "active", "draft", "in-progress").
    pub status: Option<String>,
    /// Case-insensitive substring search over title and description.
    pub search: Option<String>,
    /// Organisation-mode child project name filter.
    pub project: Option<String>,
}

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for PUT /artifacts/:id.
#[derive(Debug, Deserialize)]
pub struct UpdateArtifactRequest {
    /// The frontmatter field name to update (e.g. "status").
    pub field: String,
    /// The new value to write into that field.
    pub value: serde_json::Value,
}

/// Response body for PUT /artifacts/:id.
#[derive(Debug, Serialize)]
pub struct UpdateArtifactResponse {
    /// ID of the artifact that was updated.
    pub id: String,
    /// The field that was updated.
    pub field: String,
    /// Serialised new value written to the file.
    pub new_value: serde_json::Value,
}

/// Response body for GET /artifacts/:id/content.
#[derive(Debug, Serialize)]
pub struct ContentResponse {
    /// Full raw markdown file content (frontmatter + body).
    pub content: String,
}

/// Response body for GET /artifacts/:id/impact.
#[derive(Debug, Serialize)]
pub struct ImpactResponse {
    /// The artifact ID.
    pub id: String,
    /// The artifact type.
    pub artifact_type: String,
    /// Whether this artifact type has large governance influence.
    pub high_influence: bool,
    /// Number of artifacts with incoming references from this one.
    pub downstream_count: usize,
    /// Short prose summary of downstream artifacts.
    pub downstream_summary: String,
    /// Whether the client should display a warning before editing.
    pub should_warn: bool,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /artifacts — return all artifacts matching the query filters.
///
/// All filtering is done against the cached graph. No disk I/O is performed.
#[allow(clippy::too_many_lines)]
pub async fn list_artifacts(
    State(state): State<GraphState>,
    Query(params): Query<ArtifactsQuery>,
) -> Json<Vec<ArtifactNode>> {
    let Ok(guard) = state.0.read() else {
        return Json(Vec::new());
    };

    let nodes: Vec<ArtifactNode> = guard
        .graph
        .nodes
        .values()
        .filter(|node| {
            // Skip bare-ID alias nodes in organisation mode.
            if node.project.is_some() {
                let key = &node.id;
                if guard.graph.nodes.contains_key(key.as_str())
                    && guard.graph.nodes[key.as_str()].project.is_some()
                {
                    // This is the canonical node — include it.
                } else {
                    // Check if there's a duplicate alias: node.id exists as a key without project.
                    if guard
                        .graph
                        .nodes
                        .get(key.as_str())
                        .is_some_and(|n| n.project.is_none())
                    {
                        return false;
                    }
                }
            }

            if let Some(ref tf) = params.artifact_type {
                if !node.artifact_type.eq_ignore_ascii_case(tf) {
                    return false;
                }
            }
            if let Some(ref sf) = params.status {
                match &node.status {
                    Some(s) if s.eq_ignore_ascii_case(sf) => {}
                    _ => return false,
                }
            }
            if let Some(ref pf) = params.project {
                match &node.project {
                    Some(p) if p.eq_ignore_ascii_case(pf) => {}
                    _ => return false,
                }
            }
            if let Some(ref q) = params.search {
                let ql = q.to_lowercase();
                let title_match = node.title.to_lowercase().contains(&ql);
                let desc_match = node
                    .description
                    .as_deref()
                    .is_some_and(|d| d.to_lowercase().contains(&ql));
                if !title_match && !desc_match {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // Stable output: sort by type then ID.
    let mut sorted = nodes;
    sorted.sort_by(|a, b| {
        a.artifact_type
            .cmp(&b.artifact_type)
            .then_with(|| a.id.cmp(&b.id))
    });

    Json(sorted)
}

/// Handle GET /artifacts/:id — return a single artifact node or 404.
pub async fn get_artifact(
    State(state): State<GraphState>,
    Path(id): Path<String>,
) -> Result<Json<ArtifactNode>, (StatusCode, Json<serde_json::Value>)> {
    match state.find_node(&id) {
        Some(node) => Ok(Json(node)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(
                serde_json::json!({ "error": format!("artifact '{}' not found", id), "code": "NOT_FOUND" }),
            ),
        )),
    }
}

/// Handle GET /artifacts/:id/content — return raw file content from disk.
///
/// Reads the artifact file from `project_root / node.path`. Returns 404 if the
/// artifact is not in the graph and 500 if the file cannot be read. The file read
/// is wrapped in `spawn_blocking` so it does not stall the tokio thread pool.
pub async fn get_artifact_content(
    State(state): State<GraphState>,
    Path(id): Path<String>,
) -> Result<Json<ContentResponse>, (StatusCode, Json<serde_json::Value>)> {
    let (node, project_root) = {
        let guard = state.0.read().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            )
        })?;
        let node = guard.graph.nodes.get(&id)
            .or_else(|| guard.graph.nodes.values().find(|n| n.id == id))
            .cloned()
            .ok_or_else(|| (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("artifact '{}' not found", id), "code": "NOT_FOUND" })),
            ))?;
        (node, guard.project_root.clone())
    };

    let file_path = project_root.join(&node.path);
    let content = tokio::task::spawn_blocking(move || std::fs::read_to_string(&file_path))
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        ))?
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("could not read file: {}", e), "code": "IO_ERROR" })),
        ))?;

    Ok(Json(ContentResponse { content }))
}

/// Handle PUT /artifacts/:id — update a single frontmatter field and reload graph.
///
/// Calls `update_artifact_field` from the validation crate to write the change to
/// disk. After the write, triggers a graph reload so subsequent requests see the
/// updated state. Both the disk write and the reload are wrapped in `spawn_blocking`
/// because they perform synchronous I/O and must not block the tokio thread pool.
pub async fn update_artifact(
    State(state): State<GraphState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateArtifactRequest>,
) -> Result<Json<UpdateArtifactResponse>, (StatusCode, Json<serde_json::Value>)> {
    let (file_path, project_root) = {
        let guard = state.0.read().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            )
        })?;
        let node = guard.graph.nodes.get(&id)
            .or_else(|| guard.graph.nodes.values().find(|n| n.id == id))
            .cloned()
            .ok_or_else(|| (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("artifact '{}' not found", id), "code": "NOT_FOUND" })),
            ))?;
        let file_path = guard.project_root.join(&node.path);
        (file_path, guard.project_root.clone())
    };

    let value_str = match &req.value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    };

    let field = req.field.clone();
    tokio::task::spawn_blocking(move || update_artifact_field(&file_path, &field, &value_str))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
            )
        })?
        .map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
            )
        })?;

    // Reload the graph so subsequent requests see the updated state.
    // Wrapped in spawn_blocking because reload() does a full directory scan.
    let state_clone = state.clone();
    let root_clone = project_root.clone();
    tokio::task::spawn_blocking(move || state_clone.reload(&root_clone))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
            )
        })?;

    Ok(Json(UpdateArtifactResponse {
        id,
        field: req.field,
        new_value: req.value,
    }))
}

/// Handle GET /artifacts/:id/traceability — compute the traceability chain.
///
/// Uses the cached graph to run `compute_traceability`. Returns 404 if the
/// artifact is not found.
pub async fn get_artifact_traceability(
    State(state): State<GraphState>,
    Path(id): Path<String>,
) -> Result<Json<TraceabilityResult>, (StatusCode, Json<serde_json::Value>)> {
    let guard = state.0.read().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
        )
    })?;

    // Verify artifact exists.
    if !guard.graph.nodes.contains_key(&id) && !guard.graph.nodes.values().any(|n| n.id == id) {
        return Err((
            StatusCode::NOT_FOUND,
            Json(
                serde_json::json!({ "error": format!("artifact '{}' not found", id), "code": "NOT_FOUND" }),
            ),
        ));
    }

    let owned = guard.owned_pipeline_categories();
    let (d, l, es, et, rt) = owned.as_str_vecs();
    let result = compute_traceability(
        &guard.graph,
        &id,
        &PipelineCategories {
            delivery: &d,
            learning: &l,
            excluded_statuses: &es,
            excluded_types: &et,
            root_types: &rt,
        },
    );
    Ok(Json(result))
}

/// Handle GET /artifacts/:id/impact — return downstream impact metadata.
///
/// Reports how many artifacts reference this one (directly via references_in)
/// and whether the caller should show a warning before editing.
pub async fn get_artifact_impact(
    State(state): State<GraphState>,
    Path(id): Path<String>,
) -> Result<Json<ImpactResponse>, (StatusCode, Json<serde_json::Value>)> {
    let node = state.find_node(&id).ok_or_else(|| (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "error": format!("artifact '{}' not found", id), "code": "NOT_FOUND" })),
    ))?;

    let downstream_count = node.references_in.len();
    let high_influence = HIGH_INFLUENCE_TYPES.contains(&node.artifact_type.to_lowercase().as_str());

    // Summarise first 3 upstream references.
    let examples: Vec<&str> = node
        .references_in
        .iter()
        .take(3)
        .map(|r| r.source_id.as_str())
        .collect();

    let downstream_summary = if downstream_count == 0 {
        "No incoming references".to_owned()
    } else if examples.len() < downstream_count {
        format!(
            "{}, and {} more",
            examples.join(", "),
            downstream_count - examples.len()
        )
    } else {
        examples.join(", ")
    };

    // Warn when high-influence or many references.
    let should_warn = high_influence || downstream_count > 5;

    Ok(Json(ImpactResponse {
        id,
        artifact_type: node.artifact_type,
        high_influence,
        downstream_count,
        downstream_summary,
        should_warn,
    }))
}

/// Handle GET /artifacts/tree — build the navigation tree from schema + disk scan.
///
/// Derives artifact layout from `.orqa/schema.composed.json` (schema is the
/// single source of truth for artifact types) then scans the `.orqa/` directory
/// tree to build the full navigation structure. Returns an empty tree when the
/// project has no schema or no artifacts.
pub async fn get_artifact_tree(
    State(state): State<GraphState>,
) -> Result<Json<NavTree>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = state
        .0
        .read()
        .ok()
        .map(|g| g.project_root.clone())
        .ok_or_else(|| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "graph state unavailable", "code": "STATE_ERROR" })),
        ))?;

    tokio::task::spawn_blocking(move || {
        let entries = artifact_entries_from_schema(&project_root);
        artifact_scan_tree(&project_root, &entries)
            .map(Json)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e.to_string(), "code": "SCAN_FAILED" })),
                )
            })
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        )
    })?
}
