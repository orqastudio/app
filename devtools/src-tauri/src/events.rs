// SSE client for the OrqaDev developer tools — connects to the daemon event
// bus stream and stores events in an in-memory ring buffer for display and
// query by the frontend.
//
// The consumer connects to GET /events/stream on the daemon health endpoint.
// Each SSE data line is a JSON-encoded LogEvent. Events are pushed into a fixed
// 50,000-event ring buffer; when full, the oldest event is evicted. The
// frontend queries events via IPC commands exposed here.

use std::collections::VecDeque;
use std::sync::Arc;

use orqa_engine_types::ports::resolve_daemon_port;
use orqa_engine_types::types::event::LogEvent;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// Maximum number of events retained in the ring buffer.
const RING_BUFFER_CAPACITY: usize = 50_000;

/// Tauri event name used to push new log events to the frontend.
const TAURI_EVENT_NEW_LOG: &str = "orqa://log-event";

/// Shared state holding the event ring buffer and dropped-event counter.
pub struct EventConsumerState {
    /// Ring buffer of received log events, ordered oldest-to-newest.
    buffer: Mutex<VecDeque<LogEvent>>,
    /// Cumulative count of events evicted from the front due to overflow.
    dropped_count: Mutex<u64>,
}

impl EventConsumerState {
    /// Create a new empty consumer state.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            buffer: Mutex::new(VecDeque::with_capacity(RING_BUFFER_CAPACITY)),
            dropped_count: Mutex::new(0),
        })
    }
}

/// Push a log event into the ring buffer, evicting the oldest if at capacity.
async fn push_event(state: &Arc<EventConsumerState>, event: LogEvent) {
    let mut buffer = state.buffer.lock().await;
    if buffer.len() >= RING_BUFFER_CAPACITY {
        buffer.pop_front();
        let mut dropped = state.dropped_count.lock().await;
        *dropped += 1;
    }
    buffer.push_back(event);
}

