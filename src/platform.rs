//! Platform configuration loaded from the embedded `core.json`.
//!
//! Self-contained copy of `app/backend/src-tauri/src/domain/platform_config.rs`
//! adapted for the standalone `orqa-mcp-server` crate.
//!
//! Plugin manifests (`plugins/*/orqa-plugin.json` and `connectors/*/orqa-plugin.json`)
//! are scanned at runtime via [`scan_plugin_manifests`] to extend the platform
//! config with plugin-contributed types and relationships.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

/// The platform core config JSON, embedded at compile time from the canonical source.
///
/// Path is relative to this source file: `libs/mcp-server/src/platform.rs`
/// → `libs/types/src/platform/core.json`
const PLATFORM_JSON: &str = include_str!("../../types/src/platform/core.json");

/// A relationship definition from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipDef {
    pub key: String,
    pub inverse: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub from: Vec<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub semantic: Option<String>,
    #[serde(default)]
    pub constraints: Option<ConstraintsDef>,
}

/// Validation constraints for a relationship.
#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintsDef {
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default, rename = "minCount")]
    pub min_count: Option<usize>,
    #[serde(default, rename = "maxCount")]
    pub max_count: Option<usize>,
    #[serde(default, rename = "requireInverse")]
    pub require_inverse: Option<bool>,
    #[serde(default, rename = "statusRules")]
    pub status_rules: Vec<StatusRuleDef>,
}

/// A status-dependent auto-transition rule from the schema.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusRuleDef {
    pub evaluate: String,
    pub condition: String,
    pub statuses: Vec<String>,
    #[serde(rename = "proposedStatus")]
    pub proposed_status: String,
    #[serde(default)]
    pub description: String,
}

/// A semantic category grouping relationship keys by intent.
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticDef {
    pub description: String,
    pub keys: Vec<String>,
}

/// An artifact type from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct ArtifactTypeDef {
    pub key: String,
    pub label: String,
    pub icon: String,
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
}

/// The full platform config parsed from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    #[serde(rename = "artifactTypes")]
    pub artifact_types: Vec<ArtifactTypeDef>,
    pub relationships: Vec<RelationshipDef>,
    pub semantics: HashMap<String, SemanticDef>,
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
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub icon: String,
}

/// Minimal subset of a plugin manifest's `provides.relationships` entry.
#[derive(Debug, Clone, Deserialize)]
struct PluginProvidesRelationship {
    pub key: String,
    pub inverse: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub from: Vec<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub semantic: Option<String>,
    #[serde(default)]
    pub constraints: Option<ConstraintsDef>,
}

/// The `provides` block of a plugin manifest (only the fields we care about).
#[derive(Debug, Clone, Deserialize, Default)]
struct PluginProvides {
    #[serde(default)]
    pub schemas: Vec<PluginProvidesSchema>,
    #[serde(default)]
    pub relationships: Vec<PluginProvidesRelationship>,
}

/// A minimal plugin manifest — only the `provides` block is needed.
#[derive(Debug, Clone, Deserialize)]
struct PluginManifest {
    #[serde(default)]
    pub provides: PluginProvides,
}

/// Contributions collected from all plugin manifests found under `project_root`.
#[derive(Debug, Clone, Default)]
pub struct PluginContributions {
    /// Additional artifact types contributed by plugins (supplements `PLATFORM.artifact_types`).
    pub artifact_types: Vec<ArtifactTypeDef>,
    /// Additional (or extended) relationships contributed by plugins.
    pub relationships: Vec<RelationshipDef>,
}

/// Scan `plugins/*/orqa-plugin.json` and `connectors/*/orqa-plugin.json` under
/// `project_root` and return the combined artifact types and relationships they provide.
///
/// Malformed or unreadable manifests are silently skipped (a `tracing::warn` is
/// emitted so the caller can diagnose issues without crashing).
pub fn scan_plugin_manifests(project_root: &Path) -> PluginContributions {
    let mut contributions = PluginContributions::default();
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
                Err(e) => {
                    tracing::warn!(
                        path = %manifest_path.display(),
                        error = %e,
                        "failed to read plugin manifest — skipping"
                    );
                    continue;
                }
            };

            let manifest: PluginManifest = match serde_json::from_str(&content) {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!(
                        path = %manifest_path.display(),
                        error = %e,
                        "failed to parse plugin manifest — skipping"
                    );
                    continue;
                }
            };

            for schema in manifest.provides.schemas {
                contributions.artifact_types.push(ArtifactTypeDef {
                    key: schema.key,
                    label: schema.label,
                    icon: schema.icon,
                    id_prefix: schema.id_prefix,
                });
            }

            for rel in manifest.provides.relationships {
                contributions.relationships.push(RelationshipDef {
                    key: rel.key,
                    inverse: rel.inverse,
                    label: rel.label,
                    from: rel.from,
                    to: rel.to,
                    description: rel.description,
                    semantic: rel.semantic,
                    constraints: rel.constraints,
                });
            }
        }
    }

    contributions
}
