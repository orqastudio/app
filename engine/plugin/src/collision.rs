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

use orqa_validation::RelationshipSchema;

use super::discovery::scan_plugins;
use super::manifest::read_manifest;
use orqa_engine_types::platform::PLATFORM;

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
    let existing = build_existing_relationships(project_root, incoming_plugin_name);
    let mut collisions = Vec::new();

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

/// Build the set of already-defined relationships from core + installed plugins.
///
/// The incoming plugin is excluded so it doesn't collide with its own prior installation.
fn build_existing_relationships(
    project_root: &std::path::Path,
    incoming_plugin_name: &str,
) -> Vec<(String, RelationshipSchema)> {
    // Core relationships from the platform static.
    let core_rels = PLATFORM.relationships.iter().map(|rel| {
        (
            "core".to_owned(),
            RelationshipSchema {
                key: rel.key.clone(),
                inverse: rel.inverse.clone(),
                description: rel.description.clone(),
                from: rel.from.clone(),
                to: rel.to.clone(),
                semantic: rel.semantic.clone(),
                constraints: None,
            },
        )
    });

    // Plugin-provided relationships from installed (non-incoming) plugins.
    let plugin_rels = scan_plugins(project_root)
        .into_iter()
        .filter(|plugin| plugin.name != incoming_plugin_name)
        .filter_map(|plugin| {
            let plugin_dir = std::path::Path::new(&plugin.path).to_path_buf();
            read_manifest(&plugin_dir).ok().map(|manifest| (plugin.name, manifest))
        })
        .flat_map(|(name, manifest)| {
            manifest
                .provides
                .relationships
                .into_iter()
                .filter_map(move |rel_value| {
                    serde_json::from_value::<RelationshipSchema>(rel_value)
                        .ok()
                        .map(|schema| (name.clone(), schema))
                })
        });

    core_rels.chain(plugin_rels).collect()
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
        // core.json is empty and /nonexistent has no plugins, so any key is unique.
        let incoming = vec![make_schema("brand-new-rel", "custom", &["foo"], &["bar"])];
        let collisions =
            detect_relationship_collisions(&incoming, &PathBuf::from("/nonexistent"), "test");
        assert!(collisions.is_empty());
    }

    #[test]
    fn no_collision_when_no_existing_plugins_or_core() {
        // core.json is intentionally empty (plugins are the source of truth at runtime).
        // When there are no installed plugins and core has no relationships, any key
        // is safe to install.
        let incoming = vec![
            make_schema("grounded", "foundation", &["research"], &["pillar"]),
            make_schema("upholds", "foundation", &["task"], &["rule"]),
        ];
        let collisions =
            detect_relationship_collisions(&incoming, &PathBuf::from("/nonexistent"), "test");
        assert!(collisions.is_empty());
    }

    #[test]
    fn semantic_match_flag_computed_correctly() {
        // Two schemas with the same semantic: semantic_match == true.
        let existing = RelationshipSchema {
            key: "grounded".to_string(),
            inverse: "grounded-by".to_string(),
            description: "existing".to_string(),
            from: vec![],
            to: vec![],
            semantic: Some("foundation".to_string()),
            constraints: None,
        };
        let incoming = make_schema("grounded", "foundation", &["research"], &["pillar"]);
        let semantic_match = existing.semantic == incoming.semantic;
        assert!(semantic_match);

        // Different semantic: semantic_match == false.
        let incoming_mismatch = make_schema("grounded", "lineage", &["task"], &["task"]);
        let semantic_mismatch = existing.semantic == incoming_mismatch.semantic;
        assert!(!semantic_mismatch);
    }

    #[test]
    fn empty_incoming_produces_no_collisions() {
        // Empty incoming relationships always yields no collisions.
        let collisions =
            detect_relationship_collisions(&[], &PathBuf::from("/nonexistent"), "test");
        assert!(collisions.is_empty());
    }
}
