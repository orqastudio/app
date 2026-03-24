//! Hook lifecycle evaluation.
//!
//! Evaluates active rule enforcement entries against a [`HookContext`] and
//! returns a [`HookResult`] that the caller (CLI or sidecar) acts upon.
//!
//! # Supported enforcement entry shapes
//!
//! ## Bash pattern matching
//!
//! ```yaml
//! enforcement:
//!   - mechanism: hook
//!     event: bash
//!     pattern: "--no-verify"
//!     action: block
//!     message: "Bypassing hooks is forbidden."
//! ```
//!
//! Matches when context event is `PreAction`, tool is `Bash`, and `tool_input.command`
//! contains the regex `pattern`.
//!
//! ## File path matching
//!
//! ```yaml
//! enforcement:
//!   - mechanism: hook
//!     event: file
//!     paths:
//!       - ".env"
//!       - "*.pem"
//!     action: block
//!     message: "Do not commit secrets."
//! ```
//!
//! Matches when context has a `file_path` that matches any glob in `paths`.

use std::collections::HashMap;
use std::path::Path;

use regex::Regex;

use crate::graph::build_artifact_graph;
use crate::parse::query_artifacts;
use crate::platform::scan_plugin_manifests;
use crate::types::{HookContext, HookResult, HookViolation};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Evaluate all active rules against `ctx` and return a [`HookResult`].
///
/// Scans `.orqa/process/rules/` and any plugin-contributed rules via the
/// artifact graph, then tests each active rule's enforcement entries against
/// the context.
///
/// Never panics. I/O errors (missing `.orqa/` directory, etc.) are treated as
/// "no violations found" with action `"allow"`.
pub fn evaluate_hook(ctx: &HookContext, project_root: &Path) -> HookResult {
    let Ok(graph) = build_artifact_graph(project_root) else {
        return allow();
    };

    let plugin_contributions = scan_plugin_manifests(project_root);

    let rules = query_artifacts(
        &graph,
        project_root,
        Some("rule"),
        Some("active"),
        None,
        None,
        &plugin_contributions.artifact_types,
    );

    let mut violations: Vec<HookViolation> = Vec::new();

    // --- Plugin ownership protection ---
    // Before evaluating rules, check if the target file is owned by a plugin.
    if let Some(violation) = check_manifest_ownership(ctx, project_root) {
        violations.push(violation);
    }

    for rule in &rules {
        let Some(enforcement) = rule.frontmatter.get("enforcement") else {
            continue;
        };
        let Some(entries) = enforcement.as_array() else {
            continue;
        };

        for entry in entries {
            let Some(obj) = entry.as_object() else {
                continue;
            };

            // Only process hook-mechanism entries that have an event field.
            let mechanism = obj.get("mechanism").and_then(|v| v.as_str());
            if mechanism != Some("hook") {
                continue;
            }

            let event_kind = obj.get("event").and_then(|v| v.as_str());
            let action = obj
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("warn")
                .to_owned();
            let message = obj
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();

            match event_kind {
                Some("bash") => {
                    if let Some(violation) =
                        check_bash_entry(ctx, &rule.id, &action, &message, obj)
                    {
                        violations.push(violation);
                    }
                }
                Some("file") => {
                    if let Some(violation) =
                        check_file_entry(ctx, &rule.id, &action, &message, obj)
                    {
                        violations.push(violation);
                    }
                }
                _ => {
                    // Unknown or missing event kind — skip.
                }
            }
        }
    }

    build_result(violations)
}

// ---------------------------------------------------------------------------
// Entry-level matchers
// ---------------------------------------------------------------------------

