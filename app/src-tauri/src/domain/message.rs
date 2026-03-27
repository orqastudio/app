// Message domain types — re-exported from the orqa-engine crate.
//
// Message, MessageRole, ContentType, StreamStatus, MessageId, and SearchResult
// represent chat messages within a session. These types flow from the database
// through the Tauri IPC boundary to the frontend.

pub use orqa_engine::types::message::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_role_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(MessageRole::User)
                .expect("serialization should succeed")
                .as_str(),
            Some("user")
        );
        assert_eq!(
            serde_json::to_value(MessageRole::Assistant)
                .expect("serialization should succeed")
                .as_str(),
            Some("assistant")
        );
        assert_eq!(
            serde_json::to_value(MessageRole::System)
                .expect("serialization should succeed")
                .as_str(),
            Some("system")
        );
    }

    #[test]
    fn content_type_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(ContentType::Text)
                .expect("serialization should succeed")
                .as_str(),
            Some("text")
        );
        assert_eq!(
            serde_json::to_value(ContentType::ToolUse)
                .expect("serialization should succeed")
                .as_str(),
            Some("tool_use")
        );
        assert_eq!(
            serde_json::to_value(ContentType::ToolResult)
                .expect("serialization should succeed")
                .as_str(),
            Some("tool_result")
        );
        assert_eq!(
            serde_json::to_value(ContentType::Thinking)
                .expect("serialization should succeed")
                .as_str(),
            Some("thinking")
        );
        assert_eq!(
            serde_json::to_value(ContentType::Image)
                .expect("serialization should succeed")
                .as_str(),
            Some("image")
        );
    }

    #[test]
    fn stream_status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(StreamStatus::Pending)
                .expect("serialization should succeed")
                .as_str(),
            Some("pending")
        );
        assert_eq!(
            serde_json::to_value(StreamStatus::Complete)
                .expect("serialization should succeed")
                .as_str(),
            Some("complete")
        );
        assert_eq!(
            serde_json::to_value(StreamStatus::Error)
                .expect("serialization should succeed")
                .as_str(),
            Some("error")
        );
    }

    #[test]
    fn message_roundtrip() {
        let msg = Message {
            id: 1,
            session_id: 1,
            role: MessageRole::User,
            content_type: ContentType::Text,
            content: Some("Hello, how are you?".to_string()),
            tool_call_id: None,
            tool_name: None,
            tool_input: None,
            tool_is_error: false,
            turn_index: 0,
            block_index: 0,
            stream_status: StreamStatus::Complete,
            input_tokens: Some(10),
            output_tokens: None,
            created_at: "2026-03-03T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&msg).expect("serialization should succeed");
        let deserialized: Message =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.id, msg.id);
        assert_eq!(deserialized.role, MessageRole::User);
        assert_eq!(deserialized.content_type, ContentType::Text);
        assert_eq!(deserialized.stream_status, StreamStatus::Complete);
    }

    #[test]
    fn search_result_serialization() {
        let result = SearchResult {
            message_id: 5,
            session_id: 1,
            session_title: Some("Debug session".to_string()),
            content: "Fix the parsing bug".to_string(),
            highlighted: "Fix the <mark>parsing</mark> bug".to_string(),
            rank: 0.95,
        };

        let json = serde_json::to_value(&result).expect("serialization should succeed");
        assert_eq!(json["message_id"], 5);
        assert_eq!(json["rank"], 0.95);
        assert!(json["highlighted"]
            .as_str()
            .expect("highlighted should be a string")
            .contains("<mark>"));
    }
}
