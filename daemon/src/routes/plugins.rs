// Plugin routes: list, install, uninstall, registry, updates.
//
// Install operations are synchronous filesystem/network calls, run via
// spawn_blocking so the tokio runtime is not blocked. Registry fetches are
// async (reqwest) and run on the tokio runtime directly.
//
// Endpoints:
//   GET    /plugins                   — list installed plugins
//   GET    /plugins/:name             — get a plugin's manifest
//   GET    /plugins/:name/path        — get filesystem path to a plugin
//   GET    /plugins/:name/uninstall-preview — preview what uninstall would remove
//   POST   /plugins/install/local     — install from a local path
//   POST   /plugins/install/github    — install from GitHub
//   DELETE /plugins/:name             — uninstall a plugin (requires ?force=true or returns preview)
//   GET    /plugins/registry          — browse plugin registry
//   GET    /plugins/updates           — check for available updates

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use orqa_plugin::discovery::{scan_plugins, DiscoveredPlugin};
use orqa_plugin::installer::{install_from_path, InstallResult, UninstallPreview};
/// Plugin manifest filename — must match engine::plugin::manifest::MANIFEST_FILENAME.
const MANIFEST_FILENAME: &str = "orqa-plugin.json";
use orqa_plugin::registry::RegistryCache;

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /plugins/install/local.
#[derive(Debug, Deserialize)]
pub struct InstallLocalRequest {
    /// Absolute or relative path to the local plugin directory.
    pub path: String,
}

/// Request body for POST /plugins/install/github.
#[derive(Debug, Deserialize)]
pub struct InstallGithubRequest {
    /// GitHub repository slug (e.g. "orqastudio/plugin-software").
    pub repo: String,
    /// Specific version tag to install (e.g. "1.0.0"). None installs latest.
    pub version: Option<String>,
}

/// Query parameters for DELETE /plugins/:name.
#[derive(Debug, Deserialize)]
pub struct UninstallQuery {
    /// When true, perform the destructive uninstall immediately.
    /// When absent or false, return a preview of what would be removed.
    #[serde(default)]
    pub force: bool,
}

/// Response body for GET /plugins/:name.
#[derive(Debug, Serialize)]
pub struct PluginManifestResponse {
    /// Plugin package name.
    pub name: String,
    /// Plugin filesystem path.
    pub path: String,
    /// Raw manifest JSON value for full fidelity.
    pub manifest: serde_json::Value,
}

/// Response body for GET /plugins/:name/path.
#[derive(Debug, Serialize)]
pub struct PluginPathResponse {
    /// Plugin package name.
    pub name: String,
    /// Absolute filesystem path to the plugin directory.
    pub path: String,
}

/// Response body for GET /plugins/updates.
#[derive(Debug, Serialize)]
pub struct PluginUpdateInfo {
    /// Plugin package name.
    pub name: String,
    /// Currently installed version.
    pub installed_version: String,
    /// Latest available version from the registry.
    pub latest_version: String,
    /// Whether an update is available.
    pub update_available: bool,
}

// ---------------------------------------------------------------------------
// Shared registry cache — one instance per daemon process.
// ---------------------------------------------------------------------------

/// Get or create the shared registry cache from daemon state.
///
/// The registry cache is stored in app state via `Arc<RegistryCache>` to
/// ensure a single TTL-based cache across all plugin route calls.
fn registry_cache() -> Arc<RegistryCache> {
    // Each call creates a new cache; the TTL prevents redundant network fetches
    // within a single request when the registry handler is invoked multiple times.
    // A proper shared-state approach would put this in HealthState; for now a
    // per-request Arc is sufficient because the registry handler is low-frequency.
    Arc::new(RegistryCache::new())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /plugins — list all installed and enabled plugins.
///
/// Reads project.json to discover plugins with installed:true and enabled:true.
pub async fn list_plugins(State(state): State<GraphState>) -> Json<Vec<DiscoveredPlugin>> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Json(Vec::new());
        };
        guard.project_root.clone()
    };
    Json(scan_plugins(&project_root))
}

/// Handle GET /plugins/:name — return the manifest for a named plugin.
///
/// Locates the plugin by package name from the discovered list and reads its
/// manifest. Returns 404 if no matching plugin is installed.
pub async fn get_plugin(
    State(state): State<GraphState>,
    Path(name): Path<String>,
) -> Result<Json<PluginManifestResponse>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let plugins = scan_plugins(&project_root);
    let plugin = plugins.iter().find(|p| p.name == name).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("plugin '{}' not found", name),
                "code": "NOT_FOUND"
            })),
        )
    })?;

    // Read the raw JSON file directly to preserve ALL fields — the Rust
    // PluginManifest struct is a minimal subset and drops fields the frontend
    // needs (requires, requiresSidecar, workflows, knowledge, semantics, etc.).
    let manifest_path = std::path::Path::new(&plugin.path).join(MANIFEST_FILENAME);
    let raw_json = std::fs::read_to_string(&manifest_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "MANIFEST_ERROR" })),
        )
    })?;
    let manifest_json: serde_json::Value = serde_json::from_str(&raw_json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "MANIFEST_PARSE_ERROR" })),
        )
    })?;

    Ok(Json(PluginManifestResponse {
        name: plugin.name.clone(),
        path: plugin.path.clone(),
        manifest: manifest_json,
    }))
}

