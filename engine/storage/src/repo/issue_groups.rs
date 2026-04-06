// Issue groups repository for orqa-storage.
//
// Provides upsert, list, and get operations over the `issue_groups` table.
// Each issue group represents a deduplicated cluster of events sharing the
// same fingerprint. The upsert path maintains a 24-bucket hourly sparkline
// and a ring-buffer of the 50 most recent event IDs.

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::error::StorageError;
use crate::Storage;

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

/// Sort direction for `list_groups`.
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

/// Column to sort by in `list_groups`.
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

/// Zero-cost repository handle for the `issue_groups` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::issue_groups()`.
pub struct IssueGroupRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl IssueGroupRepo<'_> {
    /// Insert or update an issue group for the given fingerprint.
    ///
    /// On first occurrence: inserts a new row with count=1, first_seen=last_seen=timestamp,
    /// a zeroed sparkline with the current hour bucket set to 1, and recent_event_ids=[event_id].
    ///
    /// On subsequent occurrences: increments count, updates last_seen, rotates the
    /// sparkline (zero-fills skipped hours), and appends event_id to the ring buffer
    /// (capped at RECENT_EVENT_LIMIT, oldest dropped).
    pub fn upsert(
        &self,
        fingerprint: &str,
        title: &str,
        component: &str,
        level: &str,
        timestamp_ms: i64,
        event_id: u64,
    ) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;

        // Read the existing row so we can compute the updated sparkline and ring buffer.
        let mut stmt = conn.prepare(
            "SELECT last_seen, sparkline_buckets, recent_event_ids
             FROM issue_groups WHERE fingerprint = ?1",
        )?;
        let existing: Option<(i64, String, String)> = stmt
            .query_row(params![fingerprint], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .optional()
            .map_err(|e| StorageError::Database(e.to_string()))?;
        drop(stmt);

        match existing {
            None => insert_new(
                &conn,
                fingerprint,
                title,
                component,
                level,
                timestamp_ms,
                event_id,
            ),
            Some((prev_last_seen, sparkline_json, recent_json)) => update_existing(
                &conn,
                fingerprint,
                prev_last_seen,
                &sparkline_json,
                &recent_json,
                timestamp_ms,
                event_id,
            ),
        }
    }

    /// List issue groups with optional filtering and sorting.
    ///
    /// Builds a dynamic SQL query. All filter parameters are optional.
    /// Results are always paged: `limit` defaults to 100, `offset` defaults to 0.
    pub fn list(
        &self,
        sort_by: SortBy,
        sort_dir: SortDir,
        filter_component: Option<&str>,
        filter_level: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<IssueGroup>, StorageError> {
        let conn = self.storage.conn()?;

        // Build WHERE clause from optional filter fields.
        let mut conditions: Vec<String> = Vec::new();
        if filter_component.is_some() {
            conditions.push("component = ?1".to_owned());
        }
        if filter_level.is_some() {
            let idx = if filter_component.is_some() { 2 } else { 1 };
            conditions.push(format!("level = ?{idx}"));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT fingerprint, title, component, level, first_seen, last_seen,
                    count, sparkline_buckets, recent_event_ids
             FROM issue_groups
             {where_clause}
             ORDER BY {sort_col} {sort_dir}
             LIMIT {limit} OFFSET {offset}",
            sort_col = sort_by.as_sql(),
            sort_dir = sort_dir.as_sql(),
        );

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| StorageError::Database(e.to_string()))?;

        // Execute with the appropriate number of bound parameters.
        let rows: Vec<IssueGroup> = match (filter_component, filter_level) {
            (Some(comp), Some(lvl)) => stmt
                .query_map(params![comp, lvl], row_to_group)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
            (Some(comp), None) => stmt
                .query_map(params![comp], row_to_group)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
            (None, Some(lvl)) => stmt
                .query_map(params![lvl], row_to_group)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
            (None, None) => stmt
                .query_map([], row_to_group)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
        };

        Ok(rows)
    }

    /// Return a single issue group by fingerprint, or `None` if it does not exist.
    pub fn get(&self, fingerprint: &str) -> Result<Option<IssueGroup>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt = conn.prepare(
            "SELECT fingerprint, title, component, level, first_seen, last_seen,
                    count, sparkline_buckets, recent_event_ids
             FROM issue_groups WHERE fingerprint = ?1",
        )?;
        stmt.query_row(params![fingerprint], row_to_group)
            .optional()
            .map_err(|e| StorageError::Database(e.to_string()))
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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
    // We walk from (prev_hour + 1) through new_hour (inclusive), but the
    // caller will set new_hour after we return, so stop just before it.
    for i in 0..skipped {
        let bucket = ((prev_hour + 1 + i as u64) % SPARKLINE_BUCKETS as u64) as usize;
        sparkline[bucket] = 0;
    }
}

