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
    /// The plugin's package name (e.g. `@orqastudio/plugin-software`).
    pub name: String,
    /// Semantic version string from the manifest (e.g. `1.2.0`).
    pub version: String,
    /// Human-readable display name from the manifest, if present.
    pub display_name: Option<String>,
    /// One-line description from the manifest, if present.
    pub description: Option<String>,
    /// Absolute filesystem path to the plugin directory.
    pub path: String,
    /// Where this plugin came from (always `"installed"` for discovered plugins).
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
        let manifest_path = plugin_path.join("orqa-plugin.json");

        // Read raw JSON to extract name/version/description. This avoids
        // failures from strict Rust struct deserialization (e.g., agent
        // definitions missing required fields like `title`).
        let Ok(contents) = std::fs::read_to_string(&manifest_path) else {
            continue;
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents) else {
            continue;
        };
        let Some(name) = json.get("name").and_then(|v| v.as_str()) else {
            continue;
        };

        discovered.push(DiscoveredPlugin {
            name: name.to_owned(),
            version: json
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("0.0.0")
                .to_owned(),
            display_name: json
                .get("displayName")
                .and_then(|v| v.as_str())
                .map(str::to_owned),
            description: json
                .get("description")
                .and_then(|v| v.as_str())
                .map(str::to_owned),
            path: plugin_path.to_string_lossy().into_owned(),
            source: "installed".to_owned(),
        });
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
