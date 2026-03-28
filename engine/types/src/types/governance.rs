//! Governance scan domain types for the OrqaStudio engine.
//!
//! Defines structs used to represent the result of scanning the filesystem for
//! governance files (rules, hooks, agents, etc.). These are surfaced in the
//! governance UI to show coverage and health of the governance setup.

use serde::{Deserialize, Serialize};

/// Result of scanning the filesystem for governance files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceScanResult {
    /// All governance areas discovered during the scan.
    pub areas: Vec<GovernanceArea>,
    /// Fraction of governance areas that are covered (0.0–1.0).
    pub coverage_ratio: f64,
}

/// A governance area (rules, hooks, agents, etc.) found during scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceArea {
    /// Display name of the governance area (e.g. "Rules", "Agents").
    pub name: String,
    /// Source identifier (e.g. plugin key or "core").
    pub source: String,
    /// Files found within this governance area.
    pub files: Vec<GovernanceFile>,
    /// Whether this area has at least one file present.
    pub covered: bool,
}

/// A single governance file found on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceFile {
    /// Relative path to the file from the project root.
    pub path: String,
    /// Size of the file in bytes.
    pub size_bytes: u64,
    /// First few hundred characters of content for display.
    pub content_preview: String,
}
