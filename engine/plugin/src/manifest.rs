//! Plugin manifest reader — Rust representation of the PluginManifest type.
//!
//! Reads and validates `orqa-plugin.json` from a plugin directory.
//! The manifest is the authoritative descriptor of what a plugin provides.

use serde::{Deserialize, Serialize};
use std::path::Path;

use orqa_engine_types::error::EngineError;

/// Where a content entry is installed at `orqa plugin install` time.
///
/// `Surrealdb` — source files are parsed and ingested as artifact records into SurrealDB
/// with `source_plugin = <name>`. No files are written to `.orqa/<artifact-dir>/`.
/// `Runtime` — source files are byte-copied to `install_path` (e.g. `.claude/agents`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentTarget {
    /// Artifact content ingested into SurrealDB.
    Surrealdb,
    /// Runtime code copied to `install_path` on disk.
    Runtime,
}

/// A single content directory declared in a plugin manifest.
///
/// After migration (S2-08), every entry carries an explicit `target` that
/// tells the installer which sink to use. Schema validation rejects entries
/// where `target` is absent or an unrecognised value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEntry {
    /// Source directory, relative to the plugin root (e.g. `"knowledge"`).
    pub source: String,
    /// Destination path, relative to the project root (e.g. `".orqa/documentation/knowledge"`
    /// for surrealdb entries or `".claude/agents"` for runtime entries).
    #[serde(rename = "installPath")]
    pub install_path: String,
    /// Which install sink this entry targets. Required — schema validation rejects absent values.
    pub target: ContentTarget,
}

/// Minimal Rust representation of a plugin manifest.
///
/// Only the fields the engine needs are parsed. The full manifest is handled
/// by the TypeScript SDK on the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// The plugin's package name (e.g. `@orqastudio/plugin-software`).
    pub name: String,
    /// Semantic version string (e.g. `1.2.0`).
    pub version: String,
    /// Human-readable display name shown in the UI.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// One-line description of the plugin.
    pub description: Option<String>,
    /// The plugin's role(s) in the taxonomy. A plugin can appear in multiple categories.
    /// Valid enforcement-related values: "enforcement-generator", "enforcement-contributor".
    #[serde(default)]
    pub categories: Vec<String>,
    /// Enforcement declarations — generators and contributors register here.
    #[serde(default)]
    pub enforcement: Vec<EnforcementDeclaration>,
    /// Other plugin names this plugin depends on (must be installed first).
    #[serde(default)]
    pub plugin_dependencies: Vec<String>,
    /// What this plugin contributes to the engine at runtime.
    pub provides: PluginProvides,
    /// Recorded decisions from previous installations when relationship or
    /// artifact type keys collided with core or other plugins.
    /// Key: the relationship/schema key. Value: the decision made.
    #[serde(default, rename = "mergeDecisions")]
    pub merge_decisions: Vec<MergeDecision>,
    /// Default navigation tree contributed by this plugin.
    /// Methodology and workflow plugins define how the sidebar should be structured
    /// when no explicit project navigation is configured in project.json.
    #[serde(default, rename = "defaultNavigation")]
    pub default_navigation: Vec<serde_json::Value>,
    /// Installation constraints: purpose classification, stage slot, and
    /// post-install action flags. All fields default to safe values when absent.
    /// These fields are top-level in the manifest JSON (not nested under a sub-object).
    #[serde(flatten)]
    pub install_constraints: PluginInstallConstraints,
    /// Content directory declarations split by install sink.
    /// After S2-08 migration, every entry carries an explicit `target` (surrealdb | runtime).
    /// Entries with absent or unrecognised `target` values are rejected at install time
    /// with a clear error — no silent defaults.
    #[serde(default, deserialize_with = "deserialize_content_entries")]
    pub content: std::collections::HashMap<String, ContentEntry>,
}

