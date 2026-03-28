// Tool execution context trait for the orqa-engine crate.
//
// Defines the abstract interface for tool execution environment access.
// The app implements this with Tauri-specific state access; the daemon or CLI
// can implement it directly without any Tauri dependency.

use std::path::Path;

/// Abstraction for a tool execution environment.
///
/// Provides the minimal surface needed to run file and shell tools. Each
/// access layer (Tauri app, daemon, CLI) provides its own implementation.
/// Engine code depends on this trait rather than on any concrete state type,
/// keeping the engine free of Tauri and process-management concerns.
pub trait ToolExecutionContext: Send + Sync {
    /// Execute a shell command and return its combined stdout/stderr output.
    ///
    /// The implementation should run the command with `cwd` as the working
    /// directory and capture stdout and stderr. Returns the combined output
    /// string on success, or an error if the command could not be spawned.
    fn execute_shell(
        &self,
        command: &str,
        cwd: &Path,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    /// Read the full contents of a file at `path`.
    ///
    /// Returns the file contents as a UTF-8 string on success, or an error
    /// if the file does not exist or cannot be read.
    fn read_file(
        &self,
        path: &Path,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    /// Write `content` to the file at `path`, creating parent directories as needed.
    ///
    /// Overwrites any existing content at `path`. Returns an error if the
    /// write fails.
    fn write_file(
        &self,
        path: &Path,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Return `true` if a file or directory exists at `path`.
    fn path_exists(&self, path: &Path) -> bool;
}
