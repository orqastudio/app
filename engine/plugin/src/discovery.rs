//! Plugin discovery — scan for plugins registered in project.json.
//!
//! Only plugins with `installed: true` AND `enabled: true` in project.json
//! are returned. Changing either field triggers an artifact graph rebuild.
//! No fallback directory scanning — if a plugin isn't registered, it isn't loaded.

use serde::Serialize;
use std::path::Path;

use orqa_engine_types::config::{load_project_settings, PluginProjectConfig};

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

    let discovered: Vec<DiscoveredPlugin> = settings
        .plugins
        .values()
        .filter(|c| c.installed && c.enabled)
        .filter_map(|config| discover_one(project_root, config))
        .collect();

    tracing::info!(
        count = discovered.len(),
        "[plugins] discovered {} plugins",
        discovered.len()
    );

    discovered
}

/// Try to read and parse a single plugin manifest, returning `None` on failure.
fn discover_one(project_root: &Path, config: &PluginProjectConfig) -> Option<DiscoveredPlugin> {
    let plugin_path = project_root.join(&config.path);
    let manifest_path = plugin_path.join("orqa-plugin.json");

    let contents = std::fs::read_to_string(&manifest_path)
        .inspect_err(|e| {
            tracing::warn!(path = %manifest_path.display(), error = %e, "[plugins] failed to read plugin manifest");
        })
        .ok()?;

    let json: serde_json::Value = serde_json::from_str(&contents)
        .inspect_err(|e| {
            tracing::warn!(path = %plugin_path.display(), error = %e, "[plugins] failed to parse plugin manifest JSON");
        })
        .ok()?;

    let name = json.get("name").and_then(|v| v.as_str())?;

    Some(DiscoveredPlugin {
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
    })
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

    /// Write a minimal project.json with the given plugin entries.
    fn write_project_json(dir: &Path, content: &str) {
        let orqa_dir = dir.join(".orqa");
        std::fs::create_dir_all(&orqa_dir).expect("create .orqa");
        std::fs::write(orqa_dir.join("project.json"), content).expect("write project.json");
    }

    #[test]
    fn disabled_plugin_is_not_returned() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_project_json(
            dir.path(),
            r#"{"id":"proj","name":"test","plugins":{"my-plugin":{"path":"plugins/my-plugin","installed":true,"enabled":false}}}"#,
        );
        let plugins = scan_plugins(dir.path());
        assert!(plugins.is_empty(), "disabled plugin should not be returned");
    }

    #[test]
    fn not_installed_plugin_is_not_returned() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_project_json(
            dir.path(),
            r#"{"id":"proj","name":"test","plugins":{"my-plugin":{"path":"plugins/my-plugin","installed":false,"enabled":true}}}"#,
        );
        let plugins = scan_plugins(dir.path());
        assert!(
            plugins.is_empty(),
            "uninstalled plugin should not be returned"
        );
    }

    #[test]
    fn installed_and_enabled_plugin_with_manifest_is_returned() {
        let dir = tempfile::tempdir().expect("tempdir");

        // Create the plugin directory and manifest.
        let plugin_dir = dir.path().join("plugins").join("my-plugin");
        std::fs::create_dir_all(&plugin_dir).expect("create plugin dir");
        std::fs::write(
            plugin_dir.join("orqa-plugin.json"),
            r#"{"name":"@test/my-plugin","version":"1.0.0","displayName":"My Plugin","description":"A test plugin"}"#,
        )
        .expect("write manifest");

        write_project_json(
            dir.path(),
            r#"{"id":"proj","name":"test","plugins":{"my-plugin":{"path":"plugins/my-plugin","installed":true,"enabled":true}}}"#,
        );

        let plugins = scan_plugins(dir.path());
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].name, "@test/my-plugin");
        assert_eq!(plugins[0].version, "1.0.0");
        assert_eq!(plugins[0].display_name.as_deref(), Some("My Plugin"));
        assert_eq!(plugins[0].description.as_deref(), Some("A test plugin"));
        assert_eq!(plugins[0].source, "installed");
    }

    #[test]
    fn plugin_manifest_missing_name_field_is_skipped() {
        let dir = tempfile::tempdir().expect("tempdir");

        // Manifest without a name field — plugin should be skipped.
        let plugin_dir = dir.path().join("plugins").join("broken-plugin");
        std::fs::create_dir_all(&plugin_dir).expect("create plugin dir");
        std::fs::write(
            plugin_dir.join("orqa-plugin.json"),
            r#"{"version":"1.0.0"}"#, // no "name"
        )
        .expect("write manifest");

        write_project_json(
            dir.path(),
            r#"{"id":"proj","name":"test","plugins":{"broken-plugin":{"path":"plugins/broken-plugin","installed":true,"enabled":true}}}"#,
        );

        let plugins = scan_plugins(dir.path());
        assert!(plugins.is_empty(), "plugin without name should be skipped");
    }

    #[test]
    fn plugin_with_missing_version_defaults_to_zero() {
        let dir = tempfile::tempdir().expect("tempdir");

        let plugin_dir = dir.path().join("plugins").join("no-version");
        std::fs::create_dir_all(&plugin_dir).expect("create plugin dir");
        std::fs::write(
            plugin_dir.join("orqa-plugin.json"),
            r#"{"name":"@test/no-version"}"#, // no "version"
        )
        .expect("write manifest");

        write_project_json(
            dir.path(),
            r#"{"id":"proj","name":"test","plugins":{"no-version":{"path":"plugins/no-version","installed":true,"enabled":true}}}"#,
        );

        let plugins = scan_plugins(dir.path());
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].version, "0.0.0");
    }
}
