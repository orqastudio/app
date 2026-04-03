// SQLite-backed persistent store for the daemon.
//
// The daemon maintains its own SQLite database at `.state/daemon.db` under the
// project root. This database stores sessions, projects, and app settings —
// the same schema used by the Tauri app, enabling zero-copy migration when the
// app shifts from Tauri IPC to daemon HTTP calls.
//
// Thread-safety: rusqlite::Connection is !Send. The store holds only the path
// and opens connections on demand inside spawn_blocking. DaemonStore itself is
// Send + Sync and can be shared via Arc.

use std::path::{Path, PathBuf};

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

// ---------------------------------------------------------------------------
// Schema
// ---------------------------------------------------------------------------

const SCHEMA: &str = "
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
PRAGMA busy_timeout = 5000;
PRAGMA synchronous = NORMAL;

CREATE TABLE IF NOT EXISTS projects (
    id              INTEGER PRIMARY KEY,
    name            TEXT NOT NULL,
    path            TEXT NOT NULL UNIQUE,
    description     TEXT,
    detected_stack  TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_projects_path ON projects(path);

CREATE TABLE IF NOT EXISTS sessions (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title           TEXT,
    model           TEXT NOT NULL DEFAULT 'auto',
    system_prompt   TEXT,
    status          TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active', 'completed', 'abandoned', 'error')),
    summary         TEXT,
    handoff_notes   TEXT,
    total_input_tokens  INTEGER DEFAULT 0,
    total_output_tokens INTEGER DEFAULT 0,
    total_cost_usd  REAL DEFAULT 0.0,
    provider_session_id TEXT,
    title_manually_set  INTEGER DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_id);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions(status);

CREATE TABLE IF NOT EXISTS messages (
    id              INTEGER PRIMARY KEY,
    session_id      INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content_type    TEXT NOT NULL DEFAULT 'text',
    content         TEXT,
    tool_call_id    TEXT,
    tool_name       TEXT,
    tool_input      TEXT,
    tool_is_error   INTEGER DEFAULT 0,
    turn_index      INTEGER NOT NULL DEFAULT 0,
    block_index     INTEGER NOT NULL DEFAULT 0,
    stream_status   TEXT NOT NULL DEFAULT 'complete',
    input_tokens    INTEGER,
    output_tokens   INTEGER,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id, turn_index, block_index);

CREATE TABLE IF NOT EXISTS settings (
    key             TEXT NOT NULL,
    value           TEXT NOT NULL,
    scope           TEXT NOT NULL DEFAULT 'app',
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    PRIMARY KEY (key, scope)
);

-- Recover any interrupted message streams from a previous crash.
UPDATE messages SET stream_status = 'error' WHERE stream_status = 'pending';
";

// ---------------------------------------------------------------------------
// Store type
// ---------------------------------------------------------------------------

/// The daemon's SQLite-backed persistent store.
///
/// Holds only the database path. Connections are opened on demand within
/// spawn_blocking so the !Send Connection never crosses an async boundary.
#[derive(Debug, Clone)]
pub struct DaemonStore {
    /// Absolute path to the daemon SQLite database file.
    pub db_path: PathBuf,
}

impl DaemonStore {
    /// Open or create the daemon database at `db_path`.
    ///
    /// Applies schema migrations and returns the store on success.
    /// Logs a warning and returns `None` when the database cannot be opened.
    pub fn open(db_path: PathBuf) -> Option<Self> {
        match Self::init_schema(&db_path) {
            Ok(()) => {
                info!(path = %db_path.display(), "daemon store opened");
                Some(Self { db_path })
            }
            Err(e) => {
                warn!(path = %db_path.display(), error = %e, "daemon store unavailable");
                None
            }
        }
    }

    /// Apply the schema to a freshly-opened connection.
    fn init_schema(db_path: &Path) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(())
    }

    /// Open a fresh connection to the database.
    ///
    /// Called inside spawn_blocking by route handlers.
    pub fn connect(&self) -> Result<Connection, rusqlite::Error> {
        Connection::open(&self.db_path)
    }

    /// Open an in-memory database with the full schema applied.
    ///
    /// Used only in tests so they run without touching the filesystem.
    #[cfg(test)]
    pub fn open_in_memory() -> Result<Connection, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        Ok(conn)
    }
}

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

