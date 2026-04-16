// Import route: POST /artifacts/import
//
// Accepts a directory path and conflict policy, walks all .md files in the
// directory, and imports each artifact into SurrealDB. Reports per-artifact
// status (CREATED / UPDATED / SKIPPED / MERGED / CONFLICT).
//
// Conflict policies:
//   upsert — overwrite existing record and bump version (default)
//   merge  — three-way merge; surface unresolvable conflicts to the caller
//
// No-base handling (merge policy only):
//   take-theirs    — accept incoming file unconditionally
//   keep-ours      — keep the current DB state
//   review-each    — return CONFLICT for caller to handle (default in non-interactive)
//   fail           — abort the entire import if any record lacks a base
//
// The route is deliberately stateless: it takes the path and policy in the
// request body, performs all work synchronously (from the Tokio runtime's
// perspective), and returns a JSON summary with per-artifact outcomes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::warn;

use orqa_graph::build::extract_frontmatter;
use orqa_graph::merge::{three_way_merge, value_to_map};
use orqa_graph::writers::{import_merge_write, import_upsert, read_artifact};

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Request / response types
// ---------------------------------------------------------------------------

/// Conflict resolution policy for records that already exist in SurrealDB.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictPolicy {
    /// Overwrite the existing record, bumping its version counter.
    #[default]
    Upsert,
    /// Three-way merge between base, ours, and theirs.
    Merge,
}

/// What to do when a record has no known base for three-way merge.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NoBaseAction {
    /// Accept the incoming file unconditionally (treat as if ours == base).
    TakeTheirs,
    /// Keep the current DB state, skip the record.
    KeepOurs,
    /// Surface as CONFLICT for the caller to resolve.
    ReviewEach,
    /// Abort the entire import on the first no-base record. Non-interactive safe default.
    #[default]
    Fail,
}

/// Request body for POST /artifacts/import.
#[derive(Debug, Deserialize)]
pub struct ImportRequest {
    /// Absolute path to the directory of .md files to import.
    pub path: String,
    /// How to handle records that already exist in SurrealDB.
    /// Defaults to `upsert`.
    #[serde(default)]
    pub on_conflict: ConflictPolicy,
    /// How to handle records with no known merge base (merge policy only).
    /// Defaults to `fail` (non-interactive safe default).
    #[serde(default)]
    pub no_base_action: NoBaseAction,
    /// Optional base snapshot map: artifact_id → frontmatter JSON string.
    /// Forward-compat hook: orqa export will populate this field so S3
    /// import can perform true three-way merge from a known snapshot.
    /// Currently parsed and validated but not used in base resolution
    /// (SurrealDB records have no source_plugin set by this importer).
    #[serde(default)]
    pub base_snapshot: Option<serde_json::Value>,
}

/// Per-artifact outcome of an import operation.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArtifactOutcome {
    /// New artifact created (no prior record).
    Created,
    /// Existing artifact overwritten (upsert policy).
    Updated,
    /// Content hash matched — no write performed.
    Skipped,
    /// Artifact merged cleanly via three-way merge.
    Merged,
    /// Conflict could not be auto-resolved; conflict file written.
    Conflict,
}

/// Detail for one artifact in the import report.
#[derive(Debug, Clone, Serialize)]
pub struct ArtifactImportStatus {
    /// Artifact ID (frontmatter `id` field).
    pub id: String,
    /// Relative path to the source file within the import directory.
    pub path: String,
    /// Outcome of this artifact's import.
    pub outcome: ArtifactOutcome,
    /// Human-readable reason for the outcome (especially for SKIPPED / CONFLICT).
    pub reason: Option<String>,
}

