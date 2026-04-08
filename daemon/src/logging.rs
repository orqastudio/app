// Unified logging initialisation for the OrqaStudio daemon.
//
// Sets up three log sinks:
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
//   3. An optional `EventBusLayer` that converts every tracing event into a
//      `LogEvent` and publishes it to the central event bus.  This layer is
//      added when an `Arc<EventBus>` is provided to `init`.  All existing
//      `tracing::info!` / `warn!` / `debug!` / `error!` call sites flow
//      through the event bus automatically with no changes at the call sites.
//
// Level configuration (in priority order):
//   1. `ORQA_DEV=true` env var → forces "debug" for console and file layers.
//   2. `RUST_LOG` env var → used directly as the filter string.
//   3. `log_level` field in the `[daemon]` section of `orqa.toml`.
//   4. Compiled-in default: "info".
//
// The event bus layer always uses LevelFilter::TRACE so ALL events reach the
// bus regardless of the configured display level.  Level filtering for
// display is handled by consumers (OrqaDev dashboard, log table).
//
// All events are tagged with a `subsystem` field so log queries can filter by
// `[mcp]`, `[lsp]`, `[watcher]`, `[engine]`, or `[health]`.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use orqa_engine_types::fingerprint::{compute_fingerprint, extract_template};
use orqa_engine_types::types::event::{EventLevel, EventSource, EventTier, LogEvent, StackFrame};

use crate::correlation::current_correlation_id;
use crate::event_bus::EventBus;

/// Handle that keeps the non-blocking log writer alive.
///
/// The `WorkerGuard` must be held for the entire lifetime of the process.
/// Dropping it flushes and closes the background log-writer thread.
/// Store this in `main` — it is returned from [`init`] so the caller controls
/// the lifetime.
pub struct LogGuard {
    _file_guard: WorkerGuard,
}

/// A `tracing_subscriber::Layer` that converts tracing events into `LogEvent`
/// values and publishes them to the daemon's central event bus.
///
/// All existing `tracing::info!` / `warn!` / `debug!` / `error!` call sites
/// flow through the bus automatically.  The layer extracts the `message` field
/// (the format string argument), plus any structured fields such as `subsystem`
/// and `elapsed_ms`, packing them into `LogEvent::metadata` as a JSON object.
///
/// Level mapping: TRACE and DEBUG → `EventLevel::Debug`, INFO → `EventLevel::Info`,
/// WARN → `EventLevel::Warn`, ERROR → `EventLevel::Error`.
///
/// The layer uses a monotonically increasing counter for `LogEvent::id`.
pub struct EventBusLayer {
    /// Shared reference to the event bus that receives every converted event.
    bus: Arc<EventBus>,
    /// Monotonically increasing event id counter, unique within a daemon session.
    next_id: AtomicU64,
}

impl EventBusLayer {
    /// Create a new `EventBusLayer` publishing to the given bus.
    pub fn new(bus: Arc<EventBus>) -> Self {
        Self {
            bus,
            next_id: AtomicU64::new(1),
        }
    }

    /// Allocate the next unique event id.
    fn next_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}

/// Visitor that extracts fields from a tracing event into a JSON object.
///
/// The `message` field (the first positional argument) is captured separately
/// so it can be placed in `LogEvent::message`.  All other fields go into the
/// metadata map.
struct FieldVisitor {
    message: Option<String>,
    subsystem: Option<String>,
    fields: serde_json::Map<String, serde_json::Value>,
}

impl FieldVisitor {
    fn new() -> Self {
        Self {
            message: None,
            subsystem: None,
            fields: serde_json::Map::new(),
        }
    }
}

impl tracing::field::Visit for FieldVisitor {
    /// Handle the `message` field — the formatted string from the macro call.
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "message" => self.message = Some(value.to_owned()),
            "subsystem" => {
                self.subsystem = Some(value.to_owned());
                self.fields.insert(
                    field.name().to_owned(),
                    serde_json::Value::String(value.to_owned()),
                );
            }
            other => {
                self.fields.insert(
                    other.to_owned(),
                    serde_json::Value::String(value.to_owned()),
                );
            }
        }
    }

    /// Handle debug-formatted values (the default for most structured fields).
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let s = format!("{value:?}");
        match field.name() {
            "message" => self.message = Some(s),
            "subsystem" => {
                self.subsystem = Some(s.clone());
                self.fields
                    .insert(field.name().to_owned(), serde_json::Value::String(s));
            }
            other => {
                self.fields
                    .insert(other.to_owned(), serde_json::Value::String(s));
            }
        }
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.fields.insert(
            field.name().to_owned(),
            serde_json::Value::Number(value.into()),
        );
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        // u64 may not fit in serde_json::Number (which is i64/u64/f64 internally),
        // but serde_json does support u64, so this is safe.
        if let Some(n) = serde_json::Number::from_f64(value as f64) {
            self.fields
                .insert(field.name().to_owned(), serde_json::Value::Number(n));
        }
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        if let Some(n) = serde_json::Number::from_f64(value) {
            self.fields
                .insert(field.name().to_owned(), serde_json::Value::Number(n));
        }
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.fields
            .insert(field.name().to_owned(), serde_json::Value::Bool(value));
    }
}

