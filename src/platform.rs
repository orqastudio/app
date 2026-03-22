//! Platform configuration sourced from plugin manifests at runtime.
//!
//! Plugins (`plugins/*/orqa-plugin.json` and `connectors/*/orqa-plugin.json`)
//! are now the sole source of truth for artifact type schemas and relationships.
//! There is no longer a compile-time `core.json` dependency — the `PLATFORM`
//! static provides empty defaults, and all meaningful schema data is loaded via
//! [`scan_plugin_manifests`].

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use crate::types::{RelationshipConstraints, RelationshipSchema, StatusRule};

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

/// Frontmatter field requirements for an artifact type, as declared in a plugin manifest.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FrontmatterSchema {
    /// Fields that must be present in every artifact of this type.
    #[serde(default)]
    pub required: Vec<String>,
    /// Fields that are allowed but not required.
    #[serde(default)]
    pub optional: Vec<String>,
}

/// An artifact type definition, contributed by a plugin manifest.
#[derive(Debug, Clone, Deserialize)]
pub struct ArtifactTypeDef {
    pub key: String,
    pub label: String,
    pub icon: String,
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
    /// Frontmatter fields that are required for this artifact type.
    #[serde(default)]
    pub frontmatter_required: Vec<String>,
    /// Frontmatter fields that are optional for this artifact type.
    #[serde(default)]
    pub frontmatter_optional: Vec<String>,
    /// Valid status transitions: maps each status key to the statuses it may transition to.
    #[serde(default)]
    pub status_transitions: HashMap<String, Vec<String>>,
}

/// The full platform config parsed from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    #[serde(rename = "artifactTypes")]
    pub artifact_types: Vec<ArtifactTypeDef>,
    pub relationships: Vec<RelationshipDef>,
    pub semantics: HashMap<String, SemanticDef>,
}

/// Platform config with empty defaults.
///
/// Plugins are now the source of truth for artifact types, relationships, and
/// semantics. This static retains the same surface area so existing code that
/// reads `PLATFORM.relationships` or `PLATFORM.artifact_types` continues to
/// compile — it simply receives empty slices until plugins are loaded via
/// [`scan_plugin_manifests`].
pub static PLATFORM: LazyLock<PlatformConfig> = LazyLock::new(|| PlatformConfig {
    artifact_types: Vec::new(),
    relationships: Vec::new(),
    semantics: HashMap::new(),
});

// ---------------------------------------------------------------------------
// Plugin manifest scanning
// ---------------------------------------------------------------------------

/// Minimal subset of a plugin manifest's `provides.schemas` entry needed for validation.
#[derive(Debug, Clone, Deserialize)]
struct PluginProvidesSchema {
    pub key: String,
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub icon: String,
    /// Optional frontmatter requirements declared in the plugin manifest.
    #[serde(default)]
    pub frontmatter: Option<FrontmatterSchema>,
    /// Optional status transition map declared in the plugin manifest.
    #[serde(default, rename = "statusTransitions")]
    pub status_transitions: Option<HashMap<String, Vec<String>>>,
}

/// Minimal subset of a plugin manifest's `provides.relationships` entry.
#[derive(Debug, Clone, Deserialize)]
struct PluginProvidesRelationship {
    pub key: String,
    pub inverse: String,
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
    /// Additional artifact types contributed by plugins.
    pub artifact_types: Vec<ArtifactTypeDef>,
    /// Additional (or extended) relationships contributed by plugins, in the
    /// canonical `RelationshipSchema` form accepted by `build_validation_context`.
    pub relationships: Vec<RelationshipSchema>,
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
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
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
                let (frontmatter_required, frontmatter_optional) = schema
                    .frontmatter
                    .map(|f| (f.required, f.optional))
                    .unwrap_or_default();
                contributions.artifact_types.push(ArtifactTypeDef {
                    key: schema.key,
                    label: schema.label,
                    icon: schema.icon,
                    id_prefix: schema.id_prefix,
                    frontmatter_required,
                    frontmatter_optional,
                    status_transitions: schema.status_transitions.unwrap_or_default(),
                });
            }

            for rel in manifest.provides.relationships {
                let constraints = rel.constraints.map(|c| RelationshipConstraints {
                    required: c.required,
                    min_count: c.min_count,
                    max_count: c.max_count,
                    require_inverse: c.require_inverse,
                    status_rules: c
                        .status_rules
                        .into_iter()
                        .map(|sr| StatusRule {
                            evaluate: sr.evaluate,
                            condition: sr.condition,
                            statuses: sr.statuses,
                            proposed_status: sr.proposed_status,
                            description: sr.description,
                        })
                        .collect(),
                });
                contributions.relationships.push(RelationshipSchema {
                    key: rel.key,
                    inverse: rel.inverse,
                    description: rel.description,
                    from: rel.from,
                    to: rel.to,
                    semantic: rel.semantic,
                    constraints,
                });
            }
        }
    }

    contributions
}
