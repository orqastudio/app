use std::path::Path;

use tauri::State;

use crate::domain::lessons::{Lesson, NewLesson};
use crate::error::OrqaError;
use crate::repo::lesson_repo;
use crate::state::AppState;

/// List all lessons from `.orqa/lessons/` in the active project.
///
/// Returns an empty list if the directory does not exist yet.
#[tauri::command]
pub fn lessons_list(
    project_path: String,
    _state: State<'_, AppState>,
) -> Result<Vec<Lesson>, OrqaError> {
    lesson_repo::list(Path::new(&project_path))
}

/// Create a new lesson in `.orqa/lessons/`.
///
/// Assigns the next available IMPL-NNN ID, writes the file, and returns the lesson.
#[tauri::command]
pub fn lessons_create(
    project_path: String,
    title: String,
    category: String,
    body: String,
    _state: State<'_, AppState>,
) -> Result<Lesson, OrqaError> {
    let new_lesson = NewLesson {
        title,
        category,
        body,
    };
    lesson_repo::create(Path::new(&project_path), &new_lesson)
}

/// Increment the recurrence count for a lesson and update its `updated` date.
///
/// Used by review agents when they see a pattern described by this lesson recur.
#[tauri::command]
pub fn lesson_increment_recurrence(
    project_path: String,
    id: String,
    _state: State<'_, AppState>,
) -> Result<Lesson, OrqaError> {
    lesson_repo::increment_recurrence(Path::new(&project_path), &id)
}
