// Lesson domain types for the OrqaStudio engine.
//
// Defines structs for lessons — first-class governance artifacts in the learning loop.
// Lessons are stored in `.orqa/lessons/` and feed the self-improvement pipeline:
// recurring patterns are promoted to enforcement rules.

use serde::{Deserialize, Serialize};

/// A single lesson captured from agent sessions.
///
/// Lessons are stored as individual markdown files in `.orqa/lessons/`
/// with YAML frontmatter. They are first-class governance artifacts that
/// feed the self-learning loop (Pillar 1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    /// Unique identifier, e.g. "IMPL-001".
    pub id: String,
    /// Short title describing the lesson.
    pub title: String,
    /// Category: "process", "coding", or "architecture".
    pub category: String,
    /// Number of times this pattern has recurred.
    pub recurrence: i32,
    /// Status: "active", "promoted", or "resolved".
    pub status: String,
    /// Path to the rule or standard this lesson was promoted to, if any.
    pub promoted_to: Option<String>,
    /// ISO-8601 date string when the lesson was first created.
    pub created: String,
    /// ISO-8601 date string when the lesson was last updated.
    pub updated: String,
    /// Full markdown body (everything after the YAML frontmatter).
    pub body: String,
    /// Relative file path within the project, e.g. ".orqa/process/lessons/IMPL-001.md".
    pub file_path: String,
}

/// Input for creating a new lesson.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLesson {
    pub title: String,
    pub category: String,
    pub body: String,
}
