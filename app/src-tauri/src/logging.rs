//! Unified logging for OrqaStudio.
//!
//! Two tiers:
//! - **Dev mode** (`debug_assertions`): info+ logs written to stderr so the dev
//!   controller captures them and streams to the OrqaDev dashboard via SSE.
//! - **All modes**: error-level logs emit `app-error` Tauri events so the
//!   frontend can display them in an error toast.

use std::sync::OnceLock;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Wry};
use tracing_subscriber::{
    fmt::{self, MakeWriter},
    prelude::*,
    EnvFilter,
};

/// Global app handle for emitting events from the tracing layer.
static APP_HANDLE: OnceLock<AppHandle<Wry>> = OnceLock::new();

/// Payload emitted as an `app-error` Tauri event.
#[derive(Debug, Clone, Serialize)]
pub struct AppErrorEvent {
    /// Origin of the error (e.g., "rust", "sidecar").
    pub source: String,
    /// Human-readable error description.
    pub message: String,
    /// Log level string (always "error" for this event type).
    pub level: String,
}

/// Event name for errors surfaced to the frontend.
pub const APP_ERROR_EVENT: &str = "app-error";

/// A writer that emits error-level log lines as Tauri events.
struct EventWriter;

impl std::io::Write for EventWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = String::from_utf8_lossy(buf).trim().to_owned();
        if msg.is_empty() {
            return Ok(buf.len());
        }

        if let Some(handle) = APP_HANDLE.get() {
            let event = AppErrorEvent {
                source: "rust".to_owned(),
                message: msg,
                level: "error".to_owned(),
            };
            let _ = handle.emit(APP_ERROR_EVENT, &event);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// `MakeWriter` adapter for the event emitter.
struct EventWriterMaker;

impl<'a> MakeWriter<'a> for EventWriterMaker {
    type Writer = EventWriter;

    fn make_writer(&'a self) -> Self::Writer {
        EventWriter
    }
}

/// Initialise the global tracing subscriber.
///
/// Must be called once, early in `setup`, before any tracing macros fire.
///
/// - In **debug** builds: info+ logs to stderr, errors also emit Tauri events.
/// - In **release** builds: only error-level logs emitted as Tauri events.
pub fn init_logging(app: &AppHandle<Wry>) {
    let _ = APP_HANDLE.set(app.clone());

    // Error-only layer — emits Tauri events in ALL modes.
    let event_layer = fmt::layer()
        .with_writer(EventWriterMaker)
        .with_ansi(false)
        .with_target(false)
        .with_level(false)
        .without_time()
        .with_filter(EnvFilter::new("error,tao=off,wry=off"));

    #[cfg(debug_assertions)]
    {
        // Dev mode: info+ to stderr (captured by dev controller).
        // tao/wry warnings suppressed — benign event loop noise on Windows.
        let stderr_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(true)
            .with_level(true)
            .with_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("info,tao=off,wry=off")),
            );

        tracing_subscriber::registry()
            .with(stderr_layer)
            .with(event_layer)
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::registry().with(event_layer).init();
    }
}

/// Emit an `app-error` event from non-tracing code (e.g. sidecar stderr parser).
pub fn emit_app_error(source: &str, message: &str) {
    if let Some(handle) = APP_HANDLE.get() {
        let event = AppErrorEvent {
            source: source.to_owned(),
            message: message.to_owned(),
            level: "error".to_owned(),
        };
        let _ = handle.emit(APP_ERROR_EVENT, &event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_error_event_serializes() {
        let event = AppErrorEvent {
            source: "rust".to_string(),
            message: "something broke".to_string(),
            level: "error".to_string(),
        };
        let json = serde_json::to_value(&event).expect("should serialize");
        assert_eq!(json["source"], "rust");
        assert_eq!(json["message"], "something broke");
        assert_eq!(json["level"], "error");
    }

    #[test]
    fn emit_app_error_without_handle_does_not_panic() {
        // APP_HANDLE is not set in test context — this should not panic.
        emit_app_error("test", "test message");
    }
}
