//! Collision detection for plugin installation.
//!
//! When a plugin declares relationship or schema keys that already exist in
//! core or another installed plugin, the installer surfaces the collision
//! so the user can decide: merge (same intent, union constraints) or rename
//! (different intent, namespace the key).
//!
//! The `semantic` and `description` fields are compared to assess intent.
//! These fields are NOT editable — they represent the author's declared intent.
//! Decisions are recorded in the manifest's `mergeDecisions` array so that
//! future updates can resolve automatically.

use serde::{Deserialize, Serialize};

use crate::domain::integrity_engine::RelationshipSchema;
use crate::domain::platform_config::PLATFORM;

use super::discovery::scan_plugins;
use super::manifest::read_manifest;

/// A detected collision between a plugin's key and an existing definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCollision {
    /// The colliding key name.
    pub key: String,
    /// Who owns the existing definition ("core" or a plugin name).
    pub existing_source: String,
    /// The existing definition's description (read-only — author's declared intent).
    pub existing_description: String,
    /// The existing definition's semantic category (read-only).
    pub existing_semantic: Option<String>,
    /// The existing definition's from types.
    pub existing_from: Vec<String>,
    /// The existing definition's to types.
    pub existing_to: Vec<String>,
    /// The incoming plugin's description (read-only — author's declared intent).
    pub incoming_description: String,
    /// The incoming plugin's semantic category (read-only).
    pub incoming_semantic: Option<String>,
    /// The incoming plugin's from types.
    pub incoming_from: Vec<String>,
    /// The incoming plugin's to types.
    pub incoming_to: Vec<String>,
    /// Whether the semantic categories match (suggests same intent).
    pub semantic_match: bool,
}

/// Detect relationship key collisions between a plugin being installed and
/// the existing schema (core.json + already-installed plugins).
///
/// Returns an empty vec when there are no collisions (safe to install).
pub fn detect_relationship_collisions(
    incoming_relationships: &[RelationshipSchema],
    project_root: &std::path::Path,
    incoming_plugin_name: &str,
) -> Vec<KeyCollision> {
    let mut collisions = Vec::new();

    // Build the existing relationship set: (source_name, schema)
    let mut existing: Vec<(String, RelationshipSchema)> = Vec::new();

    // Core relationships from embedded core.json
    for rel in &PLATFORM.relationships {
        existing.push((
            "core".to_string(),
            RelationshipSchema {
                key: rel.key.clone(),
                inverse: rel.inverse.clone(),
                description: rel.description.clone(),
                from: rel.from.clone(),
                to: rel.to.clone(),
                semantic: rel.semantic.clone(),
                constraints: None,
            },
        ));
    }

    // Already-installed plugin relationships
    let installed = scan_plugins(project_root);
    for plugin in &installed {
        if plugin.name == incoming_plugin_name {
            continue; // Skip self on reinstall/update
        }
        let plugin_dir = std::path::Path::new(&plugin.path);
        if let Ok(manifest) = read_manifest(plugin_dir) {
            for rel_value in &manifest.provides.relationships {
                if let Ok(schema) = serde_json::from_value::<RelationshipSchema>(rel_value.clone())
                {
                    existing.push((plugin.name.clone(), schema));
                }
            }
        }
    }

    // Check each incoming key against what's already defined
    for incoming in incoming_relationships {
        for (source, ex) in &existing {
            if ex.key == incoming.key {
                let semantic_match = ex.semantic == incoming.semantic;
                collisions.push(KeyCollision {
                    key: incoming.key.clone(),
                    existing_source: source.clone(),
                    existing_description: ex.description.clone(),
                    existing_semantic: ex.semantic.clone(),
                    existing_from: ex.from.clone(),
                    existing_to: ex.to.clone(),
                    incoming_description: incoming.description.clone(),
                    incoming_semantic: incoming.semantic.clone(),
                    incoming_from: incoming.from.clone(),
                    incoming_to: incoming.to.clone(),
                    semantic_match,
                });
                break;
            }
        }
    }

    collisions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_schema(key: &str, semantic: &str, from: &[&str], to: &[&str]) -> RelationshipSchema {
        RelationshipSchema {
            key: key.to_string(),
            inverse: format!("{key}-inverse"),
            description: format!("Test {key}"),
            from: from.iter().map(|s| s.to_string()).collect(),
            to: to.iter().map(|s| s.to_string()).collect(),
            semantic: Some(semantic.to_string()),
            constraints: None,
        }
    }

    #[test]
    fn no_collision_for_unique_keys() {
        let incoming = vec![make_schema("brand-new-rel", "custom", &["foo"], &["bar"])];
        let collisions =
            detect_relationship_collisions(&incoming, &PathBuf::from("/nonexistent"), "test");
        assert!(collisions.is_empty());
    }

    #[test]
    fn detects_collision_with_core() {
        // "grounded" exists in core.json
        let incoming = vec![make_schema("grounded", "foundation", &["research"], &["pillar"])];
        let collisions =
            detect_relationship_collisions(&incoming, &PathBuf::from("/nonexistent"), "test");
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0].key, "grounded");
        assert_eq!(collisions[0].existing_source, "core");
        assert!(collisions[0].semantic_match);
    }

    #[test]
    fn semantic_mismatch_flagged() {
        let incoming = vec![make_schema("grounded", "lineage", &["task"], &["task"])];
        let collisions =
            detect_relationship_collisions(&incoming, &PathBuf::from("/nonexistent"), "test");
        assert_eq!(collisions.len(), 1);
        assert!(!collisions[0].semantic_match);
    }
}
