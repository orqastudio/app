// Storage ingestion route for `orqa migrate storage ingest`.
//
// Implements `POST /admin/migrate/storage/ingest`, which is the daemon-side
// counterpart to the CLI `orqa migrate storage ingest` subcommand.
//
// The route:
//   1. Receives an optional `project_root` override in the JSON body.
//   2. Scans `.orqa/` recursively, classifying each `.md` file as:
//        - `user`   — no `source_plugin` frontmatter field AND path does not
//                     match any installed plugin content manifest entry.
//        - `plugin` — has a `source_plugin` frontmatter field OR path matches
//                     an installed plugin content manifest entry.
//        - `unknown`— could not be classified (parse error, ambiguous path).
//   3. Inserts ONLY `user`-classified files into SurrealDB with
//      `source_plugin = NONE`. Plugin-derived files are left to the
//      `orqa install` path (TASK-S2-08).
//   4. Unknown-classified files are recorded in the report but NOT inserted.
//   5. Is idempotent — re-running against a populated SurrealDB is zero writes
//      because `upsert_artifact_node` uses content-hash dedup.
//   6. Records a `migration_id`, timestamps, and classification counts in the
//      daemon log.
//   7. Writes a per-file report to `.state/migrations/<migration_id>.json`.
//
// The route does NOT pause/resume the watcher — that is the CLI's
// responsibility (it calls POST /watcher/pause before and POST /watcher/resume
// after, on both success and error paths).

use std::path::{Path, PathBuf};

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::{info, warn};
use uuid::Uuid;

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Optional request body for POST /admin/migrate/storage/ingest.
///
/// All fields are optional. If `project_root` is absent, the daemon uses the
/// project root it was started with.
#[derive(Debug, Deserialize)]
pub struct IngestRequest {
    /// Override the project root to scan. If absent, uses the daemon's
    /// current project root.
    pub project_root: Option<String>,
}

/// Classification of a single `.orqa/` markdown file.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileClassification {
    /// User/project-authored artifact — inserted into SurrealDB.
    User,
    /// Plugin-derived artifact — skipped (re-ingest via `orqa install`).
    Plugin,
    /// Could not be classified — not inserted; flagged for Bobbi resolution.
    Unknown,
}

/// Per-file outcome entry written to the migration report.
#[derive(Debug, Serialize)]
pub struct FileOutcome {
    /// Relative path from project root (forward slashes).
    pub path: String,
    /// How the file was classified.
    pub classification: FileClassification,
    /// What was done: "inserted", "skipped", "flagged", or "error".
    pub action: &'static str,
    /// Optional reason — populated for "skipped", "flagged", and "error" actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Summary counts in the migration report.
#[derive(Debug, Serialize)]
pub struct MigrationCounts {
    pub scanned: usize,
    pub inserted: usize,
    pub skipped: usize,
    pub flagged: usize,
    pub errors: usize,
}

/// Full migration report written to `.state/migrations/<migration_id>.json`.
#[derive(Debug, Serialize)]
pub struct MigrationReport {
    pub migration_id: String,
    pub started_at: String,
    pub completed_at: String,
    pub project_root: String,
    pub counts: MigrationCounts,
    pub files: Vec<FileOutcome>,
}

/// HTTP response body for the ingest endpoint.
#[derive(Debug, Serialize)]
pub struct IngestResponse {
    pub migration_id: String,
    pub counts: MigrationCounts,
    /// Path to the written report file, relative to project root.
    pub report_path: String,
    /// Files classified as `unknown` — surfaces them for manual resolution.
    pub flagged_files: Vec<String>,
    /// Per-file outcome for every scanned `.md` file.
    pub files: Vec<FileOutcome>,
}

// ---------------------------------------------------------------------------
// Classification helpers
// ---------------------------------------------------------------------------

/// Build a set of content paths that are owned by installed plugins.
///
/// Each `orqa-plugin.json` in `{project_root}/plugins/` may declare a
/// `content` block mapping source directories to target installation paths.
/// If an artifact file's path matches a known install target prefix, it is
/// classified as plugin-derived.
///
/// Returns a Vec of target path prefixes (forward-slash, relative to project root).
fn collect_plugin_content_paths(project_root: &Path) -> Vec<String> {
    let plugins_dir = project_root.join("plugins");
    let mut prefixes: Vec<String> = Vec::new();

    let Ok(walker) = std::fs::read_dir(&plugins_dir) else {
        return prefixes;
    };

    for entry in walker.flatten() {
        // Each entry may be a category directory — recurse one level.
        let category_path = entry.path();
        if !category_path.is_dir() {
            continue;
        }
        let Ok(sub_walker) = std::fs::read_dir(&category_path) else {
            continue;
        };
        for sub_entry in sub_walker.flatten() {
            let plugin_dir = sub_entry.path();
            let manifest_path = plugin_dir.join("orqa-plugin.json");
            if !manifest_path.exists() {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&manifest_path) else {
                continue;
            };
            let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&content) else {
                continue;
            };
            // `content` block maps string keys to objects with a `target` field.
            if let Some(content_block) = manifest.get("content").and_then(|v| v.as_object()) {
                for (_, entry_val) in content_block {
                    if let Some(target) = entry_val.get("target").and_then(|v| v.as_str()) {
                        // Normalise to forward slashes and strip leading dot/slash.
                        let normalised = target
                            .replace('\\', "/")
                            .trim_start_matches("./")
                            .to_owned();
                        prefixes.push(normalised);
                    }
                }
            }
        }
    }

