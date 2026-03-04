use std::sync::{Arc, Mutex};

use serde::Serialize;

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

impl StartupTracker {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            tasks: Mutex::new(Vec::new()),
        })
    }

    pub fn register(&self, id: &str, label: &str) -> String {
        let mut tasks = self.tasks.lock().expect("startup tracker poisoned");
        tasks.push(StartupTask {
            id: id.to_string(),
            label: label.to_string(),
            status: TaskStatus::Pending,
            detail: None,
        });
        id.to_string()
    }

    pub fn update(&self, id: &str, status: TaskStatus, detail: Option<String>) {
        let mut tasks = self.tasks.lock().expect("startup tracker poisoned");
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.status = status;
            task.detail = detail;
        }
    }

    pub fn snapshot(&self) -> StartupSnapshot {
        let tasks = self.tasks.lock().expect("startup tracker poisoned");
        let all_done = !tasks.is_empty()
            && tasks
                .iter()
                .all(|t| matches!(t.status, TaskStatus::Done | TaskStatus::Error));
        StartupSnapshot {
            tasks: tasks.clone(),
            all_done,
        }
    }
}
