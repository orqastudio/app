use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use std::path::Path;

use crate::domain::project_scanner::{self, ProjectScanResult};
use crate::domain::project_settings::{self, ProjectSettings};
use crate::error::OrqaError;

/// Read project settings from the `.orqa/project.json` file.
///
/// Returns `None` if the settings file does not exist yet.
#[tauri::command]
pub fn project_settings_read(path: String) -> Result<Option<ProjectSettings>, OrqaError> {
    project_settings::read_settings(&path)
}

/// Write project settings to the `.orqa/project.json` file.
///
/// Creates the `.orqa/` directory if it does not exist.
/// Returns the written settings for confirmation.
#[tauri::command]
pub fn project_settings_write(
    path: String,
    settings: ProjectSettings,
) -> Result<ProjectSettings, OrqaError> {
    project_settings::write_settings(&path, &settings)?;
    Ok(settings)
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
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    let allowed = ["png", "jpg", "jpeg", "svg", "ico"];
    if !allowed.contains(&ext.as_str()) {
        return Err(OrqaError::Validation(format!(
            "Unsupported icon format: .{ext}. Use png, jpg, jpeg, svg, or ico"
        )));
    }

    let orqa_dir = Path::new(&project_path).join(".orqa");
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
    let icon_path = Path::new(&project_path).join(".orqa").join(&icon_filename);

    if !icon_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "Icon file not found: {icon_filename}"
        )));
    }

    let bytes = std::fs::read(&icon_path)?;

    let mime = match icon_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
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

/// Scan a project directory for language, framework, and governance info.
///
/// Uses the provided excluded paths or falls back to standard defaults.
#[tauri::command]
pub fn project_scan(
    path: String,
    excluded_paths: Option<Vec<String>>,
) -> Result<ProjectScanResult, OrqaError> {
    let defaults = vec![
        "node_modules".to_string(),
        ".git".to_string(),
        "target".to_string(),
        "dist".to_string(),
        "build".to_string(),
    ];
    let paths = excluded_paths.unwrap_or(defaults);
    project_scanner::scan_project(&path, &paths)
}
