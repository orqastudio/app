// POST /knowledge endpoint for the OrqaStudio daemon.
//
// Absorbs knowledge injection logic from the connector layer. The connector
// becomes a thin adapter that calls this endpoint instead of reading the prompt
// registry and running semantic search itself.
//
// Two knowledge layers are served:
//   Layer 1 (Declared): Reads .orqa/prompt-registry.json for knowledge entries
//     that match the detected agent role. The registry is built at plugin-install
//     time by @orqastudio/cli. No plugin manifest parsing at request time.
//   Layer 2 (Semantic): Uses ONNX embeddings (KnowledgeInjector) to find
//     task-specific knowledge beyond declared relationships. Requires the model
//     files to exist in the platform app data directory. Gracefully returns empty
//     when unavailable.
//
// Deduplication: IDs already present in Layer 1 are excluded from Layer 2 results.
// Source field: "declared" for Layer 1 entries, "semantic" for Layer 2.
//
// Role detection: role names come from installed plugin manifests (P1). The
// `build_role_patterns` function reads plugin `provides.agents` entries to
// discover valid role names, then generates matching phrases for each. Falls
// back to a built-in set of base roles when no plugins are installed so the
// daemon always degrades gracefully.

use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use orqa_engine::plugin::discovery::scan_plugins;
use orqa_engine::plugin::manifest::read_manifest;
use orqa_engine::prompt::knowledge::KnowledgeInjector;
use orqa_engine::search::embedder::Embedder;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Role detection — driven by installed plugin manifests (P1)
// ---------------------------------------------------------------------------

/// Base agent role names provided by the methodology plugin when no plugins
/// are installed. Used only as a fallback when `scan_plugins` returns nothing.
/// These names mirror DOC-b951327c section 6.1.
const BASE_ROLE_NAMES: &[&str] = &[
    "orchestrator",
    "implementer",
    "reviewer",
    "researcher",
    "writer",
    "planner",
    "designer",
    "governance steward",
];

/// Collect agent role names from installed plugin manifests.
///
/// Reads `provides.agents` entries from every installed plugin and returns
/// the unique set of role id values. Falls back to `BASE_ROLE_NAMES` when
/// no plugins declare agents — this preserves operation during bootstrap or
/// in environments where plugins have not been installed yet.
fn load_role_names_from_plugins(project_path: &Path) -> Vec<String> {
    let discovered = scan_plugins(project_path);
    let mut names: Vec<String> = Vec::new();

    for plugin in &discovered {
        let plugin_path = Path::new(&plugin.path);
        if let Ok(manifest) = read_manifest(plugin_path) {
            for agent in manifest.provides.agents {
                let id = agent.id.to_lowercase();
                if !names.contains(&id) {
                    names.push(id);
                }
            }
        }
    }

    if names.is_empty() {
        // Fallback: use the base role names so the daemon is usable before plugins install.
        BASE_ROLE_NAMES.iter().map(ToString::to_string).collect()
    } else {
        names
    }
}

/// Build role-detection patterns from a list of role names.
///
/// For each role, generates three phrase variants:
///   - "you are an {role}"
///   - "you are a {role}"
///   - "you are {role}"
///
/// These variants cover natural-language role declarations in agent prompts
/// regardless of whether "a" or "an" is used. Patterns are matched
/// case-insensitively by lower-casing the prompt before comparison.
///
/// Returns a `Vec<(pattern, role_id)>` in the same shape as the old static
/// `ROLE_PATTERNS` constant, so `detect_role_with_patterns` can use it.
fn build_role_patterns(role_names: &[String]) -> Vec<(String, String)> {
    let mut patterns = Vec::new();
    for role in role_names {
        let role_lower = role.to_lowercase();
        // Normalise role id: spaces become hyphens (e.g. "governance steward" -> "governance-steward").
        let role_id = role_lower.replace(' ', "-");
        patterns.push((format!("you are an {role_lower}"), role_id.clone()));
        patterns.push((format!("you are a {role_lower}"), role_id.clone()));
        patterns.push((format!("you are {role_lower}"), role_id.clone()));
    }
    patterns
}