/// Declares how this plugin participates in the enforcement pipeline.
///
/// A plugin with role "generator" owns an enforcement engine (e.g. eslint, tsconfig)
/// and produces generated config. A plugin with role "contributor" feeds rules into
/// another plugin's generator engine via `contributes_to`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementDeclaration {
    /// Sub-type of enforcement participation: "generator" or "contributor".
    pub role: String,
    /// Engine name for generators — becomes the `orqa enforce --<engine>` CLI flag.
    #[serde(default)]
    pub engine: Option<String>,
    /// Path (relative to project root) where the generated config is written.
    /// Always under `.orqa/configs/`. Required for generators.
    #[serde(default)]
    pub config_output: Option<String>,
    /// Path (relative to plugin root) to the generator script/binary. Required for generators.
    #[serde(default)]
    pub generator: Option<String>,
    /// Commands for running enforcement checks and fixes. Required for generators.
    #[serde(default)]
    pub actions: Option<EnforcementActions>,
    /// File paths the daemon watches to trigger regeneration. Required for generators.
    #[serde(default)]
    pub watch: Option<WatchDeclaration>,
    /// File patterns this engine operates on — used for `--staged` filtering.
    #[serde(default)]
    pub file_types: Vec<String>,
    /// Path (relative to plugin root) to the plugin's own rule files,
    /// installed to `.orqa/learning/rules/<domain>/`.
    #[serde(default)]
    pub rules_path: Option<String>,
    /// For contributors: identifies which generator this feeds into.
    /// Format: `<plugin-name>:<engine>` (e.g. `@orqastudio/plugin-typescript:eslint`).
    #[serde(default)]
    pub contributes_to: Option<String>,
}

/// Check and fix commands for an enforcement engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementActions {
    /// Command for running the enforcement check.
    pub check: ActionDeclaration,
    /// Optional command for auto-fixing violations. Not all engines support fix.
    pub fix: Option<ActionDeclaration>,
}

/// A single enforcement action (check or fix).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDeclaration {
    /// The binary/tool to invoke (e.g. "eslint").
    pub command: String,
    /// Command-line arguments passed to the binary.
    pub args: Vec<String>,
    /// Optional glob pattern for the files this action operates on.
    ///
    /// When omitted, the enforcement declaration's `file_types` (one level
    /// up) already describes the action's file scope — most generators
    /// (clippy, tsc, cargo) run workspace-wide and don't need a per-action
    /// glob. Left as an optional escape hatch for tools like eslint that
    /// take an explicit file list on the command line.
    #[serde(default)]
    pub files: Option<String>,
}

/// File watch declaration for a generator — triggers regeneration on change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchDeclaration {
    /// Glob patterns (relative to project root) that the daemon watches.
    pub paths: Vec<String>,
    /// Optional YAML frontmatter query to filter which rule files trigger this generator.
    #[serde(default)]
    pub filter: Option<String>,
    /// Action to take on file change. Currently only "regenerate" is supported.
    pub on_change: String,
}

/// A recorded decision about a key collision during plugin installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeDecision {
    /// The relationship or schema key that collided.
    pub key: String,
    /// What was decided: "merged" (union from/to) or "renamed" (key namespaced).
    pub decision: String,
    /// The original key name if renamed (e.g. "merged-into" before becoming "sw-merged-into").
    #[serde(rename = "originalKey", skip_serializing_if = "Option::is_none")]
    pub original_key: Option<String>,
    /// The source that owns the existing definition (e.g. "core" or a plugin name).
    #[serde(rename = "existingSource")]
    pub existing_source: String,
}

