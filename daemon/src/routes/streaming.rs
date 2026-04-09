// Streaming routes: daemon-side stream loop management with SSE delivery.
//
// The daemon owns the sidecar process lifecycle and runs the full stream loop
// (tool execution, enforcement, workflow tracking) in-process. The Tauri app
// is a pure SSE consumer — it posts messages and subscribes to the SSE stream.
//
// Per-session stream state lives in `SessionStreamRegistry` stored in `HealthState`.
// Each active session has a broadcast channel, a cancellation flag, and a
// pending-approval map for the tool approval handshake.
//
// Endpoints:
//   POST /sessions/:id/messages      — send a user message and start streaming
//   GET  /sessions/:id/stream        — SSE stream of events for a session
//   POST /sessions/:id/stop          — cancel an active stream
//   POST /sessions/:id/tool-approval — respond to a pending tool approval request

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use axum::extract::{Path as AxumPath, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, Sse};
use axum::Json;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;
use tracing::{debug, error, info};

use orqa_enforcement::engine::EnforcementEngine;
use orqa_enforcement::store::load_rules;
use orqa_engine_types::types::message::{MessageRole, StreamStatus};
use orqa_engine_types::types::streaming::StreamEvent;
use orqa_prompt::resolve_system_prompt;
use orqa_search::SearchEngine;
use orqa_streaming::protocol::{SidecarRequest, SidecarResponse};
use orqa_streaming::stream_loop::{
    accumulate_response, is_terminal, translate_response, StreamAccumulator,
};
use orqa_streaming::tools::{
    format_search_results, tool_bash, tool_edit_file, tool_glob, tool_grep, tool_load_knowledge,
    tool_read_file, tool_write_file, truncate_tool_output, READ_ONLY_TOOLS,
};
use orqa_workflow::gates::{evaluate_stop_verdicts, evaluate_write_verdicts, ProcessGateConfig};
use orqa_workflow::tracker::{TrackerConfig, WorkflowTracker};

use orqa_storage::traits::{MessageRepository as _, SessionRepository as _};

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Stream event type (serialised for SSE delivery)
// ---------------------------------------------------------------------------

/// A single event emitted by the stream loop and delivered to SSE consumers.
///
/// Wraps `StreamEvent` as a flat JSON object: `event_type` is the discriminant
/// name (e.g. "StreamStart"), `payload` is the event data. This matches the
/// shape the Tauri app's frontend previously received via `Channel<StreamEvent>`.
#[derive(Debug, Clone, Serialize)]
pub struct SessionStreamEvent {
    /// Discriminant name of the event (e.g. "StreamStart", "TextDelta").
    pub event_type: String,
    /// Event-specific payload serialised to JSON.
    pub payload: serde_json::Value,
}

impl SessionStreamEvent {
    /// Wrap a `StreamEvent` into a `SessionStreamEvent` for SSE delivery.
    ///
    /// Serialises the event to JSON, then extracts the `type` tag and `data`
    /// fields from the tagged-enum representation.
    fn from_stream_event(event: StreamEvent) -> Option<Self> {
        let json = serde_json::to_value(&event).ok()?;
        // StreamEvent uses #[serde(tag = "type", content = "data")] with snake_case.
        // Extract the type tag and data payload separately.
        let obj = json.as_object()?;
        let type_tag = obj.get("type")?.as_str()?;
        // Convert snake_case type tag to PascalCase for the frontend.
        let event_type = snake_to_pascal(type_tag);
        let payload = obj.get("data").cloned().unwrap_or(serde_json::Value::Null);
        Some(Self {
            event_type,
            payload,
        })
    }

    /// Create a bare error event (not from a StreamEvent variant).
    fn error(code: &str, message: &str, recoverable: bool) -> Self {
        Self {
            event_type: "StreamError".to_owned(),
            payload: serde_json::json!({
                "code": code,
                "message": message,
                "recoverable": recoverable,
            }),
        }
    }
}

