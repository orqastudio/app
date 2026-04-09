// Violations repository for orqa-storage.
//
// Provides async record and query operations over the `enforcement_violations`
// table. Violations are recorded when the enforcement engine blocks or warns
// on a tool call.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use orqa_engine_types::types::enforcement::EnforcementViolation;

use crate::error::StorageError;
use crate::traits::ViolationRepository;

/// Async repository handle for the `enforcement_violations` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::violations()`.
pub struct ViolationsRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Map a SeaORM `QueryResult` row to an `EnforcementViolation` domain value.
fn map_violation(row: &sea_orm::QueryResult) -> Result<EnforcementViolation, StorageError> {
    Ok(EnforcementViolation {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        project_id: row
            .try_get("", "project_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        rule_name: row
            .try_get("", "rule_name")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        action: row
            .try_get("", "action")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        tool_name: row
            .try_get("", "tool_name")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        detail: row
            .try_get("", "detail")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        created_at: row
            .try_get("", "created_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
    })
}

#[async_trait::async_trait]
impl ViolationRepository for ViolationsRepo {
    /// Record a new enforcement violation.
    async fn record(
        &self,
        project_id: i64,
        rule_name: &str,
        action: &str,
        tool_name: &str,
        detail: Option<&str>,
    ) -> Result<(), StorageError> {
        let detail_val: sea_orm::Value = match detail {
            Some(d) => d.into(),
            None => sea_orm::Value::String(None),
        };
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO enforcement_violations \
                 (project_id, rule_name, action, tool_name, detail) \
                 VALUES (?, ?, ?, ?, ?)",
                [
                    project_id.into(),
                    rule_name.into(),
                    action.into(),
                    tool_name.into(),
                    detail_val,
                ],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Query enforcement violation history for a project, most recent first.
    ///
    /// `limit` caps the result count; pass `None` for all rows.
    async fn list_for_project(
        &self,
        project_id: i64,
        limit: Option<u32>,
    ) -> Result<Vec<EnforcementViolation>, StorageError> {
        let sql = match limit {
            Some(n) => format!(
                "SELECT id, project_id, rule_name, action, tool_name, detail, created_at \
                 FROM enforcement_violations \
                 WHERE project_id = ? \
                 ORDER BY created_at DESC \
                 LIMIT {n}"
            ),
            None => "SELECT id, project_id, rule_name, action, tool_name, detail, created_at \
                     FROM enforcement_violations \
                     WHERE project_id = ? \
                     ORDER BY created_at DESC"
                .to_owned(),
        };

        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &sql,
                [project_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter().map(map_violation).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{ProjectRepository, ViolationRepository};
    use crate::Storage;

    async fn setup() -> Storage {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .await
            .expect("create project");
        storage
    }

    #[tokio::test]
    async fn record_and_list_violations() {
        let storage = setup().await;
        storage
            .violations()
            .record(1, "RULE-001", "block", "write_file", Some("detail"))
            .await
            .expect("record");
        let violations = storage
            .violations()
            .list_for_project(1, None)
            .await
            .expect("list");
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_name, "RULE-001");
        assert_eq!(violations[0].action, "block");
    }

    #[tokio::test]
    async fn list_respects_limit() {
        let storage = setup().await;
        for i in 0..5 {
            storage
                .violations()
                .record(1, &format!("RULE-{i:03}"), "warn", "bash", None)
                .await
                .expect("record");
        }
        let limited = storage
            .violations()
            .list_for_project(1, Some(3))
            .await
            .expect("list");
        assert_eq!(limited.len(), 3);
    }

    #[tokio::test]
    async fn list_empty_when_no_violations() {
        let storage = setup().await;
        let violations = storage
            .violations()
            .list_for_project(1, None)
            .await
            .expect("list");
        assert!(violations.is_empty());
    }
}
