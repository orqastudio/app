//! Plugin lockfile — read/write `plugins.lock.json`.
//!
//! The lockfile records the exact version and sha256 hash of each installed plugin,
//! enabling reproducible installs and upgrade detection.

use serde::{Deserialize, Serialize};
use std::path::Path;

use orqa_engine_types::error::EngineError;

const LOCKFILE_NAME: &str = "plugins.lock.json";

/// A locked plugin version entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEntry {
    /// The plugin's package name (e.g. `@orqastudio/plugin-software`).
    pub name: String,
    /// Semantic version string at install time (e.g. `1.2.0`).
    pub version: String,
    /// GitHub repository slug (e.g. `orqastudio/plugin-software`).
    pub repo: String,
    /// SHA-256 hex digest of the downloaded archive at install time.
    pub sha256: String,
    /// ISO-8601 timestamp when this plugin was installed.
    #[serde(rename = "installedAt")]
    pub installed_at: String,
}

/// The lockfile structure — version-stamped list of locked plugin entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    /// Lockfile format version (currently `1`).
    pub version: u32,
    /// Locked entries for all installed plugins.
    pub plugins: Vec<LockEntry>,
}

/// Read the lockfile from the project root.
///
/// Returns an empty lockfile (version 1, no plugins) if the file does not exist
/// or cannot be parsed — install operations always start from a valid state.
pub fn read_lockfile(project_root: &Path) -> Lockfile {
    let path = project_root.join(LOCKFILE_NAME);

    if !path.exists() {
        return Lockfile {
            version: 1,
            plugins: vec![],
        };
    }

    match std::fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or(Lockfile {
            version: 1,
            plugins: vec![],
        }),
        Err(_) => Lockfile {
            version: 1,
            plugins: vec![],
        },
    }
}

/// Write the lockfile to the project root.
///
/// Serializes to pretty-printed JSON with a trailing newline.
pub fn write_lockfile(project_root: &Path, lockfile: &Lockfile) -> Result<(), EngineError> {
    let path = project_root.join(LOCKFILE_NAME);
    let contents = serde_json::to_string_pretty(lockfile)?;
    std::fs::write(&path, format!("{contents}\n"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn read_missing_lockfile_returns_empty() {
        let lockfile = read_lockfile(&PathBuf::from("/nonexistent"));
        assert_eq!(lockfile.version, 1);
        assert!(lockfile.plugins.is_empty());
    }

    #[test]
    fn roundtrip_lockfile() {
        let dir = tempfile::tempdir().unwrap();
        let lockfile = Lockfile {
            version: 1,
            plugins: vec![LockEntry {
                name: "@orqastudio/test".to_string(),
                version: "0.1.0".to_string(),
                repo: "orqastudio/test".to_string(),
                sha256: "abc123".to_string(),
                installed_at: "2026-03-17T00:00:00Z".to_string(),
            }],
        };

        write_lockfile(dir.path(), &lockfile).unwrap();
        let read_back = read_lockfile(dir.path());

        assert_eq!(read_back.plugins.len(), 1);
        assert_eq!(read_back.plugins[0].name, "@orqastudio/test");
    }
}
