// Correlation ID context for the OrqaStudio daemon.
//
// Provides task-local storage for a correlation ID that flows through the
// lifetime of a single HTTP request or sidecar operation. All async work
// spawned within a `with_correlation_id` scope inherits the same ID,
// enabling end-to-end tracing across IPC boundaries without passing the ID
// through every function signature.
//
// The ID format is a UUID v4 hex string truncated to 16 characters (no
// dashes). This keeps the ID compact for headers and log fields while still
// having enough entropy (~64 bits) to avoid collisions within a session.

use uuid::Uuid;

tokio::task_local! {
    /// Task-local correlation ID set for the duration of a single request.
    static CORRELATION_ID: String;
}

/// Run a future with the given correlation ID set in the task-local context.
///
/// All `.await` points within `f` will see the same ID when
/// `current_correlation_id` is called. The scope ends when `f` completes.
pub async fn with_correlation_id<F, T>(id: String, f: F) -> T
where
    F: std::future::Future<Output = T>,
{
    CORRELATION_ID.scope(id, f).await
}

/// Read the current correlation ID from the task-local context.
///
/// Returns `None` when called outside a `with_correlation_id` scope (e.g.
/// background tasks that were not spawned from a request handler).
pub fn current_correlation_id() -> Option<String> {
    CORRELATION_ID.try_with(Clone::clone).ok()
}

/// Generate a new correlation ID.
///
/// Returns the first 16 hex characters of a UUID v4 (no dashes). The result
/// is unique enough for session-scoped tracing while staying compact for HTTP
/// headers and structured log fields.
pub fn new_correlation_id() -> String {
    Uuid::new_v4().simple().to_string()[..16].to_owned()
}
