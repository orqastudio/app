use rusqlite::{params, Connection};

use crate::domain::session::{Session, SessionStatus, SessionSummary};
use crate::error::OrqaError;

/// Create a new session and return the full `Session` record.
pub fn create(
    conn: &Connection,
    project_id: i64,
    model: &str,
    system_prompt: Option<&str>,
) -> Result<Session, OrqaError> {
    conn.execute(
        "INSERT INTO sessions (project_id, model, system_prompt) VALUES (?1, ?2, ?3)",
        params![project_id, model, system_prompt],
    )?;

    let id = conn.last_insert_rowid();
    get(conn, id)
}

/// Get a session by its primary key.
pub fn get(conn: &Connection, id: i64) -> Result<Session, OrqaError> {
    conn.query_row(
        "SELECT id, project_id, title, model, system_prompt, status, summary, \
                handoff_notes, total_input_tokens, total_output_tokens, total_cost_usd, \
                sdk_session_id, created_at, updated_at \
         FROM sessions WHERE id = ?1",
        params![id],
        |row| {
            let status_str: String = row.get(5)?;
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                model: row.get(3)?,
                system_prompt: row.get(4)?,
                status: parse_session_status(&status_str),
                summary: row.get(6)?,
                handoff_notes: row.get(7)?,
                total_input_tokens: row.get(8)?,
                total_output_tokens: row.get(9)?,
                total_cost_usd: row.get(10)?,
                sdk_session_id: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!("session {id}")),
        other => OrqaError::Database(other.to_string()),
    })
}

/// List sessions for a project with optional status filter and pagination.
pub fn list(
    conn: &Connection,
    project_id: i64,
    status_filter: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<SessionSummary>, OrqaError> {
    let sql = if status_filter.is_some() {
        "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id) AS message_count, \
                (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                 AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) AS preview \
         FROM sessions s \
         WHERE s.project_id = ?1 AND s.status = ?2 \
         ORDER BY s.updated_at DESC \
         LIMIT ?3 OFFSET ?4"
    } else {
        "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id) AS message_count, \
                (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                 AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) AS preview \
         FROM sessions s \
         WHERE s.project_id = ?1 \
         ORDER BY s.updated_at DESC \
         LIMIT ?2 OFFSET ?3"
    };

    let mut stmt = conn.prepare(sql)?;

    let rows = if let Some(status) = status_filter {
        stmt.query_map(
            params![project_id, status, limit, offset],
            map_session_summary,
        )?
    } else {
        stmt.query_map(params![project_id, limit, offset], map_session_summary)?
    };

    let mut sessions = Vec::new();
    for row in rows {
        sessions.push(row?);
    }
    Ok(sessions)
}

/// Update the title of a session.
pub fn update_title(conn: &Connection, id: i64, title: &str) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE sessions SET title = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?2",
        params![title, id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("session {id}")));
    }
    Ok(())
}

/// Mark a session as completed.
pub fn end_session(conn: &Connection, id: i64) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE sessions SET status = 'completed', \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?1",
        params![id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("session {id}")));
    }
    Ok(())
}

/// Delete a session and its messages (cascading).
pub fn delete(conn: &Connection, id: i64) -> Result<(), OrqaError> {
    let rows = conn.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("session {id}")));
    }
    Ok(())
}

/// Update token usage counters for a session (additive).
pub fn update_token_usage(
    conn: &Connection,
    id: i64,
    input_tokens: i64,
    output_tokens: i64,
) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE sessions SET \
         total_input_tokens = total_input_tokens + ?1, \
         total_output_tokens = total_output_tokens + ?2, \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?3",
        params![input_tokens, output_tokens, id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("session {id}")));
    }
    Ok(())
}

/// Store the SDK session UUID so it survives app restarts.
pub fn update_sdk_session_id(
    conn: &Connection,
    id: i64,
    sdk_session_id: &str,
) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE sessions SET sdk_session_id = ?1, \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?2",
        params![sdk_session_id, id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("session {id}")));
    }
    Ok(())
}

fn parse_session_status(s: &str) -> SessionStatus {
    match s {
        "active" => SessionStatus::Active,
        "completed" => SessionStatus::Completed,
        "abandoned" => SessionStatus::Abandoned,
        "error" => SessionStatus::Error,
        _ => SessionStatus::Error,
    }
}

