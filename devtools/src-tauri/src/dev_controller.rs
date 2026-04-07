// Dev environment process controller — shells out to `orqa dev start-processes`
// and feeds stdout/stderr into the OrqaDev log stream.
//
// OrqaDev launches this instead of the user running `orqa dev` directly. This
// ensures all process output flows through the devtools event ring buffer so
// the developer sees daemon startup, Vite builds, Tauri compilation, and file
// watcher activity in a single log view.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;
use tracing::{info, warn};

use orqa_engine_types::types::event::{EventLevel, EventSource, EventTier, LogEvent};

use crate::events::{EventBatchWriter, EventConsumerState};

/// Tauri event name for new log events (matches events.rs constant).
const TAURI_EVENT_NEW_LOG: &str = "orqa://log-event";

/// Tauri event name for dev controller state changes.
const TAURI_EVENT_DEV_STATE: &str = "orqa://dev-controller-state";

/// Tauri event name for the dependency graph topology.
const TAURI_EVENT_GRAPH_TOPOLOGY: &str = "orqa://graph-topology";

/// Monotonically increasing event ID for controller-generated events.
/// Starts at a high offset to avoid colliding with daemon-generated IDs.
static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(10_000_000);

/// Shared state for the dev controller child process.
pub struct DevControllerState {
    /// Whether the controller process is currently running.
    running: AtomicBool,
    /// Handle to the child process, if running. Used for killing on stop.
    child_id: Mutex<Option<u32>>,
}

/// Shared state for the process manager dependency graph topology.
/// Populated once when the PM emits the `graph-topology` JSON event at startup.
pub struct GraphTopologyState(pub Mutex<Option<serde_json::Value>>);

impl DevControllerState {
    /// Create a new dev controller state with no running processes.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            running: AtomicBool::new(false),
            child_id: Mutex::new(None),
        })
    }
}

