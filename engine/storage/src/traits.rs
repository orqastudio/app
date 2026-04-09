// Repository trait definitions for orqa-storage.
//
// Each trait defines the async contract for one storage domain. Signatures use
// only domain types from orqa-engine-types — no rusqlite or SeaORM types appear
// in any trait boundary. This makes implementations swappable: SQLite, Postgres,
// and in-memory mocks can all satisfy the same interface.
//
// Design principles applied here:
// - Pure: every method is a function of its arguments, no hidden state
// - Async: all I/O returns a Future; callers control the executor
// - Domain-typed: inputs and outputs are engine types, not ORM rows
// - Composable: traits are independent; a type may implement any subset

use std::collections::HashMap;

use orqa_engine_types::types::enforcement::EnforcementViolation;
use orqa_engine_types::types::health::{HealthSnapshot, NewHealthSnapshot};
use orqa_engine_types::types::message::{Message, MessageRole, SearchResult, StreamStatus};
use orqa_engine_types::types::project::{DetectedStack, Project, ProjectSummary};
use orqa_engine_types::types::session::{Session, SessionStatus, SessionSummary};

use crate::error::StorageError;
use crate::repo::devtools::{
    DevtoolsEventQuery, DevtoolsEventQueryResponse, DevtoolsSessionInfo, DevtoolsSessionSummary,
};
use crate::repo::events::EventFilter;
use crate::repo::issue_groups::{IssueGroup, SortBy, SortDir};
use crate::repo::messages::NewToolMessage;
use crate::repo::themes::{ThemeOverrideRow, ThemeRow};

/// Async repository contract for the `projects` domain.
///
/// All operations are stateless: the repository receives a reference to itself
/// and returns owned domain values. No connection state is threaded through.
#[async_trait::async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Create a new project record and return the full row.
    async fn create(
        &self,
        name: &str,
        path: &str,
        description: Option<&str>,
    ) -> Result<Project, StorageError>;

    /// Get a project by its primary key.
    async fn get(&self, id: i64) -> Result<Project, StorageError>;

    /// Get a project by its filesystem path.
    async fn get_by_path(&self, path: &str) -> Result<Project, StorageError>;

    /// Get the most recently updated project, or `None` if no projects exist.
    async fn get_active(&self) -> Result<Option<Project>, StorageError>;

    /// List all projects with summary info (session count, artifact count).
    async fn list(&self) -> Result<Vec<ProjectSummary>, StorageError>;

    /// Touch the `updated_at` timestamp, surfacing the project as most recently active.
    async fn touch_updated_at(&self, id: i64) -> Result<(), StorageError>;

    /// Update the detected technology stack (stored as JSON).
    async fn update_detected_stack(
        &self,
        id: i64,
        stack: &DetectedStack,
    ) -> Result<(), StorageError>;
}

/// Async repository contract for the `sessions` domain.
#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    /// Create a new session and return the full row.
    async fn create(
        &self,
        project_id: i64,
        model: &str,
        system_prompt: Option<&str>,
    ) -> Result<Session, StorageError>;

    /// Get a session by its primary key.
    async fn get(&self, id: i64) -> Result<Session, StorageError>;

    /// List sessions for a project with optional status filter and pagination.
    async fn list(
        &self,
        project_id: i64,
        status_filter: Option<SessionStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SessionSummary>, StorageError>;

    /// List all sessions across all projects with optional status filter.
    ///
    /// Applies an implicit cap of 1000 rows to prevent unbounded result sets.
    async fn list_all(
        &self,
        status_filter: Option<SessionStatus>,
    ) -> Result<Vec<SessionSummary>, StorageError>;

    /// Update a session's status.
    async fn update_status(&self, id: i64, status: SessionStatus) -> Result<(), StorageError>;

    /// Return the next turn index for a session (max existing + 1, or 0).
    async fn next_turn_index(&self, session_id: i64) -> Result<i32, StorageError>;

    /// Update the session title and mark it as manually set.
    async fn update_title(&self, id: i64, title: &str) -> Result<(), StorageError>;

    /// Auto-update session title only if not manually set.
    ///
    /// Returns `true` if updated, `false` if skipped.
    async fn auto_update_title(&self, id: i64, title: &str) -> Result<bool, StorageError>;

    /// Mark a session as completed.
    async fn end_session(&self, id: i64) -> Result<(), StorageError>;

    /// Delete a session and its messages (cascade).
    async fn delete(&self, id: i64) -> Result<(), StorageError>;

    /// Increment token usage counters for a session (additive).
    async fn update_token_usage(
        &self,
        id: i64,
        input_tokens: i64,
        output_tokens: i64,
    ) -> Result<(), StorageError>;

    /// Store the provider session ID for context continuity across restarts.
    async fn update_provider_session_id(
        &self,
        id: i64,
        provider_session_id: &str,
    ) -> Result<(), StorageError>;
}