/// An agent role definition declared by a plugin.
///
/// Supports two shapes simultaneously so plugins can use whichever fits best:
///
/// 1. **Inline** — `{ id, title, description, preamble, capabilities }`.
///    The full prompt metadata is embedded in the manifest.
/// 2. **File reference** — `{ key, id, path }`. The plugin points to an
///    external markdown file in its `agents/` directory (synced to
///    `.claude/agents/` at install time) and the prompt pipeline loads
///    the content from disk.
///
/// Every field is optional on deserialization so the parser accepts both
/// shapes.  Runtime code that needs `title` for display falls back to `id`
/// via [`AgentDefinition::display_title`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefinition {
    /// Unique identifier for this agent (e.g. "orchestrator" or "AGENT-65b56a0b").
    pub id: String,
    /// Optional stable dedup key — used by install-time sync to detect renames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Optional path to the external agent markdown file, relative to the
    /// plugin root (e.g. `"agents/AGENT-65b56a0b.md"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional human-readable name shown in the UI and logs.  When absent,
    /// callers should fall back to [`AgentDefinition::display_title`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// One-sentence description of the agent's purpose.
    #[serde(default)]
    pub description: String,
    /// The opening instruction injected into the agent's system prompt.
    #[serde(default)]
    pub preamble: String,
    /// Tool capability identifiers this agent is granted access to.
    #[serde(default)]
    pub capabilities: Vec<String>,
}

impl AgentDefinition {
    /// Return the agent's display title, falling back to `id` when `title`
    /// is absent (file-reference shape).
    #[must_use]
    pub fn display_title(&self) -> &str {
        self.title.as_deref().unwrap_or(&self.id)
    }
}

/// A plugin-registered custom viewer for a specific artifact type.
///
/// ExplorerRouter checks these declarations before falling back to the
/// default ArtifactViewer component. Enables plugins to supply rich,
/// type-specific rendering without modifying core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactViewerDeclaration {
    /// Artifact type key this viewer handles (e.g. "task", "lesson").
    pub artifact_type: String,
    /// View component key registered in `provides.views` (e.g. "task-kanban-view").
    pub view_key: String,
}

/// A plugin-provided agent role definition with a system-prompt template.
///
/// Core-framework provides the eight base roles. Other plugins extend or
/// override via the `role_definitions` list. The prompt pipeline merges
/// all installed role definitions before generating per-agent prompts (P1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    /// Unique role identifier (e.g. "implementer", "reviewer").
    pub role: String,
    /// Mustache-style system prompt template for this role.
    pub prompt_template: String,
    /// One-sentence description of this role's purpose.
    pub description: String,
}

/// A plugin-registered settings page declaration.
///
/// SettingsCategoryNav reads these declarations from the plugin registry
/// and renders the matching view component in the settings panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsPageDeclaration {
    /// Unique page identifier (e.g. "plugin-software-settings").
    pub id: String,
    /// Display label shown in the settings sidebar (e.g. "Software Project").
    pub label: String,
    /// Settings section this page belongs to (e.g. "plugins", "integrations").
    pub section: String,
    /// View component key registered in `provides.views`.
    pub view_key: String,
}

/// What a plugin declares it provides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProvides {
    /// JSON Schema definitions for artifact types this plugin declares.
    #[serde(default)]
    pub schemas: Vec<serde_json::Value>,
    /// View component definitions contributed by this plugin.
    #[serde(default)]
    pub views: Vec<serde_json::Value>,
    /// Dashboard widget definitions contributed by this plugin.
    #[serde(default)]
    pub widgets: Vec<serde_json::Value>,
    /// Relationship type definitions contributed by this plugin.
    #[serde(default)]
    pub relationships: Vec<serde_json::Value>,
    /// Optional sidecar process configuration.
    pub sidecar: Option<serde_json::Value>,
    /// CLI tool registrations contributed by this plugin.
    #[serde(default, rename = "cliTools")]
    pub cli_tools: Vec<serde_json::Value>,
    /// Lifecycle hook registrations contributed by this plugin.
    #[serde(default)]
    pub hooks: Vec<serde_json::Value>,
    /// Agent role definitions contributed by this plugin.
    #[serde(default)]
    pub agents: Vec<AgentDefinition>,
    /// Custom artifact viewer declarations — maps artifact types to plugin view components.
    /// ExplorerRouter checks these before falling back to the generic ArtifactViewer.
    #[serde(default)]
    pub artifact_viewers: Vec<ArtifactViewerDeclaration>,
    /// Role definitions with system prompt templates contributed by this plugin.
    /// Merged across all installed plugins by the prompt generation pipeline.
    #[serde(default)]
    pub role_definitions: Vec<RoleDefinition>,
    /// Settings page declarations — each entry registers a page in the settings panel.
    /// SettingsCategoryNav reads these from the plugin registry.
    #[serde(default)]
    pub settings_pages: Vec<SettingsPageDeclaration>,
}

