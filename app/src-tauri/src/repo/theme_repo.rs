use rusqlite::{params, Connection};

use crate::error::OrqaError;

/// A raw theme row from the `project_themes` table.
#[derive(Debug, Clone)]
pub struct ThemeRow {
    pub id: i64,
    pub project_id: i64,
    pub source_file: String,
    pub source_hash: String,
    pub extracted_at: String,
    pub tokens_light: String,
    pub tokens_dark: Option<String>,
    pub unmapped: Option<String>,
    pub is_active: bool,
}

/// A raw theme override row from the `project_theme_overrides` table.
#[derive(Debug, Clone)]
pub struct ThemeOverrideRow {
    pub id: i64,
    pub project_id: i64,
    pub token_name: String,
    pub value_light: String,
    pub value_dark: Option<String>,
}

/// Get all active themes for a project.
pub fn get_themes(conn: &Connection, project_id: i64) -> Result<Vec<ThemeRow>, OrqaError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, source_file, source_hash, extracted_at, \
                tokens_light, tokens_dark, unmapped, is_active \
         FROM project_themes \
         WHERE project_id = ?1 AND is_active = 1 \
         ORDER BY source_file ASC",
    )?;

    let rows = stmt.query_map(params![project_id], |row| {
        let is_active: i32 = row.get(8)?;
        Ok(ThemeRow {
            id: row.get(0)?,
            project_id: row.get(1)?,
            source_file: row.get(2)?,
            source_hash: row.get(3)?,
            extracted_at: row.get(4)?,
            tokens_light: row.get(5)?,
            tokens_dark: row.get(6)?,
            unmapped: row.get(7)?,
            is_active: is_active != 0,
        })
    })?;

    let mut themes = Vec::new();
    for row in rows {
        themes.push(row?);
    }
    Ok(themes)
}

/// Get all overrides for a project.
pub fn get_overrides(
    conn: &Connection,
    project_id: i64,
) -> Result<Vec<ThemeOverrideRow>, OrqaError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, token_name, value_light, value_dark \
         FROM project_theme_overrides \
         WHERE project_id = ?1 \
         ORDER BY token_name ASC",
    )?;

    let rows = stmt.query_map(params![project_id], |row| {
        Ok(ThemeOverrideRow {
            id: row.get(0)?,
            project_id: row.get(1)?,
            token_name: row.get(2)?,
            value_light: row.get(3)?,
            value_dark: row.get(4)?,
        })
    })?;

    let mut overrides = Vec::new();
    for row in rows {
        overrides.push(row?);
    }
    Ok(overrides)
}

/// Set (upsert) a theme override for a specific token.
pub fn set_override(
    conn: &Connection,
    project_id: i64,
    token_name: &str,
    value_light: &str,
    value_dark: Option<&str>,
) -> Result<(), OrqaError> {
    conn.execute(
        "INSERT INTO project_theme_overrides \
         (project_id, token_name, value_light, value_dark, updated_at) \
         VALUES (?1, ?2, ?3, ?4, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
         ON CONFLICT(project_id, token_name) DO UPDATE SET \
         value_light = excluded.value_light, \
         value_dark = excluded.value_dark, \
         updated_at = excluded.updated_at",
        params![project_id, token_name, value_light, value_dark],
    )?;

    Ok(())
}

