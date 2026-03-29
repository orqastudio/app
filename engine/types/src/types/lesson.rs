//! Lesson domain types for the OrqaStudio engine.
//!
//! Defines structs for lessons — first-class governance artifacts in the learning loop.
//! Lessons are stored in `.orqa/lessons/` and feed the self-improvement pipeline:
//! recurring patterns are promoted to enforcement rules.
//!
//! Category and status values are opaque strings declared by the methodology plugin.
//! The engine does not enumerate or validate specific values.

use serde::{Deserialize, Serialize};

/// A single lesson captured from agent sessions.
///
/// Lessons are stored as individual markdown files in `.orqa/lessons/`
/// with YAML frontmatter. They are first-class governance artifacts that
/// feed the self-learning loop. Category and status values come from
/// the methodology plugin's schema — the engine treats them as opaque strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    /// Unique identifier, e.g. "LEARN-001".
    pub id: String,
    /// Short title describing the lesson.
    pub title: String,
    /// Category key declared by the methodology plugin (opaque string).
    pub category: String,
    /// Number of times this pattern has recurred.
    pub recurrence: i32,
    /// Status key declared by the methodology plugin (opaque string).
    pub status: String,
    /// Path to the rule or standard this lesson was promoted to, if any.
    pub promoted_to: Option<String>,
    /// ISO-8601 date string when the lesson was first created.
    pub created: String,
    /// ISO-8601 date string when the lesson was last updated.
    pub updated: String,
    /// Full markdown body (everything after the YAML frontmatter).
    pub body: String,
    /// Relative file path within the project, e.g. ".orqa/learning/lessons/LEARN-001.md".
    pub file_path: String,
}

/// Input for creating a new lesson.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLesson {
    /// Short title describing the lesson.
    pub title: String,
    /// Category key declared by the methodology plugin.
    pub category: String,
    /// Full markdown body of the lesson.
    pub body: String,
}