/// Spawn the background SSE consumer task.
///
/// Connects to the daemon's `/events/stream` endpoint and feeds every received
/// event into the ring buffer. The task retries with a 2-second backoff when
/// the daemon is unreachable or the connection drops. It also emits a Tauri
/// event for each received log event so the frontend can react in real time.
pub fn spawn_consumer(app: AppHandle, state: Arc<EventConsumerState>) {
    tokio::spawn(async move {
        loop {
            let port = resolve_daemon_port();
            let url = format!("http://127.0.0.1:{port}/events/stream");
            info!(subsystem = "event-consumer", %url, "connecting to daemon SSE stream");

            match connect_and_consume(&app, Arc::clone(&state), &url).await {
                Ok(()) => {
                    info!(
                        subsystem = "event-consumer",
                        "SSE stream ended cleanly — reconnecting"
                    );
                }
                Err(e) => {
                    warn!(
                        subsystem = "event-consumer",
                        error = %e,
                        "SSE stream error — retrying in 2s"
                    );
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });
}

/// Connect to the daemon SSE endpoint and consume events until the stream ends.
///
/// Reads the response body in chunks, accumulating bytes until newlines are
/// found. Each `data:` SSE line is parsed as a `LogEvent`, stored in the ring
/// buffer, and emitted as a Tauri frontend event. Returns `Ok(())` when the
/// server closes the stream, or an error on connection failure.
async fn connect_and_consume(
    app: &AppHandle,
    state: Arc<EventConsumerState>,
    url: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let mut response = client
        .get(url)
        .header("Accept", "text/event-stream")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("daemon SSE endpoint returned {}", response.status()).into());
    }

    // Accumulate partial line data across chunks. SSE lines end with '\n'.
    let mut line_buf = String::new();
    while let Some(chunk) = response.chunk().await? {
        let text = match std::str::from_utf8(&chunk) {
            Ok(s) => s,
            Err(e) => {
                warn!(subsystem = "event-consumer", error = %e, "non-UTF8 SSE chunk — skipping");
                continue;
            }
        };
        line_buf.push_str(text);
        process_sse_lines(app, &state, &mut line_buf).await;
    }
    Ok(())
}

/// Parse and dispatch all complete SSE lines from `buf`, consuming them in place.
///
/// Each complete line ending with `\n` is extracted, `data:` prefix stripped,
/// and the payload deserialized as `LogEvent`. Incomplete trailing data is left
/// in `buf` for the next chunk.
async fn process_sse_lines(app: &AppHandle, state: &Arc<EventConsumerState>, buf: &mut String) {
    while let Some(newline_pos) = buf.find('\n') {
        let line = buf[..newline_pos].trim_end_matches('\r').to_owned();
        let remainder = buf[newline_pos + 1..].to_owned();
        *buf = remainder;
        if let Some(data) = line.strip_prefix("data:") {
            let data = data.trim();
            if data.is_empty() {
                continue;
            }
            match serde_json::from_str::<LogEvent>(data) {
                Ok(event) => {
                    // Emit to frontend before storing so the UI reacts immediately.
                    if let Err(e) = app.emit(TAURI_EVENT_NEW_LOG, &event) {
                        error!(
                            subsystem = "event-consumer",
                            error = %e,
                            "failed to emit Tauri log event"
                        );
                    }
                    push_event(state, event).await;
                }
                Err(e) => {
                    warn!(
                        subsystem = "event-consumer",
                        error = %e,
                        raw = data,
                        "failed to parse SSE event JSON"
                    );
                }
            }
        }
    }
}

/// Query parameters for the `get_events` IPC command.
#[derive(Debug, Deserialize)]
pub struct EventQueryParams {
    /// Return only events at or after this index in the ring buffer (0-based).
    /// Absent means return from the start of the buffer.
    pub offset: Option<usize>,
    /// Maximum number of events to return. Absent means return all.
    pub limit: Option<usize>,
    /// Filter by source string (matches `EventSource` display name).
    pub source: Option<String>,
    /// Filter by level string ("debug", "info", "warn", "error", "perf").
    pub level: Option<String>,
    /// Filter by category substring (case-insensitive).
    pub category: Option<String>,
}

/// Response envelope for the `get_events` IPC command.
#[derive(Debug, Serialize)]
pub struct EventQueryResponse {
    /// Slice of events matching the query.
    pub events: Vec<LogEvent>,
    /// Total number of events currently in the ring buffer.
    pub total: usize,
    /// Cumulative count of events dropped from the front due to overflow.
    pub dropped: u64,
}

/// IPC command — query buffered log events from the ring buffer.
///
/// Supports pagination via `offset`/`limit` and filtering by `source`,
/// `level`, and `category`. Returns the matching slice along with buffer
/// metadata so the caller can detect gaps.
#[tauri::command]
pub async fn get_events(
    state: State<'_, Arc<EventConsumerState>>,
    params: EventQueryParams,
) -> Result<EventQueryResponse, String> {
    let buffer = state.buffer.lock().await;
    let dropped = *state.dropped_count.lock().await;
    let total = buffer.len();

    let source_filter = params.source.as_deref().map(str::to_lowercase);
    let level_filter = params.level.as_deref().map(str::to_lowercase);
    let category_filter = params.category.as_deref().map(str::to_lowercase);

    let filtered: Vec<&LogEvent> = buffer
        .iter()
        .filter(|e| {
            if let Some(ref s) = source_filter {
                if !e.source.to_string().to_lowercase().contains(s.as_str()) {
                    return false;
                }
            }
            if let Some(ref l) = level_filter {
                if !e.level.to_string().to_lowercase().contains(l.as_str()) {
                    return false;
                }
            }
            if let Some(ref c) = category_filter {
                if !e.category.to_lowercase().contains(c.as_str()) {
                    return false;
                }
            }
            true
        })
        .collect();

    let offset = params.offset.unwrap_or(0);
    let slice = if offset >= filtered.len() {
        &filtered[..0]
    } else {
        let end = params
            .limit
            .map_or(filtered.len(), |l| (offset + l).min(filtered.len()));
        &filtered[offset..end]
    };

    Ok(EventQueryResponse {
        events: slice.iter().map(|e| (*e).clone()).collect(),
        total,
        dropped,
    })
}

/// IPC command — clear all events from the ring buffer.
///
/// Resets both the buffer and the dropped count. Useful when the developer
/// wants a clean slate without restarting the devtools app.
#[tauri::command]
pub async fn clear_events(state: State<'_, Arc<EventConsumerState>>) -> Result<(), String> {
    let mut buffer = state.buffer.lock().await;
    let mut dropped = state.dropped_count.lock().await;
    buffer.clear();
    *dropped = 0;
    info!(subsystem = "event-consumer", "event buffer cleared via IPC");
    Ok(())
}

/// Statistics about the current state of the event ring buffer.
#[derive(Debug, Serialize)]
pub struct EventBufferStats {
    /// Number of events currently in the buffer.
    pub buffered: usize,
    /// Cumulative count of events evicted due to overflow.
    pub dropped: u64,
    /// Maximum capacity of the ring buffer.
    pub capacity: usize,
}

/// IPC command — return ring buffer statistics without fetching events.
#[tauri::command]
pub async fn event_buffer_stats(
    state: State<'_, Arc<EventConsumerState>>,
) -> Result<EventBufferStats, String> {
    let buffer = state.buffer.lock().await;
    let dropped = *state.dropped_count.lock().await;
    Ok(EventBufferStats {
        buffered: buffer.len(),
        dropped,
        capacity: RING_BUFFER_CAPACITY,
    })
}
