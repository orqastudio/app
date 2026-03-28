//! Sidecar protocol types for the orqa-engine crate.
//!
//! Defines the NDJSON request/response protocol between the app (or any access
//! layer) and a sidecar process. These types are shared across the app, daemon,
//! and any future sidecar consumers so the protocol definition is authoritative
//! in the engine rather than duplicated per-consumer.

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
        /// Session identifier for this inference turn.
        session_id: i64,
        /// The user message content to send.
        content: String,
        /// Optional model override; uses the session default when absent.
        model: Option<String>,
        /// Optional system prompt prepended to the conversation.
        system_prompt: Option<String>,
        /// Provider session ID for resuming a prior provider context.
        provider_session_id: Option<String>,
        /// Whether to enable model thinking/reasoning output.
        enable_thinking: bool,
    },
    /// Cancel the active streaming turn for the given session.
    CancelStream {
        /// Session identifier whose active stream should be cancelled.
        session_id: i64,
    },
    /// Request a summary of a set of prior messages.
    GenerateSummary {
        /// Session identifier for this summary request.
        session_id: i64,
        /// Messages to summarise.
        messages: Vec<MessageSummary>,
    },
    /// Check sidecar health status.
    HealthCheck,
    /// Return the result of a tool execution to the sidecar.
    ToolResult {
        /// Identifier of the tool call this result belongs to.
        tool_call_id: String,
        /// The tool output string.
        output: String,
        /// Whether the tool execution produced an error.
        is_error: bool,
    },
    /// Respond to a tool approval request from the sidecar.
    ToolApproval {
        /// Identifier of the tool call this approval belongs to.
        tool_call_id: String,
        /// Whether the user approved the tool execution.
        approved: bool,
        /// Optional reason when approval was denied.
        reason: Option<String>,
    },
}

/// A condensed message used in `GenerateSummary` requests.
///
/// Carries role and content for the sidecar to summarise, without the full
/// message metadata needed for inference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSummary {
    /// The message role (e.g. "user", "assistant").
    pub role: String,
    /// The message content text.
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
        /// Unique identifier for this message in the session.
        message_id: i64,
        /// The model name actually used for inference, if resolved.
        resolved_model: Option<String>,
    },
    /// A chunk of generated text from the model.
    TextDelta {
        /// The text content of this delta chunk.
        content: String,
    },
    /// A chunk of model thinking/reasoning output.
    ThinkingDelta {
        /// The thinking content of this delta chunk.
        content: String,
    },
    /// The model has started using a tool.
    ToolUseStart {
        /// Identifier for this tool call, used to match deltas and results.
        tool_call_id: String,
        /// Name of the tool being invoked.
        tool_name: String,
    },
    /// Streaming JSON input for an in-progress tool call.
    ToolInputDelta {
        /// Identifier of the tool call this input fragment belongs to.
        tool_call_id: String,
        /// Partial JSON input fragment.
        content: String,
    },
    /// The model has received the result of a tool call.
    ToolResult {
        /// Identifier of the tool call this result belongs to.
        tool_call_id: String,
        /// Name of the tool that was executed.
        tool_name: String,
        /// The tool output string.
        result: String,
        /// Whether the tool execution produced an error.
        is_error: bool,
    },
    /// A content block (text or tool use) has completed.
    BlockComplete {
        /// Zero-based index of the completed content block.
        block_index: i32,
        /// Type of the completed block (e.g. "text", "tool_use").
        content_type: String,
    },
    /// The LLM turn has completed; carries token usage.
    TurnComplete {
        /// Number of input tokens consumed this turn.
        input_tokens: i64,
        /// Number of output tokens generated this turn.
        output_tokens: i64,
    },
    /// A streaming error occurred.
    StreamError {
        /// Machine-readable error code.
        code: String,
        /// Human-readable error description.
        message: String,
        /// Whether the caller may retry this operation.
        recoverable: bool,
    },
    /// The stream was cancelled by the client.
    StreamCancelled,
    /// Response to a `HealthCheck` request.
    HealthOk {
        /// Sidecar version string.
        version: String,
    },
    /// Response to a `GenerateSummary` request.
    SummaryResult {
        /// Session identifier this summary belongs to.
        session_id: i64,
        /// The generated summary text.
        summary: String,
    },
    /// The sidecar requests the app to execute a tool on its behalf.
    ///
    /// The app executes the tool and sends a `ToolResult` request back.
    ToolExecute {
        /// Identifier for this tool call.
        tool_call_id: String,
        /// Name of the tool to execute.
        tool_name: String,
        /// JSON-encoded input arguments for the tool.
        input: String,
    },
    /// The sidecar requests user approval before executing a write/execute tool.
    ///
    /// The app shows an approval dialog and sends a `ToolApproval` request back.
    ToolApprovalRequest {
        /// Identifier for this tool call.
        tool_call_id: String,
        /// Name of the tool awaiting approval.
        tool_name: String,
        /// JSON-encoded input arguments for the tool.
        input: String,
    },
    /// The sidecar has initialized a provider session.
    ///
    /// The `provider_session_id` can be stored and resumed on the next turn.
    SessionInitialized {
        /// Session identifier that was initialized.
        session_id: i64,
        /// Provider-specific session token for resumption.
        provider_session_id: String,
    },
}
