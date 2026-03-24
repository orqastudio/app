//! CLI Tool Runner — generic one-shot process executor for plugin-registered CLI tools.
//!
//! Discovers registered CLI tools from `plugin-cli-tools.json` (written by the frontend
//! when plugins with `provides.cliTools` are loaded), spawns them as child
//! processes, and captures structured JSON output.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A CLI tool registration loaded from plugin-cli-tools.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredCliTool {
    /// Plugin that provides this tool.
    pub plugin: String,
    /// Unique tool key within the plugin.
    pub key: String,
    /// Display label.
    pub label: String,
    /// Lucide icon name.
    pub icon: String,
    /// Runtime: "node" or "system".
    pub runtime: String,
    /// Entrypoint path (resolved relative to project root).
    pub entrypoint: String,
    /// Additional CLI arguments.
    #[serde(default)]
    pub args: Vec<String>,
    /// Tool category.
    pub category: String,
}

/// Result of a CLI tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliToolResult {
    /// Plugin that owns the tool.
    pub plugin: String,
    /// Tool key.
    pub tool_key: String,
    /// Exit code from the process (0 = success).
    pub exit_code: i32,
    /// Captured stdout (typically JSON when tool uses --json).
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
    /// Execution duration in milliseconds.
    pub duration_ms: u64,
    /// Unix timestamp (seconds) when the run completed.
    pub completed_at: u64,
}

/// Snapshot of the last run result for each CLI tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliToolStatus {
    /// Tool key.
    pub tool_key: String,
    /// Plugin that owns the tool.
    pub plugin: String,
    /// Display label.
    pub label: String,
    /// Whether the last run succeeded (exit_code == 0).
    pub success: Option<bool>,
    /// When the last run completed (Unix timestamp seconds).
    pub last_run: Option<u64>,
    /// Duration of the last run in milliseconds.
    pub last_duration_ms: Option<u64>,
    /// Error/warning counts from the last run (tool-specific).
    pub summary: Option<String>,
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// Config file written by the frontend/SDK when CLI tool plugins are registered.
const CLI_TOOLS_CONFIG_FILENAME: &str = "plugin-cli-tools.json";

/// Legacy config filename for backwards compatibility.
const LEGACY_TOOLS_CONFIG_FILENAME: &str = "plugin-tools.json";

/// Read the CLI tool registry from the config file at the project root.
fn read_cli_tool_registry(project_root: &Path) -> Vec<RegisteredCliTool> {
    // Try new filename first, fall back to legacy
    let config_path = project_root.join(CLI_TOOLS_CONFIG_FILENAME);
    let config_path = if config_path.exists() {
        config_path
    } else {
        let legacy = project_root.join(LEGACY_TOOLS_CONFIG_FILENAME);
        if !legacy.exists() {
            return vec![];
        }
        legacy
    };

    match std::fs::read_to_string(&config_path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => vec![],
    }
}

// ---------------------------------------------------------------------------
// CLI Tool Runner
// ---------------------------------------------------------------------------

/// Manages one-shot CLI tool execution and result caching.
pub struct CliToolRunner {
    /// Cached last-run results, keyed by `"plugin:tool_key"`.
    last_results: Mutex<HashMap<String, CliToolResult>>,
}

impl CliToolRunner {
    pub fn new() -> Self {
        Self {
            last_results: Mutex::new(HashMap::new()),
        }
    }

    /// Get all registered CLI tools from the config file.
    pub fn registered_cli_tools(&self, project_root: &Path) -> Vec<RegisteredCliTool> {
        read_cli_tool_registry(project_root)
    }

