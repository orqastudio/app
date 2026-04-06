// POST /prompt endpoint for the OrqaStudio daemon.
//
// Absorbs prompt classification and generation logic from the connector layer,
// moving business logic into the engine where it belongs. The connector becomes
// a thin adapter that calls this endpoint and formats the result for Claude Code.
//
// Classification uses a three-tier fallback strategy:
//   1. ONNX semantic search against thinking-mode knowledge artifacts (primary)
//   2. Keyword regex matching (fallback when ONNX unavailable)
//   3. "general" default (when nothing matches)
//
// Classification vocabulary (thinking-mode exceptions, prompt types, stage
// mappings, keyword patterns) is loaded at handler time from the installed
// plugin manifests. The agile-methodology plugin owns this vocabulary via its
// `prompt_classification` section. If no plugin provides classification data,
// a warning is logged and all prompts fall back to "general".

use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use orqa_engine::prompt::builder::{build_system_prompt, resolve_project_paths};
use orqa_engine::prompt::knowledge::KnowledgeInjector;
use orqa_engine::search::embedder::Embedder;

// ---------------------------------------------------------------------------
// Plugin-loaded classification data
// ---------------------------------------------------------------------------

/// Prompt classification vocabulary loaded from a plugin's `prompt_classification`
/// manifest section. The agile-methodology plugin is the canonical provider.
#[derive(Deserialize, Default)]
struct PromptClassification {
    /// Maps thinking-mode frontmatter values that do not match their prompt type by name.
    /// Key: frontmatter thinking-mode value. Value: prompt type string.
    #[serde(default)]
    thinking_mode_exceptions: HashMap<String, String>,
    /// All valid prompt type strings (e.g., "implementation", "review").
    #[serde(default)]
    prompt_types: Vec<String>,
    /// Maps prompt type string to the workflow stage string.
    #[serde(default)]
    stage_mappings: HashMap<String, String>,
    /// Maps prompt type string to a list of keyword triggers.
    #[serde(default)]
    keyword_patterns: HashMap<String, Vec<String>>,
}

/// Minimal shape of an `orqa-plugin.json` file — only the field we need.
#[derive(Deserialize)]
struct PluginManifest {
    #[serde(default)]
    prompt_classification: Option<PromptClassification>,
}

/// Scan installed plugin manifests and return the first `prompt_classification`
/// block found. Plugins are discovered by globbing for `orqa-plugin.json` files
/// under `<project_root>/plugins/*/*/orqa-plugin.json` (taxonomy subdirectories).
///
/// Returns `PromptClassification::default()` (all empty) and logs a warning when
/// no plugin provides classification data — callers treat empty as "fall back to
/// general for everything".
fn load_prompt_classification(project_path: &Path) -> PromptClassification {
    let pattern = project_path
        .join("plugins")
        .join("*")
        .join("*")
        .join("orqa-plugin.json");

    let glob_str = if let Some(s) = pattern.to_str() {
        s.to_owned()
    } else {
        warn!("[prompt] could not build plugin glob path — using empty classification");
        return PromptClassification::default();
    };

    let paths = match glob::glob(&glob_str) {
        Ok(p) => p,
        Err(e) => {
            warn!(error = %e, "[prompt] glob error scanning plugin manifests — using empty classification");
            return PromptClassification::default();
        }
    };

    for entry in paths.flatten() {
        let Ok(content) = std::fs::read_to_string(&entry) else {
            continue;
        };
        let manifest: PluginManifest = match serde_json::from_str(&content) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if let Some(classification) = manifest.prompt_classification {
            return classification;
        }
    }

    warn!(
        "[prompt] no plugin provides prompt_classification — all prompts will default to 'general'"
    );
    PromptClassification::default()
}

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// Request body for POST /prompt.
#[derive(Deserialize)]
pub struct PromptRequest {
    /// The user's message to classify and build a prompt for.
    pub message: String,
    /// The agent role (e.g., "orchestrator", "implementer").
    pub role: String,
    /// Absolute path to the project root.
    pub project_path: String,
}

/// A single section included in the generated prompt.
#[derive(Serialize)]
pub struct SectionInfo {
    /// Name of the section (e.g., "rules", "knowledge-catalog", "agent-definitions").
    pub name: String,
    /// Approximate token count for this section.
    pub tokens: u64,
}