/// Convert a snake_case string to PascalCase.
///
/// E.g. "stream_start" -> "StreamStart".
fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Per-session stream state
// ---------------------------------------------------------------------------

/// Broadcast channel capacity for stream events per session.
const STREAM_CHANNEL_CAPACITY: usize = 256;

/// State for a single active session stream.
struct SessionStream {
    /// Broadcast sender — stream loop publishes here; SSE clients subscribe.
    sender: tokio::sync::broadcast::Sender<SessionStreamEvent>,
    /// Set to true when POST /stop is called to cancel the running task.
    cancelled: Arc<AtomicBool>,
    /// Pending tool approval channels: tool_call_id -> oneshot sender.
    pending_approvals: Arc<Mutex<HashMap<String, tokio::sync::oneshot::Sender<bool>>>>,
}

/// Shared registry from session_id to active stream state.
///
/// Stored in `HealthState` so all route handlers can access it.
#[derive(Clone, Default)]
pub struct SessionStreamRegistry(Arc<Mutex<HashMap<i64, Arc<SessionStream>>>>);

impl SessionStreamRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new stream entry for `session_id` and return the Arc handle.
    ///
    /// Replaces any prior stream for this session (closing old SSE connections).
    fn create(&self, session_id: i64) -> Arc<SessionStream> {
        let (sender, _) = tokio::sync::broadcast::channel(STREAM_CHANNEL_CAPACITY);
        let entry = Arc::new(SessionStream {
            sender,
            cancelled: Arc::new(AtomicBool::new(false)),
            pending_approvals: Arc::new(Mutex::new(HashMap::new())),
        });
        if let Ok(mut map) = self.0.lock() {
            map.insert(session_id, Arc::clone(&entry));
        }
        entry
    }

    /// Return the stream entry for `session_id`, if any.
    fn get(&self, session_id: i64) -> Option<Arc<SessionStream>> {
        self.0.lock().ok()?.get(&session_id).cloned()
    }

    /// Remove the stream entry for `session_id`.
    fn remove(&self, session_id: i64) {
        if let Ok(mut map) = self.0.lock() {
            map.remove(&session_id);
        }
    }
}

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /sessions/:id/messages.
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    /// User message text.
    pub content: String,
    /// Optional model override.
    pub model: Option<String>,
}

/// Request body for POST /sessions/:id/tool-approval.
#[derive(Debug, Deserialize)]
pub struct ToolApprovalRequest {
    /// The tool call identifier requiring an approval decision.
    pub tool_call_id: String,
    /// Whether the tool call is approved.
    pub approved: bool,
}

// ---------------------------------------------------------------------------
// Sidecar process helpers
// ---------------------------------------------------------------------------

/// Locate the claude CLI binary on PATH via platform-specific lookup.
fn find_claude_binary() -> Result<String, String> {
    let candidates = if cfg!(windows) {
        vec!["claude.cmd", "claude"]
    } else {
        vec!["claude"]
    };
    for candidate in candidates {
        let found = std::process::Command::new(if cfg!(windows) { "where" } else { "which" })
            .arg(candidate)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if found {
            return Ok(candidate.to_owned());
        }
    }
    Err(
        "claude binary not found — install with: npm install -g @anthropic-ai/claude-code"
            .to_owned(),
    )
}

/// Spawn the claude sidecar for `session_id` rooted at `project_root`.
///
/// Returns `(child, stdin, stdout_reader)` on success.
fn spawn_sidecar(
    session_id: i64,
    project_root: &Path,
) -> Result<
    (
        std::process::Child,
        std::process::ChildStdin,
        BufReader<std::process::ChildStdout>,
    ),
    String,
> {
    let binary = find_claude_binary()?;

    let mut child = std::process::Command::new(&binary)
        .args(["--output-format", "stream-json", "--verbose"])
        .current_dir(project_root)
        .env("ORQA_SESSION_ID", session_id.to_string())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("failed to spawn sidecar '{binary}': {e}"))?;

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| "child stdin unavailable".to_owned())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "child stdout unavailable".to_owned())?;
    Ok((child, stdin, BufReader::new(stdout)))
}

