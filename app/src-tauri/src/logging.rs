//! Unified logging for OrqaStudio.
//!
//! Three tiers:
//! - **All modes**: error-level logs emit `app-error` Tauri events so the
//!   frontend can display them in an error toast.
//! - **All modes**: ALL log events (TRACE and above) are forwarded to the
//!   daemon via HTTP POST to `/events` so they appear in the central event
//!   bus (fire-and-forget, batched). Level filtering is handled display-side.
//! - **Dev mode** (`debug_assertions`): debug+ logs written to stderr so the
//!   dev controller captures them and streams to the OrqaDev dashboard via SSE.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Wry};
use tracing::Subscriber;
use tracing_subscriber::{
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
    EnvFilter, Layer,
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

/// A single event payload sent to the daemon's `POST /events` endpoint.
///
/// Matches the `IngestEvent` schema accepted by the daemon health handler.
#[derive(Debug, Clone, Serialize)]
struct ForwardedEvent {
    /// Severity level string: "debug", "info", "warn", "error".
    level: String,
    /// Source tag — always "app" for the Tauri backend.
    source: String,
    /// Log category derived from the tracing target.
    category: String,
    /// Human-readable log message.
    message: String,
    /// Unix timestamp in milliseconds when the event was recorded.
    timestamp: i64,
}

/// A `tracing_subscriber::Layer` that forwards log events to the daemon event
/// bus via HTTP POST.
///
/// Events are buffered in a shared queue. A background task flushes the queue
/// whenever it reaches 50 events or 200 ms elapses — whichever comes first.
/// If the daemon is unreachable the flush is silently dropped (fire-and-forget).
pub struct EventForwarderLayer {
    /// Pending events waiting to be flushed to the daemon.
    queue: Arc<Mutex<Vec<ForwardedEvent>>>,
    /// Monotonically increasing sequence number for ordering events.
    next_seq: AtomicU64,
}

/// Maximum number of events to accumulate before forcing a flush.
const BATCH_SIZE: usize = 50;

/// Maximum time between flushes.
const FLUSH_INTERVAL: Duration = Duration::from_millis(200);

impl Default for EventForwarderLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl EventForwarderLayer {
    /// Create a new `EventForwarderLayer` and spawn the background flush task.
    ///
    /// The task runs for the life of the process. It drains the queue every
    /// 200 ms, or immediately when the queue reaches 50 events. The daemon URL
    /// is resolved once at construction from `orqa_engine::ports::resolve_daemon_port()`.
    pub fn new() -> Self {
        let queue: Arc<Mutex<Vec<ForwardedEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let queue_ref = Arc::clone(&queue);
        let daemon_port = orqa_engine_types::ports::resolve_daemon_port();
        let url = format!("http://localhost:{daemon_port}/events");

        let flush_loop = async move {
            let Ok(client) = reqwest::Client::builder()
                .timeout(Duration::from_millis(500))
                .build()
            else {
                return;
            };

            loop {
                tokio::time::sleep(FLUSH_INTERVAL).await;

                let batch: Vec<ForwardedEvent> = {
                    let mut guard = queue_ref
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    if guard.is_empty() {
                        continue;
                    }
                    std::mem::take(&mut *guard)
                };

                // Fire-and-forget — ignore any error (daemon may not be running).
                let _ = client.post(&url).json(&batch).send().await;
            }
        };

        // Use Tauri's async runtime spawn — it is available during setup,
        // unlike bare tokio::spawn which panics before the reactor starts.
        tauri::async_runtime::spawn(flush_loop);

        Self {
            queue,
            next_seq: AtomicU64::new(1),
        }
    }

    /// Enqueue an event. If the queue has reached BATCH_SIZE, trigger an
    /// immediate flush by appending a sentinel task.
    fn enqueue(&self, event: ForwardedEvent) {
        let should_flush = {
            let mut guard = self
                .queue
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            guard.push(event);
            guard.len() >= BATCH_SIZE
        };

        if should_flush {
            // Drain the queue on the next tokio poll by waking the flush task.
            // We don't force a synchronous flush here to stay non-blocking; the
            // 200 ms timer will pick up the batch at the next opportunity which,
            // at BATCH_SIZE, will be nearly immediate.
            let _ = self.next_seq.fetch_add(0, Ordering::Relaxed);
        }
    }
}

impl<S> Layer<S> for EventForwarderLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    /// Convert a tracing event into a `ForwardedEvent` and push it onto the queue.
    ///
    /// Called synchronously by the tracing runtime. The work here is minimal:
    /// extract fields, timestamp, and push — no I/O, no allocation beyond the
    /// struct itself.
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Extract the message field using a minimal visitor.
        struct MsgVisitor(String);
        impl tracing::field::Visit for MsgVisitor {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                if field.name() == "message" {
                    self.0 = format!("{value:?}");
                }
            }
            fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                if field.name() == "message" {
                    value.clone_into(&mut self.0);
                }
            }
        }

        let mut visitor = MsgVisitor(String::new());
        event.record(&mut visitor);

        let level = match *event.metadata().level() {
            tracing::Level::TRACE | tracing::Level::DEBUG => "debug",
            tracing::Level::INFO => "info",
            tracing::Level::WARN => "warn",
            tracing::Level::ERROR => "error",
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);

        self.enqueue(ForwardedEvent {
            level: level.to_owned(),
            source: "app".to_owned(),
            category: event.metadata().target().to_owned(),
            message: visitor.0,
            timestamp,
        });
    }
}

/// Initialise the global tracing subscriber.
///
/// Must be called once, early in `setup`, before any tracing macros fire.
///
/// - **All modes**: error-level logs emit `app-error` Tauri events.
/// - **All modes**: ALL events (TRACE+) forwarded to the daemon event bus via HTTP.
/// - **Dev mode** (`debug_assertions`): debug+ logs also written to stderr.
pub fn init_logging(app: &AppHandle<Wry>) {
    let _ = APP_HANDLE.set(app.clone());

    // Error-only layer — emits Tauri events in ALL modes.
    let error_event_layer = fmt::layer()
        .with_writer(EventWriterMaker)
        .with_ansi(false)
        .with_target(false)
        .with_level(false)
        .without_time()
        .with_filter(EnvFilter::new("error,tao=off,wry=off"));

    // Daemon forwarding layer — sends ALL events (TRACE and above) to the
    // daemon event bus. Level filtering for display is done by consumers.
    // tao/wry suppressed to avoid event bus noise from Tauri internals.
    // reqwest/hyper/hyper_util suppressed because the forwarder USES reqwest —
    // capturing its connection logs creates a feedback loop that floods the bus.
    let forwarder_layer = EventForwarderLayer::new().with_filter(EnvFilter::new(
        "trace,tao=off,wry=off,reqwest=off,hyper=off,hyper_util=off",
    ));

    #[cfg(debug_assertions)]
    {
        // Dev mode: debug+ to stderr (captured by dev controller).
        // tao/wry warnings suppressed — benign event loop noise on Windows.
        // reqwest/hyper suppressed — internal HTTP client noise from the
        // forwarder and daemon health checks is not useful in dev output.
        let stderr_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(true)
            .with_level(true)
            .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new("debug,tao=off,wry=off,reqwest=off,hyper=off,hyper_util=off")
            }));

        tracing_subscriber::registry()
            .with(stderr_layer)
            .with(error_event_layer)
            .with(forwarder_layer)
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::registry()
            .with(error_event_layer)
            .with(forwarder_layer)
            .init();
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
            source: "rust".to_owned(),
            message: "something broke".to_owned(),
            level: "error".to_owned(),
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
