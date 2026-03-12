use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use serde::Serialize;

use crate::error::OrqaError;

#[derive(Debug, Clone, Serialize)]
pub struct StartupTask {
    pub id: String,
    pub label: String,
    pub status: TaskStatus,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Done,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartupSnapshot {
    pub tasks: Vec<StartupTask>,
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
        OrqaError::Database("startup tracker mutex poisoned".to_string())
    })
}

impl StartupTracker {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            tasks: Mutex::new(Vec::new()),
        })
    }

    /// Register a new startup task by id and display label.
    pub fn register(&self, id: &str, label: &str) -> Result<String, OrqaError> {
        let mut tasks = lock_tasks(&self.tasks)?;
        tasks.push(StartupTask {
            id: id.to_string(),
            label: label.to_string(),
            status: TaskStatus::Pending,
            detail: None,
        });
        Ok(id.to_string())
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