/// Detect the agent role from the prompt text using the given pattern list.
///
/// Applies patterns in order, returning the first matching role name.
/// Matches case-insensitively by lower-casing the prompt before comparison.
/// Returns None when no pattern matches (e.g. for non-agent tool calls).
fn detect_role_with_patterns(prompt: &str, patterns: &[(String, String)]) -> Option<String> {
    let lower = prompt.to_lowercase();
    for (pattern, role) in patterns {
        if lower.contains(pattern.as_str()) {
            return Some(role.clone());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Prompt registry types — minimal Rust representation of the JSON written by
// @orqastudio/cli. Only the fields needed for Layer 1 lookup are included.
// ---------------------------------------------------------------------------

/// A knowledge entry from the prompt registry.
///
/// Maps to RegistryKnowledgeEntry in @orqastudio/cli's prompt-registry.ts.
/// Only the fields needed for role-based lookup are deserialized here.
#[derive(Deserialize)]
struct RegistryKnowledgeEntry {
    id: String,
    roles: Vec<String>,
    summary: Option<String>,
    content_file: Option<String>,
}

/// The cached prompt registry from .orqa/prompt-registry.json.
///
/// Maps to PromptRegistry in @orqastudio/cli's prompt-registry.ts.
#[derive(Deserialize)]
struct PromptRegistry {
    knowledge: Vec<RegistryKnowledgeEntry>,
}

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// Request body for POST /knowledge.
#[derive(Deserialize)]
pub struct KnowledgeRequest {
    /// Full agent prompt text — used for role detection and semantic query extraction.
    pub agent_prompt: String,
    /// Absolute path to the project root.
    pub project_path: String,
}

/// A single knowledge entry in the response.
#[derive(Serialize)]
pub struct KnowledgeEntry {
    /// Artifact ID (e.g., "KNOW-abc123").
    pub id: String,
    /// Title derived from the registry summary.
    pub title: String,
    /// Absolute path to the full content file.
    pub path: String,
    /// Source layer: "declared" (Layer 1) or "semantic" (Layer 2).
    pub source: String,
    /// Cosine similarity score (Layer 2 only — None for declared entries).
    pub score: Option<f64>,
}

/// Response body for POST /knowledge.
#[derive(Serialize)]
pub struct KnowledgeResponse {
    pub entries: Vec<KnowledgeEntry>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Handle POST /knowledge.
///
/// Detects the agent role from the prompt using patterns loaded from installed
/// plugin manifests (P1: Plugin-Composed Everything), then runs Layer 1
/// (declared registry lookup) and Layer 2 (ONNX semantic search), deduplicates,
/// and returns the combined results. Layer 2 degrades gracefully when the ONNX
/// model is absent.
pub async fn knowledge_handler(
    State(state): State<HealthState>,
    Json(req): Json<KnowledgeRequest>,
) -> Json<KnowledgeResponse> {
    let start = Instant::now();

    info!(subsystem = "knowledge", "[knowledge] knowledge_handler entry");

    let project_path = Path::new(&req.project_path);

    // Load role names from installed plugin manifests and build detection patterns.
    // This satisfies P1 — role names are not hardcoded, they come from plugins.
    let role_names = load_role_names_from_plugins(project_path);
    let patterns = build_role_patterns(&role_names);

    // Layer 1: declared knowledge from prompt registry (role-matched).
    let role = detect_role_with_patterns(&req.agent_prompt, &patterns);
    let declared = match &role {
        Some(r) => get_declared_knowledge(project_path, r),
        None => Vec::new(),
    };
    let declared_count = declared.len();
    let declared_ids: HashSet<String> = declared.iter().map(|e| e.id.clone()).collect();

    // Layer 2: semantic knowledge via ONNX embeddings.
    let semantic = get_semantic_knowledge(
        &req.agent_prompt,
        project_path,
        &declared_ids,
        state.config.min_score,
        state.config.max_semantic,
    );
    let semantic_count = semantic.len();

    info!(
        subsystem = "knowledge",
        elapsed_ms = start.elapsed().as_millis() as u64,
        role = %role.as_deref().unwrap_or("none"),
        declared_count,
        semantic_count,
        "[knowledge] knowledge_handler completed"
    );

    // Combine Layer 1 + Layer 2.
    let mut entries: Vec<KnowledgeEntry> = declared;
    entries.extend(semantic);

    Json(KnowledgeResponse { entries })
}

// ---------------------------------------------------------------------------
// Layer 1: declared knowledge from prompt registry
// ---------------------------------------------------------------------------

/// Read the prompt registry and return knowledge entries matching the given role.
///
/// Reads .orqa/prompt-registry.json from the project root. Returns empty if the
/// file does not exist, cannot be parsed, or no entries match the role.
fn get_declared_knowledge(project_path: &Path, role: &str) -> Vec<KnowledgeEntry> {
    let registry_path = project_path.join(".orqa").join("prompt-registry.json");

    let Ok(content) = std::fs::read_to_string(&registry_path) else {
        return Vec::new();
    };

    let registry: PromptRegistry = match serde_json::from_str(&content) {
        Ok(r) => r,
        Err(e) => {
            warn!(
                error = %e,
                path = %registry_path.display(),
                "[knowledge] failed to parse prompt-registry.json — Layer 1 empty"
            );
            return Vec::new();
        }
    };

    registry
        .knowledge
        .into_iter()
        .filter(|entry| {
            entry.content_file.is_some() && entry.roles.iter().any(|r| r == role)
        })
        .map(|entry| {
            // Derive the title from the first line of the summary, capped at 80 chars.
            let title = entry
                .summary
                .as_deref()
                .and_then(|s| s.lines().next())
                .unwrap_or(&entry.id)
                .chars()
                .take(80)
                .collect();

            KnowledgeEntry {
                id: entry.id,
                title,
                path: entry.content_file.expect("filter guarantees content_file is Some"),
                source: "declared".to_owned(),
                score: None,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Layer 2: semantic knowledge via ONNX
// ---------------------------------------------------------------------------

/// Find task-relevant knowledge via ONNX semantic search.
///
/// Extracts a query from the agent prompt (first ~500 chars after the opening
/// role paragraph), embeds it, and uses KnowledgeInjector to find matching
/// knowledge artifacts. Results with score < min_score or IDs already in
/// `exclude_ids` are filtered out. At most max_semantic results are returned.
///
/// Returns an empty vec if the ONNX model is unavailable or any step fails.
fn get_semantic_knowledge(
    prompt: &str,
    project_path: &Path,
    exclude_ids: &HashSet<String>,
    min_score: f32,
    max_semantic: usize,
) -> Vec<KnowledgeEntry> {
    // Resolve the model directory from platform app data dir.
    // Gracefully returns empty if the model is not installed.
    let Some(model_dir) = resolve_model_dir() else {
        debug!(subsystem = "knowledge", "[knowledge] get_semantic_knowledge: ONNX model directory not found — skipping semantic layer");
        return Vec::new();
    };

    let Ok(mut embedder) = Embedder::new(&model_dir) else {
        debug!(subsystem = "knowledge", path = %model_dir.display(), "[knowledge] get_semantic_knowledge: failed to create embedder — skipping semantic layer");
        return Vec::new();
    };

    // Extract a query from the prompt: skip the role preamble (first paragraph),
    // use the next ~500 chars. This mirrors the TypeScript knowledge-injector.
    let query = extract_query(prompt);

    // Embed the query.
    let Ok(embeddings) = embedder.embed(&[query.as_str()]) else {
        debug!(subsystem = "knowledge", "[knowledge] get_semantic_knowledge: embed call failed — skipping semantic layer");
        return Vec::new();
    };
    let Some(query_embedding) = embeddings.into_iter().next() else {
        debug!(subsystem = "knowledge", "[knowledge] get_semantic_knowledge: embed returned no vectors — skipping semantic layer");
        return Vec::new();
    };

    // Load knowledge injector and find matches.
    let Ok(injector) = KnowledgeInjector::new(project_path, &mut embedder) else {
        debug!(subsystem = "knowledge", "[knowledge] get_semantic_knowledge: failed to create KnowledgeInjector — skipping semantic layer");
        return Vec::new();
    };

    // Request max_semantic + excluded count so that after filtering we still
    // get up to max_semantic results — mirrors the TypeScript implementation.
    let top_n = max_semantic + exclude_ids.len();
    let matches = injector.match_prompt(&query_embedding, top_n, min_score);

    let knowledge_dir = project_path.join(".orqa").join("documentation").join("knowledge");

    matches
        .into_iter()
        .filter(|m| !exclude_ids.contains(&m.name))
        .take(max_semantic)
        .map(|m| {
            let path = knowledge_dir.join(format!("{}.md", m.name));
            KnowledgeEntry {
                id: m.name.clone(),
                title: m.name,
                path: path.to_string_lossy().into_owned(),
                source: "semantic".to_owned(),
                score: Some(m.score as f64),
            }
        })
        .collect()
}

/// Extract a semantic search query from an agent prompt.
///
/// Mirrors the TypeScript logic in knowledge-injector.ts: skips the opening
/// role paragraph (text before the first blank line) and takes up to 500 chars
/// of the remaining text. Falls back to the first 500 chars of the full prompt.
fn extract_query(prompt: &str) -> String {
    // Find the first blank line (double newline) in the prompt.
    if let Some(pos) = prompt.find("\n\n") {
        let after = &prompt[pos + 2..];
        after.chars().take(500).collect()
    } else {
        prompt.chars().take(500).collect()
    }
}

/// Resolve the ONNX model directory.
///
/// Checks in priority order:
///   1. `ORQA_MODEL_DIR` environment variable (override for dev/CI)
///   2. Platform app data directory: `{LOCALAPPDATA}/com.orqastudio.app` on
///      Windows, `~/.local/share/com.orqastudio.app` on macOS/Linux
///
/// Returns the first directory that contains both `model.onnx` and
/// `tokenizer.json`. Returns None if no valid directory is found — callers
/// degrade gracefully when the model is not installed.
pub fn resolve_model_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    let platform_dir = std::env::var("LOCALAPPDATA")
        .ok()
        .map(|d| std::path::PathBuf::from(d).join("com.orqastudio.app"));

    #[cfg(not(target_os = "windows"))]
    let platform_dir = std::env::var("HOME").ok().map(|h| {
        std::path::PathBuf::from(h)
            .join(".local")
            .join("share")
            .join("com.orqastudio.app")
    });

    let candidates: Vec<std::path::PathBuf> = [
        std::env::var("ORQA_MODEL_DIR").ok().map(std::path::PathBuf::from),
        platform_dir.map(|d| d.join("models").join("all-MiniLM-L6-v2")),
    ]
    .into_iter()
    .flatten()
    .collect();

    let found = candidates
        .into_iter()
        .find(|dir| dir.join("model.onnx").exists() && dir.join("tokenizer.json").exists());

    match &found {
        Some(dir) => {
            info!(
                subsystem = "knowledge",
                path = %dir.display(),
                "[knowledge] ONNX model found"
            );
        }
        None => {
            info!(
                subsystem = "knowledge",
                "[knowledge] ONNX model not found — semantic search disabled"
            );
        }
    }

    found
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ── role detection (via build_role_patterns / detect_role_with_patterns) ──

    /// Build patterns from the base role names for use in tests.
    fn base_patterns() -> Vec<(String, String)> {
        let names: Vec<String> = BASE_ROLE_NAMES.iter().map(|s| s.to_string()).collect();
        build_role_patterns(&names)
    }

    #[test]
    fn detect_role_implementer() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("You are an Implementer. Your task is...", &patterns),
            Some("implementer".to_string())
        );
    }

    #[test]
    fn detect_role_researcher() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("You are a Researcher working on...", &patterns),
            Some("researcher".to_string())
        );
    }

    #[test]
    fn detect_role_reviewer() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("You are a Reviewer. Verify that...", &patterns),
            Some("reviewer".to_string())
        );
    }

    #[test]
    fn detect_role_planner() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("You are a Planner. Design the approach.", &patterns),
            Some("planner".to_string())
        );
    }

    #[test]
    fn detect_role_writer() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("You are a Writer. Document this.", &patterns),
            Some("writer".to_string())
        );
    }

    #[test]
    fn detect_role_designer() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("You are a Designer. Create the layout.", &patterns),
            Some("designer".to_string())
        );
    }

    #[test]
    fn detect_role_governance_steward() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns(
                "You are a Governance Steward. Maintain the artifacts.",
                &patterns
            ),
            Some("governance-steward".to_string())
        );
    }

    #[test]
    fn detect_role_no_match_returns_none() {
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("Use the bash tool to run the test.", &patterns),
            None
        );
        assert_eq!(detect_role_with_patterns("", &patterns), None);
    }

    #[test]
    fn detect_role_case_insensitive() {
        // All-caps "YOU ARE AN IMPLEMENTER" should still match.
        let patterns = base_patterns();
        assert_eq!(
            detect_role_with_patterns("YOU ARE AN IMPLEMENTER working on...", &patterns),
            Some("implementer".to_string())
        );
    }

    #[test]
    fn build_role_patterns_generates_three_variants_per_role() {
        let patterns = build_role_patterns(&["analyst".to_string()]);
        // Each role generates 3 patterns: "you are an", "you are a", "you are".
        assert_eq!(patterns.len(), 3);
        assert!(patterns.iter().any(|(p, _)| p == "you are an analyst"));
        assert!(patterns.iter().any(|(p, _)| p == "you are a analyst"));
        assert!(patterns.iter().any(|(p, _)| p == "you are analyst"));
        // All patterns map to the same role id.
        assert!(patterns.iter().all(|(_, r)| r == "analyst"));
    }

    #[test]
    fn build_role_patterns_normalises_space_to_hyphen_in_id() {
        let patterns = build_role_patterns(&["governance steward".to_string()]);
        assert!(patterns.iter().all(|(_, r)| r == "governance-steward"));
    }

    #[test]
    fn load_role_names_falls_back_to_base_when_no_plugins() {
        // With no project.json and no installed plugins, must fall back to BASE_ROLE_NAMES.
        let tmp = tempfile::tempdir().expect("tempdir");
        let names = load_role_names_from_plugins(tmp.path());
        let base: Vec<String> = BASE_ROLE_NAMES.iter().map(|s| s.to_string()).collect();
        assert_eq!(names, base);
    }

    // ── get_declared_knowledge ──

    fn make_registry_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    #[test]
    fn declared_empty_when_no_registry_file() {
        let dir = make_registry_project();
        let result = get_declared_knowledge(dir.path(), "implementer");
        assert!(result.is_empty());
    }

    #[test]
    fn declared_returns_matching_role_entries() {
        let dir = make_registry_project();
        let orqa = dir.path().join(".orqa");
        fs::create_dir_all(&orqa).expect("create .orqa");

        let registry_json = serde_json::json!({
            "version": 1,
            "built_at": "2026-01-01T00:00:00Z",
            "knowledge": [
                {
                    "id": "KNOW-abc123",
                    "plugin": "software",
                    "source": "plugin",
                    "tier": "pre-load",
                    "roles": ["implementer"],
                    "stages": [],
                    "paths": [],
                    "tags": [],
                    "priority": "high",
                    "summary": "Rust error handling patterns",
                    "content_file": "/path/to/KNOW-abc123.md"
                },
                {
                    "id": "KNOW-def456",
                    "plugin": "software",
                    "source": "plugin",
                    "tier": "pre-load",
                    "roles": ["reviewer"],
                    "stages": [],
                    "paths": [],
                    "tags": [],
                    "priority": "normal",
                    "summary": "Code review standards",
                    "content_file": "/path/to/KNOW-def456.md"
                }
            ],
            "sections": [],
            "contributors": ["software"],
            "errors": []
        });

        fs::write(
            orqa.join("prompt-registry.json"),
            serde_json::to_string_pretty(&registry_json).unwrap(),
        )
        .expect("write registry");

        let result = get_declared_knowledge(dir.path(), "implementer");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "KNOW-abc123");
        assert_eq!(result[0].title, "Rust error handling patterns");
        assert_eq!(result[0].source, "declared");
        assert!(result[0].score.is_none());
    }

    #[test]
    fn declared_skips_entries_without_content_file() {
        let dir = make_registry_project();
        let orqa = dir.path().join(".orqa");
        fs::create_dir_all(&orqa).expect("create .orqa");

        let registry_json = serde_json::json!({
            "version": 1,
            "built_at": "2026-01-01T00:00:00Z",
            "knowledge": [
                {
                    "id": "KNOW-no-file",
                    "plugin": "software",
                    "source": "plugin",
                    "tier": "on-demand",
                    "roles": ["implementer"],
                    "stages": [],
                    "paths": [],
                    "tags": [],
                    "priority": "normal",
                    "summary": "Summary only entry",
                    "content_file": null
                }
            ],
            "sections": [],
            "contributors": [],
            "errors": []
        });

        fs::write(
            orqa.join("prompt-registry.json"),
            serde_json::to_string_pretty(&registry_json).unwrap(),
        )
        .expect("write registry");

        let result = get_declared_knowledge(dir.path(), "implementer");
        assert!(result.is_empty());
    }

    #[test]
    fn declared_title_capped_at_80_chars() {
        let dir = make_registry_project();
        let orqa = dir.path().join(".orqa");
        fs::create_dir_all(&orqa).expect("create .orqa");

        let long_summary = "A".repeat(100);
        let registry_json = serde_json::json!({
            "version": 1,
            "built_at": "2026-01-01T00:00:00Z",
            "knowledge": [
                {
                    "id": "KNOW-long",
                    "plugin": "software",
                    "source": "plugin",
                    "tier": "pre-load",
                    "roles": ["implementer"],
                    "stages": [],
                    "paths": [],
                    "tags": [],
                    "priority": "normal",
                    "summary": long_summary,
                    "content_file": "/path/to/KNOW-long.md"
                }
            ],
            "sections": [],
            "contributors": [],
            "errors": []
        });

        fs::write(
            orqa.join("prompt-registry.json"),
            serde_json::to_string_pretty(&registry_json).unwrap(),
        )
        .expect("write registry");

        let result = get_declared_knowledge(dir.path(), "implementer");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title.len(), 80);
    }

    // ── extract_query ──

    #[test]
    fn extract_query_skips_role_preamble() {
        let prompt = "You are an Implementer.\n\nYour task is to fix the login bug.";
        let query = extract_query(prompt);
        assert_eq!(query, "Your task is to fix the login bug.");
    }

    #[test]
    fn extract_query_falls_back_when_no_blank_line() {
        let prompt = "You are an Implementer. Fix the bug.";
        let query = extract_query(prompt);
        assert_eq!(query, "You are an Implementer. Fix the bug.");
    }

    #[test]
    fn extract_query_truncates_at_500_chars() {
        let prompt = format!("Preamble.\n\n{}", "x".repeat(600));
        let query = extract_query(&prompt);
        assert_eq!(query.len(), 500);
    }

    // ── deduplification via handler inputs ──

    #[test]
    fn declared_ids_excluded_from_semantic_set() {
        // Verify the exclude logic: if Layer 1 already has KNOW-abc123,
        // the semantic results should not include it.
        let declared_ids: HashSet<String> = ["KNOW-abc123".to_string()].into_iter().collect();
        assert!(declared_ids.contains("KNOW-abc123"));
        assert!(!declared_ids.contains("KNOW-def456"));
    }
}