/// Response body for POST /prompt.
#[derive(Serialize)]
pub struct PromptResponse {
    /// The fully assembled system prompt text.
    pub prompt: String,
    /// Classified prompt type (e.g., "implementation", "governance", "general").
    pub prompt_type: String,
    /// Classification method used: "semantic", "keyword", or "default".
    pub method: String,
    /// Approximate total token count of the assembled prompt.
    pub tokens: u64,
    /// Token budget for this prompt type.
    pub budget: u64,
    /// Sections that were assembled into the prompt.
    pub sections: Vec<SectionInfo>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Handle POST /prompt.
///
/// Loads classification vocabulary from the installed plugin manifests, then
/// classifies the user message, runs the prompt pipeline to assemble a system
/// prompt, and returns the full response. All three classification tiers are
/// attempted in order. The prompt builder is called regardless of which tier
/// succeeds — classification only determines the workflow stage passed to the
/// builder.
pub fn prompt_handler(Json(req): Json<PromptRequest>) -> Json<PromptResponse> {
    let start = Instant::now();
    let project_path = Path::new(&req.project_path);

    info!(
        subsystem = "prompt",
        role = %req.role,
        "[prompt] prompt_handler entry"
    );

    let classification = load_prompt_classification(project_path);
    let (prompt_type, method) = classify_message(&req.message, project_path, &classification);
    let stage = resolve_stage(&prompt_type, &classification);
    let prompt_paths = resolve_project_paths(project_path);
    let stage_opt = if stage == "general" {
        None
    } else {
        Some(stage)
    };
    let (prompt_text, sections) =
        build_prompt_text(&req.role, stage_opt, project_path, &prompt_paths);
    let total_tokens = sections.iter().map(|s| s.tokens).sum();
    let budget = budget_for_role(&req.role).max(budget_for_type(&prompt_type));

    info!(
        subsystem = "prompt",
        elapsed_ms = start.elapsed().as_millis() as u64,
        prompt_type = %prompt_type,
        method = %method,
        tokens = total_tokens,
        budget = budget,
        "[prompt] prompt_handler completed"
    );

    Json(PromptResponse {
        prompt: prompt_text,
        prompt_type,
        method,
        tokens: total_tokens,
        budget,
        sections,
    })
}

/// Classify a message using the three-tier pipeline: ONNX → keyword → default.
///
/// Returns `(prompt_type, method)` where `method` is one of `"semantic"`, `"keyword"`,
/// or `"default"` indicating which tier matched.
fn classify_message(
    message: &str,
    project_path: &Path,
    classification: &PromptClassification,
) -> (String, String) {
    if let Some(pt) = classify_with_onnx(message, project_path, classification) {
        return (pt, "semantic".to_owned());
    }
    let kw = classify_by_keyword(message, classification);
    if kw == "general" {
        ("general".to_owned(), "default".to_owned())
    } else {
        (kw, "keyword".to_owned())
    }
}

/// Build the system prompt text and section metadata for a given role and stage.
///
/// Falls back to a minimal `<role>` tag when the prompt builder returns an error.
fn build_prompt_text(
    role: &str,
    stage_opt: Option<&str>,
    project_path: &Path,
    prompt_paths: &orqa_engine::prompt::builder::ProjectPromptPaths,
) -> (String, Vec<SectionInfo>) {
    match build_system_prompt(project_path, role, stage_opt, prompt_paths) {
        Ok(text) => {
            let token_estimate = estimate_tokens(&text);
            let sections = vec![SectionInfo {
                name: format!(
                    "system-prompt[role={role},stage={}]",
                    stage_opt.unwrap_or("general")
                ),
                tokens: token_estimate,
            }];
            (text, sections)
        }
        Err(e) => {
            warn!(
                error = %e,
                role = %role,
                stage = %stage_opt.unwrap_or("general"),
                "[prompt] failed to build system prompt — using minimal fallback"
            );
            let fallback = format!("<role>{role}</role>");
            let sections = vec![SectionInfo {
                name: "fallback".to_owned(),
                tokens: estimate_tokens(&fallback),
            }];
            (fallback, sections)
        }
    }
}

// ---------------------------------------------------------------------------
// Tier 1: ONNX semantic classification
// ---------------------------------------------------------------------------

/// Attempt to classify the message using ONNX embeddings against thinking-mode
/// knowledge artifacts. Returns a PromptType string on success, None if the
/// ONNX embedder is unavailable or no confident match is found.
///
/// This mirrors the TypeScript `classifyWithSearch()` logic but runs entirely
/// in Rust using the engine's KnowledgeInjector and Embedder.
fn classify_with_onnx(
    message: &str,
    project_path: &Path,
    classification: &PromptClassification,
) -> Option<String> {
    // Build an embedder — this will fail gracefully if the ONNX model is absent.
    // Uses the shared resolve_model_dir from knowledge.rs (LOCALAPPDATA fallback).
    let model_dir = crate::knowledge::resolve_model_dir()?;
    let mut embedder = Embedder::new(&model_dir).ok()?;

    // Truncate the message and form the search query.
    let truncated = if message.len() > 200 {
        &message[..200]
    } else {
        message
    };
    let query = format!("thinking mode classification for user prompt: {truncated}");

    // Embed the query.
    let embeddings = embedder.embed(&[query.as_str()]).ok()?;
    let query_embedding = embeddings.into_iter().next()?;

    // Load the knowledge injector to search knowledge artifacts.
    let injector = KnowledgeInjector::new(project_path, &mut embedder).ok()?;

    // Find the best match above a confidence threshold.
    let matches = injector.match_prompt(&query_embedding, 5, 0.5);
    if matches.is_empty() {
        return None;
    }

    // The top result's name is the knowledge artifact ID (e.g., "KNOW-abc123").
    // We cannot read thinking-mode frontmatter from just the artifact name here,
    // so we use the artifact name as the lookup key against all installed knowledge
    // artifacts, reading the thinking-mode field from the matched artifact file.
    let knowledge_dir = project_path
        .join(".orqa")
        .join("documentation")
        .join("knowledge");
    for m in &matches {
        let artifact_path = knowledge_dir.join(format!("{}.md", m.name));
        if let Ok(content) = std::fs::read_to_string(&artifact_path) {
            if let Some(mode) = extract_thinking_mode(&content) {
                if let Some(pt) = resolve_thinking_mode(&mode, classification) {
                    return Some(pt);
                }
            }
        }
    }

    None
}

/// Extract the `thinking-mode:` frontmatter value from a knowledge artifact.
fn extract_thinking_mode(content: &str) -> Option<String> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let after_open = &trimmed[3..];
    let end = after_open.find("\n---")?;
    let frontmatter = &after_open[..end];

