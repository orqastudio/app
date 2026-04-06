//! Standalone validation binary for `orqa-validation`.
// CLI binary — stdout/stderr are the intentional output channel.
#![allow(clippy::print_stderr, clippy::print_stdout)]
//!
//! # Subcommands
//!
//! ## Validate (default)
//!
//! ```text
//! orqa-validation <project-path> [--fix]
//! ```
//!
//! Scans the `.orqa/` directory, runs the full integrity check suite, and
//! outputs a JSON report to stdout.
//!
//! ## Parse
//!
//! ```text
//! orqa-validation parse <file-path> [--project <project-path>]
//! ```
//!
//! Parses a single `.md` artifact file and returns structured JSON. If
//! `--project` is omitted the file's parent directory is used as the project
//! root for plugin scanning.
//!
//! ## Query
//!
//! ```text
//! orqa-validation query <project-path> [--type <type>] [--status <status>] [--id <id>]
//! ```
//!
//! Queries the artifact graph and returns a JSON array of parsed artifacts.
//! Filters `--type`, `--status`, and `--id` are all optional and combinable.
//!
//! ## Hook
//!
//! ```text
//! orqa-validation hook <project-path> --event <event> --context '<json>'
//! ```
//!
//! Evaluates active enforcement rules against a lifecycle event context and
//! returns a `HookResult` JSON object with `action`, `messages`, and `violations`.
//!
//! Example:
//! ```text
//! orqa-validation hook /path/to/project \
//!   --event PreAction \
//!   --context '{"tool_name":"Bash","tool_input":{"command":"git push --force"}}'
//! ```
//!
//! # Exit codes
//!
//! | Code | Meaning |
//! |------|---------|
//! | 0    | Success / no errors |
//! | 1    | Validation errors found (validate subcommand only) |
//! | 2    | Fatal processing error |

use std::path::PathBuf;
use std::process;

use orqa_validation::{
    auto_fix, compute_health,
    content::{extract_behavioral_messages, find_agent, find_knowledge},
    context::build_validation_context_complete,
    evaluate_hook,
    graph::{build_artifact_graph, load_project_config},
    parse::{parse_artifact, query_artifacts},
    platform::scan_plugin_manifests,
    types::{HookContext, IntegrityCategory, IntegritySeverity},
    validate, AppliedFix, EnforcementEvent, EnforcementResult, GraphHealth, HookResult,
    IntegrityCheck, PipelineCategories, ValidationError,
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
    /// Enforcement events generated from schema and enforcement checks.
    enforcement_events: Vec<EnforcementEvent>,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage:\n  {} <project-path> [--fix]\n  {} parse <file> [--project <project-path>]\n  {} query <project-path> [--type <type>] [--status <status>] [--id <id>]\n  {} hook <project-path> --event <event> --context '<json>'",
            args[0], args[0], args[0], args[0]
        );
        process::exit(2);
    }

    match args[1].as_str() {
        "parse" => run_parse(&args),
        "query" => run_query(&args),
        "hook" => run_hook(&args),
        "content" => run_content(&args),
        _ => run_validate(&args),
    }
}

// ---------------------------------------------------------------------------
// Validate subcommand
// ---------------------------------------------------------------------------

