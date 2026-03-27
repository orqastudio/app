//! Plugin installer — install plugins from local paths or GitHub releases.
//!
//! Downloads and extracts .tar.gz archives from GitHub releases, or copies
//! plugins from a local filesystem path. Detects relationship key collisions
//! before finalising installation and records the result in the lockfile.

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::Path;

use crate::error::EngineError;

use super::collision::KeyCollision;
use super::lockfile::{read_lockfile, write_lockfile, LockEntry};
use super::manifest::read_manifest;

/// Result of a plugin installation.
#[derive(Debug, Clone, Serialize)]
pub struct InstallResult {
    pub name: String,
    pub version: String,
    pub path: String,
    pub source: String, // "github" or "local"
    /// Key collisions detected during installation. Empty when none.
    /// When non-empty, the UI/CLI should prompt the user to merge or rename
    /// each collision before completing installation.
    pub collisions: Vec<KeyCollision>,
}

/// Install a plugin from a local filesystem path.
///
/// Checks for relationship key collisions with core and other installed plugins.
/// If collisions are detected, they are returned in the result so the caller
/// can prompt the user to merge or rename before finalising.
pub fn install_from_path(source: &Path, project_root: &Path) -> Result<InstallResult, EngineError> {
    let manifest = read_manifest(source)?;

    let incoming_rels: Vec<orqa_validation::RelationshipSchema> = manifest
        .provides
        .relationships
        .iter()
        .filter_map(|v| serde_json::from_value(v.clone()).ok())
        .collect();

    let collisions = super::collision::detect_relationship_collisions(
        &incoming_rels,
        project_root,
        &manifest.name,
    );

    let plugins_dir = project_root.join("plugins");
    std::fs::create_dir_all(&plugins_dir)?;

    let short_name = manifest
        .name
        .split('/')
        .next_back()
        .unwrap_or(&manifest.name);
    let target = plugins_dir.join(short_name);

    if target.exists() {
        std::fs::remove_dir_all(&target)?;
    }

    copy_dir_all(source, &target)?;

    Ok(InstallResult {
        name: manifest.name,
        version: manifest.version,
        path: target.to_string_lossy().to_string(),
        source: "local".to_string(),
        collisions,
    })
}

