use std::collections::HashMap;

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::OrqaError;

/// Get a single setting value by key and scope.
pub fn get(
    conn: &Connection,
    key: &str,
    scope: &str,
) -> Result<Option<serde_json::Value>, OrqaError> {
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = ?1 AND scope = ?2",
            params![key, scope],
            |row| row.get(0),
        )
        .optional()?;

    match value {
        Some(v) => {
            let parsed = serde_json::from_str(&v)?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

/// Set a setting value (upsert).
pub fn set(
    conn: &Connection,
    key: &str,
    value: &serde_json::Value,
    scope: &str,
) -> Result<(), OrqaError> {
    let value_str = serde_json::to_string(value)?;

    conn.execute(
        "INSERT INTO settings (key, value, scope, updated_at) \
         VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
         ON CONFLICT(key, scope) DO UPDATE SET \
         value = excluded.value, updated_at = excluded.updated_at",
        params![key, value_str, scope],
    )?;

    Ok(())
}

/// Get all settings for a given scope.
pub fn get_all(
    conn: &Connection,
    scope: &str,
) -> Result<HashMap<String, serde_json::Value>, OrqaError> {
    let mut stmt =
        conn.prepare("SELECT key, value FROM settings WHERE scope = ?1 ORDER BY key ASC")?;

    let rows = stmt.query_map(params![scope], |row| {
        let key: String = row.get(0)?;
        let value_str: String = row.get(1)?;
        Ok((key, value_str))
    })?;

    let mut map = HashMap::new();
    for row in rows {
        let (key, value_str) = row?;
        let parsed: serde_json::Value = serde_json::from_str(&value_str)?;
        map.insert(key, parsed);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;

    #[test]
    fn get_nonexistent_returns_none() {
        let conn = init_memory_db().expect("db init");
        let result = get(&conn, "theme", "app").expect("get");
        assert!(result.is_none());
    }

    #[test]
    fn set_and_get_string() {
        let conn = init_memory_db().expect("db init");
        let value = serde_json::json!("dark");

        set(&conn, "theme", &value, "app").expect("set");

        let fetched = get(&conn, "theme", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[test]
    fn set_and_get_object() {
        let conn = init_memory_db().expect("db init");
        let value = serde_json::json!({
            "font_size": 14,
            "line_numbers": true
        });

        set(&conn, "editor", &value, "app").expect("set");

        let fetched = get(&conn, "editor", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched["font_size"], 14);
        assert_eq!(fetched["line_numbers"], true);
    }

    #[test]
    fn set_overwrites_existing() {
        let conn = init_memory_db().expect("db init");

        set(&conn, "theme", &serde_json::json!("light"), "app").expect("set 1");
        set(&conn, "theme", &serde_json::json!("dark"), "app").expect("set 2");

        let fetched = get(&conn, "theme", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[test]
    fn scopes_are_independent() {
        let conn = init_memory_db().expect("db init");

        set(&conn, "theme", &serde_json::json!("dark"), "app").expect("set app");
        set(&conn, "theme", &serde_json::json!("light"), "project:1").expect("set project");

        let app_theme = get(&conn, "theme", "app")
            .expect("get app")
            .expect("should exist");
        let proj_theme = get(&conn, "theme", "project:1")
            .expect("get project")
            .expect("should exist");

        assert_eq!(app_theme, serde_json::json!("dark"));
        assert_eq!(proj_theme, serde_json::json!("light"));
    }

    #[test]
    fn get_all_returns_scope() {
        let conn = init_memory_db().expect("db init");

        set(&conn, "theme", &serde_json::json!("dark"), "app").expect("set");
        set(&conn, "font_size", &serde_json::json!(14), "app").expect("set");
        set(
            &conn,
            "project_theme",
            &serde_json::json!("blue"),
            "project:1",
        )
        .expect("set other scope");

        let all = get_all(&conn, "app").expect("get_all");
        assert_eq!(all.len(), 2);
        assert_eq!(all["theme"], serde_json::json!("dark"));
        assert_eq!(all["font_size"], serde_json::json!(14));
    }

    #[test]
    fn get_all_empty_scope() {
        let conn = init_memory_db().expect("db init");
        let all = get_all(&conn, "nonexistent").expect("get_all");
        assert!(all.is_empty());
    }
}