    /// Run a specific CLI tool and capture its output.
    ///
    /// The tool is spawned as a child process with the configured runtime,
    /// entrypoint, and args. Stdout and stderr are captured. The result
    /// is cached for status queries.
    pub fn run(
        &self,
        tool: &RegisteredCliTool,
        project_root: &Path,
    ) -> Result<CliToolResult, String> {
        let (program, mut args) = match tool.runtime.as_str() {
            "node" => ("node".to_string(), vec![tool.entrypoint.clone()]),
            "system" => (tool.entrypoint.clone(), vec![]),
            other => return Err(format!("unsupported runtime: {other}")),
        };

        args.extend(tool.args.iter().cloned());

        // For the integrity tool, append the project root as the target
        if tool.category == "integrity" {
            args.push(project_root.to_string_lossy().to_string());
        }

        let start = Instant::now();

        let output = Command::new(&program)
            .args(&args)
            .current_dir(project_root)
            .output()
            .map_err(|e| format!("failed to spawn {program}: {e}"))?;

        let duration = start.elapsed();

        let completed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let result = CliToolResult {
            plugin: tool.plugin.clone(),
            tool_key: tool.key.clone(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms: duration.as_millis() as u64,
            completed_at,
        };

        // Cache the result
        let cache_key = format!("{}:{}", tool.plugin, tool.key);
        if let Ok(mut cache) = self.last_results.lock() {
            cache.insert(cache_key, result.clone());
        }

        Ok(result)
    }

    /// Get the status of all registered CLI tools (last run info).
    pub fn statuses(&self, project_root: &Path) -> Vec<CliToolStatus> {
        let tools = read_cli_tool_registry(project_root);
        let cache = self
            .last_results
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        tools
            .iter()
            .map(|tool| {
                let cache_key = format!("{}:{}", tool.plugin, tool.key);
                let last = cache.get(&cache_key);

                CliToolStatus {
                    tool_key: tool.key.clone(),
                    plugin: tool.plugin.clone(),
                    label: tool.label.clone(),
                    success: last.map(|r| r.exit_code == 0),
                    last_run: last.map(|r| r.completed_at),
                    last_duration_ms: last.map(|r| r.duration_ms),
                    summary: last.map(|r| {
                        if r.exit_code == 0 {
                            "Passed".to_string()
                        } else {
                            format!("Failed (exit {})", r.exit_code)
                        }
                    }),
                }
            })
            .collect()
    }
}

impl Default for CliToolRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn new_runner_has_empty_cache() {
        let runner = CliToolRunner::new();
        let statuses = runner.statuses(&PathBuf::from("/nonexistent"));
        assert!(statuses.is_empty());
    }

    #[test]
    fn registered_cli_tools_returns_empty_for_missing_config() {
        let runner = CliToolRunner::new();
        let tools = runner.registered_cli_tools(&PathBuf::from("/nonexistent"));
        assert!(tools.is_empty());
    }

    #[test]
    fn registered_cli_tool_deserialization() {
        let json = r#"[{
            "plugin": "@orqastudio/plugin-integrity",
            "key": "integrity-check",
            "label": "Integrity Check",
            "icon": "shield-check",
            "runtime": "node",
            "entrypoint": "node_modules/.bin/orqa-integrity",
            "args": ["--json"],
            "category": "integrity"
        }]"#;
        let tools: Vec<RegisteredCliTool> = serde_json::from_str(json).unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].key, "integrity-check");
        assert_eq!(tools[0].runtime, "node");
        assert_eq!(tools[0].args, vec!["--json"]);
    }

    #[test]
    fn cli_tool_result_serialization() {
        let result = CliToolResult {
            plugin: "@orqastudio/plugin-integrity".to_string(),
            tool_key: "integrity-check".to_string(),
            exit_code: 0,
            stdout: r#"{"errors":0,"warnings":0}"#.to_string(),
            stderr: String::new(),
            duration_ms: 1234,
            completed_at: 1710000000,
        };
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["exit_code"], 0);
        assert_eq!(json["duration_ms"], 1234);
    }

    #[test]
    fn run_returns_error_for_unsupported_runtime() {
        let runner = CliToolRunner::new();
        let tool = RegisteredCliTool {
            plugin: "test".to_string(),
            key: "test-tool".to_string(),
            label: "Test".to_string(),
            icon: "wrench".to_string(),
            runtime: "python".to_string(),
            entrypoint: "test.py".to_string(),
            args: vec![],
            category: "custom".to_string(),
        };
        let result = runner.run(&tool, &PathBuf::from("/tmp"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unsupported runtime"));
    }
}
