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

#[cfg(test)]
mod tests {
    use super::*;

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
}
