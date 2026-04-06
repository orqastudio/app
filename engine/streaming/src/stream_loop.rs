//! Stream loop pure logic for the orqa-engine crate.
//!
//! Contains the Tauri-independent parts of the sidecar stream loop:
//! protocol translation (SidecarResponse -> StreamEvent), context overflow
//! detection, terminal event classification, and response accumulation.
//!
//! The Tauri-specific loop driver (which holds AppState, Channel<T>, and
//! calls execute_tool) remains in the app layer. That loop calls these
//! pure functions for translation and accumulation, keeping all business
//! logic testable without a Tauri context.

use crate::protocol::SidecarResponse;
use orqa_engine_types::types::streaming::StreamEvent;

/// Translate a context-overflow error code into a user-friendly message.
///
/// Returns `Some(friendly_message)` when the code indicates a context or token
/// limit error that should be surfaced to the user with a clear explanation.
pub fn friendly_context_overflow_message(code: &str, message: &str) -> Option<String> {
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
                .to_owned(),
        )
    } else {
        None
    }
}

/// Translate a `SidecarResponse` into a `StreamEvent`, if applicable.
///
/// Returns `None` for sidecar-specific responses (HealthOk, SummaryResult,
/// ToolExecute, SessionInitialized) that are handled by the stream loop and
/// not forwarded to the frontend.
pub fn translate_response(response: &SidecarResponse) -> Option<StreamEvent> {
    match response {
        SidecarResponse::StreamStart { .. }
        | SidecarResponse::TextDelta { .. }
        | SidecarResponse::ThinkingDelta { .. }
        | SidecarResponse::ToolUseStart { .. }
        | SidecarResponse::ToolInputDelta { .. }
        | SidecarResponse::ToolResult { .. }
        | SidecarResponse::BlockComplete { .. }
        | SidecarResponse::TurnComplete { .. } => translate_streaming_data(response),
        SidecarResponse::StreamError {
            code,
            message,
            recoverable,
        } => Some(translate_stream_error(code, message, *recoverable)),
        SidecarResponse::StreamCancelled => Some(StreamEvent::StreamCancelled),
        SidecarResponse::ToolApprovalRequest {
            tool_call_id,
            tool_name,
            input,
        } => Some(StreamEvent::ToolApprovalRequest {
            tool_call_id: tool_call_id.clone(),
            tool_name: tool_name.clone(),
            input: input.clone(),
        }),
        // Non-streaming responses and synchronous tool execution — not forwarded to frontend
        SidecarResponse::HealthOk { .. }
        | SidecarResponse::SummaryResult { .. }
        | SidecarResponse::SessionInitialized { .. }
        | SidecarResponse::ToolExecute { .. } => None,
    }
}

/// Translate content and lifecycle streaming variants to `StreamEvent`.
///
/// Handles the subset of variants that carry text, thinking, and turn
/// lifecycle data. Returns `None` for all other variants.
fn translate_content_events(response: &SidecarResponse) -> Option<StreamEvent> {
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
        _ => None,
    }
}

/// Translate tool-related streaming variants to `StreamEvent`.
///
/// Handles ToolUseStart, ToolInputDelta, and ToolResult variants.
/// Returns `None` for all other variants.
fn translate_tool_events(response: &SidecarResponse) -> Option<StreamEvent> {
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
        _ => None,
    }
}

/// Translate streaming data variants that map 1:1 from `SidecarResponse` to `StreamEvent`.
///
/// Tries content events first, then tool events. Returns `None` if neither matches.
fn translate_streaming_data(response: &SidecarResponse) -> Option<StreamEvent> {
    translate_content_events(response).or_else(|| translate_tool_events(response))
}

/// Translate a stream error, replacing context-overflow messages with user-friendly text.
///
/// When the error code or message indicates a context/token limit, the raw
/// message is replaced with a clear explanation of what the user should do.
fn translate_stream_error(code: &str, message: &str, recoverable: bool) -> StreamEvent {
    let user_message =
        friendly_context_overflow_message(code, message).unwrap_or_else(|| message.to_owned());
    StreamEvent::StreamError {
        code: code.to_owned(),
        message: user_message,
        recoverable,
    }
}

/// Return `true` if this response is a terminal event.
///
/// Terminal events end the stream loop: turn complete, stream error, or
/// stream cancelled. The loop should not continue reading after a terminal event.
pub fn is_terminal(response: &SidecarResponse) -> bool {
    matches!(
        response,
        SidecarResponse::TurnComplete { .. }
            | SidecarResponse::StreamError { .. }
            | SidecarResponse::StreamCancelled
    )
}

/// Accumulated state from the sidecar read loop.
///
/// Collects text content and token usage as responses arrive. The stream
/// loop updates this on each response and returns the final state when
/// the stream terminates.
pub struct StreamAccumulator {
    /// All `TextDelta` content concatenated in arrival order.
    pub text: String,
    /// Input token count from the final `TurnComplete` response.
    pub input_tokens: i64,
    /// Output token count from the final `TurnComplete` response.
    pub output_tokens: i64,
    /// Set to `true` when a `TurnComplete` response is received.
    pub stream_complete: bool,
    /// Set to `true` when a `StreamError` or `StreamCancelled` response is received.
    pub had_error: bool,
}