    prefixes
}

/// Parse YAML frontmatter from a markdown file and extract the `source_plugin` field.
///
/// Returns:
/// - `Ok(Some(s))` if `source_plugin` is present and non-empty.
/// - `Ok(None)` if the field is absent or the file has no frontmatter.
/// - `Err(reason)` on UTF-8 or YAML parse failure.
fn extract_source_plugin(bytes: &[u8], path: &Path) -> Result<Option<String>, String> {
    let content = String::from_utf8_lossy(bytes);

    // Locate frontmatter delimiters.
    let stripped = content.trim_start();
    if !stripped.starts_with("---") {
        return Ok(None);
    }
    let after_first = &stripped[3..];
    let end = after_first
        .find("\n---")
        .unwrap_or_else(|| after_first.find("\r\n---").unwrap_or(0));
    if end == 0 {
        return Ok(None);
    }
    let fm_text = &after_first[..end];

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(fm_text)
        .map_err(|e| format!("YAML parse error in {}: {e}", path.display()))?;

    Ok(yaml_value
        .get("source_plugin")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_owned))
}

/// Classify a single file as `User`, `Plugin`, or `Unknown`.
///
/// Classification rules (in priority order):
/// 1. If the file cannot be read → `Unknown`.
/// 2. If YAML parse fails → `Unknown`.
/// 3. If frontmatter contains a non-empty `source_plugin` field → `Plugin`.
/// 4. If the file's relative path begins with any plugin content target prefix → `Plugin`.
/// 5. Otherwise → `User`.
fn classify_file(
    path: &Path,
    project_root: &Path,
    plugin_prefixes: &[String],
) -> (FileClassification, Option<String>) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            return (
                FileClassification::Unknown,
                Some(format!("cannot read file: {e}")),
            );
        }
    };

    match extract_source_plugin(&bytes, path) {
        Err(reason) => (FileClassification::Unknown, Some(reason)),
        Ok(Some(plugin)) => (
            FileClassification::Plugin,
            Some(format!("source_plugin = {plugin}")),
        ),
        Ok(None) => {
            // Check whether the path falls under a known plugin content target.
            let rel = path
                .strip_prefix(project_root)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");

            for prefix in plugin_prefixes {
                if rel.starts_with(prefix.as_str()) {
                    return (
                        FileClassification::Plugin,
                        Some(format!("path matches plugin content target: {prefix}")),
                    );
                }
            }

            (FileClassification::User, None)
        }
    }
}

// ---------------------------------------------------------------------------
// Route handler
// ---------------------------------------------------------------------------

