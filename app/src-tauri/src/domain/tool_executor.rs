// Tool executor for the orqa-studio Tauri app.
//
// Dispatches tool calls from the sidecar to the appropriate handler. Pure tool
// implementations (read_file, write_file, edit_file, bash, glob, grep,
// load_knowledge) live in orqa_engine::streaming::tools and are re-used here.
// Tauri-specific code (AppState access, search engine, enforcement engine) is
// in this file only.

use crate::domain::enforcement::RuleAction;
use crate::error::OrqaError;
use crate::state::AppState;

use std::path::{Path, PathBuf};

use orqa_engine::streaming::tools::{
    format_search_results, strip_frontmatter, tool_bash, tool_edit_file, tool_glob, tool_grep,
    tool_load_knowledge, tool_read_file, tool_write_file,
};

// Re-export constants so callers that import from this module continue to work.
pub use orqa_engine::streaming::tools::{
    truncate_tool_output, DEFAULT_READ_FILE_MAX_LINES, MAX_TOOL_OUTPUT_CHARS, READ_ONLY_TOOLS,
};

/// Result of running enforcement checks on a tool call.
///
/// Captures both blocking verdicts and knowledge injection content.
pub struct EnforcementResult {
    /// If set, the tool execution is blocked with this message.
    pub block_message: Option<String>,
    /// Knowledge content to inject into the agent's context.
    ///
    /// May be non-empty even when the tool is not blocked.
    pub injected_content: Option<String>,
}

/// Read a knowledge artifact's `.md` file and return its body (frontmatter stripped).
fn read_knowledge_content(project_dir: &Path, knowledge_name: &str) -> Option<String> {
    let knowledge_path = project_dir
        .join(".orqa")
        .join("process")
        .join("knowledge")
        .join(format!("{knowledge_name}.md"));
    let content = std::fs::read_to_string(&knowledge_path).ok()?;
    Some(strip_frontmatter(&content))
}

/// Collect and deduplicate knowledge content for Inject verdicts.
///
/// Reads each knowledge artifact from disk, marks it as injected in the `WorkflowTracker`,
/// and returns the combined content. Knowledge already injected this session is skipped.
fn collect_injected_knowledge(
    knowledge: &[String],
    state: &tauri::State<'_, AppState>,
    project_dir: &Path,
) -> Option<String> {
    if knowledge.is_empty() {
        return None;
    }

    let mut tracker = match state.session.workflow_tracker.lock() {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("[enforcement] workflow_tracker lock poisoned: {e}");
            return None;
        }
    };

    let mut parts: Vec<String> = Vec::new();
    for item in knowledge {
        if !tracker.mark_knowledge_injected(item) {
            tracing::debug!("[enforcement] knowledge '{item}' already injected, skipping");
            continue;
        }
        match read_knowledge_content(project_dir, item) {
            Some(body) => {
                tracing::debug!(
                    "[enforcement] injecting knowledge '{item}' ({} chars)",
                    body.len()
                );
                parts.push(format!("## Knowledge: {item}\n\n{body}"));
            }
            None => {
                tracing::warn!("[enforcement] knowledge '{item}' not found on disk, skipping");
            }
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n\n---\n\n"))
    }
}

/// Process enforcement verdicts and return a block message or collected injection knowledge.
///
/// Returns `Ok(knowledge_to_inject)` if no block occurred, or `Err(EnforcementResult)` with
/// the block message if a verdict blocks execution.
fn process_verdicts(
    verdicts: &[crate::domain::enforcement::Verdict],
    tool_label: &str,
    context_label: &str,
) -> Result<Vec<String>, EnforcementResult> {
    let mut all_inject_knowledge: Vec<String> = Vec::new();
    for verdict in verdicts {
        match verdict.action {
            RuleAction::Block => {
                tracing::debug!(
                    "[enforcement] BLOCK tool={tool_label} rule='{}' {context_label}",
                    verdict.rule_name
                );
                return Err(EnforcementResult {
                    block_message: Some(format!(
                        "Rule '{}' blocked this tool call.\n\n{}",
                        verdict.rule_name, verdict.message
                    )),
                    injected_content: None,
                });
            }
            RuleAction::Warn => {
                tracing::warn!(
                    "[enforcement] WARN tool={tool_label} rule='{}' {context_label}",
                    verdict.rule_name
                );
            }
            RuleAction::Inject => {
                tracing::debug!(
                    "[enforcement] INJECT tool={tool_label} rule='{}' {context_label} knowledge={:?}",
                    verdict.rule_name,
                    verdict.knowledge
                );
                all_inject_knowledge.extend(verdict.knowledge.clone());
            }
        }
    }
    Ok(all_inject_knowledge)
}