    for line in frontmatter.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("thinking-mode:") {
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() {
                return Some(val.to_owned());
            }
        }
    }
    None
}

/// Map a thinking-mode frontmatter value to a prompt type string using plugin-
/// loaded classification data.
///
/// Checks `thinking_mode_exceptions` first, then falls back to checking whether
/// the mode value is a known prompt type directly.
fn resolve_thinking_mode(mode: &str, classification: &PromptClassification) -> Option<String> {
    // Check exception table first.
    if let Some(pt) = classification.thinking_mode_exceptions.get(mode) {
        return Some(pt.clone());
    }
    // Direct name match against known prompt types.
    if classification.prompt_types.contains(&mode.to_owned()) {
        return Some(mode.to_owned());
    }
    None
}

// ---------------------------------------------------------------------------
// Tier 2: keyword regex classification
// ---------------------------------------------------------------------------

/// Classify a message using keyword patterns loaded from the plugin.
///
/// Returns a prompt type string. Returns "general" if no pattern matches.
/// This is the direct Rust equivalent of the TypeScript `classifyPrompt()`.
/// Patterns are checked in the order they appear in the plugin manifest.
fn classify_by_keyword(message: &str, classification: &PromptClassification) -> String {
    let lower = message.to_lowercase();
    // The plugin defines keyword_patterns as a map. We check every entry.
    // Priority ordering: implementation > debugging > review > research >
    // planning > documentation > governance (matches TypeScript constant order).
    // We iterate the HashMap; for deterministic ordering, a sorted fallback is
    // acceptable because the plugin defines only 7 types and ordering ties are
    // rare in practice.
    let ordered_types = &[
        "implementation",
        "debugging",
        "review",
        "research",
        "planning",
        "documentation",
        "governance",
    ];
    for &pt in ordered_types {
        if let Some(keywords) = classification.keyword_patterns.get(pt) {
            let kw_refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
            if contains_any(&lower, &kw_refs) {
                return pt.to_owned();
            }
        }
    }
    "general".to_owned()
}