/// Handle POST /admin/migrate/storage/ingest.
///
/// Classifies all `.md` files under `.orqa/`, inserts user-authored artifacts
/// into SurrealDB, skips plugin-derived ones, and flags unknowns. Writes a
/// per-file report to `.state/migrations/<migration_id>.json`. Returns the
/// migration ID, summary counts, report path, and flagged file list.
#[allow(clippy::too_many_lines)]
pub async fn storage_ingest(
    State(graph_state): State<GraphState>,
    body: Option<Json<IngestRequest>>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Resolve the project root: body override or daemon's current root.
    let project_root: PathBuf = body
        .as_ref()
        .and_then(|b| b.project_root.as_deref())
        .map_or_else(
            || {
                graph_state
                    .0
                    .read()
                    .map(|g| g.project_root.clone())
                    .unwrap_or_default()
            },
            PathBuf::from,
        );

    // Require SurrealDB to be available.
    let Some(db) = graph_state.surreal_db() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "SurrealDB is not available — daemon may not have initialized correctly"
            })),
        );
    };

    let migration_id = Uuid::new_v4().to_string();
    let started_at = chrono_now_iso8601();

    info!(
        subsystem = "migrate",
        migration_id = %migration_id,
        project_root = %project_root.display(),
        "[migrate] starting storage ingest"
    );

    // Build plugin content target prefixes for classification.
    let plugin_prefixes = collect_plugin_content_paths(&project_root);

    // Walk `.orqa/` to collect all .md files.
    let orqa_dir = project_root.join(".orqa");
    let md_paths: Vec<PathBuf> = walkdir::WalkDir::new(&orqa_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let p = e.path();
            p.is_file()
                && p.extension().and_then(|x| x.to_str()) == Some("md")
                && !p
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.eq_ignore_ascii_case("README.md"))
        })
        .map(walkdir::DirEntry::into_path)
        .collect();

    let mut outcomes: Vec<FileOutcome> = Vec::new();
    let mut counts = MigrationCounts {
        scanned: 0,
        inserted: 0,
        skipped: 0,
        flagged: 0,
        errors: 0,
    };

    // Pass 1: classify and upsert user artifact nodes (skipping plugin/unknown).
    // Collect (id, bytes, rel_path) for user files so edge pass can use them.
    let mut user_files: Vec<(String, Vec<u8>, String)> = Vec::new();

    for path in &md_paths {
        counts.scanned += 1;

        let rel = path
            .strip_prefix(&project_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        let (classification, reason) = classify_file(path, &project_root, &plugin_prefixes);

        match classification {
            FileClassification::Plugin => {
                info!(
                    subsystem = "migrate",
                    migration_id = %migration_id,
                    path = %rel,
                    "[migrate] SKIP plugin-derived artifact"
                );
                counts.skipped += 1;
                outcomes.push(FileOutcome {
                    path: rel,
                    classification: FileClassification::Plugin,
                    action: "skipped",
                    reason: Some(reason.unwrap_or_else(|| "plugin-derived".to_owned())),
                });
            }
            FileClassification::Unknown => {
                warn!(
                    subsystem = "migrate",
                    migration_id = %migration_id,
                    path = %rel,
                    reason = ?reason,
                    "[migrate] FLAG unknown-classification artifact — manual resolution required"
                );
                counts.flagged += 1;
                outcomes.push(FileOutcome {
                    path: rel,
                    classification: FileClassification::Unknown,
                    action: "flagged",
                    reason: Some(reason.unwrap_or_else(|| "classification failed".to_owned())),
                });
            }
            FileClassification::User => {
                // Read bytes for hashing and upsert.
                let bytes = match std::fs::read(path) {
                    Ok(b) => b,
                    Err(e) => {
                        warn!(
                            subsystem = "migrate",
                            migration_id = %migration_id,
                            path = %rel,
                            error = %e,
                            "[migrate] ERROR reading user artifact"
                        );
                        counts.errors += 1;
                        outcomes.push(FileOutcome {
                            path: rel,
                            classification: FileClassification::User,
                            action: "error",
                            reason: Some(format!("cannot read: {e}")),
                        });
                        continue;
                    }
                };

                let hash = hex::encode(Sha256::digest(&bytes));

                // Idempotency check: if the stored hash matches, this is a no-op.
                let already_present = is_unchanged_by_hash(&db, &rel, &hash).await;
                if already_present {
                    info!(
                        subsystem = "migrate",
                        migration_id = %migration_id,
                        path = %rel,
                        "[migrate] SKIP user artifact — content unchanged (idempotent)"
                    );
                    counts.skipped += 1;
                    outcomes.push(FileOutcome {
                        path: rel.clone(),
                        classification: FileClassification::User,
                        action: "skipped",
                        reason: Some("already present (content hash match)".to_owned()),
                    });
                    // Still collect for edge pass in case edges changed.
                    user_files.push((rel, bytes, hash));
                    continue;
                }

                // Upsert the artifact node.
                match upsert_user_artifact(&db, &bytes, path, &rel, &hash).await {
                    Ok(artifact_id) => {
                        info!(
                            subsystem = "migrate",
                            migration_id = %migration_id,
                            path = %rel,
                            artifact_id = %artifact_id,
                            "[migrate] INSERT user artifact"
                        );
                        counts.inserted += 1;
                        outcomes.push(FileOutcome {
                            path: rel.clone(),
                            classification: FileClassification::User,
                            action: "inserted",
                            reason: None,
                        });
                        user_files.push((rel, bytes, hash));
                    }
                    Err(e) => {
                        warn!(
                            subsystem = "migrate",
                            migration_id = %migration_id,
                            path = %rel,
                            error = %e,
                            "[migrate] ERROR upserting user artifact"
                        );
                        counts.errors += 1;
                        outcomes.push(FileOutcome {
                            path: rel,
                            classification: FileClassification::User,
                            action: "error",
                            reason: Some(format!("upsert failed: {e}")),
                        });
                    }
                }
            }
        }
    }

    let completed_at = chrono_now_iso8601();

    info!(
        subsystem = "migrate",
        migration_id = %migration_id,
        scanned = counts.scanned,
        inserted = counts.inserted,
        skipped = counts.skipped,
        flagged = counts.flagged,
        errors = counts.errors,
        "[migrate] storage ingest complete"
    );

    // Write the migration report to .state/migrations/<migration_id>.json.
    let flagged_files: Vec<String> = outcomes
        .iter()
        .filter(|o| o.action == "flagged")
        .map(|o| o.path.clone())
        .collect();

    let report = MigrationReport {
        migration_id: migration_id.clone(),
        started_at: started_at.clone(),
        completed_at: completed_at.clone(),
        project_root: project_root.to_string_lossy().replace('\\', "/"),
        counts: MigrationCounts {
            scanned: counts.scanned,
            inserted: counts.inserted,
            skipped: counts.skipped,
            flagged: counts.flagged,
            errors: counts.errors,
        },
        files: outcomes,
    };

    let report_rel = format!(".state/migrations/{migration_id}.json");
    let report_path = project_root.join(&report_rel);

    if let Some(parent) = report_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let report_json = serde_json::to_string_pretty(&report).unwrap_or_default();
    if let Err(e) = std::fs::write(&report_path, &report_json) {
        warn!(
            subsystem = "migrate",
            migration_id = %migration_id,
            path = %report_path.display(),
            error = %e,
            "[migrate] failed to write migration report"
        );
    }

    // Destructure report so files can be included in the HTTP response without cloning.
    let MigrationReport {
        counts: report_counts,
        files: report_files,
        ..
    } = report;

    let response = IngestResponse {
        migration_id,
        counts: MigrationCounts {
            scanned: report_counts.scanned,
            inserted: report_counts.inserted,
            skipped: report_counts.skipped,
            flagged: report_counts.flagged,
            errors: report_counts.errors,
        },
        report_path: report_rel,
        flagged_files,
        files: report_files,
    };

    (
        StatusCode::OK,
        Json(serde_json::to_value(response).unwrap_or_default()),
    )
}

