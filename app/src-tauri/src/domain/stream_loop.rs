//! Stream loop for the orqa-studio Tauri app.
//!
//! Drives the sidecar read loop and dispatches responses to the frontend via
//! Tauri's Channel<T>. Pure streaming logic (response translation, terminal
//! detection, accumulation) lives in orqa_engine::streaming::stream_loop and
//! is re-used here. All Tauri-specific code (AppState access, Channel<T>,
//! session persistence, workflow tracking) is in this file.

use crate::domain::enforcement::Verdict;
use crate::domain::process_gates::{evaluate_stop_verdicts, evaluate_write_verdicts};
use crate::domain::process_state::ProcessStateExt;
use crate::domain::provider_event::StreamEvent;
use crate::domain::tool_executor::{execute_tool, READ_ONLY_TOOLS};
use crate::sidecar::types::{SidecarRequest, SidecarResponse};
use crate::state::AppState;

#[cfg(test)]
use orqa_engine::streaming::stream_loop::friendly_context_overflow_message;
pub use orqa_engine::streaming::stream_loop::StreamAccumulator;
use orqa_engine::streaming::stream_loop::{accumulate_response, is_terminal, translate_response};
use orqa_engine::streaming::tools::truncate_tool_output;

use std::sync::mpsc;

/// Handle a `ToolExecute` response: execute the tool and send the result back to the sidecar.
///
/// Returns `true` to continue the loop, `false` on send failure.
pub fn handle_tool_execute(
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
        tool_call_id: tool_call_id.to_owned(),
        output,
        is_error,
    };
    if let Err(e) = state.sidecar.manager.send(&tool_result) {
        let _ = on_event.send(StreamEvent::StreamError {
            code: "tool_result_send_error".to_owned(),
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
pub fn send_approval(
    tool_call_id: &str,
    approved: bool,
    reason: Option<String>,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> bool {
    let approval = SidecarRequest::ToolApproval {
        tool_call_id: tool_call_id.to_owned(),
        approved,
        reason,
    };
    if let Err(e) = state.sidecar.manager.send(&approval) {
        let _ = on_event.send(StreamEvent::StreamError {
            code: "tool_approval_send_error".to_owned(),
            message: format!("failed to send tool approval to sidecar: {e}"),
            recoverable: false,
        });
        return false;
    }
    true
}

/// Handle a `ToolApprovalRequest`.
///
/// Read-only tools (listed in `READ_ONLY_TOOLS`) are auto-approved immediately.
/// Write/execute tools emit a `StreamEvent::ToolApprovalRequest` to the frontend
/// and block on a sync channel until `stream_tool_approval_respond` is called.
///
/// Returns `true` to continue the loop, `false` on failure.
pub fn handle_tool_approval(
    tool_call_id: &str,
    tool_name: &str,
    input: &str,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> bool {
    tracing::debug!("[stream] ToolApprovalRequest: id={tool_call_id} tool={tool_name}");

    // Auto-approve read-only tools
    if READ_ONLY_TOOLS.contains(&tool_name) {
        tracing::debug!("[stream] auto-approving read-only tool: {tool_name}");
        return send_approval(tool_call_id, true, None, state, on_event);
    }

    // For write/execute tools, emit event to frontend and wait for user decision
    tracing::debug!("[stream] requesting user approval for: {tool_name}");

    let (tx, rx) = mpsc::sync_channel::<bool>(1);

    // Register the sender so stream_tool_approval_respond can signal us
    {
        let Ok(mut map) = state.sidecar.pending_approvals.lock() else {
            tracing::error!("[stream] pending_approvals mutex poisoned");
            return send_approval(
                tool_call_id,
                false,
                Some("internal error".to_owned()),
                state,
                on_event,
            );
        };
        map.insert(tool_call_id.to_owned(), tx);
    }

    // Emit the approval request event to the frontend
    let emit_result = on_event.send(StreamEvent::ToolApprovalRequest {
        tool_call_id: tool_call_id.to_owned(),
        tool_name: tool_name.to_owned(),
        input: input.to_owned(),
    });
    if emit_result.is_err() {
        tracing::warn!("[stream] failed to emit ToolApprovalRequest to frontend");
        // Clean up our registration and deny
        if let Ok(mut map) = state.sidecar.pending_approvals.lock() {
            map.remove(tool_call_id);
        }
        return send_approval(
            tool_call_id,
            false,
            Some("frontend not reachable".to_owned()),
            state,
            on_event,
        );
    }

    // Block until the frontend calls stream_tool_approval_respond
    let approved = rx.recv().unwrap_or(false);
    tracing::debug!("[stream] received user decision for {tool_call_id}: approved={approved}");

    let reason = if approved {
        None
    } else {
        Some("denied by user".to_owned())
    };
    send_approval(tool_call_id, approved, reason, state, on_event)
}

/// Read the next sidecar response, emitting a `StreamError` on failure.
///
/// Returns `Some(response)` on success, `None` on EOF or read error.
fn read_next_response(
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> Option<SidecarResponse> {
    match state.sidecar.manager.read_line() {
        Ok(Some(resp)) => Some(resp),
        Ok(None) => {
            let _ = on_event.send(StreamEvent::StreamError {
                code: "sidecar_eof".to_owned(),
                message: "sidecar process closed unexpectedly".to_owned(),
                recoverable: false,
            });
            None
        }
        Err(e) => {
            let _ = on_event.send(StreamEvent::StreamError {
                code: "sidecar_read_error".to_owned(),
                message: e.to_string(),
                recoverable: false,
            });
            None
        }
    }
}

/// Persist a provider session UUID when the sidecar sends `SessionInitialized`.
fn persist_provider_session_id(
    state: &tauri::State<'_, AppState>,
    session_id: i64,
    provider_session_id: &str,
) {
    use crate::repo::session_repo;

    if let Ok(db) = state.db.conn.lock() {
        if let Err(e) =
            session_repo::update_provider_session_id(&db, session_id, provider_session_id)
        {
            tracing::warn!("[stream] failed to persist provider_session_id: {e}");
        }
    }
}

/// Dispatch a single sidecar response within the stream loop.
///
/// Returns `Some(true)` to continue the loop, `Some(false)` to break
/// (terminal event reached), or `None` on a fatal send error.
fn dispatch_response(
    response: &SidecarResponse,
    acc: &mut StreamAccumulator,
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) -> Option<bool> {
    if let SidecarResponse::SessionInitialized {
        session_id,
        ref provider_session_id,
    } = *response
    {
        persist_provider_session_id(state, session_id, provider_session_id);
        return Some(true);
    }
    if let SidecarResponse::ToolExecute {
        ref tool_call_id,
        ref tool_name,
        ref input,
    } = *response
    {
        return if handle_tool_execute(tool_call_id, tool_name, input, state, on_event) {
            Some(true)
        } else {
            None
        };
    }
    if let SidecarResponse::ToolApprovalRequest {
        ref tool_call_id,
        ref tool_name,
        ref input,
    } = *response
    {
        return if handle_tool_approval(tool_call_id, tool_name, input, state, on_event) {
            Some(true)
        } else {
            None
        };
    }
    accumulate_response(response, acc);
    if matches!(response, SidecarResponse::TurnComplete { .. }) {
        evaluate_stop_gates(state);
    }
    if let Some(event) = translate_response(response) {
        let _ = on_event.send(event);
    }
    Some(!is_terminal(response))
}

/// Run the sidecar read loop, accumulating results into a `StreamAccumulator`.
pub fn run_stream_loop(
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
        let Some(response) = read_next_response(state, on_event) else {
            acc.had_error = true;
            break;
        };
        match dispatch_response(&response, &mut acc, state, on_event) {
            Some(true) => {}
            Some(false) => break,
            None => {
                acc.had_error = true;
                break;
            }
        }
    }

    acc
}

/// Evaluate both the enforcement engine and process gates for a file write event,
/// returning merged verdicts from both systems.
///
/// This is the unified evaluation pipeline for write/edit tool calls. Gate verdicts
/// come first so they appear at the top of any injected context. Process gates check
/// workflow state conditions; the enforcement engine checks rule pattern matching.
fn evaluate_unified_write(
    tracker: &mut crate::domain::workflow_tracker::WorkflowTracker,
    file_path: &str,
    state: &tauri::State<'_, AppState>,
) -> Vec<Verdict> {
    let gates = match state.session.process_gates.lock() {
        Ok(g) => g.clone(),
        Err(e) => {
            tracing::warn!("[process_gates] process_gates mutex poisoned in write eval: {e}");
            Vec::new()
        }
    };
    let mut verdicts = evaluate_write_verdicts(&gates, tracker, file_path);

    // Append enforcement engine verdicts for this file write.
    let engine_guard = match state.enforcement.engine.lock() {
        Ok(g) => g,
        Err(e) => {
            tracing::warn!("[enforcement] lock poisoned in unified write eval: {e}");
            return verdicts;
        }
    };
    if let Some(engine) = engine_guard.as_ref() {
        verdicts.extend(engine.evaluate_file(file_path, ""));
    }

    verdicts
}

/// Evaluate both the enforcement engine and process gates for a turn-complete (stop) event,
/// returning merged verdicts from both systems.
///
/// Process gates check workflow state at turn end (evidence-before-done,
/// learn-after-doing). This unified path ensures both systems share the same output channel.
fn evaluate_unified_stop(
    tracker: &crate::domain::workflow_tracker::WorkflowTracker,
    state: &tauri::State<'_, AppState>,
) -> Vec<Verdict> {
    let gates = match state.session.process_gates.lock() {
        Ok(g) => g.clone(),
        Err(e) => {
            tracing::warn!("[process_gates] process_gates mutex poisoned in stop eval: {e}");
            Vec::new()
        }
    };
    evaluate_stop_verdicts(&gates, tracker)
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
    match state.session.process_state.lock() {
        Ok(mut ps) => ps.track_tool_call(tool_name, &input),
        Err(e) => {
            tracing::warn!("[process] process_state mutex poisoned, skipping track: {e}");
        }
    }
    track_workflow(tool_name, &input, state);
}

/// Record a completed tool call in the workflow tracker and evaluate process gates.
///
/// Gate messages are logged at debug level. Callers that want to surface them
/// to the agent should read the workflow tracker separately.
fn track_workflow(tool_name: &str, input: &serde_json::Value, state: &tauri::State<'_, AppState>) {
    let mut guard = match state.session.workflow_tracker.lock() {
        Ok(g) => g,
        Err(e) => {
            tracing::warn!("[workflow] workflow_tracker mutex poisoned, skipping track: {e}");
            return;
        }
    };

    match tool_name {
        "read_file" => {
            if let Some(path) = input["path"].as_str() {
                guard.record_read(path);
            }
        }
        "write_file" | "edit_file" => {
            if let Some(path) = input["path"].as_str() {
                guard.record_write(path);
                // Evaluate unified write pipeline: process gates + enforcement engine
                let verdicts = evaluate_unified_write(&mut guard, path, state);
                for v in &verdicts {
                    tracing::debug!(
                        "[enforcement] rule='{}' action={:?} fired at write: {}",
                        v.rule_name,
                        v.action,
                        v.message
                    );
                }
            }
        }
        "bash" => {
            if let Some(cmd) = input["command"].as_str() {
                guard.record_command(cmd);
            }
        }
        "search_regex" | "search_semantic" | "code_research" => {
            guard.record_search();
        }
        "load_knowledge" => {
            if let Some(name) = input["name"].as_str() {
                guard.record_knowledge_loaded(name);
            }
        }
        _ => {}
    }
}

/// Evaluate the unified stop pipeline (process gates) against the current workflow tracker.
///
/// Called at `TurnComplete`. Verdict messages are logged at debug level.
pub fn evaluate_stop_gates(state: &tauri::State<'_, AppState>) {
    let guard = match state.session.workflow_tracker.lock() {
        Ok(g) => g,
        Err(e) => {
            tracing::warn!("[workflow] workflow_tracker mutex poisoned, skipping stop gates: {e}");
            return;
        }
    };
    let verdicts = evaluate_unified_stop(&guard, state);
    for v in &verdicts {
        tracing::debug!(
            "[enforcement] rule='{}' action={:?} fired at stop: {}",
            v.rule_name,
            v.action,
            v.message
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sidecar::types::SidecarResponse;

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
}
