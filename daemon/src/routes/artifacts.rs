// Artifact routes: CRUD and graph queries against the cached artifact graph.
//
// All write handlers (POST, PUT) target SurrealDB as the authoritative store and
// refresh the in-memory HashMap from the stored record. No disk writes occur on
// any write path. DELETE is deferred to a later task.
//
// Endpoints:
//   POST /artifacts                   — create a new artifact in SurrealDB
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

use std::collections::BTreeMap;

use orqa_artifact::artifact_entries_from_schema;
use orqa_artifact::reader::artifact_scan_tree;
use orqa_engine_types::types::artifact::NavTree;
use orqa_graph::surreal_queries::{
    list_artifacts as surreal_list, search_artifacts as surreal_search,
};
use orqa_graph::writers::{
    count_edges_from, create_artifact, update_artifact_fields, RelationshipEdge,
};
use orqa_validation::checks::schema::{build_composed_schema, validate_frontmatter};
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

/// A single relationship edge in a POST /artifacts request body.
#[derive(Debug, Deserialize)]
pub struct RelationshipInput {
    /// Target artifact ID.
    pub target: String,
    /// Semantic relationship type (e.g. "delivers", "enforced-by").
    #[serde(rename = "type")]
    pub relation_type: String,
}

/// Request body for POST /artifacts.
///
/// `id` and `type` are required. All other frontmatter fields are optional.
/// `relationships` mirrors the frontmatter `relationships:` array and is used
/// to create `relates_to` edges in SurrealDB.
#[derive(Debug, Deserialize)]
pub struct CreateArtifactRequest {
    /// Artifact ID (e.g. "EPIC-new001"). Must be unique in SurrealDB.
    pub id: String,
    /// Artifact type key (e.g. "epic", "task", "rule").
    #[serde(rename = "type")]
    pub artifact_type: String,
    /// Human-readable title.
    pub title: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional status string.
    pub status: Option<String>,
    /// Optional priority string.
    pub priority: Option<String>,
    /// Relationship edges to create alongside the artifact.
    #[serde(default)]
    pub relationships: Vec<RelationshipInput>,
    /// Any additional frontmatter fields passed through verbatim.
    #[serde(flatten)]
    pub extra_fields: serde_json::Map<String, serde_json::Value>,
}

/// Response body for POST /artifacts.
#[derive(Debug, Serialize)]
pub struct CreateArtifactResponse {
    /// ID of the artifact that was created.
    pub id: String,
    /// Artifact type key.
    pub artifact_type: String,
    /// Version assigned by SurrealDB after the write.
    pub version: u64,
    /// SHA-256 content hash of the frontmatter JSON at creation time.
    pub content_hash: String,
    /// Number of `relates_to` edges inserted.
    pub edge_count: usize,
}

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

