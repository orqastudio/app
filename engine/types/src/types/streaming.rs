// Streaming event domain types for the OrqaStudio engine.
//
// Defines the StreamEvent enum representing all events that flow from the sidecar
// through the Tauri Channel<T> to the frontend during an LLM inference session.
// Each variant maps to a TypeScript discriminated union on the frontend side.

use serde::{Deserialize, Serialize};

/// Streaming events that flow through `Channel<T>` from the sidecar to the frontend.
///
/// Each variant is tagged with a `type` field and optional `data` content,
/// matching the TypeScript `StreamEvent` discriminated union.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum StreamEvent {
    StreamStart {
        message_id: i64,
        resolved_model: Option<String>,
    },
    TextDelta {
        content: String,
    },
    ThinkingDelta {
        content: String,
    },
    ToolUseStart {
        tool_call_id: String,
        tool_name: String,
    },
    ToolInputDelta {
        tool_call_id: String,
        content: String,
    },
    ToolResult {
        tool_call_id: String,
        tool_name: String,
        result: String,
        is_error: bool,
    },
    BlockComplete {
        block_index: i32,
        content_type: String,
    },
    TurnComplete {
        input_tokens: i64,
        output_tokens: i64,
    },
    StreamError {
        code: String,
        message: String,
        recoverable: bool,
    },
    StreamCancelled,
    /// Emitted when the sidecar requests approval for a write or execute tool.
    ///
    /// The frontend must call `stream_tool_approval_respond` with the matching
    /// `tool_call_id` to unblock the stream loop.
    ToolApprovalRequest {
        tool_call_id: String,
        tool_name: String,
        /// JSON string of the tool parameters, for display in the UI.
        input: String,
    },
    /// Emitted after a turn completes when a process compliance violation is detected.
    ///
    /// Violations are warnings only — they do not block execution. The frontend
    /// should display them to draw attention to documentation-first process failures.
    ProcessViolation {
        /// Machine-readable check identifier (e.g. `"docs_before_code"`).
        check: String,
        /// Human-readable description of the violation.
        message: String,
    },
    /// Emitted when the session title is auto-generated from conversation content.
    ///
    /// Only fired when the title was not manually set by the user. The frontend
    /// should update the session title in its store without marking it as manual.
    SessionTitleUpdated {
        session_id: i64,
        title: String,
    },
    /// Emitted when the system prompt is sent to the model at the start of a turn.
    ///
    /// Carries both the optional user-supplied custom prompt and the governance
    /// prompt injected by the backend, plus the combined character count.
    SystemPromptSent {
        custom_prompt: Option<String>,
        governance_prompt: String,
        total_chars: i64,
    },
    /// Emitted when prior conversation messages are injected as context.
    ///
    /// The `messages` field is a JSON array of `{role, content}` objects suitable
    /// for rendering in a dialog on the frontend.
    ContextInjected {
        message_count: i32,
        total_chars: i64,
        /// JSON array of `{role, content}` objects for dialog display.
        messages: String,
    },
}
