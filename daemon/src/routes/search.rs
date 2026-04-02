// Search routes: indexing and querying the codebase via the orqa-search engine.
//
// The SearchEngine wraps DuckDB (not Send+Sync), so all engine calls are run
// via `spawn_blocking` to avoid blocking the tokio runtime.
//
// Endpoints:
//   POST /search/index    — index or re-index the codebase
//   POST /search/embed    — generate embeddings for unembedded chunks
//   POST /search/regex    — regex search over indexed codebase
//   POST /search/semantic — semantic search using ONNX embeddings
//   GET  /search/status   — get index status

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use orqa_search::{IndexStatus, SearchResult};

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Request shapes
// ---------------------------------------------------------------------------

/// Request body for POST /search/index.
#[derive(Debug, Deserialize)]
pub struct IndexRequest {
    /// Glob patterns to exclude from indexing (e.g. "target/**", "node_modules/**").
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// Request body for POST /search/regex.
#[derive(Debug, Deserialize)]
pub struct RegexSearchRequest {
    /// Regex pattern to search for.
    pub pattern: String,
    /// Glob pattern to filter files (e.g. "*.rs").
    pub path_filter: Option<String>,
    /// Maximum number of results to return.
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

/// Request body for POST /search/semantic.
#[derive(Debug, Deserialize)]
pub struct SemanticSearchRequest {
    /// Free-text query to embed and compare against the index.
    pub query: String,
    /// Maximum number of results to return.
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

/// Response body for POST /search/index and POST /search/embed.
#[derive(Debug, Serialize)]
pub struct IndexResponse {
    /// Outcome message.
    pub status: String,
    /// Number of chunks now in the index.
    pub chunk_count: u32,
}

fn default_max_results() -> usize {
    20
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Resolve the path to the DuckDB search index file.
///
/// The index lives at `.state/search.duckdb` under the project root.
fn db_path(project_root: &std::path::Path) -> std::path::PathBuf {
    project_root.join(".state/search.duckdb")
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /search/index — index or re-index the codebase.
///
/// Builds the search index for the project root. Existing index entries are
/// replaced. Returns the chunk count after indexing completes.
pub async fn search_index(
    State(state): State<GraphState>,
    Json(req): Json<IndexRequest>,
) -> Result<Json<IndexResponse>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let db = db_path(&project_root);
    let exclude = req.exclude.clone();

    tokio::task::spawn_blocking(move || {
        let mut engine = orqa_search::SearchEngine::new(&db)
            .map_err(|e| e.to_string())?;
        engine.index(&project_root, &exclude)
            .map_err(|e| e.to_string())?;
        let status = engine.get_status().map_err(|e| e.to_string())?;
        Ok::<_, String>(status.chunk_count)
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "INDEX_PANIC" })),
    ))?
    .map(|chunk_count| Json(IndexResponse { status: "indexed".to_owned(), chunk_count }))
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e, "code": "INDEX_FAILED" })),
    ))
}

/// Handle POST /search/embed — generate ONNX embeddings for unembedded chunks.
///
/// Requires a BGE-small-en-v1.5 model directory. The model path is read from
/// the ORQA_EMBED_MODEL environment variable (defaults to ~/.orqa/models/bge-small-en).
pub async fn search_embed(
    State(state): State<GraphState>,
) -> Result<Json<IndexResponse>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let model_dir = std::env::var("ORQA_EMBED_MODEL")
        .map_or_else(|_| dirs_next_home().join(".orqa/models/bge-small-en"), std::path::PathBuf::from);

    if !model_dir.exists() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("embedding model not found at {}", model_dir.display()),
                "code": "MODEL_NOT_FOUND"
            })),
        ));
    }

    let db = db_path(&project_root);

    tokio::task::spawn_blocking(move || {
        let mut engine = orqa_search::SearchEngine::new(&db)
            .map_err(|e| e.to_string())?;
        engine.init_embedder_sync(&model_dir)
            .map_err(|e| e.to_string())?;
        let embedded = engine.embed_chunks()
            .map_err(|e| e.to_string())?;
        Ok::<_, String>(embedded)
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "EMBED_PANIC" })),
    ))?
    .map(|chunk_count| Json(IndexResponse { status: "embedded".to_owned(), chunk_count }))
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e, "code": "EMBED_FAILED" })),
    ))
}

/// Handle POST /search/regex — regex search over the indexed codebase.
///
/// Returns matching chunks with file path, line numbers, content, and score.
pub async fn search_regex(
    State(state): State<GraphState>,
    Json(req): Json<RegexSearchRequest>,
) -> Result<Json<Vec<SearchResult>>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let db = db_path(&project_root);
    let pattern = req.pattern.clone();
    let path_filter = req.path_filter.clone();
    let max_results = req.max_results;

    tokio::task::spawn_blocking(move || {
        let engine = orqa_search::SearchEngine::new(&db)
            .map_err(|e| e.to_string())?;
        engine.search_regex(&pattern, path_filter.as_deref(), max_results as u32)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "SEARCH_PANIC" })),
    ))?
    .map(Json)
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e, "code": "SEARCH_FAILED" })),
    ))
}

/// Handle POST /search/semantic — semantic search using ONNX embeddings.
///
/// Requires the index to have been built and embeddings generated via
/// POST /search/embed. Returns results sorted by cosine similarity.
pub async fn search_semantic(
    State(state): State<GraphState>,
    Json(req): Json<SemanticSearchRequest>,
) -> Result<Json<Vec<SearchResult>>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let model_dir = std::env::var("ORQA_EMBED_MODEL")
        .map_or_else(|_| dirs_next_home().join(".orqa/models/bge-small-en"), std::path::PathBuf::from);

    if !model_dir.exists() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("embedding model not found at {}", model_dir.display()),
                "code": "MODEL_NOT_FOUND"
            })),
        ));
    }

    let db = db_path(&project_root);
    let query = req.query.clone();
    let max_results = req.max_results;

    tokio::task::spawn_blocking(move || {
        let mut engine = orqa_search::SearchEngine::new(&db)
            .map_err(|e| e.to_string())?;
        engine.init_embedder_sync(&model_dir)
            .map_err(|e| e.to_string())?;
        engine.search_semantic(&query, max_results as u32)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "SEARCH_PANIC" })),
    ))?
    .map(Json)
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e, "code": "SEARCH_FAILED" })),
    ))
}

/// Handle GET /search/status — return the current index status.
///
/// Reports whether the codebase has been indexed, the chunk count, and
/// whether embeddings have been generated.
pub async fn search_status(
    State(state): State<GraphState>,
) -> Result<Json<IndexStatus>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let db = db_path(&project_root);

    tokio::task::spawn_blocking(move || {
        let engine = orqa_search::SearchEngine::new(&db)
            .map_err(|e| e.to_string())?;
        engine.get_status().map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "STATUS_PANIC" })),
    ))?
    .map(Json)
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e, "code": "STATUS_FAILED" })),
    ))
}

/// Return the user's home directory path.
///
/// Falls back to `/tmp` if the home directory cannot be determined.
fn dirs_next_home() -> std::path::PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_or_else(|_| std::path::PathBuf::from("/tmp"), std::path::PathBuf::from)
}
