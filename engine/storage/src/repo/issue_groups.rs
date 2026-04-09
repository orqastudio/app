// Issue groups repository for orqa-storage.
//
// Provides upsert, list, and get operations over the `issue_groups` table.
// Each issue group represents a deduplicated cluster of events sharing the
// same fingerprint. The upsert path maintains a 24-bucket hourly sparkline
// and a ring-buffer of the 50 most recent event IDs.
//
// The sparkline rotation and ring-buffer logic is pure Rust — only the
// SELECT/INSERT/UPDATE layer uses SeaORM raw query execution.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use serde::{Deserialize, Serialize};

use crate::error::StorageError;
use crate::traits::IssueGroupRepository;

/// Number of hourly buckets in the sparkline.
const SPARKLINE_BUCKETS: usize = 24;

/// Maximum number of recent event IDs retained per group.
const RECENT_EVENT_LIMIT: usize = 50;

/// Milliseconds per hour, used to compute the bucket index.
const MS_PER_HOUR: i64 = 3_600_000;

/// A deduplicated group of events sharing the same fingerprint.
///
/// The sparkline is 24 hourly counters. The recent_event_ids ring buffer
/// holds the last 50 event IDs that contributed to this group.
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
    /// 24 hourly occurrence counters. Index = `(timestamp_ms / MS_PER_HOUR) % 24`.
    pub sparkline_buckets: Vec<i64>,
    /// Ring buffer of the most recent `RECENT_EVENT_LIMIT` event IDs.
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