/// Installation constraint metadata declared in a plugin manifest.
///
/// These fields govern what the installer must enforce and what post-install
/// actions it must trigger. They are separate from `provides` because they
/// describe the plugin's installation behaviour rather than its runtime contributions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginInstallConstraints {
    /// The plugin's role(s) in the methodology composition pipeline.
    /// Valid values: "methodology", "workflow", "knowledge", "connector",
    /// "infrastructure", "app_extension", "sidecar".
    /// A single plugin may serve multiple purposes.
    #[serde(default)]
    pub purpose: Vec<String>,

    /// For workflow plugins: the methodology stage slot this plugin fills.
    /// Must be unique — only one plugin may occupy each slot per project.
    /// Non-workflow plugins leave this field absent.
    #[serde(default)]
    pub stage_slot: Option<String>,

    /// True when installing this plugin must trigger full schema recomposition.
    /// Definition plugins (methodology, workflow) set this to true.
    /// Knowledge, views, and infrastructure plugins set it to false.
    /// Missing field defaults to false (safe default).
    #[serde(default)]
    pub affects_schema: bool,
}

// ---------------------------------------------------------------------------
// Content entry deserialization with validation
// ---------------------------------------------------------------------------

/// Deserialize the `content` map from JSON, validating each entry's `target` field.
///
/// Rejects entries where `target` is absent (the field is required post-S2-08 migration)
/// by returning a deserialization error. Also emits lint warnings for entries that are
/// present but had to fall back via the standard serde path — in practice this function
/// enforces the "no defaults" rule at parse time.
fn deserialize_content_entries<'de, D>(
    deserializer: D,
) -> Result<std::collections::HashMap<String, ContentEntry>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    // Deserialize as raw JSON first so we can inspect field presence before
    // attempting the typed deserialization.
    let raw: std::collections::HashMap<String, serde_json::Value> =
        std::collections::HashMap::deserialize(deserializer)?;

    let mut out = std::collections::HashMap::new();

    for (key, val) in &raw {
        // Validate that `target` is explicitly present and a recognised string.
        let target_val = val.get("target");
        match target_val {
            None => {
                return Err(D::Error::custom(format!(
                    "content entry '{key}': missing required field 'target' \
                     (must be \"surrealdb\" or \"runtime\")"
                )));
            }
            Some(serde_json::Value::String(s)) if s != "surrealdb" && s != "runtime" => {
                return Err(D::Error::custom(format!(
                    "content entry '{key}': invalid target value '{s}' \
                     (must be \"surrealdb\" or \"runtime\")"
                )));
            }
            _ => {} // valid
        }

        let entry: ContentEntry = serde_json::from_value(val.clone())
            .map_err(|e| D::Error::custom(format!("content entry '{key}': {e}")))?;

        out.insert(key.clone(), entry);
    }

    Ok(out)
}

const MANIFEST_FILENAME: &str = "orqa-plugin.json";

/// Read a plugin manifest from a directory.
///
/// Returns `EngineError::Plugin` if the manifest file is absent, and
/// `EngineError::Serialization` if the JSON cannot be parsed.
pub fn read_manifest(plugin_dir: &Path) -> Result<PluginManifest, EngineError> {
    let manifest_path = plugin_dir.join(MANIFEST_FILENAME);

    if !manifest_path.exists() {
        return Err(EngineError::Plugin(format!(
            "manifest not found: {}",
            manifest_path.display()
        )));
    }

    let contents = std::fs::read_to_string(&manifest_path)?;
    let manifest: PluginManifest = serde_json::from_str(&contents)?;

    Ok(manifest)
}