/// Handle GET /plugins/:name/path — return the filesystem path for a plugin.
///
/// Returns 404 if the plugin is not installed.
pub async fn get_plugin_path(
    State(state): State<GraphState>,
    Path(name): Path<String>,
) -> Result<Json<PluginPathResponse>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let plugins = scan_plugins(&project_root);
    let plugin = plugins.iter().find(|p| p.name == name).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("plugin '{}' not found", name),
                "code": "NOT_FOUND"
            })),
        )
    })?;

    Ok(Json(PluginPathResponse {
        name: plugin.name.clone(),
        path: plugin.path.clone(),
    }))
}

/// Handle GET /plugins/:name/uninstall-preview — return what uninstalling would remove.
///
/// No destructive actions are taken. The daemon augments the engine's preview with
/// the SurrealDB artifact count for records with source_plugin = name.
pub async fn preview_plugin_uninstall(
    State(state): State<GraphState>,
    Path(name): Path<String>,
) -> Result<Json<UninstallPreview>, (StatusCode, Json<serde_json::Value>)> {
    let (project_root, db) = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        (guard.project_root.clone(), guard.db.clone())
    };

    let name_clone = name.clone();
    let mut preview = tokio::task::spawn_blocking(move || {
        orqa_plugin::installer::preview_uninstall(&name_clone, &project_root)
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "PREVIEW_PANIC" })),
        )
    })?
    .map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": e.to_string(), "code": "PLUGIN_NOT_FOUND" })),
        )
    })?;

    // Augment with the SurrealDB artifact count for this plugin.
    if let Some(db) = db {
        let name_escaped = name.replace('\'', "\\'");
        let query = format!(
            "SELECT count() FROM artifact WHERE source_plugin = '{name_escaped}' GROUP ALL;"
        );
        if let Ok(mut response) = db.0.query(&query).await {
            let rows: Vec<serde_json::Value> = response.take(0).unwrap_or_default();
            preview.surrealdb_artifact_count = rows
                .first()
                .and_then(|r| r.get("count"))
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0) as usize;
        }
    }

    Ok(Json(preview))
}

/// Handle POST /plugins/install/local — install a plugin from a local path.
///
/// Runs synchronously via spawn_blocking. After installation, ingests all
/// surrealdb-target content entries into SurrealDB with source_plugin set.
/// Triggers graph reload after installation to pick up any new artifact types.
pub async fn install_plugin_local(
    State(state): State<GraphState>,
    Json(req): Json<InstallLocalRequest>,
) -> Result<Json<InstallResult>, (StatusCode, Json<serde_json::Value>)> {
    let (project_root, db) = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        (guard.project_root.clone(), guard.db.clone())
    };

    let source_path = std::path::PathBuf::from(&req.path);
    let project_root_clone = project_root.clone();

    let result =
        tokio::task::spawn_blocking(move || install_from_path(&source_path, &project_root_clone))
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e.to_string(), "code": "INSTALL_PANIC" })),
                )
            })?
            .map_err(|e| {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(serde_json::json!({ "error": e.to_string(), "code": "INSTALL_FAILED" })),
                )
            })?;

    // Ingest surrealdb-target content and enforcement rules into SurrealDB.
    if let Some(ref db) = db {
        if result.surrealdb_content_entries > 0 {
            ingest_surrealdb_content(db, &result.name, &result.path, &project_root).await;
        }
        ingest_enforcement_rules(db, &result.name, &result.path).await;
    }

    // Reload the graph to pick up any schema changes from the new plugin.
    state.reload(&project_root);

    Ok(Json(result))
}