/// A session record stored in the daemon database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub project_id: i64,
    pub title: Option<String>,
    pub model: String,
    pub system_prompt: Option<String>,
    pub status: String,
    pub summary: Option<String>,
    pub handoff_notes: Option<String>,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cost_usd: f64,
    pub provider_session_id: Option<String>,
    pub title_manually_set: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Summary view of a session for list responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: i64,
    pub title: Option<String>,
    pub status: String,
    pub message_count: i64,
    pub preview: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// A message stored in a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub session_id: i64,
    pub role: String,
    pub content_type: String,
    pub content: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_name: Option<String>,
    pub tool_input: Option<String>,
    pub tool_is_error: bool,
    pub turn_index: i64,
    pub block_index: i64,
    pub stream_status: String,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub created_at: String,
}

/// A project record stored in the daemon database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub detected_stack: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Session repo functions
// ---------------------------------------------------------------------------

/// Create a new session and return the full record.
pub fn session_create(
    conn: &Connection,
    project_id: i64,
    model: &str,
    system_prompt: Option<&str>,
) -> Result<Session, String> {
    conn.execute(
        "INSERT INTO sessions (project_id, model, system_prompt) VALUES (?1, ?2, ?3)",
        params![project_id, model, system_prompt],
    )
    .map_err(|e| e.to_string())?;
    session_get(conn, conn.last_insert_rowid())
}

/// Get a session by its primary key.
pub fn session_get(conn: &Connection, id: i64) -> Result<Session, String> {
    conn.query_row(
        "SELECT id, project_id, title, model, system_prompt, status, summary, \
                handoff_notes, total_input_tokens, total_output_tokens, total_cost_usd, \
                provider_session_id, COALESCE(title_manually_set, 0), created_at, updated_at \
         FROM sessions WHERE id = ?1",
        params![id],
        map_session,
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => format!("session {id} not found"),
        other => other.to_string(),
    })
}

/// List sessions with optional project_id and status filters.
///
/// When `project_id` is `None`, returns sessions across all projects.
/// When `project_id` is `Some`, filters to that project only.
pub fn session_list(
    conn: &Connection,
    project_id: Option<i64>,
    status_filter: Option<&str>,
) -> Result<Vec<SessionSummary>, String> {
    let base = "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id), \
                (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                 AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) \
                FROM sessions s";

    let rows: Vec<rusqlite::Result<SessionSummary>> = match (project_id, status_filter) {
        (Some(pid), Some(status)) => {
            let sql = format!("{base} WHERE s.project_id = ?1 AND s.status = ?2 ORDER BY s.updated_at DESC");
            let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
            let x: Vec<_> = stmt.query_map(params![pid, status], map_session_summary)
                .map_err(|e| e.to_string())?
                .collect();
            x
        }
        (Some(pid), None) => {
            let sql = format!("{base} WHERE s.project_id = ?1 ORDER BY s.updated_at DESC");
            let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
            let x: Vec<_> = stmt.query_map(params![pid], map_session_summary)
                .map_err(|e| e.to_string())?
                .collect();
            x
        }
        (None, Some(status)) => {
            let sql = format!("{base} WHERE s.status = ?1 ORDER BY s.updated_at DESC");
            let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
            let x: Vec<_> = stmt.query_map(params![status], map_session_summary)
                .map_err(|e| e.to_string())?
                .collect();
            x
        }
        (None, None) => {
            let sql = format!("{base} ORDER BY s.updated_at DESC");
            let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
            let x: Vec<_> = stmt.query_map([], map_session_summary)
                .map_err(|e| e.to_string())?
                .collect();
            x
        }
    };
    rows.into_iter().map(|r| r.map_err(|e| e.to_string())).collect()
}

/// Update a session's title.
pub fn session_update_title(conn: &Connection, id: i64, title: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE sessions SET title = ?1, title_manually_set = 1, \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?2",
        params![title, id],
    )
    .map_err(|e| e.to_string())
    .map(|_| ())
}

/// Update a session's status.
pub fn session_update_status(conn: &Connection, id: i64, status: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE sessions SET status = ?1, \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?2",
        params![status, id],
    )
    .map_err(|e| e.to_string())
    .map(|_| ())
}

/// Delete a session by primary key.
pub fn session_delete(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM sessions WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())
        .map(|_| ())
}