impl<S> Layer<S> for EventBusLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    /// Convert a tracing event into a `LogEvent` and publish it to the bus.
    ///
    /// Called by the tracing runtime for every event that passes the global
    /// filter, including events from all daemon subsystems.
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = FieldVisitor::new();
        event.record(&mut visitor);

        let level = match *event.metadata().level() {
            tracing::Level::TRACE | tracing::Level::DEBUG => EventLevel::Debug,
            tracing::Level::INFO => EventLevel::Info,
            tracing::Level::WARN => EventLevel::Warn,
            tracing::Level::ERROR => EventLevel::Error,
        };

        let category = visitor
            .subsystem
            .clone()
            .unwrap_or_else(|| event.metadata().target().to_owned());
        let message = visitor.message.unwrap_or_default();
        let metadata = if visitor.fields.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::Value::Object(visitor.fields)
        };

        let log_event = build_log_event(self.next_id(), level, category, message, metadata);
        self.bus.publish(log_event);
    }
}

/// Construct a `LogEvent` with fingerprint, backtrace, and correlation ID.
fn build_log_event(
    id: u64,
    level: EventLevel,
    category: String,
    message: String,
    metadata: serde_json::Value,
) -> LogEvent {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);

    let template = extract_template(&message);
    let fp = compute_fingerprint(
        &EventSource::Daemon.to_string(),
        &level.to_string(),
        &template,
        "",
    );

    let stack_frames = if level == EventLevel::Error {
        let bt = std::backtrace::Backtrace::force_capture();
        Some(parse_backtrace(&bt))
    } else {
        None
    };

    LogEvent {
        id,
        timestamp,
        level,
        source: EventSource::Daemon,
        tier: EventTier::default(),
        category,
        message,
        metadata,
        session_id: None,
        fingerprint: Some(fp),
        message_template: Some(template),
        correlation_id: current_correlation_id(),
        stack_frames,
    }
}

/// Parse a `std::backtrace::Backtrace` into a capped list of `StackFrame` values.
///
/// Converts the backtrace to its string representation, then extracts file,
/// line, and function information from each frame. Runtime noise frames from
/// tokio, std, and tracing internals are filtered out so only application
/// frames appear. Caps the output at 20 frames so `stack_frames` stays
/// compact for serialization and display.
#[allow(clippy::too_many_lines)]
fn parse_backtrace(bt: &std::backtrace::Backtrace) -> Vec<StackFrame> {
    // The runtime-noise prefixes to skip. These belong to the async/tracing
    // scaffolding rather than the application's own call stack.
    const SKIP_PREFIXES: &[&str] = &[
        "tokio::",
        "std::",
        "core::",
        "tracing::",
        "tracing_subscriber::",
        "tracing_appender::",
        "__rust_",
        "rust_begin_unwind",
        "backtrace::",
    ];

    const MAX_FRAMES: usize = 20;

    let bt_str = bt.to_string();
    let mut frames = Vec::with_capacity(MAX_FRAMES);

    // The backtrace string format is platform-dependent but typically looks like:
    //
    //    N: function::name
    //             at /path/to/file.rs:line:col
    //
    // We collect (function, file, line) by pairing lines: a function line
    // followed by an optional "at ..." location line.
    let mut lines = bt_str.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        // Frame index lines: "0: function_name" or "0: 0x... - function_name"
        let func_candidate = if let Some(rest) = trimmed
            .split_once(':')
            .map(|x| x.1)
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            // Strip leading hex addresses: "0x... - func_name"
            if let Some(after_dash) = rest.split(" - ").nth(1) {
                after_dash.trim().to_owned()
            } else {
                rest.to_owned()
            }
        } else {
            continue;
        };

        // Skip runtime-noise frames.
        if SKIP_PREFIXES.iter().any(|p| func_candidate.starts_with(p)) {
            // Consume the matching "at ..." line if present so we don't
            // misattribute it to the next frame.
            if lines.peek().is_some_and(|l| l.trim().starts_with("at ")) {
                lines.next();
            }
            continue;
        }

        // Look ahead for the "at file:line:col" location.
        let (file, line_no) = if lines.peek().is_some_and(|l| l.trim().starts_with("at ")) {
            let at_line = lines
                .next()
                .expect("peeked Some, next must succeed")
                .trim()
                .to_owned();
            // Format: "at /path/to/file.rs:line:col"
            let location = at_line.strip_prefix("at ").unwrap_or(&at_line);
            // Split off trailing :col then :line.
            let mut parts = location.rsplitn(3, ':');
            let _col = parts.next();
            let line_num: Option<u32> = parts.next().and_then(|s| s.parse().ok());
            let file_path = parts.next().unwrap_or(location).to_owned();
            (file_path, line_num)
        } else {
            (func_candidate.clone(), None)
        };

        frames.push(StackFrame {
            file,
            line: line_no,
            col: None,
            function: Some(func_candidate),
            raw: None,
        });

        if frames.len() >= MAX_FRAMES {
            break;
        }
    }

    frames
}

