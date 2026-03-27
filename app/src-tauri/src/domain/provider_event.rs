// Streaming event domain types — re-exported from the orqa-engine crate.
//
// StreamEvent represents all events that flow from the sidecar through the
// Tauri Channel<T> to the frontend during an LLM inference session. Each variant
// maps to a TypeScript discriminated union on the frontend side.

pub use orqa_engine::types::streaming::*;

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
    fn session_title_updated_serialization() {
        let event = StreamEvent::SessionTitleUpdated {
            session_id: 42,
            title: "Rust ownership deep dive".to_string(),
        };

        let json = serde_json::to_value(&event).expect("serialization should succeed");
        assert_eq!(json["type"], "session_title_updated");
        assert_eq!(json["data"]["session_id"], 42);
        assert_eq!(json["data"]["title"], "Rust ownership deep dive");
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