fn run_validate(args: &[String]) {
    let project_path = PathBuf::from(&args[1]);
    let apply_fixes_flag = args.iter().any(|a| a == "--fix");

    if !project_path.exists() {
        eprintln!(
            "Error: project path does not exist: {}",
            project_path.display()
        );
        process::exit(2);
    }

    match validate_project(&project_path, apply_fixes_flag) {
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

/// Owned pipeline category data built from plugin contributions.
///
/// Holds `Vec<String>` so the borrow checker can derive `PipelineCategories<'_>`
/// from it without lifetime conflicts.
struct OwnedCategories {
    delivery: Vec<String>,
    learning: Vec<String>,
    excluded_statuses: Vec<String>,
    excluded_types: Vec<String>,
    root_types: Vec<String>,
}

impl OwnedCategories {
    /// Build from plugin contributions by filtering artifact types by `pipeline_category`.
    fn from_contributions(contributions: &orqa_validation::platform::PluginContributions) -> Self {
        let by_cat = |cat: &str| -> Vec<String> {
            contributions
                .artifact_types
                .iter()
                .filter(|t| t.pipeline_category.as_deref() == Some(cat))
                .map(|t| t.key.clone())
                .collect()
        };
        Self {
            delivery: by_cat("delivery"),
            learning: by_cat("learning"),
            excluded_statuses: contributions.terminal_statuses.clone(),
            excluded_types: by_cat("excluded"),
            root_types: by_cat("root"),
        }
    }

    /// Borrow as `PipelineCategories` with lifetime tied to `&self`.
    #[allow(clippy::type_complexity)]
    fn as_pipeline_categories(&self) -> (Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>) {
        (
            self.delivery.iter().map(String::as_str).collect(),
            self.learning.iter().map(String::as_str).collect(),
            self.excluded_statuses.iter().map(String::as_str).collect(),
            self.excluded_types.iter().map(String::as_str).collect(),
            self.root_types.iter().map(String::as_str).collect(),
        )
    }
}

fn validate_project(
    project_path: &std::path::Path,
    apply_fixes_flag: bool,
) -> Result<Report, ValidationError> {
    let graph = build_artifact_graph(project_path)?;
    let (valid_statuses, delivery, project_relationships) = load_project_config(project_path);
    let plugin_contributions = scan_plugin_manifests(project_path);

    let ctx = build_validation_context_complete(
        &valid_statuses,
        &delivery,
        &project_relationships,
        &plugin_contributions.relationships,
        &plugin_contributions.artifact_types,
        &plugin_contributions.schema_extensions,
        &plugin_contributions.enforcement_mechanisms,
    );

    let owned = OwnedCategories::from_contributions(&plugin_contributions);
    let (d, l, es, et, rt) = owned.as_pipeline_categories();
    let categories = PipelineCategories {
        delivery: &d,
        learning: &l,
        excluded_statuses: &es,
        excluded_types: &et,
        root_types: &rt,
    };

    let checks = validate(&graph, &ctx);
    let health = compute_health(&graph, &categories);
    let fixes_applied = if apply_fixes_flag {
        Some(auto_fix(&graph, &checks, project_path)?)
    } else {
        None
    };
    let enforcement_events = checks_to_enforcement_events(&checks);

    Ok(Report {
        checks,
        health,
        fixes_applied,
        enforcement_events,
    })
}

// ---------------------------------------------------------------------------
// Parse subcommand
// ---------------------------------------------------------------------------

fn run_parse(args: &[String]) {
    // orqa-validation parse <file> [--project <project-path>]
    if args.len() < 3 {
        eprintln!("Usage: {} parse <file> [--project <project-path>]", args[0]);
        process::exit(2);
    }

    let file_path = PathBuf::from(&args[2]);

    if !file_path.exists() {
        eprintln!("Error: file does not exist: {}", file_path.display());
        process::exit(2);
    }

    // Determine project root: explicit --project flag or the file's parent directory.
    let project_path = find_flag_value(args, "--project").map_or_else(
        || {
            file_path
                .parent()
                .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf)
        },
        PathBuf::from,
    );

    match parse_artifact(&file_path, &project_path) {
        Ok(parsed) => {
            match serde_json::to_string_pretty(&parsed) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    eprintln!("Error serialising result: {e}");
                    process::exit(2);
                }
            }
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error parsing artifact: {e}");
            process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// Query subcommand
// ---------------------------------------------------------------------------

fn run_query(args: &[String]) {
    // orqa-validation query <project-path> [--type <type>] [--status <status>] [--id <id>]
    if args.len() < 3 {
        eprintln!(
            "Usage: {} query <project-path> [--type <type>] [--status <status>] [--id <id>]",
            args[0]
        );
        process::exit(2);
    }

    let project_path = PathBuf::from(&args[2]);

    if !project_path.exists() {
        eprintln!(
            "Error: project path does not exist: {}",
            project_path.display()
        );
        process::exit(2);
    }

    let type_filter = find_flag_value(args, "--type");
    let status_filter = find_flag_value(args, "--status");
    let id_filter = find_flag_value(args, "--id");
    let search_filter = find_flag_value(args, "--search");

    let graph = match build_artifact_graph(&project_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error building artifact graph: {e}");
            process::exit(2);
        }
    };

    let plugin_contributions = scan_plugin_manifests(&project_path);

    let results = query_artifacts(
        &graph,
        &project_path,
        type_filter,
        status_filter,
        id_filter,
        search_filter,
        &plugin_contributions.artifact_types,
    );

    match serde_json::to_string_pretty(&results) {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("Error serialising results: {e}");
            process::exit(2);
        }
    }

    process::exit(0);
}

