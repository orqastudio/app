// Project domain types — re-exported from the orqa-engine crate.
//
// Project, ProjectSummary, DetectedStack, and ScanResult represent projects
// managed by OrqaStudio, including detected technology stacks and scan results.

pub use orqa_engine_types::types::project::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn project_roundtrip() {
        let project = Project {
            id: 1,
            name: "forge".to_string(),
            path: "/home/user/forge".to_string(),
            description: Some("A desktop app".to_string()),
            detected_stack: Some(DetectedStack {
                languages: vec!["rust".to_string(), "typescript".to_string()],
                frameworks: vec!["tauri".to_string(), "svelte".to_string()],
                package_manager: Some("npm".to_string()),
                has_claude_config: true,
                has_design_tokens: false,
            }),
            created_at: "2026-03-03T00:00:00Z".to_string(),
            updated_at: "2026-03-03T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&project).expect("serialization should succeed");
        let deserialized: Project =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.id, project.id);
        assert_eq!(deserialized.name, project.name);
        assert!(deserialized.detected_stack.is_some());
    }

    #[test]
    fn project_summary_serialization() {
        let summary = ProjectSummary {
            id: 1,
            name: "test".to_string(),
            path: "/tmp/test".to_string(),
            detected_stack: None,
            session_count: 5,
            artifact_count: 12,
            updated_at: "2026-03-03T00:00:00Z".to_string(),
        };

        let json = serde_json::to_value(&summary).expect("serialization should succeed");
        assert_eq!(json["session_count"], 5);
        assert_eq!(json["artifact_count"], 12);
        assert!(json["detected_stack"].is_null());
    }

    #[test]
    fn scan_result_with_artifact_counts() {
        let mut counts = HashMap::new();
        counts.insert("agent".to_string(), 5);
        counts.insert("rule".to_string(), 20);

        let result = ScanResult {
            project_id: 1,
            detected_stack: DetectedStack {
                languages: vec!["rust".to_string()],
                frameworks: vec![],
                package_manager: None,
                has_claude_config: true,
                has_design_tokens: false,
            },
            artifact_counts: counts,
            design_tokens_found: false,
            scan_duration_ms: 1234,
        };

        let json = serde_json::to_value(&result).expect("serialization should succeed");
        assert_eq!(json["scan_duration_ms"], 1234);
        assert_eq!(json["artifact_counts"]["agent"], 5);
        assert_eq!(json["artifact_counts"]["rule"], 20);
    }
}