/// Clear all theme overrides for a project.
pub fn clear_overrides(conn: &Connection, project_id: i64) -> Result<(), OrqaError> {
    conn.execute(
        "DELETE FROM project_theme_overrides WHERE project_id = ?1",
        params![project_id],
    )?;

    Ok(())
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
    fn empty_themes() {
        let conn = setup();
        let themes = get_themes(&conn, 1).expect("get_themes");
        assert!(themes.is_empty());
    }

    #[test]
    fn insert_and_get_theme() {
        let conn = setup();

        conn.execute(
            "INSERT INTO project_themes \
             (project_id, source_file, source_hash, tokens_light, tokens_dark, is_active) \
             VALUES (1, 'tailwind.config.ts', 'sha256:abc', '{\"primary\": \"#000\"}', \
                     '{\"primary\": \"#fff\"}', 1)",
            [],
        )
        .expect("insert theme");

        let themes = get_themes(&conn, 1).expect("get_themes");
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].source_file, "tailwind.config.ts");
        assert!(themes[0].is_active);
        assert!(themes[0].tokens_dark.is_some());
    }

    #[test]
    fn inactive_themes_excluded() {
        let conn = setup();

        conn.execute(
            "INSERT INTO project_themes \
             (project_id, source_file, source_hash, tokens_light, is_active) \
             VALUES (1, 'old.css', 'sha256:old', '{}', 0)",
            [],
        )
        .expect("insert inactive theme");

        conn.execute(
            "INSERT INTO project_themes \
             (project_id, source_file, source_hash, tokens_light, is_active) \
             VALUES (1, 'current.css', 'sha256:new', '{}', 1)",
            [],
        )
        .expect("insert active theme");

        let themes = get_themes(&conn, 1).expect("get_themes");
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].source_file, "current.css");
    }

    #[test]
    fn set_and_get_override() {
        let conn = setup();

        set_override(&conn, 1, "primary", "#ff0000", Some("#00ff00")).expect("set override");

        let overrides = get_overrides(&conn, 1).expect("get_overrides");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].token_name, "primary");
        assert_eq!(overrides[0].value_light, "#ff0000");
        assert_eq!(overrides[0].value_dark.as_deref(), Some("#00ff00"));
    }

    #[test]
    fn set_override_upserts() {
        let conn = setup();

        set_override(&conn, 1, "primary", "#ff0000", None).expect("first set");
        set_override(&conn, 1, "primary", "#0000ff", Some("#00ff00")).expect("second set");

        let overrides = get_overrides(&conn, 1).expect("get_overrides");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].value_light, "#0000ff");
        assert_eq!(overrides[0].value_dark.as_deref(), Some("#00ff00"));
    }

    #[test]
    fn clear_overrides_works() {
        let conn = setup();

        set_override(&conn, 1, "primary", "#ff0000", None).expect("set");
        set_override(&conn, 1, "secondary", "#00ff00", None).expect("set");

        let before = get_overrides(&conn, 1).expect("get");
        assert_eq!(before.len(), 2);

        clear_overrides(&conn, 1).expect("clear");

        let after = get_overrides(&conn, 1).expect("get");
        assert!(after.is_empty());
    }

    #[test]
    fn overrides_scoped_by_project() {
        let conn = setup();
        project_repo::create(&conn, "other", "/other", None).expect("create project 2");

        set_override(&conn, 1, "primary", "#ff0000", None).expect("set p1");
        set_override(&conn, 2, "primary", "#00ff00", None).expect("set p2");

        let p1 = get_overrides(&conn, 1).expect("get p1");
        let p2 = get_overrides(&conn, 2).expect("get p2");

        assert_eq!(p1.len(), 1);
        assert_eq!(p2.len(), 1);
        assert_eq!(p1[0].value_light, "#ff0000");
        assert_eq!(p2[0].value_light, "#00ff00");
    }

    #[test]
    fn cascade_on_project_delete() {
        let conn = setup();

        conn.execute(
            "INSERT INTO project_themes \
             (project_id, source_file, source_hash, tokens_light, is_active) \
             VALUES (1, 'theme.css', 'sha256:x', '{}', 1)",
            [],
        )
        .expect("insert theme");
        set_override(&conn, 1, "primary", "#000", None).expect("set override");

        conn.execute("DELETE FROM projects WHERE id = 1", [])
            .expect("delete project");

        let themes = get_themes(&conn, 1).expect("get_themes");
        assert!(themes.is_empty(), "themes should cascade");

        let overrides = get_overrides(&conn, 1).expect("get_overrides");
        assert!(overrides.is_empty(), "overrides should cascade");
    }
}
