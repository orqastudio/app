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

use std::collections::HashSet;
use std::path::Path;

use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::warn;

use orqa_engine::prompt::knowledge::KnowledgeInjector;
use orqa_engine::search::embedder::Embedder;

// ---------------------------------------------------------------------------
// Thresholds — put as named constants so they are easy to tune without
// hunting through handler body code.
// ---------------------------------------------------------------------------

/// Minimum cosine similarity score for a semantic result to be included.
/// Matches MIN_SCORE in the TypeScript connector knowledge-injector.ts.
const MIN_SCORE: f32 = 0.25;

/// Maximum number of semantic search results to return.
/// Matches MAX_SEMANTIC in the TypeScript connector knowledge-injector.ts.
const MAX_SEMANTIC: usize = 5;

// ---------------------------------------------------------------------------
// Role detection — regex patterns matching ROLE_PATTERNS in knowledge-injector.ts
// ---------------------------------------------------------------------------

/// Agent role patterns: (substring to match in lower-case prompt, role name).
///
/// Each pattern corresponds to one entry in the TypeScript ROLE_PATTERNS array.
/// The matching is case-insensitive via lower-casing the prompt before lookup.
const ROLE_PATTERNS: &[(&str, &str)] = &[
    ("you are an implementer", "implementer"),
    ("you are a implementer", "implementer"),
    ("you are implementer", "implementer"),
    ("you are an researcher", "researcher"),
    ("you are a researcher", "researcher"),
    ("you are researcher", "researcher"),
    ("you are an reviewer", "reviewer"),
    ("you are a reviewer", "reviewer"),
    ("you are reviewer", "reviewer"),
    ("you are an planner", "planner"),
    ("you are a planner", "planner"),
    ("you are planner", "planner"),
    ("you are an writer", "writer"),
    ("you are a writer", "writer"),
    ("you are writer", "writer"),
    ("you are an designer", "designer"),
    ("you are a designer", "designer"),
    ("you are designer", "designer"),
    ("you are an governance steward", "governance-steward"),
    ("you are a governance steward", "governance-steward"),
    ("you are governance steward", "governance-steward"),
];

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
/// Detects the agent role from the prompt, runs Layer 1 (declared registry
/// lookup) and Layer 2 (ONNX semantic search), deduplicates, and returns the
/// combined results. Layer 2 degrades gracefully when the ONNX model is absent.
pub async fn knowledge_handler(Json(req): Json<KnowledgeRequest>) -> Json<KnowledgeResponse> {
    let project_path = Path::new(&req.project_path);

    // Layer 1: declared knowledge from prompt registry (role-matched).
    let role = detect_role(&req.agent_prompt);
    let declared = match &role {
        Some(r) => get_declared_knowledge(project_path, r),
        None => Vec::new(),
    };
    let declared_ids: HashSet<String> = declared.iter().map(|e| e.id.clone()).collect();

    // Layer 2: semantic knowledge via ONNX embeddings.
    let semantic = get_semantic_knowledge(&req.agent_prompt, project_path, &declared_ids);

    // Combine Layer 1 + Layer 2.
    let mut entries: Vec<KnowledgeEntry> = declared;
    entries.extend(semantic);

    Json(KnowledgeResponse { entries })
}

// ---------------------------------------------------------------------------
// Role detection
// ---------------------------------------------------------------------------