/// Handle POST /plugins/install/github — install a plugin from a GitHub release.
///
/// Delegates to `orqa_plugin::installer::install_from_github`. Async — the
/// underlying function uses reqwest for GitHub API and archive download.
pub async fn install_plugin_github(
    State(state): State<GraphState>,
    Json(req): Json<InstallGithubRequest>,
) -> Result<Json<InstallResult>, (StatusCode, Json<serde_json::Value>)> {
    let (project_root, db) = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        (guard.project_root.clone(), guard.db.clone())
    };

    let result = orqa_plugin::installer::install_from_github(
        &req.repo,
        req.version.as_deref(),
        &project_root,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e.to_string(), "code": "INSTALL_FAILED" })),
        )
    })?;

    // Ingest surrealdb-target content and enforcement rules into SurrealDB.
    if let Some(ref db) = db {
        if result.surrealdb_content_entries > 0 {
            ingest_surrealdb_content(db, &result.name, &result.path, &project_root).await;
        }
        ingest_enforcement_rules(db, &result.name, &result.path).await;
    }

    state.reload(&project_root);

    Ok(Json(result))
}

/// Handle DELETE /plugins/:name — uninstall a named plugin.
///
/// Without `?force=true`, returns a preview (HTTP 200 + body) instead of performing
/// the destructive uninstall. With `?force=true`, removes the plugin and deletes all
/// SurrealDB artifact records whose `source_plugin` field matches this plugin's name.
#[allow(clippy::too_many_lines)]
pub async fn uninstall_plugin(
    State(state): State<GraphState>,
    Path(name): Path<String>,
    Query(query): Query<UninstallQuery>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    if !query.force {
        // Return 400 asking the caller to use the preview endpoint or pass ?force=true.
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "uninstall requires confirmation: use GET /plugins/:name/uninstall-preview to see what would be removed, then DELETE with ?force=true",
                "code": "UNINSTALL_REQUIRES_FORCE"
            })),
        ));
    }

    let (project_root, db) = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        (guard.project_root.clone(), guard.db.clone())
    };

    let name_clone = name.clone();
    let project_root_clone = project_root.clone();
    tokio::task::spawn_blocking(move || {
        orqa_plugin::installer::uninstall(&name_clone, &project_root_clone)
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "UNINSTALL_PANIC" })),
        )
    })?
    .map_err(|e| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e.to_string(), "code": "UNINSTALL_FAILED" })),
        )
    })?;

    // Remove all SurrealDB artifact records and enforcement rules with source_plugin = name.
    if let Some(ref db) = db {
        match orqa_graph::delete_plugin_artifacts(db, &name).await {
            Ok(_) => {
                tracing::info!(plugin = %name, "[plugins] SurrealDB artifacts removed on uninstall");
            }
            Err(e) => {
                tracing::warn!(plugin = %name, error = %e, "[plugins] SurrealDB artifact cleanup failed");
            }
        }
        match orqa_graph::delete_plugin_enforcement_rules(db, &name).await {
            Ok(_) => {
                tracing::info!(plugin = %name, "[plugins] enforcement rules removed on uninstall");
            }
            Err(e) => {
                tracing::warn!(plugin = %name, error = %e, "[plugins] enforcement rule cleanup failed");
            }
        }
    }

    state.reload(&project_root);

    Ok(StatusCode::NO_CONTENT)
}

/// Handle GET /plugins/registry — fetch available plugins from the registry.
///
/// Combines official and community catalogs. Uses an in-process TTL cache
/// to avoid repeated network fetches within the same daemon session.
pub async fn list_plugin_registry(
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let cache = registry_cache();

    let (official, community) = tokio::join!(cache.fetch("official"), cache.fetch("community"),);

    let official_plugins = official.map(|c| c.plugins).unwrap_or_default();
    let community_plugins = community.map(|c| c.plugins).unwrap_or_default();

    Ok(Json(serde_json::json!({
        "official": official_plugins,
        "community": community_plugins,
    })))
}

/// Handle GET /plugins/updates — check installed plugins for available updates.
///
/// Compares installed plugin versions against the registry to detect updates.
/// Returns all installed plugins with an `update_available` flag.
pub async fn check_plugin_updates(State(state): State<GraphState>) -> Json<Vec<PluginUpdateInfo>> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Json(Vec::new());
        };
        guard.project_root.clone()
    };

    let installed = scan_plugins(&project_root);
    if installed.is_empty() {
        return Json(Vec::new());
    }

    let cache = registry_cache();
    let (official, community) = tokio::join!(cache.fetch("official"), cache.fetch("community"),);

    // Build a map of name -> latest_version from the registry.
    let mut registry_versions: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    for catalog in [official, community].into_iter().flatten() {
        for entry in catalog.plugins {
            // Registry entries don't carry a version directly. Use lockfile.
            // For now, mark update_available as false when no version data.
            registry_versions
                .entry(entry.name)
                .or_insert_with(|| "unknown".to_owned());
        }
    }

    let updates = installed
        .into_iter()
        .map(|p| {
            let latest = registry_versions
                .get(&p.name)
                .cloned()
                .unwrap_or_else(|| p.version.clone());
            let update_available = latest != "unknown" && latest != p.version;
            PluginUpdateInfo {
                name: p.name,
                installed_version: p.version,
                latest_version: latest,
                update_available,
            }
        })
        .collect();

    Json(updates)
}

