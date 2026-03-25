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
//!
//! ## Field-check matching
//!
//! ```yaml
//! enforcement:
//!   - mechanism: hook
//!     event: field-check
//!     tool: Agent
//!     field: tool_input.run_in_background
//!     operator: equals
//!     value: true
//!     action: block
//!     message: "Agents must run in background (run_in_background: true)"
//! ```
//!
//! Matches when context event is `PreAction`, the tool matches `tool`, and the
//! dot-path `field` into `tool_input` fails the `operator` comparison against
//! `value`. Supported operators: `equals`, `not_equals`, `exists`,
//! `not_exists`, `contains`, `matches`.
//!
//! ## Tool-matcher (role-based access control)
//!
//! ```yaml
//! enforcement:
//!   - mechanism: hook
//!     event: tool-matcher
//!     tool: Write|Edit
//!     paths:
//!       - ".orqa/process/**"
//!     allowed_roles:
//!       - governance_steward
//!       - writer
//!     action: block
//!     message: "Only governance-steward and writer roles can modify .orqa/process/ files"
//! ```
//!
//! Matches when context event is `PreAction`, the tool name matches any of the
//! pipe-separated names in `tool`, and optionally the file path matches a glob
//! in `paths`. Role filtering via `allowed_roles` (allowlist) or
//! `denied_roles` (denylist) controls which agent types trigger a violation.

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
                    if let Some(violation) = check_bash_entry(ctx, &rule.id, &action, &message, obj)
                    {
                        violations.push(violation);
                    }
                }
                Some("file") => {
                    if let Some(violation) = check_file_entry(ctx, &rule.id, &action, &message, obj)
                    {
                        violations.push(violation);
                    }
                }
                Some("field-check") => {
                    if let Some(violation) =
                        check_field_entry(ctx, &rule.id, &action, &message, obj)
                    {
                        violations.push(violation);
                    }
                }
                Some("tool-matcher") => {
                    if let Some(violation) =
                        check_tool_matcher_entry(ctx, &rule.id, &action, &message, obj)
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

/// Check a `field-check` enforcement entry against the context.
///
/// Returns a violation when the context is a `PreAction` event, the tool name
/// matches the entry's `tool` field, and the dot-path `field` into
/// `ctx.tool_input` fails the `operator` comparison against `value`.
///
/// Supported operators:
/// - `equals`     — field value must equal `value`
/// - `not_equals` — field value must NOT equal `value`
/// - `exists`     — field must be present (not null)
/// - `not_exists` — field must be absent or null
/// - `contains`   — field (as string) must contain `value` as substring
/// - `matches`    — field (as string) must match `value` as regex
fn check_field_entry(
    ctx: &HookContext,
    rule_id: &str,
    action: &str,
    message: &str,
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Option<HookViolation> {
    // Only fires on PreAction events.
    if ctx.event != "PreAction" {
        return None;
    }

    // Tool name must match the entry's `tool` field.
    let expected_tool = obj.get("tool").and_then(|v| v.as_str())?;
    if ctx.tool_name.as_deref() != Some(expected_tool) {
        return None;
    }

    let field_path = obj.get("field").and_then(|v| v.as_str())?;
    let operator = obj.get("operator").and_then(|v| v.as_str())?;
    let entry_value = obj.get("value"); // May be None for exists/not_exists

    // Navigate the dot-path into tool_input.
    // The field path may start with "tool_input." — strip that prefix since
    // we're already starting from ctx.tool_input.
    let path = field_path.strip_prefix("tool_input.").unwrap_or(field_path);
    let segments: Vec<&str> = path.split('.').collect();

    let resolved = resolve_dot_path(ctx.tool_input.as_ref(), &segments);

    // Evaluate the operator.
    let check_passes = match operator {
        "equals" => {
            let expected = entry_value?;
            match resolved {
                Some(actual) => values_equal(actual, expected),
                None => false,
            }
        }
        "not_equals" => {
            let expected = entry_value?;
            match resolved {
                Some(actual) => !values_equal(actual, expected),
                None => true, // absent != any value
            }
        }
        "exists" => resolved.is_some() && !resolved.unwrap().is_null(),
        "not_exists" => resolved.is_none() || resolved.unwrap().is_null(),
        "contains" => {
            let needle = entry_value?.as_str()?;
            match resolved.and_then(|v| v.as_str()) {
                Some(haystack) => haystack.contains(needle),
                None => false,
            }
        }
        "matches" => {
            let pattern = entry_value?.as_str()?;
            let re = Regex::new(pattern).ok()?;
            match resolved.and_then(|v| v.as_str()) {
                Some(text) => re.is_match(text),
                None => false,
            }
        }
        _ => return None, // Unknown operator — skip.
    };

    // A violation occurs when the check FAILS.
    // For "equals": violation if the value does NOT equal the expected.
    // For "exists": violation if the field does NOT exist.
    // etc.
    if check_passes {
        None
    } else {
        Some(HookViolation {
            rule_id: rule_id.to_owned(),
            action: action.to_owned(),
            message: message.to_owned(),
        })
    }
}

/// Check a `tool-matcher` enforcement entry against the context.
///
/// Returns a violation when the context is a `PreAction` event, the tool name
/// matches one of the pipe-separated names in the entry's `tool` field,
/// optionally the file path matches a glob in `paths`, and the agent role
/// fails the allowlist/denylist check.
///
/// Role filtering:
/// - `allowed_roles`: only these roles are permitted — others get a violation.
/// - `denied_roles`: these roles are blocked — others are permitted.
/// - Neither present: the entry applies to all roles.
/// - `ctx.agent_type` of `None` is treated as `"unknown"`.
fn check_tool_matcher_entry(
    ctx: &HookContext,
    rule_id: &str,
    action: &str,
    message: &str,
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Option<HookViolation> {
    // Only fires on PreAction events.
    if ctx.event != "PreAction" {
        return None;
    }

    // Tool name must match one of the pipe-separated names in `tool`.
    let tool_spec = obj.get("tool").and_then(|v| v.as_str())?;
    let ctx_tool = ctx.tool_name.as_deref()?;
    let tool_matches = tool_spec.split('|').any(|t| t.trim() == ctx_tool);
    if !tool_matches {
        return None;
    }

    // If `paths` is present, the file path must match at least one glob.
    if let Some(paths) = obj.get("paths").and_then(|v| v.as_array()) {
        let file_path = match ctx.file_path.as_deref() {
            Some(fp) => fp.replace('\\', "/"),
            None => return None, // paths filter present but no file_path in context
        };
        let any_path_matches = paths.iter().any(|pat_val| {
            pat_val
                .as_str()
                .is_some_and(|pattern| glob_matches(pattern, &file_path))
        });
        if !any_path_matches {
            return None;
        }
    }

    // Role-based access control.
    let agent_role = ctx.agent_type.as_deref().unwrap_or("unknown");

    // Check allowed_roles (allowlist).
    if let Some(allowed) = obj.get("allowed_roles").and_then(|v| v.as_array()) {
        let is_allowed = allowed.iter().any(|r| r.as_str() == Some(agent_role));
        if !is_allowed {
            return Some(HookViolation {
                rule_id: rule_id.to_owned(),
                action: action.to_owned(),
                message: message.to_owned(),
            });
        }
    }

    // Check denied_roles (denylist).
    if let Some(denied) = obj.get("denied_roles").and_then(|v| v.as_array()) {
        let is_denied = denied.iter().any(|r| r.as_str() == Some(agent_role));
        if is_denied {
            return Some(HookViolation {
                rule_id: rule_id.to_owned(),
                action: action.to_owned(),
                message: message.to_owned(),
            });
        }
    }

    // No role filter present, or the agent passed role checks but we still
    // matched tool+paths — that means the entry applies unconditionally.
    // If neither allowed_roles nor denied_roles is present, it's a blanket
    // match: the tool+path combination itself is the violation.
    let has_role_filter = obj.contains_key("allowed_roles") || obj.contains_key("denied_roles");
    if !has_role_filter {
        return Some(HookViolation {
            rule_id: rule_id.to_owned(),
            action: action.to_owned(),
            message: message.to_owned(),
        });
    }

    None
}

/// Walk a JSON value along a dot-separated path.
///
/// Returns `Some(&Value)` if the path resolves, `None` if any segment is
/// missing or the intermediate value is not an object.
fn resolve_dot_path<'a>(
    root: Option<&'a serde_json::Value>,
    segments: &[&str],
) -> Option<&'a serde_json::Value> {
    let mut current = root?;
    for &segment in segments {
        current = current.get(segment)?;
    }
    Some(current)
}

/// Compare two `serde_json::Value`s for equality with loose type coercion.
///
/// Handles the common case where the entry `value` is a YAML/JSON boolean or
/// number but the actual field value may be stored differently.
fn values_equal(actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
    // Direct equality covers the majority of cases.
    if actual == expected {
        return true;
    }

    // Loose comparison: bool vs string representation
    match (actual, expected) {
        (serde_json::Value::String(a), serde_json::Value::Bool(b)) => {
            a == &b.to_string()
        }
        (serde_json::Value::Bool(a), serde_json::Value::String(b)) => {
            &a.to_string() == b
        }
        (serde_json::Value::String(a), serde_json::Value::Number(n)) => {
            a == &n.to_string()
        }
        (serde_json::Value::Number(n), serde_json::Value::String(a)) => {
            &n.to_string() == a
        }
        _ => false,
    }
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
            r#"{"plugins":{"@orqastudio/plugin-agile-workflow":{"version":"0.1.0-dev","installed_at":"2026-03-22T00:00:00Z","files":[".orqa/process/rules/RULE-f609242f.md"]}}}"#,
        ).unwrap();

        let ctx = file_ctx(".orqa/process/rules/RULE-f609242f.md");
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "block");
        assert!(result.messages[0].contains("@orqastudio/plugin-agile-workflow"));
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
        let ctx = file_ctx(".orqa/process/rules/RULE-f609242f.md");
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
    }

    // -----------------------------------------------------------------------
    // Field-check matching
    // -----------------------------------------------------------------------

    /// Build a HookContext for a tool invocation with the given tool_input JSON.
    fn field_check_ctx(tool: &str, input: serde_json::Value) -> HookContext {
        HookContext {
            event: "PreAction".to_owned(),
            tool_name: Some(tool.to_owned()),
            tool_input: Some(input),
            file_path: None,
            user_message: None,
            agent_type: None,
        }
    }

    #[test]
    fn field_check_agent_without_background_is_violation() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000001",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Agent",
                "field": "tool_input.run_in_background",
                "operator": "equals",
                "value": true,
                "action": "block",
                "message": "Agents must run in background (run_in_background: true)"
            }]),
        );

        // Agent call without run_in_background → violation
        let ctx = field_check_ctx("Agent", serde_json::json!({
            "prompt": "do something",
            "run_in_background": false
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations.len(), 1);
        assert!(result.messages[0].contains("run_in_background"));
    }

    #[test]
    fn field_check_agent_with_background_true_no_violation() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000002",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Agent",
                "field": "tool_input.run_in_background",
                "operator": "equals",
                "value": true,
                "action": "block",
                "message": "Agents must run in background"
            }]),
        );

        // Agent call with run_in_background: true → no violation
        let ctx = field_check_ctx("Agent", serde_json::json!({
            "prompt": "do something",
            "run_in_background": true
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
        assert!(result.violations.is_empty());
    }

    #[test]
    fn field_check_exists_missing_field_is_violation() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000003",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Agent",
                "field": "tool_input.team_name",
                "operator": "exists",
                "action": "block",
                "message": "Agents must specify team_name"
            }]),
        );

        // Agent call without team_name → violation (exists check fails)
        let ctx = field_check_ctx("Agent", serde_json::json!({
            "prompt": "do something",
            "run_in_background": true
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations.len(), 1);
        assert!(result.messages[0].contains("team_name"));
    }

    #[test]
    fn field_check_non_matching_tool_no_violation() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000004",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Agent",
                "field": "tool_input.run_in_background",
                "operator": "equals",
                "value": true,
                "action": "block",
                "message": "Agents must run in background"
            }]),
        );

        // Bash tool (not Agent) → rule does not apply, no violation
        let ctx = field_check_ctx("Bash", serde_json::json!({
            "command": "ls -la"
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
        assert!(result.violations.is_empty());
    }

    #[test]
    fn field_check_not_exists_missing_field_no_violation() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000005",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Write",
                "field": "tool_input.dangerous_flag",
                "operator": "not_exists",
                "action": "block",
                "message": "dangerous_flag must not be present"
            }]),
        );

        // Write call without dangerous_flag → not_exists passes, no violation
        let ctx = field_check_ctx("Write", serde_json::json!({
            "file_path": "/tmp/foo.txt",
            "content": "hello"
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
        assert!(result.violations.is_empty());
    }

    #[test]
    fn field_check_contains_operator() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000006",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Bash",
                "field": "tool_input.description",
                "operator": "contains",
                "value": "safe",
                "action": "block",
                "message": "Description must indicate safety"
            }]),
        );

        // Description contains "safe" → check passes → no violation
        let ctx = field_check_ctx("Bash", serde_json::json!({
            "command": "ls",
            "description": "List files safely"
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
        assert!(result.violations.is_empty());

        // Description does NOT contain "safe" → check fails → violation
        let ctx2 = field_check_ctx("Bash", serde_json::json!({
            "command": "rm -rf /",
            "description": "This is a dangerous operation"
        }));
        let result2 = evaluate_hook(&ctx2, tmp.path());
        assert_eq!(result2.action, "block");
        assert_eq!(result2.violations.len(), 1);
    }

    #[test]
    fn field_check_matches_operator() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000007",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Agent",
                "field": "tool_input.team_name",
                "operator": "matches",
                "value": "^[a-z][a-z0-9-]+$",
                "action": "block",
                "message": "team_name must be kebab-case"
            }]),
        );

        // Valid kebab-case → passes regex → no violation
        let ctx = field_check_ctx("Agent", serde_json::json!({
            "team_name": "my-team-42",
            "run_in_background": true
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");

        // Invalid name → fails regex → violation
        let ctx2 = field_check_ctx("Agent", serde_json::json!({
            "team_name": "My Team!",
            "run_in_background": true
        }));
        let result2 = evaluate_hook(&ctx2, tmp.path());
        assert_eq!(result2.action, "block");
    }

    #[test]
    fn field_check_not_equals_operator() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-fc000008",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "field-check",
                "tool": "Agent",
                "field": "tool_input.run_in_background",
                "operator": "not_equals",
                "value": false,
                "action": "block",
                "message": "run_in_background must not be false"
            }]),
        );

        // false → not_equals false fails → violation
        let ctx = field_check_ctx("Agent", serde_json::json!({
            "run_in_background": false
        }));
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "block");

        // true → not_equals false passes → no violation
        let ctx2 = field_check_ctx("Agent", serde_json::json!({
            "run_in_background": true
        }));
        let result2 = evaluate_hook(&ctx2, tmp.path());
        assert_eq!(result2.action, "allow");
    }

    // -----------------------------------------------------------------------
    // Field-check unit tests (direct function calls)
    // -----------------------------------------------------------------------

    #[test]
    fn resolve_dot_path_nested() {
        let val = serde_json::json!({"a": {"b": {"c": 42}}});
        let result = resolve_dot_path(Some(&val), &["a", "b", "c"]);
        assert_eq!(result, Some(&serde_json::json!(42)));
    }

    #[test]
    fn resolve_dot_path_missing_segment() {
        let val = serde_json::json!({"a": {"b": 1}});
        let result = resolve_dot_path(Some(&val), &["a", "x"]);
        assert!(result.is_none());
    }

    #[test]
    fn values_equal_same_types() {
        assert!(values_equal(
            &serde_json::json!(true),
            &serde_json::json!(true)
        ));
        assert!(!values_equal(
            &serde_json::json!(true),
            &serde_json::json!(false)
        ));
        assert!(values_equal(
            &serde_json::json!("hello"),
            &serde_json::json!("hello")
        ));
    }

    #[test]
    fn values_equal_loose_coercion() {
        // String "true" should equal bool true
        assert!(values_equal(
            &serde_json::json!("true"),
            &serde_json::json!(true)
        ));
        assert!(values_equal(
            &serde_json::json!(true),
            &serde_json::json!("true")
        ));
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

    // -----------------------------------------------------------------------
    // Tool-matcher tests
    // -----------------------------------------------------------------------

    /// Build a HookContext with tool, file_path, and agent_type for tool-matcher tests.
    fn tool_matcher_ctx(
        tool: &str,
        file_path: Option<&str>,
        agent_type: Option<&str>,
    ) -> HookContext {
        HookContext {
            event: "PreAction".to_owned(),
            tool_name: Some(tool.to_owned()),
            tool_input: None,
            file_path: file_path.map(|s| s.to_owned()),
            user_message: None,
            agent_type: agent_type.map(|s| s.to_owned()),
        }
    }

    #[test]
    fn tool_matcher_write_matches_write_edit_pipe() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-tm000001",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "tool-matcher",
                "tool": "Write|Edit",
                "action": "block",
                "message": "Write and Edit are blocked for all roles."
            }]),
        );

        // Write matches "Write|Edit"
        let ctx = tool_matcher_ctx("Write", None, None);
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations.len(), 1);
        assert_eq!(result.violations[0].rule_id, "RULE-tm000001");
    }

    #[test]
    fn tool_matcher_bash_does_not_match_write_edit() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-tm000002",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "tool-matcher",
                "tool": "Write|Edit",
                "action": "block",
                "message": "Write and Edit are blocked."
            }]),
        );

        // Bash does NOT match "Write|Edit"
        let ctx = tool_matcher_ctx("Bash", None, None);
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
        assert!(result.violations.is_empty());
    }

    #[test]
    fn tool_matcher_path_filtering() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-tm000003",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "tool-matcher",
                "tool": "Write|Edit",
                "paths": [".orqa/process/**"],
                "action": "block",
                "message": "Cannot write to .orqa/process."
            }]),
        );

        // File inside .orqa/process/ matches
        let ctx_match = tool_matcher_ctx(
            "Write",
            Some(".orqa/process/rules/RULE-abc.md"),
            None,
        );
        let result = evaluate_hook(&ctx_match, tmp.path());
        assert_eq!(result.action, "block");

        // File outside .orqa/process/ does not match
        let ctx_no_match = tool_matcher_ctx("Write", Some("src/main.rs"), None);
        let result2 = evaluate_hook(&ctx_no_match, tmp.path());
        assert_eq!(result2.action, "allow");

        // No file_path when paths filter present — no match
        let ctx_no_file = tool_matcher_ctx("Write", None, None);
        let result3 = evaluate_hook(&ctx_no_file, tmp.path());
        assert_eq!(result3.action, "allow");
    }

    #[test]
    fn tool_matcher_allowed_roles_enforcement() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-tm000004",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "tool-matcher",
                "tool": "Write|Edit",
                "paths": [".orqa/process/**"],
                "allowed_roles": ["governance_steward", "writer"],
                "action": "block",
                "message": "Only governance-steward and writer can modify .orqa/process/."
            }]),
        );

        // governance_steward is allowed — no violation
        let ctx_allowed = tool_matcher_ctx(
            "Edit",
            Some(".orqa/process/rules/RULE-abc.md"),
            Some("governance_steward"),
        );
        let result = evaluate_hook(&ctx_allowed, tmp.path());
        assert_eq!(result.action, "allow");

        // implementer is NOT in allowed_roles — violation
        let ctx_denied = tool_matcher_ctx(
            "Edit",
            Some(".orqa/process/rules/RULE-abc.md"),
            Some("implementer"),
        );
        let result2 = evaluate_hook(&ctx_denied, tmp.path());
        assert_eq!(result2.action, "block");
        assert_eq!(result2.violations.len(), 1);

        // No agent_type (unknown) is NOT in allowed_roles — violation
        let ctx_unknown = tool_matcher_ctx(
            "Write",
            Some(".orqa/process/agents/agent-x.md"),
            None,
        );
        let result3 = evaluate_hook(&ctx_unknown, tmp.path());
        assert_eq!(result3.action, "block");
    }

    #[test]
    fn tool_matcher_denied_roles_enforcement() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-tm000005",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "tool-matcher",
                "tool": "Bash",
                "denied_roles": ["reviewer", "writer"],
                "action": "block",
                "message": "Reviewers and writers cannot run Bash."
            }]),
        );

        // reviewer is in denied_roles — violation
        let ctx_denied = tool_matcher_ctx("Bash", None, Some("reviewer"));
        let result = evaluate_hook(&ctx_denied, tmp.path());
        assert_eq!(result.action, "block");
        assert_eq!(result.violations.len(), 1);

        // implementer is NOT in denied_roles — no violation
        let ctx_allowed = tool_matcher_ctx("Bash", None, Some("implementer"));
        let result2 = evaluate_hook(&ctx_allowed, tmp.path());
        assert_eq!(result2.action, "allow");

        // No agent_type (unknown) is NOT in denied_roles — no violation
        let ctx_unknown = tool_matcher_ctx("Bash", None, None);
        let result3 = evaluate_hook(&ctx_unknown, tmp.path());
        assert_eq!(result3.action, "allow");
    }

    #[test]
    fn tool_matcher_no_role_filter_applies_to_all() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-tm000006",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "tool-matcher",
                "tool": "Agent",
                "action": "warn",
                "message": "Agent spawning detected."
            }]),
        );

        // Any role triggers the violation when no role filter
        let ctx1 = tool_matcher_ctx("Agent", None, Some("implementer"));
        let result1 = evaluate_hook(&ctx1, tmp.path());
        assert_eq!(result1.action, "warn");
        assert_eq!(result1.violations.len(), 1);

        let ctx2 = tool_matcher_ctx("Agent", None, Some("governance_steward"));
        let result2 = evaluate_hook(&ctx2, tmp.path());
        assert_eq!(result2.action, "warn");

        let ctx3 = tool_matcher_ctx("Agent", None, None);
        let result3 = evaluate_hook(&ctx3, tmp.path());
        assert_eq!(result3.action, "warn");
    }

    #[test]
    fn tool_matcher_non_preaction_event_skipped() {
        let tmp = TempDir::new().unwrap();
        write_rule(
            tmp.path(),
            "RULE-tm000007",
            serde_json::json!([{
                "mechanism": "hook",
                "event": "tool-matcher",
                "tool": "Write",
                "action": "block",
                "message": "Write blocked."
            }]),
        );

        // PostAction event should not trigger tool-matcher
        let ctx = HookContext {
            event: "PostAction".to_owned(),
            tool_name: Some("Write".to_owned()),
            tool_input: None,
            file_path: None,
            user_message: None,
            agent_type: None,
        };
        let result = evaluate_hook(&ctx, tmp.path());
        assert_eq!(result.action, "allow");
    }
}
