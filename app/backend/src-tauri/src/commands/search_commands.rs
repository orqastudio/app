use tauri::State;

use crate::error::OrqaError;
use crate::startup::StartupSnapshot;
use crate::state::AppState;

/// Get the current status of all startup tasks.
///
/// Returns a snapshot of every registered startup task with its current
/// status and optional detail string (e.g. download percentage).
#[tauri::command]
pub async fn get_startup_status(state: State<'_, AppState>) -> Result<StartupSnapshot, OrqaError> {
    state
        .startup
        .tracker
        .snapshot()
        .map_err(|e| OrqaError::Search(e.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::startup::{StartupTracker, TaskStatus};

    #[test]
    fn empty_tracker_snapshot() {
        let tracker = StartupTracker::new();
        let snapshot = tracker.snapshot().expect("snapshot");
        assert!(snapshot.tasks.is_empty());
        assert!(!snapshot.all_done); // empty = not all done
    }

    #[test]
    fn register_and_snapshot() {
        let tracker = StartupTracker::new();
        tracker
            .register("embedder", "Loading embedder")
            .expect("register");
        let snapshot = tracker.snapshot().expect("snapshot");
        assert_eq!(snapshot.tasks.len(), 1);
        assert_eq!(snapshot.tasks[0].id, "embedder");
        assert_eq!(snapshot.tasks[0].status, TaskStatus::Pending);
        assert!(!snapshot.all_done);
    }

    #[test]
    fn all_done_when_all_tasks_complete() {
        let tracker = StartupTracker::new();
        tracker.register("task1", "Task 1").expect("register");
        tracker.register("task2", "Task 2").expect("register");
        tracker
            .update("task1", TaskStatus::Done, None)
            .expect("update");
        tracker
            .update("task2", TaskStatus::Done, None)
            .expect("update");

        let snapshot = tracker.snapshot().expect("snapshot");
        assert!(snapshot.all_done);
    }

    #[test]
    fn not_all_done_when_some_pending() {
        let tracker = StartupTracker::new();
        tracker.register("task1", "Task 1").expect("register");
        tracker.register("task2", "Task 2").expect("register");
        tracker
            .update("task1", TaskStatus::Done, None)
            .expect("update");
        // task2 still pending

        let snapshot = tracker.snapshot().expect("snapshot");
        assert!(!snapshot.all_done);
    }

    #[test]
    fn error_status_counts_as_done() {
        let tracker = StartupTracker::new();
        tracker.register("task1", "Task 1").expect("register");
        tracker
            .update("task1", TaskStatus::Error, Some("failed".to_string()))
            .expect("update");

        let snapshot = tracker.snapshot().expect("snapshot");
        assert!(snapshot.all_done);
        assert_eq!(snapshot.tasks[0].detail, Some("failed".to_string()));
    }
}
