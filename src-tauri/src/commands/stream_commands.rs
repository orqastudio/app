use crate::domain::enforcement::RuleAction;
use crate::domain::provider_event::StreamEvent;
use crate::error::OrqaError;
use crate::repo::{message_repo, project_repo, session_repo};
use crate::sidecar::types::{SidecarRequest, SidecarResponse};
use crate::state::AppState;

use std::path::{Path, PathBuf};
use std::sync::mpsc;

/// Maximum number of characters to return from a single tool output.
///
/// Outputs exceeding this limit are truncated with an explanatory message.
/// This prevents context window overflow when tools return very large results.
const MAX_TOOL_OUTPUT_CHARS: usize = 100_000;

/// Default maximum number of lines returned by `read_file`.
const DEFAULT_READ_FILE_MAX_LINES: usize = 2000;

/// Tool names that are read-only and can be auto-approved without user interaction.
const READ_ONLY_TOOLS: &[&str] = &[
    "read_file",
    "glob",
    "grep",
    "search_regex",
    "search_semantic",
    "load_skill",
    "code_research",
];

/// Truncate a tool output string to `MAX_TOOL_OUTPUT_CHARS` characters.
///
/// When the output exceeds the limit, the returned string contains the first
/// `MAX_TOOL_OUTPUT_CHARS` characters followed by a clear truncation notice.
/// This prevents context window overflow when tools return very large results.
fn truncate_tool_output(output: String) -> String {
    if output.len() <= MAX_TOOL_OUTPUT_CHARS {
        return output;
    }
    let truncated = &output[..MAX_TOOL_OUTPUT_CHARS];
    format!(
        "{truncated}\n\n[Output truncated: {} chars total, showing first {MAX_TOOL_OUTPUT_CHARS}]",
        output.len()
    )
}

/// Translate a context-overflow error code into a user-friendly message.
///
/// Returns `Some(friendly_message)` when the code indicates a context/token
/// limit error that should be surfaced with a clear explanation.
fn friendly_context_overflow_message(code: &str, message: &str) -> Option<String> {
    let lower_code = code.to_lowercase();
    let lower_msg = message.to_lowercase();
    let is_overflow = lower_code.contains("context")
        || lower_code.contains("token")
        || lower_msg.contains("context window")
        || lower_msg.contains("token limit")
        || lower_msg.contains("too long")
        || lower_msg.contains("max_tokens");
    if is_overflow {
        Some(
            "The conversation has exceeded the model's context window. \
             Start a new session to continue, or summarize earlier context before proceeding."
                .to_string(),
        )
    } else {
        None
    }
}

/// Translate content and turn lifecycle responses into `StreamEvent`s.
///
/// Returns `None` for responses not part of content/turn flow.
fn translate_content_event(response: &SidecarResponse) -> Option<StreamEvent> {
    match response {
        SidecarResponse::StreamStart {
            message_id,
            resolved_model,
        } => Some(StreamEvent::StreamStart {
            message_id: *message_id,
            resolved_model: resolved_model.clone(),
        }),
        SidecarResponse::TextDelta { content } => Some(StreamEvent::TextDelta {
            content: content.clone(),
        }),
        SidecarResponse::ThinkingDelta { content } => Some(StreamEvent::ThinkingDelta {
            content: content.clone(),
        }),
        SidecarResponse::BlockComplete {
            block_index,
            content_type,
        } => Some(StreamEvent::BlockComplete {
            block_index: *block_index,
            content_type: content_type.clone(),
        }),
        SidecarResponse::TurnComplete {
            input_tokens,
            output_tokens,
        } => Some(StreamEvent::TurnComplete {
            input_tokens: *input_tokens,
            output_tokens: *output_tokens,
        }),
        SidecarResponse::StreamCancelled => Some(StreamEvent::StreamCancelled),
        SidecarResponse::StreamError {
            code,
            message,
            recoverable,
        } => {
            let user_message =
                friendly_context_overflow_message(code, message).unwrap_or_else(|| message.clone());
            Some(StreamEvent::StreamError {
                code: code.clone(),
                message: user_message,
                recoverable: *recoverable,
            })
        }
        _ => None,
    }
}

/// Translate tool-related responses into `StreamEvent`s.
///
/// Returns `None` for non-tool responses (handled by `translate_content_event`
/// or suppressed entirely).
fn translate_tool_event(response: &SidecarResponse) -> Option<StreamEvent> {
    match response {
        SidecarResponse::ToolUseStart {
            tool_call_id,
            tool_name,
        } => Some(StreamEvent::ToolUseStart {
            tool_call_id: tool_call_id.clone(),
            tool_name: tool_name.clone(),
        }),
        SidecarResponse::ToolInputDelta {
            tool_call_id,
            content,
        } => Some(StreamEvent::ToolInputDelta {
            tool_call_id: tool_call_id.clone(),
            content: content.clone(),
        }),
        SidecarResponse::ToolResult {
            tool_call_id,
            tool_name,
            result,
            is_error,
        } => Some(StreamEvent::ToolResult {
            tool_call_id: tool_call_id.clone(),
            tool_name: tool_name.clone(),
            result: result.clone(),
            is_error: *is_error,
        }),
        SidecarResponse::ToolApprovalRequest {
            tool_call_id,
            tool_name,
            input,
        } => Some(StreamEvent::ToolApprovalRequest {
            tool_call_id: tool_call_id.clone(),
            tool_name: tool_name.clone(),
            input: input.clone(),
        }),
        _ => None,
    }
}

/// Translate a `SidecarResponse` into a `StreamEvent`, if applicable.
///
/// Returns `None` for sidecar-specific responses (HealthOk, SummaryResult,
/// SessionInitialized, ToolExecute) that are not forwarded to the frontend.
fn translate_response(response: &SidecarResponse) -> Option<StreamEvent> {
    translate_content_event(response).or_else(|| translate_tool_event(response))
}

/// Returns true if this response is a terminal event (stream complete, error, or cancelled).
fn is_terminal(response: &SidecarResponse) -> bool {
    matches!(
        response,
        SidecarResponse::TurnComplete { .. }
            | SidecarResponse::StreamError { .. }
            | SidecarResponse::StreamCancelled
    )
}

/// Persist the user message and return `(user_message_id, turn_index)`.
fn persist_user_message(
    state: &AppState,
    session_id: i64,
    content: &str,
) -> Result<(i64, i64), OrqaError> {
    let db = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;

    session_repo::get(&db, session_id)?;
    let turn_index = message_repo::next_turn_index(&db, session_id)?;
    let user_msg = message_repo::create(
        &db,
        session_id,
        "user",
        "text",
        Some(content),
        turn_index,
        0,
    )?;

    Ok((user_msg.id, i64::from(turn_index)))
}

/// Accumulated state from the sidecar read loop.
struct StreamAccumulator {
    text: String,
    input_tokens: i64,
    output_tokens: i64,
    stream_complete: bool,
    had_error: bool,
}

