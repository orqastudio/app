// SSE client for the OrqaDev developer tools — connects to the daemon event
// bus stream and stores events in an in-memory ring buffer for display and
// query by the frontend.
//
// The consumer connects to GET /events/stream on the daemon health endpoint.
// Each SSE data line is a JSON-encoded LogEvent. Events are pushed into a fixed
// 50,000-event ring buffer; when full, the oldest event is evicted. The
// frontend queries events via IPC commands exposed here.
//
// Reconnection uses exponential backoff starting at 1 s, doubling each attempt
// up to a maximum of 30 s. On each reconnect, missed events are loaded from the
// daemon's SQLite history via GET /events?after=<last_timestamp>. Connection
// state changes are broadcast to the frontend via orqa://connection-state events.

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

/// Tauri event name used to broadcast connection state changes to the frontend.
const TAURI_EVENT_CONNECTION: &str = "orqa://connection-state";

/// Initial reconnect backoff in seconds.
const BACKOFF_INITIAL_SECS: u64 = 1;

/// Maximum reconnect backoff in seconds.
const BACKOFF_MAX_SECS: u64 = 30;

/// Connection state emitted to the frontend via `orqa://connection-state` events.
///
/// The frontend renders this in the status bar so the developer always knows
/// whether OrqaDev has a live feed from the daemon.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum ConnectionState {
    /// Actively streaming events from the daemon.
    Connected,
    /// Lost connection; waiting before the next attempt. `attempt` is 1-based.
    Reconnecting {
        /// 1-based reconnect attempt counter, reset to 1 on a successful connection.
        attempt: u32,
    },
    /// Daemon is not running; OrqaDev is polling and waiting for it to start.
    WaitingForDaemon,
}

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
///
/// Public so `dev_controller` can inject parsed controller output as log events.
pub async fn push_event_pub(state: &Arc<EventConsumerState>, event: LogEvent) {
    let mut buffer = state.buffer.lock().await;
    if buffer.len() >= RING_BUFFER_CAPACITY {
        buffer.pop_front();
        let mut dropped = state.dropped_count.lock().await;
        *dropped += 1;
    }
    buffer.push_back(event);
}

/// Emit a connection state change event to the frontend.
fn emit_connection_state(app: &AppHandle, conn_state: &ConnectionState) {
    if let Err(e) = app.emit(TAURI_EVENT_CONNECTION, conn_state) {
        error!(
            subsystem = "event-consumer",
            error = %e,
            "failed to emit connection state event"
        );
    }
}

/// Fetch raw event JSON from `url`. Returns `None` on network or parse errors,
/// logging warnings so the caller can continue without crashing.
async fn fetch_events_json(url: &str) -> Option<Vec<serde_json::Value>> {
    let client = reqwest::Client::new();
    let response = match client.get(url).send().await {
        Ok(r) if r.status().is_success() => r,
        Ok(r) => {
            warn!(subsystem = "event-consumer", status = %r.status(), "event query non-success");
            return None;
        }
        Err(e) => {
            warn!(subsystem = "event-consumer", error = %e, "event query failed");
            return None;
        }
    };
    let body: serde_json::Value = match response.json().await {
        Ok(v) => v,
        Err(e) => {
            warn!(subsystem = "event-consumer", error = %e, "event response not valid JSON");
            return None;
        }
    };
    body.get("events")
        .and_then(|v| v.as_array())
        .cloned()
        .or_else(|| {
            warn!(subsystem = "event-consumer", "event response missing 'events' array");
            None
        })
}

/// Load events missed during a disconnect from the daemon's SQLite history.
///
/// Queries `GET /events?after=<after_ms>&limit=5000` and pushes each returned
/// event into the ring buffer and emits it to the frontend. Silently no-ops
/// when the daemon is unreachable — the subsequent live stream attempt will
/// surface the connectivity error.
async fn load_gap_events(
    app: &AppHandle,
    state: &Arc<EventConsumerState>,
    base_url: &str,
    after_ms: i64,
) {
    let url = format!("{base_url}/events?after={after_ms}&limit=5000");
    info!(subsystem = "event-consumer", after_ms, "loading gap events from SQLite history");

    let Some(events) = fetch_events_json(&url).await else { return };

    let count = events.len();
    for raw in events {
        match serde_json::from_value::<LogEvent>(raw) {
            Ok(event) => {
                if let Err(e) = app.emit(TAURI_EVENT_NEW_LOG, &event) {
                    error!(subsystem = "event-consumer", error = %e, "failed to emit gap event");
                }
                push_event_pub(state, event).await;
            }
            Err(e) => {
                warn!(subsystem = "event-consumer", error = %e, "failed to parse gap event");
            }
        }
    }
    info!(subsystem = "event-consumer", count, "loaded gap events from SQLite history");
}