/// Serialise and write a `SidecarRequest` to the sidecar's stdin.
fn send_to_sidecar(
    stdin: &mut std::process::ChildStdin,
    req: &SidecarRequest,
) -> Result<(), String> {
    let json = serde_json::to_string(req).map_err(|e| format!("serialise request: {e}"))?;
    stdin
        .write_all(json.as_bytes())
        .map_err(|e| format!("write to sidecar: {e}"))?;
    stdin
        .write_all(b"\n")
        .map_err(|e| format!("write newline: {e}"))?;
    stdin
        .flush()
        .map_err(|e| format!("flush sidecar stdin: {e}"))?;
    Ok(())
}

/// Read and deserialise the next `SidecarResponse` from the sidecar's stdout.
///
/// Returns `None` on EOF or parse error.
fn read_sidecar_response(
    reader: &mut BufReader<std::process::ChildStdout>,
) -> Option<SidecarResponse> {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => serde_json::from_str(line.trim()).ok(),
    }
}

// ---------------------------------------------------------------------------
// Per-session enforcement and workflow state
// ---------------------------------------------------------------------------

/// Per-session engine state for the stream loop.
///
/// Created fresh for each `send_message` call and dropped when the loop exits.
struct SessionEngineState {
    /// Compiled enforcement engine, loaded from disk. None if no rules exist.
    enforcement: Option<EnforcementEngine>,
    /// Workflow tracker for process compliance.
    tracker: WorkflowTracker,
    /// Process gate definitions. Empty until workflow plugin is loaded.
    gates: Vec<ProcessGateConfig>,
    /// Project root path for tool dispatch and knowledge reads.
    project_root: PathBuf,
    /// Semantic search engine. None until indexed.
    search: Option<SearchEngine>,
}

