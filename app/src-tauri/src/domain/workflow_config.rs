// Workflow configuration builder for the OrqaStudio Tauri backend.
//
// Builds the `ProcessGateConfig` list and `TrackerConfig` from the project's
// artifact config in `.orqa/project.json`. Gate definitions are hardcoded as
// defaults because they are specified in the resolved workflow YAML — which is
// not yet wired. Tracker path rules ARE derived from the project.json artifacts
// array so no filesystem paths are hardcoded in the engine.

use orqa_engine::config::ArtifactEntry;

use crate::domain::process_gates::{GateCondition, GateTrigger, ProcessGateConfig};
use crate::domain::workflow_tracker::{PathCategory, PathRule, TrackerConfig};

/// Build the default set of process gates.
///
/// Returns the 5 standard process gates that enforce the documentation-first,
/// plan-before-build, and learn-after-doing workflow process. These will be
/// replaced with gates loaded from the resolved workflow YAML once the workflow
/// resolver pipeline is complete.
pub fn default_process_gates() -> Vec<ProcessGateConfig> {
    vec![
        ProcessGateConfig::new(
            "understand-first",
            GateTrigger::Write,
            GateCondition::FirstCodeWriteWithoutResearch,
            "THINK FIRST: What is the system you're modifying? \
             What are its boundaries? What depends on this? What could break? \
             Read the governing docs and understand the context before writing code.",
        ),
        ProcessGateConfig::new(
            "docs-before-code",
            GateTrigger::Write,
            GateCondition::CodeWriteWithoutDocs,
            "DOCUMENTATION CHECK: Have you read the documentation that defines \
             this area? Check .orqa/documentation/ for specs, patterns, and constraints \
             before implementing.",
        ),
        ProcessGateConfig::new(
            "plan-before-build",
            GateTrigger::Write,
            GateCondition::CodeWriteWithoutPlanning,
            "PLANNING CHECK: Is there an epic or task that defines this work? \
             Check .orqa/implementation/ for the scope, acceptance criteria, and \
             implementation design.",
        ),
        ProcessGateConfig::new(
            "evidence-before-done",
            GateTrigger::Stop,
            GateCondition::CodeWrittenWithoutVerification,
            "VERIFICATION CHECK: You wrote code but didn't run make check \
             or make test. Show evidence that the work is correct before completing.",
        ),
        ProcessGateConfig::new(
            "learn-after-doing",
            GateTrigger::Stop,
            GateCondition::SignificantWorkWithoutLessons { threshold: 3 },
            "LEARNING CHECK: Significant work was done this session. \
             Check .orqa/learning/lessons/ for known patterns and consider \
             if anything unexpected should be recorded.",
        ),
    ]
}

/// Build a `TrackerConfig` from the artifact entries in `.orqa/project.json`.
///
/// Each leaf artifact entry is inspected to assign a `PathCategory` based on
/// its key using the following convention:
///
/// - key is `"docs"` or contains `"documentation"` → `Docs`
/// - key is `"lessons"` or parent group key is `"learning"` with key `"lessons"` → `Lessons`
/// - parent group key is `"implementation"` or key contains `"implementation"` → `Planning`
///
/// Entries that do not match any category are skipped. When `artifacts` is empty,
/// the fallback defaults are returned so the tracker is always usable.
pub fn tracker_config_from_artifacts(artifacts: &[ArtifactEntry]) -> TrackerConfig {
    let mut rules = Vec::new();

    for entry in artifacts {
        match entry {
            ArtifactEntry::Group { key: group_key, children, .. } => {
                for child in children {
                    if let Some(cat) = classify_artifact_key(&child.key, Some(group_key)) {
                        rules.push(PathRule::new(child.path.clone(), cat));
                    }
                }
            }
            ArtifactEntry::Type(type_config) => {
                if let Some(cat) = classify_artifact_key(&type_config.key, None) {
                    rules.push(PathRule::new(type_config.path.clone(), cat));
                }
            }
        }
    }

    if rules.is_empty() {
        default_tracker_config()
    } else {
        TrackerConfig::new(rules)
    }
}

/// Classify an artifact type key into a `PathCategory` for gate evaluation.
///
/// The classification uses the artifact key and optional parent group key. Keys
/// that do not map to any gate category return `None` — those artifact types are
/// tracked for coverage but do not satisfy any process gate.
fn classify_artifact_key(key: &str, group_key: Option<&str>) -> Option<PathCategory> {
    // Documentation: "docs" type key or the standalone "documentation" type
    if key == "docs" || key.contains("documentation") {
        return Some(PathCategory::Docs);
    }
    // Lessons: the "lessons" leaf under the "learning" group, or any key named "lessons"
    if key == "lessons" {
        return Some(PathCategory::Lessons);
    }
    // Planning: any child of the "implementation" group, or a type key containing "implementation"
    if group_key == Some("implementation") || key.contains("implementation") {
        return Some(PathCategory::Planning);
    }
    None
}

/// Build the fallback tracker path classification config.
///
/// Used when no project is open or the project has no artifact config. These
/// paths mirror the OrqaStudio default project.json artifact layout. Once a
/// project is opened, `tracker_config_from_artifacts` replaces this with paths
/// read directly from the project's artifact config.
pub fn default_tracker_config() -> TrackerConfig {
    TrackerConfig::new(vec![
        PathRule::new(".orqa/documentation", PathCategory::Docs),
        PathRule::new(".orqa/implementation", PathCategory::Planning),
        PathRule::new(".orqa/learning/lessons", PathCategory::Lessons),
    ])
}
