//! Platform configuration loaded from the embedded `core.json`.
//!
//! Minimal subset of the platform config needed for artifact type inference
//! from ID prefixes (e.g. "RULE-006" → "rule").

use serde::Deserialize;
use std::sync::LazyLock;

/// The platform core config JSON, embedded at compile time from the canonical source.
///
/// Path is relative to this source file: `libs/lsp-server/src/platform.rs`
/// → `libs/types/src/platform/core.json`
const PLATFORM_JSON: &str = include_str!("../../types/src/platform/core.json");

/// An artifact type from core.json — key and ID prefix only.
#[derive(Debug, Clone, Deserialize)]
pub struct ArtifactTypeDef {
    pub key: String,
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
}

/// Minimal platform config needed for type inference.
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    #[serde(rename = "artifactTypes")]
    pub artifact_types: Vec<ArtifactTypeDef>,
}

/// Lazily-parsed platform config, available for the lifetime of the process.
pub static PLATFORM: LazyLock<PlatformConfig> = LazyLock::new(|| {
    serde_json::from_str(PLATFORM_JSON).expect("platform core.json must be valid JSON")
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_loads_artifact_types() {
        assert!(!PLATFORM.artifact_types.is_empty());
    }

    #[test]
    fn rule_prefix_maps_to_rule_type() {
        let matched = PLATFORM
            .artifact_types
            .iter()
            .find(|t| t.id_prefix == "RULE")
            .map(|t| t.key.as_str());
        assert_eq!(matched, Some("rule"));
    }

    #[test]
    fn know_prefix_maps_to_knowledge_type() {
        let matched = PLATFORM
            .artifact_types
            .iter()
            .find(|t| t.id_prefix == "KNOW")
            .map(|t| t.key.as_str());
        assert_eq!(matched, Some("knowledge"));
    }
}
