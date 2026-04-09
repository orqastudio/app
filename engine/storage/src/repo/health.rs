// Health snapshot repository for orqa-storage.
//
// Provides async create and query operations over the `health_snapshots` table.
// Snapshots capture the state of the artifact graph at each integrity scan
// and are used for trend sparklines on the governance dashboard.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use orqa_engine_types::types::health::{HealthSnapshot, NewHealthSnapshot};

use crate::error::StorageError;
use crate::traits::HealthRepository;

/// Async repository handle for the `health_snapshots` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::health()`.
pub struct HealthRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Map a SeaORM `QueryResult` row to a `HealthSnapshot` domain value.
///
/// Column positions must match the SELECT order used in every snapshot query.
fn map_snapshot(row: &sea_orm::QueryResult) -> Result<HealthSnapshot, StorageError> {
    Ok(HealthSnapshot {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        project_id: row
            .try_get("", "project_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        node_count: row
            .try_get("", "node_count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        edge_count: row
            .try_get("", "edge_count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        broken_ref_count: row
            .try_get("", "broken_ref_count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        error_count: row
            .try_get("", "error_count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        warning_count: row
            .try_get("", "warning_count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        largest_component_ratio: row
            .try_get("", "largest_component_ratio")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        avg_degree: row
            .try_get("", "avg_degree")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        pillar_traceability: row
            .try_get("", "pillar_traceability")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        outlier_count: row
            .try_get("", "outlier_count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        outlier_percentage: row
            .try_get("", "outlier_percentage")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        delivery_connectivity: row
            .try_get("", "delivery_connectivity")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        learning_connectivity: row
            .try_get("", "learning_connectivity")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        created_at: row
            .try_get("", "created_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
    })
}

/// Fetch a health snapshot by its integer primary key.
async fn fetch_by_id(db: &DatabaseConnection, id: i64) -> Result<HealthSnapshot, StorageError> {
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "SELECT id, project_id, node_count, edge_count, broken_ref_count, \
             error_count, warning_count, largest_component_ratio, avg_degree, \
             pillar_traceability, outlier_count, outlier_percentage, \
             delivery_connectivity, learning_connectivity, created_at \
             FROM health_snapshots WHERE id = ?",
            [id.into()],
        ))
        .await
        .map_err(|e| StorageError::Database(e.to_string()))?
        .ok_or_else(|| StorageError::NotFound(format!("health snapshot {id} not found")))?;
    map_snapshot(&row)
}

#[async_trait::async_trait]
impl HealthRepository for HealthRepo {
    /// Store a new health snapshot for a project and return the inserted row.
    async fn create(
        &self,
        project_id: i64,
        snapshot: &NewHealthSnapshot,
    ) -> Result<HealthSnapshot, StorageError> {
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO health_snapshots \
                 (project_id, node_count, edge_count, broken_ref_count, \
                  error_count, warning_count, largest_component_ratio, avg_degree, \
                  pillar_traceability, outlier_count, outlier_percentage, \
                  delivery_connectivity, learning_connectivity) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                [
                    project_id.into(),
                    snapshot.node_count.into(),
                    snapshot.edge_count.into(),
                    snapshot.broken_ref_count.into(),
                    snapshot.error_count.into(),
                    snapshot.warning_count.into(),
                    snapshot.largest_component_ratio.into(),
                    snapshot.avg_degree.into(),
                    snapshot.pillar_traceability.into(),
                    snapshot.outlier_count.into(),
                    snapshot.outlier_percentage.into(),
                    snapshot.delivery_connectivity.into(),
                    snapshot.learning_connectivity.into(),
                ],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let row = self
            .db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, project_id, node_count, edge_count, broken_ref_count, \
                 error_count, warning_count, largest_component_ratio, avg_degree, \
                 pillar_traceability, outlier_count, outlier_percentage, \
                 delivery_connectivity, learning_connectivity, created_at \
                 FROM health_snapshots ORDER BY id DESC LIMIT 1"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| {
                StorageError::NotFound("health_snapshots table is empty after insert".to_owned())
            })?;
        map_snapshot(&row)
    }

    /// Get a single snapshot by its ID.
    async fn get(&self, id: i64) -> Result<HealthSnapshot, StorageError> {
        fetch_by_id(&self.db, id).await
    }

    /// Get the most recent N snapshots for a project, ordered newest first.
    async fn get_recent(
        &self,
        project_id: i64,
        limit: i64,
    ) -> Result<Vec<HealthSnapshot>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, project_id, node_count, edge_count, broken_ref_count, \
                 error_count, warning_count, largest_component_ratio, avg_degree, \
                 pillar_traceability, outlier_count, outlier_percentage, \
                 delivery_connectivity, learning_connectivity, created_at \
                 FROM health_snapshots \
                 WHERE project_id = ? \
                 ORDER BY id DESC \
                 LIMIT ?",
                [project_id.into(), limit.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter().map(map_snapshot).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{HealthRepository, ProjectRepository};
    use crate::Storage;

    async fn setup() -> Storage {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .projects()
            .create("test", "/tmp/test", None)
            .await
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

    #[tokio::test]
    async fn create_and_get_snapshot() {
        let storage = setup().await;
        let snap = storage
            .health()
            .create(1, &sample_snapshot())
            .await
            .expect("create");
        assert_eq!(snap.node_count, 100);
        assert_eq!(snap.edge_count, 200);
        assert_eq!(snap.outlier_count, 5);
        assert!(!snap.created_at.is_empty());
    }

    #[tokio::test]
    async fn get_recent_returns_newest_first() {
        let storage = setup().await;
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
            storage.health().create(1, &snap).await.expect("create");
        }
        let recent = storage.health().get_recent(1, 3).await.expect("get_recent");
        assert_eq!(recent.len(), 3);
        // Newest first (highest node_count inserted last)
        assert_eq!(recent[0].node_count, 40);
        assert_eq!(recent[1].node_count, 30);
        assert_eq!(recent[2].node_count, 20);
    }

    #[tokio::test]
    async fn get_recent_empty_project() {
        let storage = setup().await;
        let recent = storage
            .health()
            .get_recent(1, 10)
            .await
            .expect("get_recent");
        assert!(recent.is_empty());
    }
}
