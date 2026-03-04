use tauri::State;

use crate::search::types::{IndexStatus, SearchResult};
use crate::search::SearchEngine;
use crate::state::AppState;

/// Index a codebase at the given project path, storing chunks in DuckDB.
///
/// Creates or replaces the search index at `<project_path>/.forge/search.duckdb`.
#[tauri::command]
pub async fn index_codebase(
    state: State<'_, AppState>,
    project_path: String,
    excluded_paths: Vec<String>,
) -> Result<IndexStatus, String> {
    let project_path_buf = std::path::PathBuf::from(&project_path);
    let db_path = project_path_buf.join(".forge").join("search.duckdb");

    // Ensure .forge directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut search_guard = state.search.lock().map_err(|e| e.to_string())?;

    // Initialize or replace the search engine
    let engine = SearchEngine::new(&db_path)?;
    *search_guard = Some(engine);

    // Index the codebase
    search_guard
        .as_mut()
        .ok_or_else(|| "search engine not initialized".to_string())?
        .index(&project_path_buf, &excluded_paths)
}

/// Search the indexed codebase using a regex pattern.
///
/// Returns matching code chunks with file paths, line numbers, and context.
/// The codebase must be indexed first via `index_codebase`.
#[tauri::command]
pub async fn search_regex(
    state: State<'_, AppState>,
    pattern: String,
    path: Option<String>,
    max_results: Option<u32>,
) -> Result<Vec<SearchResult>, String> {
    let search_guard = state.search.lock().map_err(|e| e.to_string())?;
    let engine = search_guard
        .as_ref()
        .ok_or_else(|| "search index not initialized — index the codebase first".to_string())?;
    engine.search_regex(&pattern, path.as_deref(), max_results.unwrap_or(20))
}

/// Search the indexed codebase using semantic similarity.
///
/// Embeds the natural language query and finds the most semantically similar
/// code chunks. Requires the codebase to be indexed and embeddings generated.
#[tauri::command]
pub async fn search_semantic(
    state: State<'_, AppState>,
    query: String,
    max_results: Option<u32>,
) -> Result<Vec<SearchResult>, String> {
    let mut search_guard = state.search.lock().map_err(|e| e.to_string())?;
    let engine = search_guard
        .as_mut()
        .ok_or_else(|| "search index not initialized — index the codebase first".to_string())?;
    engine.search_semantic(&query, max_results.unwrap_or(10))
}

/// Get the current status of the search index for a project.
///
/// If the search engine is not loaded but a database file exists on disk,
/// it will be loaded automatically.
#[tauri::command]
pub async fn get_index_status(
    state: State<'_, AppState>,
    project_path: String,
) -> Result<IndexStatus, String> {
    let search_guard = state.search.lock().map_err(|e| e.to_string())?;

    if let Some(engine) = search_guard.as_ref() {
        return engine.get_status();
    }

    // If no engine loaded, check if a search DB exists on disk
    let db_path = std::path::PathBuf::from(&project_path)
        .join(".forge")
        .join("search.duckdb");

    if db_path.exists() {
        // Drop the current guard before re-acquiring as mutable
        drop(search_guard);
        let mut search_guard = state.search.lock().map_err(|e| e.to_string())?;
        let engine = SearchEngine::new(&db_path)?;
        let status = engine.get_status()?;
        *search_guard = Some(engine);
        Ok(status)
    } else {
        Ok(IndexStatus {
            is_indexed: false,
            chunk_count: 0,
            has_embeddings: false,
            last_indexed: None,
            index_path: db_path.to_string_lossy().to_string(),
        })
    }
}
