// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: cargo build -p orqa-validation

#![allow(dead_code, unused_imports, missing_docs)]

use serde::{Deserialize, Serialize};

/// Category of integrity issue found in the artifact graph. Generic categories derived from schema-driven checks — no relationship keys or artifact types are hardcoded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrityCategory {
    BrokenLink,
    TypeConstraintViolation,
    RequiredRelationshipMissing,
    CardinalityViolation,
    CircularDependency,
    InvalidStatus,
    BodyTextRefWithoutRelationship,
    ParentChildInconsistency,
    DeliveryPathMismatch,
    MissingType,
    MissingStatus,
    DuplicateRelationship,
    FilenameMismatch,
}

/// Severity of an integrity finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegritySeverity {
    Error,
    Warning,
    Info,
}

/// A single integrity finding from the artifact graph validation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCheck {
    pub category: IntegrityCategory,
    pub severity: IntegritySeverity,
    /// The ID of the artifact with the integrity issue.
    pub artifact_id: String,
    /// Human-readable description of the integrity issue.
    pub message: String,
    /// Whether this issue can be automatically fixed.
    pub auto_fixable: bool,
    /// Description of the fix that would be applied if auto_fixable is true.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fix_description: Option<String>,
}

/// A fix that was applied to resolve an integrity issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    /// The ID of the artifact that was fixed.
    pub artifact_id: String,
    /// Human-readable description of what was changed.
    pub description: String,
    /// Relative path to the file that was modified.
    pub file_path: String,
}

