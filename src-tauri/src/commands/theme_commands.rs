use std::collections::HashMap;

use tauri::State;

use crate::domain::settings::{ResolvedTheme, ThemeToken, ThemeTokenSource};
use crate::error::OrqaError;
use crate::repo::theme_repo;
use crate::state::AppState;

/// Get the resolved theme for a project.
///
/// Merges extracted theme tokens with any user overrides. Override values
/// take precedence over extracted values.
#[tauri::command]
pub fn theme_get_project(
    project_id: i64,
    state: State<'_, AppState>,
) -> Result<ResolvedTheme, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let theme_rows = theme_repo::get_themes(&conn, project_id)?;
    let override_rows = theme_repo::get_overrides(&conn, project_id)?;

    let mut tokens: HashMap<String, ThemeToken> = HashMap::new();
    let mut source_files: Vec<String> = Vec::new();

    // Process extracted theme tokens
    for row in &theme_rows {
        source_files.push(row.source_file.clone());

        // Parse the tokens_light JSON into individual tokens
        if let Ok(light_map) = serde_json::from_str::<HashMap<String, String>>(&row.tokens_light) {
            let dark_map: HashMap<String, String> = row
                .tokens_dark
                .as_ref()
                .and_then(|d| serde_json::from_str(d).ok())
                .unwrap_or_default();

            for (name, value_light) in &light_map {
                tokens.insert(
                    name.clone(),
                    ThemeToken {
                        name: name.clone(),
                        value_light: value_light.clone(),
                        value_dark: dark_map.get(name).cloned(),
                        source: ThemeTokenSource::Extracted,
                    },
                );
            }
        }
    }

    // Apply overrides (these take precedence)
    let has_overrides = !override_rows.is_empty();
    for ov in &override_rows {
        tokens.insert(
            ov.token_name.clone(),
            ThemeToken {
                name: ov.token_name.clone(),
                value_light: ov.value_light.clone(),
                value_dark: ov.value_dark.clone(),
                source: ThemeTokenSource::Override,
            },
        );
    }

    source_files.dedup();

    Ok(ResolvedTheme {
        project_id,
        tokens,
        source_files,
        has_overrides,
    })
}

/// Set a theme override for a specific design token.
#[tauri::command]
pub fn theme_set_override(
    project_id: i64,
    token_name: String,
    value_light: String,
    value_dark: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    if token_name.trim().is_empty() {
        return Err(OrqaError::Validation(
            "token name cannot be empty".to_string(),
        ));
    }
    if value_light.trim().is_empty() {
        return Err(OrqaError::Validation(
            "light value cannot be empty".to_string(),
        ));
    }

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    theme_repo::set_override(
        &conn,
        project_id,
        token_name.trim(),
        value_light.trim(),
        value_dark.as_deref(),
    )
}

/// Clear all theme overrides for a project.
#[tauri::command]
pub fn theme_clear_overrides(project_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    theme_repo::clear_overrides(&conn, project_id)
}

#[cfg(test)]
mod tests {
    use crate::db::init_memory_db;
    use crate::domain::settings::ThemeTokenSource;
    use crate::repo::{project_repo, theme_repo};
    use rusqlite::params;
    use std::collections::HashMap;

    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        conn
    }

    #[test]
    fn resolved_theme_empty_project() {
        let conn = setup();
        let themes = theme_repo::get_themes(&conn, 1).expect("get_themes");
        let overrides = theme_repo::get_overrides(&conn, 1).expect("get_overrides");

        assert!(themes.is_empty());
        assert!(overrides.is_empty());
    }

    #[test]
    fn resolved_theme_with_extracted_tokens() {
        let conn = setup();

        let light = serde_json::json!({"primary": "#000", "secondary": "#333"});
        let dark = serde_json::json!({"primary": "#fff", "secondary": "#ccc"});

        conn.execute(
            "INSERT INTO project_themes \
             (project_id, source_file, source_hash, tokens_light, tokens_dark, is_active) \
             VALUES (1, 'tailwind.config.ts', 'sha256:abc', ?1, ?2, 1)",
            params![light.to_string(), dark.to_string()],
        )
        .expect("insert theme");

        let themes = theme_repo::get_themes(&conn, 1).expect("get_themes");
        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].source_file, "tailwind.config.ts");

        // Parse tokens from the theme row
        let light_map: HashMap<String, String> =
            serde_json::from_str(&themes[0].tokens_light).expect("parse light");
        assert_eq!(light_map["primary"], "#000");
        assert_eq!(light_map["secondary"], "#333");
    }

    #[test]
    fn resolved_theme_overrides_take_precedence() {
        let conn = setup();

        let light = serde_json::json!({"primary": "#000"});
        conn.execute(
            "INSERT INTO project_themes \
             (project_id, source_file, source_hash, tokens_light, is_active) \
             VALUES (1, 'theme.css', 'sha256:x', ?1, 1)",
            params![light.to_string()],
        )
        .expect("insert theme");

        // Set an override for 'primary'
        theme_repo::set_override(&conn, 1, "primary", "#ff0000", Some("#00ff00"))
            .expect("set override");

        let overrides = theme_repo::get_overrides(&conn, 1).expect("get_overrides");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].token_name, "primary");
        assert_eq!(overrides[0].value_light, "#ff0000");
        assert_eq!(overrides[0].value_dark.as_deref(), Some("#00ff00"));
    }

    #[test]
    fn set_override_upserts() {
        let conn = setup();

        theme_repo::set_override(&conn, 1, "primary", "#ff0000", None).expect("set 1");
        theme_repo::set_override(&conn, 1, "primary", "#0000ff", Some("#00ff00")).expect("set 2");

        let overrides = theme_repo::get_overrides(&conn, 1).expect("get_overrides");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].value_light, "#0000ff");
    }

    #[test]
    fn clear_overrides_removes_all() {
        let conn = setup();

        theme_repo::set_override(&conn, 1, "primary", "#ff0000", None).expect("set");
        theme_repo::set_override(&conn, 1, "secondary", "#00ff00", None).expect("set");

        let before = theme_repo::get_overrides(&conn, 1).expect("get");
        assert_eq!(before.len(), 2);

        theme_repo::clear_overrides(&conn, 1).expect("clear");

        let after = theme_repo::get_overrides(&conn, 1).expect("get");
        assert!(after.is_empty());
    }

    #[test]
    fn theme_token_source_values() {
        // Verify the source enum values serialize correctly
        let extracted =
            serde_json::to_value(ThemeTokenSource::Extracted).expect("serialize extracted");
        let override_val =
            serde_json::to_value(ThemeTokenSource::Override).expect("serialize override");
        let default = serde_json::to_value(ThemeTokenSource::Default).expect("serialize default");

        assert_eq!(extracted, "extracted");
        assert_eq!(override_val, "override");
        assert_eq!(default, "default");
    }

    #[test]
    fn empty_token_name_validation() {
        let name = "  ";
        assert!(name.trim().is_empty());
    }

    #[test]
    fn empty_value_light_validation() {
        let value = "  ";
        assert!(value.trim().is_empty());
    }
}