/// Acquire the enforcement engine lock and return the guard.
///
/// Returns `None` (with a default `EnforcementResult`) if the lock is poisoned
/// or the engine is not initialised.
fn lock_enforcement_engine<'a>(
    state: &'a tauri::State<'a, AppState>,
) -> Option<std::sync::MutexGuard<'a, Option<crate::domain::enforcement_engine::EnforcementEngine>>>
{
    match state.enforcement.engine.lock() {
        Ok(g) if g.is_some() => Some(g),
        Ok(_) => None,
        Err(e) => {
            tracing::warn!("[enforcement] lock poisoned: {e}");
            None
        }
    }
}

/// Build an `EnforcementResult` from collected knowledge, injecting content from disk.
fn build_enforcement_result(
    knowledge: Vec<String>,
    state: &tauri::State<'_, AppState>,
    project_dir: &Path,
) -> EnforcementResult {
    let injected_content = collect_injected_knowledge(&knowledge, state, project_dir);
    EnforcementResult {
        block_message: None,
        injected_content,
    }
}

/// Run enforcement checks for a file write/edit tool call.
///
/// Returns an `EnforcementResult` with an optional block message and/or
/// injected knowledge content.
pub fn enforce_file(
    tool_name: &str,
    file_path: &str,
    new_text: &str,
    state: &tauri::State<'_, AppState>,
    project_dir: &Path,
) -> EnforcementResult {
    let Some(guard) = lock_enforcement_engine(state) else {
        return EnforcementResult {
            block_message: None,
            injected_content: None,
        };
    };
    let Some(engine) = guard.as_ref() else {
        return EnforcementResult {
            block_message: None,
            injected_content: None,
        };
    };
    let verdicts = engine.evaluate_file(file_path, new_text);
    let context = format!("file='{file_path}'");
    let knowledge = match process_verdicts(&verdicts, tool_name, &context) {
        Ok(s) => s,
        Err(result) => return result,
    };
    drop(guard);
    build_enforcement_result(knowledge, state, project_dir)
}

/// Run enforcement checks for a bash tool call.
///
/// Returns an `EnforcementResult` with an optional block message and/or
/// injected knowledge content.
pub fn enforce_bash(
    command: &str,
    state: &tauri::State<'_, AppState>,
    project_dir: &Path,
) -> EnforcementResult {
    let Some(guard) = lock_enforcement_engine(state) else {
        return EnforcementResult {
            block_message: None,
            injected_content: None,
        };
    };
    let Some(engine) = guard.as_ref() else {
        return EnforcementResult {
            block_message: None,
            injected_content: None,
        };
    };
    let verdicts = engine.evaluate_bash(command);
    let context = format!("command='{command}'");
    let knowledge = match process_verdicts(&verdicts, "bash", &context) {
        Ok(s) => s,
        Err(result) => return result,
    };
    drop(guard);
    build_enforcement_result(knowledge, state, project_dir)
}

/// Run enforcement checks for the given tool and input.
///
/// Returns the enforcement result for write/edit/bash tools, or
/// a no-op result for all other tools.
fn run_enforcement_for_tool(
    tool_name: &str,
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
    root: &Path,
) -> EnforcementResult {
    match tool_name {
        "write_file" => {
            let file_path = input["path"].as_str().unwrap_or("");
            let new_text = input["content"].as_str().unwrap_or("");
            enforce_file(tool_name, file_path, new_text, state, root)
        }
        "edit_file" => {
            let file_path = input["path"].as_str().unwrap_or("");
            let new_text = input["new_string"].as_str().unwrap_or("");
            enforce_file(tool_name, file_path, new_text, state, root)
        }
        "bash" => {
            let command = input["command"].as_str().unwrap_or("");
            enforce_bash(command, state, root)
        }
        _ => EnforcementResult {
            block_message: None,
            injected_content: None,
        },
    }
}

