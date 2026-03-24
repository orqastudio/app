//! Platform configuration loaded from the embedded `core.json`.
//!
//! This module provides the single source of truth for core artifact types
//! and relationships. The JSON is embedded at compile time and parsed once
//! on first access. Project relationships (from `project.json`) and plugin
//! relationships are merged at runtime by callers — this module only provides
//! the platform defaults.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;

/// The platform core config JSON, embedded at compile time from the canonical source.
const PLATFORM_JSON: &str = include_str!("../../../../libs/types/src/platform/core.json");

/// A relationship definition from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipDef {
    pub key: String,
    pub inverse: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub from: Vec<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub semantic: Option<String>,
    #[serde(default)]
    pub constraints: Option<ConstraintsDef>,
}

/// Validation constraints for a relationship, loaded from the schema.
#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintsDef {
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default, rename = "minCount")]
    pub min_count: Option<usize>,
    #[serde(default, rename = "maxCount")]
    pub max_count: Option<usize>,
    #[serde(default, rename = "requireInverse")]
    pub require_inverse: Option<bool>,
    #[serde(default, rename = "statusRules")]
    pub status_rules: Vec<StatusRuleDef>,
}

/// A status-dependent auto-transition rule from the schema.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusRuleDef {
    pub evaluate: String,
    pub condition: String,
    pub statuses: Vec<String>,
    #[serde(rename = "proposedStatus")]
    pub proposed_status: String,
    #[serde(default)]
    pub description: String,
}

/// A semantic category grouping relationship keys by intent.
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticDef {
    pub description: String,
    pub keys: Vec<String>,
}

/// An artifact type from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct ArtifactTypeDef {
    pub key: String,
    pub label: String,
    pub icon: String,
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
}

/// The full platform config parsed from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    #[serde(rename = "artifactTypes")]
    pub artifact_types: Vec<ArtifactTypeDef>,
    pub relationships: Vec<RelationshipDef>,
    pub semantics: HashMap<String, SemanticDef>,
}

/// Lazily-parsed platform config, available for the lifetime of the process.
pub static PLATFORM: LazyLock<PlatformConfig> = LazyLock::new(|| {
    serde_json::from_str(PLATFORM_JSON).expect("platform_core.json must be valid JSON")
});

/// Build an inverse map from a slice of relationship definitions.
///
/// Returns a HashMap where each key maps to its inverse, and vice versa.
/// This replaces the hardcoded `INVERSE_MAP` constant.
pub fn build_inverse_map(rels: &[RelationshipDef]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for rel in rels {
        map.insert(rel.key.clone(), rel.inverse.clone());
        if rel.inverse != rel.key {
            map.insert(rel.inverse.clone(), rel.key.clone());
        }
    }
    map
}

/// Build a merged inverse map from platform + project relationships.
pub fn build_merged_inverse_map(
    project_relationships: &[crate::domain::project_settings::ProjectRelationshipConfig],
) -> HashMap<String, String> {
    let mut map = build_inverse_map(&PLATFORM.relationships);
    for pr in project_relationships {
        map.insert(pr.key.clone(), pr.inverse.clone());
        if pr.inverse != pr.key {
            map.insert(pr.inverse.clone(), pr.key.clone());
        }
    }
    map
}

/// Get all relationship keys that belong to a given semantic category.
///
/// Returns an empty vec if the semantic doesn't exist.
pub fn keys_for_semantic(semantic: &str) -> Vec<String> {
    PLATFORM
        .semantics
        .get(semantic)
        .map(|s| s.keys.clone())
        .unwrap_or_default()
}

/// Check whether a relationship key has a given semantic.
pub fn has_semantic(relationship_key: &str, semantic: &str) -> bool {
    PLATFORM
        .semantics
        .get(semantic)
        .is_some_and(|s| s.keys.iter().any(|k| k == relationship_key))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Helpers — build in-memory relationship fixtures rather than relying on
    // the static PLATFORM (core.json is intentionally empty; plugins are the
    // runtime source of truth for all relationship data).
    // ---------------------------------------------------------------------------

    fn make_rel(key: &str, inverse: &str, semantic: Option<&str>) -> RelationshipDef {
        RelationshipDef {
            key: key.to_string(),
            inverse: inverse.to_string(),
            label: String::new(),
            from: Vec::new(),
            to: Vec::new(),
            description: String::new(),
            semantic: semantic.map(str::to_string),
            constraints: None,
        }
    }

    fn make_rel_with_constraints(
        key: &str,
        inverse: &str,
        required: Option<bool>,
        min_count: Option<usize>,
    ) -> RelationshipDef {
        RelationshipDef {
            key: key.to_string(),
            inverse: inverse.to_string(),
            label: String::new(),
            from: Vec::new(),
            to: Vec::new(),
            description: String::new(),
            semantic: None,
            constraints: Some(ConstraintsDef {
                required,
                min_count,
                max_count: None,
                require_inverse: None,
                status_rules: Vec::new(),
            }),
        }
    }

    #[test]
    fn platform_config_loads_without_panic() {
        // core.json is intentionally empty — plugins are the source of truth.
        // Verify the static initialises cleanly (no panic, no bad JSON).
        assert!(PLATFORM.artifact_types.is_empty());
        assert!(PLATFORM.relationships.is_empty());
        assert!(PLATFORM.semantics.is_empty());
    }

    #[test]
    fn inverse_map_contains_all_pairs() {
        let rels = vec![
            make_rel("upholds", "upheld-by", Some("foundation")),
            make_rel("grounded", "grounded-by", Some("foundation")),
            make_rel("synchronised-with", "synchronised-with", None),
        ];
        let map = build_inverse_map(&rels);
        assert_eq!(map.get("upholds").unwrap(), "upheld-by");
        assert_eq!(map.get("upheld-by").unwrap(), "upholds");
        assert_eq!(map.get("grounded").unwrap(), "grounded-by");
        assert_eq!(map.get("synchronised-with").unwrap(), "synchronised-with");
    }

    #[test]
    fn inverse_map_self_referential_key_inserted_once() {
        // When key == inverse, only one entry should be inserted.
        let rels = vec![make_rel("peer", "peer", None)];
        let map = build_inverse_map(&rels);
        assert_eq!(map.get("peer").unwrap(), "peer");
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn keys_for_semantic_returns_matching_keys() {
        // keys_for_semantic reads PLATFORM.semantics, which is empty for now.
        // The function must return an empty vec rather than panicking.
        let lineage = keys_for_semantic("lineage");
        assert!(lineage.is_empty());
    }

    #[test]
    fn has_semantic_returns_false_when_platform_is_empty() {
        // With an empty platform (plugins are the source of truth at runtime),
        // has_semantic must return false for any input — not panic.
        assert!(!has_semantic("crystallises", "lineage"));
        assert!(!has_semantic("upholds", "foundation"));
    }

    #[test]
    fn constraints_round_trip_through_struct() {
        let rel = make_rel_with_constraints("grounded", "grounded-by", Some(true), Some(1));
        let constraints = rel.constraints.as_ref().expect("constraints present");
        assert_eq!(constraints.required, Some(true));
        assert_eq!(constraints.min_count, Some(1));
        assert!(constraints.max_count.is_none());
    }

    #[test]
    fn build_inverse_map_empty_input_returns_empty_map() {
        let map = build_inverse_map(&[]);
        assert!(map.is_empty());
    }
}