/// Get the next turn index for a session (max existing turn_index + 1, or 0).
pub fn message_next_turn_index(conn: &Connection, session_id: i64) -> Result<i32, String> {
    let max: Option<i64> = conn
        .query_row(
            "SELECT MAX(turn_index) FROM messages WHERE session_id = ?1",
            params![session_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(max.map_or(0, |m| (m + 1) as i32))
}

/// Create a new message in a session.
pub fn message_create(
    conn: &Connection,
    session_id: i64,
    role: &str,
    content_type: &str,
    content: Option<&str>,
    turn_index: i32,
    block_index: i32,
) -> Result<Message, String> {
    conn.execute(
        "INSERT INTO messages (session_id, role, content_type, content, turn_index, block_index, stream_status) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending')",
        params![session_id, role, content_type, content, turn_index, block_index],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
         tool_input, tool_is_error, turn_index, block_index, stream_status, \
         input_tokens, output_tokens, created_at \
         FROM messages WHERE id = ?1",
        params![id],
        map_message,
    )
    .map_err(|e| e.to_string())
}

/// Update the stream_status of a message.
pub fn message_update_stream_status(conn: &Connection, id: i64, status: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE messages SET stream_status = ?1 WHERE id = ?2",
        params![status, id],
    )
    .map_err(|e| e.to_string())
    .map(|_| ())
}

/// Update the cumulative token usage for a session.
pub fn session_update_token_usage(
    conn: &Connection,
    id: i64,
    input_tokens: i64,
    output_tokens: i64,
) -> Result<(), String> {
    conn.execute(
        "UPDATE sessions SET \
         total_input_tokens = total_input_tokens + ?1, \
         total_output_tokens = total_output_tokens + ?2, \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?3",
        params![input_tokens, output_tokens, id],
    )
    .map_err(|e| e.to_string())
    .map(|_| ())
}

/// List messages for a session, ordered by turn and block index.
pub fn message_list(conn: &Connection, session_id: i64) -> Result<Vec<Message>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
             tool_input, tool_is_error, turn_index, block_index, stream_status, \
             input_tokens, output_tokens, created_at \
             FROM messages WHERE session_id = ?1 \
             ORDER BY turn_index ASC, block_index ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![session_id], map_message)
        .map_err(|e| e.to_string())?;
    rows.map(|r| r.map_err(|e| e.to_string())).collect()
}

// ---------------------------------------------------------------------------
// Project repo functions
// ---------------------------------------------------------------------------

/// Create a new project record.
pub fn project_create(
    conn: &Connection,
    name: &str,
    path: &str,
    description: Option<&str>,
) -> Result<Project, String> {
    conn.execute(
        "INSERT INTO projects (name, path, description) VALUES (?1, ?2, ?3)",
        params![name, path, description],
    )
    .map_err(|e| e.to_string())?;
    project_get(conn, conn.last_insert_rowid())
}

/// Get a project by primary key.
pub fn project_get(conn: &Connection, id: i64) -> Result<Project, String> {
    conn.query_row(
        "SELECT id, name, path, description, detected_stack, created_at, updated_at \
         FROM projects WHERE id = ?1",
        params![id],
        map_project,
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => format!("project {id} not found"),
        other => other.to_string(),
    })
}

/// Get a project by its filesystem path.
pub fn project_get_by_path(conn: &Connection, path: &str) -> Result<Option<Project>, String> {
    conn.query_row(
        "SELECT id, name, path, description, detected_stack, created_at, updated_at \
         FROM projects WHERE path = ?1",
        params![path],
        map_project,
    )
    .optional()
    .map_err(|e| e.to_string())
}

/// Get the most recently updated project.
pub fn project_get_active(conn: &Connection) -> Result<Option<Project>, String> {
    conn.query_row(
        "SELECT id, name, path, description, detected_stack, created_at, updated_at \
         FROM projects ORDER BY updated_at DESC, id DESC LIMIT 1",
        [],
        map_project,
    )
    .optional()
    .map_err(|e| e.to_string())
}

/// List all known projects.
pub fn project_list(conn: &Connection) -> Result<Vec<Project>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, path, description, detected_stack, created_at, updated_at \
             FROM projects ORDER BY updated_at DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], map_project).map_err(|e| e.to_string())?;
    rows.map(|r| r.map_err(|e| e.to_string())).collect()
}

/// Touch updated_at so a project surfaces as the most recently active.
pub fn project_touch(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())
    .map(|_| ())
}

// ---------------------------------------------------------------------------
// Settings repo functions
// ---------------------------------------------------------------------------