/// Route a tool call to the appropriate handler function.
///
/// Pure tools (read_file, write_file, edit_file, bash, glob, grep, load_knowledge)
/// delegate to orqa_engine::streaming::tools. Search tools delegate to the search
/// engine held in AppState and are handled locally.
fn dispatch_tool(
    tool_name: &str,
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
    root: &Path,
) -> (String, bool) {
    match tool_name {
        "read_file" => tool_read_file(input, root),
        "write_file" => tool_write_file(input, root),
        "edit_file" => tool_edit_file(input, root),
        "bash" => tool_bash(input, root),
        "glob" => tool_glob(input, root),
        "grep" => tool_grep(input, root),
        "search_regex" => tool_search_regex(input, state),
        "search_semantic" => tool_search_semantic(input, state),
        "code_research" => tool_code_research(input, state),
        "load_knowledge" => tool_load_knowledge(input, root),
        _ => (format!("unknown tool: {tool_name}"), true),
    }
}

/// Prepend injected knowledge content to a tool output string.
fn prepend_injected_content(output: &mut String, content: String) {
    *output = format!(
        "[Enforcement: the following knowledge has been loaded for context]\n\n\
         {content}\n\n\
         [End of injected knowledge]\n\n\
         {output}"
    );
}

/// Dispatch a tool call to the appropriate handler.
///
/// Parses the input JSON, resolves the project root, runs enforcement checks,
/// dispatches to the matching handler, and prepends any injected knowledge.
/// Returns `(output, is_error)`.
pub fn execute_tool(
    tool_name: &str,
    input_json: &str,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    tracing::debug!("[tool] execute_tool called: tool={tool_name} input={input_json}");

    let input: serde_json::Value = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(e) => {
            tracing::debug!("[tool] JSON parse error: {e}");
            return (format!("invalid tool input JSON: {e}"), true);
        }
    };

    let root = match project_root(state) {
        Ok(r) => {
            tracing::debug!("[tool] project root: {}", r.display());
            r
        }
        Err(e) => {
            tracing::debug!("[tool] project root error: {e}");
            return (format!("cannot resolve project: {e}"), true);
        }
    };

    let enforcement = run_enforcement_for_tool(tool_name, &input, state, &root);
    if let Some(msg) = enforcement.block_message {
        return (msg, true);
    }

    let (mut output, is_error) = dispatch_tool(tool_name, &input, state, &root);

    if let Some(content) = enforcement.injected_content {
        prepend_injected_content(&mut output, content);
    }

    tracing::debug!(
        "[tool] result: is_error={is_error} output_len={} first_100={}",
        output.len(),
        &output[..output.len().min(100)]
    );
    (output, is_error)
}

/// Resolve the active project's root path for use as working directory.
pub fn project_root(state: &tauri::State<'_, AppState>) -> Result<PathBuf, String> {
    use crate::repo::project_repo;
    let conn = state.db.conn.lock().map_err(|e| format!("db lock: {e}"))?;
    let project = project_repo::get_active(&conn)
        .map_err(|e| format!("db query: {e}"))?
        .ok_or_else(|| "no active project".to_string())?;
    Ok(PathBuf::from(project.path))
}

/// Resolve the active project path from `AppState` without the Tauri wrapper.
///
/// Used in contexts where `tauri::State<'_, AppState>` is not available.
pub fn project_root_from_state(state: &AppState) -> Result<PathBuf, OrqaError> {
    use crate::repo::project_repo;
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("db lock: {e}")))?;
    let project = project_repo::get_active(&conn)
        .map_err(|e| OrqaError::Database(format!("db query: {e}")))?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_string()))?;
    Ok(PathBuf::from(project.path))
}