/// Handle a `ToolExecute` response: execute the tool and send the result back to the sidecar.
///
/// Returns `true` to continue the loop, `false` on send failure.
fn handle_tool_execute(
    tool_call_id: &str,
    tool_name: &str,
    input: &str,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> bool {
    tracing::debug!("[stream] received ToolExecute: id={tool_call_id} tool={tool_name}");
    let (raw_output, is_error) = execute_tool(tool_name, input, state);

    // Track completed tool call for process compliance checks.
    track_process_state(tool_name, input, state);

    let output = truncate_tool_output(raw_output);
    let tool_result = SidecarRequest::ToolResult {
        tool_call_id: tool_call_id.to_string(),
        output,
        is_error,
    };
    if let Err(e) = state.sidecar.send(&tool_result) {
        let _ = on_event.send(StreamEvent::StreamError {
            code: "tool_result_send_error".to_string(),
            message: format!("failed to send tool result to sidecar: {e}"),
            recoverable: false,
        });
        return false;
    }
    true
}

/// Send a tool approval decision to the sidecar.
///
/// Returns `true` to continue the loop, `false` on send failure.
fn send_approval(
    tool_call_id: &str,
    approved: bool,
    reason: Option<String>,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> bool {
    let approval = SidecarRequest::ToolApproval {
        tool_call_id: tool_call_id.to_string(),
        approved,
        reason,
    };
    if let Err(e) = state.sidecar.send(&approval) {
        let _ = on_event.send(StreamEvent::StreamError {
            code: "tool_approval_send_error".to_string(),
            message: format!("failed to send tool approval to sidecar: {e}"),
            recoverable: false,
        });
        return false;
    }
    true
}

/// Register an approval sender in `pending_approvals` and return the receiver.
///
/// Returns `None` if the mutex is poisoned, in which case the caller should deny.
fn register_approval_sender(
    tool_call_id: &str,
    state: &tauri::State<'_, AppState>,
) -> Option<mpsc::Receiver<bool>> {
    let (tx, rx) = mpsc::sync_channel::<bool>(1);
    let Ok(mut map) = state.pending_approvals.lock() else {
        tracing::error!("[stream] pending_approvals mutex poisoned");
        return None;
    };
    map.insert(tool_call_id.to_string(), tx);
    Some(rx)
}

/// Handle a `ToolApprovalRequest`.
///
/// Read-only tools (listed in `READ_ONLY_TOOLS`) are auto-approved immediately.
/// Write/execute tools emit a `StreamEvent::ToolApprovalRequest` to the frontend
/// and block on a sync channel until `stream_tool_approval_respond` is called.
///
/// Returns `true` to continue the loop, `false` on failure.
/// Emit an approval request event to the frontend and wait for the user's decision.
///
/// Returns `Some(approved)` on success, or `None` if the event could not be sent.
/// On failure, removes the pending approval entry to avoid a dangling sender.
fn emit_and_await_approval(
    tool_call_id: &str,
    tool_name: &str,
    input: &str,
    rx: mpsc::Receiver<bool>,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> Option<bool> {
    let emit_result = on_event.send(StreamEvent::ToolApprovalRequest {
        tool_call_id: tool_call_id.to_string(),
        tool_name: tool_name.to_string(),
        input: input.to_string(),
    });
    if emit_result.is_err() {
        tracing::warn!("[stream] failed to emit ToolApprovalRequest to frontend");
        if let Ok(mut map) = state.pending_approvals.lock() {
            map.remove(tool_call_id);
        }
        return None;
    }
    let approved = rx.recv().unwrap_or(false);
    tracing::debug!("[stream] received user decision for {tool_call_id}: approved={approved}");
    Some(approved)
}

/// Handle a `ToolApprovalRequest`.
///
/// Read-only tools (listed in `READ_ONLY_TOOLS`) are auto-approved immediately.
/// Write/execute tools emit a `StreamEvent::ToolApprovalRequest` to the frontend
/// and block on a sync channel until `stream_tool_approval_respond` is called.
///
/// Returns `true` to continue the loop, `false` on failure.
fn handle_tool_approval(
    tool_call_id: &str,
    tool_name: &str,
    input: &str,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> bool {
    tracing::debug!("[stream] ToolApprovalRequest: id={tool_call_id} tool={tool_name}");

    if READ_ONLY_TOOLS.contains(&tool_name) {
        tracing::debug!("[stream] auto-approving read-only tool: {tool_name}");
        return send_approval(tool_call_id, true, None, state, on_event);
    }

    tracing::debug!("[stream] requesting user approval for: {tool_name}");
    let rx = match register_approval_sender(tool_call_id, state) {
        Some(rx) => rx,
        None => {
            return send_approval(
                tool_call_id,
                false,
                Some("internal error".to_string()),
                state,
                on_event,
            )
        }
    };

    match emit_and_await_approval(tool_call_id, tool_name, input, rx, state, on_event) {
        Some(approved) => {
            let reason = if approved {
                None
            } else {
                Some("denied by user".to_string())
            };
            send_approval(tool_call_id, approved, reason, state, on_event)
        }
        None => send_approval(
            tool_call_id,
            false,
            Some("frontend not reachable".to_string()),
            state,
            on_event,
        ),
    }
}

/// Persist the provider session UUID when a `SessionInitialized` response is received.
fn handle_session_initialized(
    session_id: i64,
    provider_session_id: &str,
    state: &tauri::State<'_, AppState>,
) {
    if let Ok(db) = state.db.lock() {
        if let Err(e) =
            session_repo::update_provider_session_id(&db, session_id, provider_session_id)
        {
            tracing::warn!("[stream] failed to persist provider_session_id: {e}");
        }
    }
}

/// Read one line from the sidecar, emitting an error event and returning `None` on failure.
fn read_sidecar_line(
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> Option<SidecarResponse> {
    match state.sidecar.read_line() {
        Ok(Some(resp)) => Some(resp),
        Ok(None) => {
            let _ = on_event.send(StreamEvent::StreamError {
                code: "sidecar_eof".to_string(),
                message: "sidecar process closed unexpectedly".to_string(),
                recoverable: false,
            });
            None
        }
        Err(e) => {
            let _ = on_event.send(StreamEvent::StreamError {
                code: "sidecar_read_error".to_string(),
                message: e.to_string(),
                recoverable: false,
            });
            None
        }
    }
}

/// Dispatch one sidecar response to its handler.
///
/// Returns `Ok(true)` to continue the loop, `Ok(false)` to break (terminal event),
/// and `Err(())` when a handler signals a fatal error.
fn dispatch_response(
    response: SidecarResponse,
    acc: &mut StreamAccumulator,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> Result<bool, ()> {
    if let SidecarResponse::SessionInitialized {
        session_id,
        ref provider_session_id,
    } = response
    {
        handle_session_initialized(session_id, provider_session_id, state);
        return Ok(true);
    }
    if let SidecarResponse::ToolExecute {
        ref tool_call_id,
        ref tool_name,
        ref input,
    } = response
    {
        if !handle_tool_execute(tool_call_id, tool_name, input, state, on_event) {
            return Err(());
        }
        return Ok(true);
    }
    if let SidecarResponse::ToolApprovalRequest {
        ref tool_call_id,
        ref tool_name,
        ref input,
    } = response
    {
        if !handle_tool_approval(tool_call_id, tool_name, input, state, on_event) {
            return Err(());
        }
        return Ok(true);
    }
    accumulate_response(&response, acc);
    let terminal = is_terminal(&response);
    if let Some(event) = translate_response(&response) {
        let _ = on_event.send(event);
    }
    Ok(!terminal)
}

/// Run the sidecar read loop, accumulating results into a `StreamAccumulator`.
fn run_stream_loop(
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> StreamAccumulator {
    let mut acc = StreamAccumulator {
        text: String::new(),
        input_tokens: 0,
        output_tokens: 0,
        stream_complete: false,
        had_error: false,
    };

    loop {
        let response = match read_sidecar_line(state, on_event) {
            Some(r) => r,
            None => {
                acc.had_error = true;
                break;
            }
        };

        match dispatch_response(response, &mut acc, state, on_event) {
            Ok(true) => continue,
            Ok(false) => break,
            Err(()) => {
                acc.had_error = true;
                break;
            }
        }
    }

    acc
}

/// Update the accumulator with data from a streaming response.
fn accumulate_response(response: &SidecarResponse, acc: &mut StreamAccumulator) {
    if let SidecarResponse::TextDelta { ref content } = response {
        acc.text.push_str(content);
    }
    if let SidecarResponse::TurnComplete {
        input_tokens,
        output_tokens,
    } = response
    {
        acc.input_tokens = *input_tokens;
        acc.output_tokens = *output_tokens;
        acc.stream_complete = true;
    }
    if matches!(
        response,
        SidecarResponse::StreamError { .. } | SidecarResponse::StreamCancelled
    ) {
        acc.had_error = true;
    }
}

/// Persist the assistant message and update session token usage.
fn persist_assistant_message(
    state: &AppState,
    session_id: i64,
    turn_index: i64,
    acc: &StreamAccumulator,
) -> Result<(), OrqaError> {
    let db = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;

    let assistant_turn = i32::try_from(turn_index + 1)
        .map_err(|_| OrqaError::Database("turn index overflow".to_string()))?;
    let content_value = if acc.text.is_empty() {
        None
    } else {
        Some(acc.text.as_str())
    };

    let assistant_msg = message_repo::create(
        &db,
        session_id,
        "assistant",
        "text",
        content_value,
        assistant_turn,
        0,
    )?;

    let status = if acc.stream_complete && !acc.had_error {
        "complete"
    } else {
        "error"
    };
    message_repo::update_stream_status(&db, assistant_msg.id, status)?;

    if acc.stream_complete {
        session_repo::update_token_usage(&db, session_id, acc.input_tokens, acc.output_tokens)?;
    }

    Ok(())
}

/// Reset the session process state when a new session begins.
///
/// If the stored session id differs from `session_id`, all process compliance
/// flags are cleared and the session id is updated. This ensures violations
/// from a previous conversation do not carry over into a new one.
fn reset_process_state_if_new_session(state: &tauri::State<'_, AppState>, session_id: i64) {
    match state.process_state.lock() {
        Ok(mut ps) => {
            if ps.session_id != Some(session_id) {
                ps.reset(session_id);
                // Also reset the workflow tracker for the new session.
                reset_workflow_tracker(state);
            }
        }
        Err(e) => {
            tracing::warn!("[process] process_state mutex poisoned, skipping reset: {e}");
        }
    }
}

/// Reset the workflow tracker to a clean state for a new session.
fn reset_workflow_tracker(state: &tauri::State<'_, AppState>) {
    use crate::domain::workflow_tracker::WorkflowTracker;
    match state.workflow_tracker.lock() {
        Ok(mut wt) => {
            *wt = WorkflowTracker::new();
        }
        Err(e) => {
            tracing::warn!("[workflow] workflow_tracker mutex poisoned, skipping reset: {e}");
        }
    }
}

/// Record a completed tool call in the session process state.
///
/// Parses `input_json` into a `serde_json::Value`; silently skips tracking
/// if the JSON is malformed (the tool execution result takes precedence).
fn track_process_state(tool_name: &str, input_json: &str, state: &tauri::State<'_, AppState>) {
    let input: serde_json::Value = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(_) => return,
    };
    match state.process_state.lock() {
        Ok(mut ps) => ps.track_tool_call(tool_name, &input),
        Err(e) => {
            tracing::warn!("[process] process_state mutex poisoned, skipping track: {e}");
        }
    }
}

/// Emit `StreamEvent::ProcessViolation` for any active process compliance violations.
///
/// Called after every turn completes. Violations are warnings only and do not
/// interrupt the stream or prevent persistence.
fn emit_process_violations(
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) {
    let violations = match state.process_state.lock() {
        Ok(ps) => ps.check_violations(),
        Err(e) => {
            tracing::warn!("[process] process_state mutex poisoned, skipping violation check: {e}");
            return;
        }
    };
    for v in violations {
        tracing::debug!(
            "[process] violation: check={} severity={}",
            v.check,
            v.severity
        );
        let _ = on_event.send(StreamEvent::ProcessViolation {
            check: v.check,
            message: v.message,
        });
    }
}

/// Read a governance file from the project directory, returning its contents.
/// Returns `None` if the file does not exist, `Err` on read errors.
fn read_governance_file(project_path: &Path, relative: &str) -> Result<Option<String>, OrqaError> {
    let full_path = project_path.join(relative);
    if !full_path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&full_path)?;
    Ok(Some(contents))
}

/// List skill names with one-line descriptions from `.orqa/skills/*/SKILL.md`.
///
/// Reads only the first non-empty line of each SKILL.md as the description.
/// Full skill content is intentionally NOT loaded here — skills are loaded
/// on demand via the `load_skill` tool.
fn list_skill_catalog(project_path: &Path) -> Vec<(String, String)> {
    let skills_dir = project_path.join(".orqa").join("skills");
    let mut catalog = Vec::new();

    let read_dir = match std::fs::read_dir(&skills_dir) {
        Ok(rd) => rd,
        Err(_) => return catalog,
    };

    for entry in read_dir.flatten() {
        let skill_md = entry.path().join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }

        let skill_name = entry.file_name().to_string_lossy().to_string();
        let description = std::fs::read_to_string(&skill_md)
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|l| !l.trim().is_empty())
                    .map(|l| l.trim_start_matches('#').trim().to_string())
            })
            .unwrap_or_else(|| "No description".to_string());

        catalog.push((skill_name, description));
    }

    catalog.sort_by(|a, b| a.0.cmp(&b.0));
    catalog
}

/// Read all rule files from `.orqa/rules/*.md`.
fn read_rules(project_path: &Path) -> Vec<(String, String)> {
    let rules_dir = project_path.join(".orqa").join("rules");
    let mut rules = Vec::new();

    let read_dir = match std::fs::read_dir(&rules_dir) {
        Ok(rd) => rd,
        Err(_) => return rules,
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let rule_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        if let Ok(contents) = std::fs::read_to_string(&path) {
            rules.push((rule_name, contents));
        }
    }

    rules.sort_by(|a, b| a.0.cmp(&b.0));
    rules
}

/// Build a structured system prompt from the project's governance artifacts.
///
/// Reads:
/// - `.orqa/rules/*.md` — rule files (full content)
/// - `.claude/CLAUDE.md` — project instructions (full content, platform config)
/// - `AGENTS.md` — agent definitions (full content)
/// - `.orqa/skills/*/SKILL.md` — skill catalog (name + one-line description only)
///
/// Returns `Ok(None)` when the project path cannot be resolved (no active project).
fn build_system_prompt(project_path: &Path) -> Result<String, OrqaError> {
    let mut parts: Vec<String> = Vec::new();
    parts.push("# Project Governance".to_string());

    let rules = read_rules(project_path);
    if !rules.is_empty() {
        parts.push("\n## Rules".to_string());
        for (name, content) in &rules {
            parts.push(format!("\n### {name}\n\n{content}"));
        }
    }

    let catalog = list_skill_catalog(project_path);
    if !catalog.is_empty() {
        parts.push("\n## Available Skills".to_string());
        parts.push(
            "Use the `load_skill` tool to load the full content of any skill by name.".to_string(),
        );
        for (name, description) in &catalog {
            parts.push(format!("- **{name}**: {description}"));
        }
    }

    if let Some(claude_md) = read_governance_file(project_path, ".claude/CLAUDE.md")? {
        parts.push("\n## Project Instructions".to_string());
        parts.push(claude_md);
    }

    if let Some(agents_md) = read_governance_file(project_path, "AGENTS.md")? {
        parts.push("\n## Agent Definitions".to_string());
        parts.push(agents_md);
    }

    Ok(parts.join("\n"))
}

/// Load a summary of prior text messages in a session for context injection.
///
/// Returns `(message_count, total_chars, messages_json)` where:
/// - `message_count` is the number of text messages found (excluding `exclude_id`)
/// - `total_chars` is the sum of content lengths (not the JSON string length)
/// - `messages_json` is a JSON array of `{"role": "...", "content": "..."}` objects
///
/// Only `ContentType::Text` messages are included; tool calls, thinking, and images
/// are excluded. The message with `exclude_id` is skipped so the just-persisted
/// user message is not counted as prior context.
fn load_context_summary(
    state: &AppState,
    session_id: i64,
    exclude_id: i64,
) -> Result<(i32, i64, String), OrqaError> {
    use crate::domain::message::ContentType;

    let db = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;

    let messages = message_repo::list(&db, session_id, 1000, 0)?;

    let mut message_count: i32 = 0;
    let mut total_chars: i64 = 0;
    let mut entries: Vec<serde_json::Value> = Vec::new();

    for msg in messages {
        if msg.id == exclude_id {
            continue;
        }
        if msg.content_type != ContentType::Text {
            continue;
        }
        if let Some(ref content) = msg.content {
            let role_str = match msg.role {
                crate::domain::message::MessageRole::User => "user",
                crate::domain::message::MessageRole::Assistant => "assistant",
                crate::domain::message::MessageRole::System => "system",
            };
            total_chars += content.len() as i64;
            message_count += 1;
            entries.push(serde_json::json!({
                "role": role_str,
                "content": content,
            }));
        }
    }

    let messages_json = serde_json::to_string(&entries).map_err(|e| {
        OrqaError::Serialization(format!("failed to serialize context messages: {e}"))
    })?;

    Ok((message_count, total_chars, messages_json))
}

/// Look up the persisted provider session UUID for the given session, used to resume across restarts.
fn lookup_provider_session_id(
    state: &AppState,
    session_id: i64,
) -> Result<Option<String>, OrqaError> {
    let db = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;
    Ok(session_repo::get(&db, session_id)
        .ok()
        .and_then(|s| s.provider_session_id))
}

/// Resolve the optional system prompt for the current project, logging but not failing on errors.
fn resolve_system_prompt(state: &tauri::State<'_, AppState>) -> Option<String> {
    match project_root(state) {
        Ok(root) => match build_system_prompt(&root) {
            Ok(prompt) => {
                tracing::debug!("[stream] system prompt built ({} chars)", prompt.len());
                Some(prompt)
            }
            Err(e) => {
                tracing::warn!("[stream] failed to build system prompt: {e}");
                None
            }
        },
        Err(e) => {
            tracing::warn!("[stream] no active project for system prompt: {e}");
            None
        }
    }
}

/// Send a message to the sidecar and stream responses back via `Channel<T>`.
///
/// This command:
/// 1. Validates the input content is not empty
/// 2. Persists the user message to SQLite
/// 3. Builds a system prompt from the project's governance artifacts
/// 4. Sends the message to the sidecar via NDJSON stdin
/// 5. Reads sidecar responses in a blocking loop, forwarding each as a `StreamEvent`
/// 6. Accumulates text content for the assistant message
/// 7. On completion, persists the assistant message and updates session token usage
///
/// The DB mutex is only held briefly for persistence operations, never during
/// the sidecar read loop.
#[tauri::command]
pub fn stream_send_message(
    session_id: i64,
    content: String,
    model: Option<String>,
    on_event: tauri::ipc::Channel<StreamEvent>,
    state: tauri::State<'_, AppState>,
) -> Result<i64, OrqaError> {
    let content = content.trim().to_string();
    if content.is_empty() {
        return Err(OrqaError::Validation(
            "message content cannot be empty".to_string(),
        ));
    }

    let (user_message_id, turn_index) = persist_user_message(&state, session_id, &content)?;
    reset_process_state_if_new_session(&state, session_id);
    super::sidecar_commands::ensure_sidecar_running(&state)?;

    let system_prompt = resolve_system_prompt(&state);
    let provider_session_id = lookup_provider_session_id(&state, session_id)?;

    if let Some(ref prompt) = system_prompt {
        let _ = on_event.send(StreamEvent::SystemPromptSent {
            custom_prompt: None,
            governance_prompt: prompt.clone(),
            total_chars: prompt.len() as i64,
        });
    }

    // Emit ContextInjected if there are prior messages in this session.
    // The just-persisted user message is excluded so only conversation history
    // visible to the AI as prior context is counted.
    match load_context_summary(&state, session_id, user_message_id) {
        Ok((count, chars, json)) if count > 0 => {
            let _ = on_event.send(StreamEvent::ContextInjected {
                message_count: count,
                total_chars: chars,
                messages: json,
            });
        }
        Ok(_) => {}
        Err(e) => {
            tracing::warn!("[stream] failed to load context summary for session {session_id}: {e}");
        }
    }

    let request = SidecarRequest::SendMessage {
        session_id,
        content,
        model,
        system_prompt,
        provider_session_id,
        enable_thinking: false,
    };
    state.sidecar.send(&request)?;

    let acc = run_stream_loop(&state, &on_event);
    persist_assistant_message(&state, session_id, turn_index, &acc)?;
    emit_process_violations(&state, &on_event);

    Ok(user_message_id)
}

/// Request cancellation of an active stream for the given session.
///
/// Sends a `CancelStream` request to the sidecar. The sidecar will respond
/// with a `StreamCancelled` event, which the read loop in `stream_send_message`
/// will handle.
#[tauri::command]
pub fn stream_stop(session_id: i64, state: tauri::State<'_, AppState>) -> Result<(), OrqaError> {
    state
        .sidecar
        .send(&SidecarRequest::CancelStream { session_id })
}

/// Respond to a pending tool approval request from the frontend.
///
/// The stream loop blocks waiting for this command whenever a write or execute
/// tool (e.g. `write_file`, `edit_file`, `bash`) requests approval. Calling this
/// command with `approved = true` permits the tool to run; `approved = false`
/// causes the sidecar to receive a denial and the tool call to fail gracefully.
///
/// Returns `NotFound` if no pending approval exists for the given `tool_call_id`,
/// which can happen if the stream already timed out or completed.
#[tauri::command]
pub fn stream_tool_approval_respond(
    tool_call_id: String,
    approved: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), OrqaError> {
    let sender = state
        .pending_approvals
        .lock()
        .map_err(|_| OrqaError::Sidecar("pending_approvals mutex poisoned".to_string()))?
        .remove(&tool_call_id);

    match sender {
        Some(tx) => {
            tx.send(approved).map_err(|_| {
                OrqaError::Sidecar(format!(
                    "stream loop receiver dropped for tool_call_id={tool_call_id}"
                ))
            })?;
            Ok(())
        }
        None => Err(OrqaError::NotFound(format!(
            "no pending approval for tool_call_id={tool_call_id}"
        ))),
    }
}

// ── Tool Execution ──

/// Resolve the active project's root path for use as working directory.
fn project_root(state: &tauri::State<'_, AppState>) -> Result<PathBuf, String> {
    let conn = state.db.lock().map_err(|e| format!("db lock: {e}"))?;
    let project = project_repo::get_active(&conn)
        .map_err(|e| format!("db query: {e}"))?
        .ok_or_else(|| "no active project".to_string())?;
    Ok(PathBuf::from(project.path))
}

/// Resolve a path from tool input relative to the project root.
/// Returns an error string if the resolved path escapes the project root.
fn resolve_path(raw: &str, root: &Path) -> Result<PathBuf, String> {
    let candidate = if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    };

    let resolved = candidate
        .canonicalize()
        .unwrap_or_else(|_| candidate.clone());
    let root_canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    if !resolved.starts_with(&root_canon) {
        return Err(format!("path '{}' is outside the project root", raw));
    }
    Ok(resolved)
}