// ---------------------------------------------------------------------------
// Hook subcommand
// ---------------------------------------------------------------------------

/// Run the `hook` subcommand: evaluate rules against a lifecycle event.
fn run_hook(args: &[String]) {
    // orqa-validation hook <project-path> --event <event> --context '<json>'
    if args.len() < 3 {
        eprintln!(
            "Usage: {} hook <project-path> --event <event> --context '<json>'",
            args[0]
        );
        process::exit(2);
    }

    let project_path = PathBuf::from(&args[2]);

    if !project_path.exists() {
        eprintln!(
            "Error: project path does not exist: {}",
            project_path.display()
        );
        process::exit(2);
    }

    let ctx = build_hook_context(args);
    let result: HookResult = evaluate_hook(&ctx, &project_path);

    match serde_json::to_string_pretty(&result) {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("Error serialising result: {e}");
            process::exit(2);
        }
    }

    process::exit(0);
}

/// Parse `--event` and `--context` flags from args and build a [`HookContext`].
///
/// Exits the process with code 2 if required flags are missing or `--context` is not valid JSON.
fn build_hook_context(args: &[String]) -> HookContext {
    let Some(event_str) = find_flag_value(args, "--event") else {
        eprintln!("Error: --event is required");
        process::exit(2);
    };
    let event = event_str.to_owned();

    let Some(context_json) = find_flag_value(args, "--context") else {
        eprintln!("Error: --context is required");
        process::exit(2);
    };

    // Accept both a full HookContext object and a partial object —
    // any fields not present default to None.
    let partial: serde_json::Value = match serde_json::from_str(context_json) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: --context is not valid JSON: {e}");
            process::exit(2);
        }
    };

    hook_context_from_json(event, &partial)
}

/// Construct a [`HookContext`] from a parsed JSON object.
///
/// All optional fields default to `None` when absent from the JSON.
fn hook_context_from_json(event: String, partial: &serde_json::Value) -> HookContext {
    HookContext {
        event,
        tool_name: partial
            .get("tool_name")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        tool_input: partial.get("tool_input").cloned(),
        file_path: partial
            .get("file_path")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        user_message: partial
            .get("user_message")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        agent_type: partial
            .get("agent_type")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
    }
}

// ---------------------------------------------------------------------------
// Content subcommand
// ---------------------------------------------------------------------------

fn run_content(args: &[String]) {
    // orqa-validation content <subcommand> <project-path> [--flags]
    if args.len() < 4 {
        eprintln!(
            "Usage:\n  {} content agent <project-path> --match <agent-type>\n  {} content knowledge <project-path> --key <knowledge-key>\n  {} content behavioral <project-path>",
            args[0], args[0], args[0]
        );
        process::exit(2);
    }

    match args[2].as_str() {
        "agent" => run_content_agent(args),
        "knowledge" => run_content_knowledge(args),
        "behavioral" => run_content_behavioral(args),
        sub => {
            eprintln!("Unknown content subcommand: {sub}");
            process::exit(2);
        }
    }
}

