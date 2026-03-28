// Storage trait interfaces for the OrqaStudio engine.
//
// These traits define the abstract I/O contracts that each access layer must fulfil.
// They are intentionally free of implementation detail — no filesystem paths baked in,
// no SQLite types, no Tauri state. Implementors provide the concrete mechanism;
// engine logic works against these traits exclusively.
//
// Each trait uses an associated `Error` type so that implementors can surface their
// own error kinds without forcing a common error enum into the engine crate.

use std::path::Path;

use crate::types::artifact::Artifact;
use crate::types::enforcement::EnforcementRule;
use crate::types::lesson::Lesson;

/// Read, write, scan, and delete governance artifact files.
///
/// Covers the operations performed by `artifact_fs` in the app layer. The trait
/// is generic enough to support both a filesystem implementation (current app) and
/// a future SQLite-backed or in-memory implementation used in tests.
pub trait ArtifactStore {
    /// The error type returned by all operations on this store.
    type Error: std::error::Error;

    /// Read a single artifact from the given path.
    ///
    /// Returns a fully-populated `Artifact` value including content and metadata.
    fn read(&self, path: &Path) -> Result<Artifact, Self::Error>;

    /// Write an artifact's content to the given path, creating parent directories if needed.
    fn write(&self, path: &Path, artifact: &Artifact) -> Result<(), Self::Error>;

    /// Recursively scan a directory and return all artifacts found within it.
    ///
    /// The scan is shallow or deep depending on the implementation; callers should
    /// not assume a specific traversal order.
    fn scan(&self, dir: &Path) -> Result<Vec<Artifact>, Self::Error>;

    /// Delete the artifact file at the given path.
    fn delete(&self, path: &Path) -> Result<(), Self::Error>;
}

/// Load enforcement rules from a project's rules directory.
///
/// Covers the operation performed by `enforcement_rules_repo::load_rules` in the
/// app layer. Rules are sourced from `.orqa/learning/rules/*.md` (or equivalent);
/// the trait does not prescribe the directory layout.
pub trait EnforcementRuleStore {
    /// The error type returned by all operations on this store.
    type Error: std::error::Error;

    /// Load and parse all enforcement rules accessible from the given project root.
    ///
    /// Implementations may skip unreadable or unparseable files with a warning
    /// rather than failing the entire load, but must document that behaviour.
    fn load_rules(&self, root: &Path) -> Result<Vec<EnforcementRule>, Self::Error>;
}

/// Read, write, and scan lesson files.
///
/// Covers the operations performed against `.orqa/learning/lessons/` in the app
/// layer. Lessons are first-class governance artifacts that feed the learning loop.
pub trait LessonStore {
    /// The error type returned by all operations on this store.
    type Error: std::error::Error;

    /// Read and parse a single lesson from the given path.
    fn read(&self, path: &Path) -> Result<Lesson, Self::Error>;

    /// Serialise and write a lesson to the given path, creating parent directories if needed.
    fn write(&self, path: &Path, lesson: &Lesson) -> Result<(), Self::Error>;

    /// Scan a directory and return all lessons found within it.
    fn scan(&self, dir: &Path) -> Result<Vec<Lesson>, Self::Error>;
}

/// Load project settings from the project root.
///
/// Covers the operation performed by `config_loader::load_project_settings` in the
/// app layer. The return type is `serde_json::Value` so that both the current
/// `ProjectSettings` struct and any future schema evolution are supported without
/// coupling the engine trait to a specific settings shape.
pub trait ProjectSettingsStore {
    /// The error type returned by all operations on this store.
    type Error: std::error::Error;

    /// Load project settings from the given project root.
    ///
    /// Returns `Ok(None)` if no settings file exists at the root.
    /// Returns `Err` if the file exists but cannot be read or parsed.
    fn load(&self, root: &Path) -> Result<Option<serde_json::Value>, Self::Error>;
}
