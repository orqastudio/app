//! Plugin registry — fetch and cache official and community plugin catalogs.
//!
//! Catalogs are fetched from GitHub and cached in-memory with a 1-hour TTL.
//! The registry is the discovery mechanism for installable plugins; it does not
//! manage the installation itself (see installer.rs for that).

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use orqa_engine_types::error::EngineError;

const OFFICIAL_URL: &str =
    "https://raw.githubusercontent.com/orqastudio/orqastudio-official-plugins/main/registry.json";
const COMMUNITY_URL: &str =
    "https://raw.githubusercontent.com/orqastudio/orqastudio-community-plugins/main/registry.json";
const CACHE_TTL: Duration = Duration::from_secs(3600); // 1 hour

/// A plugin entry in a registry catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    /// The plugin's package name (e.g. `@orqastudio/plugin-software`).
    pub name: String,
    /// Human-readable display name shown in the plugin browser.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// One-line description of the plugin.
    pub description: String,
    /// GitHub repository slug (e.g. `orqastudio/plugin-software`).
    pub repo: String,
    /// Category label for UI grouping (e.g. `"methodology"`, `"tool"`).
    pub category: String,
    /// Lucide icon name for display in the plugin browser.
    pub icon: String,
    /// Tool capability identifiers this plugin grants.
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Optional requirements (other plugins or capabilities).
    #[serde(default)]
    pub requires: serde_json::Value,
}

/// A registry catalog fetched from GitHub.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegistryCatalog {
    /// Catalog format version (currently `1`).
    pub version: u32,
    /// Registry source identifier (e.g. `"official"`, `"community"`).
    pub source: String,
    /// Plugin entries in this catalog.
    pub plugins: Vec<RegistryEntry>,
}

/// Internal cache entry storing a fetched catalog and when it was fetched.
struct CacheEntry {
    data: RegistryCatalog,
    fetched_at: Instant,
}

/// Registry cache — holds fetched catalogs with TTL-based invalidation.
///
/// One instance is shared across the process lifetime (e.g. via `Arc<RegistryCache>`
/// in the app state). Cache entries expire after `CACHE_TTL` and are re-fetched on
/// the next call.
pub struct RegistryCache {
    official: Mutex<Option<CacheEntry>>,
    community: Mutex<Option<CacheEntry>>,
}

impl RegistryCache {
    /// Create a new, empty registry cache.
    pub fn new() -> Self {
        Self {
            official: Mutex::new(None),
            community: Mutex::new(None),
        }
    }

    /// Fetch a registry catalog by source name, using the in-memory cache if fresh.
    ///
    /// `source` must be "official" or "community". Returns `EngineError::Plugin`
    /// for unknown sources or network failures.
    pub async fn fetch(&self, source: &str) -> Result<RegistryCatalog, EngineError> {
        let (cache_mutex, url) = match source {
            "official" => (&self.official, OFFICIAL_URL),
            "community" => (&self.community, COMMUNITY_URL),
            _ => {
                return Err(EngineError::Plugin(format!(
                    "unknown registry source: {source}"
                )))
            }
        };

        // Return cached data if it is still fresh.
        if let Ok(guard) = cache_mutex.lock() {
            if let Some(entry) = guard.as_ref() {
                if entry.fetched_at.elapsed() < CACHE_TTL {
                    return Ok(entry.data.clone());
                }
            }
        }

        // Fetch from remote.
        let response = reqwest::get(url)
            .await
            .map_err(|e| EngineError::Plugin(format!("failed to fetch {source} registry: {e}")))?;

        if !response.status().is_success() {
            return Err(EngineError::Plugin(format!(
                "registry fetch failed: HTTP {}",
                response.status()
            )));
        }

        let catalog: RegistryCatalog = response
            .json()
            .await
            .map_err(|e| EngineError::Plugin(format!("invalid registry JSON: {e}")))?;

        // Store in cache.
        if let Ok(mut guard) = cache_mutex.lock() {
            *guard = Some(CacheEntry {
                data: catalog.clone(),
                fetched_at: Instant::now(),
            });
        }

        Ok(catalog)
    }

    /// Invalidate all cached entries, forcing the next fetch to hit the network.
    pub fn invalidate(&self) {
        if let Ok(mut guard) = self.official.lock() {
            *guard = None;
        }
        if let Ok(mut guard) = self.community.lock() {
            *guard = None;
        }
    }
}

impl Default for RegistryCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_registry_entry() {
        let json = r#"{
            "name": "@orqastudio/plugin-claude",
            "displayName": "Claude Integration",
            "description": "Claude AI provider.",
            "repo": "orqastudio/orqastudio-plugin-claude",
            "category": "ai-provider",
            "icon": "bot",
            "capabilities": ["sidecar", "hooks"],
            "requires": { "node": ">=20" }
        }"#;

        let entry: RegistryEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.name, "@orqastudio/plugin-claude");
        assert_eq!(entry.capabilities, vec!["sidecar", "hooks"]);
    }

    #[test]
    fn deserialize_catalog() {
        let json = r#"{
            "version": 1,
            "source": "official",
            "plugins": []
        }"#;

        let catalog: RegistryCatalog = serde_json::from_str(json).unwrap();
        assert_eq!(catalog.version, 1);
        assert!(catalog.plugins.is_empty());
    }

    #[test]
    fn registry_cache_new_is_empty() {
        let cache = RegistryCache::new();
        // A freshly created cache should have no entries.
        let official = cache.official.lock().unwrap();
        assert!(official.is_none());
        let community = cache.community.lock().unwrap();
        assert!(community.is_none());
    }

    #[test]
    fn registry_cache_default_is_same_as_new() {
        let cache = RegistryCache::default();
        let official = cache.official.lock().unwrap();
        assert!(official.is_none());
    }

    #[test]
    fn registry_cache_invalidate_clears_entries() {
        let cache = RegistryCache::new();
        // Manually populate a cache entry.
        {
            let mut guard = cache.official.lock().unwrap();
            *guard = Some(CacheEntry {
                data: RegistryCatalog {
                    version: 1,
                    source: "official".to_owned(),
                    plugins: vec![],
                },
                fetched_at: Instant::now(),
            });
        }
        // Invalidate should clear it.
        cache.invalidate();
        let guard = cache.official.lock().unwrap();
        assert!(guard.is_none(), "cache should be empty after invalidate");
    }

    #[test]
    fn registry_entry_default_capabilities_is_empty() {
        // Capabilities field defaults to empty when absent from JSON.
        let json = r#"{
            "name": "@test/plugin",
            "displayName": "Test",
            "description": "desc",
            "repo": "test/plugin",
            "category": "test",
            "icon": "circle"
        }"#;
        let entry: RegistryEntry = serde_json::from_str(json).unwrap();
        assert!(entry.capabilities.is_empty());
    }
}