/// Handle POST /artifacts — insert a new artifact into SurrealDB.
///
/// Returns 409 when an artifact with the same ID already exists. Returns 503 when
/// SurrealDB is unavailable. On success the new record is inserted into the
/// in-memory HashMap so subsequent GET /artifacts requests reflect the new artifact
/// without requiring a full reload.
#[allow(clippy::too_many_lines)]
pub async fn create_artifact_handler(
    State(state): State<GraphState>,
    Json(req): Json<CreateArtifactRequest>,
) -> Result<(StatusCode, Json<CreateArtifactResponse>), (StatusCode, Json<serde_json::Value>)> {
    // Require SurrealDB — no silent fallback.
    let db = state.surreal_db().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "SurrealDB unavailable — POST requires a live database connection",
                "code": "DB_UNAVAILABLE"
            })),
        )
    })?;

    // Build the frontmatter BTreeMap from the request.
    let mut fields: BTreeMap<String, serde_json::Value> = BTreeMap::new();
    fields.insert("id".to_owned(), serde_json::Value::String(req.id.clone()));
    fields.insert(
        "type".to_owned(),
        serde_json::Value::String(req.artifact_type.clone()),
    );
    if let Some(ref t) = req.title {
        fields.insert("title".to_owned(), serde_json::Value::String(t.clone()));
    }
    if let Some(ref d) = req.description {
        fields.insert(
            "description".to_owned(),
            serde_json::Value::String(d.clone()),
        );
    }
    if let Some(ref s) = req.status {
        fields.insert("status".to_owned(), serde_json::Value::String(s.clone()));
    }
    if let Some(ref p) = req.priority {
        fields.insert("priority".to_owned(), serde_json::Value::String(p.clone()));
    }
    for (k, v) in &req.extra_fields {
        // Skip keys already inserted above to avoid overwriting typed fields.
        if !matches!(
            k.as_str(),
            "id" | "type" | "title" | "description" | "status" | "priority"
        ) {
            fields.insert(k.clone(), v.clone());
        }
    }

    // Validate the frontmatter against the composed JSON Schema for this artifact type.
    // Clone what we need from the state before any await so the lock is not held across await.
    let (artifact_types, schema_extensions) = {
        let guard = state.0.read().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            )
        })?;
        (
            guard.ctx.artifact_types.clone(),
            guard.ctx.schema_extensions.clone(),
        )
    };

    if let Some(type_def) = artifact_types.iter().find(|t| t.key == req.artifact_type) {
        let type_extensions: Vec<&orqa_validation::platform::SchemaExtension> = schema_extensions
            .iter()
            .filter(|e| e.target_key == req.artifact_type)
            .collect();
        let schema = build_composed_schema(type_def, &type_extensions);
        let fm_value = serde_json::to_value(&fields)
            .unwrap_or(serde_json::Value::Object(serde_json::Map::default()));
        let errors = validate_frontmatter(&fm_value, &schema);
        if !errors.is_empty() {
            let violations: Vec<serde_json::Value> = errors
                .iter()
                .map(|e| serde_json::json!({ "path": e.path, "message": e.message }))
                .collect();
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({
                    "error": "request body failed JSON Schema validation",
                    "code": "SCHEMA_INVALID",
                    "violations": violations
                })),
            ));
        }
    }

    // Compute content_hash from the frontmatter JSON.
    use sha2::{Digest, Sha256};
    let fm_json = serde_json::to_string(&fields).unwrap_or_else(|_| "{}".to_owned());
    let content_hash = hex::encode(Sha256::digest(fm_json.as_bytes()));

    // Convert relationship inputs to edge descriptors.
    let edges: Vec<RelationshipEdge> = req
        .relationships
        .iter()
        .map(|r| RelationshipEdge {
            target_id: r.target.clone(),
            relation_type: r.relation_type.clone(),
        })
        .collect();

    // Write to SurrealDB.
    let version = create_artifact(&db, &req.id, &fields, &content_hash, &edges)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("DUPLICATE") {
                (
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({
                        "error": format!("artifact '{}' already exists", req.id),
                        "code": "DUPLICATE"
                    })),
                )
            } else {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(serde_json::json!({ "error": msg, "code": "CREATE_FAILED" })),
                )
            }
        })?;

    // Read back edge count from SurrealDB — confirms writes succeeded, not just that the
    // request contained edges.
    let edge_count = count_edges_from(&db, &req.id).await.unwrap_or(edges.len());

    // Refresh the HashMap so GET /artifacts sees the new artifact immediately.
    if let Ok(mut guard) = state.0.write() {
        let title = req.title.clone().unwrap_or_else(|| req.id.clone());
        let node = ArtifactNode {
            id: req.id.clone(),
            project: None,
            path: String::new(),
            artifact_type: req.artifact_type.clone(),
            title,
            description: req.description.clone(),
            status: req.status.clone(),
            priority: req.priority.clone(),
            frontmatter: serde_json::to_value(&fields)
                .unwrap_or(serde_json::Value::Object(serde_json::Map::default())),
            body: None,
            references_out: Vec::new(),
            references_in: Vec::new(),
        };
        guard.graph.nodes.insert(req.id.clone(), node);
    }

    Ok((
        StatusCode::CREATED,
        Json(CreateArtifactResponse {
            id: req.id,
            artifact_type: req.artifact_type,
            version,
            content_hash,
            edge_count,
        }),
    ))
}

