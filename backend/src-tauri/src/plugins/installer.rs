//! Plugin installer — download and extract .tar.gz archives from GitHub releases.
//!
//! Prefers shelling out to `orqa plugin install` if the CLI is available,
//! falls back to built-in download + extract using tar + flate2 crates.

use sha2::{Digest, Sha256};
use serde::Serialize;
use std::path::Path;

use crate::error::OrqaError;

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
    pub collisions: Vec<super::collision::KeyCollision>,
}

/// Install a plugin from a local filesystem path.
///
/// Checks for relationship key collisions with core and other installed
/// plugins. If collisions are detected, they are returned in the result
/// so the caller can prompt the user to merge or rename before finalizing.
pub fn install_from_path(source: &Path, project_root: &Path) -> Result<InstallResult, OrqaError> {
    let manifest = read_manifest(source)?;

    // Parse incoming relationship schemas for collision detection
    let incoming_rels: Vec<crate::domain::integrity_engine::RelationshipSchema> = manifest
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

    let short_name = manifest.name.split('/').next_back().unwrap_or(&manifest.name);
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
pub async fn install_from_github(
    repo: &str,
    version: Option<&str>,
    project_root: &Path,
) -> Result<InstallResult, OrqaError> {
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

    let incoming_rels: Vec<crate::domain::integrity_engine::RelationshipSchema> = manifest
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

    let short_name = manifest.name.split('/').next_back().unwrap_or(&manifest.name);
    let target = plugins_dir.join(short_name);
    if target.exists() {
        std::fs::remove_dir_all(&target)?;
    }
    let extracted_dir = find_extracted_dir(&tmp_dir)?;
    std::fs::rename(&extracted_dir, &target)?;
    let _ = std::fs::remove_dir_all(&tmp_dir);

    let mut lockfile = read_lockfile(project_root);
    lockfile.plugins.retain(|p| p.name != manifest.name);
    lockfile.plugins.push(LockEntry {
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        repo: repo.to_string(),
        sha256,
        installed_at: chrono_now_iso(),
    });
    write_lockfile(project_root, &lockfile)?;

    Ok(InstallResult {
        name: manifest.name,
        version: manifest.version,
        path: target.to_string_lossy().to_string(),
        source: "github".to_string(),
        collisions,
    })
}

async fn download_plugin_archive(
    repo: &str,
    tag: &str,
) -> Result<(Vec<u8>, String), OrqaError> {
    let repo_name = repo
        .split('/')
        .next_back()
        .ok_or_else(|| OrqaError::Plugin("invalid repo format".to_string()))?;

    let archive_url =
        format!("https://github.com/{repo}/releases/download/{tag}/{repo_name}-{tag}.tar.gz");
    tracing::info!("downloading plugin: {archive_url}");

    let response = reqwest::get(&archive_url)
        .await
        .map_err(|e| OrqaError::Plugin(format!("download failed: {e}")))?;

    if !response.status().is_success() {
        return Err(OrqaError::Plugin(format!(
            "download failed: HTTP {}",
            response.status()
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| OrqaError::Plugin(format!("failed to read response: {e}")))?
        .to_vec();
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    Ok((bytes, sha256))
}

fn extract_and_read_manifest(
    bytes: &[u8],
    tmp_dir: &Path,
) -> Result<super::manifest::PluginManifest, OrqaError> {
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

fn find_extracted_dir(tmp_dir: &Path) -> Result<std::path::PathBuf, OrqaError> {
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

/// Uninstall a plugin by name.
pub fn uninstall(name: &str, project_root: &Path) -> Result<(), OrqaError> {
    let short_name = name.split('/').next_back().unwrap_or(name);
    let plugin_dir = project_root.join("plugins").join(short_name);

    if !plugin_dir.exists() {
        return Err(OrqaError::Plugin(format!(
            "plugin not found: {name} (expected at {})",
            plugin_dir.display()
        )));
    }

    std::fs::remove_dir_all(&plugin_dir)?;

    // Update lockfile
    let mut lockfile = read_lockfile(project_root);
    lockfile.plugins.retain(|p| p.name != name);
    write_lockfile(project_root, &lockfile)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_tar_gz(bytes: &[u8], target_dir: &Path) -> Result<(), OrqaError> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let decoder = GzDecoder::new(bytes);
    let mut archive = Archive::new(decoder);

    archive
        .unpack(target_dir)
        .map_err(|e| OrqaError::Plugin(format!("extraction failed: {e}")))?;

    Ok(())
}

async fn fetch_latest_tag(repo: &str) -> Result<String, OrqaError> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "orqastudio-installer")
        .send()
        .await
        .map_err(|e| OrqaError::Plugin(format!("failed to fetch latest release: {e}")))?;

    if !response.status().is_success() {
        return Err(OrqaError::Plugin(format!(
            "failed to fetch latest release: HTTP {}",
            response.status()
        )));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| OrqaError::Plugin(format!("invalid release JSON: {e}")))?;

    data["tag_name"]
        .as_str()
        .map(String::from)
        .ok_or_else(|| OrqaError::Plugin("no tag_name in release response".to_string()))
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), OrqaError> {
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

fn chrono_now_iso() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    // Simple ISO 8601 approximation (no chrono dependency)
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
}