/// Insert a brand-new issue group row on first occurrence.
fn insert_new(
    conn: &rusqlite::Connection,
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

    conn.execute(
        "INSERT INTO issue_groups
         (fingerprint, title, component, level,
          first_seen, last_seen, count, sparkline_buckets, recent_event_ids)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8)",
        params![
            fingerprint,
            title,
            component,
            level,
            timestamp_ms,
            timestamp_ms,
            sparkline_json,
            recent_json
        ],
    )
    .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(())
}

/// Increment an existing issue group row: count, last_seen, sparkline, ring buffer.
fn update_existing(
    conn: &rusqlite::Connection,
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

    conn.execute(
        "UPDATE issue_groups
         SET count = count + 1,
             last_seen = ?2,
             sparkline_buckets = ?3,
             recent_event_ids = ?4
         WHERE fingerprint = ?1",
        params![fingerprint, timestamp_ms, updated_sparkline, updated_recent],
    )
    .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(())
}

/// Map a SQLite row from `issue_groups` to an `IssueGroup`.
fn row_to_group(row: &rusqlite::Row<'_>) -> rusqlite::Result<IssueGroup> {
    let fingerprint: String = row.get(0)?;
    let title: String = row.get(1)?;
    let component: String = row.get(2)?;
    let level: String = row.get(3)?;
    let first_seen: i64 = row.get(4)?;
    let last_seen: i64 = row.get(5)?;
    let count: i64 = row.get(6)?;
    let sparkline_json: String = row.get(7)?;
    let recent_json: String = row.get(8)?;

    let sparkline_buckets: Vec<i64> =
        serde_json::from_str(&sparkline_json).unwrap_or_else(|_| vec![0; SPARKLINE_BUCKETS]);
    let recent_event_ids: Vec<u64> = serde_json::from_str(&recent_json).unwrap_or_default();

    Ok(IssueGroup {
        fingerprint,
        title,
        component,
        level,
        first_seen,
        last_seen,
        count,
        sparkline_buckets,
        recent_event_ids,
    })
}

// ---------------------------------------------------------------------------
// Extension trait on rusqlite::Statement for `.optional()`
// ---------------------------------------------------------------------------

/// Extension trait that adds `.optional()` to `query_row` results.
///
/// Converts `QueryReturnedNoRows` into `Ok(None)` instead of an error.
trait OptionalExt<T> {
    fn optional(self) -> rusqlite::Result<Option<T>>;
}

impl<T> OptionalExt<T> for rusqlite::Result<T> {
    fn optional(self) -> rusqlite::Result<Option<T>> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Storage;

    fn open() -> Storage {
        Storage::open_in_memory().expect("in-memory storage")
    }

    /// Compute the millisecond timestamp for a given hour offset from a base.
    fn at_hour(base_ms: i64, hour_offset: i64) -> i64 {
        base_ms + hour_offset * MS_PER_HOUR
    }

    const BASE_MS: i64 = 1_700_000_000_000; // arbitrary fixed point in time

    // -----------------------------------------------------------------------
    // Insert a new group
    // -----------------------------------------------------------------------