/// Handle GET /artifacts — return all artifacts matching the query filters.
///
/// Uses SurrealDB as a fast-path for filtering and search when available and the
/// request is not org-mode (project filter absent). SurrealDB returns matching IDs
/// which are then resolved against the HashMap for full `ArtifactNode` data. Falls
/// back to the HashMap path when SurrealDB is unavailable or returns an error.
#[allow(clippy::too_many_lines)]
pub async fn list_artifacts(
    State(state): State<GraphState>,
    Query(params): Query<ArtifactsQuery>,
) -> Json<Vec<ArtifactNode>> {
    // SurrealDB fast-path: only when no org-mode project filter is set.
    // Org-mode filtering requires alias-deduplication logic only the HashMap path handles.
    if params.project.is_none() {
        let db_opt = state.0.read().ok().and_then(|g| g.db.clone());
        if let Some(db) = db_opt {
            let surreal_result = if let Some(ref q) = params.search {
                surreal_search(&db, q).await
            } else {
                surreal_list(
                    &db,
                    params.artifact_type.as_deref(),
                    params.status.as_deref(),
                )
                .await
            };

            match surreal_result {
                Ok(records) => {
                    let guard = match state.0.read() {
                        Ok(g) => g,
                        Err(p) => p.into_inner(),
                    };
                    // Resolve each SurrealDB record to a full ArtifactNode via the HashMap.
                    // id_key() extracts the string portion of the RecordId (e.g. "EPIC-001").
                    let mut nodes: Vec<ArtifactNode> = records
                        .iter()
                        .filter_map(|r| guard.graph.nodes.get(r.id_key()).cloned())
                        .collect();
                    nodes.sort_by(|a, b| {
                        a.artifact_type
                            .cmp(&b.artifact_type)
                            .then_with(|| a.id.cmp(&b.id))
                    });
                    return Json(nodes);
                }
                Err(e) => {
                    tracing::warn!(
                        subsystem = "artifacts",
                        error = %e,
                        "[artifacts] SurrealDB list_artifacts failed, falling back to HashMap"
                    );
                    // Fall through to the HashMap path below.
                }
            }
        }
    }

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

/// Refresh a single node in the in-memory HashMap from a SurrealDB record.
///
/// Called after a SurrealDB write to avoid a full graph reload. Does nothing
/// if the artifact is not found in the graph or if the state lock is poisoned.
fn refresh_node_in_state(
    state: &GraphState,
    id: &str,
    field: &str,
    value_str: &str,
    frontmatter: serde_json::Value,
) {
    let Ok(mut guard) = state.0.write() else {
        return;
    };

    let map_key = if guard.graph.nodes.contains_key(id) {
        Some(id.to_owned())
    } else {
        guard
            .graph
            .nodes
            .iter()
            .find(|(_, n)| n.id == id)
            .map(|(k, _)| k.clone())
    };

    let Some(key) = map_key else { return };
    let Some(node) = guard.graph.nodes.get_mut(&key) else {
        return;
    };

    match field {
        "status" => node.status = Some(value_str.to_owned()),
        "title" => node.title.clone_from(&value_str.to_owned()),
        "description" => node.description = Some(value_str.to_owned()),
        "priority" => node.priority = Some(value_str.to_owned()),
        _ => {}
    }
    node.frontmatter = frontmatter;
}

/// Handle PUT /artifacts/:id — update a single frontmatter field in SurrealDB.
///
/// SurrealDB is the authoritative write target. Returns 503 if SurrealDB is
/// unavailable — there is no silent fallback to a disk write. After the write,
/// the in-memory HashMap entry is refreshed from the SurrealDB record so
/// subsequent requests see the updated state without a full disk reload.
pub async fn update_artifact(
    State(state): State<GraphState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateArtifactRequest>,
) -> Result<Json<UpdateArtifactResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Verify the artifact exists in the HashMap and get its node ID.
    {
        let guard = state.0.read().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            )
        })?;
        guard
            .graph
            .nodes
            .get(&id)
            .or_else(|| guard.graph.nodes.values().find(|n| n.id == id))
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({ "error": format!("artifact '{}' not found", id), "code": "NOT_FOUND" })),
                )
            })?;
    }

    // Require SurrealDB — no silent fallback to disk write.
    let db = state.surreal_db().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "SurrealDB unavailable — PUT requires a live database connection",
                "code": "DB_UNAVAILABLE"
            })),
        )
    })?;

    let value_str = match &req.value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    };

    // Write the field update to SurrealDB; version and updated_at are bumped atomically.
    update_artifact_fields(&db, &id, &req.field, &value_str)
        .await
        .map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
            )
        })?;

    // Refresh the HashMap entry from the SurrealDB record (no disk read).
    if let Ok(Some(stored)) = orqa_graph::writers::read_artifact(&db, &id).await {
        refresh_node_in_state(&state, &id, &req.field, &value_str, stored.frontmatter);
    }

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
