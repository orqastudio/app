// Session domain types — re-exported from the orqa-engine crate.
//
// Session, SessionSummary, and SessionStatus represent agent sessions — the
// primary unit of interaction between a user and an LLM sidecar. Sessions
// contain messages, accumulate token counts, and transition through a lifecycle.

pub use orqa_engine_types::types::session::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(SessionStatus::Active)
                .expect("serialization should succeed")
                .as_str(),
            Some("active")
        );
        assert_eq!(
            serde_json::to_value(SessionStatus::Completed)
                .expect("serialization should succeed")
                .as_str(),
            Some("completed")
        );
        assert_eq!(
            serde_json::to_value(SessionStatus::Abandoned)
                .expect("serialization should succeed")
                .as_str(),
            Some("abandoned")
        );
        assert_eq!(
            serde_json::to_value(SessionStatus::Error)
                .expect("serialization should succeed")
                .as_str(),
            Some("error")
        );
    }

    #[test]
    fn session_status_deserializes_snake_case() {
        let active: SessionStatus =
            serde_json::from_str("\"active\"").expect("deserialization should succeed");
        assert_eq!(active, SessionStatus::Active);
    }

    #[test]
    fn session_roundtrip() {
        let session = Session {
            id: 1,
            project_id: 1,
            title: Some("Initial setup".to_string()),
            model: "auto".to_string(),
            system_prompt: None,
            status: SessionStatus::Active,
            summary: None,
            handoff_notes: None,
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cost_usd: 0.0,
            provider_session_id: None,
            created_at: "2026-03-03T00:00:00Z".to_string(),
            updated_at: "2026-03-03T00:00:00Z".to_string(),
            title_manually_set: false,
        };

        let json = serde_json::to_string(&session).expect("serialization should succeed");
        let deserialized: Session =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.id, session.id);
        assert_eq!(deserialized.status, SessionStatus::Active);
        assert_eq!(deserialized.model, "auto");
    }

    #[test]
    fn session_summary_serialization() {
        let summary = SessionSummary {
            id: 1,
            title: Some("Test session".to_string()),
            status: SessionStatus::Completed,
            message_count: 42,
            preview: Some("How do I...".to_string()),
            created_at: "2026-03-03T00:00:00Z".to_string(),
            updated_at: "2026-03-03T00:00:00Z".to_string(),
        };

        let json = serde_json::to_value(&summary).expect("serialization should succeed");
        assert_eq!(json["message_count"], 42);
        assert_eq!(json["status"], "completed");
    }
}