/// Spawn the background SSE consumer task.
///
/// Connects to the daemon's `/events/stream` endpoint and feeds every received
/// event into the ring buffer. On disconnect or error the task retries with
/// exponential backoff (1 s → 2 s → 4 s → … → 30 s max). Before reconnecting
/// to the live stream, the task queries `/events?after=<last_timestamp>` to
/// replay any events missed during the gap. Connection state transitions are
/// emitted to the frontend via `orqa://connection-state` Tauri events.
pub fn spawn_consumer(app: AppHandle, state: Arc<EventConsumerState>) {
    tauri::async_runtime::spawn(async move {
        // Unix-ms timestamp of the last received event; used to fill gaps on reconnect.
        let mut last_timestamp: Option<i64> = None;
        // Current sleep duration before the next attempt; doubles on each failure.
        let mut backoff_secs = BACKOFF_INITIAL_SECS;
        // 1-based counter for the current reconnect sequence; reset on success.
        let mut attempt: u32 = 1;
        // Flag to skip gap-fill on the very first connection attempt.
        let mut first_attempt = true;

        loop {
            let port = resolve_daemon_port();
            let base_url = format!("http://127.0.0.1:{port}");
            let stream_url = format!("{base_url}/events/stream");

            if first_attempt {
                emit_connection_state(&app, &ConnectionState::WaitingForDaemon);
                info!(subsystem = "event-consumer", %stream_url, "connecting to daemon SSE stream");
            } else {
                // Fill the event gap before reconnecting to the live stream.
                if let Some(ts) = last_timestamp {
                    load_gap_events(&app, &state, &base_url, ts).await;
                }
                emit_connection_state(&app, &ConnectionState::Reconnecting { attempt });
                info!(
                    subsystem = "event-consumer",
                    attempt,
                    backoff_secs,
                    "reconnecting to daemon SSE stream"
                );
            }

            match connect_and_consume(&app, Arc::clone(&state), &stream_url, &mut last_timestamp).await {
                Ok(()) => {
                    info!(subsystem = "event-consumer", "SSE stream ended cleanly — reconnecting");
                    // Server closed the stream gracefully; reset backoff.
                    backoff_secs = BACKOFF_INITIAL_SECS;
                    attempt = 1;
                }
                Err(e) => {
                    warn!(
                        subsystem = "event-consumer",
                        error = %e,
                        backoff_secs,
                        "SSE stream error — retrying after backoff"
                    );
                }
            }

            first_attempt = false;

            // Notify the frontend we are pausing before the next attempt.
            emit_connection_state(&app, &ConnectionState::WaitingForDaemon);
            tokio::time::sleep(tokio::time::Duration::from_secs(backoff_secs)).await;

            // Double the backoff, capped at BACKOFF_MAX_SECS.
            backoff_secs = (backoff_secs * 2).min(BACKOFF_MAX_SECS);
            attempt = attempt.saturating_add(1);
        }
    });
}

/// Connect to the daemon SSE endpoint and consume events until the stream ends.
///
/// Reads the response body in chunks, accumulating bytes until newlines are
/// found. Each `data:` SSE line is parsed as a `LogEvent`, stored in the ring
/// buffer, and emitted as a Tauri frontend event. Emits a `Connected` state
/// event when the HTTP response is successfully established. Updates
/// `last_timestamp` on each event so the caller can fill gaps on reconnect.
/// Returns `Ok(())` when the server closes the stream, or an error on failure.
async fn connect_and_consume(
    app: &AppHandle,
    state: Arc<EventConsumerState>,
    url: &str,
    last_timestamp: &mut Option<i64>,
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

    // Notify the frontend the stream is live.
    emit_connection_state(app, &ConnectionState::Connected);
    info!(subsystem = "event-consumer", "SSE stream connected");

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
        process_sse_lines(app, &state, &mut line_buf, last_timestamp).await;
    }
    Ok(())
}