/// Resolve a path for writing — the file may not exist yet, so we
/// canonicalize the parent directory instead.
fn resolve_write_path(raw: &str, root: &Path) -> Result<PathBuf, String> {
    let candidate = if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    };

    let root_canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    if let Some(parent) = candidate.parent() {
        let parent_resolved = parent
            .canonicalize()
            .unwrap_or_else(|_| parent.to_path_buf());
        if !parent_resolved.starts_with(&root_canon) {
            return Err(format!("path '{}' is outside the project root", raw));
        }
    }
    Ok(candidate)
}

/// Run enforcement checks for a file write/edit tool call.
///
/// Returns `Some((error_message, true))` if a Block verdict fires.
/// Warn verdicts log but return `None` (execution continues).
fn enforce_file(
    tool_name: &str,
    file_path: &str,
    new_text: &str,
    state: &tauri::State<'_, AppState>,
) -> Option<(String, bool)> {
    let guard = match state.enforcement.lock() {
        Ok(g) => g,
        Err(e) => {
            tracing::warn!("[enforcement] lock poisoned: {e}");
            return None;
        }
    };

    let engine = guard.as_ref()?;

    let verdicts = engine.evaluate_file(file_path, new_text);
    for verdict in &verdicts {
        match verdict.action {
            RuleAction::Block => {
                tracing::debug!(
                    "[enforcement] BLOCK tool={tool_name} rule='{}' file='{file_path}'",
                    verdict.rule_name
                );
                return Some((
                    format!(
                        "Rule '{}' blocked this tool call.\n\n{}",
                        verdict.rule_name, verdict.message
                    ),
                    true,
                ));
            }
            RuleAction::Warn => {
                tracing::warn!(
                    "[enforcement] WARN tool={tool_name} rule='{}' file='{file_path}'",
                    verdict.rule_name
                );
            }
            RuleAction::Inject => {
                tracing::debug!(
                    "[enforcement] INJECT tool={tool_name} rule='{}' file='{file_path}' skills={:?}",
                    verdict.rule_name, verdict.skills
                );
                // Inject is non-blocking — execution continues; caller uses the skills list
            }
        }
    }

    None
}

