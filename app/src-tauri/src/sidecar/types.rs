use serde::{Deserialize, Serialize};

/// Request sent to the sidecar process via stdin as NDJSON.
///
/// Each variant is tagged with a `type` field using snake_case naming,
/// matching the echo sidecar and future Agent SDK sidecar implementations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SidecarRequest {
    SendMessage {
        session_id: i64,
        content: String,
        model: Option<String>,
        system_prompt: Option<String>,
        provider_session_id: Option<String>,
        enable_thinking: bool,
    },
    CancelStream {
        session_id: i64,
    },
    GenerateSummary {
        session_id: i64,
        messages: Vec<MessageSummary>,
    },
    HealthCheck,
    ToolResult {
        tool_call_id: String,
        output: String,
        is_error: bool,
    },
    ToolApproval {
        tool_call_id: String,
        approved: bool,
        reason: Option<String>,
    },
}

/// A condensed message used in summary generation requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSummary {
    pub role: String,
    pub content: String,
}

/// Response or event from the sidecar process via stdout as NDJSON.
///
/// Each variant is tagged with a `type` field using snake_case naming.
/// These map closely to `StreamEvent` but include additional response
/// types (health, summary) that are not streamed to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SidecarResponse {
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
    HealthOk {
        version: String,
    },
    SummaryResult {
        session_id: i64,
        summary: String,
    },
    ToolExecute {
        tool_call_id: String,
        tool_name: String,
        input: String,
    },
    ToolApprovalRequest {
        tool_call_id: String,
        tool_name: String,
        input: String,
    },
    SessionInitialized {
        session_id: i64,
        provider_session_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── SidecarRequest tests ──

    #[test]
    fn send_message_serialization() {
        let req = SidecarRequest::SendMessage {
            session_id: 1,
            content: "hello".to_string(),
            model: Some("claude-opus-4-6".to_string()),
            system_prompt: None,
            provider_session_id: None,
            enable_thinking: false,
        };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "send_message");
        assert_eq!(json["session_id"], 1);
        assert_eq!(json["content"], "hello");
        assert_eq!(json["model"], "claude-opus-4-6");
        assert!(json["system_prompt"].is_null());
        assert!(json["provider_session_id"].is_null());
        assert!(!json["enable_thinking"].as_bool().expect("should be bool"));
    }

    #[test]
    fn cancel_stream_serialization() {
        let req = SidecarRequest::CancelStream { session_id: 42 };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "cancel_stream");
        assert_eq!(json["session_id"], 42);
    }

    #[test]
    fn generate_summary_serialization() {
        let req = SidecarRequest::GenerateSummary {
            session_id: 5,
            messages: vec![
                MessageSummary {
                    role: "user".to_string(),
                    content: "hi".to_string(),
                },
                MessageSummary {
                    role: "assistant".to_string(),
                    content: "hello".to_string(),
                },
            ],
        };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "generate_summary");
        assert_eq!(json["session_id"], 5);
        assert_eq!(
            json["messages"].as_array().expect("should be array").len(),
            2
        );
    }

    #[test]
    fn health_check_serialization() {
        let req = SidecarRequest::HealthCheck;

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "health_check");
    }

    #[test]
    fn tool_result_request_serialization() {
        let req = SidecarRequest::ToolResult {
            tool_call_id: "call_001".to_string(),
            output: "result data".to_string(),
            is_error: false,
        };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_result");
        assert_eq!(json["tool_call_id"], "call_001");
        assert_eq!(json["output"], "result data");
        assert!(!json["is_error"].as_bool().expect("should be bool"));
    }

    #[test]
    fn tool_result_request_error_serialization() {
        let req = SidecarRequest::ToolResult {
            tool_call_id: "call_002".to_string(),
            output: "something went wrong".to_string(),
            is_error: true,
        };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_result");
        assert!(json["is_error"].as_bool().expect("should be bool"));
    }

    #[test]
    fn tool_approval_request_approved_serialization() {
        let req = SidecarRequest::ToolApproval {
            tool_call_id: "call_001".to_string(),
            approved: true,
            reason: None,
        };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_approval");
        assert_eq!(json["tool_call_id"], "call_001");
        assert!(json["approved"].as_bool().expect("should be bool"));
        assert!(json["reason"].is_null());
    }

    #[test]
    fn tool_approval_request_denied_serialization() {
        let req = SidecarRequest::ToolApproval {
            tool_call_id: "call_001".to_string(),
            approved: false,
            reason: Some("dangerous operation".to_string()),
        };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_approval");
        assert!(!json["approved"].as_bool().expect("should be bool"));
        assert_eq!(json["reason"], "dangerous operation");
    }

    #[test]
    fn request_roundtrip() {
        let requests = vec![
            SidecarRequest::SendMessage {
                session_id: 1,
                content: "test".to_string(),
                model: None,
                system_prompt: Some("be helpful".to_string()),
                provider_session_id: Some("abc-123".to_string()),
                enable_thinking: true,
            },
            SidecarRequest::CancelStream { session_id: 2 },
            SidecarRequest::HealthCheck,
            SidecarRequest::ToolResult {
                tool_call_id: "call_001".to_string(),
                output: "result data".to_string(),
                is_error: false,
            },
            SidecarRequest::ToolApproval {
                tool_call_id: "call_002".to_string(),
                approved: true,
                reason: None,
            },
            SidecarRequest::ToolApproval {
                tool_call_id: "call_003".to_string(),
                approved: false,
                reason: Some("not allowed".to_string()),
            },
        ];

        for req in &requests {
            let json = serde_json::to_string(req).expect("serialization should succeed");
            let deserialized: SidecarRequest =
                serde_json::from_str(&json).expect("deserialization should succeed");
            let re_json =
                serde_json::to_string(&deserialized).expect("re-serialization should succeed");
            assert_eq!(json, re_json);
        }
    }

    // ── SidecarResponse tests ──

    #[test]
    fn stream_start_response() {
        let resp = SidecarResponse::StreamStart {
            message_id: 10,
            resolved_model: Some("claude-sonnet-4-20250514".to_string()),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "stream_start");
        assert_eq!(json["message_id"], 10);
    }

    #[test]
    fn text_delta_response() {
        let resp = SidecarResponse::TextDelta {
            content: "Hello ".to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "text_delta");
        assert_eq!(json["content"], "Hello ");
    }

    #[test]
    fn thinking_delta_response() {
        let resp = SidecarResponse::ThinkingDelta {
            content: "Let me consider...".to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "thinking_delta");
        assert_eq!(json["content"], "Let me consider...");
    }

    #[test]
    fn tool_use_start_response() {
        let resp = SidecarResponse::ToolUseStart {
            tool_call_id: "call_001".to_string(),
            tool_name: "read_file".to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_use_start");
        assert_eq!(json["tool_call_id"], "call_001");
        assert_eq!(json["tool_name"], "read_file");
    }

    #[test]
    fn tool_input_delta_response() {
        let resp = SidecarResponse::ToolInputDelta {
            tool_call_id: "call_001".to_string(),
            content: r#"{"path":"#.to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_input_delta");
        assert_eq!(json["tool_call_id"], "call_001");
    }

    #[test]
    fn tool_result_response() {
        let resp = SidecarResponse::ToolResult {
            tool_call_id: "call_001".to_string(),
            tool_name: "read_file".to_string(),
            result: "file contents".to_string(),
            is_error: false,
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_result");
        assert!(!json["is_error"].as_bool().expect("should be bool"));
    }

    #[test]
    fn block_complete_response() {
        let resp = SidecarResponse::BlockComplete {
            block_index: 0,
            content_type: "text".to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "block_complete");
        assert_eq!(json["block_index"], 0);
    }

    #[test]
    fn turn_complete_response() {
        let resp = SidecarResponse::TurnComplete {
            input_tokens: 500,
            output_tokens: 200,
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "turn_complete");
        assert_eq!(json["input_tokens"], 500);
        assert_eq!(json["output_tokens"], 200);
    }

    #[test]
    fn stream_error_response() {
        let resp = SidecarResponse::StreamError {
            code: "rate_limit".to_string(),
            message: "Too many requests".to_string(),
            recoverable: true,
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "stream_error");
        assert_eq!(json["code"], "rate_limit");
        assert!(json["recoverable"].as_bool().expect("should be bool"));
    }

    #[test]
    fn stream_cancelled_response() {
        let resp = SidecarResponse::StreamCancelled;

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "stream_cancelled");
    }

    #[test]
    fn health_ok_response() {
        let resp = SidecarResponse::HealthOk {
            version: "0.1.0".to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "health_ok");
        assert_eq!(json["version"], "0.1.0");
    }

    #[test]
    fn summary_result_response() {
        let resp = SidecarResponse::SummaryResult {
            session_id: 3,
            summary: "User asked about Rust, assistant explained ownership.".to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "summary_result");
        assert_eq!(json["session_id"], 3);
    }

    #[test]
    fn tool_execute_response() {
        let resp = SidecarResponse::ToolExecute {
            tool_call_id: "call_010".to_string(),
            tool_name: "read_file".to_string(),
            input: r#"{"path":"/src/main.rs"}"#.to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_execute");
        assert_eq!(json["tool_call_id"], "call_010");
        assert_eq!(json["tool_name"], "read_file");
        assert_eq!(json["input"], r#"{"path":"/src/main.rs"}"#);
    }

    #[test]
    fn tool_approval_request_response() {
        let resp = SidecarResponse::ToolApprovalRequest {
            tool_call_id: "call_011".to_string(),
            tool_name: "write_file".to_string(),
            input: r#"{"path":"/tmp/out.txt","content":"hello"}"#.to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "tool_approval_request");
        assert_eq!(json["tool_call_id"], "call_011");
        assert_eq!(json["tool_name"], "write_file");
    }

    #[test]
    fn session_initialized_response() {
        let resp = SidecarResponse::SessionInitialized {
            session_id: 1,
            provider_session_id: "abc-def-123".to_string(),
        };

        let json = serde_json::to_value(&resp).expect("serialization should succeed");
        assert_eq!(json["type"], "session_initialized");
        assert_eq!(json["session_id"], 1);
        assert_eq!(json["provider_session_id"], "abc-def-123");
    }

    #[test]
    fn send_message_with_provider_session_id() {
        let req = SidecarRequest::SendMessage {
            session_id: 1,
            content: "hello".to_string(),
            model: None,
            system_prompt: None,
            provider_session_id: Some("resume-uuid".to_string()),
            enable_thinking: false,
        };

        let json = serde_json::to_value(&req).expect("serialization should succeed");
        assert_eq!(json["provider_session_id"], "resume-uuid");
    }

    #[test]
    fn response_roundtrip() {
        let responses = vec![
            SidecarResponse::StreamStart {
                message_id: 1,
                resolved_model: None,
            },
            SidecarResponse::TextDelta {
                content: "hi".to_string(),
            },
            SidecarResponse::ThinkingDelta {
                content: "hmm".to_string(),
            },
            SidecarResponse::ToolUseStart {
                tool_call_id: "c1".to_string(),
                tool_name: "grep".to_string(),
            },
            SidecarResponse::ToolInputDelta {
                tool_call_id: "c1".to_string(),
                content: "{}".to_string(),
            },
            SidecarResponse::ToolResult {
                tool_call_id: "c1".to_string(),
                tool_name: "grep".to_string(),
                result: "found".to_string(),
                is_error: false,
            },
            SidecarResponse::BlockComplete {
                block_index: 0,
                content_type: "text".to_string(),
            },
            SidecarResponse::TurnComplete {
                input_tokens: 100,
                output_tokens: 50,
            },
            SidecarResponse::StreamError {
                code: "err".to_string(),
                message: "fail".to_string(),
                recoverable: false,
            },
            SidecarResponse::StreamCancelled,
            SidecarResponse::HealthOk {
                version: "1.0".to_string(),
            },
            SidecarResponse::SummaryResult {
                session_id: 1,
                summary: "test".to_string(),
            },
            SidecarResponse::ToolExecute {
                tool_call_id: "c2".to_string(),
                tool_name: "read_file".to_string(),
                input: r#"{"path":"foo"}"#.to_string(),
            },
            SidecarResponse::ToolApprovalRequest {
                tool_call_id: "c3".to_string(),
                tool_name: "write_file".to_string(),
                input: r#"{"path":"bar"}"#.to_string(),
            },
            SidecarResponse::SessionInitialized {
                session_id: 1,
                provider_session_id: "uuid-123".to_string(),
            },
        ];

        for resp in &responses {
            let json = serde_json::to_string(resp).expect("serialization should succeed");
            let deserialized: SidecarResponse =
                serde_json::from_str(&json).expect("deserialization should succeed");
            let re_json =
                serde_json::to_string(&deserialized).expect("re-serialization should succeed");
            assert_eq!(json, re_json);
        }
    }
}