/// Async repository contract for the `messages` domain.
#[async_trait::async_trait]
pub trait MessageRepository: Send + Sync {
    /// Create a standard (non-tool) message and return the full row.
    async fn create(
        &self,
        session_id: i64,
        role: MessageRole,
        content: Option<&str>,
        turn_index: i32,
        block_index: i32,
    ) -> Result<Message, StorageError>;

    /// Create a tool-related message (tool_use or tool_result).
    async fn create_tool_message(&self, msg: &NewToolMessage<'_>) -> Result<Message, StorageError>;

    /// List messages for a session ordered by turn and block index.
    async fn list(
        &self,
        session_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, StorageError>;

    /// Search messages across a project using FTS5 full-text search.
    async fn search(
        &self,
        project_id: i64,
        query: &str,
        limit: i64,
    ) -> Result<Vec<SearchResult>, StorageError>;

    /// Return the next turn index for a session.
    async fn next_turn_index(&self, session_id: i64) -> Result<i32, StorageError>;

    /// Update the content of a message (streaming accumulation).
    async fn update_content(&self, id: i64, content: &str) -> Result<(), StorageError>;

    /// Update the stream status of a message.
    async fn update_stream_status(&self, id: i64, status: StreamStatus)
        -> Result<(), StorageError>;
}

/// Async repository contract for the `settings` domain.
#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    /// Get a single setting value by key and scope.
    ///
    /// Returns `None` if the key does not exist in the given scope.
    async fn get(&self, key: &str, scope: &str) -> Result<Option<serde_json::Value>, StorageError>;

    /// Set a setting value (upsert by key + scope).
    async fn set(
        &self,
        key: &str,
        value: &serde_json::Value,
        scope: &str,
    ) -> Result<(), StorageError>;

    /// Get all settings regardless of scope, ordered by key.
    async fn get_all_any_scope(&self) -> Result<HashMap<String, serde_json::Value>, StorageError>;

    /// Get all settings for a given scope, ordered by key.
    async fn get_all(
        &self,
        scope: &str,
    ) -> Result<HashMap<String, serde_json::Value>, StorageError>;
}

/// Async repository contract for the `project_themes` and `project_theme_overrides` domains.
#[async_trait::async_trait]
pub trait ThemeRepository: Send + Sync {
    /// Get all active themes for a project, ordered by source file path.
    async fn get_themes(&self, project_id: i64) -> Result<Vec<ThemeRow>, StorageError>;

    /// Get all theme overrides for a project, ordered by token name.
    async fn get_overrides(&self, project_id: i64) -> Result<Vec<ThemeOverrideRow>, StorageError>;

    /// Set (upsert) a theme override for a specific token.
    async fn set_override(
        &self,
        project_id: i64,
        token_name: &str,
        value_light: &str,
        value_dark: Option<&str>,
    ) -> Result<(), StorageError>;

    /// Clear all theme overrides for a project.
    async fn clear_overrides(&self, project_id: i64) -> Result<(), StorageError>;
}

/// Async repository contract for the `enforcement_violations` domain.
#[async_trait::async_trait]
pub trait ViolationRepository: Send + Sync {
    /// Record a new enforcement violation.
    async fn record(
        &self,
        project_id: i64,
        rule_name: &str,
        action: &str,
        tool_name: &str,
        detail: Option<&str>,
    ) -> Result<(), StorageError>;

    /// Query enforcement violation history for a project, most recent first.
    ///
    /// `limit` caps the result count; pass `None` for all rows.
    async fn list_for_project(
        &self,
        project_id: i64,
        limit: Option<u32>,
    ) -> Result<Vec<EnforcementViolation>, StorageError>;
}

/// Async repository contract for the `health_snapshots` domain.
#[async_trait::async_trait]
pub trait HealthRepository: Send + Sync {
    /// Store a new health snapshot for a project and return the inserted row.
    async fn create(
        &self,
        project_id: i64,
        snapshot: &NewHealthSnapshot,
    ) -> Result<HealthSnapshot, StorageError>;