/// Update the accumulator with data from a streaming response.
///
/// Appends text deltas, records token counts from turn-complete, and flags
/// errors. Called once per response in the stream loop.
pub fn accumulate_response(response: &SidecarResponse, acc: &mut StreamAccumulator) {
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

#[cfg(test)]
mod tests {
    use super::*;

    // ── translate_response tests ──

    #[test]
    fn translate_stream_start() {
        let resp = SidecarResponse::StreamStart {
            message_id: 42,
            resolved_model: Some("claude-opus-4-6".to_owned()),
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
            content: "Hello ".to_owned(),
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
            content: "Let me consider...".to_owned(),
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
            tool_call_id: "call_001".to_owned(),
            tool_name: "read_file".to_owned(),
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
            tool_call_id: "call_001".to_owned(),
            content: r#"{"path":"#.to_owned(),
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
            tool_call_id: "call_001".to_owned(),
            tool_name: "read_file".to_owned(),
            result: "file contents".to_owned(),
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
            content_type: "text".to_owned(),
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
    fn translate_stream_error_event() {
        let resp = SidecarResponse::StreamError {
            code: "rate_limit".to_owned(),
            message: "Too many requests".to_owned(),
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
            version: "0.1.0".to_owned(),
        };
        assert!(translate_response(&resp).is_none());
    }

    #[test]
    fn translate_summary_result_returns_none() {
        let resp = SidecarResponse::SummaryResult {
            session_id: 1,
            summary: "a summary".to_owned(),
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
            code: "err".to_owned(),
            message: "failed".to_owned(),
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
            content: "hello".to_owned(),
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
            content_type: "text".to_owned(),
        };
        assert!(!is_terminal(&resp));
    }

    #[test]
    fn health_ok_is_not_terminal() {
        let resp = SidecarResponse::HealthOk {
            version: "1.0".to_owned(),
        };
        assert!(!is_terminal(&resp));
    }

    // ── ToolExecute / ToolApprovalRequest tests ──

    #[test]
    fn translate_tool_execute_returns_none() {
        let resp = SidecarResponse::ToolExecute {
            tool_call_id: "call_010".to_owned(),
            tool_name: "read_file".to_owned(),
            input: r#"{"path":"/src/main.rs"}"#.to_owned(),
        };
        assert!(translate_response(&resp).is_none());
    }

    #[test]
    fn translate_tool_approval_request_returns_event() {
        let resp = SidecarResponse::ToolApprovalRequest {
            tool_call_id: "call_011".to_owned(),
            tool_name: "write_file".to_owned(),
            input: r#"{"path":"/tmp/out.txt"}"#.to_owned(),
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
            tool_call_id: "call_010".to_owned(),
            tool_name: "read_file".to_owned(),
            input: "{}".to_owned(),
        };
        assert!(!is_terminal(&resp));
    }

    #[test]
    fn tool_approval_request_is_not_terminal() {
        let resp = SidecarResponse::ToolApprovalRequest {
            tool_call_id: "call_011".to_owned(),
            tool_name: "write_file".to_owned(),
            input: "{}".to_owned(),
        };
        assert!(!is_terminal(&resp));
    }

    // ── Content accumulation ──

    #[test]
    fn accumulate_text_deltas() {
        let responses = vec![
            SidecarResponse::StreamStart {
                message_id: 1,
                resolved_model: None,
            },
            SidecarResponse::TextDelta {
                content: "Hello".to_owned(),
            },
            SidecarResponse::TextDelta {
                content: ", ".to_owned(),
            },
            SidecarResponse::TextDelta {
                content: "world!".to_owned(),
            },
            SidecarResponse::BlockComplete {
                block_index: 0,
                content_type: "text".to_owned(),
            },
            SidecarResponse::TurnComplete {
                input_tokens: 100,
                output_tokens: 50,
            },
        ];

        let mut acc = StreamAccumulator {
            text: String::new(),
            input_tokens: 0,
            output_tokens: 0,
            stream_complete: false,
            had_error: false,
        };
        for resp in &responses {
            accumulate_response(resp, &mut acc);
        }

        assert_eq!(acc.text, "Hello, world!");
        assert_eq!(acc.input_tokens, 100);
        assert_eq!(acc.output_tokens, 50);
        assert!(acc.stream_complete);
        assert!(!acc.had_error);
    }

    #[test]
    fn accumulate_error_flags_had_error() {
        let mut acc = StreamAccumulator {
            text: String::new(),
            input_tokens: 0,
            output_tokens: 0,
            stream_complete: false,
            had_error: false,
        };
        accumulate_response(
            &SidecarResponse::StreamError {
                code: "err".to_owned(),
                message: "fail".to_owned(),
                recoverable: false,
            },
            &mut acc,
        );
        assert!(acc.had_error);
        assert!(!acc.stream_complete);
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
            code: "context_length_exceeded".to_owned(),
            message: "Input is too long".to_owned(),
            recoverable: false,
        };
        let event = translate_response(&resp).expect("should translate");
        match event {
            StreamEvent::StreamError { code, message, .. } => {
                assert_eq!(code, "context_length_exceeded");
                assert!(message.contains("context window"));
            }
            _ => panic!("expected StreamError"),
        }
    }

    #[test]
    fn translate_stream_error_normal_error_keeps_original_message() {
        let resp = SidecarResponse::StreamError {
            code: "rate_limit".to_owned(),
            message: "Too many requests".to_owned(),
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