/// Validate a plugin manifest, returning a list of error messages.
///
/// An empty return value means the manifest is valid. Checks that required
/// fields are non-empty and that the categories array is non-empty.
pub fn validate_manifest(manifest: &PluginManifest) -> Vec<String> {
    let mut errors = Vec::new();

    if manifest.name.is_empty() {
        errors.push("missing required field: name".to_owned());
    }

    if manifest.version.is_empty() {
        errors.push("missing required field: version".to_owned());
    }

    if manifest.categories.is_empty() {
        errors.push("missing required field: categories (must be non-empty)".to_owned());
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_minimal_manifest() {
        // A minimal manifest with required fields should deserialize without error.
        let json = r#"{
            "name": "@orqastudio/test-plugin",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {
                "schemas": [],
                "views": [],
                "widgets": [],
                "relationships": []
            }
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.name, "@orqastudio/test-plugin");
        assert_eq!(manifest.version, "0.1.0");
        assert_eq!(manifest.categories, vec!["domain-knowledge"]);
        assert!(manifest.provides.cli_tools.is_empty());
        assert!(manifest.enforcement.is_empty());
        assert!(manifest.plugin_dependencies.is_empty());
    }

    #[test]
    fn validate_rejects_empty_name() {
        // An empty name must produce a validation error.
        let manifest = PluginManifest {
            name: String::new(),
            version: "0.1.0".to_owned(),
            display_name: None,
            description: None,
            categories: vec!["domain-knowledge".to_owned()],
            enforcement: vec![],
            plugin_dependencies: vec![],
            provides: PluginProvides {
                schemas: vec![],
                views: vec![],
                widgets: vec![],
                relationships: vec![],
                sidecar: None,
                cli_tools: vec![],
                hooks: vec![],
                agents: vec![],
                artifact_viewers: vec![],
                role_definitions: vec![],
                settings_pages: vec![],
            },
            merge_decisions: vec![],
            default_navigation: vec![],
            install_constraints: PluginInstallConstraints::default(),
            content: std::collections::HashMap::new(),
        };

        let errors = validate_manifest(&manifest);
        assert!(errors.iter().any(|e| e.contains("name")));
    }

    #[test]
    fn validate_rejects_empty_version() {
        // An empty version must produce a validation error.
        let manifest = PluginManifest {
            name: "@orqastudio/test".to_owned(),
            version: String::new(),
            display_name: None,
            description: None,
            categories: vec!["domain-knowledge".to_owned()],
            enforcement: vec![],
            plugin_dependencies: vec![],
            provides: PluginProvides {
                schemas: vec![],
                views: vec![],
                widgets: vec![],
                relationships: vec![],
                sidecar: None,
                cli_tools: vec![],
                hooks: vec![],
                agents: vec![],
                artifact_viewers: vec![],
                role_definitions: vec![],
                settings_pages: vec![],
            },
            merge_decisions: vec![],
            default_navigation: vec![],
            install_constraints: PluginInstallConstraints::default(),
            content: std::collections::HashMap::new(),
        };

        let errors = validate_manifest(&manifest);
        assert!(errors.iter().any(|e| e.contains("version")));
    }

    #[test]
    fn validate_rejects_empty_categories() {
        // An empty categories array must produce a validation error.
        let manifest = PluginManifest {
            name: "@orqastudio/test".to_owned(),
            version: "0.1.0".to_owned(),
            display_name: None,
            description: None,
            categories: vec![],
            enforcement: vec![],
            plugin_dependencies: vec![],
            provides: PluginProvides {
                schemas: vec![],
                views: vec![],
                widgets: vec![],
                relationships: vec![],
                sidecar: None,
                cli_tools: vec![],
                hooks: vec![],
                agents: vec![],
                artifact_viewers: vec![],
                role_definitions: vec![],
                settings_pages: vec![],
            },
            merge_decisions: vec![],
            default_navigation: vec![],
            install_constraints: PluginInstallConstraints::default(),
            content: std::collections::HashMap::new(),
        };

        let errors = validate_manifest(&manifest);
        assert!(errors.iter().any(|e| e.contains("categories")));
    }

    #[test]
    fn deserialize_manifest_with_agents() {
        // Agent definitions should round-trip through JSON correctly.
        let json = r#"{
            "name": "@orqastudio/core-framework",
            "version": "0.1.0",
            "categories": ["infrastructure"],
            "provides": {
                "agents": [
                    {
                        "id": "orchestrator",
                        "title": "Orchestrator",
                        "description": "Coordinates ephemeral task-scoped workers.",
                        "preamble": "Coordinate and delegate, never implement.",
                        "capabilities": ["file_read"]
                    }
                ]
            }
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.provides.agents.len(), 1);
        assert_eq!(manifest.provides.agents[0].id, "orchestrator");
        assert_eq!(manifest.provides.agents[0].capabilities, vec!["file_read"]);
    }

    #[test]
    fn install_constraints_default_to_safe_values_when_absent() {
        // A manifest without installConstraints should default all flags to false
        // and purpose/stageSlot to empty/None. This is the safe default per P5-28.
        let json = r#"{
            "name": "@orqastudio/knowledge-plugin",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {}
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert!(manifest.install_constraints.purpose.is_empty());
        assert!(manifest.install_constraints.stage_slot.is_none());
        assert!(!manifest.install_constraints.affects_schema);
    }

    #[test]
    fn install_constraints_deserialized_when_present() {
        // A methodology plugin with full install constraints should round-trip correctly.
        // Fields are top-level in the manifest JSON, using snake_case names.
        let json = r#"{
            "name": "@orqastudio/plugin-agile-methodology",
            "version": "0.1.0",
            "categories": ["methodology"],
            "provides": {},
            "purpose": ["methodology"],
            "affects_schema": true
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.install_constraints.purpose, vec!["methodology"]);
        assert!(manifest.install_constraints.stage_slot.is_none());
        assert!(manifest.install_constraints.affects_schema);
    }

    #[test]
    fn install_constraints_stage_slot_deserialized() {
        // A workflow plugin with stage_slot should deserialize correctly.
        // Fields are top-level in the manifest JSON, using snake_case names.
        let json = r#"{
            "name": "@orqastudio/plugin-agile-discovery",
            "version": "0.1.0",
            "categories": ["workflow"],
            "provides": {},
            "purpose": ["workflow"],
            "stage_slot": "discovery",
            "affects_schema": true
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(
            manifest.install_constraints.stage_slot.as_deref(),
            Some("discovery")
        );
    }

    #[test]
    fn deserialize_enforcement_generator_declaration() {
        // An enforcement-generator plugin should deserialize its enforcement block correctly.
        let json = r#"{
            "name": "@orqastudio/plugin-typescript",
            "version": "0.2.0-dev",
            "categories": ["domain-knowledge", "enforcement-generator"],
            "enforcement": [
                {
                    "role": "generator",
                    "engine": "eslint",
                    "config_output": ".orqa/configs/eslint.config.js",
                    "generator": "scripts/generate-eslint-config.ts",
                    "actions": {
                        "check": {
                            "command": "eslint",
                            "args": ["--config", ".orqa/configs/eslint.config.js"],
                            "files": "*.{ts,svelte,js}"
                        }
                    },
                    "watch": {
                        "paths": [".orqa/learning/rules/**/*.md"],
                        "on_change": "regenerate"
                    },
                    "file_types": ["*.ts", "*.js"],
                    "rules_path": "rules/eslint/"
                }
            ],
            "provides": {}
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.enforcement.len(), 1);
        let decl = &manifest.enforcement[0];
        assert_eq!(decl.role, "generator");
        assert_eq!(decl.engine.as_deref(), Some("eslint"));
        assert_eq!(
            decl.config_output.as_deref(),
            Some(".orqa/configs/eslint.config.js")
        );
        assert!(decl.actions.is_some());
        let actions = decl.actions.as_ref().unwrap();
        assert_eq!(actions.check.command, "eslint");
        assert!(actions.fix.is_none());
        let watch = decl.watch.as_ref().unwrap();
        assert_eq!(watch.on_change, "regenerate");
        assert_eq!(decl.file_types, vec!["*.ts", "*.js"]);
    }

    #[test]
    fn deserialize_enforcement_contributor_declaration() {
        // An enforcement-contributor plugin should deserialize its enforcement block correctly.
        let json = r#"{
            "name": "@orqastudio/plugin-svelte",
            "version": "0.2.0-dev",
            "categories": ["domain-knowledge", "enforcement-contributor"],
            "plugin_dependencies": ["@orqastudio/plugin-typescript"],
            "enforcement": [
                {
                    "role": "contributor",
                    "contributes_to": "@orqastudio/plugin-typescript:eslint",
                    "rules_path": "rules/eslint/"
                }
            ],
            "provides": {}
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.enforcement.len(), 1);
        assert_eq!(
            manifest.plugin_dependencies,
            vec!["@orqastudio/plugin-typescript"]
        );
        let decl = &manifest.enforcement[0];
        assert_eq!(decl.role, "contributor");
        assert_eq!(
            decl.contributes_to.as_deref(),
            Some("@orqastudio/plugin-typescript:eslint")
        );
        assert_eq!(decl.rules_path.as_deref(), Some("rules/eslint/"));
    }

    #[test]
    fn read_manifest_returns_error_when_file_absent() {
        let dir = tempfile::tempdir().unwrap();
        let result = read_manifest(dir.path());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("manifest not found") || err.contains("not found"));
    }

    #[test]
    fn read_manifest_returns_error_on_invalid_json() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(MANIFEST_FILENAME), "not json").unwrap();
        let result = read_manifest(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn read_manifest_succeeds_with_valid_file() {
        let dir = tempfile::tempdir().unwrap();
        let json = r#"{
            "name": "@orqastudio/plugin-test",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {}
        }"#;
        std::fs::write(dir.path().join(MANIFEST_FILENAME), json).unwrap();
        let manifest = read_manifest(dir.path()).unwrap();
        assert_eq!(manifest.name, "@orqastudio/plugin-test");
        assert_eq!(manifest.version, "0.1.0");
    }

    #[test]
    fn validate_manifest_passes_for_valid_manifest() {
        let json = r#"{
            "name": "@orqastudio/plugin-test",
            "version": "1.0.0",
            "categories": ["methodology"],
            "provides": {}
        }"#;
        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        let errors = validate_manifest(&manifest);
        assert!(
            errors.is_empty(),
            "valid manifest produced errors: {errors:?}"
        );
    }

    #[test]
    fn validate_manifest_reports_all_missing_fields() {
        let manifest = PluginManifest {
            name: String::new(),
            version: String::new(),
            display_name: None,
            description: None,
            categories: vec![],
            enforcement: vec![],
            plugin_dependencies: vec![],
            provides: PluginProvides {
                schemas: vec![],
                views: vec![],
                widgets: vec![],
                relationships: vec![],
                sidecar: None,
                cli_tools: vec![],
                hooks: vec![],
                agents: vec![],
                artifact_viewers: vec![],
                role_definitions: vec![],
                settings_pages: vec![],
            },
            merge_decisions: vec![],
            default_navigation: vec![],
            install_constraints: PluginInstallConstraints::default(),
            content: std::collections::HashMap::new(),
        };
        let errors = validate_manifest(&manifest);
        assert!(
            errors.len() >= 3,
            "expected at least 3 errors, got: {errors:?}"
        );
    }

    #[test]
    fn deserialize_manifest_with_merge_decisions() {
        let json = r#"{
            "name": "@orqastudio/plugin-software",
            "version": "0.1.0",
            "categories": ["methodology"],
            "provides": {},
            "mergeDecisions": [
                {
                    "key": "delivers",
                    "decision": "merged",
                    "existingSource": "core"
                }
            ]
        }"#;
        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.merge_decisions.len(), 1);
        assert_eq!(manifest.merge_decisions[0].key, "delivers");
        assert_eq!(manifest.merge_decisions[0].decision, "merged");
        assert_eq!(manifest.merge_decisions[0].existing_source, "core");
        assert!(manifest.merge_decisions[0].original_key.is_none());
    }

    #[test]
    fn deserialize_manifest_with_settings_page() {
        let json = r#"{
            "name": "@orqastudio/plugin-test",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {
                "settings_pages": [
                    {
                        "id": "plugin-test-settings",
                        "label": "Test Settings",
                        "section": "plugins",
                        "view_key": "PluginTestSettings"
                    }
                ]
            }
        }"#;
        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.provides.settings_pages.len(), 1);
        assert_eq!(
            manifest.provides.settings_pages[0].id,
            "plugin-test-settings"
        );
        assert_eq!(manifest.provides.settings_pages[0].section, "plugins");
    }

    /// Canonical surrealdb content entry parses into the typed `ContentEntry`.
    #[test]
    fn content_entry_surrealdb_target_parsed() {
        let json = r#"{
            "name": "@orqastudio/plugin-test",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {},
            "content": {
                "knowledge": {
                    "source": "knowledge",
                    "installPath": ".orqa/documentation/knowledge",
                    "target": "surrealdb"
                }
            }
        }"#;
        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        let entry = manifest.content.get("knowledge").expect("knowledge entry");
        assert_eq!(entry.source, "knowledge");
        assert_eq!(entry.install_path, ".orqa/documentation/knowledge");
        assert_eq!(entry.target, ContentTarget::Surrealdb);
    }

    /// Canonical runtime content entry parses into the typed `ContentEntry`.
    #[test]
    fn content_entry_runtime_target_parsed() {
        let json = r#"{
            "name": "@orqastudio/plugin-test",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {},
            "content": {
                "agents": {
                    "source": "agents",
                    "installPath": ".claude/agents",
                    "target": "runtime"
                }
            }
        }"#;
        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        let entry = manifest.content.get("agents").expect("agents entry");
        assert_eq!(entry.source, "agents");
        assert_eq!(entry.install_path, ".claude/agents");
        assert_eq!(entry.target, ContentTarget::Runtime);
    }

    /// A missing `target` field must be rejected at parse time with a clear error
    /// mentioning the entry key and the valid values. No silent default is allowed
    /// — the installer's strict-validation contract depends on this.
    #[test]
    fn content_entry_missing_target_rejected() {
        let json = r#"{
            "name": "@orqastudio/plugin-test",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {},
            "content": {
                "docs": {
                    "source": "docs",
                    "installPath": ".orqa/documentation"
                }
            }
        }"#;
        let err = serde_json::from_str::<PluginManifest>(json)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("docs") && err.contains("missing") && err.contains("target"),
            "unexpected error message: {err}"
        );
    }

    /// An unrecognised `target` value (e.g. "files") must be rejected at parse
    /// time with a clear error naming the bad value and the valid set.
    #[test]
    fn content_entry_invalid_target_rejected() {
        let json = r#"{
            "name": "@orqastudio/plugin-test",
            "version": "0.1.0",
            "categories": ["domain-knowledge"],
            "provides": {},
            "content": {
                "docs": {
                    "source": "docs",
                    "installPath": ".orqa/documentation",
                    "target": "files"
                }
            }
        }"#;
        let err = serde_json::from_str::<PluginManifest>(json)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("docs") && err.contains("invalid") && err.contains("files"),
            "unexpected error message: {err}"
        );
    }
}
