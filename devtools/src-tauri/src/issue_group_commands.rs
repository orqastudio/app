// Tauri IPC command wrappers for issue group queries via orqa-storage.
//
// Exposes list and get operations over the `issue_groups` table so the frontend
// can display deduplicated error summaries.

use std::sync::Arc;

use orqa_storage::repo::issue_groups::{IssueGroup, SortBy, SortDir};
use orqa_storage::traits::IssueGroupRepository as _;
use orqa_storage::Storage;
use tauri::State;

/// IPC command — list issue groups with optional filtering and sorting.
///
/// Defaults: sort by `last_seen` descending, `limit` 100, `offset` 0.
/// Accepts optional `sort_by` ("last_seen", "count", "level", "component"),
/// `sort_dir` ("asc", "desc"), `filter_component`, and `filter_level` strings.
#[tauri::command]
pub async fn devtools_list_issue_groups(
    storage: State<'_, Arc<Storage>>,
    sort_by: Option<String>,
    sort_dir: Option<String>,
    filter_component: Option<String>,
    filter_level: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<IssueGroup>, String> {
    let sort_by = parse_sort_by(sort_by.as_deref());
    let sort_dir = parse_sort_dir(sort_dir.as_deref());
    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);
    storage
        .issue_groups()
        .list(
            sort_by,
            sort_dir,
            filter_component.as_deref(),
            filter_level.as_deref(),
            limit,
            offset,
        )
        .await
        .map_err(|e| e.to_string())
}

/// IPC command — get a single issue group by fingerprint.
///
/// Returns `null` in JSON when no group with the given fingerprint exists.
#[tauri::command]
pub async fn devtools_get_issue_group(
    storage: State<'_, Arc<Storage>>,
    fingerprint: String,
) -> Result<Option<IssueGroup>, String> {
    storage
        .issue_groups()
        .get(&fingerprint)
        .await
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

/// Parse a `sort_by` string into a `SortBy` variant, defaulting to `LastSeen`.
fn parse_sort_by(s: Option<&str>) -> SortBy {
    match s {
        Some("count") => SortBy::Count,
        Some("level") => SortBy::Level,
        Some("component") => SortBy::Component,
        _ => SortBy::LastSeen,
    }
}

/// Parse a `sort_dir` string into a `SortDir` variant, defaulting to `Desc`.
fn parse_sort_dir(s: Option<&str>) -> SortDir {
    match s {
        Some("asc") => SortDir::Asc,
        _ => SortDir::Desc,
    }
}
