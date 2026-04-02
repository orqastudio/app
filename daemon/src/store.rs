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
