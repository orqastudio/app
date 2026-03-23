use std::path::Path;

use tauri::State;

use crate::domain::lessons::{Lesson, NewLesson};
use crate::domain::paths::ProjectPaths;
use crate::error::OrqaError;
use crate::repo::lesson_repo;
use crate::state::AppState;

/// List all lessons from the configured lessons directory in the active project.
///
/// Returns an empty list if the directory does not exist yet.
#[tauri::command]
pub fn lessons_list(
    project_path: String,
    _state: State<'_, AppState>,
) -> Result<Vec<Lesson>, OrqaError> {
    let paths = ProjectPaths::load(Path::new(&project_path))?;
    lesson_repo::list(&paths)
}

/// Create a new lesson in the configured lessons directory.
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
    let paths = ProjectPaths::load(Path::new(&project_path))?;
    lesson_repo::create(&paths, &new_lesson)
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
    let paths = ProjectPaths::load(Path::new(&project_path))?;
    lesson_repo::increment_recurrence(&paths, &id)
}

#[cfg(test)]
mod tests {
    use crate::domain::lessons::NewLesson;
    use crate::domain::paths::ProjectPaths;
    use crate::domain::project_settings::{ArtifactEntry, ArtifactTypeConfig, ProjectSettings};
    use crate::repo::lesson_repo;

    fn make_project_paths(tmp: &tempfile::TempDir) -> ProjectPaths {
        let settings = ProjectSettings {
            name: "test".to_string(),
            dogfood: false,
            description: None,
            default_model: "auto".to_string(),
            excluded_paths: vec![],
            stack: None,
            governance: None,
            icon: None,
            show_thinking: false,
            custom_system_prompt: None,
            artifacts: vec![ArtifactEntry::Group {
                key: "process".to_string(),
                label: None,
                icon: None,
                children: vec![ArtifactTypeConfig {
                    key: "lessons".to_string(),
                    label: None,
                    icon: None,
                    path: ".orqa/process/lessons".to_string(),
                }],
            }],
            artifact_links: Default::default(),
            statuses: vec![],
            delivery: Default::default(),
            relationships: vec![],
            plugins: std::collections::HashMap::new(),
        };
        ProjectPaths::from_settings(tmp.path(), &settings)
    }

    #[test]
    fn list_empty_project_returns_empty() {
        let dir = tempfile::tempdir().expect("tempdir");
        let paths = make_project_paths(&dir);
        let lessons = lesson_repo::list(&paths).expect("should succeed");
        assert!(lessons.is_empty());
    }

    #[test]
    fn create_and_list_lessons() {
        let dir = tempfile::tempdir().expect("tempdir");
        let paths = make_project_paths(&dir);
        let new = NewLesson {
            title: "Test lesson".to_string(),
            category: "process".to_string(),
            body: "## Description\nContent here.\n".to_string(),
        };
        let lesson = lesson_repo::create(&paths, &new).expect("create");
        assert_eq!(lesson.id, "IMPL-001");
        assert_eq!(lesson.recurrence, 1);

        let all = lesson_repo::list(&paths).expect("list");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, "IMPL-001");
    }

    #[test]
    fn increment_recurrence_updates_count() {
        let dir = tempfile::tempdir().expect("tempdir");
        let paths = make_project_paths(&dir);
        let new = NewLesson {
            title: "Recurring pattern".to_string(),
            category: "coding".to_string(),
            body: "body".to_string(),
        };
        lesson_repo::create(&paths, &new).expect("create");
        let updated = lesson_repo::increment_recurrence(&paths, "IMPL-001").expect("increment");
        assert_eq!(updated.recurrence, 2);
    }

    #[test]
    fn increment_nonexistent_returns_error() {
        let dir = tempfile::tempdir().expect("tempdir");
        let paths = make_project_paths(&dir);
        let result = lesson_repo::increment_recurrence(&paths, "IMPL-999");
        assert!(result.is_err());
    }

    #[test]
    fn get_nonexistent_returns_not_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let paths = make_project_paths(&dir);
        let result = lesson_repo::get(&paths, "IMPL-999");
        assert!(result.is_err());
    }
}
