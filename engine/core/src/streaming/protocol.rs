// Sidecar protocol types for the orqa-engine crate.
//
// Defines the NDJSON request/response protocol between the app (or any access
// layer) and a sidecar process. These types are shared across the app, daemon,
// and any future sidecar consumers so the protocol definition is authoritative
// in the engine rather than duplicated per-consumer.

use serde::{Deserialize, Serialize};

/// Request sent to the sidecar process via stdin as NDJSON.
///
/// Each variant is tagged with a `type` field using snake_case naming,
/// matching the echo sidecar and Agent SDK sidecar implementations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SidecarRequest {
    /// Send a user message and start an LLM inference turn.
    SendMessage {
        session_id: i64,
        content: String,
        model: Option<String>,
        system_prompt: Option<String>,
        provider_session_id: Option<String>,
        enable_thinking: bool,
    },
    /// Cancel the active streaming turn for the given session.
    CancelStream { session_id: i64 },
    /// Request a summary of a set of prior messages.
    GenerateSummary {
        session_id: i64,
        messages: Vec<MessageSummary>,
    },
    /// Check sidecar health status.
    HealthCheck,
    /// Return the result of a tool execution to the sidecar.
    ToolResult {
        tool_call_id: String,
        output: String,
        is_error: bool,
    },
    /// Respond to a tool approval request from the sidecar.
    ToolApproval {
        tool_call_id: String,
        approved: bool,
        reason: Option<String>,
    },
}

/// A condensed message used in `GenerateSummary` requests.
///
/// Carries role and content for the sidecar to summarise, without the full
/// message metadata needed for inference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSummary {
    pub role: String,
    pub content: String,
}

/// Response or event from the sidecar process via stdout as NDJSON.
///
/// Each variant is tagged with a `type` field using snake_case naming.
/// Streaming variants map closely to `StreamEvent`; additional variants
/// (health, summary, tool execution) are handled by the stream loop and
/// not forwarded to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SidecarResponse {
    /// Inference has started; carries a message ID and resolved model name.
    StreamStart {
        message_id: i64,
        resolved_model: Option<String>,
    },
    /// A chunk of generated text from the model.
    TextDelta { content: String },
    /// A chunk of model thinking/reasoning output.
    ThinkingDelta { content: String },
    /// The model has started using a tool.
    ToolUseStart {
        tool_call_id: String,
        tool_name: String,
    },
    /// Streaming JSON input for an in-progress tool call.
    ToolInputDelta {
        tool_call_id: String,
        content: String,
    },
    /// The model has received the result of a tool call.
    ToolResult {
        tool_call_id: String,
        tool_name: String,
        result: String,
        is_error: bool,
    },
    /// A content block (text or tool use) has completed.
    BlockComplete {
        block_index: i32,
        content_type: String,
    },
    /// The LLM turn has completed; carries token usage.
    TurnComplete {
        input_tokens: i64,
        output_tokens: i64,
    },
    /// A streaming error occurred.
    StreamError {
        code: String,
        message: String,
        recoverable: bool,
    },
    /// The stream was cancelled by the client.
    StreamCancelled,
    /// Response to a `HealthCheck` request.
    HealthOk { version: String },
    /// Response to a `GenerateSummary` request.
    SummaryResult { session_id: i64, summary: String },
    /// The sidecar requests the app to execute a tool on its behalf.
    ///
    /// The app executes the tool and sends a `ToolResult` request back.
    ToolExecute {
        tool_call_id: String,
        tool_name: String,
        input: String,
    },
    /// The sidecar requests user approval before executing a write/execute tool.
    ///
    /// The app shows an approval dialog and sends a `ToolApproval` request back.
    ToolApprovalRequest {
        tool_call_id: String,
        tool_name: String,
        input: String,
    },
    /// The sidecar has initialized a provider session.
    ///
    /// The `provider_session_id` can be stored and resumed on the next turn.
    SessionInitialized {
        session_id: i64,
        provider_session_id: String,
    },
}