/// Search the indexed codebase with a regex pattern.
///
/// Delegates to the search engine held in AppState. Returns an error if the
/// search engine has not been initialized.
pub fn tool_search_regex(
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    let Some(pattern) = input["pattern"].as_str() else {
        return ("missing 'pattern' parameter".to_string(), true);
    };
    let path_filter = input["path"].as_str();
    let max_results = input["max_results"].as_u64().map_or(20, |n| n as u32);

    let search_guard = match state.search.engine.lock() {
        Ok(g) => g,
        Err(e) => return (format!("search lock error: {e}"), true),
    };
    let Some(engine) = search_guard.as_ref() else {
        return (
            "search index not initialized — index the codebase first".to_string(),
            true,
        );
    };

    match engine.search_regex(pattern, path_filter, max_results) {
        Ok(results) => (format_search_results(&results), false),
        Err(e) => (format!("search_regex failed: {e}"), true),
    }
}

/// Search the indexed codebase using semantic similarity.
///
/// Delegates to the search engine held in AppState. Returns an error if the
/// search engine has not been initialized.
pub fn tool_search_semantic(
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    let Some(query) = input["query"].as_str() else {
        return ("missing 'query' parameter".to_string(), true);
    };
    let max_results = input["max_results"].as_u64().map_or(10, |n| n as u32);

    let mut search_guard = match state.search.engine.lock() {
        Ok(g) => g,
        Err(e) => return (format!("search lock error: {e}"), true),
    };
    let Some(engine) = search_guard.as_mut() else {
        return (
            "search index not initialized — index the codebase first".to_string(),
            true,
        );
    };

    match engine.search_semantic(query, max_results) {
        Ok(results) => (format_search_results(&results), false),
        Err(e) => (format!("search_semantic failed: {e}"), true),
    }
}

/// Run semantic search and append results to the output buffer.
///
/// Returns `Err` with the error tuple if the search lock cannot be acquired.
fn append_semantic_results(
    query: &str,
    max_results: u32,
    state: &tauri::State<'_, AppState>,
    out: &mut String,
) -> Result<(), (String, bool)> {
    use std::fmt::Write;
    let mut guard = state
        .search
        .engine
        .lock()
        .map_err(|e| (format!("search lock error: {e}"), true))?;
    if let Some(engine) = guard.as_mut() {
        match engine.search_semantic(query, max_results) {
            Ok(results) if !results.is_empty() => {
                out.push_str("## Semantic Matches\n\n");
                out.push_str(&format_search_results(&results));
                out.push('\n');
            }
            Ok(_) => {}
            Err(e) => {
                let _ = write!(out, "(semantic search unavailable: {e})\n\n");
            }
        }
    }
    Ok(())
}

/// Run regex search and append results to the output buffer.
///
/// Returns `Err` with the error tuple if the search lock cannot be acquired.
fn append_regex_results(
    query: &str,
    max_results: u32,
    state: &tauri::State<'_, AppState>,
    out: &mut String,
) -> Result<(), (String, bool)> {
    use std::fmt::Write;
    let guard = state
        .search
        .engine
        .lock()
        .map_err(|e| (format!("search lock error: {e}"), true))?;
    if let Some(engine) = guard.as_ref() {
        let escaped = regex::escape(query);
        match engine.search_regex(&escaped, None, max_results) {
            Ok(results) if !results.is_empty() => {
                out.push_str("## Regex Matches\n\n");
                out.push_str(&format_search_results(&results));
            }
            Ok(_) => {}
            Err(e) => {
                let _ = write!(out, "(regex search unavailable: {e})\n\n");
            }
        }
    }
    Ok(())
}

