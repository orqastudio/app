// Project settings types shared across engine domain crates.
//
// Contains the status definition and auto-rule types used by the workflow
// transition engine. These are pure data shapes extracted from the engine's
// project/settings module to avoid circular dependencies.

use serde::{Deserialize, Serialize};

/// An automatic transition rule on a status definition.
///
/// `condition` is a named condition evaluated by the transition engine.
/// `target` is the status key to transition to when the condition is met.
/// `params` carries condition-specific configuration (e.g. `child_type` for
/// `all-children-completed` and `all-p1-children-completed` so the engine
/// does not hardcode artifact type names).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusAutoRule {
    pub condition: String,
    pub target: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
}

/// A status definition loaded from `project.json`.
///
/// Extends the validation lib's `StatusDefinition` with `auto_rules` for the
/// transition engine (app-specific feature).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDefinition {
    pub key: String,
    pub label: String,
    pub icon: String,
    #[serde(default)]
    pub spin: bool,
    /// Ordered list of status keys that can be manually transitioned to from this status.
    #[serde(default)]
    pub transitions: Vec<String>,
    /// Automatic transition rules evaluated by the transition engine.
    #[serde(default)]
    pub auto_rules: Vec<StatusAutoRule>,
}
