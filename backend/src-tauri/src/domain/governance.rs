use serde::{Deserialize, Serialize};

/// Result of scanning the filesystem for governance files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceScanResult {
    pub areas: Vec<GovernanceArea>,
    pub coverage_ratio: f64,
}

/// A governance area (rules, hooks, agents, etc.) found during scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceArea {
    pub name: String,
    pub source: String,
    pub files: Vec<GovernanceFile>,
    pub covered: bool,
}

/// A single governance file found on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceFile {
    pub path: String,
    pub size_bytes: u64,
    pub content_preview: String,
}

/// Claude's analysis of the project's governance state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAnalysis {
    pub id: i64,
    pub project_id: i64,
    pub scan_data: GovernanceScanResult,
    pub summary: String,
    pub strengths: Vec<String>,
    pub gaps: Vec<String>,
    pub session_id: Option<i64>,
    pub created_at: String,
}

/// Priority of a governance recommendation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecommendationPriority {
    Critical,
    Recommended,
    Optional,
}

impl RecommendationPriority {
    /// Parse a priority string from the database.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "critical" => Some(Self::Critical),
            "recommended" => Some(Self::Recommended),
            "optional" => Some(Self::Optional),
            _ => None,
        }
    }

    /// Serialize to the database string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
        }
    }
}

/// Lifecycle status of a governance recommendation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecommendationStatus {
    Pending,
    Approved,
    Rejected,
    Applied,
}

impl RecommendationStatus {
    /// Parse a status string from the database.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "approved" => Some(Self::Approved),
            "rejected" => Some(Self::Rejected),
            "applied" => Some(Self::Applied),
            _ => None,
        }
    }

    /// Serialize to the database string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Applied => "applied",
        }
    }
}

/// A single recommendation from Claude's governance analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: i64,
    pub project_id: i64,
    pub analysis_id: i64,
    pub category: String,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub artifact_type: String,
    pub target_path: String,
    pub content: String,
    pub rationale: String,
    pub status: RecommendationStatus,
    pub applied_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// A raw recommendation as parsed from Claude's JSON output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRecommendation {
    pub category: String,
    pub priority: String,
    pub title: String,
    pub description: String,
    pub artifact_type: String,
    pub target_path: String,
    pub content: String,
    pub rationale: String,
}

/// Claude's structured analysis output (parsed from JSON in the response).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeAnalysisOutput {
    pub summary: String,
    pub strengths: Vec<String>,
    pub gaps: Vec<String>,
    pub recommendations: Vec<RawRecommendation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommendation_priority_round_trip() {
        let variants = [
            (RecommendationPriority::Critical, "critical"),
            (RecommendationPriority::Recommended, "recommended"),
            (RecommendationPriority::Optional, "optional"),
        ];

        for (variant, s) in &variants {
            assert_eq!(variant.as_str(), *s);
            assert_eq!(RecommendationPriority::parse(s), Some(variant.clone()));
        }
    }

    #[test]
    fn recommendation_status_round_trip() {
        let variants = [
            (RecommendationStatus::Pending, "pending"),
            (RecommendationStatus::Approved, "approved"),
            (RecommendationStatus::Rejected, "rejected"),
            (RecommendationStatus::Applied, "applied"),
        ];

        for (variant, s) in &variants {
            assert_eq!(variant.as_str(), *s);
            assert_eq!(RecommendationStatus::parse(s), Some(variant.clone()));
        }
    }

    #[test]
    fn recommendation_priority_unknown_returns_none() {
        assert!(RecommendationPriority::parse("high").is_none());
    }

    #[test]
    fn recommendation_status_unknown_returns_none() {
        assert!(RecommendationStatus::parse("done").is_none());
    }

    #[test]
    fn governance_scan_result_serializes() {
        let result = GovernanceScanResult {
            areas: vec![],
            coverage_ratio: 0.5,
        };
        let json = serde_json::to_string(&result).expect("serialize");
        let back: GovernanceScanResult = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.coverage_ratio, 0.5);
        assert!(back.areas.is_empty());
    }

    #[test]
    fn claude_analysis_output_deserializes() {
        let json = r#"{
            "summary": "Good start",
            "strengths": ["Has rules"],
            "gaps": ["No hooks"],
            "recommendations": []
        }"#;
        let output: ClaudeAnalysisOutput = serde_json::from_str(json).expect("deserialize");
        assert_eq!(output.summary, "Good start");
        assert_eq!(output.strengths, ["Has rules"]);
        assert_eq!(output.gaps, ["No hooks"]);
        assert!(output.recommendations.is_empty());
    }
}