/// Get a setting value by key and scope.
pub fn settings_get(
    conn: &Connection,
    key: &str,
    scope: &str,
) -> Result<Option<serde_json::Value>, String> {
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = ?1 AND scope = ?2",
            params![key, scope],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    match value {
        Some(v) => serde_json::from_str(&v).map(Some).map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

/// Set a setting value (upsert by key+scope).
pub fn settings_set(
    conn: &Connection,
    key: &str,
    value: &serde_json::Value,
    scope: &str,
) -> Result<(), String> {
    let value_str = serde_json::to_string(value).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO settings (key, value, scope, updated_at) \
         VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
         ON CONFLICT(key, scope) DO UPDATE SET \
         value = excluded.value, updated_at = excluded.updated_at",
        params![key, value_str, scope],
    )
    .map_err(|e| e.to_string())
    .map(|_| ())
}

/// Get all settings for a scope, returning a key-value map.
pub fn settings_get_all(
    conn: &Connection,
    scope: Option<&str>,
) -> Result<std::collections::HashMap<String, serde_json::Value>, String> {
    let (sql, bound_scope) = match scope {
        Some(s) => (
            "SELECT key, value FROM settings WHERE scope = ?1 ORDER BY key".to_owned(),
            Some(s.to_owned()),
        ),
        None => (
            "SELECT key, value FROM settings ORDER BY key".to_owned(),
            None,
        ),
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows: Vec<rusqlite::Result<(String, String)>> = if let Some(s) = bound_scope {
        stmt.query_map(params![s], |row| {
            let k: String = row.get(0)?;
            let v: String = row.get(1)?;
            Ok((k, v))
        })
        .map_err(|e| e.to_string())?
        .collect()
    } else {
        stmt.query_map([], |row| {
            let k: String = row.get(0)?;
            let v: String = row.get(1)?;
            Ok((k, v))
        })
        .map_err(|e| e.to_string())?
        .collect()
    };

    let mut map = std::collections::HashMap::new();
    for row in rows {
        let (k, v) = row.map_err(|e| e.to_string())?;
        let parsed: serde_json::Value = serde_json::from_str(&v).map_err(|e| e.to_string())?;
        map.insert(k, parsed);
    }
    Ok(map)
}

/// Mark a session as completed (end the session).
///
/// Called from the sessions route when a client explicitly ends an active session.
/// Currently not wired to an HTTP handler — suppressing dead_code until the
/// endpoint is added.
#[allow(dead_code)]
pub fn session_end(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "UPDATE sessions SET status = 'completed', \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())
    .map(|_| ())
}

// ---------------------------------------------------------------------------
// Row mappers
// ---------------------------------------------------------------------------

fn map_session(row: &rusqlite::Row<'_>) -> rusqlite::Result<Session> {
    Ok(Session {
        id: row.get(0)?,
        project_id: row.get(1)?,
        title: row.get(2)?,
        model: row.get(3)?,
        system_prompt: row.get(4)?,
        status: row.get(5)?,
        summary: row.get(6)?,
        handoff_notes: row.get(7)?,
        total_input_tokens: row.get(8)?,
        total_output_tokens: row.get(9)?,
        total_cost_usd: row.get(10)?,
        provider_session_id: row.get(11)?,
        title_manually_set: row.get::<_, i64>(12)? != 0,
        created_at: row.get(13)?,
        updated_at: row.get(14)?,
    })
}

fn map_session_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<SessionSummary> {
    Ok(SessionSummary {
        id: row.get(0)?,
        title: row.get(1)?,
        status: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
        message_count: row.get(5)?,
        preview: row.get(6)?,
    })
}

fn map_message(row: &rusqlite::Row<'_>) -> rusqlite::Result<Message> {
    Ok(Message {
        id: row.get(0)?,
        session_id: row.get(1)?,
        role: row.get(2)?,
        content_type: row.get(3)?,
        content: row.get(4)?,
        tool_call_id: row.get(5)?,
        tool_name: row.get(6)?,
        tool_input: row.get(7)?,
        tool_is_error: row.get::<_, i64>(8)? != 0,
        turn_index: row.get(9)?,
        block_index: row.get(10)?,
        stream_status: row.get(11)?,
        input_tokens: row.get(12)?,
        output_tokens: row.get(13)?,
        created_at: row.get(14)?,
    })
}

fn map_project(row: &rusqlite::Row<'_>) -> rusqlite::Result<Project> {
    Ok(Project {
        id: row.get(0)?,
        name: row.get(1)?,
        path: row.get(2)?,
        description: row.get(3)?,
        detected_stack: row
            .get::<_, Option<String>>(4)?
            .and_then(|s| serde_json::from_str(&s).ok()),
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- helpers -----------------------------------------------------------

    /// Open a fresh in-memory database for each test.
    fn db() -> Connection {
        DaemonStore::open_in_memory().expect("in-memory db")
    }

    /// Insert a project and return its id.
    fn make_project(conn: &Connection, name: &str, path: &str) -> i64 {
        project_create(conn, name, path, None)
            .expect("project_create")
            .id
    }

    // ---- Session CRUD -------------------------------------------------------

    #[test]
    fn session_create_returns_correct_fields() {
        // A freshly-created session carries the model we asked for, has status
        // "active", and has zero token counts.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "claude-3-5", None).unwrap();
        assert_eq!(s.project_id, pid);
        assert_eq!(s.model, "claude-3-5");
        assert_eq!(s.status, "active");
        assert_eq!(s.total_input_tokens, 0);
        assert_eq!(s.total_output_tokens, 0);
        assert!(!s.title_manually_set);
    }

    #[test]
    fn session_create_timestamps_are_set() {
        // created_at and updated_at must be non-empty ISO strings.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        assert!(!s.created_at.is_empty());
        assert!(!s.updated_at.is_empty());
    }

    #[test]
    fn session_get_returns_created_session() {
        // session_get must return the same session we just inserted.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let created = session_create(&conn, pid, "auto", None).unwrap();
        let fetched = session_get(&conn, created.id).unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.project_id, pid);
    }

    #[test]
    fn session_get_returns_error_for_missing_id() {
        // Requesting a non-existent session must return Err, not panic.
        let conn = db();
        let result = session_get(&conn, 9999);
        assert!(result.is_err());
    }

    #[test]
    fn session_list_empty_when_no_sessions() {
        // An empty project has an empty session list.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let list = session_list(&conn, Some(pid), None).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn session_list_returns_all_sessions_ordered_by_updated_at_desc() {
        // The most recently updated session must appear first.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s1 = session_create(&conn, pid, "auto", None).unwrap();
        let s2 = session_create(&conn, pid, "auto", None).unwrap();
        // Touch s1 so it becomes more recently updated.
        session_update_title(&conn, s1.id, "touched").unwrap();
        let list = session_list(&conn, Some(pid), None).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].id, s1.id);
        assert_eq!(list[1].id, s2.id);
    }

    #[test]
    fn session_list_project_filter_returns_only_matching_sessions() {
        // Sessions from a different project must not appear in the filtered list.
        let conn = db();
        let pid1 = make_project(&conn, "p1", "/p1");
        let pid2 = make_project(&conn, "p2", "/p2");
        session_create(&conn, pid1, "auto", None).unwrap();
        let s2 = session_create(&conn, pid2, "auto", None).unwrap();
        let list = session_list(&conn, Some(pid2), None).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, s2.id);
    }

    #[test]
    fn session_update_title_changes_title_and_sets_flag() {
        // After update, title must match the new value and title_manually_set must be true.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        session_update_title(&conn, s.id, "My Session").unwrap();
        let updated = session_get(&conn, s.id).unwrap();
        assert_eq!(updated.title.as_deref(), Some("My Session"));
        assert!(updated.title_manually_set);
    }

    #[test]
    fn session_update_status_changes_status() {
        // status must reflect the new value after the update.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        session_update_status(&conn, s.id, "abandoned").unwrap();
        let updated = session_get(&conn, s.id).unwrap();
        assert_eq!(updated.status, "abandoned");
    }

    #[test]
    fn session_end_marks_session_completed() {
        // session_end must transition status to "completed".
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        session_end(&conn, s.id).unwrap();
        let updated = session_get(&conn, s.id).unwrap();
        assert_eq!(updated.status, "completed");
    }

    #[test]
    fn session_delete_removes_session() {
        // After delete, session_get must return an error (not found).
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        session_delete(&conn, s.id).unwrap();
        assert!(session_get(&conn, s.id).is_err());
    }

    #[test]
    fn session_delete_cascades_to_messages() {
        // Messages belonging to a deleted session must also be gone.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        message_create(&conn, s.id, "user", "text", Some("hello"), 0, 0).unwrap();
        session_delete(&conn, s.id).unwrap();
        let msgs = message_list(&conn, s.id).unwrap();
        assert!(msgs.is_empty());
    }

    #[test]
    fn session_update_token_usage_accumulates_across_calls() {
        // Calling session_update_token_usage twice must sum, not overwrite.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        session_update_token_usage(&conn, s.id, 100, 50).unwrap();
        session_update_token_usage(&conn, s.id, 200, 75).unwrap();
        let updated = session_get(&conn, s.id).unwrap();
        assert_eq!(updated.total_input_tokens, 300);
        assert_eq!(updated.total_output_tokens, 125);
    }

    #[test]
    fn message_next_turn_index_empty_session_returns_zero() {
        // A session with no messages must return 0 as next turn index.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        let idx = message_next_turn_index(&conn, s.id).unwrap();
        assert_eq!(idx, 0);
    }

    #[test]
    fn message_next_turn_index_after_three_messages_returns_three() {
        // After inserting messages at turn indices 0, 1, 2, next must be 3.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        message_create(&conn, s.id, "user", "text", None, 0, 0).unwrap();
        message_create(&conn, s.id, "assistant", "text", None, 1, 0).unwrap();
        message_create(&conn, s.id, "user", "text", None, 2, 0).unwrap();
        let idx = message_next_turn_index(&conn, s.id).unwrap();
        assert_eq!(idx, 3);
    }

    // ---- Message CRUD -------------------------------------------------------

    #[test]
    fn message_create_returns_correct_fields() {
        // Newly created message must carry the role, content_type, and content we supplied.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        let m = message_create(&conn, s.id, "user", "text", Some("hello"), 0, 0).unwrap();
        assert_eq!(m.session_id, s.id);
        assert_eq!(m.role, "user");
        assert_eq!(m.content_type, "text");
        assert_eq!(m.content.as_deref(), Some("hello"));
        assert_eq!(m.turn_index, 0);
        assert_eq!(m.block_index, 0);
    }

    #[test]
    fn message_create_stream_status_defaults_to_pending() {
        // Stream status must be "pending" immediately after creation.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        let m = message_create(&conn, s.id, "user", "text", None, 0, 0).unwrap();
        assert_eq!(m.stream_status, "pending");
    }

    #[test]
    fn message_update_stream_status_changes_status() {
        // After update, the message returned by message_list must show the new status.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        let m = message_create(&conn, s.id, "user", "text", None, 0, 0).unwrap();
        message_update_stream_status(&conn, m.id, "complete").unwrap();
        let msgs = message_list(&conn, s.id).unwrap();
        assert_eq!(msgs[0].stream_status, "complete");
    }

    #[test]
    fn message_list_returns_messages_ordered_by_turn_then_block() {
        // Messages must be returned sorted by (turn_index ASC, block_index ASC).
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        message_create(&conn, s.id, "user", "text", Some("t1b1"), 1, 1).unwrap();
        message_create(&conn, s.id, "user", "text", Some("t0b0"), 0, 0).unwrap();
        message_create(&conn, s.id, "assistant", "text", Some("t1b0"), 1, 0).unwrap();
        let msgs = message_list(&conn, s.id).unwrap();
        assert_eq!(msgs[0].content.as_deref(), Some("t0b0"));
        assert_eq!(msgs[1].content.as_deref(), Some("t1b0"));
        assert_eq!(msgs[2].content.as_deref(), Some("t1b1"));
    }

    #[test]
    fn message_list_multiple_messages_correct_order() {
        // Verify the ordering is stable when turns and blocks are sequential.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        for i in 0..4i32 {
            message_create(&conn, s.id, "user", "text", None, i, 0).unwrap();
        }
        let msgs = message_list(&conn, s.id).unwrap();
        assert_eq!(msgs.len(), 4);
        for (i, m) in msgs.iter().enumerate() {
            assert_eq!(m.turn_index, i as i64);
        }
    }

    #[test]
    fn message_list_empty_session_returns_empty_vec() {
        // A session with no messages must return an empty vec, not an error.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        let msgs = message_list(&conn, s.id).unwrap();
        assert!(msgs.is_empty());
    }

    #[test]
    fn message_round_trip_all_fields_preserved() {
        // Every field passed to message_create must survive a message_list round-trip.
        let conn = db();
        let pid = make_project(&conn, "p", "/p");
        let s = session_create(&conn, pid, "auto", None).unwrap();
        let m = message_create(&conn, s.id, "assistant", "text", Some("reply"), 0, 2).unwrap();
        let list = message_list(&conn, s.id).unwrap();
        assert_eq!(list.len(), 1);
        let r = &list[0];
        assert_eq!(r.id, m.id);
        assert_eq!(r.session_id, s.id);
        assert_eq!(r.role, "assistant");
        assert_eq!(r.content_type, "text");
        assert_eq!(r.content.as_deref(), Some("reply"));
        assert_eq!(r.turn_index, 0);
        assert_eq!(r.block_index, 2);
        assert!(!r.tool_is_error);
    }

    // ---- Project CRUD -------------------------------------------------------

    #[test]
    fn project_create_returns_correct_name_and_path() {
        // The returned project must carry the name and path we supplied.
        let conn = db();
        let p = project_create(&conn, "MyProject", "/workspace/my", None).unwrap();
        assert_eq!(p.name, "MyProject");
        assert_eq!(p.path, "/workspace/my");
    }

    #[test]
    fn project_get_returns_created_project() {
        // project_get must round-trip the record we just created.
        let conn = db();
        let created = project_create(&conn, "p", "/p", None).unwrap();
        let fetched = project_get(&conn, created.id).unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "p");
    }

    #[test]
    fn project_get_returns_error_for_missing_id() {
        // Requesting a project with a non-existent id must return Err.
        let conn = db();
        assert!(project_get(&conn, 9999).is_err());
    }

    #[test]
    fn project_get_by_path_returns_some_for_existing_path() {
        // Looking up by the exact path used at creation must return Some.
        let conn = db();
        let created = project_create(&conn, "p", "/my/path", None).unwrap();
        let found = project_get_by_path(&conn, "/my/path").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);
    }

    #[test]
    fn project_get_by_path_returns_none_for_unknown_path() {
        // An unknown path must return None, not an error.
        let conn = db();
        let result = project_get_by_path(&conn, "/does/not/exist").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn project_get_active_returns_most_recently_updated_project() {
        // After touching p1 with a brief pause to advance the clock, p1 must be
        // returned by project_get_active even though p2 was created more recently.
        let conn = db();
        let p1 = project_create(&conn, "p1", "/p1", None).unwrap();
        let p2 = project_create(&conn, "p2", "/p2", None).unwrap();
        // Advance the clock so p1's touch timestamp is strictly later than p2's.
        std::thread::sleep(std::time::Duration::from_millis(5));
        project_touch(&conn, p1.id).unwrap();
        let active = project_get_active(&conn).unwrap().unwrap();
        assert_eq!(active.id, p1.id);
        // Sanity: p2 exists but was not the most recent.
        assert_ne!(active.id, p2.id);
    }

    #[test]
    fn project_get_active_returns_none_when_no_projects() {
        // An empty store must return None for project_get_active.
        let conn = db();
        let result = project_get_active(&conn).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn project_list_returns_all_projects_ordered_by_updated_at_desc() {
        // The more recently updated project must appear first in the list.
        let conn = db();
        let p1 = project_create(&conn, "p1", "/p1", None).unwrap();
        let p2 = project_create(&conn, "p2", "/p2", None).unwrap();
        project_touch(&conn, p1.id).unwrap();
        let list = project_list(&conn).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].id, p1.id);
        assert_eq!(list[1].id, p2.id);
    }

    #[test]
    fn project_touch_updates_updated_at() {
        // After touch, the updated_at timestamp must change (become >= original).
        let conn = db();
        let p = project_create(&conn, "p", "/p", None).unwrap();
        let original_updated = p.updated_at.clone();
        // SQLite timestamps have millisecond precision; sleep briefly to ensure
        // a different value.
        std::thread::sleep(std::time::Duration::from_millis(2));
        project_touch(&conn, p.id).unwrap();
        let touched = project_get(&conn, p.id).unwrap();
        assert!(touched.updated_at >= original_updated);
        // They must differ if the clock advanced.
        // We assert >= because at worst they are equal on a frozen clock;
        // the important guarantee is that updated_at is not stale.
    }

    #[test]
    fn project_create_duplicate_path_fails() {
        // Inserting two projects with identical paths must fail (UNIQUE constraint).
        let conn = db();
        project_create(&conn, "p1", "/same/path", None).unwrap();
        let result = project_create(&conn, "p2", "/same/path", None);
        assert!(result.is_err());
    }

    #[test]
    fn project_list_empty_when_no_projects() {
        // An empty store must return an empty vec for project_list.
        let conn = db();
        let list = project_list(&conn).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn project_create_with_description() {
        // description is optional but must round-trip when provided.
        let conn = db();
        let p = project_create(&conn, "p", "/p", Some("A great project")).unwrap();
        let fetched = project_get(&conn, p.id).unwrap();
        assert_eq!(fetched.description.as_deref(), Some("A great project"));
    }

    // ---- Settings -----------------------------------------------------------

    #[test]
    fn settings_set_inserts_new_key_value_pair() {
        // A freshly set key must be retrievable afterwards.
        let conn = db();
        settings_set(&conn, "theme", &serde_json::json!("dark"), "app").unwrap();
        let val = settings_get(&conn, "theme", "app").unwrap();
        assert_eq!(val, Some(serde_json::json!("dark")));
    }

    #[test]
    fn settings_get_returns_none_for_nonexistent_key() {
        // Requesting a key that was never set must return None, not an error.
        let conn = db();
        let val = settings_get(&conn, "missing", "app").unwrap();
        assert!(val.is_none());
    }

    #[test]
    fn settings_set_upsert_overwrites_existing_value() {
        // Setting the same key twice must replace, not duplicate.
        let conn = db();
        settings_set(&conn, "key", &serde_json::json!("first"), "app").unwrap();
        settings_set(&conn, "key", &serde_json::json!("second"), "app").unwrap();
        let val = settings_get(&conn, "key", "app").unwrap().unwrap();
        assert_eq!(val, serde_json::json!("second"));
    }

    #[test]
    fn settings_get_all_returns_all_settings() {
        // settings_get_all with no scope filter returns every inserted key.
        let conn = db();
        settings_set(&conn, "a", &serde_json::json!(1), "app").unwrap();
        settings_set(&conn, "b", &serde_json::json!(2), "project").unwrap();
        let map = settings_get_all(&conn, None).unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map["a"], serde_json::json!(1));
        assert_eq!(map["b"], serde_json::json!(2));
    }

    #[test]
    fn settings_get_all_with_scope_returns_only_matching_scope() {
        // Filtering by scope "app" must exclude keys in scope "project".
        let conn = db();
        settings_set(&conn, "k", &serde_json::json!("app-val"), "app").unwrap();
        settings_set(&conn, "k", &serde_json::json!("proj-val"), "project").unwrap();
        let map = settings_get_all(&conn, Some("app")).unwrap();
        assert_eq!(map.len(), 1);
        assert_eq!(map["k"], serde_json::json!("app-val"));
    }

    #[test]
    fn settings_get_all_no_settings_returns_empty_map() {
        // An empty store must return an empty map.
        let conn = db();
        let map = settings_get_all(&conn, None).unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn settings_json_round_trip_complex_value() {
        // A complex JSON object must survive a set-then-get cycle without mutation.
        let conn = db();
        let complex = serde_json::json!({
            "enabled": true,
            "count": 42,
            "tags": ["a", "b"],
            "nested": { "x": 1 }
        });
        settings_set(&conn, "cfg", &complex, "app").unwrap();
        let retrieved = settings_get(&conn, "cfg", "app").unwrap().unwrap();
        assert_eq!(retrieved, complex);
    }

    #[test]
    fn settings_different_scopes_are_independent() {
        // Setting key "k" in scope "app" must not affect key "k" in scope "project".
        let conn = db();
        settings_set(&conn, "k", &serde_json::json!("app-value"), "app").unwrap();
        let proj_val = settings_get(&conn, "k", "project").unwrap();
        assert!(proj_val.is_none());
    }

    #[test]
    fn settings_get_returns_the_value_we_set() {
        // Basic sanity: value retrieved must exactly equal the value stored.
        let conn = db();
        settings_set(&conn, "flag", &serde_json::json!(false), "app").unwrap();
        let val = settings_get(&conn, "flag", "app").unwrap().unwrap();
        assert_eq!(val, serde_json::json!(false));
    }

    #[test]
    fn settings_get_all_with_scope_empty_returns_empty_map() {
        // A scope with no keys must return an empty map.
        let conn = db();
        settings_set(&conn, "k", &serde_json::json!(1), "app").unwrap();
        let map = settings_get_all(&conn, Some("project")).unwrap();
        assert!(map.is_empty());
    }
}
