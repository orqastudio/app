//! Platform configuration sourced from plugin manifests at runtime.
//!
//! Plugins are now the sole source of truth for artifact type schemas and
//! relationships. Plugins live under taxonomy subdirectories:
//! `plugins/<taxonomy>/<plugin>/orqa-plugin.json`. Connectors live at
//! `connectors/<connector>/orqa-plugin.json`. There is no longer a compile-time
//! `core.json` dependency — the `PLATFORM` static provides empty defaults, and
//! all meaningful schema data is loaded via [`scan_plugin_manifests`].

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use crate::types::{RelationshipConstraints, RelationshipSchema, StatusRule};

// Re-export the canonical ArtifactTypeDef from engine/types so all callers get
// the same type regardless of whether they import from validation or engine/types.
pub use orqa_engine_types::platform::ArtifactTypeDef;

/// A relationship definition from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipDef {
    /// Unique relationship key (e.g. `"delivers"`).
    pub key: String,
    /// Inverse relationship key (e.g. `"delivered-by"`).
    pub inverse: String,
    /// Human-readable forward-direction label.
    #[serde(default)]
    pub label: String,
    /// Allowed source artifact types.
    #[serde(default)]
    pub from: Vec<String>,
    /// Allowed target artifact types.
    #[serde(default)]
    pub to: Vec<String>,
    /// Human-readable description of the relationship's meaning.
    #[serde(default)]
    pub description: String,
    /// Semantic category key (e.g. `"dependency"`, `"delivery"`).
    #[serde(default)]
    pub semantic: Option<String>,
    /// Validation constraint block for this relationship.
    #[serde(default)]
    pub constraints: Option<ConstraintsDef>,
}

/// Validation constraints for a relationship.
#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintsDef {
    /// Whether at least one instance of this relationship is required.
    #[serde(default)]
    pub required: Option<bool>,
    /// Minimum number of instances required when `required` is true.
    #[serde(default, rename = "minCount")]
    pub min_count: Option<usize>,
    /// Maximum number of instances allowed.
    #[serde(default, rename = "maxCount")]
    pub max_count: Option<usize>,
    /// Whether the inverse relationship edge must also exist.
    #[serde(default, rename = "requireInverse")]
    pub require_inverse: Option<bool>,
    /// Status-based auto-transition rules.
    #[serde(default, rename = "statusRules")]
    pub status_rules: Vec<StatusRuleDef>,
}

/// A status-dependent auto-transition rule from the schema.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusRuleDef {
    /// Which side to evaluate: `"source"` or `"target"`.
    pub evaluate: String,
    /// Condition to test: `"all-targets-in"`, `"any-target-in"`, `"no-targets-in"`.
    pub condition: String,
    /// Status values to check against.
    pub statuses: Vec<String>,
    /// Status to propose when the condition is met.
    #[serde(rename = "proposedStatus")]
    pub proposed_status: String,
    /// Human-readable description of this rule.
    #[serde(default)]
    pub description: String,
}

/// A semantic category grouping relationship keys by intent.
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticDef {
    /// Human-readable description of what this semantic category means.
    pub description: String,
    /// Relationship keys that belong to this semantic category.
    pub keys: Vec<String>,
}

/// An enforcement mechanism provided by a plugin.
///
/// Rules reference mechanisms by key in their `enforcement` entries.
/// The validator checks that every referenced mechanism is registered.
#[derive(Debug, Clone, Deserialize)]
pub struct EnforcementMechanism {
    /// Unique mechanism key (e.g. "behavioral", "pre-commit", "eslint").
    pub key: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: String,
    /// Strength level (1-10). Higher = stronger enforcement.
    #[serde(default)]
    pub strength: u8,
}

/// The full platform config.
///
/// Plugins are the source of truth — artifact types come from plugin manifests,
/// not from deserialization. This struct is constructed manually.
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    /// Artifact type definitions loaded from plugin manifests.
    pub artifact_types: Vec<ArtifactTypeDef>,
    /// Relationship definitions (empty until plugins are loaded).
    pub relationships: Vec<RelationshipDef>,
    /// Semantic category definitions keyed by category name.
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
    /// JSON Schema for frontmatter validation (raw JSON value).
    #[serde(default)]
    pub frontmatter: serde_json::Value,
    /// Optional status transition map declared in the plugin manifest.
    #[serde(default, rename = "statusTransitions")]
    pub status_transitions: Option<HashMap<String, Vec<String>>>,
    /// When set, this schema extends another type's schema via allOf composition.
    /// The value is the key of the base schema to extend.
    #[serde(default)]
    pub extends: Option<String>,
}