/// State emitted to the frontend via `orqa://dev-controller-state`.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum DevState {
    /// No dev processes running.
    Stopped,
    /// Spawning `orqa dev start-processes`.
    Starting,
    /// Controller process is alive and managing dev processes.
    Running,
    /// Sending stop signal, waiting for processes to exit.
    Stopping,
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Parse a prefixed log line from the dev controller output.
///
/// The controller prefixes lines as: `HH:MM:SS [source] message`
/// We extract the source tag and message. Lines without the prefix pattern
/// are treated as dev-controller output.
///
/// JSON lines (from the ProcessManager) are parsed as structured NodeEvents
/// and mapped to the appropriate source/level/category/message.
#[allow(clippy::too_many_lines)]
fn parse_controller_line(line: &str) -> (EventSource, EventLevel, String, String) {
    // Detect structured JSON events from the ProcessManager.
    let trimmed = line.trim();
    if trimmed.starts_with('{') {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            let status = val.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let node_id = val.get("nodeId").and_then(|v| v.as_str()).unwrap_or("pm");
            let node_name = val.get("nodeName").and_then(|v| v.as_str()).unwrap_or("");
            let error = val.get("error").and_then(|v| v.as_str()).unwrap_or("");
            let duration = val.get("durationMs").and_then(serde_json::Value::as_u64);

            let level = match status {
                "build-failed" | "crashed" => EventLevel::Error,
                _ => EventLevel::Info,
            };

            let category = format!("process:{node_id}");

            let message = match status {
                "building" | "rebuilding" => format!("Building {node_name}..."),
                "built" => match duration {
                    Some(ms) => format!("Built {node_name} in {ms}ms"),
                    None => format!("Built {node_name}"),
                },
                "build-failed" => format!("Build failed for {node_name}: {error}"),
                "starting" => format!("Starting {node_name}..."),
                "running" => format!("{node_name} running"),
                "watching" => format!("Watching {node_name}"),
                "stopping" => format!("Stopping {node_name}..."),
                "stopped" => format!("{node_name} stopped"),
                "crashed" => format!("{node_name} crashed: {error}"),
                _ => format!("{node_name}: {status}"),
            };

            return (EventSource::DevController, level, category, message);
        }
    }

    // Strip ANSI escape codes for parsing (keep original for message)
    let stripped = strip_ansi(line);

    // Try to extract [source] prefix: "HH:MM:SS [tag] message"
    if let Some(bracket_start) = stripped.find('[') {
        if let Some(bracket_end) = stripped[bracket_start..].find(']') {
            let tag = &stripped[bracket_start + 1..bracket_start + bracket_end];
            let message = stripped[bracket_start + bracket_end + 1..].trim();

            let source = match tag {
                "app" => EventSource::App,
                "search" => EventSource::Search,
                "daemon" => EventSource::Daemon,
                _ => EventSource::DevController,
            };

            // For app/search sources, try to parse Rust tracing format:
            // "2026-04-07T13:55:45.787509Z DEBUG module::path: actual message"
            // Extract the level and strip the ISO timestamp + module prefix.
            if matches!(source, EventSource::App | EventSource::Search) {
                if let Some(parsed) = parse_rust_tracing(message) {
                    return (source, parsed.0, parsed.1, parsed.2);
                }
            }

            let level = if stripped.contains("[31m")
                || message.contains("error")
                || message.contains("ERROR")
            {
                EventLevel::Error
            } else if stripped.contains("[33m")
                || message.contains("warning")
                || message.contains("WARN")
            {
                EventLevel::Warn
            } else if stripped.contains("[32m") {
                EventLevel::Info
            } else {
                EventLevel::Debug
            };

            let category = match tag {
                "ctrl" => "controller",
                "app" => "tauri-dev",
                "search" => "search-server",
                "daemon" => "daemon",
                "storybook" => "storybook",
                "build" | "watch" => "file-watcher",
                "plugin" => "plugin-watcher",
                s if s.starts_with("tsc:") => "typescript",
                _ => "controller",
            };

            return (source, level, category.to_owned(), message.to_owned());
        }
    }

    // Fallback: unstructured line
    (
        EventSource::DevController,
        EventLevel::Info,
        "controller".to_owned(),
        stripped.clone(),
    )
}

/// Try to parse a Rust tracing-formatted line.
///
/// Format: `2026-04-07T13:55:45.787509Z LEVEL module::path: message`
/// Returns (level, category, message) if the line matches, or None if it doesn't.
fn parse_rust_tracing(line: &str) -> Option<(EventLevel, String, String)> {
    // ISO timestamp is 20+ chars followed by Z and a space.
    let rest = if line.len() > 22 && line.as_bytes().get(10) == Some(&b'T') {
        // Skip the ISO timestamp (everything up to and including "Z ")
        line.find("Z ").map(|pos| &line[pos + 2..])
    } else {
        None
    }?;

    // Next token is the level keyword.
    let (level, after_level) = if let Some(r) = rest.strip_prefix("ERROR ") {
        (EventLevel::Error, r)
    } else if let Some(r) = rest.strip_prefix("WARN ") {
        (EventLevel::Warn, r)
    } else if let Some(r) = rest.strip_prefix("INFO ") {
        (EventLevel::Info, r)
    } else if let Some(r) = rest.strip_prefix("DEBUG ") {
        (EventLevel::Debug, r)
    } else if let Some(r) = rest.strip_prefix("TRACE ") {
        (EventLevel::Debug, r)
    } else {
        return None;
    };

    // After level: "module::path: message" or "module::path: key=value key=value"
    let (category, message) = if let Some(colon_pos) = after_level.find(": ") {
        let module = &after_level[..colon_pos];
        let msg = after_level[colon_pos + 2..].trim();
        (module.to_owned(), msg.to_owned())
    } else {
        ("app".to_owned(), after_level.trim().to_owned())
    };

    Some((level, category, message))
}