// ---------------------------------------------------------------------------
// SurrealDB helpers (thin wrappers to avoid duplicating sync.rs internals)
// ---------------------------------------------------------------------------

/// Check whether the artifact at `rel_path` already has a matching content hash.
///
/// Returns `true` if the database contains a record with the same hash —
/// meaning the content is unchanged and no write is needed.
async fn is_unchanged_by_hash(
    db: &orqa_graph::surreal::GraphDb,
    rel_path: &str,
    hash: &str,
) -> bool {
    let path_esc = rel_path.replace('\\', "\\\\").replace('\'', "\\'");
    let query = format!("SELECT content_hash FROM artifact WHERE path = '{path_esc}' LIMIT 1;");
    db.0.query(&query)
        .await
        .ok()
        .and_then(|mut r| r.take::<Vec<serde_json::Value>>(0).ok())
        .and_then(|rows| rows.into_iter().next())
        .and_then(|row| row.get("content_hash").cloned())
        .and_then(|v| v.as_str().map(str::to_owned))
        .is_some_and(|stored| stored == hash)
}

/// Upsert a user-authored artifact from raw bytes into SurrealDB.
///
/// Delegates to `orqa_graph::sync::sync_file` which handles frontmatter parsing,
/// node upsert, and edge replacement. The `source_plugin = NONE` constraint is
/// already hard-coded in `sync_file`'s `upsert_artifact_node` call.
///
/// Returns the artifact ID on success.
async fn upsert_user_artifact(
    db: &orqa_graph::surreal::GraphDb,
    _bytes: &[u8],
    path: &Path,
    _rel_path: &str,
    _hash: &str,
) -> Result<String, String> {
    // Re-use `sync_file` from the graph crate so we don't duplicate the
    // upsert logic. `sync_file` reads the file from disk internally.
    let project_root = path
        .ancestors()
        .find(|p| p.join(".orqa").is_dir())
        .unwrap_or_else(|| path.parent().unwrap_or(path));

    match orqa_graph::sync::sync_file(db, path, project_root).await {
        Ok(orqa_graph::sync::SyncResult::Upserted { id, .. }) => Ok(id),
        Ok(orqa_graph::sync::SyncResult::Unchanged) => {
            // Hash matched — treat as success with a placeholder ID.
            // (is_unchanged_by_hash would have caught this earlier but
            // a race could land here; it's not an error.)
            Ok("(unchanged)".to_owned())
        }
        Ok(orqa_graph::sync::SyncResult::Skipped { reason }) => {
            Err(format!("skipped by sync_file: {reason}"))
        }
        Err(e) => Err(format!("{e}")),
    }
}