/// A schema extension collected during plugin scanning.
/// Applied to the base type's schema via allOf composition.
#[derive(Debug, Clone)]
pub struct SchemaExtension {
    /// The artifact type key being extended.
    pub target_key: String,
    /// Additional JSON Schema to compose via allOf.
    pub frontmatter_schema: serde_json::Value,
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
    #[serde(default)]
    pub enforcement_mechanisms: Vec<EnforcementMechanism>,
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
    /// Schema extensions: plugins that extend another type's frontmatter schema.
    pub schema_extensions: Vec<SchemaExtension>,
    /// Enforcement mechanisms registered by plugins.
    pub enforcement_mechanisms: Vec<EnforcementMechanism>,
}

/// Scan plugin manifests under `project_root` and return the combined artifact
/// types and relationships they provide.
///
/// Scanning strategy:
/// - `plugins/` — two levels deep (`plugins/<taxonomy>/<plugin>/orqa-plugin.json`)
///   because plugins are now organised into taxonomy subdirectories.
/// - `connectors/` — one level deep (`connectors/<connector>/orqa-plugin.json`)
///   because connectors are not organised into taxonomy subdirectories.
///
/// Malformed or unreadable manifests are silently skipped (a `tracing::warn` is
/// emitted so the caller can diagnose issues without crashing).
pub fn scan_plugin_manifests(project_root: &Path) -> PluginContributions {
    let mut contributions = PluginContributions::default();

    // Scan plugins/ two levels deep: plugins/<taxonomy>/<plugin>/orqa-plugin.json.
    let plugins_dir = project_root.join("plugins");
    if let Ok(taxonomy_entries) = std::fs::read_dir(&plugins_dir) {
        for taxonomy_entry in taxonomy_entries.flatten() {
            let Ok(inner) = std::fs::read_dir(taxonomy_entry.path()) else {
                continue;
            };
            for plugin_entry in inner.flatten() {
                let manifest_path = plugin_entry.path().join("orqa-plugin.json");
                if let Some(manifest) = load_plugin_manifest(&manifest_path) {
                    apply_manifest(manifest, &mut contributions);
                }
            }
        }
    }

    // Scan connectors/ one level deep: connectors/<connector>/orqa-plugin.json.
    let connectors_dir = project_root.join("connectors");
    if let Ok(entries) = std::fs::read_dir(&connectors_dir) {
        for entry in entries.flatten() {
            let manifest_path = entry.path().join("orqa-plugin.json");
            if let Some(manifest) = load_plugin_manifest(&manifest_path) {
                apply_manifest(manifest, &mut contributions);
            }
        }
    }

    contributions
}

/// Load and parse a single plugin manifest from disk.
///
/// Returns `None` and emits a warning if the file is missing, unreadable, or malformed.
fn load_plugin_manifest(manifest_path: &Path) -> Option<PluginManifest> {
    if !manifest_path.exists() {
        return None;
    }
    let content = match std::fs::read_to_string(manifest_path) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(
                path = %manifest_path.display(),
                error = %e,
                "failed to read plugin manifest — skipping"
            );
            return None;
        }
    };
    match serde_json::from_str(&content) {
        Ok(m) => Some(m),
        Err(e) => {
            tracing::warn!(
                path = %manifest_path.display(),
                error = %e,
                "failed to parse plugin manifest — skipping"
            );
            None
        }
    }
}

/// Apply a parsed plugin manifest's contributions to the accumulated `PluginContributions`.
///
/// Processes schemas (base definitions and extensions), relationships, and enforcement mechanisms.
fn apply_manifest(manifest: PluginManifest, contributions: &mut PluginContributions) {
    for schema in manifest.provides.schemas {
        apply_schema(schema, contributions);
    }
    contributions
        .enforcement_mechanisms
        .extend(manifest.provides.enforcement_mechanisms);
    for rel in manifest.provides.relationships {
        contributions.relationships.push(plugin_rel_to_schema(rel));
    }
}

/// Classify a plugin schema as a base type definition or an extension, and add it to contributions.
///
/// Schemas with a null frontmatter block default to an open object schema.
fn apply_schema(schema: PluginProvidesSchema, contributions: &mut PluginContributions) {
    let frontmatter_schema = if schema.frontmatter.is_null() {
        serde_json::json!({ "type": "object", "additionalProperties": true })
    } else {
        schema.frontmatter.clone()
    };

    if let Some(target_key) = schema.extends {
        contributions.schema_extensions.push(SchemaExtension {
            target_key,
            frontmatter_schema,
        });
    } else {
        contributions.artifact_types.push(ArtifactTypeDef {
            key: schema.key,
            label: schema.label,
            icon: schema.icon,
            id_prefix: schema.id_prefix,
            frontmatter_schema,
            status_transitions: schema.status_transitions.unwrap_or_default(),
        });
    }
}

/// Convert a plugin-provided relationship to the canonical `RelationshipSchema` type.
fn plugin_rel_to_schema(rel: PluginProvidesRelationship) -> RelationshipSchema {
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
    RelationshipSchema {
        key: rel.key,
        inverse: rel.inverse,
        description: rel.description,
        from: rel.from,
        to: rel.to,
        semantic: rel.semantic,
        constraints,
    }
}