/// Strip ANSI escape sequences from a string.
fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip until we hit a letter (the terminator of the escape sequence)
            while let Some(&next) = chars.peek() {
                chars.next();
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Push a log event into the ring buffer, emit it to the frontend, and queue
/// it for SQLite persistence via the batch writer.
async fn emit_log(
    app: &AppHandle,
    state: &Arc<EventConsumerState>,
    batch_writer: &Arc<EventBatchWriter>,
    source: EventSource,
    level: EventLevel,
    category: String,
    message: String,
) {
    let event = LogEvent {
        id: next_id(),
        timestamp: now_ms(),
        level,
        source,
        tier: EventTier::Build,
        category,
        message: message.clone(),
        metadata: serde_json::Value::Null,
        session_id: None,
        fingerprint: None,
        message_template: None,
        correlation_id: None,
        stack_frames: None,
    };

    // Push to ring buffer.
    crate::events::push_event_pub(state, event.clone()).await;

    // Queue for SQLite persistence. Non-blocking channel send.
    batch_writer.queue_event(event.clone());

    // Emit to frontend.
    let _ = app.emit(TAURI_EVENT_NEW_LOG, &event);
}

fn emit_dev_state(app: &AppHandle, state: &DevState) {
    let _ = app.emit(TAURI_EVENT_DEV_STATE, state);
}

/// IPC command — start the dev environment by spawning `orqa dev start-processes`.
#[allow(clippy::too_many_lines)]
#[tauri::command]
pub async fn devtools_start_dev(
    app: AppHandle,
    dev_state: State<'_, Arc<DevControllerState>>,
    event_state: State<'_, Arc<EventConsumerState>>,
    batch_writer: State<'_, Arc<EventBatchWriter>>,
) -> Result<(), String> {
    if dev_state.running.load(Ordering::Relaxed) {
        return Err("Dev environment is already running".into());
    }

    dev_state.running.store(true, Ordering::Relaxed);
    emit_dev_state(&app, &DevState::Starting);

    // Resolve the orqa CLI binary. On Windows it's orqa.cmd via npm link.
    let orqa_cmd = if cfg!(target_os = "windows") {
        "orqa.cmd".to_owned()
    } else {
        "orqa".to_owned()
    };

    let dev_state_ref = Arc::clone(&dev_state);
    let event_state_ref = Arc::clone(&event_state);
    let batch_writer_ref = Arc::clone(&batch_writer);

    tauri::async_runtime::spawn(async move {
        emit_log(
            &app,
            &event_state_ref,
            &batch_writer_ref,
            EventSource::DevController,
            EventLevel::Info,
            "controller".into(),
            "Starting dev environment...".into(),
        )
        .await;

        let result = Command::new(&orqa_cmd)
            .args(["dev", "start-processes"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn();

        let mut child = match result {
            Ok(c) => c,
            Err(e) => {
                warn!("failed to spawn orqa dev start-processes: {e}");
                emit_log(
                    &app,
                    &event_state_ref,
                    &batch_writer_ref,
                    EventSource::DevController,
                    EventLevel::Error,
                    "controller".into(),
                    format!("Failed to start dev environment: {e}"),
                )
                .await;
                dev_state_ref.running.store(false, Ordering::Relaxed);
                emit_dev_state(&app, &DevState::Stopped);
                return;
            }
        };

        // Store child PID for stop command.
        if let Some(pid) = child.id() {
            *dev_state_ref.child_id.lock().await = Some(pid);
        }

        emit_dev_state(&app, &DevState::Running);

        crate::events::spawn_consumer(app.clone(), Arc::clone(&event_state_ref));

        // Read stdout and stderr concurrently, converting lines to log events.
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let app_out = app.clone();
        let state_out = Arc::clone(&event_state_ref);
        let bw_out = Arc::clone(&batch_writer_ref);
        let topo_state = app
            .try_state::<Arc<GraphTopologyState>>()
            .map(|s| Arc::clone(&s));
        let stdout_task = tauri::async_runtime::spawn(async move {
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if line.trim().is_empty() {
                        continue;
                    }
                    // Intercept graph-topology events before normal log parsing.
                    if line.trim_start().starts_with('{') {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line.trim()) {
                            if val.get("type").and_then(|v| v.as_str()) == Some("graph-topology") {
                                if let Some(ref ts) = topo_state {
                                    *ts.0.lock().await = Some(val.clone());
                                }
                                let _ = app_out.emit(TAURI_EVENT_GRAPH_TOPOLOGY, &val);
                                continue;
                            }
                        }
                    }
                    let (source, level, category, message) = parse_controller_line(&line);
                    emit_log(
                        &app_out, &state_out, &bw_out, source, level, category, message,
                    )
                    .await;
                }
            }
        });

        let app_err = app.clone();
        let state_err = Arc::clone(&event_state_ref);
        let bw_err = Arc::clone(&batch_writer_ref);
        let stderr_task = tauri::async_runtime::spawn(async move {
            if let Some(stderr) = stderr {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let (source, level, category, message) = parse_controller_line(&line);
                    emit_log(
                        &app_err, &state_err, &bw_err, source, level, category, message,
                    )
                    .await;
                }
            }
        });

        // Wait for the child process to exit.
        let _ = stdout_task.await;
        let _ = stderr_task.await;
        let status = child.wait().await;

        let exit_msg = match status {
            Ok(s) if s.success() => "Dev environment stopped.".to_owned(),
            Ok(s) => format!(
                "Dev environment exited with code {}",
                s.code().unwrap_or(-1)
            ),
            Err(e) => format!("Dev environment process error: {e}"),
        };

        emit_log(
            &app,
            &event_state_ref,
            &batch_writer_ref,
            EventSource::DevController,
            EventLevel::Info,
            "controller".into(),
            exit_msg,
        )
        .await;

        *dev_state_ref.child_id.lock().await = None;
        dev_state_ref.running.store(false, Ordering::Relaxed);
        emit_dev_state(&app, &DevState::Stopped);
    });

    Ok(())
}

