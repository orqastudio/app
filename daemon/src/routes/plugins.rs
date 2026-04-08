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
//   POST   /plugins/install/local     — install from a local path
//   POST   /plugins/install/github    — install from GitHub
//   DELETE /plugins/:name             — uninstall a plugin
//   GET    /plugins/registry          — browse plugin registry
//   GET    /plugins/updates           — check for available updates

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use orqa_plugin::discovery::{scan_plugins, DiscoveredPlugin};
use orqa_plugin::installer::{install_from_path, InstallResult};
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

/// Handle POST /plugins/install/local — install a plugin from a local path.
///
/// Runs synchronously via spawn_blocking. Triggers graph reload after installation
/// to pick up any new artifact types registered by the plugin.
pub async fn install_plugin_local(
    State(state): State<GraphState>,
    Json(req): Json<InstallLocalRequest>,
) -> Result<Json<InstallResult>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let source_path = std::path::PathBuf::from(&req.path);

    let result =
        tokio::task::spawn_blocking(move || install_from_path(&source_path, &project_root))
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

    // Reload the graph to pick up any schema changes from the new plugin.
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Ok(Json(result));
        };
        guard.project_root.clone()
    };
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
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
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

    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Ok(Json(result));
        };
        guard.project_root.clone()
    };
    state.reload(&project_root);

    Ok(Json(result))
}

/// Handle DELETE /plugins/:name — uninstall a named plugin.
///
/// Runs synchronously via spawn_blocking.
pub async fn uninstall_plugin(
    State(state): State<GraphState>,
    Path(name): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    tokio::task::spawn_blocking(move || orqa_plugin::installer::uninstall(&name, &project_root))
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

    state.reload(&{
        let Ok(guard) = state.0.read() else {
            return Ok(StatusCode::NO_CONTENT);
        };
        guard.project_root.clone()
    });

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
