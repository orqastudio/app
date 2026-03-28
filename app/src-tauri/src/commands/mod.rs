/// Tauri IPC command: artifact tree scanning and file watching.
pub mod artifact_commands;
/// Tauri IPC commands: CLI tool listing, execution, and status.
pub mod cli_tool_commands;
/// Tauri IPC command: daemon health check.
pub mod daemon_commands;
/// Tauri IPC commands: enforcement rule listing, reload, and violation queries.
pub mod enforcement_commands;
/// Tauri IPC commands: artifact graph queries, integrity scans, health snapshots.
pub mod graph_commands;
/// Shared command helper utilities (e.g., path validation).
pub mod helpers;
/// Tauri IPC commands: plugin hook registry and dispatcher generation.
pub mod hook_commands;
/// Tauri IPC commands: lesson CRUD.
pub mod lesson_commands;
/// Tauri IPC commands: conversation message listing.
pub mod message_commands;
/// Tauri IPC commands: plugin install, list, uninstall, and update checks.
pub mod plugin_commands;
/// Tauri IPC commands: project open, active project retrieval, and project listing.
pub mod project_commands;
/// Tauri IPC commands: project settings read/write, icon upload/read, and project scan.
pub mod project_settings_commands;
/// Tauri IPC command: startup task status query.
pub mod search_commands;
/// Tauri IPC commands: session CRUD and title generation.
pub mod session_commands;
/// Tauri IPC commands: user settings get and set.
pub mod settings_commands;
/// Tauri IPC commands: setup wizard status, CLI auth checks, and model availability.
pub mod setup_commands;
/// Tauri IPC commands: sidecar status and restart.
pub mod sidecar_commands;
/// Tauri IPC commands: artifact status transition evaluation and application.
pub mod status_transition_commands;
/// Tauri IPC commands: streaming message send, stop, and tool approval.
pub mod stream_commands;