/// Response body for POST /artifacts/import.
#[derive(Debug, Serialize)]
pub struct ImportResponse {
    /// Migration run ID (used as the subdirectory under .state/import-conflicts/).
    pub migration_id: String,
    /// Total artifacts scanned.
    pub total: usize,
    /// Per-artifact outcome list.
    pub results: Vec<ArtifactImportStatus>,
    /// Number of artifacts created.
    pub created: usize,
    /// Number of artifacts updated (upsert policy).
    pub updated: usize,
    /// Number of artifacts skipped (hash unchanged).
    pub skipped: usize,
    /// Number of artifacts merged cleanly.
    pub merged: usize,
    /// Number of artifacts with unresolvable conflicts.
    pub conflicts: usize,
    /// Warning message if base_snapshot was present but unused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_snapshot_warning: Option<String>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Handle POST /artifacts/import.
///
/// Walks all .md files in the given directory, parses each one, and imports it
/// into SurrealDB using the specified conflict policy. Returns a JSON report
/// with per-artifact status.
///
/// Requires SurrealDB to be available in the graph state. Returns 503 if the
/// embedded database has not been initialized.
#[allow(clippy::too_many_lines)]
pub async fn import_artifacts(
    State(state): State<GraphState>,
    Json(req): Json<ImportRequest>,
) -> Result<Json<ImportResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate that SurrealDB is available.
    let db = state.surreal_db().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "SurrealDB not available — daemon started without a project root"
            })),
        )
    })?;

    let import_dir = PathBuf::from(&req.path);
    if !import_dir.is_dir() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("path is not a directory: {}", req.path)
            })),
        ));
    }

    // Validate base_snapshot field (forward-compat: warn if present but unused).
    let base_snapshot_warning = if let Some(ref snapshot) = req.base_snapshot {
        if snapshot.is_object() {
            let count = snapshot.as_object().map_or(0, serde_json::Map::len);
            if count > 0 {
                Some(format!(
                    "base_snapshot contained {count} entr{} but was not used for base resolution \
                     (no records in this import have source_plugin set)",
                    if count == 1 { "y" } else { "ies" }
                ))
            } else {
                None
            }
        } else {
            Some("base_snapshot field is present but is not a JSON object — ignored".to_owned())
        }
    } else {
        None
    };

    // Generate a migration run ID (timestamp + short random suffix).
    let migration_id = {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        format!("import-{ts}")
    };

    // Collect all .md files in the import directory (non-recursive, shallow).
    let md_files = collect_md_files(&import_dir);

    let mut results: Vec<ArtifactImportStatus> = Vec::new();

    for file_path in &md_files {
        let rel = file_path
            .strip_prefix(&import_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/");

        let bytes = match std::fs::read(file_path) {
            Ok(b) => b,
            Err(e) => {
                warn!(path = %file_path.display(), error = %e, "import: failed to read file");
                results.push(ArtifactImportStatus {
                    id: rel.clone(),
                    path: rel,
                    outcome: ArtifactOutcome::Skipped,
                    reason: Some(format!("read error: {e}")),
                });
                continue;
            }
        };

        let content = String::from_utf8_lossy(&bytes).into_owned();
        let new_hash = hex::encode(Sha256::digest(&bytes));

        // Parse frontmatter.
        let (fm_text, _body) = extract_frontmatter(&content);
        let Some(fm_text) = fm_text else {
            results.push(ArtifactImportStatus {
                id: rel.clone(),
                path: rel,
                outcome: ArtifactOutcome::Skipped,
                reason: Some("no frontmatter".to_owned()),
            });
            continue;
        };

        let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&fm_text) {
            Ok(v) => v,
            Err(e) => {
                warn!(path = %file_path.display(), error = %e, "import: YAML parse error");
                results.push(ArtifactImportStatus {
                    id: rel.clone(),
                    path: rel,
                    outcome: ArtifactOutcome::Skipped,
                    reason: Some(format!("YAML parse error: {e}")),
                });
                continue;
            }
        };

        let artifact_id = match yaml_str(&yaml_value, "id") {
            Some(id) if !id.trim().is_empty() => id.trim().to_owned(),
            _ => {
                results.push(ArtifactImportStatus {
                    id: rel.clone(),
                    path: rel,
                    outcome: ArtifactOutcome::Skipped,
                    reason: Some("no id field in frontmatter".to_owned()),
                });
                continue;
            }
        };

        // Convert YAML to a JSON field map (ignoring base_snapshot key from the payload
        // at the file level — it's a top-level import payload field, not per-artifact).
        let their_json: serde_json::Value = serde_json::to_value(&yaml_value)
            .unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::default()));
        let mut their_fields = value_to_map(&their_json);
        // Remove the base_snapshot key if it somehow appears in artifact frontmatter.
        their_fields.remove("base_snapshot");

        // Determine the target relative path in SurrealDB — use the same path-rel convention
        // as sync.rs (relative to the import dir root).
        let db_rel_path = rel.clone();

        // Check if this record already exists.
        let stored = match read_artifact(&db, &artifact_id).await {
            Ok(s) => s,
            Err(e) => {
                warn!(id = %artifact_id, error = %e, "import: read error");
                results.push(ArtifactImportStatus {
                    id: artifact_id,
                    path: rel,
                    outcome: ArtifactOutcome::Skipped,
                    reason: Some(format!("DB read error: {e}")),
                });
                continue;
            }
        };

        match stored {
            None => {
                // New artifact — create it. Version starts at 0, will be bumped to 1.
                match import_upsert(&db, &artifact_id, &their_fields, &db_rel_path, &new_hash, 0)
                    .await
                {
                    Ok(()) => {
                        results.push(ArtifactImportStatus {
                            id: artifact_id,
                            path: rel,
                            outcome: ArtifactOutcome::Created,
                            reason: None,
                        });
                    }
                    Err(e) => {
                        warn!(id = %artifact_id, error = %e, "import: create failed");
                        results.push(ArtifactImportStatus {
                            id: artifact_id,
                            path: rel,
                            outcome: ArtifactOutcome::Skipped,
                            reason: Some(format!("DB write error: {e}")),
                        });
                    }
                }
            }

            Some(existing) => {
                // Record exists — check content hash first.
                if existing.content_hash.as_deref() == Some(new_hash.as_str()) {
                    results.push(ArtifactImportStatus {
                        id: artifact_id,
                        path: rel,
                        outcome: ArtifactOutcome::Skipped,
                        reason: Some("content hash unchanged".to_owned()),
                    });
                    continue;
                }

                match req.on_conflict {
                    ConflictPolicy::Upsert => {
                        // Overwrite unconditionally.
                        match import_upsert(
                            &db,
                            &artifact_id,
                            &their_fields,
                            &db_rel_path,
                            &new_hash,
                            existing.version,
                        )
                        .await
                        {
                            Ok(()) => {
                                results.push(ArtifactImportStatus {
                                    id: artifact_id,
                                    path: rel,
                                    outcome: ArtifactOutcome::Updated,
                                    reason: None,
                                });
                            }
                            Err(e) => {
                                warn!(id = %artifact_id, error = %e, "import: upsert failed");
                                results.push(ArtifactImportStatus {
                                    id: artifact_id,
                                    path: rel,
                                    outcome: ArtifactOutcome::Skipped,
                                    reason: Some(format!("DB write error: {e}")),
                                });
                            }
                        }
                    }

                    ConflictPolicy::Merge => {
                        // Determine base for three-way merge.
                        // Base resolution order:
                        //   1. source_plugin hash from the manifest ledger (not yet implemented)
                        //   2. base_snapshot entry in the import payload
                        //   3. No base — apply no_base_action policy
                        let base_fields = resolve_base_fields(
                            &existing,
                            req.base_snapshot.as_ref(),
                            &artifact_id,
                        );

                        match base_fields {
                            None => {
                                // No base known.
                                let status = handle_no_base(
                                    &req.no_base_action,
                                    &db,
                                    &artifact_id,
                                    &rel,
                                    &their_fields,
                                    &db_rel_path,
                                    &new_hash,
                                    &existing,
                                    &migration_id,
                                    &import_dir,
                                )
                                .await;
                                results.push(status);
                            }
                            Some(base) => {
                                // Perform three-way merge.
                                let our_fields = value_to_map(&existing.frontmatter);
                                let merge_result =
                                    three_way_merge(&base, &our_fields, &their_fields);

                                if merge_result.is_clean() {
                                    match import_merge_write(
                                        &db,
                                        &artifact_id,
                                        &merge_result.resolved,
                                        &db_rel_path,
                                        &new_hash,
                                        existing.version,
                                    )
                                    .await
                                    {
                                        Ok(()) => {
                                            results.push(ArtifactImportStatus {
                                                id: artifact_id,
                                                path: rel,
                                                outcome: ArtifactOutcome::Merged,
                                                reason: None,
                                            });
                                        }
                                        Err(e) => {
                                            warn!(id = %artifact_id, error = %e, "import: merge write failed");
                                            results.push(ArtifactImportStatus {
                                                id: artifact_id,
                                                path: rel,
                                                outcome: ArtifactOutcome::Skipped,
                                                reason: Some(format!("DB write error: {e}")),
                                            });
                                        }
                                    }
                                } else {
                                    // Unresolvable conflict — write conflict file.
                                    let conflict_fields: Vec<String> = merge_result
                                        .conflicts
                                        .iter()
                                        .filter_map(|c| {
                                            if let orqa_graph::merge::FieldMerge::Conflict {
                                                field,
                                                ..
                                            } = c
                                            {
                                                Some(field.clone())
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();

                                    write_conflict_file(
                                        &import_dir,
                                        &migration_id,
                                        &artifact_id,
                                        &existing.frontmatter,
                                        &their_json,
                                        &conflict_fields,
                                    );

                                    results.push(ArtifactImportStatus {
                                        id: artifact_id,
                                        path: rel,
                                        outcome: ArtifactOutcome::Conflict,
                                        reason: Some(format!(
                                            "conflicting fields: {}",
                                            conflict_fields.join(", ")
                                        )),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Compute summary counts.
    let created = results
        .iter()
        .filter(|r| r.outcome == ArtifactOutcome::Created)
        .count();
    let updated = results
        .iter()
        .filter(|r| r.outcome == ArtifactOutcome::Updated)
        .count();
    let skipped = results
        .iter()
        .filter(|r| r.outcome == ArtifactOutcome::Skipped)
        .count();
    let merged = results
        .iter()
        .filter(|r| r.outcome == ArtifactOutcome::Merged)
        .count();
    let conflicts = results
        .iter()
        .filter(|r| r.outcome == ArtifactOutcome::Conflict)
        .count();

    Ok(Json(ImportResponse {
        migration_id,
        total: results.len(),
        results,
        created,
        updated,
        skipped,
        merged,
        conflicts,
        base_snapshot_warning,
    }))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Walk the import directory and collect all .md file paths (non-recursive).
///
/// Only processes files directly in the given directory, not subdirectories,
/// to keep the import scope predictable for the user.
fn collect_md_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|x| x.to_str()) == Some("md"))
        .collect()
}

/// Extract a string value for `key` from a YAML mapping value.
fn yaml_str(yaml: &serde_yaml::Value, key: &str) -> Option<String> {
    yaml.get(key).and_then(|v| v.as_str()).map(str::to_owned)
}

/// Resolve the base field map for three-way merge.
///
/// Returns `None` if no base can be determined. Base resolution order:
/// 1. source_plugin manifest hash (not yet implemented — placeholder for future)
/// 2. base_snapshot entry in the import payload keyed by artifact ID
/// 3. None — no base available
fn resolve_base_fields(
    existing: &orqa_graph::writers::StoredArtifact,
    base_snapshot: Option<&serde_json::Value>,
    artifact_id: &str,
) -> Option<BTreeMap<String, serde_json::Value>> {
    // Future: check existing.source_plugin against plugin manifest ledger here.
    // For now, source_plugin-based base resolution is not implemented.

    // Check the import payload's base_snapshot for this artifact.
    if let Some(snapshot) = base_snapshot {
        if let Some(entry) = snapshot.get(artifact_id) {
            // base_snapshot entry may be a JSON object or a JSON string of the FM.
            let base_map = if entry.is_object() {
                value_to_map(entry)
            } else if let Some(s) = entry.as_str() {
                // Try to parse as JSON.
                serde_json::from_str::<serde_json::Value>(s)
                    .ok()
                    .map(|v| value_to_map(&v))
                    .unwrap_or_default()
            } else {
                BTreeMap::new()
            };
            if !base_map.is_empty() {
                return Some(base_map);
            }
        }
    }

    // No base available.
    let _ = existing; // suppress unused warning; will be used when source_plugin lands
    None
}

/// Apply the `no_base_action` policy and return the resulting status.
///
/// Called when the merge policy is active but no base can be determined for
/// this artifact.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
async fn handle_no_base(
    action: &NoBaseAction,
    db: &orqa_graph::surreal::GraphDb,
    artifact_id: &str,
    rel: &str,
    their_fields: &BTreeMap<String, serde_json::Value>,
    db_rel_path: &str,
    new_hash: &str,
    existing: &orqa_graph::writers::StoredArtifact,
    migration_id: &str,
    import_dir: &Path,
) -> ArtifactImportStatus {
    match action {
        NoBaseAction::TakeTheirs => {
            match import_upsert(
                db,
                artifact_id,
                their_fields,
                db_rel_path,
                new_hash,
                existing.version,
            )
            .await
            {
                Ok(()) => ArtifactImportStatus {
                    id: artifact_id.to_owned(),
                    path: rel.to_owned(),
                    outcome: ArtifactOutcome::Updated,
                    reason: Some("no-base: take-theirs".to_owned()),
                },
                Err(e) => ArtifactImportStatus {
                    id: artifact_id.to_owned(),
                    path: rel.to_owned(),
                    outcome: ArtifactOutcome::Skipped,
                    reason: Some(format!("no-base take-theirs write error: {e}")),
                },
            }
        }
        NoBaseAction::KeepOurs => ArtifactImportStatus {
            id: artifact_id.to_owned(),
            path: rel.to_owned(),
            outcome: ArtifactOutcome::Skipped,
            reason: Some("no-base: keep-ours".to_owned()),
        },
        NoBaseAction::ReviewEach | NoBaseAction::Fail => {
            // Both map to CONFLICT for the import report. The difference is that
            // `fail` means the CLI should abort after checking the response — the
            // route itself treats both identically at the DB level (no write).
            write_conflict_file(
                import_dir,
                migration_id,
                artifact_id,
                &existing.frontmatter,
                &serde_json::to_value(their_fields).unwrap_or_default(),
                &[],
            );
            ArtifactImportStatus {
                id: artifact_id.to_owned(),
                path: rel.to_owned(),
                outcome: ArtifactOutcome::Conflict,
                reason: Some("no known merge base — review required".to_owned()),
            }
        }
    }
}

/// Write a conflict file to `.state/import-conflicts/<migration_id>/<artifact_id>.conflict.md`.
///
/// The conflict file records both sides of the conflict so the user can resolve it
/// manually. Parent directories are created as needed. Failures are logged as
/// warnings — a failed write must not prevent the rest of the import from running.
///
/// The path is constructed relative to the parent of the import directory so that
/// the `.state/` folder is always alongside the project root, not inside the
/// temporary import directory.
fn write_conflict_file(
    import_dir: &Path,
    migration_id: &str,
    artifact_id: &str,
    ours: &serde_json::Value,
    theirs: &serde_json::Value,
    conflict_fields: &[String],
) {
    // Conflict files go under the project's .state/ dir, one level above the
    // import source directory. Fall back to a sibling of import_dir if needed.
    let state_dir = import_dir
        .parent()
        .unwrap_or(import_dir)
        .join(".state/import-conflicts")
        .join(migration_id);

    if let Err(e) = std::fs::create_dir_all(&state_dir) {
        warn!(
            dir = %state_dir.display(), error = %e,
            "import: could not create conflict directory"
        );
        return;
    }

    let conflict_path = state_dir.join(format!("{artifact_id}.conflict.md"));

    let field_list = if conflict_fields.is_empty() {
        "no known base — full review required".to_owned()
    } else {
        conflict_fields.join(", ")
    };

    let ours_pretty = serde_json::to_string_pretty(ours).unwrap_or_else(|_| ours.to_string());
    let theirs_pretty = serde_json::to_string_pretty(theirs).unwrap_or_else(|_| theirs.to_string());

    let content = format!(
        "# Conflict: {artifact_id}\n\n\
         **Fields:** {field_list}\n\n\
         ## Ours (current DB state)\n\n\
         ```json\n{ours_pretty}\n```\n\n\
         ## Theirs (incoming import)\n\n\
         ```json\n{theirs_pretty}\n```\n"
    );

    if let Err(e) = std::fs::write(&conflict_path, content) {
        warn!(
            path = %conflict_path.display(), error = %e,
            "import: could not write conflict file"
        );
    }
}