/// Return the current time as an ISO 8601 string.
///
/// Uses `std::time::SystemTime` to avoid pulling in chrono. The format is
/// millisecond-precision UTC: `2026-04-16T12:34:56.789Z`.
fn chrono_now_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let secs = ms / 1000;
    let millis = ms % 1000;

    // Manual ISO 8601 formatting from Unix timestamp.
    // We compute calendar fields from the Unix epoch without external crates.
    // This avoids adding chrono/time as a daemon dependency.
    let (year, month, day, hour, min, sec) = unix_secs_to_utc(secs as u64);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}.{millis:03}Z")
}

/// Convert Unix seconds since epoch to (year, month, day, hour, min, sec) UTC.
///
/// Implements the algorithm from https://howardhinnant.github.io/date_algorithms.html
/// (civil_from_days). Accurate for the Unix epoch range 1970–9999.
fn unix_secs_to_utc(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let hour = (secs / 3600) % 24;
    let min = (secs / 60) % 60;
    let sec = secs % 60;

    // Days since epoch.
    let z = (secs / 86400) as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = (yoe as i64 + era * 400) as u64;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };

    (year, month, day, hour, min, sec)
}

// ---------------------------------------------------------------------------
// Manifest.json migration handler
// ---------------------------------------------------------------------------

/// Request body for POST /admin/migrate/storage/manifest.
///
/// All fields are optional. If `project_root` is absent, the daemon uses the
/// project root it was started with.
#[derive(Debug, Deserialize)]
pub struct ManifestMigrateRequest {
    /// Override the project root to scan. If absent, uses the daemon's root.
    pub project_root: Option<String>,
}

/// Response body for POST /admin/migrate/storage/manifest.
#[derive(Debug, Serialize)]
pub struct ManifestMigrateResponse {
    /// Unique ID for this migration run.
    pub migration_id: String,
    /// Number of plugin entries ported to SurrealDB.
    pub ported: usize,
    /// Number of plugin entries skipped (already present with matching hash).
    pub skipped: usize,
    /// Number of plugin entries that failed.
    pub errors: usize,
    /// Path where the archived manifest.json was written.
    pub archive_path: String,
}