impl SortDir {
    /// Return the SQL keyword for this direction.
    fn as_sql(self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
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

impl SortBy {
    /// Return the SQL column name for this sort key.
    fn as_sql(self) -> &'static str {
        match self {
            Self::LastSeen => "last_seen",
            Self::Count => "count",
            Self::Level => "level",
            Self::Component => "component",
        }
    }
}

/// Async repository handle for the `issue_groups` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::issue_groups()`.
pub struct IssueGroupRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Map a SeaORM `QueryResult` row from `issue_groups` to an `IssueGroup`.
fn map_group(row: &sea_orm::QueryResult) -> Result<IssueGroup, StorageError> {
    let sparkline_json: String = row
        .try_get("", "sparkline_buckets")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let recent_json: String = row
        .try_get("", "recent_event_ids")
        .map_err(|e| StorageError::Database(e.to_string()))?;

    let sparkline_buckets: Vec<i64> =
        serde_json::from_str(&sparkline_json).unwrap_or_else(|_| vec![0; SPARKLINE_BUCKETS]);
    let recent_event_ids: Vec<u64> = serde_json::from_str(&recent_json).unwrap_or_default();

    Ok(IssueGroup {
        fingerprint: row
            .try_get("", "fingerprint")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        title: row
            .try_get("", "title")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        component: row
            .try_get("", "component")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        level: row
            .try_get("", "level")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        first_seen: row
            .try_get("", "first_seen")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        last_seen: row
            .try_get("", "last_seen")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        count: row
            .try_get("", "count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        sparkline_buckets,
        recent_event_ids,
    })
}

/// Compute the 24-bucket hourly index for a Unix millisecond timestamp.
///
/// Bucket index = (timestamp_ms / MS_PER_HOUR) % SPARKLINE_BUCKETS
fn bucket_index(timestamp_ms: i64) -> usize {
    let hour = (timestamp_ms / MS_PER_HOUR).max(0) as u64;
    (hour % SPARKLINE_BUCKETS as u64) as usize
}

/// Zero-fill sparkline buckets for hours that were skipped between
/// `prev_last_seen_ms` and `new_timestamp_ms`.
///
/// When time advances past one or more hour boundaries, those intermediate
/// buckets should show zero events rather than stale counts from an earlier
/// cycle. We only zero-fill up to SPARKLINE_BUCKETS - 1 buckets; if more
/// time has passed than the entire window width, every bucket is zeroed.
fn rotate_sparkline(sparkline: &mut Vec<i64>, prev_ms: i64, new_ms: i64) {
    if sparkline.len() < SPARKLINE_BUCKETS {
        sparkline.resize(SPARKLINE_BUCKETS, 0);
    }

    let prev_hour = (prev_ms / MS_PER_HOUR).max(0) as u64;
    let new_hour = (new_ms / MS_PER_HOUR).max(0) as u64;

    if new_hour <= prev_hour {
        // Same hour or clock went backwards — nothing to zero-fill.
        return;
    }

    // Number of buckets that have elapsed (excluding the current one which the
    // caller will increment). Capped at the full window to avoid over-zeroing.
    let skipped = (new_hour - prev_hour).min(SPARKLINE_BUCKETS as u64) as usize;

    // Zero-fill the buckets for the hours that have elapsed since last_seen.
    for i in 0..skipped {
        let bucket = ((prev_hour + 1 + i as u64) % SPARKLINE_BUCKETS as u64) as usize;
        sparkline[bucket] = 0;
    }
}

/// Insert a brand-new issue group row on first occurrence.
async fn insert_new(
    db: &DatabaseConnection,
    fingerprint: &str,
    title: &str,
    component: &str,
    level: &str,
    timestamp_ms: i64,
    event_id: u64,
) -> Result<(), StorageError> {
    let current_bucket = bucket_index(timestamp_ms);
    let mut sparkline = vec![0i64; SPARKLINE_BUCKETS];
    sparkline[current_bucket] = 1;
    let sparkline_json = serde_json::to_string(&sparkline)?;
    let recent_json = serde_json::to_string(&[event_id])?;

    db.execute_raw(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        "INSERT INTO issue_groups \
         (fingerprint, title, component, level, \
          first_seen, last_seen, count, sparkline_buckets, recent_event_ids) \
         VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?)",
        [
            fingerprint.into(),
            title.into(),
            component.into(),
            level.into(),
            timestamp_ms.into(),
            timestamp_ms.into(),
            sparkline_json.into(),
            recent_json.into(),
        ],
    ))
    .await
    .map_err(|e| StorageError::Database(e.to_string()))?;

    Ok(())
}

/// Increment an existing issue group row: count, last_seen, sparkline, ring buffer.
async fn update_existing(
    db: &DatabaseConnection,
    fingerprint: &str,
    prev_last_seen: i64,
    sparkline_json: &str,
    recent_json: &str,
    timestamp_ms: i64,
    event_id: u64,
) -> Result<(), StorageError> {
    let current_bucket = bucket_index(timestamp_ms);
    let mut sparkline: Vec<i64> =
        serde_json::from_str(sparkline_json).unwrap_or_else(|_| vec![0; SPARKLINE_BUCKETS]);
    let mut recent: Vec<u64> = serde_json::from_str(recent_json).unwrap_or_default();

    // Zero-fill hours that were skipped between last_seen and now.
    rotate_sparkline(&mut sparkline, prev_last_seen, timestamp_ms);
    sparkline[current_bucket] = sparkline[current_bucket].saturating_add(1);

    // Append to ring buffer, dropping oldest entries when over the cap.
    recent.push(event_id);
    if recent.len() > RECENT_EVENT_LIMIT {
        let overflow = recent.len() - RECENT_EVENT_LIMIT;
        recent.drain(..overflow);
    }

    let updated_sparkline = serde_json::to_string(&sparkline)?;
    let updated_recent = serde_json::to_string(&recent)?;

    db.execute_raw(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        "UPDATE issue_groups \
         SET count = count + 1, \
             last_seen = ?, \
             sparkline_buckets = ?, \
             recent_event_ids = ? \
         WHERE fingerprint = ?",
        [
            timestamp_ms.into(),
            updated_sparkline.into(),
            updated_recent.into(),
            fingerprint.into(),
        ],
    ))
    .await
    .map_err(|e| StorageError::Database(e.to_string()))?;

    Ok(())
}

