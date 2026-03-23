use std::path::{Path, PathBuf};

use crate::domain::lessons::{parse_lesson, render_lesson, Lesson, NewLesson};
use crate::domain::paths::ProjectPaths;
use crate::domain::time_utils;
use crate::error::OrqaError;

/// Resolve the lessons directory from the project paths config.
///
/// Falls back to `None` if the "lessons" key is not in the config.
fn lessons_dir_from_config(paths: &ProjectPaths) -> Option<PathBuf> {
    paths.artifact_dir("lessons")
}

/// Resolve the lessons directory, returning an error if not configured.
fn require_lessons_dir(paths: &ProjectPaths) -> Result<PathBuf, OrqaError> {
    lessons_dir_from_config(paths).ok_or_else(|| {
        OrqaError::Validation("no 'lessons' artifact path configured in project.json".to_string())
    })
}

/// Get the relative path prefix for lessons from config.
fn lessons_relative_path(paths: &ProjectPaths) -> Result<String, OrqaError> {
    paths
        .artifact_relative_path("lessons")
        .map(String::from)
        .ok_or_else(|| {
            OrqaError::Validation(
                "no 'lessons' artifact path configured in project.json".to_string(),
            )
        })
}

/// List all lessons from the configured lessons directory.
///
/// Reads every `.md` file in the directory and parses its frontmatter.
/// Files that fail to parse are skipped and a warning is logged.
/// Returns lessons sorted by ID ascending.
pub fn list(paths: &ProjectPaths) -> Result<Vec<Lesson>, OrqaError> {
    let Some(lessons_dir) = lessons_dir_from_config(paths) else {
        return Ok(vec![]);
    };

    if !lessons_dir.exists() {
        return Ok(vec![]);
    }

    let entries = std::fs::read_dir(&lessons_dir)?;
    let mut lessons = Vec::new();
    let project_root = paths.project_root();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        match read_lesson_file(&path, project_root) {
            Ok(lesson) => lessons.push(lesson),
            Err(e) => {
                tracing::warn!("skipping unparseable lesson file {}: {}", path.display(), e);
            }
        }
    }

    lessons.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(lessons)
}

/// Get a single lesson by ID from the configured lessons directory.
///
/// Returns `OrqaError::NotFound` if no lesson with the given ID exists.
pub fn get(paths: &ProjectPaths, id: &str) -> Result<Lesson, OrqaError> {
    let lessons_dir = require_lessons_dir(paths)?;
    let file_path = lessons_dir.join(format!("{id}.md"));
    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!("lesson not found: {id}")));
    }
    read_lesson_file(&file_path, paths.project_root())
}

/// Create a new lesson file in the configured lessons directory.
///
/// Generates the next available IMPL-NNN ID, writes the markdown file
/// with YAML frontmatter, and returns the created lesson.
pub fn create(paths: &ProjectPaths, new_lesson: &NewLesson) -> Result<Lesson, OrqaError> {
    let lessons_dir = require_lessons_dir(paths)?;
    std::fs::create_dir_all(&lessons_dir)?;

    let id = next_id(paths)?;
    let today = time_utils::today_date_string();
    let rel_prefix = lessons_relative_path(paths)?;

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
        file_path: format!("{rel_prefix}/{id}.md"),
    };

    let content = render_lesson(&lesson);
    let file_path = lessons_dir.join(format!("{id}.md"));
    std::fs::write(&file_path, content)?;

    Ok(lesson)
}

/// Increment the recurrence count for a lesson and update its `updated` date.
///
/// Reads the existing file, increments the count, writes it back,
/// and returns the updated lesson.
pub fn increment_recurrence(paths: &ProjectPaths, id: &str) -> Result<Lesson, OrqaError> {
    let lessons_dir = require_lessons_dir(paths)?;
    let file_path = lessons_dir.join(format!("{id}.md"));
    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!("lesson not found: {id}")));
    }

    let mut lesson = read_lesson_file(&file_path, paths.project_root())?;
    lesson.recurrence += 1;
    lesson.updated = time_utils::today_date_string();

    let content = render_lesson(&lesson);
    std::fs::write(&file_path, content)?;

    Ok(lesson)
}