/// Handle POST /admin/migrate/storage/manifest.
///
/// Reads `.orqa/manifest.json`, ports each plugin entry into the
/// `plugin_installation` SurrealDB table, then archives the file to
/// `.state/archive/orqa-files/<migration_id>/manifest.json`.
///
/// Idempotent: re-running when a record already exists in SurrealDB will
/// call `upsert_plugin_installation` which bumps version and updates the record.
/// After archiving, `.orqa/manifest.json` is removed.
#[allow(clippy::too_many_lines)]
pub async fn manifest_migrate(
    State(graph_state): State<GraphState>,
    body: Option<Json<ManifestMigrateRequest>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let project_root: PathBuf = body
        .as_ref()
        .and_then(|b| b.project_root.as_deref())
        .map_or_else(
            || {
                graph_state
                    .0
                    .read()
                    .map(|g| g.project_root.clone())
                    .unwrap_or_default()
            },
            PathBuf::from,
        );

    let Some(db) = graph_state.surreal_db() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "SurrealDB is not available — daemon may not have initialized correctly"
            })),
        );
    };

    let migration_id = Uuid::new_v4().to_string();
    let manifest_path = project_root.join(".orqa").join("manifest.json");

    if !manifest_path.exists() {
        return (
            StatusCode::OK,
            Json(
                serde_json::to_value(ManifestMigrateResponse {
                    migration_id,
                    ported: 0,
                    skipped: 0,
                    errors: 0,
                    archive_path: String::new(),
                })
                .unwrap_or_default(),
            ),
        );
    }

    let manifest_bytes = match std::fs::read(&manifest_path) {
        Ok(b) => b,
        Err(e) => {
            warn!(
                subsystem = "migrate",
                migration_id = %migration_id,
                error = %e,
                "[migrate] cannot read manifest.json"
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("cannot read manifest.json: {e}") })),
            );
        }
    };

    let manifest_json: serde_json::Value = match serde_json::from_slice(&manifest_bytes) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": format!("cannot parse manifest.json: {e}") })),
            );
        }
    };

    let plugins = match manifest_json.get("plugins").and_then(|v| v.as_object()) {
        Some(p) => p.clone(),
        None => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": "manifest.json has no 'plugins' object" })),
            );
        }
    };

    let (mut ported, mut skipped, mut errors) = (0usize, 0usize, 0usize);

    for (plugin_name, entry) in &plugins {
        let manifest_version = entry
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_owned();

        let manifest_hash = entry
            .get("manifestHash")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();

        let installed_at = entry
            .get("installed_at")
            .and_then(|v| v.as_str())
            .map(str::to_owned);

        // Build PluginFileEntry list from the legacy files map.
        let files = build_file_entries_from_manifest(entry);

        // Check if already present with the same hash — skip to avoid unnecessary bumps.
        let already_present =
            is_plugin_installation_current(&db, plugin_name, &manifest_hash).await;
        if already_present {
            info!(
                subsystem = "migrate",
                migration_id = %migration_id,
                plugin = %plugin_name,
                "[migrate] SKIP manifest entry — already present with matching hash"
            );
            skipped += 1;
            continue;
        }

        match orqa_graph::upsert_plugin_installation_with_timestamp(
            &db,
            plugin_name,
            &manifest_version,
            &manifest_hash,
            &files,
            installed_at.as_deref(),
            orqa_graph::PluginInstallStatus::Installed,
        )
        .await
        {
            Ok(()) => {
                info!(
                    subsystem = "migrate",
                    migration_id = %migration_id,
                    plugin = %plugin_name,
                    "[migrate] PORT manifest entry to SurrealDB"
                );
                ported += 1;
            }
            Err(e) => {
                warn!(
                    subsystem = "migrate",
                    migration_id = %migration_id,
                    plugin = %plugin_name,
                    error = %e,
                    "[migrate] ERROR porting manifest entry"
                );
                errors += 1;
            }
        }
    }

    // Archive the manifest.json file before removing it.
    let archive_dir = project_root
        .join(".state")
        .join("archive")
        .join("orqa-files")
        .join(&migration_id);
    let _ = std::fs::create_dir_all(&archive_dir);
    let archive_path = archive_dir.join("manifest.json");
    let archive_rel = format!(".state/archive/orqa-files/{migration_id}/manifest.json");

    if let Err(e) = std::fs::copy(&manifest_path, &archive_path) {
        warn!(
            subsystem = "migrate",
            migration_id = %migration_id,
            error = %e,
            "[migrate] failed to archive manifest.json — not removing original"
        );
        return (
            StatusCode::OK,
            Json(
                serde_json::to_value(ManifestMigrateResponse {
                    migration_id,
                    ported,
                    skipped,
                    errors,
                    archive_path: String::new(),
                })
                .unwrap_or_default(),
            ),
        );
    }

    // Remove the original only after a successful archive copy.
    if let Err(e) = std::fs::remove_file(&manifest_path) {
        warn!(
            subsystem = "migrate",
            migration_id = %migration_id,
            error = %e,
            "[migrate] failed to remove manifest.json after archiving"
        );
    }

    info!(
        subsystem = "migrate",
        migration_id = %migration_id,
        ported,
        skipped,
        errors,
        archive = %archive_rel,
        "[migrate] manifest.json migration complete"
    );

    (
        StatusCode::OK,
        Json(
            serde_json::to_value(ManifestMigrateResponse {
                migration_id,
                ported,
                skipped,
                errors,
                archive_path: archive_rel,
            })
            .unwrap_or_default(),
        ),
    )
}

