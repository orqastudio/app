// Tauri IPC commands for lesson management.
//
// All lesson operations are delegated to the daemon via HTTP. The daemon
// owns lesson storage and the lesson filesystem. The app is a thin client.
//
// Endpoints used:
//   GET /lessons                   — list all lessons
//   POST /lessons                  — create a new lesson
//   PUT /lessons/:id/recurrence    — increment lesson recurrence count

use tauri::State;

use crate::daemon_client::NewLessonRequest;
use crate::error::OrqaError;
use crate::state::AppState;
use orqa_engine_types::types::lesson::Lesson;

/// List all lessons from the daemon.
#[tauri::command]
pub async fn lessons_list(
    _project_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<Lesson>, OrqaError> {
    state.daemon.client.list_lessons().await
}

/// Create a new lesson via the daemon.
#[tauri::command]
pub async fn lessons_create(
    _project_path: String,
    title: String,
    category: String,
    body: String,
    state: State<'_, AppState>,
) -> Result<Lesson, OrqaError> {
    let req = NewLessonRequest {
        title,
        category,
        body,
    };
    state.daemon.client.create_lesson(&req).await
}

/// Increment the recurrence count for a lesson via the daemon.
#[tauri::command]
pub async fn lesson_increment_recurrence(
    _project_path: String,
    id: String,
    state: State<'_, AppState>,
) -> Result<Lesson, OrqaError> {
    state.daemon.client.increment_lesson_recurrence(&id).await
}

#[cfg(test)]
mod tests {
    use orqa_engine_types::types::lesson::Lesson;

    #[test]
    fn lesson_serialization() {
        let lesson = Lesson {
            id: "IMPL-001".to_owned(),
            title: "Test lesson".to_owned(),
            category: "process".to_owned(),
            recurrence: 1,
            status: "active".to_owned(),
            promoted_to: None,
            created: "2026-04-01".to_owned(),
            updated: "2026-04-01".to_owned(),
            body: "## Body\nContent here.\n".to_owned(),
            file_path: ".orqa/learning/lessons/IMPL-001.md".to_owned(),
        };
        let json = serde_json::to_value(&lesson).expect("should serialize");
        assert_eq!(json["id"], "IMPL-001");
        assert_eq!(json["recurrence"], 1);
    }
}
