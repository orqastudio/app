//! Plugin discovery — scan for plugins registered in project.json.
//!
//! Only plugins with `installed: true` AND `enabled: true` in project.json
//! are returned. Changing either field triggers an artifact graph rebuild.
//! No fallback directory scanning — if a plugin isn't registered, it isn't loaded.

use serde::Serialize;
use std::path::Path;

use super::manifest::read_manifest;
use orqa_engine_types::config::load_project_settings;

/// A discovered plugin from scanning the project.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveredPlugin {
    pub name: String,
    pub version: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub path: String,
    pub source: String,
}

/// Scan for plugins that are registered as installed AND enabled in project.json.
///
/// Only plugins explicitly registered in project.json are discovered.
/// Returns an empty vec if the project has no settings or no enabled plugins.
pub fn scan_plugins(project_root: &Path) -> Vec<DiscoveredPlugin> {
    let Some(settings) = load_project_settings(project_root).ok().flatten() else {
        return vec![];
    };

    let mut discovered = Vec::new();

    for config in settings.plugins.values() {
        if !config.installed || !config.enabled {
            continue;
        }

        let plugin_path = project_root.join(&config.path);

        if let Ok(manifest) = read_manifest(&plugin_path) {
            discovered.push(DiscoveredPlugin {
                name: manifest.name.clone(),
                version: manifest.version.clone(),
                display_name: manifest.display_name.clone(),
                description: manifest.description.clone(),
                path: plugin_path.to_string_lossy().to_string(),
                source: "installed".to_string(),
            });
        }
    }

    discovered
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn scan_empty_project() {
        // A nonexistent project root returns an empty list without panicking.
        let plugins = scan_plugins(&PathBuf::from("/nonexistent"));
        assert!(plugins.is_empty());
    }
}