fn map_session_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<SessionSummary> {
    let status_str: String = row.get(2)?;
    Ok(SessionSummary {
        id: row.get(0)?,
        title: row.get(1)?,
        status: parse_session_status(&status_str),
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
        message_count: row.get(5)?,
        preview: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::repo::project_repo;

    fn setup() -> Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        conn
    }

    #[test]
    fn create_and_get_session() {
        let conn = setup();
        let session =
            create(&conn, 1, "claude-opus-4-6", Some("You are helpful")).expect("create session");

        assert_eq!(session.project_id, 1);
        assert_eq!(session.model, "claude-opus-4-6");
        assert_eq!(session.system_prompt.as_deref(), Some("You are helpful"));
        assert_eq!(session.status, SessionStatus::Active);
        assert_eq!(session.total_input_tokens, 0);
        assert_eq!(session.total_output_tokens, 0);

        let fetched = get(&conn, session.id).expect("get session");
        assert_eq!(fetched.model, "claude-opus-4-6");
    }

    #[test]
    fn get_nonexistent_session() {
        let conn = setup();
        let result = get(&conn, 999);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn list_sessions_with_filter() {
        let conn = setup();
        create(&conn, 1, "auto", None).expect("create s1");
        let s2 = create(&conn, 1, "auto", None).expect("create s2");
        end_session(&conn, s2.id).expect("end s2");

        let all = list(&conn, 1, None, 100, 0).expect("list all");
        assert_eq!(all.len(), 2);

        let active = list(&conn, 1, Some("active"), 100, 0).expect("list active");
        assert_eq!(active.len(), 1);

        let completed = list(&conn, 1, Some("completed"), 100, 0).expect("list completed");
        assert_eq!(completed.len(), 1);
    }

    #[test]
    fn list_sessions_pagination() {
        let conn = setup();
        for _ in 0..5 {
            create(&conn, 1, "auto", None).expect("create");
        }

        let page1 = list(&conn, 1, None, 2, 0).expect("page 1");
        assert_eq!(page1.len(), 2);

        let page2 = list(&conn, 1, None, 2, 2).expect("page 2");
        assert_eq!(page2.len(), 2);

        let page3 = list(&conn, 1, None, 2, 4).expect("page 3");
        assert_eq!(page3.len(), 1);
    }

    #[test]
    fn update_title_works() {
        let conn = setup();
        let s = create(&conn, 1, "auto", None).expect("create");
        assert!(s.title.is_none());

        update_title(&conn, s.id, "My Session").expect("update title");

        let fetched = get(&conn, s.id).expect("get");
        assert_eq!(fetched.title.as_deref(), Some("My Session"));
    }

    #[test]
    fn end_session_works() {
        let conn = setup();
        let s = create(&conn, 1, "auto", None).expect("create");
        assert_eq!(s.status, SessionStatus::Active);

        end_session(&conn, s.id).expect("end session");

        let fetched = get(&conn, s.id).expect("get");
        assert_eq!(fetched.status, SessionStatus::Completed);
    }

    #[test]
    fn delete_session_cascades() {
        let conn = setup();
        let s = create(&conn, 1, "auto", None).expect("create");

        // Add a message
        conn.execute(
            "INSERT INTO messages (session_id, role, content, turn_index, block_index) \
             VALUES (?1, 'user', 'hello', 0, 0)",
            params![s.id],
        )
        .expect("insert message");

        delete(&conn, s.id).expect("delete");

        let result = get(&conn, s.id);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));

        // Verify messages were cascaded
        let msg_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM messages WHERE session_id = ?1",
                params![s.id],
                |row| row.get(0),
            )
            .expect("count messages");
        assert_eq!(msg_count, 0);
    }

    #[test]
    fn update_token_usage_additive() {
        let conn = setup();
        let s = create(&conn, 1, "auto", None).expect("create");

        update_token_usage(&conn, s.id, 100, 50).expect("first update");
        update_token_usage(&conn, s.id, 200, 100).expect("second update");

        let fetched = get(&conn, s.id).expect("get");
        assert_eq!(fetched.total_input_tokens, 300);
        assert_eq!(fetched.total_output_tokens, 150);
    }

    #[test]
    fn update_token_usage_not_found() {
        let conn = setup();
        let result = update_token_usage(&conn, 999, 100, 50);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn update_sdk_session_id_roundtrip() {
        let conn = setup();
        let s = create(&conn, 1, "auto", None).expect("create");
        assert!(s.sdk_session_id.is_none());

        update_sdk_session_id(&conn, s.id, "sdk-uuid-abc").expect("update sdk_session_id");

        let fetched = get(&conn, s.id).expect("get");
        assert_eq!(fetched.sdk_session_id.as_deref(), Some("sdk-uuid-abc"));
    }

    #[test]
    fn update_sdk_session_id_not_found() {
        let conn = setup();
        let result = update_sdk_session_id(&conn, 999, "sdk-uuid");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn session_fk_constraint() {
        let conn = init_memory_db().expect("db init");
        let result = create(&conn, 999, "auto", None);
        assert!(result.is_err(), "should fail with FK violation");
    }
}
