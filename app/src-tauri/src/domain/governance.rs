// Governance scan domain types — re-exported from the orqa-engine crate.
//
// These types represent the result of scanning the filesystem for governance
// files (rules, hooks, agents, etc.). Surfaced in the governance UI to show
// coverage and health of the governance setup.

pub use orqa_engine_types::types::governance::*;

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
        assert!((back.coverage_ratio - 0.5_f64).abs() < f64::EPSILON);
        assert!(back.areas.is_empty());
    }
}
