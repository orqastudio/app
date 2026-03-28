// POST /compact-context daemon endpoint.
//
// Composes a governance context document for pre-compaction preservation. The
// document captures active epics, in-progress tasks, existing session state,
// and recovery instructions. This absorbs the composition logic from the
// connector's save-context.ts hook so the daemon is the authoritative source
// for governance context assembly.
//
// Design: artifact data is read directly from the `.orqa/` directory structure
// using the engine's graph library — no inter-daemon calls are made.

use std::path::Path;

use axum::{http::StatusCode, Json};
use orqa_engine::graph::{build_artifact_graph, ArtifactNode};
use serde::{Deserialize, Serialize};

/// Request body for POST /compact-context.
#[derive(Debug, Deserialize)]
pub struct CompactContextRequest {
    /// Absolute path to the project root (the directory containing `.orqa/`).
    pub project_path: String,
}

/// Response body for POST /compact-context.
#[derive(Debug, Serialize)]
pub struct CompactContextResponse {
    /// The full governance context document text.
    pub context_document: String,
    /// A short human-readable summary of what was preserved.
    pub summary: String,
}

/// A minimal representation of an artifact item used within the composed document.
struct ArtifactItem {
    id: String,
    title: String,
    status: Option<String>,
}

impl ArtifactItem {
    fn from_node(node: &ArtifactNode) -> Self {
        ArtifactItem {
            id: node.id.clone(),
            title: node.title.clone(),
            status: node.status.clone(),
        }
    }
}

/// Query all artifacts of a given type and status from the project's `.orqa/` directory.
///
/// Builds the artifact graph once and filters by artifact_type and status. Both
/// comparisons are case-insensitive to tolerate minor schema variations.
fn query_artifacts(project_path: &Path, artifact_type: &str, status: &str) -> Vec<ArtifactItem> {
    let graph = match build_artifact_graph(project_path) {
        Ok(g) => g,
        Err(e) => {
            tracing::warn!(
                error = %e,
                artifact_type = artifact_type,
                status = status,
                "[compact-context] could not build artifact graph — returning empty list"
            );
            return Vec::new();
        }
    };

    let type_lower = artifact_type.to_lowercase();
    let status_lower = status.to_lowercase();

    let mut items: Vec<ArtifactItem> = graph
        .nodes
        .values()
        .filter(|node| {
            node.artifact_type.to_lowercase() == type_lower
                && node
                    .status
                    .as_deref()
                    .is_some_and(|s| s.to_lowercase() == status_lower)
        })
        .map(ArtifactItem::from_node)
        .collect();

    // Sort by ID for deterministic document output.
    items.sort_by(|a, b| a.id.cmp(&b.id));
    items
}

/// Read the existing session state from `.state/session-state.md`, if present.
///
/// Returns an empty string when the file does not exist — the caller renders
/// the section only when there is content.
fn read_session_state(project_path: &Path) -> String {
    let path = project_path.join(".state").join("session-state.md");
    if !path.exists() {
        return String::new();
    }
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        tracing::warn!(
            error = %e,
            path = %path.display(),
            "[compact-context] could not read session state"
        );
        String::new()
    })
}

/// Compose the full governance context document from epics, tasks, and session state.
///
/// The document structure mirrors what save-context.ts produced so that the
/// connector's thin adapter can consume it unchanged. Recovery instructions
/// reference the plugin-resolved agent role rather than any hardcoded path —
/// this satisfies P1 (Plugin-Composed Everything) and avoids hardcoding
/// `.orqa/process/agents/orchestrator.md`.
fn compose_document(
    epics: &[ArtifactItem],
    tasks: &[ArtifactItem],
    session_state: &str,
) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push("# Governance Context (saved before compaction)".to_string());
    lines.push(String::new());
    lines.push(format!("Saved: {}", chrono_timestamp()));
    lines.push(String::new());

    if !epics.is_empty() {
        lines.push("## Active Epics".to_string());
        lines.push(String::new());
        for e in epics {
            lines.push(format!("- **{}**: {}", e.id, e.title));
        }
        lines.push(String::new());
    }

    if !tasks.is_empty() {
        lines.push("## Active Tasks".to_string());
        lines.push(String::new());
        for t in tasks {
            let status_label = t
                .status
                .as_deref()
                .unwrap_or("active");
            lines.push(format!("- **{}** [{}]: {}", t.id, status_label, t.title));
        }
        lines.push(String::new());
    }

    if !session_state.is_empty() {
        lines.push("## Previous Session State".to_string());
        lines.push(String::new());
        lines.push(session_state.to_string());
    }

    lines.push(String::new());
    lines.push("## Recovery Instructions".to_string());
    lines.push(String::new());
    lines.push("After compaction, re-read:".to_string());
    lines.push("1. The active epic files listed above".to_string());
    lines.push("2. The active task files listed above".to_string());
    lines.push(
        "3. Your role definition as resolved by the plugin system (use the prompt generation \
         pipeline to regenerate your agent context from installed plugins)"
            .to_string(),
    );
    lines.push("4. Any skills referenced by the current tasks".to_string());

    lines.join("\n")
}

