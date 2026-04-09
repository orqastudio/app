// SeaORM entity module for orqa-storage.
//
// Re-exports all entity definitions so callers can use `entity::projects::Entity`,
// `entity::sessions::Column`, etc. One entity file per database table.

/// Devtools events entity (devtools_events table).
pub mod devtools_events;
/// Devtools sessions entity (devtools_sessions table).
pub mod devtools_sessions;
/// Enforcement violations entity (enforcement_violations table).
pub mod enforcement_violations;
/// Health snapshots entity (health_snapshots table).
pub mod health_snapshots;
/// Issue groups entity (issue_groups table).
pub mod issue_groups;
/// Log events entity (log_events table).
pub mod log_events;
/// Messages entity (messages table) with role, content_type, and stream_status enums.
pub mod messages;
/// Project theme overrides entity (project_theme_overrides table).
pub mod project_theme_overrides;
/// Project themes entity (project_themes table).
pub mod project_themes;
/// Projects entity (projects table).
pub mod projects;
/// Sessions entity (sessions table) with status enum.
pub mod sessions;
/// Settings entity (settings table) with composite primary key on (key, scope).
pub mod settings;
