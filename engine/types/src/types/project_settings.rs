//! Project settings types shared across engine domain crates.
//!
//! Contains the status definition and auto-rule types used by the workflow
//! transition engine. These are pure data shapes extracted from the engine's
//! project/settings module to avoid circular dependencies.

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
    /// Named condition to evaluate (e.g. `"all-children-completed"`).
    pub condition: String,
    /// Status key to transition to when the condition is met.
    pub target: String,
    /// Condition-specific parameters (e.g. `child_type`).
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
}

/// A status definition loaded from `project.json`.
///
/// Extends the validation lib's `StatusDefinition` with `auto_rules` for the
/// transition engine (app-specific feature).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDefinition {
    /// Unique status key (e.g. `"active"`, `"closed"`).
    pub key: String,
    /// Human-readable label shown in the UI.
    pub label: String,
    /// Icon identifier for the UI.
    pub icon: String,
    /// Whether the status icon should spin (indicates in-progress state).
    #[serde(default)]
    pub spin: bool,
    /// Ordered list of status keys that can be manually transitioned to from this status.
    #[serde(default)]
    pub transitions: Vec<String>,
    /// Automatic transition rules evaluated by the transition engine.
    #[serde(default)]
    pub auto_rules: Vec<StatusAutoRule>,
}