fn run_content_agent(args: &[String]) {
    // orqa-validation content agent <project-path> --match <agent-type>
    let project_path = PathBuf::from(&args[3]);

    if !project_path.exists() {
        eprintln!(
            "Error: project path does not exist: {}",
            project_path.display()
        );
        process::exit(2);
    }

    let Some(agent_type) = find_flag_value(args, "--match") else {
        eprintln!("Error: --match is required");
        process::exit(2);
    };

    match find_agent(&project_path, agent_type) {
        Ok(Some(agent)) => {
            match serde_json::to_string_pretty(&agent) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    eprintln!("Error serialising result: {e}");
                    process::exit(2);
                }
            }
            process::exit(0);
        }
        Ok(None) => {
            eprintln!("No agent found matching: {agent_type}");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error loading agent: {e}");
            process::exit(2);
        }
    }
}

fn run_content_knowledge(args: &[String]) {
    // orqa-validation content knowledge <project-path> --key <knowledge-key>
    let project_path = PathBuf::from(&args[3]);

    if !project_path.exists() {
        eprintln!(
            "Error: project path does not exist: {}",
            project_path.display()
        );
        process::exit(2);
    }

    let Some(key) = find_flag_value(args, "--key") else {
        eprintln!("Error: --key is required");
        process::exit(2);
    };

    match find_knowledge(&project_path, key) {
        Ok(Some(knowledge)) => {
            match serde_json::to_string_pretty(&knowledge) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    eprintln!("Error serialising result: {e}");
                    process::exit(2);
                }
            }
            process::exit(0);
        }
        Ok(None) => {
            eprintln!("No knowledge found for key: {key}");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error loading knowledge: {e}");
            process::exit(2);
        }
    }
}

fn run_content_behavioral(args: &[String]) {
    // orqa-validation content behavioral <project-path>
    let project_path = PathBuf::from(&args[3]);

    if !project_path.exists() {
        eprintln!(
            "Error: project path does not exist: {}",
            project_path.display()
        );
        process::exit(2);
    }

    let graph = match build_artifact_graph(&project_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error building artifact graph: {e}");
            process::exit(2);
        }
    };

    let plugin_contributions = scan_plugin_manifests(&project_path);
    // Determine the rule type key from plugin contributions — the type whose
    // pipeline_category is "rule" is the source of behavioral rules.
    // Fall back to "rule" if no plugin declares one explicitly.
    let rule_type_key = plugin_contributions
        .artifact_types
        .iter()
        .find(|t| t.pipeline_category.as_deref() == Some("rule"))
        .map_or("rule", |t| t.key.as_str());

    match extract_behavioral_messages(&graph, &project_path, rule_type_key) {
        Ok(result) => {
            match serde_json::to_string_pretty(&result) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    eprintln!("Error serialising result: {e}");
                    process::exit(2);
                }
            }
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error extracting behavioral messages: {e}");
            process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Find the value of a `--flag <value>` pair in the argument list.
fn find_flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}

/// Convert integrity checks to enforcement events for the centralised log.
///
/// Only `SchemaViolation` findings are converted — they represent enforcement
/// actions. Other integrity categories are structural issues, not enforcement.
fn checks_to_enforcement_events(checks: &[IntegrityCheck]) -> Vec<EnforcementEvent> {
    checks
        .iter()
        .filter(|c| matches!(c.category, IntegrityCategory::SchemaViolation))
        .map(|c| {
            let result = match c.severity {
                IntegritySeverity::Error => EnforcementResult::Fail,
                IntegritySeverity::Warning => EnforcementResult::Warn,
                IntegritySeverity::Info => EnforcementResult::Pass,
            };
            EnforcementEvent {
                mechanism: "json-schema".to_owned(),
                check_type: "frontmatter".to_owned(),
                rule_id: None,
                artifact_id: Some(c.artifact_id.clone()),
                result,
                message: c.message.clone(),
            }
        })
        .collect()
}
