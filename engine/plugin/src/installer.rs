//! Plugin installer — install plugins from local paths or GitHub releases.
//!
//! Downloads and extracts .tar.gz archives from GitHub releases, or copies
//! plugins from a local filesystem path. Detects relationship key collisions
//! and enforces installation constraints (one methodology plugin per project,
//! one workflow plugin per stage slot) before finalising installation.
//! Records the result in the lockfile.

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::Path;

use orqa_engine_types::error::EngineError;

use super::collision::KeyCollision;
use super::constraints::{check_one_methodology, check_one_per_stage};
use super::lockfile::{read_lockfile, write_lockfile, LockEntry};
use super::manifest::read_manifest;

/// Result of a plugin installation.
#[derive(Debug, Clone, Serialize)]
pub struct InstallResult {
    /// The plugin's package name (e.g. `@orqastudio/plugin-software`).
    pub name: String,
    /// Semantic version string from the manifest (e.g. `1.2.0`).
    pub version: String,
    /// Absolute filesystem path to the installed plugin directory.
    pub path: String,
    /// Install source: `"github"` or `"local"`.
    pub source: String,
    /// Key collisions detected during installation. Empty when none.
    /// When non-empty, the UI/CLI should prompt the user to merge or rename
    /// each collision before completing installation.
    pub collisions: Vec<KeyCollision>,
    /// True when the installed plugin declares `affects_schema: true`.
    /// The caller must trigger schema recomposition after installation.
    pub requires_schema_recomposition: bool,
    /// True when the installed plugin declares enforcement entries.
    /// The caller must trigger enforcement config regeneration after installation.
    pub requires_enforcement_regeneration: bool,
}

/// Install a plugin from a local filesystem path.
///
/// Enforces installation constraints (P5-26: one methodology plugin per project,
/// P5-27: one workflow plugin per stage slot) before proceeding. Checks for
/// relationship key collisions with core and other installed plugins.
/// If collisions are detected, they are returned in the result so the caller
/// can prompt the user to merge or rename before finalising.
/// The result flag `requires_schema_recomposition` indicates what post-install
/// actions the caller must trigger.
pub fn install_from_path(source: &Path, project_root: &Path) -> Result<InstallResult, EngineError> {
    let manifest = read_manifest(source)?;

    // P5-26: enforce one-methodology constraint.
    if let Err(e) = check_one_methodology(&manifest, project_root) {
        tracing::warn!(
            plugin = %manifest.name,
            constraint = "one-methodology",
            message = %e.message,
            "[plugins] install constraint violated"
        );
        return Err(EngineError::from(e));
    }

    // P5-27: enforce one-per-stage constraint.
    if let Err(e) = check_one_per_stage(&manifest, project_root) {
        tracing::warn!(
            plugin = %manifest.name,
            constraint = "one-per-stage",
            message = %e.message,
            "[plugins] install constraint violated"
        );
        return Err(EngineError::from(e));
    }

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

    // P5-28: read post-install action flags from the manifest.
    let requires_schema_recomposition = manifest.install_constraints.affects_schema;
    // A non-empty enforcement array means the plugin participates in enforcement generation.
    let requires_enforcement_regeneration = !manifest.enforcement.is_empty();

    let result = InstallResult {
        name: manifest.name,
        version: manifest.version,
        path: target.to_string_lossy().into_owned(),
        source: "local".to_owned(),
        collisions,
        requires_schema_recomposition,
        requires_enforcement_regeneration,
    };

    tracing::info!(
        plugin = %result.name,
        version = %result.version,
        "[plugins] install_from_path succeeded"
    );

    Ok(result)
}