/// Convert a legacy manifest.json plugin entry's `files` map into `PluginFileEntry` list.
///
/// The legacy shape is: `files: { "path": { "sourceHash": "...", "installedHash": "..." } }`.
fn build_file_entries_from_manifest(entry: &serde_json::Value) -> Vec<orqa_graph::PluginFileEntry> {
    let Some(files_map) = entry.get("files").and_then(|v| v.as_object()) else {
        return Vec::new();
    };

    files_map
        .iter()
        .map(|(path, hashes)| {
            let source_hash = hashes
                .get("sourceHash")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let installed_hash = hashes
                .get("installedHash")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            orqa_graph::PluginFileEntry {
                path: path.clone(),
                source_hash,
                installed_hash,
                target: String::new(),
                artifact_id: None,
            }
        })
        .collect()
}

/// Check if a plugin_installation record already has the given manifest hash.
///
/// Returns true when the stored record's manifest_hash matches — indicating no
/// re-migration is needed.
async fn is_plugin_installation_current(
    db: &orqa_graph::GraphDb,
    plugin_name: &str,
    manifest_hash: &str,
) -> bool {
    match orqa_graph::read_plugin_installation(db, plugin_name).await {
        Ok(Some(record)) => record
            .get("manifest_hash")
            .and_then(|v| v.as_str())
            .is_some_and(|h| h == manifest_hash),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_user_file_returns_user() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("EPIC-001.md");
        std::fs::write(
            &path,
            "---\nid: EPIC-001\ntype: epic\ntitle: Test\nstatus: active\n---\nBody.\n",
        )
        .unwrap();
        let (cls, reason) = classify_file(&path, dir.path(), &[]);
        assert_eq!(cls, FileClassification::User);
        assert!(reason.is_none());
    }

    #[test]
    fn classify_plugin_file_by_frontmatter() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("KNOW-001.md");
        std::fs::write(
            &path,
            "---\nid: KNOW-001\ntype: knowledge\ntitle: K\nsource_plugin: '@orqastudio/plugin-agile'\n---\nBody.\n",
        )
        .unwrap();
        let (cls, reason) = classify_file(&path, dir.path(), &[]);
        assert_eq!(cls, FileClassification::Plugin);
        assert!(reason.is_some());
    }

    #[test]
    fn classify_plugin_file_by_path_prefix() {
        let dir = tempfile::tempdir().unwrap();
        let target_dir = dir.path().join(".orqa/documentation/knowledge");
        std::fs::create_dir_all(&target_dir).unwrap();
        let path = target_dir.join("KNOW-001.md");
        std::fs::write(
            &path,
            "---\nid: KNOW-001\ntype: knowledge\ntitle: K\n---\nBody.\n",
        )
        .unwrap();
        let prefixes = vec![".orqa/documentation/knowledge".to_owned()];
        let (cls, _) = classify_file(&path, dir.path(), &prefixes);
        assert_eq!(cls, FileClassification::Plugin);
    }

    #[test]
    fn classify_unreadable_file_returns_unknown() {
        let path = PathBuf::from("/nonexistent/path/EPIC-999.md");
        let (cls, reason) = classify_file(&path, Path::new("/nonexistent"), &[]);
        assert_eq!(cls, FileClassification::Unknown);
        assert!(reason.is_some());
    }

    #[test]
    fn iso8601_format_is_well_formed() {
        let ts = chrono_now_iso8601();
        // Should be exactly 24 chars: 2026-04-16T12:34:56.789Z
        assert_eq!(ts.len(), 24, "ISO 8601 string must be 24 chars: {ts}");
        assert!(ts.ends_with('Z'), "must end with Z");
        assert!(ts.contains('T'), "must contain T separator");
    }
}
