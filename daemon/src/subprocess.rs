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
//   - Crashed subprocesses are automatically restarted with exponential backoff
//     (2s, 4s, 8s, 16s, 30s) up to max_restarts (5) consecutive crashes.
//   - crash_count resets to zero after the subprocess runs stably for 60s,
//     so transient failures do not permanently exhaust the restart budget.
//   - `check_status` polls the child's exit code rather than sending a signal
//     so the check is non-destructive on all platforms.
//   - `binary_path` and `started_at` are recorded at spawn time so the health
//     endpoint can report per-process detail without re-querying the OS.

use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, Command};
use std::time::{Duration, Instant};

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
///
/// Auto-restart: when a crash is detected the manager schedules a restart with
/// exponential backoff. The event loop calls `poll_restart` on each tick to
/// trigger the restart once the backoff delay has elapsed. After `max_restarts`
/// consecutive crashes the manager stops retrying and logs an error. The crash
/// count resets to zero when the subprocess runs stably for 60 seconds.
pub struct SubprocessManager {
    /// Human-readable name for logging (e.g., "mcp-server").
    pub name: String,
    /// Binary to execute (looked up in PATH or as an absolute path).
    pub binary: String,
    /// Arguments to pass to the binary on startup.
    pub args: Vec<String>,
    /// Additional environment variables to set for the child process. These
    /// are merged on top of the daemon's inherited environment. Callers use
    /// `set_env` to add trace context (e.g., `ORQA_TRACE_ID`) before spawning.
    pub env_vars: Vec<(String, String)>,
    /// Handle to the running child process, if any.
    child: Option<Child>,
    /// Last known status.
    status: SubprocessStatus,
    /// Number of consecutive crashes. Incremented on each crash, reset to zero
    /// after 60 seconds of stable running.
    pub crash_count: u32,
    /// Maximum consecutive crashes before giving up on auto-restart.
    max_restarts: u32,
    /// Project root passed to the most recent `start()` call, stored so that
    /// `restart()` can re-spawn without the caller needing to re-supply it.
    project_root: Option<PathBuf>,
    /// When `Some`, the event loop should trigger a restart at this instant.
    /// Set by `schedule_restart()`, cleared by `poll_restart()`.
    next_restart_at: Option<Instant>,
    /// Resolved binary path recorded at spawn time. `None` until first successful start.
    pub binary_path: Option<PathBuf>,
    /// Instant the subprocess was most recently started. `None` until first successful start.
    started_at: Option<Instant>,
    /// Captured stderr from the child process for crash diagnostics.
    stderr_output: Option<ChildStderr>,
}

