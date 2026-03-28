//! File-backed lesson store for the orqa-lesson crate.
//!
//! Provides `FileLessonStore`, a concrete implementation of `LessonStore` that
//! reads and writes lesson files in the configured lessons directory. This
//! replaces the app-layer `lesson_repo` with an engine-level implementation that
//! any access layer (app, daemon, CLI) can use without duplicating file I/O logic.

use std::path::{Path, PathBuf};

use orqa_engine_types::paths::ProjectPaths;
use orqa_engine_types::traits::storage::LessonStore;
use orqa_engine_types::utils::time::today_date_string;

use crate::{parse_lesson, render_lesson, Lesson, NewLesson};

/// Concrete error type for `FileLessonStore` operations.
#[derive(Debug)]
pub enum LessonStoreError {
    /// A required configuration key was not found in `project.json`.
    NotConfigured(String),
    /// The requested lesson does not exist on disk.
    NotFound(String),
    /// An I/O error occurred while reading or writing lesson files.
    Io(std::io::Error),
    /// A lesson file could not be parsed.
    Parse(String),
}

impl std::fmt::Display for LessonStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConfigured(msg) => write!(f, "not configured: {msg}"),
            Self::NotFound(id) => write!(f, "lesson not found: {id}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Parse(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

impl std::error::Error for LessonStoreError {}

impl From<std::io::Error> for LessonStoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// A file-backed implementation of `LessonStore`.
///
/// Reads and writes individual markdown files in the project's configured
/// lessons directory. Uses `ProjectPaths` for path resolution, so the
/// directory is driven by `project.json` rather than hardcoded paths.
pub struct FileLessonStore {
    paths: ProjectPaths,
}

impl FileLessonStore {
    /// Create a new `FileLessonStore` for the given project paths configuration.
    pub fn new(paths: ProjectPaths) -> Self {
        Self { paths }
    }

    /// Resolve the absolute path to the lessons directory from project config.
    ///
    /// Returns `Err(NotConfigured)` when the `lessons` key is absent from config.
    fn lessons_dir(&self) -> Result<PathBuf, LessonStoreError> {
        self.paths.artifact_dir("lessons").ok_or_else(|| {
            LessonStoreError::NotConfigured(
                "no 'lessons' artifact path configured in project.json".to_owned(),
            )
        })
    }

    /// Get the relative path prefix for lessons (e.g. `.orqa/learning/lessons`).
    fn lessons_relative_path(&self) -> Result<String, LessonStoreError> {
        self.paths
            .artifact_relative_path("lessons")
            .map(String::from)
            .ok_or_else(|| {
                LessonStoreError::NotConfigured(
                    "no 'lessons' artifact path configured in project.json".to_owned(),
                )
            })
    }

    /// List all lessons from the configured directory.
    ///
    /// Reads every `.md` file in the directory and parses its frontmatter.
    /// Files that cannot be parsed are skipped with a warning log.
    /// Returns lessons sorted by ID ascending.
    pub fn list(&self) -> Result<Vec<Lesson>, LessonStoreError> {
        let Some(lessons_dir) = self.paths.artifact_dir("lessons") else {
            return Ok(vec![]);
        };

        if !lessons_dir.exists() {
            return Ok(vec![]);
        }

        let entries = std::fs::read_dir(&lessons_dir)?;
        let mut lessons = Vec::new();
        let project_root = self.paths.project_root();

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

    /// Get a single lesson by ID.
    ///
    /// Returns `Err(NotFound)` when no lesson with the given ID exists.
    pub fn get(&self, id: &str) -> Result<Lesson, LessonStoreError> {
        let lessons_dir = self.lessons_dir()?;
        let file_path = lessons_dir.join(format!("{id}.md"));
        if !file_path.exists() {
            return Err(LessonStoreError::NotFound(id.to_owned()));
        }
        read_lesson_file(&file_path, self.paths.project_root())
    }

    /// Create a new lesson file and return the created lesson.
    ///
    /// Generates the next sequential `IMPL-NNN` ID, writes the markdown file
    /// with YAML frontmatter, and returns the fully-populated `Lesson`.
    pub fn create(&self, new_lesson: &NewLesson) -> Result<Lesson, LessonStoreError> {
        let lessons_dir = self.lessons_dir()?;
        std::fs::create_dir_all(&lessons_dir)?;

        let id = self.next_id()?;
        let today = today_date_string();
        let rel_prefix = self.lessons_relative_path()?;

        let lesson = Lesson {
            id: id.clone(),
            title: new_lesson.title.clone(),
            category: new_lesson.category.clone(),
            recurrence: 1,
            status: "active".to_owned(),
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
    pub fn increment_recurrence(&self, id: &str) -> Result<Lesson, LessonStoreError> {
        let lessons_dir = self.lessons_dir()?;
        let file_path = lessons_dir.join(format!("{id}.md"));
        if !file_path.exists() {
            return Err(LessonStoreError::NotFound(id.to_owned()));
        }

        let mut lesson = read_lesson_file(&file_path, self.paths.project_root())?;
        lesson.recurrence += 1;
        lesson.updated = today_date_string();

        let content = render_lesson(&lesson);
        std::fs::write(&file_path, content)?;

        Ok(lesson)
    }

    /// Determine the next available `IMPL-NNN` ID by scanning existing files.
    fn next_id(&self) -> Result<String, LessonStoreError> {
        let lessons_dir = self.lessons_dir()?;
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
}

/// Implement the abstract `LessonStore` trait using the file-backed `FileLessonStore`.
///
/// The trait methods delegate to the same-named concrete methods defined above.
impl LessonStore for FileLessonStore {
    type Error = LessonStoreError;

    fn read(&self, path: &Path) -> Result<Lesson, Self::Error> {
        read_lesson_file(path, self.paths.project_root())
    }

    fn write(&self, path: &Path, lesson: &Lesson) -> Result<(), Self::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = render_lesson(lesson);
        std::fs::write(path, content)?;
        Ok(())
    }

    fn scan(&self, dir: &Path) -> Result<Vec<Lesson>, Self::Error> {
        if !dir.exists() {
            return Ok(vec![]);
        }
        let entries = std::fs::read_dir(dir)?;
        let mut lessons = Vec::new();
        let project_root = self.paths.project_root();

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

        Ok(lessons)
    }
}

/// Read and parse a lesson file, computing its relative path from the project root.
fn read_lesson_file(file_path: &Path, project_root: &Path) -> Result<Lesson, LessonStoreError> {
    let content = std::fs::read_to_string(file_path)?;
    let relative = file_path.strip_prefix(project_root).map_or_else(
        |_| file_path.to_string_lossy().replace('\\', "/"),
        |p| p.to_string_lossy().replace('\\', "/"),
    );
    parse_lesson(&content, &relative)
        .map_err(|e| LessonStoreError::Parse(format!("failed to parse {}: {e}", file_path.display())))
}

/// Parse the numeric suffix from a filename like `IMPL-042.md`.
fn parse_impl_number(filename: &str) -> Option<u32> {
    let stem = filename.strip_suffix(".md")?;
    let num_str = stem.strip_prefix("IMPL-")?;
    num_str.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_validation::settings::{ArtifactEntry, ArtifactTypeConfig, ProjectSettings};
    use tempfile::TempDir;

    fn make_store(tmp: &TempDir) -> FileLessonStore {
        let settings = make_settings();
        let paths = ProjectPaths::from_settings(tmp.path(), &settings);
        FileLessonStore::new(paths)
    }

    fn make_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test".to_string(),
            organisation: false,
            projects: vec![],
            artifacts: vec![ArtifactEntry::Group {
                key: "process".to_string(),
                label: None,
                icon: None,
                children: vec![ArtifactTypeConfig {
                    key: "lessons".to_string(),
                    label: None,
                    icon: None,
                    path: ".orqa/learning/lessons".to_string(),
                }],
            }],
            statuses: vec![],
            delivery: Default::default(),
            relationships: vec![],
            plugins: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn list_empty_when_no_lessons_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let lessons = store.list().expect("list should succeed");
        assert!(lessons.is_empty());
    }

    #[test]
    fn create_writes_file_and_returns_lesson() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let new = NewLesson {
            title: "Test lesson".to_string(),
            category: "process".to_string(),
            body: "## Description\nSome content.\n".to_string(),
        };
        let lesson = store.create(&new).expect("create should succeed");
        assert_eq!(lesson.id, "IMPL-001");
        assert_eq!(lesson.title, "Test lesson");
        assert_eq!(lesson.category, "process");
        assert_eq!(lesson.recurrence, 1);
        assert_eq!(lesson.status, "active");
        assert_eq!(lesson.file_path, ".orqa/learning/lessons/IMPL-001.md");

        let file = dir
            .path()
            .join(".orqa/learning/lessons/IMPL-001.md");
        assert!(file.exists(), "lesson file should be created on disk");
    }

    #[test]
    fn create_sequential_ids() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let new = |title: &str| NewLesson {
            title: title.to_string(),
            category: "coding".to_string(),
            body: "body".to_string(),
        };
        let l1 = store.create(&new("First")).expect("create first");
        let l2 = store.create(&new("Second")).expect("create second");
        let l3 = store.create(&new("Third")).expect("create third");
        assert_eq!(l1.id, "IMPL-001");
        assert_eq!(l2.id, "IMPL-002");
        assert_eq!(l3.id, "IMPL-003");
    }

    #[test]
    fn get_existing_lesson() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let new = NewLesson {
            title: "My lesson".to_string(),
            category: "architecture".to_string(),
            body: "body".to_string(),
        };
        store.create(&new).expect("create");
        let lesson = store.get("IMPL-001").expect("get should succeed");
        assert_eq!(lesson.title, "My lesson");
    }

    #[test]
    fn get_missing_lesson_returns_not_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let result = store.get("IMPL-999");
        assert!(matches!(result, Err(LessonStoreError::NotFound(_))));
    }

    #[test]
    fn list_returns_lessons_sorted_by_id() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let new = |title: &str| NewLesson {
            title: title.to_string(),
            category: "process".to_string(),
            body: "body".to_string(),
        };
        store.create(&new("C")).expect("c");
        store.create(&new("A")).expect("a");
        store.create(&new("B")).expect("b");
        let lessons = store.list().expect("list");
        assert_eq!(lessons.len(), 3);
        assert_eq!(lessons[0].id, "IMPL-001");
        assert_eq!(lessons[1].id, "IMPL-002");
        assert_eq!(lessons[2].id, "IMPL-003");
    }

    #[test]
    fn increment_recurrence_updates_count() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let new = NewLesson {
            title: "Recurring".to_string(),
            category: "process".to_string(),
            body: "body".to_string(),
        };
        store.create(&new).expect("create");
        let updated = store
            .increment_recurrence("IMPL-001")
            .expect("increment");
        assert_eq!(updated.recurrence, 2);

        let reloaded = store.get("IMPL-001").expect("reload");
        assert_eq!(reloaded.recurrence, 2);
    }

    #[test]
    fn increment_recurrence_missing_id_returns_not_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let result = store.increment_recurrence("IMPL-999");
        assert!(matches!(result, Err(LessonStoreError::NotFound(_))));
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
    fn trait_read_and_write_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let new = NewLesson {
            title: "Roundtrip".to_string(),
            category: "coding".to_string(),
            body: "body content\n".to_string(),
        };
        let lesson = store.create(&new).expect("create");
        let file_path = dir
            .path()
            .join(".orqa/learning/lessons/IMPL-001.md");

        let read_back = LessonStore::read(&store, &file_path).expect("trait read");
        assert_eq!(read_back.id, lesson.id);
        assert_eq!(read_back.title, lesson.title);
    }

    #[test]
    fn trait_scan_returns_all_lessons() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = make_store(&dir);
        let new = |t: &str| NewLesson {
            title: t.to_string(),
            category: "process".to_string(),
            body: "body".to_string(),
        };
        store.create(&new("One")).expect("one");
        store.create(&new("Two")).expect("two");

        let lessons_dir = dir.path().join(".orqa/learning/lessons");
        let scanned = LessonStore::scan(&store, &lessons_dir).expect("scan");
        assert_eq!(scanned.len(), 2);
    }
}