/// Determine the next available IMPL-NNN ID by scanning existing lesson files.
fn next_id(paths: &ProjectPaths) -> Result<String, OrqaError> {
    let lessons_dir = require_lessons_dir(paths)?;
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

/// Read and parse a lesson file, computing its relative path from the project root.
fn read_lesson_file(file_path: &Path, project_root: &Path) -> Result<Lesson, OrqaError> {
    let content = std::fs::read_to_string(file_path)?;
    let relative = file_path.strip_prefix(project_root).map_or_else(
        |_| file_path.to_string_lossy().replace('\\', "/"),
        |p| p.to_string_lossy().replace('\\', "/"),
    );
    parse_lesson(&content, &relative).map_err(|e| {
        OrqaError::Serialization(format!("failed to parse {}: {e}", file_path.display()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::project_settings::{ArtifactEntry, ArtifactTypeConfig, ProjectSettings};
    use tempfile::TempDir;

    fn make_project_paths(tmp: &TempDir) -> ProjectPaths {
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

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    #[test]
    fn list_empty_when_no_lessons_dir() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let lessons = list(&paths).expect("list should succeed");
        assert!(lessons.is_empty());
    }

    #[test]
    fn create_writes_file_and_returns_lesson() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let new = NewLesson {
            title: "Test lesson".to_string(),
            category: "process".to_string(),
            body: "## Description\nSome content.\n".to_string(),
        };
        let lesson = create(&paths, &new).expect("create should succeed");
        assert_eq!(lesson.id, "IMPL-001");
        assert_eq!(lesson.title, "Test lesson");
        assert_eq!(lesson.category, "process");
        assert_eq!(lesson.recurrence, 1);
        assert_eq!(lesson.status, "active");
        assert_eq!(lesson.file_path, ".orqa/process/lessons/IMPL-001.md");

        let file = paths
            .artifact_dir("lessons")
            .expect("lessons dir")
            .join("IMPL-001.md");
        assert!(file.exists(), "lesson file should be created on disk");
    }

    #[test]
    fn create_sequential_ids() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let new = |title: &str| NewLesson {
            title: title.to_string(),
            category: "coding".to_string(),
            body: "body".to_string(),
        };
        let l1 = create(&paths, &new("First")).expect("create first");
        let l2 = create(&paths, &new("Second")).expect("create second");
        let l3 = create(&paths, &new("Third")).expect("create third");
        assert_eq!(l1.id, "IMPL-001");
        assert_eq!(l2.id, "IMPL-002");
        assert_eq!(l3.id, "IMPL-003");
    }

    #[test]
    fn get_existing_lesson() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let new = NewLesson {
            title: "My lesson".to_string(),
            category: "architecture".to_string(),
            body: "body".to_string(),
        };
        create(&paths, &new).expect("create");
        let lesson = get(&paths, "IMPL-001").expect("get should succeed");
        assert_eq!(lesson.title, "My lesson");
    }

    #[test]
    fn get_missing_lesson_returns_not_found() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let result = get(&paths, "IMPL-999");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn list_returns_lessons_sorted_by_id() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let new = |title: &str| NewLesson {
            title: title.to_string(),
            category: "process".to_string(),
            body: "body".to_string(),
        };
        create(&paths, &new("C")).expect("c");
        create(&paths, &new("A")).expect("a");
        create(&paths, &new("B")).expect("b");
        let lessons = list(&paths).expect("list");
        assert_eq!(lessons.len(), 3);
        assert_eq!(lessons[0].id, "IMPL-001");
        assert_eq!(lessons[1].id, "IMPL-002");
        assert_eq!(lessons[2].id, "IMPL-003");
    }

    #[test]
    fn increment_recurrence_updates_count() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let new = NewLesson {
            title: "Recurring".to_string(),
            category: "process".to_string(),
            body: "body".to_string(),
        };
        create(&paths, &new).expect("create");
        let updated = increment_recurrence(&paths, "IMPL-001").expect("increment");
        assert_eq!(updated.recurrence, 2);

        // Verify it persisted
        let reloaded = get(&paths, "IMPL-001").expect("reload");
        assert_eq!(reloaded.recurrence, 2);
    }

    #[test]
    fn increment_recurrence_missing_id_returns_not_found() {
        let dir = make_project();
        let paths = make_project_paths(&dir);
        let result = increment_recurrence(&paths, "IMPL-999");
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

    #[test]
    fn list_returns_empty_when_no_config() {
        let dir = make_project();
        // Create ProjectPaths with NO artifacts configured
        let settings = ProjectSettings {
            name: "empty".to_string(),
            dogfood: false,
            description: None,
            default_model: "auto".to_string(),
            excluded_paths: vec![],
            stack: None,
            governance: None,
            icon: None,
            show_thinking: false,
            custom_system_prompt: None,
            artifacts: vec![],
            artifact_links: Default::default(),
            statuses: vec![],
            delivery: Default::default(),
            relationships: vec![],
            plugins: std::collections::HashMap::new(),
        };
        let paths = ProjectPaths::from_settings(dir.path(), &settings);
        let lessons = list(&paths).expect("list should succeed");
        assert!(lessons.is_empty());
    }
}
