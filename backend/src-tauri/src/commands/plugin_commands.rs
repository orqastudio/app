//! Plugin management Tauri commands — install, uninstall, registry browsing.

use crate::error::OrqaError;
use crate::plugins::{discovery, installer, lockfile};
use crate::state::AppState;

use super::helpers::active_project_path;

/// List all installed plugins discovered from the plugins/ directory.
#[tauri::command]
pub fn plugin_list_installed(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<discovery::DiscoveredPlugin>, OrqaError> {
    let project_path = active_project_path(&state)?;
    Ok(discovery::scan_plugins(std::path::Path::new(&project_path)))
}

/// Fetch the plugin registry catalog (official, community, or both).
#[tauri::command]
pub async fn plugin_registry_list(
    source: Option<String>,
) -> Result<serde_json::Value, OrqaError> {
    let src = source.as_deref().unwrap_or("official");
    let cache = crate::plugins::registry::RegistryCache::new();
    let catalog = cache.fetch(src).await?;
    serde_json::to_value(&catalog).map_err(|e| OrqaError::Serialization(e.to_string()))
}

/// Install a plugin from a local path.
#[tauri::command]
pub fn plugin_install_local(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<installer::InstallResult, OrqaError> {
    let project_path = active_project_path(&state)?;
    installer::install_from_path(
        std::path::Path::new(&path),
        std::path::Path::new(&project_path),
    )
}

/// Install a plugin from a GitHub release archive.
#[tauri::command]
pub async fn plugin_install_github(
    repo: String,
    version: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<installer::InstallResult, OrqaError> {
    let project_path = active_project_path(&state)?;
    installer::install_from_github(
        &repo,
        version.as_deref(),
        std::path::Path::new(&project_path),
    )
    .await
}

/// Uninstall a plugin by name.
#[tauri::command]
pub fn plugin_uninstall(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), OrqaError> {
    let project_path = active_project_path(&state)?;
    installer::uninstall(&name, std::path::Path::new(&project_path))
}

/// Check for available plugin updates by comparing lockfile versions against registry.
#[tauri::command]
pub async fn plugin_check_updates(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let project_root = std::path::Path::new(&project_path);
    let lock = lockfile::read_lockfile(project_root);

    if lock.plugins.is_empty() {
        return Ok(vec![]);
    }

    // Fetch both registries
    let cache = crate::plugins::registry::RegistryCache::new();
    let official = cache.fetch("official").await.unwrap_or_default();
    let community = cache.fetch("community").await.unwrap_or_default();

    let all_registry: Vec<_> = official
        .plugins
        .iter()
        .chain(community.plugins.iter())
        .collect();

    let mut updates = Vec::new();
    for locked in &lock.plugins {
        if let Some(entry) = all_registry.iter().find(|e| e.name == locked.name) {
            // Simple version comparison — in production this would be semver-aware
            if entry.name != locked.name {
                continue;
            }
            updates.push(serde_json::json!({
                "name": locked.name,
                "currentVersion": locked.version,
                "repo": locked.repo,
                "registryName": entry.display_name,
            }));
        }
    }

    Ok(updates)
}

/// Read the plugin manifest for a specific installed plugin.
#[tauri::command]
pub fn plugin_get_manifest(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, OrqaError> {
    let project_path = active_project_path(&state)?;
    let project_root = std::path::Path::new(&project_path);
    let plugins_dir = project_root.join("plugins");

    let short_name = name.split('/').last().unwrap_or(&name);
    let plugin_dir = plugins_dir.join(short_name);

    let manifest = crate::plugins::manifest::read_manifest(&plugin_dir)?;
    serde_json::to_value(&manifest).map_err(|e| OrqaError::Serialization(e.to_string()))
}