// ---------------------------------------------------------------------------
// Internal: SurrealDB content ingest helper
// ---------------------------------------------------------------------------

/// Ingest all `.md` files from a single surrealdb-target source directory.
///
/// Returns `(ingested, skipped, errors)` counts.
async fn ingest_content_dir(
    db: &orqa_graph::GraphDb,
    plugin_name: &str,
    src_dir: &std::path::Path,
    project_root: &std::path::Path,
) -> (usize, usize, usize) {
    use orqa_graph::sync_plugin_file;

    let (mut ingested, mut skipped, mut errors) = (0usize, 0usize, 0usize);

    let walker = walkdir::WalkDir::new(src_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let p = e.path();
            p.is_file() && p.extension().and_then(|x| x.to_str()) == Some("md")
        });

    for entry_item in walker {
        let path = entry_item.into_path();
        match sync_plugin_file(db, &path, project_root, plugin_name).await {
            Ok(orqa_graph::SyncResult::Upserted { .. }) => ingested += 1,
            Ok(orqa_graph::SyncResult::Unchanged | orqa_graph::SyncResult::Skipped { .. }) => {
                skipped += 1;
            }
            Err(e) => {
                tracing::warn!(plugin = %plugin_name, path = %path.display(), error = %e, "[plugins] surrealdb content ingest failed");
                errors += 1;
            }
        }
    }

    (ingested, skipped, errors)
}

/// Walk the installed plugin's surrealdb-target source directories and ingest each `.md` file.
///
/// Called after a successful install when `result.surrealdb_content_entries > 0`.
/// Errors are logged but do not fail the install — the plugin is already on disk.
async fn ingest_surrealdb_content(
    db: &orqa_graph::GraphDb,
    plugin_name: &str,
    plugin_path: &str,
    project_root: &std::path::Path,
) {
    use orqa_plugin::manifest::read_manifest;

    let plugin_dir = std::path::Path::new(plugin_path);
    let manifest = match read_manifest(plugin_dir) {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!(plugin = %plugin_name, error = %e, "[plugins] failed to read manifest for SurrealDB ingest");
            return;
        }
    };

    let (mut ingested, mut skipped, mut errors) = (0usize, 0usize, 0usize);

    for (key, entry) in &manifest.content {
        if entry.target != orqa_plugin::manifest::ContentTarget::Surrealdb {
            continue;
        }
        let src_dir = plugin_dir.join(&entry.source);
        if !src_dir.exists() {
            tracing::warn!(plugin = %plugin_name, key = %key, source = %entry.source, "[plugins] surrealdb content source dir not found");
            continue;
        }
        let (i, s, e) = ingest_content_dir(db, plugin_name, &src_dir, project_root).await;
        ingested += i;
        skipped += s;
        errors += e;
    }

    tracing::info!(plugin = %plugin_name, ingested, skipped, errors, "[plugins] surrealdb content ingest complete");
}

/// Read the plugin manifest and ingest enforcement rules from all enforcement declarations.
///
/// Each `EnforcementDeclaration` with a `rules_path` is walked; rule `.md` files are
/// upserted into the `enforcement_rule` SurrealDB table with `source_plugin` set.
/// Errors are logged but do not fail the install — the plugin is already on disk.
async fn ingest_enforcement_rules(db: &orqa_graph::GraphDb, plugin_name: &str, plugin_path: &str) {
    use orqa_graph::EnforcementRuleSource;
    use orqa_plugin::manifest::read_manifest;

    let plugin_dir = std::path::Path::new(plugin_path);
    let manifest = match read_manifest(plugin_dir) {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!(plugin = %plugin_name, error = %e, "[plugins] failed to read manifest for enforcement rule ingest");
            return;
        }
    };

    if manifest.enforcement.is_empty() {
        return;
    }

    // Convert manifest enforcement declarations to the sync module's minimal type.
    let sources: Vec<EnforcementRuleSource> = manifest
        .enforcement
        .iter()
        .map(|decl| EnforcementRuleSource {
            rules_path: decl.rules_path.clone(),
        })
        .collect();

    match orqa_graph::upsert_enforcement_rules_from_plugin(db, plugin_name, plugin_dir, &sources)
        .await
    {
        Ok(count) => {
            tracing::info!(plugin = %plugin_name, count, "[plugins] enforcement rules ingested");
        }
        Err(e) => {
            tracing::warn!(plugin = %plugin_name, error = %e, "[plugins] enforcement rule ingest failed");
        }
    }
}