/// Run enforcement checks for a bash tool call.
///
/// Returns `Some((error_message, true))` if a Block verdict fires.
/// Warn verdicts log but return `None` (execution continues).
fn enforce_bash(command: &str, state: &tauri::State<'_, AppState>) -> Option<(String, bool)> {
    let guard = match state.enforcement.lock() {
        Ok(g) => g,
        Err(e) => {
            tracing::warn!("[enforcement] lock poisoned: {e}");
            return None;
        }
    };

    let engine = guard.as_ref()?;

    let verdicts = engine.evaluate_bash(command);
    for verdict in &verdicts {
        match verdict.action {
            RuleAction::Block => {
                tracing::debug!(
                    "[enforcement] BLOCK tool=bash rule='{}' command='{command}'",
                    verdict.rule_name
                );
                return Some((
                    format!(
                        "Rule '{}' blocked this bash command.\n\n{}",
                        verdict.rule_name, verdict.message
                    ),
                    true,
                ));
            }
            RuleAction::Warn => {
                tracing::warn!(
                    "[enforcement] WARN tool=bash rule='{}' command='{command}'",
                    verdict.rule_name
                );
            }
            RuleAction::Inject => {
                tracing::debug!(
                    "[enforcement] INJECT tool=bash rule='{}' command='{command}' skills={:?}",
                    verdict.rule_name,
                    verdict.skills
                );
                // Inject is non-blocking — execution continues; caller uses the skills list
            }
        }
    }

    None
}

