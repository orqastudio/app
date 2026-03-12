use std::path::{Path, PathBuf};

use crate::domain::lessons::{parse_lesson, render_lesson, Lesson, NewLesson};
use crate::domain::paths;
use crate::domain::time_utils;
use crate::error::OrqaError;

/// List all lessons from `.orqa/lessons/` in the given project directory.
///
/// Reads every `.md` file in the directory and parses its frontmatter.
/// Files that fail to parse are skipped and a warning is logged.
/// Returns lessons sorted by ID ascending.
pub fn list(project_path: &Path) -> Result<Vec<Lesson>, OrqaError> {
    let lessons_dir = lessons_dir(project_path);
    if !lessons_dir.exists() {
        return Ok(vec![]);
    }

    let entries = std::fs::read_dir(&lessons_dir)?;
    let mut lessons = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        match read_lesson_file(&path, project_path) {
            Ok(lesson) => lessons.push(lesson),
            Err(e) => {
                tracing::warn!("skipping unparseable lesson file {}: {}", path.display(), e);
            }
        }
    }

    lessons.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(lessons)
}

/// Get a single lesson by ID from `.orqa/lessons/`.
///
/// Returns `OrqaError::NotFound` if no lesson with the given ID exists.
pub fn get(project_path: &Path, id: &str) -> Result<Lesson, OrqaError> {
    let file_path = lesson_file_path(project_path, id);
    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!("lesson not found: {id}")));
    }
    read_lesson_file(&file_path, project_path)
}

/// Create a new lesson file in `.orqa/lessons/`.
///
/// Generates the next available IMPL-NNN ID, writes the markdown file
/// with YAML frontmatter, and returns the created lesson.
pub fn create(project_path: &Path, new_lesson: &NewLesson) -> Result<Lesson, OrqaError> {
    let lessons_dir = lessons_dir(project_path);
    std::fs::create_dir_all(&lessons_dir)?;

    let id = next_id(project_path)?;
    let today = time_utils::today_date_string();

    let lesson = Lesson {
        id: id.clone(),
        title: new_lesson.title.clone(),
        category: new_lesson.category.clone(),
        recurrence: 1,
        status: "active".to_string(),
        promoted_to: None,
        created: today.clone(),
        updated: today,
        body: new_lesson.body.clone(),
        file_path: relative_file_path(&id),
    };

    let content = render_lesson(&lesson);
    let file_path = lesson_file_path(project_path, &id);
    std::fs::write(&file_path, content)?;

    Ok(lesson)
}

/// Increment the recurrence count for a lesson and update its `updated` date.
///
/// Reads the existing file, increments the count, writes it back,
/// and returns the updated lesson.
pub fn increment_recurrence(project_path: &Path, id: &str) -> Result<Lesson, OrqaError> {
    let file_path = lesson_file_path(project_path, id);
    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!("lesson not found: {id}")));
    }

    let mut lesson = read_lesson_file(&file_path, project_path)?;
    lesson.recurrence += 1;
    lesson.updated = time_utils::today_date_string();

    let content = render_lesson(&lesson);
    std::fs::write(&file_path, content)?;

    Ok(lesson)
}

/// Determine the next available IMPL-NNN ID by scanning existing lesson files.
fn next_id(project_path: &Path) -> Result<String, OrqaError> {
    let lessons_dir = lessons_dir(project_path);
    let mut max_num: u32 = 0;

    if lessons_dir.exists() {
        let entries = std::fs::read_dir(&lessons_dir)?;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if let Some(num) = parse_impl_number(&name_str) {
                if num > max_num {
                    max_num = num;
                }
            }
        }
    }

    Ok(format!("IMPL-{:03}", max_num + 1))
}

/// Parse the numeric suffix from a filename like "IMPL-042.md".
fn parse_impl_number(filename: &str) -> Option<u32> {
    let stem = filename.strip_suffix(".md")?;
    let num_str = stem.strip_prefix("IMPL-")?;
    num_str.parse::<u32>().ok()
}

/// Resolve the absolute path to the lessons directory.
fn lessons_dir(project_path: &Path) -> PathBuf {
    project_path.join(paths::LESSONS_DIR)
}

/// Resolve the absolute path to a specific lesson file.
fn lesson_file_path(project_path: &Path, id: &str) -> PathBuf {
    lessons_dir(project_path).join(format!("{id}.md"))
}