/// Install a plugin from a GitHub release .tar.gz archive.
///
/// Downloads the archive, verifies the sha256 hash, extracts it, enforces
/// installation constraints (P5-26/P5-27), checks for collisions, and records
/// the result in the lockfile. The result flags indicate what post-install
/// actions the caller must trigger (P5-28).
pub async fn install_from_github(
    repo: &str,
    version: Option<&str>,
    project_root: &Path,
) -> Result<InstallResult, EngineError> {
    let tag = match version {
        Some(v) => v.to_owned(),
        None => fetch_latest_tag(repo).await?,
    };
    let (bytes, sha256) = download_plugin_archive(repo, &tag).await?;

    let plugins_dir = project_root.join("plugins");
    std::fs::create_dir_all(&plugins_dir)?;
    let tmp_dir = plugins_dir.join(format!(".tmp-{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir)?;

    let manifest = extract_and_read_manifest(&bytes, &tmp_dir)?;
    finalize_github_install(manifest, plugins_dir, tmp_dir, project_root, repo, &tag, sha256)
}

/// Enforce constraints, move extracted plugin into place, and build the result.
///
/// Called by `install_from_github` after extraction. Cleans up the temp directory
/// on any constraint or I/O error before returning. `resolved_ref` is the git tag
/// that was resolved for this install, included in the success log for traceability.
fn finalize_github_install(
    manifest: super::manifest::PluginManifest,
    plugins_dir: std::path::PathBuf,
    tmp_dir: std::path::PathBuf,
    project_root: &Path,
    repo: &str,
    resolved_ref: &str,
    sha256: String,
) -> Result<InstallResult, EngineError> {
    // P5-26: enforce one-methodology constraint before moving files.
    if let Err(e) = check_one_methodology(&manifest, project_root) {
        tracing::warn!(
            plugin = %manifest.name,
            constraint = "one-methodology",
            message = %e.message,
            "[plugins] install constraint violated"
        );
        let _ = std::fs::remove_dir_all(&tmp_dir);
        return Err(e.into());
    }

    // P5-27: enforce one-per-stage constraint before moving files.
    if let Err(e) = check_one_per_stage(&manifest, project_root) {
        tracing::warn!(
            plugin = %manifest.name,
            constraint = "one-per-stage",
            message = %e.message,
            "[plugins] install constraint violated"
        );
        let _ = std::fs::remove_dir_all(&tmp_dir);
        return Err(e.into());
    }

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

    let short_name = manifest.name.split('/').next_back().unwrap_or(&manifest.name);
    let target = plugins_dir.join(short_name);
    if target.exists() {
        std::fs::remove_dir_all(&target)?;
    }
    let extracted_dir = find_extracted_dir(&tmp_dir)?;
    std::fs::rename(&extracted_dir, &target)?;
    let _ = std::fs::remove_dir_all(&tmp_dir);

    update_lockfile(project_root, &manifest, repo, sha256)?;

    // P5-28: read post-install action flags from the manifest.
    let requires_schema_recomposition = manifest.install_constraints.affects_schema;
    // A plugin that declares any enforcement entries requires enforcement regeneration.
    let requires_enforcement_regeneration = !manifest.enforcement.is_empty();

    let result = InstallResult {
        name: manifest.name,
        version: manifest.version,
        path: target.to_string_lossy().into_owned(),
        source: "github".to_owned(),
        collisions,
        requires_schema_recomposition,
        requires_enforcement_regeneration,
    };

    tracing::info!(
        plugin = %result.name,
        version = %result.version,
        resolved_ref = %resolved_ref,
        "[plugins] install_from_github succeeded"
    );

    Ok(result)
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
        repo: repo.to_owned(),
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
        .ok_or_else(|| EngineError::Plugin("invalid repo format".to_owned()))?;

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
        .filter_map(Result::ok)
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
        .ok_or_else(|| EngineError::Plugin("no tag_name in release response".to_owned()))
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

    /// Write a minimal plugin manifest JSON to a directory for use in tests.
    ///
    /// Uses `enforcement_entries` as a JSON array fragment (e.g. `""` for none,
    /// or a serialized array of enforcement declarations) to drive the
    /// `requires_enforcement_regeneration` flag.
    fn write_plugin_manifest(
        dir: &Path,
        name: &str,
        purpose: &[&str],
        stage_slot: Option<&str>,
        affects_schema: bool,
        enforcement_entries: &str,
    ) {
        let stage_slot_json = match stage_slot {
            Some(s) => format!(r#",
  "stage_slot": "{}""#, s),
            None => String::new(),
        };
        let purpose_json = purpose
            .iter()
            .map(|p| format!(r#""{}""#, p))
            .collect::<Vec<_>>()
            .join(", ");
        // Fields are top-level in the manifest JSON using snake_case, matching actual plugin manifests.
        let manifest = format!(
            r#"{{
  "name": "{}",
  "version": "0.1.0",
  "categories": ["domain-knowledge"],
  "provides": {{}},
  "purpose": [{}],
  "affects_schema": {},
  "enforcement": {}{}
}}"#,
            name, purpose_json, affects_schema, enforcement_entries, stage_slot_json
        );
        std::fs::write(dir.join("orqa-plugin.json"), manifest).unwrap();
    }

    #[test]
    fn install_from_path_sets_schema_recomposition_flag() {
        // A definition plugin (affects_schema: true) must set requires_schema_recomposition.
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        write_plugin_manifest(
            plugin_dir.path(),
            "@orqastudio/plugin-agile-methodology",
            &["methodology"],
            None,
            true,  // affects_schema
            "[]",  // no enforcement entries
        );

        let result = install_from_path(plugin_dir.path(), project_dir.path()).unwrap();
        assert!(result.requires_schema_recomposition);
        assert!(!result.requires_enforcement_regeneration);
    }

    #[test]
    fn install_from_path_sets_enforcement_regeneration_flag() {
        // A plugin with enforcement declarations must set requires_enforcement_regeneration.
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        write_plugin_manifest(
            plugin_dir.path(),
            "@orqastudio/plugin-typescript",
            &["infrastructure"],
            None,
            false, // affects_schema
            r#"[{"role": "generator", "engine": "eslint"}]"#,
        );

        let result = install_from_path(plugin_dir.path(), project_dir.path()).unwrap();
        assert!(!result.requires_schema_recomposition);
        assert!(result.requires_enforcement_regeneration);
    }

    #[test]
    fn install_from_path_no_flags_for_knowledge_plugin() {
        // A knowledge plugin with no enforcement entries must not set either recomposition flag.
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        write_plugin_manifest(
            plugin_dir.path(),
            "@orqastudio/plugin-rust",
            &["knowledge"],
            None,
            false, // affects_schema
            "[]",  // no enforcement entries
        );

        let result = install_from_path(plugin_dir.path(), project_dir.path()).unwrap();
        assert!(!result.requires_schema_recomposition);
        assert!(!result.requires_enforcement_regeneration);
    }

    #[test]
    fn install_from_path_rejects_second_methodology_plugin() {
        // Installing a second methodology plugin must fail.
        // Set up a project with an already-installed methodology plugin.
        let project_dir = tempfile::tempdir().unwrap();
        let plugins_dir = project_dir.path().join("plugins");
        std::fs::create_dir_all(&plugins_dir).unwrap();

        // Write an existing methodology plugin into the plugins/methodology/ sub-dir.
        let methodology_dir = plugins_dir.join("methodology");
        let existing_dir = methodology_dir.join("agile-methodology");
        std::fs::create_dir_all(&existing_dir).unwrap();
        write_plugin_manifest(
            &existing_dir,
            "@orqastudio/plugin-agile-methodology",
            &["methodology"],
            None,
            true,
            "[]",
        );

        // Write a project.json so scan_plugins can find it.
        let orqa_dir = project_dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let project_json = r#"{
            "name": "test",
            "organisation": false,
            "projects": [],
            "artifacts": [],
            "statuses": [],
            "delivery": {},
            "relationships": [],
            "plugins": {
                "@orqastudio/plugin-agile-methodology": {
                    "path": "plugins/methodology/agile-methodology",
                    "installed": true,
                    "enabled": true
                }
            }
        }"#;
        std::fs::write(orqa_dir.join("project.json"), project_json).unwrap();

        // Try to install a different methodology plugin.
        let second_methodology_dir = tempfile::tempdir().unwrap();
        write_plugin_manifest(
            second_methodology_dir.path(),
            "@orqastudio/plugin-scrum-methodology",
            &["methodology"],
            None,
            true,
            "[]",
        );

        let result = install_from_path(second_methodology_dir.path(), project_dir.path());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("agile-methodology"),
            "error should name existing plugin, got: {err}"
        );
        assert!(
            err.contains("methodology"),
            "error should mention methodology, got: {err}"
        );
    }

    #[test]
    fn install_from_path_allows_reinstall_of_same_methodology_plugin() {
        // Reinstalling the same methodology plugin (update) must succeed.
        let project_dir = tempfile::tempdir().unwrap();
        let plugins_dir = project_dir.path().join("plugins");
        std::fs::create_dir_all(&plugins_dir).unwrap();

        let methodology_dir = plugins_dir.join("methodology");
        let existing_dir = methodology_dir.join("agile-methodology");
        std::fs::create_dir_all(&existing_dir).unwrap();
        write_plugin_manifest(
            &existing_dir,
            "@orqastudio/plugin-agile-methodology",
            &["methodology"],
            None,
            true,
            "[]",
        );

        let orqa_dir = project_dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let project_json = r#"{
            "name": "test",
            "organisation": false,
            "projects": [],
            "artifacts": [],
            "statuses": [],
            "delivery": {},
            "relationships": [],
            "plugins": {
                "@orqastudio/plugin-agile-methodology": {
                    "path": "plugins/methodology/agile-methodology",
                    "installed": true,
                    "enabled": true
                }
            }
        }"#;
        std::fs::write(orqa_dir.join("project.json"), project_json).unwrap();

        // Install the same plugin again (same name = update).
        let reinstall_dir = tempfile::tempdir().unwrap();
        write_plugin_manifest(
            reinstall_dir.path(),
            "@orqastudio/plugin-agile-methodology",
            &["methodology"],
            None,
            true,
            "[]",
        );

        let result = install_from_path(reinstall_dir.path(), project_dir.path());
        assert!(
            result.is_ok(),
            "reinstalling same methodology plugin should succeed"
        );
    }

    #[test]
    fn install_from_path_rejects_stage_slot_conflict() {
        // Installing a workflow plugin whose stage_slot is already filled must fail.
        let project_dir = tempfile::tempdir().unwrap();
        let plugins_dir = project_dir.path().join("plugins");
        std::fs::create_dir_all(&plugins_dir).unwrap();

        let existing_dir = plugins_dir.join("agile-discovery");
        std::fs::create_dir_all(&existing_dir).unwrap();
        write_plugin_manifest(
            &existing_dir,
            "@orqastudio/plugin-agile-discovery",
            &["workflow"],
            Some("discovery"),
            true,
            "[]",
        );

        let orqa_dir = project_dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let project_json = r#"{
            "name": "test",
            "organisation": false,
            "projects": [],
            "artifacts": [],
            "statuses": [],
            "delivery": {},
            "relationships": [],
            "plugins": {
                "@orqastudio/plugin-agile-discovery": {
                    "path": "plugins/agile-discovery",
                    "installed": true,
                    "enabled": true
                }
            }
        }"#;
        std::fs::write(orqa_dir.join("project.json"), project_json).unwrap();

        // Try to install a different plugin filling the same stage slot.
        let conflict_dir = tempfile::tempdir().unwrap();
        write_plugin_manifest(
            conflict_dir.path(),
            "@orqastudio/plugin-custom-discovery",
            &["workflow"],
            Some("discovery"),
            true,
            "[]",
        );

        let result = install_from_path(conflict_dir.path(), project_dir.path());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("discovery"),
            "error should name the stage slot, got: {err}"
        );
        assert!(
            err.contains("agile-discovery"),
            "error should name existing plugin, got: {err}"
        );
    }

    #[test]
    fn install_from_path_allows_different_stage_slots() {
        // Two workflow plugins with different stage slots must not conflict.
        let project_dir = tempfile::tempdir().unwrap();
        let plugins_dir = project_dir.path().join("plugins");
        std::fs::create_dir_all(&plugins_dir).unwrap();

        let existing_dir = plugins_dir.join("agile-discovery");
        std::fs::create_dir_all(&existing_dir).unwrap();
        write_plugin_manifest(
            &existing_dir,
            "@orqastudio/plugin-agile-discovery",
            &["workflow"],
            Some("discovery"),
            true,
            "[]",
        );

        let orqa_dir = project_dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let project_json = r#"{
            "name": "test",
            "organisation": false,
            "projects": [],
            "artifacts": [],
            "statuses": [],
            "delivery": {},
            "relationships": [],
            "plugins": {
                "@orqastudio/plugin-agile-discovery": {
                    "path": "plugins/agile-discovery",
                    "installed": true,
                    "enabled": true
                }
            }
        }"#;
        std::fs::write(orqa_dir.join("project.json"), project_json).unwrap();

        // Install a plugin for a different stage slot.
        let planning_dir = tempfile::tempdir().unwrap();
        write_plugin_manifest(
            planning_dir.path(),
            "@orqastudio/plugin-agile-planning",
            &["workflow"],
            Some("planning"),
            true,
            "[]",
        );

        let result = install_from_path(planning_dir.path(), project_dir.path());
        assert!(
            result.is_ok(),
            "different stage slots should not conflict, got: {:?}",
            result.err()
        );
    }
}
