use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub session_id: i64,
    pub role: MessageRole,
    pub content_type: ContentType,
    pub content: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_name: Option<String>,
    pub tool_input: Option<String>,
    pub tool_is_error: bool,
    pub turn_index: i32,
    pub block_index: i32,
    pub stream_status: StreamStatus,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    ToolUse,
    ToolResult,
    Thinking,
    Image,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StreamStatus {
    Pending,
    Complete,
    Error,
}

pub type MessageId = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub message_id: i64,
    pub session_id: i64,
    pub session_title: Option<String>,
    pub content: String,
    pub highlighted: String,
    pub rank: f64,
}

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
