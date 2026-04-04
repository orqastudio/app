//! Project domain types for the OrqaStudio engine.
//!
//! Defines structs representing projects managed by OrqaStudio, including detected
//! technology stacks and scan results. Projects are the top-level container for
//! sessions, artifacts, and governance content.
//!
//! # ID representation
//!
//! `id` and `project_id` fields are raw `i64` SQLite rowids. Full newtype wrappers
//! (`ProjectId(i64)`) would require changes across `orqa-storage`,
//! `orqa-engine-types`, and both Tauri backends simultaneously. The current
//! representation is kept as `i64` to preserve a single migration boundary.
//! The storage layer is the correct place to introduce typed IDs when that
//! refactor is scoped.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A project managed by OrqaStudio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Database row ID.
    pub id: i64,
    /// Display name of the project.
    pub name: String,
    /// Absolute filesystem path to the project root.
    pub path: String,
    /// Optional description of the project's purpose.
    pub description: Option<String>,
    /// Technology stack detected in the project directory.
    pub detected_stack: Option<DetectedStack>,
    /// ISO-8601 timestamp when this project was added to OrqaStudio.
    pub created_at: String,
    /// ISO-8601 timestamp of the last update to this project record.
    pub updated_at: String,
}

/// A lightweight summary of a project for list views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    /// Database row ID.
    pub id: i64,
    /// Display name of the project.
    pub name: String,
    /// Absolute filesystem path to the project root.
    pub path: String,
    /// Technology stack detected in the project directory.
    pub detected_stack: Option<DetectedStack>,
    /// Number of sessions recorded for this project.
    pub session_count: i64,
    /// Number of governance artifacts found in `.orqa/`.
    pub artifact_count: i64,
    /// ISO-8601 timestamp of the last update.
    pub updated_at: String,
}

/// The technology stack detected in a project directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedStack {
    /// Programming languages detected (e.g. `["rust", "typescript"]`).
    pub languages: Vec<String>,
    /// Frameworks detected (e.g. `["tauri", "svelte"]`).
    pub frameworks: Vec<String>,
    /// Primary package manager detected (e.g. `"npm"`, `"cargo"`).
    pub package_manager: Option<String>,
    /// Whether a Claude Code configuration file was found.
    pub has_claude_config: bool,
    /// Whether design token files were found (e.g. `tailwind.config.ts`).
    pub has_design_tokens: bool,
}

/// The result of scanning a project directory for artifacts and stack information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// ID of the project that was scanned.
    pub project_id: i64,
    /// Technology stack detected during the scan.
    pub detected_stack: DetectedStack,
    /// Count of artifacts found per type key.
    pub artifact_counts: HashMap<String, i64>,
    /// Whether design token files were found during the scan.
    pub design_tokens_found: bool,
    /// How long the scan took in milliseconds.
    pub scan_duration_ms: u64,
}
