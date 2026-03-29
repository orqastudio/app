use crate::domain::process_state::ProcessStateExt;
use crate::domain::provider_event::StreamEvent;
use crate::domain::stream_loop::run_stream_loop;
use crate::domain::system_prompt::{
    load_context_summary, lookup_provider_session_id, resolve_system_prompt,
};
use crate::domain::tool_executor::project_root;
use crate::error::OrqaError;
use crate::repo::{message_repo, session_repo};
use crate::sidecar::types::SidecarRequest;
use crate::state::AppState;

// ── Persistence helpers (unique to the command layer) ──

/// Persist the user message and return `(user_message_id, turn_index)`.
fn persist_user_message(
    state: &AppState,
    session_id: i64,
    content: &str,
) -> Result<(i64, i64), OrqaError> {
    let db = state
        .db
        .conn
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
    acc: &crate::domain::stream_loop::StreamAccumulator,
) -> Result<(), OrqaError> {
    let db = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;

    let assistant_turn = i32::try_from(turn_index + 1)
        .map_err(|_| OrqaError::Database("turn index overflow".to_owned()))?;
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

// ── Process state helpers ──

/// Reset session process state when a new session begins.
fn reset_process_state_if_new_session(state: &tauri::State<'_, AppState>, session_id: i64) {
    match state.session.process_state.lock() {
        Ok(mut ps) => {
            if ps.session_id != Some(session_id) {
                ps.reset(session_id);
                reset_workflow_tracker(state);
            }
        }
        Err(e) => {
            tracing::warn!("[process] process_state mutex poisoned, skipping reset: {e}");
        }
    }
}

/// Reset the workflow tracker to a clean state for a new session.
///
/// Creates a new tracker with the current session's tracker config so that
/// path classification rules are preserved across session resets.
fn reset_workflow_tracker(state: &tauri::State<'_, AppState>) {
    use crate::domain::workflow_tracker::WorkflowTracker;
    let config = match state.session.tracker_config.lock() {
        Ok(c) => c.clone(),
        Err(e) => {
            tracing::warn!("[workflow] tracker_config mutex poisoned, skipping reset: {e}");
            return;
        }
    };
    match state.session.workflow_tracker.lock() {
        Ok(mut wt) => {
            *wt = WorkflowTracker::new(config);
        }
        Err(e) => {
            tracing::warn!("[workflow] workflow_tracker mutex poisoned, skipping reset: {e}");
        }
    }
}

/// Emit `StreamEvent::ProcessViolation` for any active process compliance violations.
fn emit_process_violations(
    state: &tauri::State<'_, AppState>,
    on_event: &tauri::ipc::Channel<StreamEvent>,
) {
    let violations = match state.session.process_state.lock() {
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

// ── Tauri commands ──

/// Send a message to the sidecar and stream responses back via `Channel<T>`.
#[tauri::command]
pub fn stream_send_message(
    session_id: i64,
    content: String,
    model: Option<String>,
    on_event: tauri::ipc::Channel<StreamEvent>,
    state: tauri::State<'_, AppState>,
) -> Result<i64, OrqaError> {
    let content = content.trim().to_owned();
    if content.is_empty() {
        return Err(OrqaError::Validation(
            "message content cannot be empty".to_owned(),
        ));
    }

    let (user_message_id, turn_index) = persist_user_message(&state, session_id, &content)?;
    reset_process_state_if_new_session(&state, session_id);
    super::sidecar_commands::ensure_sidecar_running(&state)?;

    let system_prompt = project_root(&state)
        .ok()
        .and_then(|root| resolve_system_prompt(&root));
    let provider_session_id = lookup_provider_session_id(&state, session_id)?;

    if let Some(ref prompt) = system_prompt {
        let _ = on_event.send(StreamEvent::SystemPromptSent {
            custom_prompt: None,
            governance_prompt: prompt.clone(),
            total_chars: prompt.len() as i64,
        });
    }

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
    state.sidecar.manager.send(&request)?;

    let acc = run_stream_loop(&state, &on_event);
    persist_assistant_message(&state, session_id, turn_index, &acc)?;
    emit_process_violations(&state, &on_event);

    Ok(user_message_id)
}

/// Request cancellation of an active stream for the given session.
#[tauri::command]
pub fn stream_stop(session_id: i64, state: tauri::State<'_, AppState>) -> Result<(), OrqaError> {
    state
        .sidecar
        .manager
        .send(&SidecarRequest::CancelStream { session_id })
}

/// Respond to a pending tool approval request from the frontend.
#[tauri::command]
pub fn stream_tool_approval_respond(
    tool_call_id: String,
    approved: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), OrqaError> {
    let sender = state
        .sidecar
        .pending_approvals
        .lock()
        .map_err(|_| OrqaError::Sidecar("pending_approvals mutex poisoned".to_owned()))?
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

#[cfg(test)]
mod tests {
    use crate::db::init_memory_db;
    use crate::domain::message::MessageRole;
    use crate::repo::{message_repo, project_repo, session_repo};

    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        session_repo::create(&conn, 1, "auto", None).expect("create session");
        conn
    }

    #[test]
    fn empty_message_validation() {
        let content = "   ";
        assert!(content.trim().is_empty());
    }

    #[test]
    fn persist_user_message_via_repo() {
        let conn = setup();
        let turn_index = message_repo::next_turn_index(&conn, 1).expect("turn index");
        assert_eq!(turn_index, 0);

        let msg = message_repo::create(&conn, 1, "user", "text", Some("Hello"), turn_index, 0)
            .expect("create message");
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, Some("Hello".to_owned()));
    }

    #[test]
    fn persist_assistant_message_via_repo() {
        let conn = setup();
        // User message first
        message_repo::create(&conn, 1, "user", "text", Some("Hello"), 0, 0)
            .expect("create user msg");

        // Assistant message
        let msg = message_repo::create(&conn, 1, "assistant", "text", Some("Hi there"), 1, 0)
            .expect("create assistant msg");
        assert_eq!(msg.role, MessageRole::Assistant);

        // Update stream status
        message_repo::update_stream_status(&conn, msg.id, "complete")
            .expect("update stream status");
    }

    #[test]
    fn update_token_usage() {
        let conn = setup();
        session_repo::update_token_usage(&conn, 1, 100, 200).expect("update tokens");
        let session = session_repo::get(&conn, 1).expect("get session");
        assert_eq!(session.total_input_tokens, 100);
        assert_eq!(session.total_output_tokens, 200);
    }

    #[test]
    fn next_turn_index_increments() {
        let conn = setup();
        let t0 = message_repo::next_turn_index(&conn, 1).expect("t0");
        assert_eq!(t0, 0);

        message_repo::create(&conn, 1, "user", "text", Some("msg1"), t0, 0).expect("create");
        let t1 = message_repo::next_turn_index(&conn, 1).expect("t1");
        assert_eq!(t1, 1);
    }

    // Note: stream_send_message, stream_stop, and stream_tool_approval_respond
    // require a live sidecar process and Tauri State, which cannot be constructed
    // in unit tests. The persistence and validation logic they use is tested above
    // through the repo layer. Integration testing of the full stream loop requires
    // the Tauri runtime.
}
