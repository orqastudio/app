// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: cargo build -p orqa-validation

#![allow(dead_code, unused_imports, missing_docs)]

use serde::{Deserialize, Serialize};

/// A domain-specific branch contributed by a plugin to the agent decision tree. Branches extend the orchestrator and implementer reasoning protocols with domain context, helping agents form better search queries and classify work correctly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTreeBranch {
    /// The reasoning mode this branch applies to. Matches Step 1 classifications in the decision tree.
    pub mode: String,
    /// Unique domain key (lowercase, hyphenated). Used as the display label in the injected tree.
    pub domain: String,
    /// Human-readable description of when this domain applies. Shown inline in the decision tree injection.
    pub description: String,
    /// Comma-separated keywords that help the agent form a targeted search_semantic query when working in this domain.
    pub search_context: String,
}

/// Decision tree extension contributed by a plugin. Each branch adds a domain-specific entry to the agent reasoning protocol injected on every UserPromptSubmit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDecisionTree {
    /// Domain branches to merge into the orchestrator and implementer decision trees.
    pub branches: Vec<DecisionTreeBranch>,
}

/// An artifact type schema contributed by a plugin. Plugin schemas must declare key, label, icon, idPrefix, and frontmatter. They may use $ref to reference core types in core.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginArtifactSchema {
    /// Unique artifact type key (lowercase, hyphenated). MUST NOT collide with core artifact type keys.
    pub key: String,
    /// Singular display label.
    pub label: String,
    /// Plural display label. Defaults to label + 's' if omitted.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub plural: Option<String>,
    /// Lucide icon name.
    pub icon: String,
    /// Default relative path within .orqa/ for storing artifacts of this type.
    #[serde(rename = "defaultPath")]
    pub default_path: String,
    /// ID prefix for auto-generated IDs (e.g. 'EPIC', 'TASK'). MUST NOT collide with core ID prefixes.
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
    /// Frontmatter field declarations for this artifact type.
    pub frontmatter: serde_json::Value,
    /// Status transition rules specific to this artifact type.
    #[serde(rename = "statusTransitions", skip_serializing_if = "Option::is_none", default)]
    pub status_transitions: Option<serde_json::Value>,
}

/// A relationship type contributed by a plugin. Relationships MUST include key, inverse, from, to, and semantic to integrate correctly with the validation engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRelationshipExtension {
    /// Relationship type key (lowercase, hyphenated). MUST NOT collide with core relationship keys.
    pub key: String,
    /// Inverse relationship key. May be the same as key for symmetric relationships.
    pub inverse: String,
    /// Human-readable label for the forward direction.
    pub label: String,
    /// Human-readable label for the inverse direction.
    #[serde(rename = "inverseLabel")]
    pub inverse_label: String,
    /// Artifact type keys that are valid sources for this relationship.
    pub from: Vec<String>,
    /// Artifact type keys that are valid targets for this relationship.
    pub to: Vec<String>,
    /// Human-readable explanation of what this relationship means.
    pub description: String,
    /// The semantic category this relationship belongs to. Determines how the validation engine treats it.
    pub semantic: String,
    /// Optional validation constraints for this relationship.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub constraints: Option<serde_json::Value>,
}

/// The 'provides' block of a plugin manifest. Declares all capabilities the plugin registers with the app at install time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifestProvides {
    /// Artifact type schemas this plugin introduces. Each schema extends the core type system.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub schemas: Option<Vec<PluginArtifactSchema>>,
    /// Relationship types this plugin introduces. Each relationship extends the core relationship vocabulary.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub relationships: Option<Vec<PluginRelationshipExtension>>,
    /// Knowledge artifact paths contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub knowledge: Option<Vec<String>>,
    /// Agent artifact paths contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agents: Option<Vec<String>>,
    /// Custom view registrations contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub views: Option<Vec<serde_json::Value>>,
    /// Dashboard widget registrations contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub widgets: Option<Vec<serde_json::Value>>,
    /// Hook registrations contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hooks: Option<Vec<String>>,
    /// Agent decision tree branches contributed by this plugin. Branches are merged at runtime into the orchestrator and implementer reasoning protocols injected on every UserPromptSubmit.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub decision_tree: Option<PluginDecisionTree>,
    /// LSP server registrations contributed by this plugin.
    #[serde(rename = "lspServers", skip_serializing_if = "Option::is_none", default)]
    pub lsp_servers: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Development tool registrations contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tools: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Dependencies required by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dependencies: Option<serde_json::Value>,
    /// CLI tool registrations contributed by this plugin.
    #[serde(rename = "cliTools", skip_serializing_if = "Option::is_none", default)]
    pub cli_tools: Option<Vec<serde_json::Value>>,
    /// Behavioral rules contributed by this plugin. Appended to BEHAVIORAL_RULES in the prompt-injector.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub behavioral_rules: Option<Vec<String>>,
    /// Mode templates contributed by this plugin. Merged into MODE_TEMPLATES. Plugin keys must not collide with built-in mode keys.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode_templates: Option<std::collections::HashMap<String, String>>,
    /// Session state reminders contributed by this plugin. Appended to sessionConstant in the prompt-injector.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub session_reminders: Option<Vec<String>>,
}

/// The full orqa-plugin.json manifest for an OrqaStudio plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Package name. Scoped (@org/name) or unscoped (name).
    pub name: String,
    /// Semver version string.
    pub version: String,
    /// Human-readable name shown in the plugin manager UI.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none", default)]
    pub display_name: Option<String>,
    /// Short description of what the plugin does.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<serde_json::Value>,
    /// SPDX license identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub license: Option<String>,
    /// Plugin dependencies — names of plugins that must be loaded first.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub requires: Option<Vec<String>>,
    /// Plugin category. The app requires at least one plugin from each of: thinking, delivery, governance.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub category: Option<String>,
    /// Minimum versions required for this plugin to function.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub compatibility: Option<serde_json::Value>,
    pub provides: PluginManifestProvides,
    /// Semantic category definitions contributed by this plugin. Merged with platform semantics at runtime.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub semantics: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Artifact link display configuration contributed by this plugin.
    #[serde(rename = "artifactLinks", skip_serializing_if = "Option::is_none", default)]
    pub artifact_links: Option<serde_json::Value>,
}

/// Result of validating a plugin manifest at install time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInstallValidationResult {
    /// Whether the plugin manifest passed all validation checks.
    pub valid: bool,
    /// Blocking validation errors that must be resolved before installation.
    pub errors: Vec<String>,
    /// Non-blocking warnings about the plugin manifest.
    pub warnings: Vec<String>,
    /// Schema or relationship keys that conflict with existing definitions.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub key_collisions: Option<Vec<String>>,
}

