use crate::domain::provider_event::StreamEvent;
use crate::error::ForgeError;
use crate::repo::{message_repo, session_repo};
use crate::sidecar::types::{SidecarRequest, SidecarResponse};
use crate::state::AppState;

/// Translate a `SidecarResponse` into a `StreamEvent`, if applicable.
///
/// Returns `None` for sidecar-specific responses (HealthOk, SummaryResult)
/// that are not part of the streaming conversation flow.
fn translate_response(response: &SidecarResponse) -> Option<StreamEvent> {
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
        SidecarResponse::StreamError {
            code,
            message,
            recoverable,
        } => Some(StreamEvent::StreamError {
            code: code.clone(),
            message: message.clone(),
            recoverable: *recoverable,
        }),
        SidecarResponse::StreamCancelled => Some(StreamEvent::StreamCancelled),
        // Non-streaming responses — not forwarded to the frontend channel
        SidecarResponse::HealthOk { .. } | SidecarResponse::SummaryResult { .. } => None,
    }
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

/// Send a message to the sidecar and stream responses back via `Channel<T>`.
///
/// This command:
/// 1. Validates the input content is not empty
/// 2. Persists the user message to SQLite
/// 3. Sends the message to the sidecar via NDJSON stdin
/// 4. Reads sidecar responses in a blocking loop, forwarding each as a `StreamEvent`
/// 5. Accumulates text content for the assistant message
/// 6. On completion, persists the assistant message and updates session token usage
///
/// The DB mutex is only held briefly for persistence operations, never during
/// the sidecar read loop.
#[tauri::command]
pub fn stream_send_message(
    session_id: i64,
    content: String,
    on_event: tauri::ipc::Channel<StreamEvent>,
    state: tauri::State<'_, AppState>,
) -> Result<i64, ForgeError> {
    // 1. Validate content
    let content = content.trim().to_string();
    if content.is_empty() {
        return Err(ForgeError::Validation(
            "message content cannot be empty".to_string(),
        ));
    }

    // 2. Lock DB briefly: verify session exists, get turn index, persist user message
    let (user_message_id, turn_index) = {
        let db = state
            .db
            .lock()
            .map_err(|e| ForgeError::Database(format!("failed to acquire db lock: {e}")))?;

        // Verify session exists
        session_repo::get(&db, session_id)?;

        // Get next turn index
        let turn_index = message_repo::next_turn_index(&db, session_id)?;

        // Create user message
        let user_msg = message_repo::create(
            &db,
            session_id,
            "user",
            "text",
            Some(&content),
            turn_index,
            0,
        )?;

        (user_msg.id, turn_index)
    };
    // DB lock released here

    // 3. Send message to sidecar
    let request = SidecarRequest::SendMessage {
        session_id,
        content,
        model: None,
        system_prompt: None,
    };
    state.sidecar.send(&request)?;

    // 4. Read response loop — no DB lock held
    let mut accumulated_text = String::new();
    let mut input_tokens: i64 = 0;
    let mut output_tokens: i64 = 0;
    let mut stream_complete = false;
    let mut had_error = false;

    loop {
        let response = match state.sidecar.read_line() {
            Ok(Some(resp)) => resp,
            Ok(None) => {
                // EOF — sidecar closed stdout unexpectedly
                let error_event = StreamEvent::StreamError {
                    code: "sidecar_eof".to_string(),
                    message: "sidecar process closed unexpectedly".to_string(),
                    recoverable: false,
                };
                let _ = on_event.send(error_event);
                had_error = true;
                break;
            }
            Err(e) => {
                let error_event = StreamEvent::StreamError {
                    code: "sidecar_read_error".to_string(),
                    message: e.to_string(),
                    recoverable: false,
                };
                let _ = on_event.send(error_event);
                had_error = true;
                break;
            }
        };

        // Accumulate text content
        if let SidecarResponse::TextDelta { ref content } = response {
            accumulated_text.push_str(content);
        }

        // Capture token usage
        if let SidecarResponse::TurnComplete {
            input_tokens: inp,
            output_tokens: out,
        } = &response
        {
            input_tokens = *inp;
            output_tokens = *out;
            stream_complete = true;
        }

        // Track error/cancel
        if matches!(
            response,
            SidecarResponse::StreamError { .. } | SidecarResponse::StreamCancelled
        ) {
            had_error = true;
        }

        let terminal = is_terminal(&response);

        // Translate and send to frontend channel
        if let Some(event) = translate_response(&response) {
            let _ = on_event.send(event);
        }

        if terminal {
            break;
        }
    }

    // 5. Lock DB briefly: persist assistant message and update token usage
    {
        let db = state
            .db
            .lock()
            .map_err(|e| ForgeError::Database(format!("failed to acquire db lock: {e}")))?;

        let assistant_turn = turn_index + 1;
        let content_value = if accumulated_text.is_empty() {
            None
        } else {
            Some(accumulated_text.as_str())
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

        // Set stream status based on outcome
        let status = if stream_complete && !had_error {
            "complete"
        } else {
            "error"
        };
        message_repo::update_stream_status(&db, assistant_msg.id, status)?;

        // Update session token usage if we got a TurnComplete
        if stream_complete {
            session_repo::update_token_usage(&db, session_id, input_tokens, output_tokens)?;
        }
    }
    // DB lock released here

    Ok(user_message_id)
}

/// Request cancellation of an active stream for the given session.
///
/// Sends a `CancelStream` request to the sidecar. The sidecar will respond
/// with a `StreamCancelled` event, which the read loop in `stream_send_message`
/// will handle.
#[tauri::command]
pub fn stream_stop(session_id: i64, state: tauri::State<'_, AppState>) -> Result<(), ForgeError> {
    state
        .sidecar
        .send(&SidecarRequest::CancelStream { session_id })
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
}
