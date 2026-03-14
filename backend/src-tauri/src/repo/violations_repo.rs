use rusqlite::{params, Connection};

use crate::domain::enforcement_violation::EnforcementViolation;
use crate::error::OrqaError;

/// Query enforcement violation history for a project, most recent first.
///
/// Returns up to `limit` violations. Pass `None` for no limit.
pub fn list_for_project(
    conn: &Connection,
    project_id: i64,
    limit: Option<u32>,
) -> Result<Vec<EnforcementViolation>, OrqaError> {
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
            .to_string(),
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

    let mut violations = Vec::new();
    for row in rows {
        violations.push(row?);
    }
    Ok(violations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;

    fn insert_project(conn: &Connection, path: &str) -> i64 {
        conn.execute(
            "INSERT INTO projects (name, path) VALUES ('test', ?1)",
            params![path],
        )
        .expect("insert project");
        conn.last_insert_rowid()
    }

    fn insert_violation(conn: &Connection, project_id: i64, rule: &str, action: &str) {
        conn.execute(
            "INSERT INTO enforcement_violations (project_id, rule_name, action, tool_name, detail) \
             VALUES (?1, ?2, ?3, 'write_file', 'test detail')",
            params![project_id, rule, action],
        )
        .expect("insert violation");
    }

    #[test]
    fn list_empty_when_no_violations() {
        let conn = init_memory_db().expect("db init");
        let pid = insert_project(&conn, "/test1");
        let violations = list_for_project(&conn, pid, None).expect("list");
        assert!(violations.is_empty());
    }

    #[test]
    fn list_returns_violations_for_project() {
        let conn = init_memory_db().expect("db init");
        let pid = insert_project(&conn, "/test2");
        insert_violation(&conn, pid, "RULE-001", "block");
        insert_violation(&conn, pid, "RULE-002", "warn");

        let violations = list_for_project(&conn, pid, None).expect("list");
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].project_id, pid);
    }

    #[test]
    fn list_excludes_other_project_violations() {
        let conn = init_memory_db().expect("db init");
        let pid1 = insert_project(&conn, "/proj1");
        let pid2 = insert_project(&conn, "/proj2");
        insert_violation(&conn, pid1, "RULE-001", "block");
        insert_violation(&conn, pid2, "RULE-002", "warn");

        let v1 = list_for_project(&conn, pid1, None).expect("list p1");
        let v2 = list_for_project(&conn, pid2, None).expect("list p2");
        assert_eq!(v1.len(), 1);
        assert_eq!(v1[0].rule_name, "RULE-001");
        assert_eq!(v2.len(), 1);
        assert_eq!(v2[0].rule_name, "RULE-002");
    }

    #[test]
    fn list_respects_limit() {
        let conn = init_memory_db().expect("db init");
        let pid = insert_project(&conn, "/test3");
        for i in 0..5 {
            insert_violation(&conn, pid, &format!("RULE-{i:03}"), "warn");
        }

        let violations = list_for_project(&conn, pid, Some(3)).expect("list with limit");
        assert_eq!(violations.len(), 3);
    }

    #[test]
    fn list_ordered_most_recent_first() {
        let conn = init_memory_db().expect("db init");
        let pid = insert_project(&conn, "/test4");
        // Insert with explicit timestamps to ensure ordering
        conn.execute(
            "INSERT INTO enforcement_violations \
             (project_id, rule_name, action, tool_name, detail, created_at) \
             VALUES (?1, 'RULE-EARLY', 'warn', 'bash', 'd', '2026-01-01T00:00:00')",
            params![pid],
        )
        .expect("insert early");
        conn.execute(
            "INSERT INTO enforcement_violations \
             (project_id, rule_name, action, tool_name, detail, created_at) \
             VALUES (?1, 'RULE-LATE', 'block', 'write_file', 'd', '2026-06-01T00:00:00')",
            params![pid],
        )
        .expect("insert late");

        let violations = list_for_project(&conn, pid, None).expect("list");
        assert_eq!(violations[0].rule_name, "RULE-LATE");
        assert_eq!(violations[1].rule_name, "RULE-EARLY");
    }
}
