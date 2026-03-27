// Governance scan domain types for the OrqaStudio engine.
//
// Defines structs used to represent the result of scanning the filesystem for
// governance files (rules, hooks, agents, etc.). These are surfaced in the
// governance UI to show coverage and health of the governance setup.

use serde::{Deserialize, Serialize};

/// Result of scanning the filesystem for governance files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceScanResult {
    pub areas: Vec<GovernanceArea>,
    pub coverage_ratio: f64,
}

/// A governance area (rules, hooks, agents, etc.) found during scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceArea {
    pub name: String,
    pub source: String,
    pub files: Vec<GovernanceFile>,
    pub covered: bool,
}

/// A single governance file found on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceFile {
    pub path: String,
    pub size_bytes: u64,
    pub content_preview: String,
}
