//! Standalone validation binary for `orqa-validation`.
//!
//! Accepts a project path, scans the `.orqa/` directory, runs the full
//! integrity check suite, and outputs a JSON report to stdout.
//!
//! # Usage
//!
//! ```text
//! orqa-validation <project-path> [--fix]
//! ```
//!
//! # Output
//!
//! Without `--fix`:
//! ```json
//! {
//!   "checks": [...],
//!   "health": { ... }
//! }
//! ```
//!
//! With `--fix`:
//! ```json
//! {
//!   "checks": [...],
//!   "health": { ... },
//!   "fixes_applied": [...]
//! }
//! ```
//!
//! Exits 0 if no errors were found (warnings are not errors). Exits 1 if there
//! are any `IntegritySeverity::Error` findings, or 2 on a fatal processing error.

use std::path::PathBuf;
use std::process;

use orqa_validation::{
    auto_fix, build_validation_context_with_types, compute_health,
    graph::{build_artifact_graph, load_project_config},
    platform::scan_plugin_manifests,
    types::IntegritySeverity,
    validate, AppliedFix, GraphHealth, IntegrityCheck, ValidationError,
};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct Report {
    checks: Vec<IntegrityCheck>,
    health: GraphHealth,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixes_applied: Option<Vec<AppliedFix>>,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <project-path> [--fix]", args[0]);
        process::exit(2);
    }

    let project_path = PathBuf::from(&args[1]);
    let apply_fixes_flag = args.iter().any(|a| a == "--fix");

    if !project_path.exists() {
        eprintln!(
            "Error: project path does not exist: {}",
            project_path.display()
        );
        process::exit(2);
    }

    match run(&project_path, apply_fixes_flag) {
        Ok(report) => {
            let has_errors = report
                .checks
                .iter()
                .any(|c| c.severity == IntegritySeverity::Error);

            match serde_json::to_string_pretty(&report) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    eprintln!("Error serialising report: {e}");
                    process::exit(2);
                }
            }

            process::exit(i32::from(has_errors));
        }
        Err(e) => {
            eprintln!("Fatal error: {e}");
            process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// Core logic
// ---------------------------------------------------------------------------

fn run(project_path: &std::path::Path, apply_fixes_flag: bool) -> Result<Report, ValidationError> {
    // Build the graph.
    let graph = build_artifact_graph(project_path)?;

    // Load project settings and plugin contributions.
    let (valid_statuses, delivery, project_relationships) = load_project_config(project_path);
    let plugin_contributions = scan_plugin_manifests(project_path);

    let ctx = build_validation_context_with_types(
        &valid_statuses,
        &delivery,
        &project_relationships,
        &plugin_contributions.relationships,
        &plugin_contributions.artifact_types,
    );

    // Run integrity checks.
    let checks = validate(&graph, &ctx);

    // Compute health metrics.
    let health = compute_health(&graph);

    // Optionally apply fixes.
    let fixes_applied = if apply_fixes_flag {
        Some(auto_fix(&graph, &checks, project_path)?)
    } else {
        None
    };

    Ok(Report {
        checks,
        health,
        fixes_applied,
    })
}
