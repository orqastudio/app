// Violations repository for orqa-storage.
//
// Provides query operations over the `enforcement_violations` table. Violations
// are recorded when the enforcement engine blocks or warns on a tool call. All
// SQL is ported from app/src-tauri/src/repo/violations_repo.rs.

use rusqlite::params;

use orqa_engine_types::types::enforcement::EnforcementViolation;

use crate::error::StorageError;
use crate::Storage;

/// Zero-cost repository handle for the `enforcement_violations` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::violations()`.
pub struct ViolationsRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl ViolationsRepo<'_> {
    /// Record a new enforcement violation.
    pub fn record(
        &self,
        project_id: i64,
        rule_name: &str,
        action: &str,
        tool_name: &str,
        detail: Option<&str>,
    ) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO enforcement_violations \
             (project_id, rule_name, action, tool_name, detail) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![project_id, rule_name, action, tool_name, detail],
        )?;
        Ok(())
    }

    /// Query enforcement violation history for a project, most recent first.
    ///
    /// Returns up to `limit` violations. Pass `None` for no limit.
    pub fn list_for_project(
        &self,
        project_id: i64,
        limit: Option<u32>,
    ) -> Result<Vec<EnforcementViolation>, StorageError> {
        let conn = self.storage.conn()?;

        let sql = match limit {
            Some(n) => format!(
                "SELECT id, project_id, rule_name, action, tool_name, detail, created_at \
                 FROM enforcement_violations \
                 WHERE project_id = ?1 \
                 ORDER BY created_at DESC \
                 LIMIT {n}"
            ),
            None => "SELECT id, project_id, rule_name, action, tool_name, detail, created_at \
                     FROM enforcement_violations \
                     WHERE project_id = ?1 \
                     ORDER BY created_at DESC"
                .to_owned(),
        };

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![project_id], |row| {
            Ok(EnforcementViolation {
                id: row.get(0)?,
                project_id: row.get(1)?,
                rule_name: row.get(2)?,
                action: row.get(3)?,
                tool_name: row.get(4)?,
                detail: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::Storage;

    fn setup() -> Storage {
        let storage = Storage::open_in_memory().expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .expect("create project");
        storage
    }

    #[test]
    fn record_and_list_violations() {
        let storage = setup();
        storage
            .violations()
            .record(1, "RULE-001", "block", "write_file", Some("detail"))
            .expect("record");
        let violations = storage
            .violations()
            .list_for_project(1, None)
            .expect("list");
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_name, "RULE-001");
        assert_eq!(violations[0].action, "block");
    }

    #[test]
    fn list_respects_limit() {
        let storage = setup();
        for i in 0..5 {
            storage
                .violations()
                .record(1, &format!("RULE-{i:03}"), "warn", "bash", None)
                .expect("record");
        }
        let limited = storage
            .violations()
            .list_for_project(1, Some(3))
            .expect("list");
        assert_eq!(limited.len(), 3);
    }

    #[test]
    fn list_empty_when_no_violations() {
        let storage = setup();
        let violations = storage
            .violations()
            .list_for_project(1, None)
            .expect("list");
        assert!(violations.is_empty());
    }
}
