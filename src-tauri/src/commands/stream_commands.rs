use crate::domain::project_settings::read_settings;
use crate::domain::provider_event::StreamEvent;
use crate::domain::session_title::maybe_auto_title;
use crate::domain::stream_loop::{run_stream_loop, StreamAccumulator};
use crate::domain::system_prompt::{build_system_prompt, load_context_messages};
use crate::domain::tool_executor::project_root;
use crate::error::OrqaError;
use crate::repo::{message_repo, session_repo};
use crate::sidecar::types::SidecarRequest;
use crate::state::AppState;

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
            }
        }
        Err(e) => {
            tracing::warn!("[process] process_state mutex poisoned, skipping reset: {e}");
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

    let (governance_prompt, custom_prompt, enable_thinking) = match project_root(&state) {
        Ok(root) => {
            let gov = match build_system_prompt(&root) {
                Ok(prompt) => {
                    tracing::debug!("[stream] system prompt built ({} chars)", prompt.len());
                    Some(prompt)
                }
                Err(e) => {
                    tracing::warn!("[stream] failed to build system prompt: {e}");
                    None
                }
            };
            let (custom, thinking) = match read_settings(root.to_str().unwrap_or_default()) {
                Ok(Some(settings)) => (settings.custom_system_prompt, settings.show_thinking),
                _ => (None, false),
            };
            (gov, custom, thinking)
        }
        Err(e) => {
            tracing::warn!("[stream] no active project for system prompt: {e}");
            (None, None, false)
        }
    };

    // Assemble final system prompt: custom + governance
    let final_prompt = match (&custom_prompt, &governance_prompt) {
        (Some(c), Some(g)) => Some(format!("{c}\n\n---\n\n{g}")),
        (Some(c), None) => Some(c.clone()),
        (None, g) => g.clone(),
    };

    // Emit transparency event so the frontend can display what system prompt was sent.
    if let Some(ref prompt) = final_prompt {
        let total_chars = prompt.len() as i64;
        let _ = on_event.send(StreamEvent::SystemPromptSent {
            custom_prompt: custom_prompt.clone(),
            governance_prompt: governance_prompt.unwrap_or_default(),
            total_chars,
        });
    }

    // Look up persisted SDK session UUID for resume across restarts
    let sdk_session_id = {
        let db = state
            .db
            .lock()
            .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;
        session_repo::get(&db, session_id)
            .ok()
            .and_then(|s| s.sdk_session_id)
    };

    // ── Context injection for session continuity ──
    // When resuming a session that has an existing SDK session ID, load recent
    // message history from SQLite and append it to the system prompt as a
    // fallback. The SDK will also attempt resume via sdk_session_id, but if
    // local storage was cleared or the session expired, this ensures Claude
    // still has the conversation context it needs.
    let final_prompt = if sdk_session_id.is_some() {
        match load_context_messages(&state, session_id) {
            Some(context_messages) if !context_messages.is_empty() => {
                let msg_count = context_messages.len() as i32;
                let context_json = serde_json::to_string(&context_messages).unwrap_or_default();
                let total_chars = context_json.len() as i64;

                let history = context_messages
                    .iter()
                    .map(|m| format!("[{}]: {}", m.role, m.content))
                    .collect::<Vec<_>>()
                    .join("\n\n");

                let context_section = format!(
                    "## Conversation History\n\
                     The following is the recent conversation history from this session. \
                     Use it to maintain continuity.\n\n{history}"
                );

                tracing::debug!(
                    "[stream] injecting {} messages of context ({} chars)",
                    msg_count,
                    total_chars
                );

                let _ = on_event.send(StreamEvent::ContextInjected {
                    message_count: msg_count,
                    total_chars,
                    messages: context_json,
                });

                match final_prompt {
                    Some(p) => Some(format!("{p}\n\n---\n\n{context_section}")),
                    None => Some(context_section),
                }
            }
            _ => final_prompt,
        }
    } else {
        final_prompt
    };

    let request = SidecarRequest::SendMessage {
        session_id,
        content,
        model,
        system_prompt: final_prompt,
        sdk_session_id,
        enable_thinking,
    };
    state.sidecar.send(&request)?;

    let acc = run_stream_loop(&state, &on_event);

    persist_assistant_message(&state, session_id, turn_index, &acc)?;

    emit_process_violations(&state, &on_event);

    // Auto-generate session title after first successful turn.
    if acc.stream_complete && !acc.had_error {
        maybe_auto_title(&state, session_id, &on_event);
    }

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

/// Return the auto-generated governance system prompt for the active project.
///
/// This allows the Settings UI to show a preview of the governance prompt that
/// will be prepended to every conversation. Returns `Ok(None)` when no project
/// is currently active.
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
}
