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
//   - `binary_path` and `started_at` are recorded at spawn time so the health
//     endpoint can report per-process detail without re-querying the OS.

use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::Instant;

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
/// `binary_path` and `started_at` are set when the subprocess is successfully
/// spawned so callers can report per-process detail for the health endpoint.
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
    /// Number of times this subprocess has crashed since it was created.
    /// Incremented each time `check_status` observes a non-zero exit code.
    pub crash_count: u32,
    /// Resolved binary path recorded at spawn time. `None` until first successful start.
    pub binary_path: Option<PathBuf>,
    /// Instant the subprocess was most recently started. `None` until first successful start.
    started_at: Option<Instant>,
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
            crash_count: 0,
            binary_path: None,
            started_at: None,
        }
    }

    /// Return the PID of the running child, if any.
    pub fn pid(&self) -> Option<u32> {
        self.child.as_ref().map(Child::id)
    }

    /// Return uptime in seconds since the most recent successful start.
    /// Returns `None` if the subprocess has never started or is not running.
    pub fn uptime_seconds(&self) -> Option<u64> {
        if self.status == SubprocessStatus::Running {
            self.started_at.map(|t| t.elapsed().as_secs())
        } else {
            None
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

        let mut cmd = Command::new(&binary_path);
        cmd.arg(project_root)
            .args(&self.args)
            // Stdin is null — subprocesses do not expect interactive input from
            // the daemon. Stdout is discarded; stderr is captured so subprocess
            // logs are available without opening a visible console window.
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped());

        // On Windows, CREATE_NO_WINDOW prevents the child process from opening
        // a visible console window. Without this flag, every LSP/MCP subprocess
        // pops up a black cmd.exe window on the desktop.
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let child = cmd.spawn()?;

        info!(
            name = %self.name,
            pid = child.id(),
            "subprocess started"
        );

        self.binary_path = Some(binary_path);
        self.started_at = Some(Instant::now());
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
                        self.crash_count += 1;
                        error!(
                            name = %self.name,
                            pid,
                            exit_status = ?exit_status,
                            crash_count = self.crash_count,
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

/// A point-in-time snapshot of one managed subprocess, used by the health
/// endpoint to report per-process detail without holding the subprocess lock.
///
/// Constructed by `SubprocessManager::snapshot` and serialised into the
/// health response `processes` array. The daemon itself is added separately by
/// the health handler because it is not a managed subprocess.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessSnapshot {
    /// Human-readable name shown in OrqaDev (e.g., "LSP Server").
    pub name: String,
    /// Canonical log source identifier for log filtering (e.g., "lsp").
    pub source: String,
    /// Lifecycle status string: "running", "stopped", "crashed", or "not_found".
    pub status: String,
    /// OS process ID, populated only when the process is running.
    pub pid: Option<u32>,
    /// Uptime in seconds since last start. `None` when not running.
    pub uptime_seconds: Option<u64>,
    /// Absolute path to the binary that was spawned. `None` when never started.
    pub binary_path: Option<String>,
}

impl SubprocessManager {
    /// Build a `ProcessSnapshot` from the current manager state.
    ///
    /// Used by the health endpoint to expose per-process detail. The `source`
    /// parameter is the canonical log source key (e.g., "lsp", "mcp") since
    /// the manager's `name` field is an internal label, not a log source key.
    pub fn snapshot(&self, source: &str) -> ProcessSnapshot {
        ProcessSnapshot {
            name: self.name.clone(),
            source: source.to_owned(),
            status: match self.status {
                SubprocessStatus::Running => "running".to_owned(),
                SubprocessStatus::Stopped => "stopped".to_owned(),
                SubprocessStatus::Crashed => "crashed".to_owned(),
                SubprocessStatus::BinaryNotFound => "not_found".to_owned(),
            },
            pid: self.pid(),
            uptime_seconds: self.uptime_seconds(),
            binary_path: self.binary_path.as_ref().map(|p| p.display().to_string()),
        }
    }
}

/// Return `true` if `binary_name` can be found on PATH.
///
/// Spawns the binary with `--help`, immediately kills it, and returns whether
/// the spawn succeeded. This is the most portable cross-platform approach to
/// PATH resolution without pulling in a `which` crate.
///
/// On Windows, CREATE_NO_WINDOW is set so the probe spawn does not flash a
/// console window.
fn which_binary(binary_name: &str) -> bool {
    let mut cmd = Command::new(binary_name);
    cmd.arg("--help")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.spawn()
        .map(|mut child| {
            let _ = child.kill();
            let _ = child.wait();
            true
        })
        .unwrap_or(false)
}
