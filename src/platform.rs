//! Platform configuration loaded from the embedded `core.json`.
//!
//! Minimal subset of the platform config needed for artifact type inference
//! from ID prefixes (e.g. "RULE-006" → "rule").
//!
//! Plugin manifests (`plugins/*/orqa-plugin.json` and `connectors/*/orqa-plugin.json`)
//! are scanned at runtime via [`scan_plugin_manifests`] to merge additional
//! artifact types into the platform type list.

use serde::Deserialize;
use std::path::Path;
use std::sync::LazyLock;

/// The platform core config JSON, embedded at compile time from the canonical source.
///
/// Path is relative to this source file: `libs/lsp-server/src/platform.rs`
/// → `libs/types/src/platform/core.json`
const PLATFORM_JSON: &str = include_str!("../../types/src/platform/core.json");

/// An artifact type from core.json — key and ID prefix only.
#[derive(Debug, Clone, Deserialize)]
pub struct ArtifactTypeDef {
    pub key: String,
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
}

/// Minimal platform config needed for type inference.
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    #[serde(rename = "artifactTypes")]
    pub artifact_types: Vec<ArtifactTypeDef>,
}

/// Lazily-parsed platform config, available for the lifetime of the process.
pub static PLATFORM: LazyLock<PlatformConfig> = LazyLock::new(|| {
    serde_json::from_str(PLATFORM_JSON).expect("platform core.json must be valid JSON")
});

// ---------------------------------------------------------------------------
// Plugin manifest scanning
// ---------------------------------------------------------------------------

/// Minimal subset of a plugin manifest's `provides.schemas` entry.
#[derive(Debug, Clone, Deserialize)]
struct PluginProvidesSchema {
    pub key: String,
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
}

/// The `provides` block of a plugin manifest (only the fields we care about).
#[derive(Debug, Clone, Deserialize, Default)]
struct PluginProvides {
    #[serde(default)]
    pub schemas: Vec<PluginProvidesSchema>,
}

/// A minimal plugin manifest — only the `provides` block is needed.
#[derive(Debug, Clone, Deserialize)]
struct PluginManifest {
    #[serde(default)]
    pub provides: PluginProvides,
}

/// Scan `plugins/*/orqa-plugin.json` and `connectors/*/orqa-plugin.json` under
/// `project_root` and return the artifact type definitions they contribute.
///
/// These supplement `PLATFORM.artifact_types` for ID-prefix → type-key inference.
/// Malformed or unreadable manifests are silently skipped.
pub fn scan_plugin_manifests(project_root: &Path) -> Vec<ArtifactTypeDef> {
    let mut types = Vec::new();
    let search_dirs = ["plugins", "connectors"];

    for search_dir in &search_dirs {
        let dir = project_root.join(search_dir);
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let manifest_path = entry.path().join("orqa-plugin.json");
            if !manifest_path.exists() {
                continue;
            }

            let content = match std::fs::read_to_string(&manifest_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let manifest: PluginManifest = match serde_json::from_str(&content) {
                Ok(m) => m,
                Err(_) => continue,
            };

            for schema in manifest.provides.schemas {
                types.push(ArtifactTypeDef {
                    key: schema.key,
                    id_prefix: schema.id_prefix,
                });
            }
        }
    }

    types
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_core_json_is_empty() {
        // core.json is now empty — all artifact types come from plugin manifests.
        assert!(
            PLATFORM.artifact_types.is_empty(),
            "core.json should be empty; plugins are the source of truth"
        );
    }

    #[test]
    fn scan_plugin_manifests_finds_types() {
        // This test requires running from the project root. Skip if not available.
        let project_root = std::env::current_dir()
            .ok()
            .and_then(|p| {
                let mut candidate = p.as_path();
                while !candidate.join("plugins").exists() {
                    candidate = candidate.parent()?;
                }
                Some(candidate.to_path_buf())
            });
        let Some(root) = project_root else { return };

        let types = scan_plugin_manifests(&root);
        assert!(!types.is_empty(), "Should find artifact types from plugins");

        let rule = types.iter().find(|t| t.id_prefix == "RULE");
        assert!(rule.is_some(), "Should find RULE type from agile-governance plugin");
        assert_eq!(rule.map(|t| t.key.as_str()), Some("rule"));
    }
}