/// Parse and dispatch all complete SSE lines from `buf`, consuming them in place.
///
/// Each complete line ending with `\n` is extracted, `data:` prefix stripped,
/// and the payload deserialized as `LogEvent`. Incomplete trailing data is left
/// in `buf` for the next chunk. Updates `last_timestamp` to the most recent
/// event's timestamp so reconnect logic can query the missed event gap.
async fn process_sse_lines(
    app: &AppHandle,
    state: &Arc<EventConsumerState>,
    buf: &mut String,
    last_timestamp: &mut Option<i64>,
) {
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
                    // Track the most recent event timestamp for gap-fill on reconnect.
                    *last_timestamp = Some(event.timestamp);
                    // Emit to frontend before storing so the UI reacts immediately.
                    if let Err(e) = app.emit(TAURI_EVENT_NEW_LOG, &event) {
                        error!(
                            subsystem = "event-consumer",
                            error = %e,
                            "failed to emit Tauri log event"
                        );
                    }
                    push_event_pub(state, event).await;
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

/// Filter parameters accepted by `devtools_query_history`.
///
/// All fields are optional and map directly to the daemon's `GET /events`
/// query parameters. `before` is converted to the daemon's `after` parameter
/// by querying events whose timestamp is strictly less than `before`.
#[derive(Debug, Deserialize)]
pub struct HistoryQueryParams {
    /// Unix timestamp in milliseconds — return events before this point.
    /// Used to page backward from the oldest visible event.
    pub before: Option<i64>,
    /// Filter by source string (e.g. "Daemon", "App").
    pub source: Option<String>,
    /// Filter by level string (e.g. "Error", "Warn").
    pub level: Option<String>,
    /// Maximum events to return. Capped at 1000 by this command.
    pub limit: Option<u32>,
}

/// Response from the daemon's `GET /events` HTTP endpoint.
#[derive(Debug, Deserialize)]
struct DaemonEventsResponse {
    events: Vec<serde_json::Value>,
}

/// Build the `GET /events` URL for a history query.
///
/// When `before` is set the daemon is asked for all events (after=0) with a
/// high fetch limit so the caller can filter client-side to those with
/// timestamp < before. The daemon currently only supports an `after` lower
/// bound; `before` filtering is applied after the response is received.
fn build_history_url(port: u16, params: &HistoryQueryParams, limit: u32) -> String {
    // When paging backward we fetch a larger window so we can trim client-side.
    let fetch_limit = if params.before.is_some() { 5000_u32 } else { limit };
    let mut parts = vec![format!("limit={fetch_limit}")];
    if let Some(ref source) = params.source {
        parts.push(format!("source={source}"));
    }
    if let Some(ref level) = params.level {
        parts.push(format!("level={level}"));
    }
    format!("http://127.0.0.1:{port}/events?{}", parts.join("&"))
}

/// Filter `events` to those with timestamp < `before` and return the last
/// `limit` of them (most recent before the cutoff).
fn apply_before_cutoff(mut events: Vec<serde_json::Value>, before: i64, limit: usize) -> Vec<serde_json::Value> {
    events.retain(|ev| {
        ev.get("timestamp")
            .and_then(serde_json::Value::as_i64)
            .is_some_and(|t| t < before)
    });
    if events.len() > limit {
        events = events.split_off(events.len() - limit);
    }
    events
}

/// IPC command — query historical events from the daemon's SQLite store.
///
/// Calls the daemon's `GET /events` HTTP endpoint with the supplied filter
/// parameters. Results are returned as raw JSON values so the frontend can
/// merge them into the log store with its own deduplication logic. A maximum
/// of 1000 events per request is enforced; the caller pages backward by
/// supplying a decreasing `before` timestamp.
#[tauri::command]
pub async fn devtools_query_history(
    params: HistoryQueryParams,
) -> Result<Vec<serde_json::Value>, String> {
    let port = resolve_daemon_port();
    let limit = params.limit.unwrap_or(1000).min(1000);
    let url = build_history_url(port, &params, limit);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("failed to reach daemon: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("daemon returned {}", response.status()));
    }

    let body: DaemonEventsResponse = response
        .json()
        .await
        .map_err(|e| format!("failed to parse daemon response: {e}"))?;

    let events = if let Some(before) = params.before {
        apply_before_cutoff(body.events, before, limit as usize)
    } else {
        body.events
    };

    info!(
        subsystem = "history-query",
        count = events.len(),
        "history query returned events"
    );

    Ok(events)
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