    #[test]
    fn insert_new_group_count_one_first_eq_last() {
        let storage = open();
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", BASE_MS, 1)
            .expect("upsert");

        let group = storage
            .issue_groups()
            .get("fp1")
            .expect("get")
            .expect("exists");
        assert_eq!(group.count, 1);
        assert_eq!(group.first_seen, BASE_MS);
        assert_eq!(group.last_seen, BASE_MS);
        assert_eq!(group.recent_event_ids, vec![1u64]);
    }

    #[test]
    fn insert_new_group_sparkline_has_one_in_current_bucket() {
        let storage = open();
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", BASE_MS, 1)
            .expect("upsert");

        let group = storage
            .issue_groups()
            .get("fp1")
            .expect("get")
            .expect("exists");
        assert_eq!(group.sparkline_buckets.len(), SPARKLINE_BUCKETS);
        let bucket = bucket_index(BASE_MS);
        assert_eq!(group.sparkline_buckets[bucket], 1);
        // All other buckets should be zero.
        let total: i64 = group.sparkline_buckets.iter().sum();
        assert_eq!(total, 1);
    }

    // -----------------------------------------------------------------------
    // Upsert same fingerprint
    // -----------------------------------------------------------------------

    #[test]
    fn upsert_same_fingerprint_increments_count_updates_last_seen() {
        let storage = open();
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", BASE_MS, 1)
            .expect("first");
        let ts2 = at_hour(BASE_MS, 0); // same hour
        storage
            .issue_groups()
            .upsert("fp1", "Title", "daemon", "Error", ts2 + 1000, 2)
            .expect("second");

        let group = storage
            .issue_groups()
            .get("fp1")
            .expect("get")
            .expect("exists");
        assert_eq!(group.count, 2);
        assert_eq!(group.first_seen, BASE_MS, "first_seen must not change");
        assert_eq!(group.last_seen, ts2 + 1000);
    }

    // -----------------------------------------------------------------------
    // Ring buffer cap at 50
    // -----------------------------------------------------------------------

    #[test]
    fn ring_buffer_caps_at_fifty_drops_oldest() {
        let storage = open();
        // Insert 60 events into the same fingerprint.
        for id in 1u64..=60 {
            let ts = BASE_MS + id as i64 * 1000;
            storage
                .issue_groups()
                .upsert("fp_ring", "Title", "daemon", "Error", ts, id)
                .expect("upsert");
        }

        let group = storage
            .issue_groups()
            .get("fp_ring")
            .expect("get")
            .expect("exists");
        assert_eq!(group.recent_event_ids.len(), RECENT_EVENT_LIMIT);
        // The oldest 10 (IDs 1-10) should have been dropped; IDs 11-60 remain.
        assert_eq!(*group.recent_event_ids.first().unwrap(), 11u64);
        assert_eq!(*group.recent_event_ids.last().unwrap(), 60u64);
    }

    // -----------------------------------------------------------------------
    // Sparkline: same hour increments correctly
    // -----------------------------------------------------------------------

    #[test]
    fn sparkline_same_hour_increments_same_bucket() {
        let storage = open();
        // Three events in the same hour.
        storage
            .issue_groups()
            .upsert("fp_spark", "T", "d", "E", BASE_MS, 1)
            .expect("1");
        storage
            .issue_groups()
            .upsert("fp_spark", "T", "d", "E", BASE_MS + 1, 2)
            .expect("2");
        storage
            .issue_groups()
            .upsert("fp_spark", "T", "d", "E", BASE_MS + 2, 3)
            .expect("3");

        let group = storage
            .issue_groups()
            .get("fp_spark")
            .expect("get")
            .expect("exists");
        let bucket = bucket_index(BASE_MS);
        assert_eq!(group.sparkline_buckets[bucket], 3);
        let total: i64 = group.sparkline_buckets.iter().sum();
        assert_eq!(total, 3);
    }

    // -----------------------------------------------------------------------
    // Sparkline: rotation across hours
    // -----------------------------------------------------------------------