/// Compute the relative file path for a lesson ID, for storage in the struct.
fn relative_file_path(id: &str) -> String {
    format!("{}/{id}.md", paths::LESSONS_DIR)
}

/// Read and parse a lesson file, computing its relative path from the project root.
fn read_lesson_file(file_path: &Path, project_path: &Path) -> Result<Lesson, OrqaError> {
    let content = std::fs::read_to_string(file_path)?;
    let relative = file_path
        .strip_prefix(project_path)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| file_path.to_string_lossy().replace('\\', "/"));
    parse_lesson(&content, &relative).map_err(|e| {
        OrqaError::Serialization(format!("failed to parse {}: {e}", file_path.display()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    #[test]
    fn list_empty_when_no_lessons_dir() {
        let dir = make_project();
        let lessons = list(dir.path()).expect("list should succeed");
        assert!(lessons.is_empty());
    }

    #[test]
    fn create_writes_file_and_returns_lesson() {
        let dir = make_project();
        let new = NewLesson {
            title: "Test lesson".to_string(),
            category: "process".to_string(),
            body: "## Description\nSome content.\n".to_string(),
        };
        let lesson = create(dir.path(), &new).expect("create should succeed");
        assert_eq!(lesson.id, "IMPL-001");
        assert_eq!(lesson.title, "Test lesson");
        assert_eq!(lesson.category, "process");
        assert_eq!(lesson.recurrence, 1);
        assert_eq!(lesson.status, "active");

        let file = lesson_file_path(dir.path(), "IMPL-001");
        assert!(file.exists(), "lesson file should be created on disk");
    }

    #[test]
    fn create_sequential_ids() {
        let dir = make_project();
        let new = |title: &str| NewLesson {
            title: title.to_string(),
            category: "coding".to_string(),
            body: "body".to_string(),
        };
        let l1 = create(dir.path(), &new("First")).expect("create first");
        let l2 = create(dir.path(), &new("Second")).expect("create second");
        let l3 = create(dir.path(), &new("Third")).expect("create third");
        assert_eq!(l1.id, "IMPL-001");
        assert_eq!(l2.id, "IMPL-002");
        assert_eq!(l3.id, "IMPL-003");
    }

    #[test]
    fn get_existing_lesson() {
        let dir = make_project();
        let new = NewLesson {
            title: "My lesson".to_string(),
            category: "architecture".to_string(),
            body: "body".to_string(),
        };
        create(dir.path(), &new).expect("create");
        let lesson = get(dir.path(), "IMPL-001").expect("get should succeed");
        assert_eq!(lesson.title, "My lesson");
    }

    #[test]
    fn get_missing_lesson_returns_not_found() {
        let dir = make_project();
        let result = get(dir.path(), "IMPL-999");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn list_returns_lessons_sorted_by_id() {
        let dir = make_project();
        let new = |title: &str| NewLesson {
            title: title.to_string(),
            category: "process".to_string(),
            body: "body".to_string(),
        };
        create(dir.path(), &new("C")).expect("c");
        create(dir.path(), &new("A")).expect("a");
        create(dir.path(), &new("B")).expect("b");
        let lessons = list(dir.path()).expect("list");
        assert_eq!(lessons.len(), 3);
        assert_eq!(lessons[0].id, "IMPL-001");
        assert_eq!(lessons[1].id, "IMPL-002");
        assert_eq!(lessons[2].id, "IMPL-003");
    }

    #[test]
    fn increment_recurrence_updates_count() {
        let dir = make_project();
        let new = NewLesson {
            title: "Recurring".to_string(),
            category: "process".to_string(),
            body: "body".to_string(),
        };
        create(dir.path(), &new).expect("create");
        let updated = increment_recurrence(dir.path(), "IMPL-001").expect("increment");
        assert_eq!(updated.recurrence, 2);

        // Verify it persisted
        let reloaded = get(dir.path(), "IMPL-001").expect("reload");
        assert_eq!(reloaded.recurrence, 2);
    }

    #[test]
    fn increment_recurrence_missing_id_returns_not_found() {
        let dir = make_project();
        let result = increment_recurrence(dir.path(), "IMPL-999");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn parse_impl_number_valid() {
        assert_eq!(parse_impl_number("IMPL-001.md"), Some(1));
        assert_eq!(parse_impl_number("IMPL-042.md"), Some(42));
    }

    #[test]
    fn parse_impl_number_invalid() {
        assert_eq!(parse_impl_number("README.md"), None);
        assert_eq!(parse_impl_number("IMPL-.md"), None);
    }
}
