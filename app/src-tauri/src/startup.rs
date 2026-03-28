use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use serde::Serialize;

use crate::error::OrqaError;

/// A single initialization task tracked during application startup.
#[derive(Debug, Clone, Serialize)]
pub struct StartupTask {
    /// Unique identifier for the task (e.g., "sidecar", "embedding_model").
    pub id: String,
    /// Human-readable label shown in the startup progress UI.
    pub label: String,
    /// Current state of this task.
    pub status: TaskStatus,
    /// Optional detail message (e.g., download percentage, error reason).
    pub detail: Option<String>,
}

/// Possible states for a startup task.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Task has not yet started.
    Pending,
    /// Task is currently running.
    InProgress,
    /// Task completed successfully.
    Done,
    /// Task failed with an error.
    Error,
}

/// A snapshot of all startup task states at a point in time.
#[derive(Debug, Clone, Serialize)]
pub struct StartupSnapshot {
    /// All registered startup tasks and their current states.
    pub tasks: Vec<StartupTask>,
    /// True when every task has reached Done or Error — startup is no longer in progress.
    pub all_done: bool,
}

/// Thread-safe startup task registry.
///
/// Tracks the status of long-running initialization tasks (sidecar startup,
/// model downloads, etc.) so the frontend can display progress to the user.
pub struct StartupTracker {
    tasks: Mutex<Vec<StartupTask>>,
}

/// Acquire a mutex lock, mapping a poison error to `OrqaError::Database`.
fn lock_tasks(
    mutex: &Mutex<Vec<StartupTask>>,
) -> Result<MutexGuard<'_, Vec<StartupTask>>, OrqaError> {
    mutex.lock().map_err(|_: PoisonError<_>| {
        OrqaError::Database("startup tracker mutex poisoned".to_owned())
    })
}

impl StartupTracker {
    /// Create a new `StartupTracker` wrapped in an `Arc`.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            tasks: Mutex::new(Vec::new()),
        })
    }

    /// Register a new startup task by id and display label.
    pub fn register(&self, id: &str, label: &str) -> Result<String, OrqaError> {
        let mut tasks = lock_tasks(&self.tasks)?;
        tasks.push(StartupTask {
            id: id.to_owned(),
            label: label.to_owned(),
            status: TaskStatus::Pending,
            detail: None,
        });
        Ok(id.to_owned())
    }

    /// Update the status of a registered task.
    pub fn update(
        &self,
        id: &str,
        status: TaskStatus,
        detail: Option<String>,
    ) -> Result<(), OrqaError> {
        let mut tasks = lock_tasks(&self.tasks)?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.status = status;
            task.detail = detail;
        }
        Ok(())
    }

    /// Snapshot the current state of all registered tasks.
    pub fn snapshot(&self) -> Result<StartupSnapshot, OrqaError> {
        let tasks = lock_tasks(&self.tasks)?;
        let all_done = !tasks.is_empty()
            && tasks
                .iter()
                .all(|t| matches!(t.status, TaskStatus::Done | TaskStatus::Error));
        Ok(StartupSnapshot {
            tasks: tasks.clone(),
            all_done,
        })
    }
}
