// HTTP client for `/issue-groups` endpoints.
//
// Mirrors IssueGroupRepository from engine/storage/src/traits.rs.
// IssueGroup, SortBy, and SortDir are redefined here with serde derives since
// libs/db does not depend on engine/storage.

use serde::{Deserialize, Serialize};

use crate::{parse_empty_response, parse_response, DbError};

/// A deduplicated group of events sharing the same fingerprint.
///
/// Mirrors `engine/storage/src/repo/issue_groups.rs::IssueGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueGroup {
    /// Stable hash derived from source + level + message_template + stack_top.
    pub fingerprint: String,
    /// Human-readable title derived from the message template.
    pub title: String,
    /// Source component that produced the events (e.g. "daemon", "mcp").
    pub component: String,
    /// Severity level (e.g. "Error", "Warn").
    pub level: String,
    /// Unix millisecond timestamp of the first event with this fingerprint.
    pub first_seen: i64,
    /// Unix millisecond timestamp of the most recent event.
    pub last_seen: i64,
    /// Total number of events matching this fingerprint.
    pub count: i64,
    /// 24 hourly occurrence counters.
    pub sparkline_buckets: Vec<i64>,
    /// Ring buffer of the most recent 50 event IDs.
    pub recent_event_ids: Vec<u64>,
}

/// Sort direction for `list`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    /// Ascending order.
    Asc,
    /// Descending order.
    Desc,
}

/// Column to sort by in `list`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    /// Sort by the `last_seen` timestamp.
    LastSeen,
    /// Sort by occurrence `count`.
    Count,
    /// Sort by `level` text.
    Level,
    /// Sort by `component` text.
    Component,
}

/// Sub-client for `/issue-groups` daemon endpoints.
///
/// Obtained via `DbClient::issue_groups()`.
pub struct IssueGroupsClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl IssueGroupsClient<'_> {
    /// Insert or update an issue group for the given fingerprint.
    ///
    /// Calls POST /issue-groups/upsert.
    pub async fn upsert(
        &self,
        fingerprint: &str,
        title: &str,
        component: &str,
        level: &str,
        timestamp_ms: i64,
        event_id: u64,
    ) -> Result<(), DbError> {
        let body = serde_json::json!({
            "fingerprint": fingerprint,
            "title": title,
            "component": component,
            "level": level,
            "timestamp_ms": timestamp_ms,
            "event_id": event_id,
        });
        let resp = self
            .http
            .post(format!("{}/issue-groups/upsert", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// List issue groups with optional filtering and sorting.
    ///
    /// Calls GET /issue-groups?sort_by=...&sort_dir=...&limit=...&offset=...
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        sort_by: SortBy,
        sort_dir: SortDir,
        filter_component: Option<&str>,
        filter_level: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<IssueGroup>, DbError> {
        let mut req = self
            .http
            .get(format!("{}/issue-groups", self.base_url))
            .query(&[
                ("sort_by", sort_by_str(sort_by)),
                ("sort_dir", sort_dir_str(sort_dir)),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ]);
        if let Some(c) = filter_component {
            req = req.query(&[("component", c)]);
        }
        if let Some(l) = filter_level {
            req = req.query(&[("level", l)]);
        }
        let resp = req.send().await?;
        parse_response(resp).await
    }

    /// Return a single issue group by fingerprint, or `None` if it does not exist.
    ///
    /// Calls GET /issue-groups/:fingerprint.
    pub async fn get(&self, fingerprint: &str) -> Result<Option<IssueGroup>, DbError> {
        let resp = self
            .http
            .get(format!("{}/issue-groups/{fingerprint}", self.base_url))
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        parse_response(resp).await.map(Some)
    }
}

/// Serialize `SortBy` to its wire string.
fn sort_by_str(sort_by: SortBy) -> &'static str {
    match sort_by {
        SortBy::LastSeen => "last_seen",
        SortBy::Count => "count",
        SortBy::Level => "level",
        SortBy::Component => "component",
    }
}

/// Serialize `SortDir` to its wire string.
fn sort_dir_str(sort_dir: SortDir) -> &'static str {
    match sort_dir {
        SortDir::Asc => "asc",
        SortDir::Desc => "desc",
    }
}
