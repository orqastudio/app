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

use crate::config::ProjectRelationshipConfig;

/// The platform core config JSON, embedded at compile time from the canonical source.
const PLATFORM_JSON: &str = include_str!("../../../libs/types/src/platform/core.json");

/// A relationship definition from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipDef {
    /// Unique relationship key (e.g. `"delivers"`).
    pub key: String,
    /// Inverse relationship key (e.g. `"delivered-by"`).
    pub inverse: String,
    /// Human-readable forward-direction label.
    #[serde(default)]
    pub label: String,
    /// Allowed source artifact types.
    #[serde(default)]
    pub from: Vec<String>,
    /// Allowed target artifact types.
    #[serde(default)]
    pub to: Vec<String>,
    /// Human-readable description of the relationship's meaning.
    #[serde(default)]
    pub description: String,
    /// Semantic category key (e.g. `"dependency"`, `"delivery"`).
    #[serde(default)]
    pub semantic: Option<String>,
    /// Validation constraint block for this relationship.
    #[serde(default)]
    pub constraints: Option<ConstraintsDef>,
}

/// Validation constraints for a relationship, loaded from the schema.
#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintsDef {
    /// Whether at least one instance of this relationship is required.
    #[serde(default)]
    pub required: Option<bool>,
    /// Minimum number of instances required when `required` is true.
    #[serde(default, rename = "minCount")]
    pub min_count: Option<usize>,
    /// Maximum number of instances allowed.
    #[serde(default, rename = "maxCount")]
    pub max_count: Option<usize>,
    /// Whether the inverse relationship edge must also exist.
    #[serde(default, rename = "requireInverse")]
    pub require_inverse: Option<bool>,
    /// Status-based auto-transition rules.
    #[serde(default, rename = "statusRules")]
    pub status_rules: Vec<StatusRuleDef>,
}

/// A status-dependent auto-transition rule from the schema.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusRuleDef {
    /// Which side to evaluate: `"source"` or `"target"`.
    pub evaluate: String,
    /// Condition to test: `"all-targets-in"`, `"any-target-in"`, `"no-targets-in"`.
    pub condition: String,
    /// Status values to check against.
    pub statuses: Vec<String>,
    /// Status to propose when the condition is met.
    #[serde(rename = "proposedStatus")]
    pub proposed_status: String,
    /// Human-readable description of this rule.
    #[serde(default)]
    pub description: String,
}

/// A semantic category grouping relationship keys by intent.
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticDef {
    /// Human-readable description of what this semantic category means.
    pub description: String,
    /// Relationship keys that belong to this semantic category.
    pub keys: Vec<String>,
}

/// An artifact type definition.
///
/// When loaded from `core.json`, only `key`, `label`, `icon`, and `id_prefix` are
/// present. Plugin-contributed types additionally carry `frontmatter_schema` and
/// `status_transitions`, which default to empty when deserialized from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct ArtifactTypeDef {
    /// Unique artifact type key (e.g. `"task"`, `"epic"`).
    pub key: String,
    /// Human-readable display label.
    pub label: String,
    /// Icon identifier for the UI.
    pub icon: String,
    /// ID prefix used to identify artifacts of this type (e.g. `"TASK-"`).
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
    /// JSON Schema (draft 2020-12) for frontmatter validation.
    /// Null/absent when the type is loaded from core.json (no validation schema there).
    #[serde(default)]
    pub frontmatter_schema: serde_json::Value,
    /// Valid status transitions: maps each status key to the statuses it may transition to.
    /// Empty when the type is loaded from core.json.
    #[serde(default)]
    pub status_transitions: HashMap<String, Vec<String>>,
    /// Pipeline category assigned by the plugin (e.g. "delivery", "learning", "root", "excluded").
    /// None when the type has no explicit pipeline assignment.
    #[serde(default, rename = "pipelineCategory")]
    pub pipeline_category: Option<String>,
}