/// Check a `bash` enforcement entry against the context.
///
/// Returns a violation if the context is a `PreAction` Bash call and the
/// command matches the entry's `pattern` regex.
fn check_bash_entry(
    ctx: &HookContext,
    rule_id: &str,
    action: &str,
    message: &str,
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Option<HookViolation> {
    // Only fires on PreAction events targeting the Bash tool.
    if ctx.event != "PreAction" {
        return None;
    }
    if ctx.tool_name.as_deref() != Some("Bash") {
        return None;
    }

    let command = ctx
        .tool_input
        .as_ref()
        .and_then(|v| v.get("command"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let pattern = obj.get("pattern").and_then(|v| v.as_str())?;

    let re = Regex::new(pattern).ok()?;
    if re.is_match(command) {
        Some(HookViolation {
            rule_id: rule_id.to_owned(),
            action: action.to_owned(),
            message: message.to_owned(),
        })
    } else {
        None
    }
}

/// Check a `file` enforcement entry against the context.
///
/// Returns a violation if `ctx.file_path` matches any glob pattern in the
/// entry's `paths` array.
fn check_file_entry(
    ctx: &HookContext,
    rule_id: &str,
    action: &str,
    message: &str,
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Option<HookViolation> {
    let file_path = ctx.file_path.as_deref()?;

    let paths = obj.get("paths").and_then(|v| v.as_array())?;

    for pat_val in paths {
        let Some(pattern) = pat_val.as_str() else {
            continue;
        };
        if glob_matches(pattern, file_path) {
            return Some(HookViolation {
                rule_id: rule_id.to_owned(),
                action: action.to_owned(),
                message: message.to_owned(),
            });
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Manifest ownership check
// ---------------------------------------------------------------------------

/// Content manifest entry for a single plugin.
#[derive(Debug, serde::Deserialize)]
struct ManifestEntry {
    #[serde(default)]
    files: Vec<String>,
}

/// Content manifest format (`.orqa/manifest.json`).
#[derive(Debug, serde::Deserialize)]
struct ContentManifest {
    #[serde(default)]
    plugins: HashMap<String, ManifestEntry>,
}

/// Check if the target file is owned by a plugin (listed in `.orqa/manifest.json`).
///
/// Blocks the write with a message identifying the owning plugin. Only fires on
/// file-write contexts (contexts that have a `file_path`).
fn check_manifest_ownership(ctx: &HookContext, project_root: &Path) -> Option<HookViolation> {
    let file_path = ctx.file_path.as_deref()?;

    // Normalise to forward slashes for comparison against manifest entries
    let normalised = file_path.replace('\\', "/");

    let manifest_path = project_root.join(".orqa/manifest.json");
    let content = std::fs::read_to_string(&manifest_path).ok()?;
    let manifest: ContentManifest = serde_json::from_str(&content).ok()?;

    for (plugin_name, entry) in &manifest.plugins {
        for tracked_file in &entry.files {
            if tracked_file == &normalised {
                return Some(HookViolation {
                    rule_id: "plugin-ownership".to_owned(),
                    action: "block".to_owned(),
                    message: format!(
                        "This artifact is owned by plugin {plugin_name}. Edit the plugin source and run 'orqa plugin refresh' instead."
                    ),
                });
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Result construction
// ---------------------------------------------------------------------------

fn build_result(violations: Vec<HookViolation>) -> HookResult {
    if violations.is_empty() {
        return allow();
    }

    let has_block = violations.iter().any(|v| v.action == "block");
    let overall_action = if has_block { "block" } else { "warn" }.to_owned();

    let messages = violations.iter().map(|v| v.message.clone()).collect();

    HookResult {
        action: overall_action,
        messages,
        violations,
    }
}

fn allow() -> HookResult {
    HookResult {
        action: "allow".to_owned(),
        messages: Vec::new(),
        violations: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Glob matcher (no external dep)
// ---------------------------------------------------------------------------

/// Minimal glob matcher supporting `*` (any chars except `/`) and `**` (any
/// sequence including `/`). Matching is case-sensitive.
///
/// Recognised patterns:
/// - `?`  — matches exactly one character that is not `/`
/// - `*`  — matches zero or more characters that are not `/`
/// - `**` — matches zero or more characters including `/`
///
/// When `**` is followed by `/`, the slash is consumed as part of the
/// double-star token (so `**/*.rs` matches both `foo.rs` and `a/b/foo.rs`).
fn glob_matches(pattern: &str, path: &str) -> bool {
    glob_match_impl(pattern, path)
}

/// Recursive glob matcher.
///
/// Both `p` (pattern) and `s` (string to match) are `&str` slices advanced on
/// each call. The recursion depth is bounded by the number of `*` / `**`
/// tokens in the pattern.
fn glob_match_impl(p: &str, s: &str) -> bool {
    // Base cases.
    if p.is_empty() {
        return s.is_empty();
    }

    // Double-star token.
    if p.starts_with("**") {
        let rest_p = p.strip_prefix("**").unwrap_or(p);
        // Consume optional trailing slash: `**/` treats the slash as part of
        // the double-star so `**/foo` matches both `foo` and `a/foo`.
        let rest_p = rest_p.strip_prefix('/').unwrap_or(rest_p);

        // `**` alone (or `**/` at end) matches everything remaining.
        if rest_p.is_empty() {
            return true;
        }

        // Try matching the rest of the pattern against every suffix of `s`.
        // This lets `**` consume zero or more path segments.
        let mut candidate = s;
        loop {
            if glob_match_impl(rest_p, candidate) {
                return true;
            }
            // Advance by one character (including `/`) to let `**` grow.
            let mut chars = candidate.chars();
            if chars.next().is_none() {
                break;
            }
            candidate = chars.as_str();
        }
        return false;
    }

    // Single-star token.
    if let Some(rest_p) = p.strip_prefix('*') {
        // `*` matches zero or more non-`/` characters.
        let mut candidate = s;
        loop {
            if glob_match_impl(rest_p, candidate) {
                return true;
            }
            // Stop at a path separator — `*` cannot cross directory boundaries.
            if candidate.starts_with('/') {
                break;
            }
            let mut chars = candidate.chars();
            if chars.next().is_none() {
                break;
            }
            candidate = chars.as_str();
        }
        return false;
    }

    // Literal character or `?` wildcard.
    let mut p_chars = p.chars();
    let mut s_chars = s.chars();
    let pc = p_chars.next().unwrap_or('\0');
    let sc = s_chars.next();

    match sc {
        None => false, // pattern has more chars but string is exhausted
        Some(sc) => {
            let matches = pc == '?' && sc != '/' || pc == sc;
            matches && glob_match_impl(p_chars.as_str(), s_chars.as_str())
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn write_rule(dir: &Path, id: &str, enforcement: serde_json::Value) {
        let rules_dir = dir.join(".orqa/process/rules");
        fs::create_dir_all(&rules_dir).unwrap();
        let content = format!(
            "---\nid: {id}\ntitle: Test Rule\nstatus: active\nenforcement: {}\n---\n\nBody.\n",
            serde_json::to_string(&enforcement).unwrap()
        );
        fs::write(rules_dir.join(format!("{id}.md")), content).unwrap();
    }

    fn bash_ctx(command: &str) -> HookContext {
        HookContext {
            event: "PreAction".to_owned(),
            tool_name: Some("Bash".to_owned()),
            tool_input: Some(serde_json::json!({ "command": command })),
            file_path: None,
            user_message: None,
            agent_type: None,
        }
    }

    fn file_ctx(path: &str) -> HookContext {
        HookContext {
            event: "PreAction".to_owned(),
            tool_name: None,
            tool_input: None,
            file_path: Some(path.to_owned()),
            user_message: None,
            agent_type: None,
        }
    }

    // -----------------------------------------------------------------------
    // Bash pattern matching
    // -----------------------------------------------------------------------

    #[test]
    fn no_verify_is_blocked() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-a1b2c3d4",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "bash",
                "pattern": "--no-verify",
                "action": "block",
                "message": "Bypassing hooks is forbidden."
            }]),
        );

        let result = evaluate_hook(&bash_ctx("git commit --no-verify -m 'skip'"), tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations.len(), 1);
        assert_eq!(result.violations[0].rule_id, "RULE-a1b2c3d4");
        assert!(result.messages[0].contains("Bypassing hooks"));
    }

    #[test]
    fn force_push_is_blocked() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-b2c3d4e5",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "bash",
                "pattern": "push.*--force",
                "action": "block",
                "message": "Force push to main is forbidden."
            }]),
        );

        let result = evaluate_hook(&bash_ctx("git push origin main --force"), tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations[0].rule_id, "RULE-b2c3d4e5");
    }

    #[test]
    fn clean_bash_command_is_allowed() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-a1b2c3d4",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "bash",
                "pattern": "--no-verify",
                "action": "block",
                "message": "Bypassing hooks is forbidden."
            }]),
        );

        let result = evaluate_hook(&bash_ctx("git commit -m 'normal commit'"), tmp.path());
        assert_eq!(result.action, "allow");
        assert!(result.violations.is_empty());
    }

    // -----------------------------------------------------------------------
    // File path matching
    // -----------------------------------------------------------------------

    #[test]
    fn env_file_is_blocked() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-c3d4e5f6",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "file",
                "paths": [".env", "*.pem", "**/*.key"],
                "action": "block",
                "message": "Do not commit secrets."
            }]),
        );

        let result = evaluate_hook(&file_ctx(".env"), tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations[0].rule_id, "RULE-c3d4e5f6");
    }

    #[test]
    fn single_star_does_not_match_path_separator() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-c3d4e5f6",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "file",
                "paths": ["*.pem"],
                "action": "block",
                "message": "Do not commit secrets."
            }]),
        );

        // *.pem matches only at the top level — "certs/server.pem" has a separator.
        let result = evaluate_hook(&file_ctx("certs/server.pem"), tmp.path());
        assert_eq!(result.action, "allow");
    }

    #[test]
    fn nested_key_file_matched_by_globstar() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-c3d4e5f6",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "file",
                "paths": ["**/*.key"],
                "action": "block",
                "message": "Do not commit private keys."
            }]),
        );

        let result = evaluate_hook(&file_ctx("secrets/prod/server.key"), tmp.path());
        assert_eq!(result.action, "block");
    }

    #[test]
    fn safe_file_is_allowed() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-c3d4e5f6",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "file",
                "paths": [".env", "*.pem"],
                "action": "block",
                "message": "Do not commit secrets."
            }]),
        );

        let result = evaluate_hook(&file_ctx("src/main.rs"), tmp.path());
        assert_eq!(result.action, "allow");
    }

    // -----------------------------------------------------------------------
    // Mixed violations: block + warn precedence
    // -----------------------------------------------------------------------

    #[test]
    fn block_takes_precedence_over_warn() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-a1b2c3d4",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "bash",
                "pattern": "--no-verify",
                "action": "block",
                "message": "Block: no-verify."
            }]),
        );
        write_rule(
            tmp.path(),
            "RULE-b2c3d4e5",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "bash",
                "pattern": "commit",
                "action": "warn",
                "message": "Warn: commit detected."
            }]),
        );

        let result = evaluate_hook(&bash_ctx("git commit --no-verify -m 'test'"), tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations.len(), 2);
    }

    #[test]
    fn warn_only_returns_warn() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-a1b2c3d4",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "bash",
                "pattern": "commit",
                "action": "warn",
                "message": "Commits should be reviewed."
            }]),
        );

        let result = evaluate_hook(&bash_ctx("git commit -m 'normal'"), tmp.path());
        assert_eq!(result.action, "warn");
        assert_eq!(result.violations.len(), 1);
    }

    // -----------------------------------------------------------------------
    // Clean project — no rules
    // -----------------------------------------------------------------------

    #[test]
    fn empty_project_allows_everything() {
        let tmp = TempDir::new().unwrap();
        let result = evaluate_hook(&bash_ctx("git push --force origin main"), tmp.path());
        assert_eq!(result.action, "allow");
        assert!(result.violations.is_empty());
    }

    // -----------------------------------------------------------------------
    // Inactive rules are ignored
    // -----------------------------------------------------------------------

    #[test]
    fn inactive_rules_are_skipped() {
        let tmp = TempDir::new().unwrap();
        let rules_dir = tmp.path().join(".orqa/process/rules");
        fs::create_dir_all(&rules_dir).unwrap();
        let content = concat!(
            "---\nid: RULE-a1b2c3d4\ntitle: Inactive Rule\nstatus: inactive\n",
            "enforcement:\n  - mechanism: hook\n    event: bash\n",
            "    pattern: \"--no-verify\"\n    action: block\n",
            "    message: \"Should not fire.\"\n---\n\nBody.\n"
        );
        fs::write(rules_dir.join("RULE-a1b2c3d4.md"), content).unwrap();

        let result = evaluate_hook(&bash_ctx("git commit --no-verify"), tmp.path());
        assert_eq!(result.action, "allow");
    }

    // -----------------------------------------------------------------------
    // Non-hook mechanism entries are ignored
    // -----------------------------------------------------------------------

    #[test]
    fn non_hook_entries_are_ignored() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-a1b2c3d4",
            serde_json::json!([{
                "mechanism": "behavioral",
                "message": "Always do X."
            }]),
        );

        let result = evaluate_hook(&bash_ctx("git commit --no-verify"), tmp.path());
        assert_eq!(result.action, "allow");
    }

    // -----------------------------------------------------------------------
    // Plugin ownership protection
    // -----------------------------------------------------------------------

    #[test]
    fn plugin_owned_file_is_blocked() {
        let tmp = TempDir::new().unwrap();

        // Create manifest.json with an owned file
        let orqa_dir = tmp.path().join(".orqa");
        fs::create_dir_all(&orqa_dir).unwrap();
        fs::write(
            orqa_dir.join("manifest.json"),
            r#"{"plugins":{"@orqastudio/plugin-agile-governance":{"version":"0.1.0-dev","installed_at":"2026-03-22T00:00:00Z","files":[".orqa/process/rules/RULE-633e636d.md"]}}}"#,
        ).unwrap();

        let ctx = file_ctx(".orqa/process/rules/RULE-633e636d.md");
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "block");
        assert!(result.messages[0].contains("@orqastudio/plugin-agile-governance"));
        assert!(result.messages[0].contains("orqa plugin refresh"));
    }

    #[test]
    fn non_owned_file_is_allowed() {
        let tmp = TempDir::new().unwrap();

        let orqa_dir = tmp.path().join(".orqa");
        fs::create_dir_all(&orqa_dir).unwrap();
        fs::write(
            orqa_dir.join("manifest.json"),
            r#"{"plugins":{"@orqastudio/plugin-core":{"version":"0.1.0","installed_at":"2026-03-22T00:00:00Z","files":[".orqa/process/rules/RULE-abc.md"]}}}"#,
        ).unwrap();

        let ctx = file_ctx(".orqa/process/rules/RULE-other.md");
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
    }

    #[test]
    fn no_manifest_allows_all_files() {
        let tmp = TempDir::new().unwrap();
        let ctx = file_ctx(".orqa/process/rules/RULE-633e636d.md");
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
    }

    // -----------------------------------------------------------------------
    // Glob matcher unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn glob_exact_match() {
        assert!(glob_matches(".env", ".env"));
        assert!(!glob_matches(".env", ".env.local"));
    }

    #[test]
    fn glob_star_matches_single_segment() {
        assert!(glob_matches("*.pem", "server.pem"));
        assert!(!glob_matches("*.pem", "certs/server.pem"));
    }

    #[test]
    fn glob_doublestar_matches_nested() {
        assert!(glob_matches("**/*.key", "secrets/prod/server.key"));
        assert!(glob_matches("**/*.key", "server.key"));
        assert!(!glob_matches("**/*.key", "server.crt"));
    }
}