/// IPC command — stop the dev environment.
#[allow(clippy::too_many_lines)]
#[tauri::command]
pub async fn devtools_stop_dev(
    app: AppHandle,
    dev_state: State<'_, Arc<DevControllerState>>,
    event_state: State<'_, Arc<EventConsumerState>>,
    batch_writer: State<'_, Arc<EventBatchWriter>>,
) -> Result<(), String> {
    if !dev_state.running.load(Ordering::Relaxed) {
        return Err("Dev environment is not running".into());
    }

    emit_dev_state(&app, &DevState::Stopping);

    emit_log(
        &app,
        &event_state,
        &batch_writer,
        EventSource::DevController,
        EventLevel::Info,
        "controller".into(),
        "Stopping dev environment...".into(),
    )
    .await;

    // Send stop signal via the CLI
    let orqa_cmd = if cfg!(target_os = "windows") {
        "orqa.cmd"
    } else {
        "orqa"
    };

    match Command::new(orqa_cmd).args(["dev", "stop"]).output().await {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                warn!("orqa dev stop failed: {stderr}");
            }
        }
        Err(e) => {
            warn!("failed to run orqa dev stop: {e}");
            // Fall back to killing the child directly
            if let Some(pid) = *dev_state.child_id.lock().await {
                info!("falling back to killing PID {pid}");
                #[cfg(target_os = "windows")]
                {
                    let _ = Command::new("taskkill")
                        .args(["/F", "/T", "/PID", &pid.to_string()])
                        .output()
                        .await;
                }
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = Command::new("kill")
                        .args(["-9", &pid.to_string()])
                        .output()
                        .await;
                }
            }
        }
    }

    Ok(())
}

/// IPC command — check if the dev environment is running.
#[tauri::command]
pub async fn devtools_dev_status(
    dev_state: State<'_, Arc<DevControllerState>>,
) -> Result<bool, String> {
    Ok(dev_state.running.load(Ordering::Relaxed))
}