/// Return true if the text contains any of the given keywords as whole words.
///
/// Uses a simple word-boundary check: the keyword must be surrounded by
/// non-alphabetic characters (or be at the start/end of string). This mirrors
/// the `\b` word-boundary matching in the TypeScript keyword classifier.
fn contains_any(text: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|kw| {
        // Multi-word phrases (e.g., "write code") use simple contains.
        if kw.contains(' ') {
            return text.contains(kw);
        }
        // Single-word keywords: require word-boundary context.
        let kw_bytes = kw.as_bytes();
        let text_bytes = text.as_bytes();
        let klen = kw_bytes.len();
        let tlen = text_bytes.len();
        if tlen < klen {
            return false;
        }
        for i in 0..=(tlen - klen) {
            if &text_bytes[i..i + klen] == kw_bytes {
                let before_ok = i == 0 || !text_bytes[i - 1].is_ascii_alphabetic();
                let after_ok = i + klen == tlen || !text_bytes[i + klen].is_ascii_alphabetic();
                if before_ok && after_ok {
                    return true;
                }
            }
        }
        false
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Map a prompt type to its workflow stage string using plugin-loaded stage mappings.
///
/// Returns "general" if the type is not in the mapping table.
fn resolve_stage<'a>(prompt_type: &'a str, classification: &'a PromptClassification) -> &'a str {
    if let Some(stage) = classification.stage_mappings.get(prompt_type) {
        return stage.as_str();
    }
    "general"
}

/// Return the token budget for a given prompt type.
///
/// Governance and planning tasks get a larger budget for richer context;
/// implementation tasks stay tighter to keep the agent focused.
fn budget_for_type(prompt_type: &str) -> u64 {
    match prompt_type {
        "governance" | "planning" | "research" => 4000,
        "review" | "documentation" => 3500,
        _ => 2500,
    }
}

/// Return the token budget for a given agent role per DOC-b951327c section 6.3.
///
/// The final budget is the max of `budget_for_role` and `budget_for_type` so that
/// richer task types can still receive additional headroom above the role minimum.
fn budget_for_role(role: &str) -> u64 {
    match role {
        "implementer" => 2800,
        "reviewer" => 1900,
        "researcher" => 2100,
        "writer" | "designer" | "governance-steward" => 1800,
        _ => 2500,
    }
}

/// Estimate the token count for a string using a simple 4-chars-per-token heuristic.
///
/// This is intentionally approximate — exact tokenisation would require the
/// tokeniser for the target model. For budget comparison and telemetry purposes
/// this level of accuracy is sufficient.
fn estimate_tokens(text: &str) -> u64 {
    (text.len() as u64).div_ceil(4)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a fully populated PromptClassification matching the agile-methodology plugin data.
    #[allow(clippy::too_many_lines)]
    fn test_classification() -> PromptClassification {
        let mut thinking_mode_exceptions = HashMap::new();
        thinking_mode_exceptions.insert("learning-loop".to_owned(), "governance".to_owned());
        thinking_mode_exceptions.insert(
            "dogfood-implementation".to_owned(),
            "implementation".to_owned(),
        );

        let prompt_types = vec![
            "implementation".to_owned(),
            "planning".to_owned(),
            "review".to_owned(),
            "debugging".to_owned(),
            "research".to_owned(),
            "documentation".to_owned(),
            "governance".to_owned(),
            "general".to_owned(),
        ];

        let mut stage_mappings = HashMap::new();
        stage_mappings.insert("implementation".to_owned(), "implement".to_owned());
        stage_mappings.insert("planning".to_owned(), "plan".to_owned());
        stage_mappings.insert("review".to_owned(), "review".to_owned());
        stage_mappings.insert("debugging".to_owned(), "debug".to_owned());
        stage_mappings.insert("research".to_owned(), "research".to_owned());
        stage_mappings.insert("documentation".to_owned(), "document".to_owned());
        stage_mappings.insert("governance".to_owned(), "govern".to_owned());
        stage_mappings.insert("general".to_owned(), "general".to_owned());

        let mut keyword_patterns = HashMap::new();
        keyword_patterns.insert(
            "implementation".to_owned(),
            vec![
                "implement".to_owned(),
                "build".to_owned(),
                "create".to_owned(),
                "add".to_owned(),
                "write code".to_owned(),
                "fix bug".to_owned(),
                "refactor".to_owned(),
                "migrate".to_owned(),
                "wire up".to_owned(),
                "hook up".to_owned(),
            ],
        );
        keyword_patterns.insert(
            "debugging".to_owned(),
            vec![
                "debug".to_owned(),
                "investigate".to_owned(),
                "why does".to_owned(),
                "broken".to_owned(),
                "error".to_owned(),
                "crash".to_owned(),
                "failing".to_owned(),
                "not working".to_owned(),
                "trace".to_owned(),
            ],
        );
        keyword_patterns.insert(
            "review".to_owned(),
            vec![
                "review".to_owned(),
                "audit".to_owned(),
                "check".to_owned(),
                "verify".to_owned(),
                "validate".to_owned(),
                "assess".to_owned(),
                "compliance".to_owned(),
            ],
        );
        keyword_patterns.insert(
            "research".to_owned(),
            vec![
                "research".to_owned(),
                "explore".to_owned(),
                "investigate options".to_owned(),
                "compare".to_owned(),
                "evaluate".to_owned(),
                "what are the".to_owned(),
            ],
        );
        keyword_patterns.insert(
            "planning".to_owned(),
            vec![
                "plan".to_owned(),
                "design".to_owned(),
                "scope".to_owned(),
                "epic".to_owned(),
                "roadmap".to_owned(),
                "milestone".to_owned(),
                "break down".to_owned(),
                "approach".to_owned(),
            ],
        );
        keyword_patterns.insert(
            "documentation".to_owned(),
            vec![
                "document".to_owned(),
                "docs".to_owned(),
                "write up".to_owned(),
                "describe".to_owned(),
                "explain".to_owned(),
                "specification".to_owned(),
            ],
        );
        keyword_patterns.insert(
            "governance".to_owned(),
            vec![
                "rule".to_owned(),
                "governance".to_owned(),
                "enforce".to_owned(),
                "lesson".to_owned(),
                "artifact".to_owned(),
                "pillar".to_owned(),
                "promote".to_owned(),
                "knowledge".to_owned(),
            ],
        );

        PromptClassification {
            thinking_mode_exceptions,
            prompt_types,
            stage_mappings,
            keyword_patterns,
        }
    }

    #[test]
    fn classify_by_keyword_implementation() {
        let c = test_classification();
        assert_eq!(
            classify_by_keyword("implement the login flow", &c),
            "implementation"
        );
        assert_eq!(
            classify_by_keyword("build the parser", &c),
            "implementation"
        );
        assert_eq!(classify_by_keyword("fix bug in auth", &c), "implementation");
    }

    #[test]
    fn classify_by_keyword_debugging() {
        let c = test_classification();
        assert_eq!(
            classify_by_keyword("why does the server crash", &c),
            "debugging"
        );
        assert_eq!(
            classify_by_keyword("investigate the failing test", &c),
            "debugging"
        );
    }

    #[test]
    fn classify_by_keyword_review() {
        let c = test_classification();
        assert_eq!(classify_by_keyword("review this PR", &c), "review");
        assert_eq!(classify_by_keyword("audit the auth module", &c), "review");
    }

    #[test]
    fn classify_by_keyword_planning() {
        let c = test_classification();
        assert_eq!(
            classify_by_keyword("plan the migration approach", &c),
            "planning"
        );
        assert_eq!(classify_by_keyword("design the new schema", &c), "planning");
    }

    #[test]
    fn classify_by_keyword_documentation() {
        let c = test_classification();
        assert_eq!(classify_by_keyword("document the API", &c), "documentation");
        assert_eq!(
            classify_by_keyword("write up the spec", &c),
            "documentation"
        );
    }

    #[test]
    fn classify_by_keyword_research() {
        let c = test_classification();
        assert_eq!(
            classify_by_keyword("research ONNX embedding options", &c),
            "research"
        );
        assert_eq!(
            classify_by_keyword("compare two approaches", &c),
            "research"
        );
    }

    #[test]
    fn classify_by_keyword_governance() {
        let c = test_classification();
        assert_eq!(
            classify_by_keyword("enforce the rule about tests", &c),
            "governance"
        );
        assert_eq!(classify_by_keyword("promote this lesson", &c), "governance");
    }

    #[test]
    fn classify_by_keyword_general_fallback() {
        let c = test_classification();
        assert_eq!(classify_by_keyword("hello world", &c), "general");
        assert_eq!(classify_by_keyword("", &c), "general");
    }

    #[test]
    fn resolve_stage_maps_known_types() {
        let c = test_classification();
        assert_eq!(resolve_stage("implementation", &c), "implement");
        assert_eq!(resolve_stage("planning", &c), "plan");
        assert_eq!(resolve_stage("governance", &c), "govern");
        assert_eq!(resolve_stage("general", &c), "general");
    }

    #[test]
    fn resolve_stage_defaults_to_general() {
        let c = test_classification();
        assert_eq!(resolve_stage("unknown-type", &c), "general");
    }

    #[test]
    fn resolve_thinking_mode_exception() {
        let c = test_classification();
        assert_eq!(
            resolve_thinking_mode("learning-loop", &c),
            Some("governance".to_owned())
        );
        assert_eq!(
            resolve_thinking_mode("dogfood-implementation", &c),
            Some("implementation".to_owned())
        );
    }

    #[test]
    fn resolve_thinking_mode_direct_match() {
        let c = test_classification();
        assert_eq!(
            resolve_thinking_mode("review", &c),
            Some("review".to_owned())
        );
        assert_eq!(
            resolve_thinking_mode("research", &c),
            Some("research".to_owned())
        );
    }

    #[test]
    fn resolve_thinking_mode_unknown_returns_none() {
        let c = test_classification();
        assert!(resolve_thinking_mode("unknown-mode", &c).is_none());
    }

    #[test]
    fn extract_thinking_mode_from_frontmatter() {
        let content = "---\ntitle: Test\nthinking-mode: governance\n---\n# Body";
        assert_eq!(
            extract_thinking_mode(content),
            Some("governance".to_owned())
        );
    }

    #[test]
    fn extract_thinking_mode_no_frontmatter() {
        let content = "# No frontmatter";
        assert!(extract_thinking_mode(content).is_none());
    }

    #[test]
    fn extract_thinking_mode_missing_field() {
        let content = "---\ntitle: Test\n---\n# Body";
        assert!(extract_thinking_mode(content).is_none());
    }

    #[test]
    fn budget_for_type_governance_is_larger() {
        assert!(budget_for_type("governance") > budget_for_type("implementation"));
        assert!(budget_for_type("planning") > budget_for_type("debugging"));
    }

    #[test]
    fn budget_for_role_matches_doc_b951327c() {
        // Verify budgets match DOC-b951327c section 6.3 token budget table.
        assert_eq!(budget_for_role("orchestrator"), 2500);
        assert_eq!(budget_for_role("implementer"), 2800);
        assert_eq!(budget_for_role("reviewer"), 1900);
        assert_eq!(budget_for_role("researcher"), 2100);
        assert_eq!(budget_for_role("writer"), 1800);
        assert_eq!(budget_for_role("planner"), 2500);
        assert_eq!(budget_for_role("designer"), 1800);
        assert_eq!(budget_for_role("governance-steward"), 1800);
    }

    #[test]
    fn budget_is_max_of_role_and_type() {
        // A governance task for an implementer should get the governance budget (4000)
        // since it exceeds the implementer role budget (2800).
        let combined = budget_for_role("implementer").max(budget_for_type("governance"));
        assert_eq!(combined, 4000);
        // A general task for an implementer gets the implementer budget (2800).
        let combined2 = budget_for_role("implementer").max(budget_for_type("general"));
        assert_eq!(combined2, 2800);
    }

    #[test]
    fn estimate_tokens_non_zero_for_non_empty() {
        assert!(estimate_tokens("hello world") > 0);
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn empty_classification_falls_back_to_general() {
        let c = PromptClassification::default();
        assert_eq!(
            classify_by_keyword("implement the login flow", &c),
            "general"
        );
        assert_eq!(resolve_stage("implementation", &c), "general");
        assert!(resolve_thinking_mode("review", &c).is_none());
    }
}