/// Combined code research: runs both regex and semantic search, merging results.
///
/// Accepts a `query` string and optional `max_results`. The query is used as-is
/// for semantic search. For regex search, it is treated as a literal pattern
/// (special regex chars are escaped).
pub fn tool_code_research(
    input: &serde_json::Value,
    state: &tauri::State<'_, AppState>,
) -> (String, bool) {
    let Some(query) = input["query"].as_str() else {
        return ("missing 'query' parameter".to_string(), true);
    };
    let max_results = input["max_results"].as_u64().map_or(10, |n| n as u32);
    let half = max_results / 2 + 1;
    let mut out = String::new();

    if let Err(e) = append_semantic_results(query, half, state, &mut out) {
        return e;
    }
    if let Err(e) = append_regex_results(query, half, state, &mut out) {
        return e;
    }

    if out.is_empty() {
        (
            "search index not initialized — index the codebase first".to_string(),
            true,
        )
    } else if out.trim().is_empty() || (out.contains("unavailable") && !out.contains("Matches")) {
        ("no results found".to_string(), false)
    } else {
        (out, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_tool_output_short_output_unchanged() {
        let short = "hello world".to_string();
        let result = truncate_tool_output(short.clone());
        assert_eq!(result, short);
    }

    #[test]
    fn truncate_tool_output_exactly_at_limit_unchanged() {
        let at_limit = "x".repeat(MAX_TOOL_OUTPUT_CHARS);
        let result = truncate_tool_output(at_limit.clone());
        assert_eq!(result, at_limit);
    }

    #[test]
    fn truncate_tool_output_over_limit_includes_notice() {
        let over_limit = "x".repeat(MAX_TOOL_OUTPUT_CHARS + 500);
        let total_len = over_limit.len();
        let result = truncate_tool_output(over_limit);
        assert!(result.contains("[Output truncated:"));
        assert!(result.contains(&total_len.to_string()));
        assert!(result.len() > MAX_TOOL_OUTPUT_CHARS);
        // The first MAX_TOOL_OUTPUT_CHARS chars should be the 'x' characters
        assert!(result.starts_with(&"x".repeat(MAX_TOOL_OUTPUT_CHARS)));
    }

    #[test]
    fn read_only_tools_are_recognized() {
        let read_only = [
            "read_file",
            "glob",
            "grep",
            "search_regex",
            "search_semantic",
            "load_knowledge",
            "code_research",
        ];
        for tool in &read_only {
            assert!(
                READ_ONLY_TOOLS.contains(tool),
                "{tool} should be in READ_ONLY_TOOLS"
            );
        }
    }

    #[test]
    fn write_tools_are_not_read_only() {
        let write_tools = ["write_file", "edit_file", "bash"];
        for tool in &write_tools {
            assert!(
                !READ_ONLY_TOOLS.contains(tool),
                "{tool} must NOT be in READ_ONLY_TOOLS — it requires user approval"
            );
        }
    }

    #[test]
    fn read_file_line_limit_applies_truncation_notice() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut f = NamedTempFile::new().expect("temp file");
        for i in 0..2500 {
            writeln!(f, "line {i}").expect("write");
        }
        let path = f.path().to_path_buf();
        let root = path.parent().expect("parent").to_path_buf();
        let file_name = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();

        let input = serde_json::json!({ "path": file_name });
        let (output, is_error) = tool_read_file(&input, &root);
        assert!(!is_error, "should not be an error: {output}");
        assert!(
            output.contains("[File truncated:"),
            "should contain truncation notice"
        );
    }

    #[test]
    fn read_file_offset_and_limit_respected() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut f = NamedTempFile::new().expect("temp file");
        for i in 0..100 {
            writeln!(f, "line {i}").expect("write");
        }
        let path = f.path().to_path_buf();
        let root = path.parent().expect("parent").to_path_buf();
        let file_name = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();

        let input = serde_json::json!({ "path": file_name, "offset": 10, "limit": 5 });
        let (output, is_error) = tool_read_file(&input, &root);
        assert!(!is_error, "should not be an error: {output}");
        assert!(output.contains("line 10"));
        assert!(output.contains("line 14"));
        assert!(
            !output.contains("line 9"),
            "should not include line before offset"
        );
        assert!(
            !output.contains("line 15"),
            "should not include line past limit"
        );
    }

    #[test]
    fn read_file_small_file_no_truncation_notice() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut f = NamedTempFile::new().expect("temp file");
        for i in 0..10 {
            writeln!(f, "line {i}").expect("write");
        }
        let path = f.path().to_path_buf();
        let root = path.parent().expect("parent").to_path_buf();
        let file_name = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();

        let input = serde_json::json!({ "path": file_name });
        let (output, is_error) = tool_read_file(&input, &root);
        assert!(!is_error);
        assert!(!output.contains("[File truncated:"));
    }
}
