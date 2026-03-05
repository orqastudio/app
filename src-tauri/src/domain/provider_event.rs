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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_start_serialization() {
        let event = StreamEvent::StreamStart {
            message_id: 42,
            resolved_model: Some("claude-opus-4-6".to_string()),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "stream_start");
        assert_eq!(json["data"]["message_id"], 42);
        assert_eq!(json["data"]["resolved_model"], "claude-opus-4-6");
    }

    #[test]
    fn text_delta_serialization() {
        let event = StreamEvent::TextDelta {
            content: "Hello ".to_string(),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "text_delta");
        assert_eq!(json["data"]["content"], "Hello ");
    }

    #[test]
    fn thinking_delta_serialization() {
        let event = StreamEvent::ThinkingDelta {
            content: "Let me think...".to_string(),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "thinking_delta");
        assert_eq!(json["data"]["content"], "Let me think...");
    }

    #[test]
    fn tool_use_start_serialization() {
        let event = StreamEvent::ToolUseStart {
            tool_call_id: "call_abc123".to_string(),
            tool_name: "read_file".to_string(),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_use_start");
        assert_eq!(json["data"]["tool_call_id"], "call_abc123");
        assert_eq!(json["data"]["tool_name"], "read_file");
    }

    #[test]
    fn tool_input_delta_serialization() {
        let event = StreamEvent::ToolInputDelta {
            tool_call_id: "call_abc123".to_string(),
            content: r#"{"path": "/src"#.to_string(),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_input_delta");
        assert_eq!(json["data"]["tool_call_id"], "call_abc123");
    }

    #[test]
    fn tool_result_serialization() {
        let event = StreamEvent::ToolResult {
            tool_call_id: "call_abc123".to_string(),
            tool_name: "read_file".to_string(),
            result: "file contents here".to_string(),
            is_error: false,
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_result");
        assert!(!json["data"]["is_error"]
            .as_bool()
            .expect("should be a bool"));
    }

    #[test]
    fn block_complete_serialization() {
        let event = StreamEvent::BlockComplete {
            block_index: 0,
            content_type: "text".to_string(),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "block_complete");
        assert_eq!(json["data"]["block_index"], 0);
        assert_eq!(json["data"]["content_type"], "text");
    }

    #[test]
    fn turn_complete_serialization() {
        let event = StreamEvent::TurnComplete {
            input_tokens: 1500,
            output_tokens: 800,
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "turn_complete");
        assert_eq!(json["data"]["input_tokens"], 1500);
        assert_eq!(json["data"]["output_tokens"], 800);
    }

    #[test]
    fn stream_error_serialization() {
        let event = StreamEvent::StreamError {
            code: "rate_limit".to_string(),
            message: "Too many requests".to_string(),
            recoverable: true,
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "stream_error");
        assert_eq!(json["data"]["code"], "rate_limit");
        assert!(json["data"]["recoverable"]
            .as_bool()
            .expect("should be a bool"));
    }

    #[test]
    fn stream_cancelled_serialization() {
        let event = StreamEvent::StreamCancelled;

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "stream_cancelled");
        // StreamCancelled has no data — serde renders it without a "data" field
    }

    #[test]
    fn tool_approval_request_serialization() {
        let event = StreamEvent::ToolApprovalRequest {
            tool_call_id: "call_abc123".to_string(),
            tool_name: "write_file".to_string(),
            input: r#"{"path":"/tmp/out.txt","content":"hello"}"#.to_string(),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_approval_request");
        assert_eq!(json["data"]["tool_call_id"], "call_abc123");
        assert_eq!(json["data"]["tool_name"], "write_file");
        assert_eq!(
            json["data"]["input"],
            r#"{"path":"/tmp/out.txt","content":"hello"}"#
        );
    }

    #[test]
    fn stream_event_roundtrip() {
        let events = vec![
            StreamEvent::StreamStart {
                message_id: 1,
                resolved_model: None,
            },
            StreamEvent::TextDelta {
                content: "hi".to_string(),
            },
            StreamEvent::TurnComplete {
                input_tokens: 100,
                output_tokens: 50,
            },
            StreamEvent::StreamCancelled,
        ];

        for event in &events {
            let json = serde_json::to_string(event).expect("serialization should succeed");
            let deserialized: StreamEvent =
                serde_json::from_str(&json).expect("deserialization should succeed");
            // Verify the roundtrip produces valid JSON and deserializes
            let re_json =
                serde_json::to_string(&deserialized).expect("re-serialization should succeed");
            assert_eq!(json, re_json);
        }
    }
}