impl SubprocessManager {
    /// Create a new manager for `binary` with `args`. Does not start the
    /// process — call `start` to spawn it.
    pub fn new(name: impl Into<String>, binary: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            binary: binary.into(),
            args,
            env_vars: Vec::new(),
            child: None,
            status: SubprocessStatus::Stopped,
            crash_count: 0,
            max_restarts: 5,
            project_root: None,
            next_restart_at: None,
            binary_path: None,
            started_at: None,
            stderr_output: None,
        }
    }

    /// Add an environment variable that will be set on the child process at
    /// spawn time. Call this before `start`. Repeated calls accumulate; later
    /// entries with the same key override earlier ones.
    pub fn set_env(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.env_vars.push((key.into(), value.into()));
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

        let mut cmd = self.build_command(&binary_path, project_root);
        let mut child = cmd.spawn()?;
        let stderr = child.stderr.take();

        info!(name = %self.name, pid = child.id(), "subprocess started");

        self.binary_path = Some(binary_path);
        self.started_at = Some(Instant::now());
        self.stderr_output = stderr;
        self.child = Some(child);
        self.status = SubprocessStatus::Running;
        self.project_root = Some(project_root.to_path_buf());
        Ok(())
    }

    /// Build the `Command` for spawning this subprocess.
    fn build_command(&self, binary_path: &Path, project_root: &Path) -> Command {
        let mut cmd = Command::new(binary_path);
        cmd.arg(project_root)
            .args(&self.args)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped());

        for (key, val) in &self.env_vars {
            cmd.env(key, val);
        }

        // On Windows, CREATE_NO_WINDOW prevents the child process from opening
        // a visible console window.
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        cmd
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
        self.stderr_output = None;
        self.status = SubprocessStatus::Stopped;
    }

    /// Poll the child's exit status and update `self.status` accordingly.
    ///
    /// When the subprocess has been running stably for 60 seconds, resets
    /// `crash_count` to zero so transient past failures do not exhaust the
    /// restart budget. When a crash is detected, schedules an auto-restart
    /// with exponential backoff unless the restart budget is exhausted.
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
                        let stderr_text = self.read_stderr();
                        error!(
                            name = %self.name,
                            pid,
                            exit_status = ?exit_status,
                            crash_count = self.crash_count,
                            stderr = %stderr_text,
                            "subprocess crashed"
                        );
                        self.status = SubprocessStatus::Crashed;
                        self.schedule_restart();
                    }
                    self.child = None;
                }
                Ok(None) => {
                    // Still running — reset crash count if uptime exceeds 60s.
                    if let Some(started) = self.started_at {
                        if started.elapsed() >= Duration::from_secs(60) && self.crash_count > 0 {
                            info!(
                                name = %self.name,
                                "subprocess stable for 60s — resetting crash count"
                            );
                            self.crash_count = 0;
                        }
                    }
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

    /// Compute the backoff delay in milliseconds for the current crash count.
    ///
    /// Schedule: 2s → 4s → 8s → 16s → 30s (capped). Uses crash_count before
    /// incrementing so crash_count=1 yields the first backoff delay of 2000ms.
    fn backoff_ms(&self) -> u64 {
        // crash_count is already incremented by the time this is called.
        let exponent = self.crash_count.saturating_sub(1);
        let delay = 2000u64.saturating_mul(1u64 << exponent.min(4));
        delay.min(30_000)
    }

    /// Return `true` if auto-restart should be attempted.
    ///
    /// Returns `false` once crash_count exceeds max_restarts or if there is no
    /// stored project_root to restart with.
    fn should_restart(&self) -> bool {
        self.crash_count <= self.max_restarts && self.project_root.is_some()
    }

    /// Schedule a restart after the appropriate backoff delay.
    ///
    /// Does nothing if `should_restart()` is false. Logs whether a restart is
    /// being scheduled or whether the retry budget is exhausted.
    fn schedule_restart(&mut self) {
        if !self.should_restart() {
            error!(
                name = %self.name,
                crash_count = self.crash_count,
                max_restarts = self.max_restarts,
                "subprocess exceeded max restarts — giving up"
            );
            return;
        }
        let delay = self.backoff_ms();
        let restart_at = Instant::now() + Duration::from_millis(delay);
        self.next_restart_at = Some(restart_at);
        warn!(
            name = %self.name,
            crash_count = self.crash_count,
            backoff_ms = delay,
            "scheduling auto-restart after backoff"
        );
    }

    /// Restart the subprocess: stop any remnant, then start again.
    ///
    /// Uses the project_root stored from the previous `start()` call. Logs an
    /// error and returns early if no project_root is available.
    pub fn restart(&mut self) {
        let Some(root) = self.project_root.clone() else {
            error!(name = %self.name, "cannot restart — no project_root stored");
            return;
        };
        info!(name = %self.name, crash_count = self.crash_count, "restarting subprocess");
        self.stop();
        if let Err(e) = self.start(&root) {
            error!(name = %self.name, error = %e, "restart failed — subprocess will not run");
        }
    }

    /// Check whether a scheduled restart is due and execute it if so.
    ///
    /// Called by the event loop on every polling tick. Returns `true` if a
    /// restart was triggered this tick, `false` otherwise.
    pub fn poll_restart(&mut self) -> bool {
        let Some(restart_at) = self.next_restart_at else {
            return false;
        };
        if Instant::now() < restart_at {
            return false;
        }
        self.next_restart_at = None;
        self.restart();
        true
    }

    /// Read remaining stderr bytes from the child process, up to 4 KB.
    /// Returns an empty string if no stderr is available.
    fn read_stderr(&mut self) -> String {
        let Some(stderr) = self.stderr_output.take() else {
            return String::new();
        };
        use std::io::Read;
        let mut buf = vec![0u8; 4096];
        let mut reader = std::io::BufReader::new(stderr);
        match reader.read(&mut buf) {
            Ok(n) => String::from_utf8_lossy(&buf[..n]).to_string(),
            Err(_) => String::new(),
        }
        // Note: don't store it back — we only need it once per crash
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
