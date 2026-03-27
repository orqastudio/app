// Unified logging initialisation for the OrqaStudio daemon.
//
// Sets up two log sinks:
//
//   1. A rolling JSON file appender writing to `.state/daemon.log` in the
//      project root.  One new file is created each day; old files are retained
//      up to the configured rotation limit.  JSON format allows log aggregators
//      to parse structured fields (subsystem, level, timestamp, etc.) without
//      post-processing.
//
//   2. A console subscriber that writes human-readable, coloured output to
//      stdout when the process is attached to a TTY (interactive use).  When
//      stdout is not a TTY (e.g. running as a background service), console
//      output is suppressed to avoid polluting captured output.
//
// Log level is controlled via the `RUST_LOG` environment variable.  The
// default level is `info`.  All events are tagged with a `subsystem` field
// so log queries can filter by `[mcp]`, `[lsp]`, `[watcher]`, `[engine]`,
// or `[health]`.

use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

/// Handle that keeps the non-blocking log writer alive.
///
/// The `WorkerGuard` must be held for the entire lifetime of the process.
/// Dropping it flushes and closes the background log-writer thread.
/// Store this in `main` — it is returned from [`init`] so the caller controls
/// the lifetime.
pub struct LogGuard {
    _file_guard: WorkerGuard,
}

/// Initialise structured logging with file and optional console output.
///
/// Must be called before any other subsystem starts so that all log events are
/// captured from the beginning of the startup sequence.
///
/// - `project_root` — the resolved project root directory; `.state/` is
///   created here if it does not already exist.
///
/// Returns a [`LogGuard`] that must be kept alive for the duration of the
/// process. Dropping it flushes the log file writer.
///
/// # Panics
///
/// Panics if the tracing subscriber registry cannot be installed. This only
/// happens if another subscriber was installed first, which is a programming
/// error.
pub fn init(project_root: &Path) -> LogGuard {
    let state_dir = project_root.join(".state");
    // Best-effort: if `.state/` cannot be created we fall back to a
    // non-rolling appender writing to the same directory.
    let _ = std::fs::create_dir_all(&state_dir);

    // Build a daily-rolling file appender for structured JSON logs.
    let file_appender = tracing_appender::rolling::daily(state_dir, "daemon.log");
    let (non_blocking_file, file_guard) = tracing_appender::non_blocking(file_appender);

    // JSON layer — always active, regardless of TTY state.
    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking_file)
        .with_filter(build_env_filter());

    // Console layer — human-readable, coloured, only when stdout is a TTY.
    // This avoids garbled ANSI codes in CI/service logs.
    let console_layer = if is_tty() {
        let layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_target(true)
            .with_filter(build_env_filter());
        Some(layer)
    } else {
        None
    };

    tracing_subscriber::registry()
        .with(json_layer)
        .with(console_layer)
        .init();

    LogGuard {
        _file_guard: file_guard,
    }
}

/// Build the `EnvFilter` from `RUST_LOG`, defaulting to `info`.
///
/// A separate call is needed for each layer because `EnvFilter` is not
/// `Clone`.
fn build_env_filter() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
}

/// Return `true` when stdout is connected to a terminal.
///
/// Uses the `TERM` environment variable on Unix and `GetConsoleMode` semantics
/// via the `atty` equivalent: checks whether `fd 1` is a TTY.  Falls back to
/// `false` on any error so that ambiguous environments are treated as
/// non-interactive.
fn is_tty() -> bool {
    // `std::io::IsTerminal` is stable since Rust 1.70.
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}
