// Health snapshot repository — SQLite persistence for point-in-time graph health metrics.
//
// Each snapshot captures the state of the artifact graph at the moment of an integrity
// scan. Snapshots are used for trend sparklines on the governance dashboard. The schema
// tracks the outlier-based pipeline health model (delivery + learning connectivity,
// outlier counts) rather than raw structural metrics.

use rusqlite::{params, Connection};

use crate::domain::health_snapshot::{HealthSnapshot, NewHealthSnapshot};
use crate::error::OrqaError;

/// Store a new health snapshot for a project.
pub fn create(
    conn: &Connection,
    project_id: i64,
    snapshot: &NewHealthSnapshot,
) -> Result<HealthSnapshot, OrqaError> {
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
    get(conn, id)
}

/// Get a single snapshot by ID.
pub fn get(conn: &Connection, id: i64) -> Result<HealthSnapshot, OrqaError> {
    conn.query_row(
        "SELECT id, project_id, node_count, edge_count, broken_ref_count, \
         error_count, warning_count, largest_component_ratio, avg_degree, \
         pillar_traceability, outlier_count, outlier_percentage, \
         delivery_connectivity, learning_connectivity, created_at \
         FROM health_snapshots WHERE id = ?1",
        params![id],
        |row| {
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
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            OrqaError::NotFound(format!("health snapshot {id} not found"))
        }
        other => OrqaError::Database(other.to_string()),
    })
}

/// Get the most recent N snapshots for a project, ordered newest first.
pub fn get_recent(
    conn: &Connection,
    project_id: i64,
    limit: i64,
) -> Result<Vec<HealthSnapshot>, OrqaError> {
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

    let rows = stmt.query_map(params![project_id, limit], |row| {
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
    })?;

    let mut snapshots = Vec::new();
    for row in rows {
        snapshots.push(row?);
    }
    Ok(snapshots)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;

    fn setup_project(conn: &Connection) -> i64 {
        conn.execute(
            "INSERT INTO projects (name, path) VALUES ('test', '/tmp/test')",
            [],
        )
        .expect("insert project");
        conn.last_insert_rowid()
    }

    #[test]
    fn create_and_get_snapshot() {
        let conn = init_memory_db().expect("db init");
        let project_id = setup_project(&conn);

        let snapshot = create(
            &conn,
            project_id,
            &NewHealthSnapshot {
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
            },
        )
        .expect("create");

        assert_eq!(snapshot.node_count, 100);
        assert_eq!(snapshot.edge_count, 200);
        assert_eq!(snapshot.broken_ref_count, 2);
        assert_eq!(snapshot.error_count, 3);
        assert_eq!(snapshot.warning_count, 7);
        assert!((snapshot.largest_component_ratio - 0.95).abs() < f64::EPSILON);
        assert!((snapshot.pillar_traceability - 87.5).abs() < f64::EPSILON);
        assert_eq!(snapshot.outlier_count, 5);
        assert!((snapshot.outlier_percentage - 5.0).abs() < f64::EPSILON);
        assert!(!snapshot.created_at.is_empty());
    }

    #[test]
    fn get_recent_returns_newest_first() {
        let conn = init_memory_db().expect("db init");
        let project_id = setup_project(&conn);

        for i in 0..5_i64 {
            create(
                &conn,
                project_id,
                &NewHealthSnapshot {
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
                },
            )
            .expect("create");
        }

        let recent = get_recent(&conn, project_id, 3).expect("get_recent");
        assert_eq!(recent.len(), 3);
        // Newest first (highest node_count)
        assert_eq!(recent[0].node_count, 40);
        assert_eq!(recent[1].node_count, 30);
        assert_eq!(recent[2].node_count, 20);
    }

    #[test]
    fn get_recent_empty_project() {
        let conn = init_memory_db().expect("db init");
        let project_id = setup_project(&conn);

        let recent = get_recent(&conn, project_id, 10).expect("get_recent");
        assert!(recent.is_empty());
    }
}
