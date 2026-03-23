use serde::{Deserialize, Serialize};

use crate::error::OrqaError;

/// Serialize a value to an NDJSON line (compact JSON followed by a newline).
///
/// NDJSON (Newline-Delimited JSON) is the wire format for sidecar communication.
/// Each message is a single JSON object on one line, terminated by `\n`.
pub fn to_ndjson<T: Serialize>(value: &T) -> Result<String, OrqaError> {
    let mut json =
        serde_json::to_string(value).map_err(|e| OrqaError::Serialization(e.to_string()))?;
    json.push('\n');
    Ok(json)
}

/// Deserialize a single NDJSON line into a typed value.
///
/// Trims leading/trailing whitespace (including the newline delimiter)
/// before parsing.
pub fn from_ndjson<T: for<'de> Deserialize<'de>>(line: &str) -> Result<T, OrqaError> {
    serde_json::from_str(line.trim()).map_err(|e| OrqaError::Serialization(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sidecar::types::{SidecarRequest, SidecarResponse};

    #[test]
    fn to_ndjson_appends_newline() {
        let req = SidecarRequest::HealthCheck;
        let line = to_ndjson(&req).expect("serialization should succeed");
        assert!(line.ends_with('\n'), "NDJSON line must end with newline");
        assert_eq!(
            line.chars().filter(|c| *c == '\n').count(),
            1,
            "should contain exactly one newline"
        );
    }

    #[test]
    fn to_ndjson_produces_valid_json() {
        let req = SidecarRequest::SendMessage {
            session_id: 1,
            content: "hello".to_string(),
            model: None,
            system_prompt: None,
            provider_session_id: None,
            enable_thinking: false,
        };
        let line = to_ndjson(&req).expect("serialization should succeed");
        let parsed: serde_json::Value =
            serde_json::from_str(line.trim()).expect("should be valid JSON");
        assert_eq!(parsed["type"], "send_message");
    }

    #[test]
    fn from_ndjson_trims_whitespace() {
        let input = r#"  {"type":"health_check"}  "#;
        let req: SidecarRequest = from_ndjson(input).expect("deserialization should succeed");
        assert!(matches!(req, SidecarRequest::HealthCheck));
    }

    #[test]
    fn from_ndjson_handles_trailing_newline() {
        let input = "{\"type\":\"health_check\"}\n";
        let req: SidecarRequest = from_ndjson(input).expect("deserialization should succeed");
        assert!(matches!(req, SidecarRequest::HealthCheck));
    }

    #[test]
    fn from_ndjson_rejects_invalid_json() {
        let result: Result<SidecarRequest, _> = from_ndjson("not json at all");
        assert!(result.is_err());
    }

    #[test]
    fn roundtrip_request() {
        let req = SidecarRequest::SendMessage {
            session_id: 42,
            content: "test message".to_string(),
            model: Some("claude-opus-4-6".to_string()),
            system_prompt: Some("be concise".to_string()),
            provider_session_id: None,
            enable_thinking: false,
        };

        let line = to_ndjson(&req).expect("serialization should succeed");
        let deserialized: SidecarRequest =
            from_ndjson(&line).expect("deserialization should succeed");

        let re_line = to_ndjson(&deserialized).expect("re-serialization should succeed");
        assert_eq!(line, re_line);
    }

    #[test]
    fn roundtrip_response() {
        let resp = SidecarResponse::TurnComplete {
            input_tokens: 1500,
            output_tokens: 800,
        };

        let line = to_ndjson(&resp).expect("serialization should succeed");
        let deserialized: SidecarResponse =
            from_ndjson(&line).expect("deserialization should succeed");

        let re_line = to_ndjson(&deserialized).expect("re-serialization should succeed");
        assert_eq!(line, re_line);
    }

    #[test]
    fn roundtrip_all_response_variants() {
        let responses = vec![
            SidecarResponse::StreamStart {
                message_id: 1,
                resolved_model: Some("model".to_string()),
            },
            SidecarResponse::TextDelta {
                content: "word".to_string(),
            },
            SidecarResponse::StreamCancelled,
            SidecarResponse::HealthOk {
                version: "0.1.0".to_string(),
            },
            SidecarResponse::SummaryResult {
                session_id: 5,
                summary: "a summary".to_string(),
            },
        ];

        for resp in &responses {
            let line = to_ndjson(resp).expect("serialization should succeed");
            let deserialized: SidecarResponse =
                from_ndjson(&line).expect("deserialization should succeed");
            let re_line = to_ndjson(&deserialized).expect("re-serialization should succeed");
            assert_eq!(line, re_line);
        }
    }

    #[test]
    fn to_ndjson_escapes_special_characters() {
        let req = SidecarRequest::SendMessage {
            session_id: 1,
            content: "line1\nline2\ttab \"quoted\"".to_string(),
            model: None,
            system_prompt: None,
            provider_session_id: None,
            enable_thinking: false,
        };
        let line = to_ndjson(&req).expect("serialization should succeed");
        // The JSON itself must be a single line (newlines in content are escaped)
        let lines: Vec<&str> = line.trim().split('\n').collect();
        assert_eq!(
            lines.len(),
            1,
            "NDJSON must be a single line even with special chars in content"
        );
    }
}