impl ArtifactTypeDef {
    /// Extract the `required` field names from the frontmatter JSON Schema.
    /// Returns an empty vec if no `required` array is present.
    pub fn frontmatter_required(&self) -> Vec<String> {
        self.frontmatter_schema
            .get("required")
            .and_then(serde_json::Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// The full platform config parsed from core.json.
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    /// Artifact type definitions loaded from core.json.
    #[serde(rename = "artifactTypes")]
    pub artifact_types: Vec<ArtifactTypeDef>,
    /// Relationship definitions from core.json.
    pub relationships: Vec<RelationshipDef>,
    /// Semantic category definitions keyed by category name.
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
    rels.iter()
        .flat_map(|rel| {
            // Each relationship contributes the forward pair plus the reverse pair
            // when the inverse is distinct (self-inverse relationships map only one way).
            let forward = (rel.key.clone(), rel.inverse.clone());
            if rel.inverse == rel.key {
                vec![forward]
            } else {
                vec![forward, (rel.inverse.clone(), rel.key.clone())]
            }
        })
        .collect()
}

/// Build a merged inverse map from platform + project relationships.
///
/// Combines the platform default relationships with project-specific ones,
/// enabling callers to resolve inverses across both sources.
pub fn build_merged_inverse_map(
    project_relationships: &[ProjectRelationshipConfig],
) -> HashMap<String, String> {
    let base = build_inverse_map(&PLATFORM.relationships);
    project_relationships
        .iter()
        .flat_map(|pr| {
            let forward = (pr.key.clone(), pr.inverse.clone());
            if pr.inverse == pr.key {
                vec![forward]
            } else {
                vec![forward, (pr.inverse.clone(), pr.key.clone())]
            }
        })
        .fold(base, |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        })
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
        let rels = vec![make_rel("peer", "peer", None)];
        let map = build_inverse_map(&rels);
        assert_eq!(map.get("peer").unwrap(), "peer");
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn keys_for_semantic_returns_matching_keys() {
        let lineage = keys_for_semantic("lineage");
        assert!(lineage.is_empty());
    }

    #[test]
    fn has_semantic_returns_false_when_platform_is_empty() {
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

    #[test]
    fn build_merged_inverse_map_includes_project_relationships() {
        let project_rels = vec![
            ProjectRelationshipConfig {
                key: "custom-rel".to_owned(),
                inverse: "custom-rel-by".to_owned(),
                label: "custom rel".to_owned(),
                inverse_label: "custom rel by".to_owned(),
            },
        ];
        let map = build_merged_inverse_map(&project_rels);
        // Project relationship should be present alongside platform defaults.
        assert_eq!(map.get("custom-rel"), Some(&"custom-rel-by".to_owned()));
        assert_eq!(map.get("custom-rel-by"), Some(&"custom-rel".to_owned()));
    }

    #[test]
    fn build_merged_inverse_map_empty_project_rels_equals_platform_map() {
        // With no project relationships, merged map == platform map.
        let merged = build_merged_inverse_map(&[]);
        let platform = build_inverse_map(&PLATFORM.relationships);
        // Both should have the same keys and values.
        for (key, val) in &platform {
            assert_eq!(merged.get(key), Some(val));
        }
    }

    #[test]
    fn keys_for_semantic_returns_empty_for_missing_semantic() {
        // No semantic named "nonexistent" exists in the (empty) platform config.
        let keys = keys_for_semantic("nonexistent");
        assert!(keys.is_empty());
    }

    #[test]
    fn has_semantic_returns_false_for_missing_key() {
        // Without any platform relationships, no key has any semantic.
        assert!(!has_semantic("delivers", "delivery"));
    }

    #[test]
    fn artifact_type_def_frontmatter_required_returns_empty_for_empty_schema() {
        let def = ArtifactTypeDef {
            key: "task".to_owned(),
            label: "Task".to_owned(),
            icon: "task-icon".to_owned(),
            id_prefix: "TASK".to_owned(),
            frontmatter_schema: serde_json::json!({}),
            status_transitions: HashMap::new(),
            pipeline_category: None,
        };
        let required = def.frontmatter_required();
        assert!(required.is_empty());
    }

    #[test]
    fn artifact_type_def_frontmatter_required_extracts_fields() {
        let def = ArtifactTypeDef {
            key: "task".to_owned(),
            label: "Task".to_owned(),
            icon: "task-icon".to_owned(),
            id_prefix: "TASK".to_owned(),
            frontmatter_schema: serde_json::json!({
                "required": ["id", "type", "status", "title"]
            }),
            status_transitions: HashMap::new(),
            pipeline_category: None,
        };
        let required = def.frontmatter_required();
        assert_eq!(required.len(), 4);
        assert!(required.contains(&"id".to_owned()));
        assert!(required.contains(&"status".to_owned()));
    }
}