#[async_trait::async_trait]
impl IssueGroupRepository for IssueGroupRepo {
    /// Insert or update an issue group for the given fingerprint.
    ///
    /// On first occurrence: inserts a new row with count=1, first_seen=last_seen=timestamp,
    /// a zeroed sparkline with the current hour bucket set to 1, and recent_event_ids=[event_id].
    ///
    /// On subsequent occurrences: increments count, updates last_seen, rotates the
    /// sparkline (zero-fills skipped hours), and appends event_id to the ring buffer
    /// (capped at RECENT_EVENT_LIMIT, oldest dropped).
    async fn upsert(
        &self,
        fingerprint: &str,
        title: &str,
        component: &str,
        level: &str,
        timestamp_ms: i64,
        event_id: u64,
    ) -> Result<(), StorageError> {
        // Read existing row to determine whether to insert or update.
        let existing = self
            .db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT last_seen, sparkline_buckets, recent_event_ids \
                 FROM issue_groups WHERE fingerprint = ?",
                [fingerprint.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        match existing {
            None => {
                insert_new(
                    &self.db,
                    fingerprint,
                    title,
                    component,
                    level,
                    timestamp_ms,
                    event_id,
                )
                .await
            }
            Some(row) => {
                let prev_last_seen: i64 = row
                    .try_get("", "last_seen")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                let sparkline_json: String = row
                    .try_get("", "sparkline_buckets")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                let recent_json: String = row
                    .try_get("", "recent_event_ids")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                update_existing(
                    &self.db,
                    fingerprint,
                    prev_last_seen,
                    &sparkline_json,
                    &recent_json,
                    timestamp_ms,
                    event_id,
                )
                .await
            }
        }
    }

    /// List issue groups with optional filtering and sorting.
    ///
    /// Builds a dynamic SQL query. All filter parameters are optional.
    /// Results are always paged: `limit` defaults to 100, `offset` defaults to 0.
    async fn list(
        &self,
        sort_by: SortBy,
        sort_dir: SortDir,
        filter_component: Option<&str>,
        filter_level: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<IssueGroup>, StorageError> {
        // Build WHERE clause and bind values from optional filter fields.
        let mut conditions: Vec<String> = Vec::new();
        let mut values: Vec<sea_orm::Value> = Vec::new();

        if let Some(comp) = filter_component {
            conditions.push("component = ?".to_owned());
            values.push(comp.into());
        }
        if let Some(lvl) = filter_level {
            conditions.push("level = ?".to_owned());
            values.push(lvl.into());
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT fingerprint, title, component, level, first_seen, last_seen, \
                    count, sparkline_buckets, recent_event_ids \
             FROM issue_groups \
             {where_clause} \
             ORDER BY {sort_col} {sort_dir} \
             LIMIT {limit} OFFSET {offset}",
            sort_col = sort_by.as_sql(),
            sort_dir = sort_dir.as_sql(),
        );

        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &sql,
                values,
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter().map(map_group).collect()
    }

    /// Return a single issue group by fingerprint, or `None` if it does not exist.
    async fn get(&self, fingerprint: &str) -> Result<Option<IssueGroup>, StorageError> {
        let row = self
            .db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT fingerprint, title, component, level, first_seen, last_seen, \
                        count, sparkline_buckets, recent_event_ids \
                 FROM issue_groups WHERE fingerprint = ?",
                [fingerprint.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(r) => map_group(&r).map(Some),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::IssueGroupRepository;
    use crate::Storage;

    async fn open() -> Storage {
        Storage::open_in_memory().await.expect("in-memory storage")
    }

    /// Compute the millisecond timestamp for a given hour offset from a base.
    fn at_hour(base_ms: i64, hour_offset: i64) -> i64 {
        base_ms + hour_offset * MS_PER_HOUR
    }

    const BASE_MS: i64 = 1_700_000_000_000; // arbitrary fixed point in time

    #[tokio::test]
    async fn insert_new_group_count_one_first_eq_last() {
        let storage = open().await;
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", BASE_MS, 1)
            .await
            .expect("upsert");

        let group = storage
            .issue_groups()
            .get("fp1")
            .await
            .expect("get")
            .expect("exists");
        assert_eq!(group.count, 1);
        assert_eq!(group.first_seen, BASE_MS);
        assert_eq!(group.last_seen, BASE_MS);
        assert_eq!(group.recent_event_ids, vec![1u64]);
    }

    #[tokio::test]
    async fn insert_new_group_sparkline_has_one_in_current_bucket() {
        let storage = open().await;
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", BASE_MS, 1)
            .await
            .expect("upsert");

        let group = storage
            .issue_groups()
            .get("fp1")
            .await
            .expect("get")
            .expect("exists");
        assert_eq!(group.sparkline_buckets.len(), SPARKLINE_BUCKETS);
        let bucket = bucket_index(BASE_MS);
        assert_eq!(group.sparkline_buckets[bucket], 1);
        let total: i64 = group.sparkline_buckets.iter().sum();
        assert_eq!(total, 1);
    }

    #[tokio::test]
    async fn upsert_same_fingerprint_increments_count_updates_last_seen() {
        let storage = open().await;
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", BASE_MS, 1)
            .await
            .expect("first");
        let ts2 = at_hour(BASE_MS, 0);
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", ts2 + 1000, 2)
            .await
            .expect("second");

        let group = storage
            .issue_groups()
            .get("fp1")
            .await
            .expect("get")
            .expect("exists");
        assert_eq!(group.count, 2);
        assert_eq!(group.first_seen, BASE_MS, "first_seen must not change");
        assert_eq!(group.last_seen, ts2 + 1000);
    }

    #[tokio::test]
    async fn ring_buffer_caps_at_fifty_drops_oldest() {
        let storage = open().await;
        for id in 1u64..=60 {
            let ts = BASE_MS + id as i64 * 1000;
            storage
                .issue_groups()
                .upsert("fp_ring", "Title", "daemon", "Error", ts, id)
                .await
                .expect("upsert");
        }

        let group = storage
            .issue_groups()
            .get("fp_ring")
            .await
            .expect("get")
            .expect("exists");
        assert_eq!(group.recent_event_ids.len(), RECENT_EVENT_LIMIT);
        assert_eq!(*group.recent_event_ids.first().unwrap(), 11u64);
        assert_eq!(*group.recent_event_ids.last().unwrap(), 60u64);
    }

    #[tokio::test]
    async fn sparkline_same_hour_increments_same_bucket() {
        let storage = open().await;
        for id in 1u64..=3 {
            storage
                .issue_groups()
                .upsert("fp_spark", "T", "d", "E", BASE_MS + id as i64 - 1, id)
                .await
                .expect("upsert");
        }

        let group = storage
            .issue_groups()
            .get("fp_spark")
            .await
            .expect("get")
            .expect("exists");
        let bucket = bucket_index(BASE_MS);
        assert_eq!(group.sparkline_buckets[bucket], 3);
        let total: i64 = group.sparkline_buckets.iter().sum();
        assert_eq!(total, 3);
    }

    #[tokio::test]
    async fn sparkline_rotation_zeros_skipped_hours() {
        let storage = open().await;
        storage
            .issue_groups()
            .upsert("fp_rot", "T", "d", "E", BASE_MS, 1)
            .await
            .expect("1");

        let ts2 = at_hour(BASE_MS, 3);
        storage
            .issue_groups()
            .upsert("fp_rot", "T", "d", "E", ts2, 2)
            .await
            .expect("2");

        let group = storage
            .issue_groups()
            .get("fp_rot")
            .await
            .expect("get")
            .expect("exists");
        let bucket_h = bucket_index(BASE_MS);
        let bucket_h3 = bucket_index(ts2);

        assert_eq!(group.sparkline_buckets[bucket_h], 1, "first bucket still 1");
        assert_eq!(group.sparkline_buckets[bucket_h3], 1, "new bucket = 1");

        let bucket_h1 = (bucket_h + 1) % SPARKLINE_BUCKETS;
        let bucket_h2 = (bucket_h + 2) % SPARKLINE_BUCKETS;
        assert_eq!(group.sparkline_buckets[bucket_h1], 0);
        assert_eq!(group.sparkline_buckets[bucket_h2], 0);
    }

    #[tokio::test]
    async fn list_sort_by_count_desc_returns_highest_first() {
        let storage = open().await;
        storage
            .issue_groups()
            .upsert("fp_a", "A", "daemon", "Info", BASE_MS, 1)
            .await
            .expect("a");
        for id in 2u64..=4 {
            storage
                .issue_groups()
                .upsert("fp_b", "B", "daemon", "Info", BASE_MS + id as i64, id)
                .await
                .expect("b");
        }
        for id in 5u64..=6 {
            storage
                .issue_groups()
                .upsert("fp_c", "C", "daemon", "Info", BASE_MS + id as i64, id)
                .await
                .expect("c");
        }

        let groups = storage
            .issue_groups()
            .list(SortBy::Count, SortDir::Desc, None, None, 10, 0)
            .await
            .expect("list");

        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].fingerprint, "fp_b", "highest count first");
        assert_eq!(groups[0].count, 3);
        assert!(groups[0].count >= groups[1].count);
        assert!(groups[1].count >= groups[2].count);
    }

    #[tokio::test]
    async fn list_filter_component_returns_only_matching() {
        let storage = open().await;
        storage
            .issue_groups()
            .upsert("fp1", "T", "daemon", "Error", BASE_MS, 1)
            .await
            .expect("1");
        storage
            .issue_groups()
            .upsert("fp2", "T", "mcp", "Error", BASE_MS, 2)
            .await
            .expect("2");
        storage
            .issue_groups()
            .upsert("fp3", "T", "daemon", "Warn", BASE_MS, 3)
            .await
            .expect("3");

        let groups = storage
            .issue_groups()
            .list(SortBy::LastSeen, SortDir::Desc, Some("daemon"), None, 10, 0)
            .await
            .expect("list");

        assert_eq!(groups.len(), 2);
        assert!(groups.iter().all(|g| g.component == "daemon"));
    }

    #[tokio::test]
    async fn get_returns_none_for_missing_fingerprint() {
        let storage = open().await;
        let result = storage
            .issue_groups()
            .get("nonexistent")
            .await
            .expect("get");
        assert!(result.is_none());
    }
}
