//! Plugin schema conflict detection.
//!
//! When multiple plugins define schemas for the same artifact type key,
//! or define conflicting `statusTransitions`, the validator detects
//! the overlap and reports it. Conflicts must be resolved by the user
//! at install time — unresolved conflicts are errors.

use std::collections::HashMap;

use crate::platform::ArtifactTypeDef;
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

/// A conflict resolution stored in project.json.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ConflictResolution {
    /// The conflicting schema or field key.
    pub conflict: String,
    /// The plugins involved.
    pub plugins: Vec<String>,
    /// How it was resolved: "merge", "keep-existing", "adopt-new".
    pub resolution: String,
    /// When the decision was made (ISO 8601).
    #[serde(default)]
    pub decided: Option<String>,
    /// Human-readable rationale.
    #[serde(default)]
    pub rationale: Option<String>,
}

/// Check for duplicate artifact type keys across plugins.
///
/// Two plugins defining the same `key` (e.g. both define "rule") is
/// a conflict unless resolved in project.json.
pub fn check_schema_conflicts(
    artifact_types: &[ArtifactTypeDef],
    resolutions: &[ConflictResolution],
    checks: &mut Vec<IntegrityCheck>,
) {
    // Group types by key
    let mut by_key: HashMap<&str, Vec<&ArtifactTypeDef>> = HashMap::new();
    for t in artifact_types {
        by_key.entry(t.key.as_str()).or_default().push(t);
    }

    let resolved_keys: std::collections::HashSet<&str> =
        resolutions.iter().map(|r| r.conflict.as_str()).collect();

    for (key, types) in &by_key {
        if types.len() <= 1 {
            continue;
        }

        // Check if this conflict has been resolved
        if resolved_keys.contains(key) {
            continue;
        }

        // Check for status transition conflicts
        let first = &types[0].status_transitions;
        for other in &types[1..] {
            if other.status_transitions != *first {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::SchemaViolation,
                    severity: IntegritySeverity::Error,
                    artifact_id: format!("schema:{key}"),
                    message: format!(
                        "Schema conflict: multiple plugins define '{key}' with different statusTransitions — resolve via project.json enforcement.resolutions"
                    ),
                    auto_fixable: false,
                    fix_description: None,
                });
                break;
            }
        }

        // Check for required field conflicts
        let first_required = types[0].frontmatter_required();
        for other in &types[1..] {
            let other_required = other.frontmatter_required();
            if other_required != first_required {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::SchemaViolation,
                    severity: IntegritySeverity::Error,
                    artifact_id: format!("schema:{key}"),
                    message: format!(
                        "Schema overlap: multiple plugins define '{key}' with different required fields — consider resolving via project.json"
                    ),
                    auto_fixable: false,
                    fix_description: None,
                });
                break;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_type(key: &str, transitions: HashMap<String, Vec<String>>) -> ArtifactTypeDef {
        ArtifactTypeDef {
            key: key.to_owned(),
            label: key.to_owned(),
            icon: "file".to_owned(),
            id_prefix: key.to_uppercase(),
            frontmatter_schema: serde_json::json!({ "type": "object" }),
            status_transitions: transitions,
        }
    }

    #[test]
    fn no_conflict_with_single_definition() {
        let types = vec![make_type("rule", HashMap::new())];
        let mut checks = Vec::new();
        check_schema_conflicts(&types, &[], &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn detects_status_transition_conflict() {
        let mut t1 = HashMap::new();
        t1.insert("active".to_owned(), vec!["archived".to_owned()]);
        let mut t2 = HashMap::new();
        t2.insert("active".to_owned(), vec!["completed".to_owned()]);

        let types = vec![make_type("rule", t1), make_type("rule", t2)];
        let mut checks = Vec::new();
        check_schema_conflicts(&types, &[], &mut checks);
        assert!(!checks.is_empty());
        assert!(checks[0].message.contains("conflict"));
    }

    #[test]
    fn resolved_conflict_is_skipped() {
        let mut t1 = HashMap::new();
        t1.insert("active".to_owned(), vec!["archived".to_owned()]);
        let mut t2 = HashMap::new();
        t2.insert("active".to_owned(), vec!["completed".to_owned()]);

        let types = vec![make_type("rule", t1), make_type("rule", t2)];
        let resolutions = vec![ConflictResolution {
            conflict: "rule".to_owned(),
            plugins: vec!["a".to_owned(), "b".to_owned()],
            resolution: "merge".to_owned(),
            decided: None,
            rationale: None,
        }];
        let mut checks = Vec::new();
        check_schema_conflicts(&types, &resolutions, &mut checks);
        assert!(checks.is_empty());
    }
}
