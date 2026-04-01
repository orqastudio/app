// Daemon configuration loaded from orqa.toml at project root.
//
// Provides tunable constants that were previously hard-coded in knowledge.rs
// and parse.rs. When orqa.toml is absent or the [daemon] section is missing,
// all fields fall back to their compiled-in defaults so the daemon starts
// without any configuration file.

use serde::Deserialize;
use std::path::Path;
use tracing::{info, warn};

/// Runtime configuration for the daemon.
///
/// Loaded from the `[daemon]` section of `orqa.toml` at the project root.
/// All fields are optional in the TOML file; missing fields use the values
/// in `Default::default()`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DaemonConfig {
    /// Minimum cosine similarity score for a semantic result to be included.
    /// Matches MIN_SCORE in the TypeScript connector knowledge-injector.ts.
    pub min_score: f32,
    /// Maximum number of semantic search results to return.
    /// Matches MAX_SEMANTIC in the TypeScript connector knowledge-injector.ts.
    pub max_semantic: usize,
    /// Downstream relationship count above which artifacts get a warning.
    pub downstream_warn_threshold: u32,
}

impl Default for DaemonConfig {
    /// Returns compiled-in defaults used when no orqa.toml is present.
    fn default() -> Self {
        Self {
            min_score: 0.25,
            max_semantic: 5,
            downstream_warn_threshold: 20,
        }
    }
}

impl DaemonConfig {
    /// Load daemon configuration from orqa.toml at the given project root.
    ///
    /// Returns `Default::default()` when the file does not exist, cannot be
    /// read, or cannot be parsed — the daemon must always start regardless of
    /// configuration state.
    pub fn load(project_root: &Path) -> Self {
        let config_path = project_root.join("orqa.toml");
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                match toml::from_str::<Self>(&content) {
                    Ok(config) => {
                        info!(
                            subsystem = "config",
                            path = %config_path.display(),
                            min_score = config.min_score,
                            max_semantic = config.max_semantic,
                            downstream_warn_threshold = config.downstream_warn_threshold,
                            "[config] loaded daemon configuration from orqa.toml"
                        );
                        return config;
                    }
                    Err(e) => {
                        warn!(
                            subsystem = "config",
                            path = %config_path.display(),
                            error = %e,
                            "[config] orqa.toml exists but could not be parsed — using defaults"
                        );
                    }
                }
            }
        }
        let defaults = Self::default();
        info!(
            subsystem = "config",
            min_score = defaults.min_score,
            max_semantic = defaults.max_semantic,
            downstream_warn_threshold = defaults.downstream_warn_threshold,
            "[config] no orqa.toml found — using compiled-in defaults"
        );
        defaults
    }
}
