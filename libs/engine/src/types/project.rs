// Project domain types for the OrqaStudio engine.
//
// Defines structs representing projects managed by OrqaStudio, including detected
// technology stacks and scan results. Projects are the top-level container for
// sessions, artifacts, and governance content.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A project managed by OrqaStudio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub detected_stack: Option<DetectedStack>,
    pub created_at: String,
    pub updated_at: String,
}

/// A lightweight summary of a project for list views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub detected_stack: Option<DetectedStack>,
    pub session_count: i64,
    pub artifact_count: i64,
    pub updated_at: String,
}

/// The technology stack detected in a project directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedStack {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub package_manager: Option<String>,
    pub has_claude_config: bool,
    pub has_design_tokens: bool,
}

/// The result of scanning a project directory for artifacts and stack information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub project_id: i64,
    pub detected_stack: DetectedStack,
    pub artifact_counts: HashMap<String, i64>,
    pub design_tokens_found: bool,
    pub scan_duration_ms: u64,
}