/// Run enforcement checks for the given tool before execution.
///
/// Returns `Some((error_message, true))` if a rule blocks the call, `None` to proceed.
fn check_enforcement(
    tool_name: &str,
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
) -> Option<(String, bool)> {
    match tool_name {
        "write_file" => {
            let file_path = input["path"].as_str().unwrap_or("");
            let new_text = input["content"].as_str().unwrap_or("");
            enforce_file(tool_name, file_path, new_text, state)
        }
        "edit_file" => {
            let file_path = input["path"].as_str().unwrap_or("");
            let new_text = input["new_string"].as_str().unwrap_or("");
            enforce_file(tool_name, file_path, new_text, state)
        }
        "bash" => {
            let command = input["command"].as_str().unwrap_or("");
            enforce_bash(command, state)
        }
        _ => None,
    }
}

/// Dispatch a tool call to the appropriate handler.
/// Returns `(output, is_error)`.
fn execute_tool(
    tool_name: &str,
    input_json: &str,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    tracing::debug!("[tool] execute_tool called: tool={tool_name} input={input_json}");

    let input: serde_json::Value = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(e) => {
            tracing::debug!("[tool] JSON parse error: {e}");
            return (format!("invalid tool input JSON: {e}"), true);
        }
    };

    let root = match project_root(state) {
        Ok(r) => {
            tracing::debug!("[tool] project root: {}", r.display());
            r
        }
        Err(e) => {
            tracing::debug!("[tool] project root error: {e}");
            return (format!("cannot resolve project: {e}"), true);
        }
    };

    if let Some(blocked) = check_enforcement(tool_name, &input, state) {
        return blocked;
    }

    let (output, is_error) = match tool_name {
        "read_file" => tool_read_file(&input, &root),
        "write_file" => tool_write_file(&input, &root),
        "edit_file" => tool_edit_file(&input, &root),
        "bash" => tool_bash(&input, &root),
        "glob" => tool_glob(&input, &root),
        "grep" => tool_grep(&input, &root),
        "search_regex" => tool_search_regex(&input, state),
        "search_semantic" => tool_search_semantic(&input, state),
        "code_research" => tool_code_research(&input, state),
        "load_skill" => tool_load_skill(&input, &root),
        _ => (format!("unknown tool: {tool_name}"), true),
    };

    tracing::debug!(
        "[tool] result: is_error={is_error} output_len={} first_100={}",
        output.len(),
        &output[..output.len().min(100)]
    );
    (output, is_error)
}

/// Read a file's contents, with optional line offset and limit.
///
/// Parameters:
/// - `path` (required): path to the file, relative to the project root.
/// - `offset` (optional, default 0): 0-based line number to start from.
/// - `limit` (optional, default 2000): maximum number of lines to return.
///
/// If the file contains more lines than the effective limit, a truncation notice
/// is appended so the caller knows additional lines exist.
fn tool_read_file(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let raw_path = match input["path"].as_str() {
        Some(p) => p,
        None => return ("missing 'path' parameter".to_string(), true),
    };

    let path = match resolve_path(raw_path, root) {
        Ok(p) => p,
        Err(e) => return (e, true),
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return (format!("failed to read '{}': {e}", path.display()), true),
    };

    let offset = input["offset"].as_u64().unwrap_or(0) as usize;
    let limit = input["limit"]
        .as_u64()
        .map(|n| n as usize)
        .unwrap_or(DEFAULT_READ_FILE_MAX_LINES);

    let all_lines: Vec<&str> = contents.lines().collect();
    let total_lines = all_lines.len();

    if offset >= total_lines && total_lines > 0 {
        return (
            format!("offset {offset} is past end of file ({total_lines} lines total)"),
            true,
        );
    }

    let end = (offset + limit).min(total_lines);
    let selected = &all_lines[offset..end];
    let result = selected.join("\n");

    if end < total_lines {
        (
            format!(
                "{result}\n\n[File truncated: showing lines {}-{} of {total_lines} total. \
                 Use offset/limit parameters for specific ranges.]",
                offset + 1,
                end,
            ),
            false,
        )
    } else {
        (result, false)
    }
}

/// Write content to a file, creating parent directories as needed.
fn tool_write_file(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let raw_path = match input["path"].as_str() {
        Some(p) => p,
        None => return ("missing 'path' parameter".to_string(), true),
    };
    let content = match input["content"].as_str() {
        Some(c) => c,
        None => return ("missing 'content' parameter".to_string(), true),
    };

    let path = match resolve_write_path(raw_path, root) {
        Ok(p) => p,
        Err(e) => return (e, true),
    };

    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return (format!("failed to create directories: {e}"), true);
        }
    }

    match std::fs::write(&path, content) {
        Ok(()) => (
            format!("wrote {} bytes to '{}'", content.len(), path.display()),
            false,
        ),
        Err(e) => (format!("failed to write '{}': {e}", path.display()), true),
    }
}

/// Edit a file by replacing old_string with new_string.
fn tool_edit_file(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let raw_path = match input["path"].as_str() {
        Some(p) => p,
        None => return ("missing 'path' parameter".to_string(), true),
    };
    let old_string = match input["old_string"].as_str() {
        Some(s) => s,
        None => return ("missing 'old_string' parameter".to_string(), true),
    };
    let new_string = match input["new_string"].as_str() {
        Some(s) => s,
        None => return ("missing 'new_string' parameter".to_string(), true),
    };

    let path = match resolve_path(raw_path, root) {
        Ok(p) => p,
        Err(e) => return (e, true),
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return (format!("failed to read '{}': {e}", path.display()), true),
    };

    let count = contents.matches(old_string).count();
    if count == 0 {
        return (
            format!("old_string not found in '{}'", path.display()),
            true,
        );
    }
    if count > 1 {
        return (
            format!(
                "old_string found {count} times in '{}' — must be unique",
                path.display()
            ),
            true,
        );
    }

    let updated = contents.replacen(old_string, new_string, 1);
    match std::fs::write(&path, &updated) {
        Ok(()) => (
            format!("edited '{}' (1 replacement)", path.display()),
            false,
        ),
        Err(e) => (format!("failed to write '{}': {e}", path.display()), true),
    }
}

