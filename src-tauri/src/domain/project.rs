use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub detected_stack: Option<DetectedStack>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub detected_stack: Option<DetectedStack>,
    pub session_count: i64,
    pub artifact_count: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedStack {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub package_manager: Option<String>,
    pub has_claude_config: bool,
    pub has_design_tokens: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub project_id: i64,
    pub detected_stack: DetectedStack,
    pub artifact_counts: HashMap<String, i64>,
    pub design_tokens_found: bool,
    pub scan_duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

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
