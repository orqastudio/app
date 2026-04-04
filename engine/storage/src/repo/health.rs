// Health snapshot repository for orqa-storage.
//
// Provides create and query operations over the `health_snapshots` table.
// Snapshots capture the state of the artifact graph at each integrity scan
// and are used for trend sparklines on the governance dashboard. All SQL is
// ported from app/src-tauri/src/repo/health_snapshot_repo.rs.

use rusqlite::params;

use orqa_engine_types::types::health::{HealthSnapshot, NewHealthSnapshot};

use crate::Storage;
use crate::error::StorageError;

/// Zero-cost repository handle for the `health_snapshots` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::health()`.
pub struct HealthRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl HealthRepo<'_> {
    /// Store a new health snapshot for a project and return the inserted row.
    pub fn create(
        &self,
        project_id: i64,
        snapshot: &NewHealthSnapshot,
    ) -> Result<HealthSnapshot, StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO health_snapshots \
             (project_id, node_count, edge_count, broken_ref_count, \
              error_count, warning_count, largest_component_ratio, avg_degree, \
              pillar_traceability, outlier_count, outlier_percentage, \
              delivery_connectivity, learning_connectivity) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                project_id,
                snapshot.node_count,
                snapshot.edge_count,
                snapshot.broken_ref_count,
                snapshot.error_count,
                snapshot.warning_count,
                snapshot.largest_component_ratio,
                snapshot.avg_degree,
                snapshot.pillar_traceability,
                snapshot.outlier_count,
                snapshot.outlier_percentage,
                snapshot.delivery_connectivity,
                snapshot.learning_connectivity,
            ],
        )?;
        let id = conn.last_insert_rowid();
        get_conn(&conn, id)
    }

    /// Get a single snapshot by its ID.
    pub fn get(&self, id: i64) -> Result<HealthSnapshot, StorageError> {
        let conn = self.storage.conn()?;
        get_conn(&conn, id)
    }

    /// Get the most recent N snapshots for a project, ordered newest first.
    pub fn get_recent(
        &self,
        project_id: i64,
        limit: i64,
    ) -> Result<Vec<HealthSnapshot>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, node_count, edge_count, broken_ref_count, \
             error_count, warning_count, largest_component_ratio, avg_degree, \
             pillar_traceability, outlier_count, outlier_percentage, \
             delivery_connectivity, learning_connectivity, created_at \
             FROM health_snapshots \
             WHERE project_id = ?1 \
             ORDER BY id DESC \
             LIMIT ?2",
        )?;

        let result = stmt
            .query_map(params![project_id, limit], map_snapshot)?
            .map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect();
        result
    }
}

/// Fetch a snapshot by id from an existing open connection.
fn get_conn(conn: &rusqlite::Connection, id: i64) -> Result<HealthSnapshot, StorageError> {
    conn.query_row(
        "SELECT id, project_id, node_count, edge_count, broken_ref_count, \
         error_count, warning_count, largest_component_ratio, avg_degree, \
         pillar_traceability, outlier_count, outlier_percentage, \
         delivery_connectivity, learning_connectivity, created_at \
         FROM health_snapshots WHERE id = ?1",
        params![id],
        map_snapshot,
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            StorageError::NotFound(format!("health snapshot {id} not found"))
        }
        other => StorageError::Database(other.to_string()),
    })
}

fn map_snapshot(row: &rusqlite::Row<'_>) -> rusqlite::Result<HealthSnapshot> {
    Ok(HealthSnapshot {
        id: row.get(0)?,
        project_id: row.get(1)?,
        node_count: row.get(2)?,
        edge_count: row.get(3)?,
        broken_ref_count: row.get(4)?,
        error_count: row.get(5)?,
        warning_count: row.get(6)?,
        largest_component_ratio: row.get(7)?,
        avg_degree: row.get(8)?,
        pillar_traceability: row.get(9)?,
        outlier_count: row.get(10)?,
        outlier_percentage: row.get(11)?,
        delivery_connectivity: row.get(12)?,
        learning_connectivity: row.get(13)?,
        created_at: row.get(14)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Storage;

    fn setup() -> Storage {
        let storage = Storage::open_in_memory().expect("in-memory storage");
        storage
            .projects()
            .create("test", "/tmp/test", None)
            .expect("create project");
        storage
    }

    fn sample_snapshot() -> NewHealthSnapshot {
        NewHealthSnapshot {
            node_count: 100,
            edge_count: 200,
            broken_ref_count: 2,
            error_count: 3,
            warning_count: 7,
            largest_component_ratio: 0.95,
            avg_degree: 4.0,
            pillar_traceability: 87.5,
            outlier_count: 5,
            outlier_percentage: 5.0,
            delivery_connectivity: 0.92,
            learning_connectivity: 0.88,
        }
    }

    #[test]
    fn create_and_get_snapshot() {
        let storage = setup();
        let snap = storage.health().create(1, &sample_snapshot()).expect("create");
        assert_eq!(snap.node_count, 100);
        assert_eq!(snap.edge_count, 200);
        assert_eq!(snap.outlier_count, 5);
        assert!(!snap.created_at.is_empty());
    }

    #[test]
    fn get_recent_returns_newest_first() {
        let storage = setup();
        for i in 0..5_i64 {
            let snap = NewHealthSnapshot {
                node_count: i * 10,
                edge_count: 0,
                broken_ref_count: 0,
                error_count: 0,
                warning_count: 0,
                largest_component_ratio: 0.0,
                avg_degree: 0.0,
                pillar_traceability: 100.0,
                outlier_count: 0,
                outlier_percentage: 0.0,
                delivery_connectivity: 1.0,
                learning_connectivity: 1.0,
            };
            storage.health().create(1, &snap).expect("create");
        }
        let recent = storage.health().get_recent(1, 3).expect("get_recent");
        assert_eq!(recent.len(), 3);
        // Newest first (highest node_count inserted last)
        assert_eq!(recent[0].node_count, 40);
        assert_eq!(recent[1].node_count, 30);
        assert_eq!(recent[2].node_count, 20);
    }

    #[test]
    fn get_recent_empty_project() {
        let storage = setup();
        let recent = storage.health().get_recent(1, 10).expect("get_recent");
        assert!(recent.is_empty());
    }
}