/// Execute a bash command in the project root.
fn tool_bash(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let command = match input["command"].as_str() {
        Some(c) => c,
        None => return ("missing 'command' parameter".to_string(), true),
    };

    let output = match std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .current_dir(root)
        .output()
    {
        Ok(o) => o,
        Err(e) => return (format!("failed to execute bash: {e}"), true),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut result = String::new();
    if !stdout.is_empty() {
        result.push_str(&stdout);
    }
    if !stderr.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str("STDERR:\n");
        result.push_str(&stderr);
    }
    if result.is_empty() {
        result.push_str("(no output)");
    }

    let is_error = !output.status.success();
    (result, is_error)
}

/// Find files matching a glob pattern.
fn tool_glob(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let pattern = match input["pattern"].as_str() {
        Some(p) => p,
        None => return ("missing 'pattern' parameter".to_string(), true),
    };

    let search_root = match input["path"].as_str() {
        Some(p) => root.join(p),
        None => root.to_path_buf(),
    };

    let full_pattern = search_root.join(pattern);
    let pattern_str = full_pattern.to_string_lossy();

    match glob::glob(&pattern_str) {
        Ok(entries) => {
            let mut paths: Vec<String> = Vec::new();
            for entry in entries {
                match entry {
                    Ok(path) => {
                        let display = path
                            .strip_prefix(root)
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .to_string();
                        paths.push(display);
                    }
                    Err(e) => {
                        paths.push(format!("(error: {e})"));
                    }
                }
            }
            if paths.is_empty() {
                ("no matches found".to_string(), false)
            } else {
                (paths.join("\n"), false)
            }
        }
        Err(e) => (format!("invalid glob pattern: {e}"), true),
    }
}

/// Search file contents with a regex pattern.
fn tool_grep(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let pattern = match input["pattern"].as_str() {
        Some(p) => p,
        None => return ("missing 'pattern' parameter".to_string(), true),
    };

    let search_path = match input["path"].as_str() {
        Some(p) => root.join(p),
        None => root.to_path_buf(),
    };

    let search_str = search_path.to_string_lossy();
    let cmd = format!(
        "rg --no-heading --line-number --color never -e {} {} 2>/dev/null || grep -rn {} {} 2>/dev/null",
        shell_escape(pattern),
        shell_escape(&search_str),
        shell_escape(pattern),
        shell_escape(&search_str),
    );

    let output = match std::process::Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .current_dir(root)
        .output()
    {
        Ok(o) => o,
        Err(e) => return (format!("failed to execute grep: {e}"), true),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return ("no matches found".to_string(), false);
    }

    let lines: Vec<&str> = stdout.lines().collect();
    if lines.len() > 200 {
        let truncated: String = lines[..200].join("\n");
        (
            format!(
                "{truncated}\n\n... ({} total matches, showing first 200)",
                lines.len()
            ),
            false,
        )
    } else {
        (stdout.to_string(), false)
    }
}

/// Simple shell escaping — wraps in single quotes.
fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Format search results as a readable text block.
fn format_search_results(results: &[crate::search::types::SearchResult]) -> String {
    if results.is_empty() {
        return "no matches found".to_string();
    }
    let mut out = String::new();
    for result in results {
        out.push_str(&format!(
            "{}:{}-{}\n{}\n---\n",
            result.file_path, result.start_line, result.end_line, result.content,
        ));
    }
    out
}

/// Search the indexed codebase with a regex pattern.
fn tool_search_regex(
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    let pattern = match input["pattern"].as_str() {
        Some(p) => p,
        None => return ("missing 'pattern' parameter".to_string(), true),
    };
    let path_filter = input["path"].as_str();
    let max_results = input["max_results"]
        .as_u64()
        .map(|n| n as u32)
        .unwrap_or(20);

    let search_guard = match state.search.lock() {
        Ok(g) => g,
        Err(e) => return (format!("search lock error: {e}"), true),
    };
    let engine = match search_guard.as_ref() {
        Some(e) => e,
        None => {
            return (
                "search index not initialized — index the codebase first".to_string(),
                true,
            )
        }
    };

    match engine.search_regex(pattern, path_filter, max_results) {
        Ok(results) => (format_search_results(&results), false),
        Err(e) => (format!("search_regex failed: {e}"), true),
    }
}

/// Search the indexed codebase using semantic similarity.
fn tool_search_semantic(
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    let query = match input["query"].as_str() {
        Some(q) => q,
        None => return ("missing 'query' parameter".to_string(), true),
    };
    let max_results = input["max_results"]
        .as_u64()
        .map(|n| n as u32)
        .unwrap_or(10);

    let mut search_guard = match state.search.lock() {
        Ok(g) => g,
        Err(e) => return (format!("search lock error: {e}"), true),
    };
    let engine = match search_guard.as_mut() {
        Some(e) => e,
        None => {
            return (
                "search index not initialized — index the codebase first".to_string(),
                true,
            )
        }
    };

    match engine.search_semantic(query, max_results) {
        Ok(results) => (format_search_results(&results), false),
        Err(e) => (format!("search_semantic failed: {e}"), true),
    }
}

/// Append semantic search results for `query` to `out`, returning an error string on lock failure.
fn collect_semantic_section(
    query: &str,
    max_results: u32,
    state: &tauri::State<'_, AppState>,
    out: &mut String,
) -> Result<(), String> {
    let mut search_guard = state
        .search
        .lock()
        .map_err(|e| format!("search lock error: {e}"))?;
    if let Some(engine) = search_guard.as_mut() {
        match engine.search_semantic(query, max_results) {
            Ok(results) if !results.is_empty() => {
                out.push_str("## Semantic Matches\n\n");
                out.push_str(&format_search_results(&results));
                out.push('\n');
            }
            Ok(_) => {}
            Err(e) => out.push_str(&format!("(semantic search unavailable: {e})\n\n")),
        }
    }
    Ok(())
}

/// Append regex search results for the escaped `query` to `out`, returning an error on lock failure.
fn collect_regex_section(
    query: &str,
    max_results: u32,
    state: &tauri::State<'_, AppState>,
    out: &mut String,
) -> Result<(), String> {
    let search_guard = state
        .search
        .lock()
        .map_err(|e| format!("search lock error: {e}"))?;
    if let Some(engine) = search_guard.as_ref() {
        let escaped = regex::escape(query);
        match engine.search_regex(&escaped, None, max_results) {
            Ok(results) if !results.is_empty() => {
                out.push_str("## Regex Matches\n\n");
                out.push_str(&format_search_results(&results));
            }
            Ok(_) => {}
            Err(e) => out.push_str(&format!("(regex search unavailable: {e})\n\n")),
        }
    }
    Ok(())
}

/// Combined code research: runs both regex and semantic search, merging results.
///
/// Accepts a `query` string and optional `max_results`. The query is used as-is
/// for semantic search. For regex search, it is treated as a literal pattern
/// (special regex chars are escaped).
fn tool_code_research(
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    let query = match input["query"].as_str() {
        Some(q) => q,
        None => return ("missing 'query' parameter".to_string(), true),
    };
    let max_results = input["max_results"]
        .as_u64()
        .map(|n| n as u32)
        .unwrap_or(10);

    let half = max_results / 2 + 1;
    let mut out = String::new();

    if let Err(e) = collect_semantic_section(query, half, state, &mut out) {
        return (e, true);
    }
    if let Err(e) = collect_regex_section(query, half, state, &mut out) {
        return (e, true);
    }

    if out.is_empty() {
        (
            "search index not initialized — index the codebase first".to_string(),
            true,
        )
    } else if out.trim().is_empty() || (out.contains("unavailable") && !out.contains("Matches")) {
        ("no results found".to_string(), false)
    } else {
        (out, false)
    }
}

/// Load the full content of a skill from `.orqa/skills/{name}/SKILL.md`.
fn tool_load_skill(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let name = match input["name"].as_str() {
        Some(n) => n,
        None => return ("missing 'name' parameter".to_string(), true),
    };

    // Validate skill name: must be a simple directory name with no path separators
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return (
            format!("invalid skill name '{name}': must not contain path separators"),
            true,
        );
    }

    let skill_path = root
        .join(".orqa")
        .join("skills")
        .join(name)
        .join("SKILL.md");

    match std::fs::read_to_string(&skill_path) {
        Ok(contents) => (contents, false),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (
            format!("skill '{name}' not found at '{}'", skill_path.display()),
            true,
        ),
        Err(e) => (format!("failed to read skill '{name}': {e}"), true),
    }
}