    #[test]
    fn sparkline_rotation_zeros_skipped_hours() {
        let storage = open();
        // First event at hour H.
        storage
            .issue_groups()
            .upsert("fp_rot", "T", "d", "E", BASE_MS, 1)
            .expect("1");

        // Second event 3 hours later.
        let ts2 = at_hour(BASE_MS, 3);
        storage
            .issue_groups()
            .upsert("fp_rot", "T", "d", "E", ts2, 2)
            .expect("2");

        let group = storage
            .issue_groups()
            .get("fp_rot")
            .expect("get")
            .expect("exists");
        let bucket_h = bucket_index(BASE_MS);
        let bucket_h3 = bucket_index(ts2);

        assert_eq!(group.sparkline_buckets[bucket_h], 1, "first bucket still 1");
        assert_eq!(group.sparkline_buckets[bucket_h3], 1, "new bucket = 1");

        // The two buckets in between (H+1, H+2) should be zero.
        let bucket_h1 = (bucket_h + 1) % SPARKLINE_BUCKETS;
        let bucket_h2 = (bucket_h + 2) % SPARKLINE_BUCKETS;
        assert_eq!(group.sparkline_buckets[bucket_h1], 0);
        assert_eq!(group.sparkline_buckets[bucket_h2], 0);
    }

    // -----------------------------------------------------------------------
    // list_groups sort by count desc
    // -----------------------------------------------------------------------

    #[test]
    fn list_sort_by_count_desc_returns_highest_first() {
        let storage = open();
        // Insert three groups with different counts.
        storage
            .issue_groups()
            .upsert("fp_a", "A", "daemon", "Info", BASE_MS, 1)
            .expect("a");
        storage
            .issue_groups()
            .upsert("fp_b", "B", "daemon", "Info", BASE_MS, 2)
            .expect("b1");
        storage
            .issue_groups()
            .upsert("fp_b", "B", "daemon", "Info", BASE_MS + 1, 3)
            .expect("b2");
        storage
            .issue_groups()
            .upsert("fp_b", "B", "daemon", "Info", BASE_MS + 2, 4)
            .expect("b3");
        storage
            .issue_groups()
            .upsert("fp_c", "C", "daemon", "Info", BASE_MS, 5)
            .expect("c1");
        storage
            .issue_groups()
            .upsert("fp_c", "C", "daemon", "Info", BASE_MS + 1, 6)
            .expect("c2");

        let groups = storage
            .issue_groups()
            .list(SortBy::Count, SortDir::Desc, None, None, 10, 0)
            .expect("list");

        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].fingerprint, "fp_b", "highest count first");
        assert_eq!(groups[0].count, 3);
        assert!(groups[0].count >= groups[1].count);
        assert!(groups[1].count >= groups[2].count);
    }

    // -----------------------------------------------------------------------
    // list_groups filter_component
    // -----------------------------------------------------------------------

    #[test]
    fn list_filter_component_returns_only_matching() {
        let storage = open();
        storage
            .issue_groups()
            .upsert("fp1", "T", "daemon", "Error", BASE_MS, 1)
            .expect("1");
        storage
            .issue_groups()
            .upsert("fp2", "T", "mcp", "Error", BASE_MS, 2)
            .expect("2");
        storage
            .issue_groups()
            .upsert("fp3", "T", "daemon", "Warn", BASE_MS, 3)
            .expect("3");

        let groups = storage
            .issue_groups()
            .list(SortBy::LastSeen, SortDir::Desc, Some("daemon"), None, 10, 0)
            .expect("list");

        assert_eq!(groups.len(), 2);
        assert!(groups.iter().all(|g| g.component == "daemon"));
    }

    // -----------------------------------------------------------------------
    // get returns None for missing fingerprint
    // -----------------------------------------------------------------------

    #[test]
    fn get_returns_none_for_missing_fingerprint() {
        let storage = open();
        let result = storage.issue_groups().get("nonexistent").expect("get");
        assert!(result.is_none());
    }
}
