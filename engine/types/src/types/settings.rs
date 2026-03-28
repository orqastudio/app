//! Settings domain types for the OrqaStudio engine.
//!
//! Defines structs and enums for theme tokens and sidecar status. Theme tokens
//! support the design token extraction pipeline; sidecar status tracks the
//! lifecycle of the LLM inference sidecar process.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The fully resolved theme for a project, merging extracted tokens with overrides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedTheme {
    /// ID of the project this theme belongs to.
    pub project_id: i64,
    /// All resolved tokens, keyed by token name.
    pub tokens: HashMap<String, ThemeToken>,
    /// Source files that tokens were extracted from.
    pub source_files: Vec<String>,
    /// Whether any user-level overrides are present.
    pub has_overrides: bool,
}

/// A single design token with light/dark values and its origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeToken {
    /// Token name (e.g. `"color-primary"`, `"font-size-base"`).
    pub name: String,
    /// CSS value for light mode.
    pub value_light: String,
    /// CSS value for dark mode, if different from light.
    pub value_dark: Option<String>,
    /// Where this token value originated.
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
    /// Current lifecycle state of the sidecar.
    pub state: SidecarState,
    /// OS process ID of the sidecar, if running.
    pub pid: Option<u32>,
    /// Seconds since the sidecar process started.
    pub uptime_seconds: Option<u64>,
    /// Whether a compatible CLI binary was found on `PATH`.
    pub cli_detected: bool,
    /// Version string of the detected CLI binary.
    pub cli_version: Option<String>,
    /// Human-readable error message when `state` is `Error`.
    pub error_message: Option<String>,
}

/// The lifecycle state of the sidecar process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SidecarState {
    /// Sidecar has not been started yet.
    NotStarted,
    /// Sidecar process is launching and not yet ready.
    Starting,
    /// Sidecar is running and accepting requests.
    Connected,
    /// Sidecar encountered an error and is not functional.
    Error,
    /// Sidecar was stopped cleanly.
    Stopped,
}
