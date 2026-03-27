// Subprocess lifecycle manager for the OrqaStudio daemon.
//
// Provides a generic `SubprocessManager` that the daemon uses to start,
// monitor, and stop child processes such as `orqa-mcp-server` and
// `orqa-lsp-server`. Each managed subprocess has a name, a binary path,
// a set of arguments, and an optional child handle.
//
// Design decisions:
//   - Binary presence is checked before spawning so the daemon degrades
//     gracefully when the server binaries are not yet built.
//   - The daemon does NOT restart subprocesses on crash (that is a future
//     phase enhancement). It logs the exit and marks the process as stopped.
//   - `check_status` polls the child's exit code rather than sending a signal
//     so the check is non-destructive on all platforms.

use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use tracing::{error, info, warn};

/// Status of a managed subprocess, used for system tray display and health
/// endpoint reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubprocessStatus {
    /// The binary was not found at startup — subprocess will not be started.
    BinaryNotFound,
    /// The subprocess has not been started yet (or was stopped).
    Stopped,
    /// The subprocess is currently running.
    Running,
    /// The subprocess exited unexpectedly (non-zero or killed).
    Crashed,
}

/// Manages the lifecycle of a single child process.
///
/// Holds the binary name, arguments, and (when running) a handle to the child.
/// Callers start and stop the subprocess through `start` and `stop`. The
/// `check_status` method polls the child's exit code to detect crashes.
pub struct SubprocessManager {
    /// Human-readable name for logging (e.g., "mcp-server").
    pub name: String,
    /// Binary to execute (looked up in PATH or as an absolute path).
    pub binary: String,
    /// Arguments to pass to the binary on startup.
    pub args: Vec<String>,
    /// Handle to the running child process, if any.
    child: Option<Child>,
    /// Last known status.
    status: SubprocessStatus,
}

impl SubprocessManager {
    /// Create a new manager for `binary` with `args`. Does not start the
    /// process — call `start` to spawn it.
    pub fn new(name: impl Into<String>, binary: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            binary: binary.into(),
            args,
            child: None,
            status: SubprocessStatus::Stopped,
        }
    }

    /// Return the last known status of the subprocess.
    pub fn status(&self) -> SubprocessStatus {
        self.status
    }

    /// Attempt to find a binary by name.
    ///
    /// Search order:
    /// 1. Next to the currently-running daemon executable (sibling in the same
    ///    directory — covers `cargo run` and installed deployments where all
    ///    OrqaStudio binaries live together).
    /// 2. On PATH via a probe-and-kill spawn.
    ///
    /// Returns the resolved path if found, or `None` if unavailable. When
    /// `None` is returned the daemon skips spawning and logs a warning, so the
    /// daemon still starts successfully.
    pub fn find_binary(binary_name: &str) -> Option<PathBuf> {
        // 1. Sibling to the current executable.
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let candidate = dir.join(binary_name);
                if candidate.is_file() {
                    return Some(candidate);
                }
                // Windows: also try with .exe extension.
                #[cfg(target_os = "windows")]
                {
                    let candidate_exe = dir.join(format!("{binary_name}.exe"));
                    if candidate_exe.is_file() {
                        return Some(candidate_exe);
                    }
                }
            }
        }

        // 2. PATH lookup — probe whether the OS can resolve the binary name.
        if which_binary(binary_name) {
            return Some(PathBuf::from(binary_name));
        }

        None
    }

    /// Spawn the subprocess, passing `project_root` as the first positional
    /// argument followed by `self.args`.
    ///
    /// If the binary cannot be found, sets status to `BinaryNotFound` and
    /// returns `Ok(())` — the daemon continues running without the subprocess.
    /// Hard OS errors during spawn are returned as `Err`.
    pub fn start(&mut self, project_root: &Path) -> std::io::Result<()> {
        // Guard: do not double-start.
        if self.is_running() {
            warn!(name = %self.name, "subprocess already running — skipping start");
            return Ok(());
        }

        // Resolve binary.
        let Some(binary_path) = Self::find_binary(&self.binary) else {
            warn!(
                name = %self.name,
                binary = %self.binary,
                "binary not found — {} will not be started. \
                Build it with `cargo build -p {}`.",
                self.name,
                self.binary,
            );
            self.status = SubprocessStatus::BinaryNotFound;
            return Ok(());
        };

        info!(
            name = %self.name,
            binary = %binary_path.display(),
            project_root = %project_root.display(),
            args = ?self.args,
            "spawning subprocess"
        );

        let child = Command::new(&binary_path)
            .arg(project_root)
            .args(&self.args)
            // Stdin is null — subprocesses do not expect interactive input from
            // the daemon. Stdout is discarded; stderr is inherited so subprocess
            // logs appear in the daemon's stderr stream.
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        info!(
            name = %self.name,
            pid = child.id(),
            "subprocess started"
        );

        self.child = Some(child);
        self.status = SubprocessStatus::Running;
        Ok(())
    }

    /// Stop the subprocess by sending SIGKILL (or TerminateProcess on Windows).
    ///
    /// No-op if the subprocess is not running. Logs a warning if the kill call
    /// fails (the process may have already exited before the signal was sent).
    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            info!(name = %self.name, pid = child.id(), "stopping subprocess");
            child.kill().unwrap_or_else(|e| {
                // Process may have already exited — not a hard error.
                warn!(
                    name = %self.name,
                    error = %e,
                    "kill returned error (process may have already exited)"
                );
            });
            // Reap the exit status to avoid a zombie process.
            let _ = child.wait();
            info!(name = %self.name, "subprocess stopped");
        }
        self.status = SubprocessStatus::Stopped;
    }

    /// Poll the child's exit status and update `self.status` accordingly.
    ///
    /// Returns the current status after polling. Call this periodically from
    /// the event loop to detect crashes without blocking.
    pub fn check_status(&mut self) -> SubprocessStatus {
        if let Some(child) = &mut self.child {
            match child.try_wait() {
                Ok(Some(exit_status)) => {
                    let pid = child.id();
                    if exit_status.success() {
                        info!(name = %self.name, pid, "subprocess exited cleanly");
                        self.status = SubprocessStatus::Stopped;
                    } else {
                        error!(
                            name = %self.name,
                            pid,
                            exit_status = ?exit_status,
                            "subprocess crashed"
                        );
                        self.status = SubprocessStatus::Crashed;
                    }
                    self.child = None;
                }
                Ok(None) => {
                    // Still running — no state change.
                    self.status = SubprocessStatus::Running;
                }
                Err(e) => {
                    error!(name = %self.name, error = %e, "could not poll subprocess status");
                }
            }
        }
        self.status
    }

    /// Return `true` if the subprocess is currently running.
    pub fn is_running(&self) -> bool {
        self.status == SubprocessStatus::Running
    }
}

/// Return `true` if `binary_name` can be found on PATH.
///
/// Spawns the binary with `--help`, immediately kills it, and returns whether
/// the spawn succeeded. This is the most portable cross-platform approach to
/// PATH resolution without pulling in a `which` crate.
fn which_binary(binary_name: &str) -> bool {
    Command::new(binary_name)
        .arg("--help")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map(|mut child| {
            let _ = child.kill();
            let _ = child.wait();
            true
        })
        .unwrap_or(false)
}
