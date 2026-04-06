// Tauri IPC commands for streaming AI responses.
//
// All stream operations are delegated to the daemon via HTTP. The app is a
// pure SSE consumer — the daemon manages the claude child process, runs the
// stream loop, dispatches tools, and tracks enforcement and workflow state.
//
// Flow:
//   1. stream_send_message  → POST /sessions/:id/messages (daemon starts loop)
//   2. (Frontend subscribes to SSE via JS EventSource — not handled here)
//   3. stream_stop          → POST /sessions/:id/stop
//   4. stream_tool_approval_respond → POST /sessions/:id/tool-approval
//
// The stream_subscribe_events command subscribes to the daemon SSE stream and
// forwards each event to the Tauri frontend channel, bridging the HTTP SSE
// stream and the Tauri IPC channel.

use serde::Deserialize;
use tauri::State;

use crate::domain::provider_event::StreamEvent;
use crate::error::OrqaError;
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Daemon request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /sessions/:id/messages.
#[derive(serde::Serialize)]
struct SendMessageRequest {
    content: String,
    model: Option<String>,
}

/// Response body from POST /sessions/:id/messages.
#[derive(Deserialize)]
struct SendMessageResponse {
    /// The newly-created user message ID.
    pub user_message_id: i64,
}

/// Request body for POST /sessions/:id/tool-approval.
#[derive(serde::Serialize)]
struct ToolApprovalRequest {
    tool_call_id: String,
    approved: bool,
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

/// Send a message to the daemon stream loop and subscribe to SSE events.
///
/// POSTs the message to the daemon, which starts the claude sidecar in-process.
/// Then subscribes to GET /sessions/:id/stream and forwards each SSE event to
/// the Tauri frontend channel until the stream ends.
#[allow(clippy::too_many_lines)]
#[tauri::command]
pub async fn stream_send_message(
    session_id: i64,
    content: String,
    model: Option<String>,
    on_event: tauri::ipc::Channel<StreamEvent>,
    state: State<'_, AppState>,
) -> Result<i64, OrqaError> {
    let content = content.trim().to_owned();
    if content.is_empty() {
        return Err(OrqaError::Validation(
            "message content cannot be empty".to_owned(),
        ));
    }

    tracing::debug!(
        session_id = session_id,
        content_len = content.len(),
        "[stream] stream_send_message: delegating to daemon"
    );

    let client = state.daemon.client.clone();
    let base_url = client.base_url().to_owned();
    let reqwest_client = client.reqwest_client().clone();

    // POST the message to the daemon — this starts the stream loop.
    let send_url = format!("{base_url}/sessions/{session_id}/messages");
    let body = SendMessageRequest { content, model };

    let post_response = reqwest_client
        .post(&send_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable: {e}")))?;

    if !post_response.status().is_success() {
        let status = post_response.status();
        let text = post_response.text().await.unwrap_or_default();
        return Err(OrqaError::Sidecar(format!(
            "daemon returned HTTP {status}: {text}"
        )));
    }

    let send_resp: SendMessageResponse = post_response
        .json()
        .await
        .map_err(|e| OrqaError::Sidecar(format!("failed to parse send_message response: {e}")))?;
    let user_message_id = send_resp.user_message_id;

    // Subscribe to the SSE stream and forward events to the Tauri channel.
    let stream_url = format!("{base_url}/sessions/{session_id}/stream");
    let sse_response = reqwest_client
        .get(&stream_url)
        .header("Accept", "text/event-stream")
        .send()
        .await
        .map_err(|e| OrqaError::Sidecar(format!("failed to connect to SSE stream: {e}")))?;

    if !sse_response.status().is_success() {
        let status = sse_response.status();
        return Err(OrqaError::Sidecar(format!(
            "daemon SSE stream returned HTTP {status}"
        )));
    }

    // Read SSE lines and forward deserialized events to the Tauri channel.
    // The SSE format is: lines starting with "data: " contain JSON payloads.
    // We stop when the stream closes or a terminal event is received.
    use futures_util::StreamExt as _;
    let mut byte_stream = sse_response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = byte_stream.next().await {
        let chunk = chunk.map_err(|e| OrqaError::Sidecar(format!("SSE read error: {e}")))?;
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        // Process complete lines from the buffer.
        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim_end_matches('\r').to_owned();
            buffer = buffer[newline_pos + 1..].to_owned();

            if let Some(data) = line.strip_prefix("data: ") {
                if data == "[DONE]" {
                    return Ok(user_message_id);
                }

                // Parse the SSE payload: {"event_type": "...", "payload": {...}}
                match serde_json::from_str::<serde_json::Value>(data) {
                    Ok(val) => {
                        let event_type = val["event_type"].as_str().unwrap_or("").to_owned();
                        let payload = val["payload"].clone();

                        // Convert the payload to a StreamEvent by reconstructing the
                        // tagged enum format: {"type": "<snake>", "data": {...}}.
                        // The daemon's event_type is PascalCase; StreamEvent serde tag
                        // is snake_case, so we convert.
                        let snake = pascal_to_snake(&event_type);
                        let tagged = serde_json::json!({"type": snake, "data": payload});

                        match serde_json::from_value::<StreamEvent>(tagged) {
                            Ok(event) => {
                                let is_terminal = matches!(
                                    event,
                                    StreamEvent::TurnComplete { .. }
                                        | StreamEvent::StreamError { .. }
                                        | StreamEvent::StreamCancelled
                                );
                                let _ = on_event.send(event);
                                if is_terminal {
                                    return Ok(user_message_id);
                                }
                            }
                            Err(e) => {
                                tracing::warn!(
                                    event_type = %event_type,
                                    error = %e,
                                    "[stream] failed to deserialize SSE event, skipping"
                                );
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!(data = %data, error = %e, "[stream] invalid SSE JSON");
                    }
                }
            }
        }
    }

    Ok(user_message_id)
}

/// Request cancellation of an active stream for the given session.
#[tauri::command]
pub async fn stream_stop(session_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    tracing::info!(
        session_id = session_id,
        "[stream] stream_stop: delegating to daemon"
    );
    let client = state.daemon.client.clone();
    let base_url = client.base_url().to_owned();
    let reqwest_client = client.reqwest_client().clone();

    let url = format!("{base_url}/sessions/{session_id}/stop");
    let response = reqwest_client
        .post(&url)
        .send()
        .await
        .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(OrqaError::Sidecar(format!(
            "daemon returned HTTP {status}: {text}"
        )));
    }

    Ok(())
}

/// Respond to a pending tool approval request from the frontend.
///
/// Forwards the approval decision to the daemon, which unblocks the stream loop.
#[tauri::command]
pub async fn stream_tool_approval_respond(
    session_id: i64,
    tool_call_id: String,
    approved: bool,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    tracing::info!(
        session_id = session_id,
        tool_call_id = %tool_call_id,
        approved = approved,
        "[stream] stream_tool_approval_respond: delegating to daemon"
    );

    let client = state.daemon.client.clone();
    let base_url = client.base_url().to_owned();
    let reqwest_client = client.reqwest_client().clone();

    let url = format!("{base_url}/sessions/{session_id}/tool-approval");
    let body = ToolApprovalRequest {
        tool_call_id,
        approved,
    };

    let response = reqwest_client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(OrqaError::Sidecar(format!(
            "daemon returned HTTP {status}: {text}"
        )));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a PascalCase string to snake_case.
///
/// Used to map daemon SSE event_type (PascalCase) to the StreamEvent serde tag
/// (snake_case). For example: "StreamStart" -> "stream_start".
fn pascal_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_to_snake_basic() {
        assert_eq!(pascal_to_snake("StreamStart"), "stream_start");
        assert_eq!(pascal_to_snake("TurnComplete"), "turn_complete");
        assert_eq!(pascal_to_snake("TextDelta"), "text_delta");
        assert_eq!(pascal_to_snake("StreamError"), "stream_error");
        assert_eq!(pascal_to_snake("ToolUseStart"), "tool_use_start");
        assert_eq!(pascal_to_snake("ToolInputDelta"), "tool_input_delta");
        assert_eq!(pascal_to_snake("StreamCancelled"), "stream_cancelled");
        assert_eq!(
            pascal_to_snake("ToolApprovalRequest"),
            "tool_approval_request"
        );
    }

    #[test]
    fn pascal_to_snake_single_word() {
        assert_eq!(pascal_to_snake("Stream"), "stream");
        assert_eq!(pascal_to_snake("Complete"), "complete");
    }

    #[test]
    fn empty_content_validation() {
        let content = "   ";
        assert!(content.trim().is_empty());
    }
}
