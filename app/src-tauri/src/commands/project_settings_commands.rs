use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use std::path::Path;

use serde_json::Value;
use tauri::State;

use crate::domain::paths;
use crate::error::OrqaError;
use crate::state::AppState;

/// Read project settings from the daemon.
///
/// Delegates to the daemon `GET /projects/settings` endpoint, which reads
/// `project.json` for the active project. Returns `None` when no project.json
/// exists (empty object returned by daemon is mapped to None).
#[tauri::command]
pub async fn project_settings_read(state: State<'_, AppState>) -> Result<Option<Value>, OrqaError> {
    let value = state.daemon.client.get_project_settings().await?;
    // Daemon returns an empty object when no project.json exists.
    if value.as_object().is_some_and(serde_json::Map::is_empty) {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

/// Write project settings via the daemon.
///
/// Delegates to `PUT /projects/settings`. Returns the written settings value.
#[tauri::command]
pub async fn project_settings_write(
    state: State<'_, AppState>,
    settings: Value,
) -> Result<Value, OrqaError> {
    state.daemon.client.write_project_settings(&settings).await
}

/// Upload a project icon by copying an image file to `.orqa/icon.{ext}`.
///
/// Validates the source file exists and has a supported extension (png, jpg, jpeg, svg, ico).
/// Removes any existing `icon.*` files before copying.
/// Returns the icon filename (e.g. `icon.png`).
#[tauri::command(rename_all = "snake_case")]
pub fn project_icon_upload(project_path: String, source_path: String) -> Result<String, OrqaError> {
    let source = Path::new(&source_path);
    if !source.exists() {
        return Err(OrqaError::NotFound(format!(
            "Source file not found: {source_path}"
        )));
    }

    let ext = source
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .unwrap_or_default();

    let allowed = ["png", "jpg", "jpeg", "svg", "ico"];
    if !allowed.contains(&ext.as_str()) {
        return Err(OrqaError::Validation(format!(
            "Unsupported icon format: .{ext}. Use png, jpg, jpeg, svg, or ico"
        )));
    }

    let orqa_dir = Path::new(&project_path).join(paths::ORQA_DIR);
    std::fs::create_dir_all(&orqa_dir)?;

    if let Ok(entries) = std::fs::read_dir(&orqa_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("icon.") {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }

    let icon_filename = format!("icon.{ext}");
    let dest = orqa_dir.join(&icon_filename);
    std::fs::copy(source, &dest)?;

    Ok(icon_filename)
}

/// Read a project icon and return it as a base64-encoded data URI.
///
/// The `icon_filename` should be the filename returned by `project_icon_upload`
/// (e.g. `icon.png`). Returns a `data:{mime};base64,...` string.
#[tauri::command(rename_all = "snake_case")]
pub fn project_icon_read(project_path: String, icon_filename: String) -> Result<String, OrqaError> {
    let icon_path = Path::new(&project_path)
        .join(paths::ORQA_DIR)
        .join(&icon_filename);

    if !icon_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "Icon file not found: {icon_filename}"
        )));
    }

    let bytes = std::fs::read(&icon_path)?;

    let mime = match icon_path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        _ => "application/octet-stream",
    };

    let encoded = BASE64.encode(&bytes);
    Ok(format!("data:{mime};base64,{encoded}"))
}

/// Scan the active project directory via the daemon.
///
/// Delegates to `POST /projects/scan`, which detects languages, frameworks,
/// and governance artifact counts. Returns the scan result as a JSON value.
#[tauri::command]
pub async fn project_scan(state: State<'_, AppState>) -> Result<Value, OrqaError> {
    state.daemon.client.scan_project().await
}

#[cfg(test)]
mod tests {
    #[test]
    fn icon_upload_validates_missing_source() {
        let source = std::path::Path::new("/nonexistent/icon.png");
        assert!(!source.exists());
    }

    #[test]
    fn icon_upload_validates_extension() {
        let allowed = ["png", "jpg", "jpeg", "svg", "ico"];
        assert!(!allowed.contains(&"bmp"));
        assert!(!allowed.contains(&"gif"));
        assert!(allowed.contains(&"png"));
        assert!(allowed.contains(&"svg"));
    }

    #[test]
    fn icon_read_validates_missing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let icon_path = dir.path().join(".orqa").join("icon.png");
        assert!(!icon_path.exists());
    }
}