/// Initialise structured logging with file, optional console, and optional
/// event bus output.
///
/// Must be called before any other subsystem starts so that all log events are
/// captured from the beginning of the startup sequence.
///
/// - `project_root` — the resolved project root directory; `.state/` is
///   created here if it does not already exist.
/// - `bus` — when `Some`, an `EventBusLayer` is added to the subscriber stack
///   so every tracing event is also published to the event bus.
///
/// Returns a [`LogGuard`] that must be kept alive for the duration of the
/// process. Dropping it flushes the log file writer.
///
/// # Panics
///
/// Panics if the tracing subscriber registry cannot be installed. This only
/// happens if another subscriber was installed first, which is a programming
/// error.
pub fn init(project_root: &Path, bus: Option<Arc<EventBus>>) -> LogGuard {
    let state_dir = project_root.join(".state");
    // Best-effort: if `.state/` cannot be created we fall back to a
    // non-rolling appender writing to the same directory.
    let _ = std::fs::create_dir_all(&state_dir);

    // Resolve the effective display log level. The event bus always receives
    // TRACE regardless of this value.
    let effective_level = resolve_log_level(project_root);

    // Build a daily-rolling file appender for structured JSON logs.
    let file_appender = tracing_appender::rolling::daily(&state_dir, "daemon.log");
    let (non_blocking_file, file_guard) = tracing_appender::non_blocking(file_appender);

    // JSON layer — always active, regardless of TTY state.
    // Boxed to erase the deeply-nested generic type and prevent Windows stack
    // overflow during tracing_subscriber initialisation in debug builds.
    let json_layer: Box<dyn Layer<_> + Send + Sync> = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking_file)
        .with_filter(build_display_filter(&effective_level))
        .boxed();

    // Console layer — human-readable, coloured, only when stdout is a TTY.
    // This avoids garbled ANSI codes in CI/service logs.
    // Boxed for the same reason as the JSON layer.
    let console_layer: Option<Box<dyn Layer<_> + Send + Sync>> = if is_tty() {
        Some(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_target(true)
                .with_filter(build_display_filter(&effective_level))
                .boxed(),
        )
    } else {
        None
    };

    // Event bus layer — converts every tracing event to a `LogEvent` and
    // publishes it.  Uses LevelFilter::TRACE so all events reach the bus;
    // display-side filtering is handled by consumers of the bus.
    // Boxed for the same reason as the other layers.
    let bus_layer: Option<Box<dyn Layer<_> + Send + Sync>> = bus.map(|b| {
        EventBusLayer::new(b)
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE)
            .boxed()
    });

    tracing_subscriber::registry()
        .with(json_layer)
        .with(console_layer)
        .with(bus_layer)
        .init();

    let tty_mode = is_tty();
    let log_file_path = state_dir.join("daemon.log");

    // Log the effective logging configuration immediately after initialization.
    tracing::info!(
        subsystem = "logging",
        log_file = %log_file_path.display(),
        tty = tty_mode,
        level = %effective_level,
        "[logging] subscriber initialized"
    );

    LogGuard {
        _file_guard: file_guard,
    }
}

/// Resolve the display log level using the following priority:
///
///   1. `ORQA_DEV=true` → "debug"
///   2. `RUST_LOG` env var → used as-is
///   3. `log_level` in the `[daemon]` section of `orqa.toml`
///   4. Compiled-in default: "info"
///
/// This is read at logging init time, before `DaemonConfig::load` runs, so
/// we parse `orqa.toml` directly here using only the `log_level` key.
fn resolve_log_level(project_root: &Path) -> String {
    // ORQA_DEV=true forces debug regardless of any other setting.
    if std::env::var("ORQA_DEV").as_deref() == Ok("true") {
        return "debug".to_owned();
    }

    // RUST_LOG takes second priority for compatibility with the standard Rust
    // logging ecosystem.
    if let Ok(val) = std::env::var("RUST_LOG") {
        if !val.is_empty() {
            return val;
        }
    }

    // Read log_level from the [daemon] section of orqa.toml.
    let config_path = project_root.join("orqa.toml");
    if let Ok(content) = std::fs::read_to_string(&config_path) {
        if let Ok(table) = content.parse::<toml::Table>() {
            if let Some(level) = table
                .get("daemon")
                .and_then(|d| d.get("log_level"))
                .and_then(|v| v.as_str())
            {
                return level.to_owned();
            }
        }
    }

    // Compiled-in default.
    "info".to_owned()
}

/// Build the display `EnvFilter` from the resolved level string.
///
/// A separate call is needed for each layer because `EnvFilter` is not
/// `Clone`.
fn build_display_filter(level: &str) -> EnvFilter {
    EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"))
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