/// Preview the auto-generated system prompt for the active project.
#[tauri::command]
pub fn system_prompt_preview(
    state: tauri::State<'_, AppState>,
) -> Result<Option<String>, OrqaError> {
    let project_root = match project_root(&state) {
        Ok(root) => root,
        Err(_) => return Ok(None),
    };
    Ok(build_system_prompt(&project_root).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── translate_response tests ──

    #[test]
    fn translate_stream_start() {
        let resp = SidecarResponse::StreamStart {
            message_id: 42,
            resolved_model: Some("claude-opus-4-6".to_string()),
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::StreamStart {
                message_id,
                resolved_model,
            } => {
                assert_eq!(message_id, 42);
                assert_eq!(resolved_model.as_deref(), Some("claude-opus-4-6"));
            }
            _ => panic!("expected StreamStart"),
        }
    }

    #[test]
    fn translate_text_delta() {
        let resp = SidecarResponse::TextDelta {
            content: "Hello ".to_string(),
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::TextDelta { content } => assert_eq!(content, "Hello "),
            _ => panic!("expected TextDelta"),
        }
    }

    #[test]
    fn translate_thinking_delta() {
        let resp = SidecarResponse::ThinkingDelta {
            content: "Let me consider...".to_string(),
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::ThinkingDelta { content } => assert_eq!(content, "Let me consider..."),
            _ => panic!("expected ThinkingDelta"),
        }
    }

    #[test]
    fn translate_tool_use_start() {
        let resp = SidecarResponse::ToolUseStart {
            tool_call_id: "call_001".to_string(),
            tool_name: "read_file".to_string(),
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::ToolUseStart {
                tool_call_id,
                tool_name,
            } => {
                assert_eq!(tool_call_id, "call_001");
                assert_eq!(tool_name, "read_file");
            }
            _ => panic!("expected ToolUseStart"),
        }
    }

    #[test]
    fn translate_tool_input_delta() {
        let resp = SidecarResponse::ToolInputDelta {
            tool_call_id: "call_001".to_string(),
            content: r#"{"path":"#.to_string(),
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::ToolInputDelta {
                tool_call_id,
                content,
            } => {
                assert_eq!(tool_call_id, "call_001");
                assert_eq!(content, r#"{"path":"#);
            }
            _ => panic!("expected ToolInputDelta"),
        }
    }

    #[test]
    fn translate_tool_result() {
        let resp = SidecarResponse::ToolResult {
            tool_call_id: "call_001".to_string(),
            tool_name: "read_file".to_string(),
            result: "file contents".to_string(),
            is_error: false,
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::ToolResult {
                tool_call_id,
                tool_name,
                result,
                is_error,
            } => {
                assert_eq!(tool_call_id, "call_001");
                assert_eq!(tool_name, "read_file");
                assert_eq!(result, "file contents");
                assert!(!is_error);
            }
            _ => panic!("expected ToolResult"),
        }
    }

    #[test]
    fn translate_block_complete() {
        let resp = SidecarResponse::BlockComplete {
            block_index: 2,
            content_type: "text".to_string(),
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::BlockComplete {
                block_index,
                content_type,
            } => {
                assert_eq!(block_index, 2);
                assert_eq!(content_type, "text");
            }
            _ => panic!("expected BlockComplete"),
        }
    }

    #[test]
    fn translate_turn_complete() {
        let resp = SidecarResponse::TurnComplete {
            input_tokens: 500,
            output_tokens: 200,
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::TurnComplete {
                input_tokens,
                output_tokens,
            } => {
                assert_eq!(input_tokens, 500);
                assert_eq!(output_tokens, 200);
            }
            _ => panic!("expected TurnComplete"),
        }
    }

    #[test]
    fn translate_stream_error() {
        let resp = SidecarResponse::StreamError {
            code: "rate_limit".to_string(),
            message: "Too many requests".to_string(),
            recoverable: true,
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::StreamError {
                code,
                message,
                recoverable,
            } => {
                assert_eq!(code, "rate_limit");
                assert_eq!(message, "Too many requests");
                assert!(recoverable);
            }
            _ => panic!("expected StreamError"),
        }
    }

    #[test]
    fn translate_stream_cancelled() {
        let resp = SidecarResponse::StreamCancelled;
        let event = translate_response(&resp).expect("should translate");
        assert!(matches!(event, StreamEvent::StreamCancelled));
    }

    #[test]
    fn translate_health_ok_returns_none() {
        let resp = SidecarResponse::HealthOk {
            version: "0.1.0".to_string(),
        };
        assert!(translate_response(&resp).is_none());
    }

    #[test]
    fn translate_summary_result_returns_none() {
        let resp = SidecarResponse::SummaryResult {
            session_id: 1,
            summary: "a summary".to_string(),
        };
        assert!(translate_response(&resp).is_none());
    }

    // ── is_terminal tests ──

    #[test]
    fn turn_complete_is_terminal() {
        let resp = SidecarResponse::TurnComplete {
            input_tokens: 100,
            output_tokens: 50,
        };
        assert!(is_terminal(&resp));
    }

    #[test]
    fn stream_error_is_terminal() {
        let resp = SidecarResponse::StreamError {
            code: "err".to_string(),
            message: "failed".to_string(),
            recoverable: false,
        };
        assert!(is_terminal(&resp));
    }

    #[test]
    fn stream_cancelled_is_terminal() {
        let resp = SidecarResponse::StreamCancelled;
        assert!(is_terminal(&resp));
    }

    #[test]
    fn text_delta_is_not_terminal() {
        let resp = SidecarResponse::TextDelta {
            content: "hello".to_string(),
        };
        assert!(!is_terminal(&resp));
    }

    #[test]
    fn stream_start_is_not_terminal() {
        let resp = SidecarResponse::StreamStart {
            message_id: 1,
            resolved_model: None,
        };
        assert!(!is_terminal(&resp));
    }

    #[test]
    fn block_complete_is_not_terminal() {
        let resp = SidecarResponse::BlockComplete {
            block_index: 0,
            content_type: "text".to_string(),
        };
        assert!(!is_terminal(&resp));
    }

    #[test]
    fn health_ok_is_not_terminal() {
        let resp = SidecarResponse::HealthOk {
            version: "1.0".to_string(),
        };
        assert!(!is_terminal(&resp));
    }

    // ── ToolExecute / ToolApprovalRequest tests ──

    #[test]
    fn translate_tool_execute_returns_none() {
        let resp = SidecarResponse::ToolExecute {
            tool_call_id: "call_010".to_string(),
            tool_name: "read_file".to_string(),
            input: r#"{"path":"/src/main.rs"}"#.to_string(),
        };
        assert!(translate_response(&resp).is_none());
    }

    #[test]
    fn translate_tool_approval_request_returns_event() {
        // ToolApprovalRequest is forwarded to the frontend as a StreamEvent so
        // the UI can display the approval dialog for write/execute tools.
        let resp = SidecarResponse::ToolApprovalRequest {
            tool_call_id: "call_011".to_string(),
            tool_name: "write_file".to_string(),
            input: r#"{"path":"/tmp/out.txt"}"#.to_string(),
        };
        let event = translate_response(&resp);
        assert!(event.is_some());
        if let Some(StreamEvent::ToolApprovalRequest {
            tool_call_id,
            tool_name,
            input,
        }) = event
        {
            assert_eq!(tool_call_id, "call_011");
            assert_eq!(tool_name, "write_file");
            assert_eq!(input, r#"{"path":"/tmp/out.txt"}"#);
        } else {
            panic!("expected StreamEvent::ToolApprovalRequest");
        }
    }

    #[test]
    fn tool_execute_is_not_terminal() {
        let resp = SidecarResponse::ToolExecute {
            tool_call_id: "call_010".to_string(),
            tool_name: "read_file".to_string(),
            input: "{}".to_string(),
        };
        assert!(!is_terminal(&resp));
    }

    #[test]
    fn tool_approval_request_is_not_terminal() {
        let resp = SidecarResponse::ToolApprovalRequest {
            tool_call_id: "call_011".to_string(),
            tool_name: "write_file".to_string(),
            input: "{}".to_string(),
        };
        assert!(!is_terminal(&resp));
    }

    // ── Content accumulation simulation ──

    #[test]
    fn accumulate_text_deltas() {
        let responses = vec![
            SidecarResponse::StreamStart {
                message_id: 1,
                resolved_model: None,
            },
            SidecarResponse::TextDelta {
                content: "Hello".to_string(),
            },
            SidecarResponse::TextDelta {
                content: ", ".to_string(),
            },
            SidecarResponse::TextDelta {
                content: "world!".to_string(),
            },
            SidecarResponse::BlockComplete {
                block_index: 0,
                content_type: "text".to_string(),
            },
            SidecarResponse::TurnComplete {
                input_tokens: 100,
                output_tokens: 50,
            },
        ];

        let mut accumulated = String::new();
        let mut final_input_tokens: i64 = 0;
        let mut final_output_tokens: i64 = 0;

        for resp in &responses {
            if let SidecarResponse::TextDelta { content } = resp {
                accumulated.push_str(content);
            }
            if let SidecarResponse::TurnComplete {
                input_tokens,
                output_tokens,
            } = resp
            {
                final_input_tokens = *input_tokens;
                final_output_tokens = *output_tokens;
            }
        }

        assert_eq!(accumulated, "Hello, world!");
        assert_eq!(final_input_tokens, 100);
        assert_eq!(final_output_tokens, 50);
    }

    #[test]
    fn empty_stream_produces_empty_accumulation() {
        let responses = vec![
            SidecarResponse::StreamStart {
                message_id: 1,
                resolved_model: None,
            },
            SidecarResponse::StreamError {
                code: "internal".to_string(),
                message: "something went wrong".to_string(),
                recoverable: false,
            },
        ];

        let mut accumulated = String::new();
        for resp in &responses {
            if let SidecarResponse::TextDelta { content } = resp {
                accumulated.push_str(content);
            }
        }

        assert!(accumulated.is_empty());
    }

    // ── Validation tests (logic only, not Tauri state) ──

    #[test]
    fn empty_content_validation() {
        let content = "   ".trim().to_string();
        assert!(content.is_empty());
    }

    #[test]
    fn whitespace_only_content_validation() {
        let content = "  \t\n  ".trim().to_string();
        assert!(content.is_empty());
    }

    #[test]
    fn valid_content_passes_validation() {
        let content = "  Hello world  ".trim().to_string();
        assert!(!content.is_empty());
        assert_eq!(content, "Hello world");
    }

    // ── Tool approval classification tests ──

    #[test]
    fn read_only_tools_are_recognized() {
        let read_only = [
            "read_file",
            "glob",
            "grep",
            "search_regex",
            "search_semantic",
            "load_skill",
            "code_research",
        ];
        for tool in &read_only {
            assert!(
                READ_ONLY_TOOLS.contains(tool),
                "{tool} should be in READ_ONLY_TOOLS"
            );
        }
    }

    #[test]
    fn write_tools_are_not_read_only() {
        let write_tools = ["write_file", "edit_file", "bash"];
        for tool in &write_tools {
            assert!(
                !READ_ONLY_TOOLS.contains(tool),
                "{tool} must NOT be in READ_ONLY_TOOLS — it requires user approval"
            );
        }
    }

    // ── truncate_tool_output tests ──

    #[test]
    fn truncate_tool_output_short_output_unchanged() {
        let short = "hello world".to_string();
        let result = truncate_tool_output(short.clone());
        assert_eq!(result, short);
    }

    #[test]
    fn truncate_tool_output_exactly_at_limit_unchanged() {
        let at_limit = "x".repeat(MAX_TOOL_OUTPUT_CHARS);
        let result = truncate_tool_output(at_limit.clone());
        assert_eq!(result, at_limit);
    }

    #[test]
    fn truncate_tool_output_over_limit_includes_notice() {
        let over_limit = "x".repeat(MAX_TOOL_OUTPUT_CHARS + 500);
        let total_len = over_limit.len();
        let result = truncate_tool_output(over_limit);
        assert!(result.contains("[Output truncated:"));
        assert!(result.contains(&total_len.to_string()));
        assert!(result.len() > MAX_TOOL_OUTPUT_CHARS);
        // The first MAX_TOOL_OUTPUT_CHARS chars should be the 'x' characters
        assert!(result.starts_with(&"x".repeat(MAX_TOOL_OUTPUT_CHARS)));
    }

    // ── friendly_context_overflow_message tests ──

    #[test]
    fn friendly_message_returned_for_context_code() {
        let msg = friendly_context_overflow_message("context_length_exceeded", "too long");
        assert!(msg.is_some());
        let text = msg.unwrap();
        assert!(text.contains("context window"));
    }

    #[test]
    fn friendly_message_returned_for_token_in_message() {
        let msg = friendly_context_overflow_message("api_error", "token limit reached");
        assert!(msg.is_some());
    }

    #[test]
    fn friendly_message_returned_for_context_window_in_message() {
        let msg = friendly_context_overflow_message("api_error", "context window exceeded");
        assert!(msg.is_some());
    }

    #[test]
    fn friendly_message_none_for_unrelated_error() {
        let msg = friendly_context_overflow_message("rate_limit", "Too many requests");
        assert!(msg.is_none());
    }

    #[test]
    fn translate_stream_error_context_overflow_gets_friendly_message() {
        let resp = SidecarResponse::StreamError {
            code: "context_length_exceeded".to_string(),
            message: "Input is too long".to_string(),
            recoverable: false,
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::StreamError { code, message, .. } => {
                assert_eq!(code, "context_length_exceeded");
                // Message should be the friendly version, not the raw one
                assert!(message.contains("context window"));
            }
            _ => panic!("expected StreamError"),
        }
    }

    #[test]
    fn translate_stream_error_normal_error_keeps_original_message() {
        let resp = SidecarResponse::StreamError {
            code: "rate_limit".to_string(),
            message: "Too many requests".to_string(),
            recoverable: true,
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::StreamError { message, .. } => {
                assert_eq!(message, "Too many requests");
            }
            _ => panic!("expected StreamError"),
        }
    }

    // ── tool_read_file line limit tests ──

    #[test]
    fn read_file_line_limit_applies_truncation_notice() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut f = NamedTempFile::new().expect("temp file");
        for i in 0..2500 {
            writeln!(f, "line {i}").expect("write");
        }
        let path = f.path().to_path_buf();
        let root = path.parent().expect("parent").to_path_buf();
        let file_name = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();

        let input = serde_json::json!({ "path": file_name });
        let (output, is_error) = tool_read_file(&input, &root);
        assert!(!is_error, "should not be an error: {output}");
        assert!(
            output.contains("[File truncated:"),
            "should contain truncation notice"
        );
    }

    #[test]
    fn read_file_offset_and_limit_respected() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut f = NamedTempFile::new().expect("temp file");
        for i in 0..100 {
            writeln!(f, "line {i}").expect("write");
        }
        let path = f.path().to_path_buf();
        let root = path.parent().expect("parent").to_path_buf();
        let file_name = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();

        let input = serde_json::json!({ "path": file_name, "offset": 10, "limit": 5 });
        let (output, is_error) = tool_read_file(&input, &root);
        assert!(!is_error, "should not be an error: {output}");
        assert!(output.contains("line 10"));
        assert!(output.contains("line 14"));
        assert!(
            !output.contains("line 9"),
            "should not include line before offset"
        );
        assert!(
            !output.contains("line 15"),
            "should not include line past limit"
        );
    }

    #[test]
    fn read_file_small_file_no_truncation_notice() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut f = NamedTempFile::new().expect("temp file");
        for i in 0..10 {
            writeln!(f, "line {i}").expect("write");
        }
        let path = f.path().to_path_buf();
        let root = path.parent().expect("parent").to_path_buf();
        let file_name = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();

        let input = serde_json::json!({ "path": file_name });
        let (output, is_error) = tool_read_file(&input, &root);
        assert!(!is_error);
        assert!(!output.contains("[File truncated:"));
    }
}
