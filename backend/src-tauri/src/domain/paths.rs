/// Central directory name for Orqa project configuration and metadata.
pub const ORQA_DIR: &str = ".orqa";

/// Path to the project settings file, relative to the project root.
pub const SETTINGS_FILE: &str = ".orqa/project.json";

/// Directory containing research documents, relative to the project root.
pub const RESEARCH_DIR: &str = ".orqa/delivery/research";

/// Directory containing pillar definitions, relative to the project root.
pub const PILLARS_DIR: &str = ".orqa/process/pillars";

/// Directory containing implementation lessons, relative to the project root.
pub const LESSONS_DIR: &str = ".orqa/process/lessons";

/// Directory containing project milestones, relative to the project root.
pub const MILESTONES_DIR: &str = ".orqa/delivery/milestones";

/// Directory containing project epics, relative to the project root.
pub const EPICS_DIR: &str = ".orqa/delivery/epics";

/// Directory containing project tasks, relative to the project root.
pub const TASKS_DIR: &str = ".orqa/delivery/tasks";

/// Directory containing captured ideas, relative to the project root.
pub const IDEAS_DIR: &str = ".orqa/delivery/ideas";

/// Directory containing architecture decision records, relative to the project root.
pub const DECISIONS_DIR: &str = ".orqa/process/decisions";

/// Path to the DuckDB search index, relative to the project root.
pub const SEARCH_DB: &str = ".orqa/search.duckdb";