    /// Get a single snapshot by its ID.
    async fn get(&self, id: i64) -> Result<HealthSnapshot, StorageError>;

    /// Get the most recent N snapshots for a project, ordered newest first.
    async fn get_recent(
        &self,
        project_id: i64,
        limit: i64,
    ) -> Result<Vec<HealthSnapshot>, StorageError>;
}

/// Async repository contract for the `log_events` domain.
#[async_trait::async_trait]
pub trait EventRepository: Send + Sync {
    /// Insert a batch of events in a single transaction.
    ///
    /// Duplicate IDs are silently ignored.
    async fn insert_batch(
        &self,
        events: Vec<orqa_engine_types::types::event::LogEvent>,
    ) -> Result<(), StorageError>;

    /// Return stored events matching the filter, ordered by timestamp ascending.
    async fn query(&self, filter: &EventFilter) -> Result<Vec<serde_json::Value>, StorageError>;

    /// Delete all events older than the given retention window.
    ///
    /// Returns the number of rows deleted.
    async fn purge(&self, retention_days: u32) -> Result<usize, StorageError>;
}

/// Async repository contract for the `devtools_sessions` and `devtools_events` domains.
#[async_trait::async_trait]
pub trait DevtoolsRepository: Send + Sync {
    /// Create a new devtools session with the given UUID and start timestamp.
    async fn create_session(&self, session_id: &str, started_at: i64) -> Result<(), StorageError>;

    /// Mark all sessions with `ended_at IS NULL` as interrupted.
    async fn mark_orphaned_sessions_interrupted(&self) -> Result<(), StorageError>;

    /// Mark a session as ended.
    async fn end_session(&self, session_id: &str, ended_at: i64) -> Result<(), StorageError>;

    /// Insert a batch of events for the given session in a single transaction.
    async fn insert_events(
        &self,
        session_id: &str,
        events: Vec<orqa_engine_types::types::event::LogEvent>,
    ) -> Result<(), StorageError>;

    /// List all devtools sessions ordered by `started_at DESC`.
    ///
    /// `current_session_id` is used to set the `is_current` flag on the matching row.
    async fn list_sessions(
        &self,
        current_session_id: &str,
    ) -> Result<Vec<DevtoolsSessionSummary>, StorageError>;

    /// Get metadata for a specific session by ID.
    async fn get_session(&self, session_id: &str) -> Result<DevtoolsSessionInfo, StorageError>;

    /// Update the user-editable label for a session.
    async fn rename_session(&self, session_id: &str, label: &str) -> Result<(), StorageError>;

    /// Delete a session and cascade its events.
    async fn delete_session(&self, session_id: &str) -> Result<(), StorageError>;

    /// Delete sessions older than the given retention window.
    ///
    /// Returns the number of sessions deleted.
    async fn purge_old_sessions(&self, retention_days: u32) -> Result<usize, StorageError>;

    /// Return paginated and filtered events for a session.
    async fn query_events(
        &self,
        query: &DevtoolsEventQuery,
    ) -> Result<DevtoolsEventQueryResponse, StorageError>;
}

/// Async repository contract for the `issue_groups` domain.
#[async_trait::async_trait]
pub trait IssueGroupRepository: Send + Sync {
    /// Insert or update an issue group for the given fingerprint.
    ///
    /// On first occurrence: creates a new row with count=1 and initializes the
    /// sparkline. On subsequent occurrences: increments count, rotates sparkline,
    /// and maintains the recent-event-IDs ring buffer.
    async fn upsert(
        &self,
        fingerprint: &str,
        title: &str,
        component: &str,
        level: &str,
        timestamp_ms: i64,
        event_id: u64,
    ) -> Result<(), StorageError>;

    /// List issue groups with optional filtering and sorting.
    async fn list(
        &self,
        sort_by: SortBy,
        sort_dir: SortDir,
        filter_component: Option<&str>,
        filter_level: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<IssueGroup>, StorageError>;

    /// Return a single issue group by fingerprint, or `None` if it does not exist.
    async fn get(&self, fingerprint: &str) -> Result<Option<IssueGroup>, StorageError>;
}