impl SessionEngineState {
    /// Create a new engine state for `project_root`.
    ///
    /// Loads enforcement rules from disk. Search engine starts as None.
    fn new(project_root: PathBuf) -> Self {
        let enforcement = load_rules(&project_root).ok().map(EnforcementEngine::new);

        Self {
            enforcement,
            tracker: WorkflowTracker::new(TrackerConfig::default()),
            gates: Vec::new(),
            project_root,
            search: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Tool dispatch (daemon-side)
// ---------------------------------------------------------------------------

/// Dispatch a tool call to the appropriate handler and return `(output, is_error)`.
fn dispatch_tool(
    tool_name: &str,
    input: &serde_json::Value,
    state: &mut SessionEngineState,
) -> (String, bool) {
    let root = state.project_root.clone();
    match tool_name {
        "read_file" => tool_read_file(input, &root),
        "write_file" => tool_write_file(input, &root),
        "edit_file" => tool_edit_file(input, &root),
        "bash" => tool_bash(input, &root),
        "glob" => tool_glob(input, &root),
        "grep" => tool_grep(input, &root),
        "load_knowledge" => tool_load_knowledge(input, &root),
        "search_regex" => match &state.search {
            Some(engine) => {
                let query = input["query"].as_str().unwrap_or("");
                match engine.search_regex(query, None, 100) {
                    Ok(results) => (format_search_results(&results), false),
                    Err(e) => (format!("search_regex error: {e}"), true),
                }
            }
            None => ("search engine not available".to_owned(), true),
        },
        "search_semantic" | "code_research" => match &mut state.search {
            Some(engine) => {
                let query = input["query"].as_str().unwrap_or("");
                let limit = input["limit"].as_u64().unwrap_or(10) as u32;
                match engine.search_semantic(query, limit) {
                    Ok(results) => (format_search_results(&results), false),
                    Err(e) => (format!("search error: {e}"), true),
                }
            }
            None => ("search engine not available".to_owned(), true),
        },
        _ => (format!("unknown tool: {tool_name}"), true),
    }
}

/// Record a completed tool call in the workflow tracker and run enforcement checks.
fn track_and_enforce(tool_name: &str, input: &serde_json::Value, state: &mut SessionEngineState) {
    match tool_name {
        "read_file" => {
            if let Some(path) = input["path"].as_str() {
                state.tracker.record_read(path);
            }
        }
        "write_file" | "edit_file" => {
            if let Some(path) = input["path"].as_str() {
                state.tracker.record_write(path);
                let verdicts = evaluate_write_verdicts(&state.gates, &mut state.tracker, path);
                for v in &verdicts {
                    debug!(
                        "[enforcement] gate fired at write: rule='{}' action={:?}",
                        v.rule_name, v.action
                    );
                }
                if let Some(ref engine) = state.enforcement {
                    for v in &engine.evaluate_file(path, "") {
                        debug!(
                            "[enforcement] rule='{}' fired at write: {}",
                            v.rule_name, v.message
                        );
                    }
                }
            }
        }
        "bash" => {
            if let Some(cmd) = input["command"].as_str() {
                state.tracker.record_command(cmd);
                if let Some(ref engine) = state.enforcement {
                    for v in &engine.evaluate_bash(cmd) {
                        debug!(
                            "[enforcement] rule='{}' fired at bash: {}",
                            v.rule_name, v.message
                        );
                    }
                }
            }
        }
        "search_regex" | "search_semantic" | "code_research" => {
            state.tracker.record_search();
        }
        "load_knowledge" => {
            if let Some(name) = input["name"].as_str() {
                state.tracker.record_knowledge_loaded(name);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Stream loop (blocking, runs on a spawn_blocking thread)
// ---------------------------------------------------------------------------

/// Publish a `SessionStreamEvent` to the broadcast channel.
///
/// Ignores send errors (no active SSE subscribers).
fn publish(sender: &tokio::sync::broadcast::Sender<SessionStreamEvent>, event: SessionStreamEvent) {
    let _ = sender.send(event);
}

/// Run the full stream loop for one `send_message` request.
///
/// Spawns the sidecar, sends the message, reads responses, dispatches tools,
/// and publishes events to the broadcast channel. Runs synchronously on a
/// blocking thread so stdin/stdout I/O never blocks the tokio executor.
///
/// Returns the final `StreamAccumulator` for persistence by the caller.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn run_loop_blocking(
    session_id: i64,
    content: String,
    model: Option<String>,
    system_prompt: Option<String>,
    provider_session_id: Option<String>,
    project_root: PathBuf,
    sender: tokio::sync::broadcast::Sender<SessionStreamEvent>,
    cancelled: Arc<AtomicBool>,
    pending_approvals: Arc<Mutex<HashMap<String, tokio::sync::oneshot::Sender<bool>>>>,
) -> StreamAccumulator {
    let mut acc = StreamAccumulator {
        text: String::new(),
        input_tokens: 0,
        output_tokens: 0,
        stream_complete: false,
        had_error: false,
    };

    // Spawn the sidecar subprocess.
    let (mut child, mut stdin, mut reader) = match spawn_sidecar(session_id, &project_root) {
        Ok(r) => r,
        Err(e) => {
            error!("[streaming] session {session_id}: failed to spawn sidecar: {e}");
            publish(
                &sender,
                SessionStreamEvent::error("sidecar_spawn_error", &e, false),
            );
            acc.had_error = true;
            return acc;
        }
    };

    // Send the initial message request to the sidecar.
    let request = SidecarRequest::SendMessage {
        session_id,
        content,
        model,
        system_prompt,
        provider_session_id,
        enable_thinking: false,
    };
    if let Err(e) = send_to_sidecar(&mut stdin, &request) {
        error!("[streaming] session {session_id}: failed to send to sidecar: {e}");
        publish(
            &sender,
            SessionStreamEvent::error("sidecar_send_error", &e, false),
        );
        let _ = child.kill();
        acc.had_error = true;
        return acc;
    }

    let mut engine_state = SessionEngineState::new(project_root);

    // Sidecar read loop.
    loop {
        // Honour cancellation: send CancelStream then drain until terminal.
        if cancelled.load(Ordering::Relaxed) {
            info!("[streaming] session {session_id}: stream cancelled by client");
            let _ = send_to_sidecar(&mut stdin, &SidecarRequest::CancelStream { session_id });
        }

        let Some(response) = read_sidecar_response(&mut reader) else {
            if !cancelled.load(Ordering::Relaxed) {
                publish(
                    &sender,
                    SessionStreamEvent::error(
                        "sidecar_eof",
                        "sidecar process closed unexpectedly",
                        false,
                    ),
                );
                acc.had_error = true;
            }
            break;
        };

        // ToolExecute — dispatch synchronously and return result to sidecar.
        if let SidecarResponse::ToolExecute {
            ref tool_call_id,
            ref tool_name,
            ref input,
        } = response
        {
            debug!(
                "[streaming] session {session_id}: ToolExecute id={tool_call_id} tool={tool_name}"
            );
            let input_val: serde_json::Value = serde_json::from_str(input).unwrap_or_default();
            track_and_enforce(tool_name, &input_val, &mut engine_state);
            let (raw_output, is_error) = dispatch_tool(tool_name, &input_val, &mut engine_state);
            let output = truncate_tool_output(raw_output);
            let result_req = SidecarRequest::ToolResult {
                tool_call_id: tool_call_id.clone(),
                output,
                is_error,
            };
            if let Err(e) = send_to_sidecar(&mut stdin, &result_req) {
                publish(
                    &sender,
                    SessionStreamEvent::error("tool_result_send_error", &e, false),
                );
                acc.had_error = true;
                break;
            }
            continue;
        }

        // ToolApprovalRequest — auto-approve read-only tools; block on user decision otherwise.
        if let SidecarResponse::ToolApprovalRequest {
            ref tool_call_id,
            ref tool_name,
            ref input,
        } = response
        {
            debug!("[streaming] session {session_id}: ToolApprovalRequest id={tool_call_id} tool={tool_name}");

            let approved = if READ_ONLY_TOOLS.contains(&tool_name.as_str()) {
                debug!("[streaming] auto-approving read-only tool: {tool_name}");
                true
            } else {
                // Register a oneshot sender and notify the SSE consumer.
                let (tx, rx) = tokio::sync::oneshot::channel::<bool>();
                {
                    let mut map = pending_approvals
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    map.insert(tool_call_id.clone(), tx);
                }
                publish(
                    &sender,
                    SessionStreamEvent {
                        event_type: "ToolApprovalRequest".to_owned(),
                        payload: serde_json::json!({
                            "tool_call_id": tool_call_id,
                            "tool_name": tool_name,
                            "input": input,
                        }),
                    },
                );
                // Block until POST /tool-approval delivers the decision.
                rx.blocking_recv().unwrap_or(false)
            };

            let approval_req = SidecarRequest::ToolApproval {
                tool_call_id: tool_call_id.clone(),
                approved,
                reason: if approved {
                    None
                } else {
                    Some("denied by user".to_owned())
                },
            };
            if let Err(e) = send_to_sidecar(&mut stdin, &approval_req) {
                publish(
                    &sender,
                    SessionStreamEvent::error("tool_approval_send_error", &e, false),
                );
                acc.had_error = true;
                break;
            }
            continue;
        }

        // SessionInitialized — log the provider session ID (persisted by caller after loop).
        if let SidecarResponse::SessionInitialized {
            session_id: _,
            ref provider_session_id,
        } = response
        {
            debug!("[streaming] session {session_id}: provider_session_id={provider_session_id}");
        }

        // Accumulate state for persistence.
        accumulate_response(&response, &mut acc);

        // Evaluate stop gates at turn end.
        if matches!(response, SidecarResponse::TurnComplete { .. }) {
            let verdicts = evaluate_stop_verdicts(&engine_state.gates, &engine_state.tracker);
            for v in &verdicts {
                debug!(
                    "[enforcement] gate fired at stop: rule='{}' action={:?}",
                    v.rule_name, v.action
                );
            }
        }

        // Translate and publish the event to SSE consumers.
        if let Some(stream_event) = translate_response(&response) {
            if let Some(sse_event) = SessionStreamEvent::from_stream_event(stream_event) {
                publish(&sender, sse_event);
            }
        }

        if is_terminal(&response) {
            break;
        }
    }

    // Always terminate the sidecar child process.
    let _ = child.kill();
    let _ = child.wait();

    acc
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /sessions/:id/messages — accept a user message and start streaming.
///
/// Persists the user message, spawns a blocking task running the sidecar stream
/// loop, and returns immediately with the user message ID. The app subscribes to
/// GET /sessions/:id/stream to receive events as they are produced.
#[allow(clippy::too_many_lines)]
pub async fn send_message(
    State(state): State<HealthState>,
    AxumPath(session_id): AxumPath<i64>,
    Json(req): Json<SendMessageRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let content = req.content.trim().to_owned();
    if content.is_empty() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(
                serde_json::json!({ "error": "message content cannot be empty", "code": "EMPTY_CONTENT" }),
            ),
        ));
    }

    let storage = state.storage.clone().ok_or_else(|| (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({ "error": "session store unavailable", "code": "STORE_UNAVAILABLE" })),
    ))?;

    // Resolve project root from the cached graph.
    let project_root = state
        .graph_state
        .0
        .read()
        .ok()
        .map(|g| g.project_root.clone())
        .ok_or_else(|| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "project root unavailable", "code": "NO_PROJECT" })),
        ))?;

    // Persist the user message and get its ID and the current provider session ID.
    let session = storage
        .sessions()
        .get(session_id)
        .await
        .map_err(|e| (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("session not found: {e}"), "code": "SESSION_NOT_FOUND" })),
        ))?;
    let provider_session_id = session.provider_session_id;
    let turn_index = storage
        .messages()
        .next_turn_index(session_id)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("next turn index: {e}"), "code": "DB_ERROR" })),
        ))?;
    let user_msg = storage
        .messages()
        .create(session_id, MessageRole::User, Some(&content), turn_index, 0)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("create user message: {e}"), "code": "DB_ERROR" })),
        ))?;
    let user_message_id = user_msg.id;

    // Resolve the system prompt from disk.
    let system_prompt = resolve_system_prompt(&project_root);

    // Create a fresh session stream entry (replaces any prior stream for this session).
    let stream_entry = state.stream_registry.create(session_id);
    let sender = stream_entry.sender.clone();
    let cancelled = Arc::clone(&stream_entry.cancelled);
    let pending_approvals = Arc::clone(&stream_entry.pending_approvals);
    let registry = state.stream_registry.clone();
    let storage_for_loop = Arc::clone(&storage);

    // Spawn the blocking stream loop on a dedicated thread.
    // stdin/stdout I/O must not run on the tokio executor.
    tokio::task::spawn_blocking(move || {
        info!("[streaming] session {session_id}: stream loop starting");
        let acc = run_loop_blocking(
            session_id,
            content,
            req.model,
            system_prompt,
            provider_session_id,
            project_root,
            sender.clone(),
            cancelled,
            pending_approvals,
        );
        info!(
            "[streaming] session {session_id}: stream loop finished — \
             complete={} error={} input_tokens={} output_tokens={}",
            acc.stream_complete, acc.had_error, acc.input_tokens, acc.output_tokens,
        );

        // Persist the assistant message using block_on since we are inside
        // a spawn_blocking thread and cannot use .await directly.
        {
            let handle = tokio::runtime::Handle::current();
            let turn_index = handle
                .block_on(storage_for_loop.messages().next_turn_index(session_id))
                .unwrap_or(1);
            let content_val = if acc.text.is_empty() {
                None
            } else {
                Some(acc.text.as_str())
            };
            if let Ok(assistant_msg) = handle.block_on(storage_for_loop.messages().create(
                session_id,
                MessageRole::Assistant,
                content_val,
                turn_index,
                0,
            )) {
                let status = if acc.stream_complete && !acc.had_error {
                    StreamStatus::Complete
                } else {
                    StreamStatus::Error
                };
                let _ = handle.block_on(
                    storage_for_loop
                        .messages()
                        .update_stream_status(assistant_msg.id, status),
                );
            }
            if acc.stream_complete {
                let _ = handle.block_on(storage_for_loop.sessions().update_token_usage(
                    session_id,
                    acc.input_tokens,
                    acc.output_tokens,
                ));
            }
        }

        // Publish a synthetic StreamEnd so SSE consumers know the loop finished.
        let _ = sender.send(SessionStreamEvent {
            event_type: "StreamEnd".to_owned(),
            payload: serde_json::json!({
                "stream_complete": acc.stream_complete,
                "had_error": acc.had_error,
            }),
        });

        // Remove the stream registry entry once the loop is done.
        registry.remove(session_id);
    });

    Ok(Json(serde_json::json!({
        "user_message_id": user_message_id,
        "session_id": session_id,
        "status": "streaming",
    })))
}