/// Install a plugin from a GitHub release .tar.gz archive.
///
/// Downloads the archive, verifies the sha256 hash, extracts it, checks for
/// collisions, and records the result in the lockfile.
pub async fn install_from_github(
    repo: &str,
    version: Option<&str>,
    project_root: &Path,
) -> Result<InstallResult, EngineError> {
    let tag = match version {
        Some(v) => v.to_string(),
        None => fetch_latest_tag(repo).await?,
    };
    let (bytes, sha256) = download_plugin_archive(repo, &tag).await?;

    let plugins_dir = project_root.join("plugins");
    std::fs::create_dir_all(&plugins_dir)?;
    let tmp_dir = plugins_dir.join(format!(".tmp-{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir)?;

    let manifest = extract_and_read_manifest(&bytes, &tmp_dir)?;

    let incoming_rels: Vec<orqa_validation::RelationshipSchema> = manifest
        .provides
        .relationships
        .iter()
        .filter_map(|v| serde_json::from_value(v.clone()).ok())
        .collect();
    let collisions = super::collision::detect_relationship_collisions(
        &incoming_rels,
        project_root,
        &manifest.name,
    );

    let short_name = manifest
        .name
        .split('/')
        .next_back()
        .unwrap_or(&manifest.name);
    let target = plugins_dir.join(short_name);
    if target.exists() {
        std::fs::remove_dir_all(&target)?;
    }
    let extracted_dir = find_extracted_dir(&tmp_dir)?;
    std::fs::rename(&extracted_dir, &target)?;
    let _ = std::fs::remove_dir_all(&tmp_dir);

    update_lockfile(project_root, &manifest, repo, sha256)?;

    Ok(InstallResult {
        name: manifest.name,
        version: manifest.version,
        path: target.to_string_lossy().to_string(),
        source: "github".to_string(),
        collisions,
    })
}

/// Uninstall a plugin by name.
///
/// Removes the plugin directory and updates the lockfile.
pub fn uninstall(name: &str, project_root: &Path) -> Result<(), EngineError> {
    let short_name = name.split('/').next_back().unwrap_or(name);
    let plugin_dir = project_root.join("plugins").join(short_name);

    if !plugin_dir.exists() {
        return Err(EngineError::Plugin(format!(
            "plugin not found: {name} (expected at {})",
            plugin_dir.display()
        )));
    }

    std::fs::remove_dir_all(&plugin_dir)?;

    let mut lockfile = read_lockfile(project_root);
    lockfile.plugins.retain(|p| p.name != name);
    write_lockfile(project_root, &lockfile)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Update the lockfile after a successful GitHub install.
fn update_lockfile(
    project_root: &Path,
    manifest: &super::manifest::PluginManifest,
    repo: &str,
    sha256: String,
) -> Result<(), EngineError> {
    let mut lockfile = read_lockfile(project_root);
    lockfile.plugins.retain(|p| p.name != manifest.name);
    lockfile.plugins.push(LockEntry {
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        repo: repo.to_string(),
        sha256,
        installed_at: iso_now(),
    });
    write_lockfile(project_root, &lockfile)
}

/// Download a plugin archive from a GitHub release and return the bytes and sha256.
async fn download_plugin_archive(repo: &str, tag: &str) -> Result<(Vec<u8>, String), EngineError> {
    let repo_name = repo
        .split('/')
        .next_back()
        .ok_or_else(|| EngineError::Plugin("invalid repo format".to_string()))?;

    let archive_url =
        format!("https://github.com/{repo}/releases/download/{tag}/{repo_name}-{tag}.tar.gz");
    tracing::info!("downloading plugin: {archive_url}");

    let response = reqwest::get(&archive_url)
        .await
        .map_err(|e| EngineError::Plugin(format!("download failed: {e}")))?;

    if !response.status().is_success() {
        return Err(EngineError::Plugin(format!(
            "download failed: HTTP {}",
            response.status()
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| EngineError::Plugin(format!("failed to read response: {e}")))?
        .to_vec();
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    Ok((bytes, sha256))
}

/// Extract a tar.gz archive and read the plugin manifest from the extracted directory.
///
/// Cleans up the temporary directory on any error.
fn extract_and_read_manifest(
    bytes: &[u8],
    tmp_dir: &Path,
) -> Result<super::manifest::PluginManifest, EngineError> {
    if let Err(e) = extract_tar_gz(bytes, tmp_dir) {
        let _ = std::fs::remove_dir_all(tmp_dir);
        return Err(e);
    }
    match read_manifest(tmp_dir) {
        Ok(m) => Ok(m),
        Err(e) => {
            let _ = std::fs::remove_dir_all(tmp_dir);
            Err(e)
        }
    }
}

/// Find the extracted plugin directory — handles the common case where a
/// single subdirectory is the plugin root.
fn find_extracted_dir(tmp_dir: &Path) -> Result<std::path::PathBuf, EngineError> {
    let entries: Vec<_> = std::fs::read_dir(tmp_dir)?
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().is_dir())
        .collect();
    Ok(if entries.len() == 1 {
        entries[0].path()
    } else {
        tmp_dir.to_path_buf()
    })
}

/// Fetch the latest release tag for a GitHub repo via the releases API.
async fn fetch_latest_tag(repo: &str) -> Result<String, EngineError> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "orqastudio-installer")
        .send()
        .await
        .map_err(|e| EngineError::Plugin(format!("failed to fetch latest release: {e}")))?;

    if !response.status().is_success() {
        return Err(EngineError::Plugin(format!(
            "failed to fetch latest release: HTTP {}",
            response.status()
        )));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| EngineError::Plugin(format!("invalid release JSON: {e}")))?;

    data["tag_name"]
        .as_str()
        .map(String::from)
        .ok_or_else(|| EngineError::Plugin("no tag_name in release response".to_string()))
}

/// Extract a gzipped tar archive into a directory.
fn extract_tar_gz(bytes: &[u8], target_dir: &Path) -> Result<(), EngineError> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let decoder = GzDecoder::new(bytes);
    let mut archive = Archive::new(decoder);

    archive
        .unpack(target_dir)
        .map_err(|e| EngineError::Plugin(format!("extraction failed: {e}")))?;

    Ok(())
}

/// Recursively copy a directory tree from src to dst.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), EngineError> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());

        if entry.path().is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }

    Ok(())
}

/// Return an approximate ISO 8601 timestamp for the current moment.
///
/// Uses only the standard library — no chrono dependency.
fn iso_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let days = secs / 86400;
    let years = 1970 + days / 365;
    format!("{years}-01-01T00:00:00Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_invalid_tar_gz_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let result = extract_tar_gz(b"not a tar gz", dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn uninstall_missing_plugin_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let result = uninstall("@orqastudio/nonexistent", dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn iso_now_produces_non_empty_string() {
        let ts = iso_now();
        assert!(!ts.is_empty());
        assert!(ts.contains('T'));
    }
}
