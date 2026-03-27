// Settings domain types for the OrqaStudio engine.
//
// Defines structs and enums for theme tokens and sidecar status. Theme tokens
// support the design token extraction pipeline; sidecar status tracks the
// lifecycle of the LLM inference sidecar process.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The fully resolved theme for a project, merging extracted tokens with overrides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedTheme {
    pub project_id: i64,
    pub tokens: HashMap<String, ThemeToken>,
    pub source_files: Vec<String>,
    pub has_overrides: bool,
}

/// A single design token with light/dark values and its origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeToken {
    pub name: String,
    pub value_light: String,
    pub value_dark: Option<String>,
    pub source: ThemeTokenSource,
}

/// Where a theme token value originated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ThemeTokenSource {
    /// Extracted from the project's design token files (e.g. tailwind.config.ts).
    Extracted,
    /// Set by the user as a project-level override.
    Override,
    /// Provided by the engine's built-in defaults.
    Default,
}

/// Current operational status of the LLM inference sidecar process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarStatus {
    pub state: SidecarState,
    pub pid: Option<u32>,
    pub uptime_seconds: Option<u64>,
    pub cli_detected: bool,
    pub cli_version: Option<String>,
    pub error_message: Option<String>,
}

/// The lifecycle state of the sidecar process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SidecarState {
    NotStarted,
    Starting,
    Connected,
    Error,
    Stopped,
}