/// Handle GET /sessions/:id/stream — subscribe to the SSE event stream for a session.
///
/// SSE clients connect immediately after POST /messages. Events are serialised
/// as `data: <json>\n\n` per the SSE spec. The stream ends when the loop task
/// drops the broadcast sender (registry entry removed).
pub async fn session_stream(
    State(state): State<HealthState>,
    AxumPath(session_id): AxumPath<i64>,
) -> Result<
    Sse<impl futures_util::Stream<Item = Result<Event, axum::Error>>>,
    (StatusCode, Json<serde_json::Value>),
> {
    let entry = state.stream_registry.get(session_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("no active stream for session {session_id}"),
                "code": "NO_STREAM",
            })),
        )
    })?;

    let receiver = entry.sender.subscribe();
    let stream = BroadcastStream::new(receiver).filter_map(|result| match result {
        Ok(event) => match serde_json::to_string(&event) {
            Ok(json) => Some(Ok(Event::default().data(json))),
            Err(e) => {
                error!("[streaming] SSE: failed to serialise event: {e}");
                None
            }
        },
        Err(e) => {
            error!("[streaming] SSE: broadcast error: {e}");
            None
        }
    });

    Ok(Sse::new(stream))
}

/// Handle POST /sessions/:id/stop — cancel an active stream.
///
/// Sets the cancellation flag. The blocking loop checks this flag before each
/// sidecar read and sends a CancelStream request to the sidecar.
pub async fn stop_stream(
    State(state): State<HealthState>,
    AxumPath(session_id): AxumPath<i64>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let entry = state.stream_registry.get(session_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("no active stream for session {session_id}"),
                "code": "NO_STREAM",
            })),
        )
    })?;
    entry.cancelled.store(true, Ordering::Relaxed);
    info!("[streaming] session {session_id}: stop requested");
    Ok(StatusCode::NO_CONTENT)
}

/// Handle POST /sessions/:id/tool-approval — deliver a tool approval decision.
///
/// The stream loop blocks on a `oneshot::Receiver<bool>` after emitting a
/// `ToolApprovalRequest` SSE event. This handler looks up the sender by
/// `tool_call_id` and signals the decision to unblock the loop.
pub async fn tool_approval(
    State(state): State<HealthState>,
    AxumPath(session_id): AxumPath<i64>,
    Json(req): Json<ToolApprovalRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let entry = state.stream_registry.get(session_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("no active stream for session {session_id}"),
                "code": "NO_STREAM",
            })),
        )
    })?;

    let sender = {
        let mut map = entry.pending_approvals
            .lock()
            .map_err(|_| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "pending_approvals mutex poisoned", "code": "MUTEX_POISON" })),
            ))?;
        map.remove(&req.tool_call_id)
    };

    match sender {
        Some(tx) => {
            let _ = tx.send(req.approved);
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("no pending approval for tool_call_id={}", req.tool_call_id),
                "code": "NOT_FOUND",
            })),
        )),
    }
}