/// Build a short summary string for the response and telemetry.
fn compose_summary(epics: &[ArtifactItem], tasks: &[ArtifactItem]) -> String {
    let epic_part = if epics.is_empty() {
        "No active epics".to_string()
    } else {
        format!(
            "Active epics: {}",
            epics
                .iter()
                .map(|e| e.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let task_part = if tasks.is_empty() {
        "No active tasks".to_string()
    } else {
        format!(
            "Active tasks: {}",
            tasks
                .iter()
                .map(|t| format!(
                    "{} [{}]",
                    t.id,
                    t.status.as_deref().unwrap_or("active")
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    format!(
        "GOVERNANCE CONTEXT PRESERVED before compaction:\n{epic_part}\n{task_part}\n\n\
         Full context in response — write to .state/governance-context.md before compaction."
    )
}

/// Return the current UTC timestamp as an ISO-8601 string.
///
/// Uses `std::time::SystemTime` to avoid pulling in a full datetime crate.
fn chrono_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Manual ISO-8601 formatting from epoch seconds (UTC, second precision).
    // Avoids the `chrono` dependency; the daemon Cargo.toml does not include it.
    let s = secs;
    let sec = s % 60;
    let min = (s / 60) % 60;
    let hour = (s / 3600) % 24;
    let days = s / 86400;

    // Days since Unix epoch → Gregorian date.
    let (year, month, day) = days_to_date(days);

    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}Z")
}

/// Convert days-since-Unix-epoch to (year, month, day) using the Gregorian calendar.
///
/// This is a standalone algorithm used only for the context document timestamp.
fn days_to_date(days: u64) -> (u64, u64, u64) {
    // Shift epoch to 1 Mar 0000 (Gregorian proleptic) using the civil calendar algo.
    // Reference: http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Handle POST /compact-context.
///
/// Reads the artifact graph from disk, filters for active epics and in-progress
/// tasks, reads any existing session state, and assembles the governance context
/// document. Returns both the full document and a short summary.
pub async fn compact_context_handler(
    Json(req): Json<CompactContextRequest>,
) -> Result<Json<CompactContextResponse>, (StatusCode, String)> {
    let project_path = Path::new(&req.project_path);

    if !project_path.exists() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("project_path does not exist: {}", req.project_path),
        ));
    }

    let epics = query_artifacts(project_path, "epic", "active");
    let tasks = query_artifacts(project_path, "task", "in-progress");
    let session_state = read_session_state(project_path);

    tracing::info!(
        project_path = %req.project_path,
        active_epics = epics.len(),
        active_tasks = tasks.len(),
        had_session_state = !session_state.is_empty(),
        "[compact-context] composing governance context document"
    );

    let context_document = compose_document(&epics, &tasks, &session_state);
    let summary = compose_summary(&epics, &tasks);

    Ok(Json(CompactContextResponse {
        context_document,
        summary,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Create a minimal artifact markdown file with YAML frontmatter.
    fn write_artifact(dir: &std::path::Path, filename: &str, id: &str, title: &str, artifact_type: &str, status: &str) {
        let content = format!(
            "---\nid: {id}\ntitle: {title}\ntype: {artifact_type}\nstatus: {status}\n---\n\nBody.\n"
        );
        fs::write(dir.join(filename), content).expect("write artifact");
    }

    #[test]
    fn query_artifacts_returns_empty_for_missing_orqa_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let result = query_artifacts(dir.path(), "epic", "active");
        assert!(result.is_empty());
    }

    #[test]
    fn query_artifacts_finds_matching_artifacts() {
        let dir = tempfile::tempdir().expect("tempdir");
        let epics_dir = dir.path().join(".orqa").join("delivery").join("epics");
        fs::create_dir_all(&epics_dir).expect("create epics dir");

        write_artifact(&epics_dir, "EPIC-001.md", "EPIC-001", "Alpha Feature", "epic", "active");
        write_artifact(&epics_dir, "EPIC-002.md", "EPIC-002", "Beta Feature", "epic", "completed");

        let results = query_artifacts(dir.path(), "epic", "active");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "EPIC-001");
        assert_eq!(results[0].title, "Alpha Feature");
    }

    #[test]
    fn query_artifacts_excludes_wrong_status() {
        let dir = tempfile::tempdir().expect("tempdir");
        let tasks_dir = dir.path().join(".orqa").join("delivery").join("tasks");
        fs::create_dir_all(&tasks_dir).expect("create tasks dir");

        write_artifact(&tasks_dir, "TASK-001.md", "TASK-001", "Do work", "task", "pending");
        write_artifact(&tasks_dir, "TASK-002.md", "TASK-002", "In flight", "task", "in-progress");

        let results = query_artifacts(dir.path(), "task", "in-progress");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "TASK-002");
    }

    #[test]
    fn read_session_state_returns_empty_when_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let state = read_session_state(dir.path());
        assert!(state.is_empty());
    }

    #[test]
    fn read_session_state_returns_content_when_present() {
        let dir = tempfile::tempdir().expect("tempdir");
        let state_dir = dir.path().join(".state");
        fs::create_dir_all(&state_dir).expect("create .state dir");
        fs::write(state_dir.join("session-state.md"), "# Session\n\nDoing X.").expect("write state");

        let state = read_session_state(dir.path());
        assert!(state.contains("Doing X"));
    }

    #[test]
    fn compose_document_includes_recovery_instructions_without_hardcoded_path() {
        let epics = vec![ArtifactItem {
            id: "EPIC-001".to_string(),
            title: "My Epic".to_string(),
            status: Some("active".to_string()),
        }];
        let tasks: Vec<ArtifactItem> = Vec::new();
        let doc = compose_document(&epics, &tasks, "");

        assert!(doc.contains("## Recovery Instructions"));
        // Must NOT reference any hardcoded agent file path.
        assert!(!doc.contains(".orqa/process/agents/"));
        assert!(!doc.contains("orchestrator.md"));
        // Must reference dynamic plugin resolution.
        assert!(doc.contains("plugin system"));
    }

    #[test]
    fn compose_document_includes_active_epics_section() {
        let epics = vec![ArtifactItem {
            id: "EPIC-042".to_string(),
            title: "Big Feature".to_string(),
            status: Some("active".to_string()),
        }];
        let tasks: Vec<ArtifactItem> = Vec::new();
        let doc = compose_document(&epics, &tasks, "");

        assert!(doc.contains("## Active Epics"));
        assert!(doc.contains("EPIC-042"));
        assert!(doc.contains("Big Feature"));
    }

    #[test]
    fn compose_document_includes_active_tasks_section() {
        let epics: Vec<ArtifactItem> = Vec::new();
        let tasks = vec![ArtifactItem {
            id: "TASK-007".to_string(),
            title: "Fix the thing".to_string(),
            status: Some("in-progress".to_string()),
        }];
        let doc = compose_document(&epics, &tasks, "");

        assert!(doc.contains("## Active Tasks"));
        assert!(doc.contains("TASK-007"));
        assert!(doc.contains("in-progress"));
    }

    #[test]
    fn compose_document_includes_previous_session_state() {
        let epics: Vec<ArtifactItem> = Vec::new();
        let tasks: Vec<ArtifactItem> = Vec::new();
        let doc = compose_document(&epics, &tasks, "# Last Session\n\nDid X.");

        assert!(doc.contains("## Previous Session State"));
        assert!(doc.contains("Did X."));
    }

    #[test]
    fn compose_document_omits_session_state_when_empty() {
        let epics: Vec<ArtifactItem> = Vec::new();
        let tasks: Vec<ArtifactItem> = Vec::new();
        let doc = compose_document(&epics, &tasks, "");
        assert!(!doc.contains("## Previous Session State"));
    }

    #[test]
    fn compose_summary_describes_no_epics_and_no_tasks() {
        let summary = compose_summary(&[], &[]);
        assert!(summary.contains("No active epics"));
        assert!(summary.contains("No active tasks"));
    }

    #[test]
    fn compose_summary_lists_epic_ids() {
        let epics = vec![
            ArtifactItem { id: "EPIC-001".to_string(), title: "A".to_string(), status: None },
            ArtifactItem { id: "EPIC-002".to_string(), title: "B".to_string(), status: None },
        ];
        let summary = compose_summary(&epics, &[]);
        assert!(summary.contains("EPIC-001"));
        assert!(summary.contains("EPIC-002"));
    }

    #[test]
    fn timestamp_is_reasonable_format() {
        let ts = chrono_timestamp();
        // Must look like ISO-8601 UTC: YYYY-MM-DDTHH:MM:SSZ
        assert!(ts.ends_with('Z'), "timestamp must end with Z: {ts}");
        assert_eq!(ts.len(), 20, "timestamp length unexpected: {ts}");
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
    }
}