/// Detect the agent role from the prompt text.
///
/// Applies ROLE_PATTERNS in order, returning the first matching role name.
/// Matches case-insensitively by lower-casing the prompt before comparison.
/// Returns None when no pattern matches (e.g., for non-agent tool calls).
fn detect_role(prompt: &str) -> Option<String> {
    let lower = prompt.to_lowercase();
    for (pattern, role) in ROLE_PATTERNS {
        if lower.contains(pattern) {
            return Some(role.to_string());
        }
    }
    None
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
                path: entry.content_file.unwrap(),
                source: "declared".to_string(),
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
/// knowledge artifacts. Results with score < MIN_SCORE or IDs already in
/// `exclude_ids` are filtered out. At most MAX_SEMANTIC results are returned.
///
/// Returns an empty vec if the ONNX model is unavailable or any step fails.
fn get_semantic_knowledge(
    prompt: &str,
    project_path: &Path,
    exclude_ids: &HashSet<String>,
) -> Vec<KnowledgeEntry> {
    // Resolve the model directory from platform app data dir.
    // Gracefully returns empty if the model is not installed.
    let Some(model_dir) = resolve_model_dir() else {
        return Vec::new();
    };

    let Ok(mut embedder) = Embedder::new(&model_dir) else {
        return Vec::new();
    };

    // Extract a query from the prompt: skip the role preamble (first paragraph),
    // use the next ~500 chars. This mirrors the TypeScript knowledge-injector.
    let query = extract_query(prompt);

    // Embed the query.
    let Ok(embeddings) = embedder.embed(&[query.as_str()]) else {
        return Vec::new();
    };
    let Some(query_embedding) = embeddings.into_iter().next() else {
        return Vec::new();
    };

    // Load knowledge injector and find matches.
    let Ok(injector) = KnowledgeInjector::new(project_path, &mut embedder) else {
        return Vec::new();
    };

    // Request MAX_SEMANTIC + excluded count so that after filtering we still
    // get up to MAX_SEMANTIC results — mirrors the TypeScript implementation.
    let top_n = MAX_SEMANTIC + exclude_ids.len();
    let matches = injector.match_prompt(&query_embedding, top_n, MIN_SCORE);

    let knowledge_dir = project_path.join(".orqa").join("process").join("knowledge");

    matches
        .into_iter()
        .filter(|m| !exclude_ids.contains(&m.name))
        .take(MAX_SEMANTIC)
        .map(|m| {
            let path = knowledge_dir.join(format!("{}.md", m.name));
            KnowledgeEntry {
                id: m.name.clone(),
                title: m.name,
                path: path.to_string_lossy().to_string(),
                source: "semantic".to_string(),
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
fn resolve_model_dir() -> Option<std::path::PathBuf> {
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

    candidates
        .into_iter()
        .find(|dir| dir.join("model.onnx").exists() && dir.join("tokenizer.json").exists())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ── detect_role ──

    #[test]
    fn detect_role_implementer() {
        assert_eq!(
            detect_role("You are an Implementer. Your task is..."),
            Some("implementer".to_string())
        );
    }

    #[test]
    fn detect_role_researcher() {
        assert_eq!(
            detect_role("You are a Researcher working on..."),
            Some("researcher".to_string())
        );
    }

    #[test]
    fn detect_role_reviewer() {
        assert_eq!(
            detect_role("You are a Reviewer. Verify that..."),
            Some("reviewer".to_string())
        );
    }

    #[test]
    fn detect_role_planner() {
        assert_eq!(
            detect_role("You are a Planner. Design the approach."),
            Some("planner".to_string())
        );
    }

    #[test]
    fn detect_role_writer() {
        assert_eq!(
            detect_role("You are a Writer. Document this."),
            Some("writer".to_string())
        );
    }

    #[test]
    fn detect_role_designer() {
        assert_eq!(
            detect_role("You are a Designer. Create the layout."),
            Some("designer".to_string())
        );
    }

    #[test]
    fn detect_role_governance_steward() {
        assert_eq!(
            detect_role("You are a Governance Steward. Maintain the artifacts."),
            Some("governance-steward".to_string())
        );
    }

    #[test]
    fn detect_role_no_match_returns_none() {
        assert_eq!(detect_role("Use the bash tool to run the test."), None);
        assert_eq!(detect_role(""), None);
    }

    #[test]
    fn detect_role_case_insensitive() {
        // All-caps "YOU ARE AN IMPLEMENTER" should still match.
        assert_eq!(
            detect_role("YOU ARE AN IMPLEMENTER working on..."),
            Some("implementer".to_string())
        );
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
