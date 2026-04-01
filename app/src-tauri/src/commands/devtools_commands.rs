//! Tauri IPC commands for launching and querying the OrqaDev companion app.
//!
//! OrqaDev (`orqa-devtools`) is a separate Tauri window used by developers to
//! inspect daemon logs, artifact state, and plugin behaviour. These commands
//! allow the main app toolbar to launch OrqaDev on demand and query whether it
//! is already running.

use std::process::Command;

use crate::error::OrqaError;

/// Name of the OrqaDev binary (without extension).
const DEVTOOLS_BIN: &str = "orqa-devtools";

/// Resolve the path to the `orqa-devtools` binary.
///
/// Checks `target/debug` and `target/release` relative to the workspace root
/// (two levels up from the `app/` directory). Falls back to the bare binary
/// name so that PATH is searched when no pre-built binary is found.
fn find_devtools_bin() -> String {
    // The Tauri process CWD is `app/` during `cargo tauri dev`.
    // Walk up two levels to reach the workspace root.
    let ext = if cfg!(windows) { ".exe" } else { "" };
    let bin_name = format!("{DEVTOOLS_BIN}{ext}");

    let workspace_root = std::path::Path::new("../..").to_path_buf();
    let candidates = [
        workspace_root.join("target").join("debug").join(&bin_name),
        workspace_root.join("target").join("release").join(&bin_name),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return candidate.to_string_lossy().into_owned();
        }
    }

    // Fall back to bare name — relies on PATH.
    bin_name
}

/// Launch the OrqaDev companion app.
///
/// Spawns `orqa-devtools` as a detached process so it survives independently
/// of the main OrqaStudio process. If OrqaDev is already running this command
/// is a no-op from the process perspective — the OS will open a second window,
/// which is acceptable developer behaviour.
///
/// Returns `Ok(())` on successful spawn, or an error if the binary is not found
/// or cannot be executed.
#[tauri::command]
pub fn launch_devtools() -> Result<(), OrqaError> {
    let bin = find_devtools_bin();

    tracing::info!(bin = %bin, "launching OrqaDev");

    let mut cmd = Command::new(&bin);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP — no console window,
        // process survives after the parent exits.
        cmd.creation_flags(0x00000008 | 0x00000200);
    }

    cmd.spawn().map_err(|e| {
        OrqaError::Sidecar(format!(
            "failed to launch orqa-devtools ({bin}): {e}. \
             Build it with `cargo build -p orqa-devtools` first."
        ))
    })?;

    Ok(())
}

/// Check whether an `orqa-devtools` process is currently running.
///
/// Uses platform-specific process listing to detect a running OrqaDev instance.
/// Returns `true` if at least one `orqa-devtools` process is found.
#[tauri::command]
pub fn is_devtools_running() -> bool {
    #[cfg(windows)]
    {
        let output = Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-Command",
                "Get-Process -Name 'orqa-devtools' -ErrorAction SilentlyContinue | Measure-Object | Select-Object -ExpandProperty Count",
            ])
            .output();

        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);
            return s.trim().parse::<u32>().unwrap_or(0) > 0;
        }
        false
    }

    #[cfg(not(windows))]
    {
        let output = Command::new("pgrep")
            .args(["-f", DEVTOOLS_BIN])
            .output();

        if let Ok(out) = output {
            return !out.stdout.is_empty();
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_devtools_bin_returns_string() {
        // Should always return a non-empty string (even if binary does not exist).
        let bin = find_devtools_bin();
        assert!(!bin.is_empty());
        assert!(bin.contains("orqa-devtools"));
    }
}
