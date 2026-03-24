use crate::domain::enforcement::RuleAction;
use crate::error::OrqaError;
use crate::state::AppState;

use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

/// Maximum number of characters to return from a single tool output.
///
/// Outputs exceeding this limit are truncated with an explanatory message.
/// This prevents context window overflow when tools return very large results.
pub const MAX_TOOL_OUTPUT_CHARS: usize = 100_000;

/// Default maximum number of lines returned by `read_file`.
pub const DEFAULT_READ_FILE_MAX_LINES: usize = 2000;

/// Tool names that are read-only and can be auto-approved without user interaction.
pub const READ_ONLY_TOOLS: &[&str] = &[
    "read_file",
    "glob",
    "grep",
    "search_regex",
    "search_semantic",
    "load_knowledge",
    "code_research",
];

/// Truncate a tool output string to `MAX_TOOL_OUTPUT_CHARS` characters.
///
/// When the output exceeds the limit, the returned string contains the first
/// `MAX_TOOL_OUTPUT_CHARS` characters followed by a clear truncation notice.
/// This prevents context window overflow when tools return very large results.
pub fn truncate_tool_output(output: String) -> String {
    if output.len() <= MAX_TOOL_OUTPUT_CHARS {
        return output;
    }
    let truncated = &output[..MAX_TOOL_OUTPUT_CHARS];
    format!(
        "{truncated}\n\n[Output truncated: {} chars total, showing first {MAX_TOOL_OUTPUT_CHARS}]",
        output.len()
    )
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

/// Resolve a path from tool input relative to the project root.
/// Returns an error string if the resolved path escapes the project root.
pub fn resolve_path(raw: &str, root: &Path) -> Result<PathBuf, String> {
    let candidate = if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    };

    let resolved = candidate
        .canonicalize()
        .unwrap_or_else(|_| candidate.clone());
    let root_canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    if !resolved.starts_with(&root_canon) {
        return Err(format!("path '{raw}' is outside the project root"));
    }
    Ok(resolved)
}

/// Resolve a path for writing — the file may not exist yet, so we
/// canonicalize the parent directory instead.
pub fn resolve_write_path(raw: &str, root: &Path) -> Result<PathBuf, String> {
    let candidate = if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    };

    let root_canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    if let Some(parent) = candidate.parent() {
        let parent_resolved = parent
            .canonicalize()
            .unwrap_or_else(|_| parent.to_path_buf());
        if !parent_resolved.starts_with(&root_canon) {
            return Err(format!("path '{raw}' is outside the project root"));
        }
    }
    Ok(candidate)
}

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

/// Strip YAML frontmatter from a markdown document.
///
/// If the content starts with `---`, everything up to and including the
/// closing `---` line is removed. Returns the body content trimmed of
/// leading whitespace.
fn strip_frontmatter(content: &str) -> String {
    if !content.starts_with("---") {
        return content.to_string();
    }
    if let Some(end) = content[3..].find("\n---") {
        content[3 + end + 4..].trim_start().to_string()
    } else {
        content.to_string()
    }
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

/// Read a file's contents, with optional line offset and limit.
///
/// Parameters:
/// - `path` (required): path to the file, relative to the project root.
/// - `offset` (optional, default 0): 0-based line number to start from.
/// - `limit` (optional, default 2000): maximum number of lines to return.
///
/// If the file contains more lines than the effective limit, a truncation notice
/// is appended so the caller knows additional lines exist.
pub fn tool_read_file(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let Some(raw_path) = input["path"].as_str() else {
        return ("missing 'path' parameter".to_string(), true);
    };

    let path = match resolve_path(raw_path, root) {
        Ok(p) => p,
        Err(e) => return (e, true),
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return (format!("failed to read '{}': {e}", path.display()), true),
    };

    let offset = input["offset"].as_u64().unwrap_or(0) as usize;
    let limit = input["limit"]
        .as_u64()
        .map_or(DEFAULT_READ_FILE_MAX_LINES, |n| n as usize);

    let all_lines: Vec<&str> = contents.lines().collect();
    let total_lines = all_lines.len();

    if offset >= total_lines && total_lines > 0 {
        return (
            format!("offset {offset} is past end of file ({total_lines} lines total)"),
            true,
        );
    }

    let end = (offset + limit).min(total_lines);
    let selected = &all_lines[offset..end];
    let result = selected.join("\n");

    if end < total_lines {
        (
            format!(
                "{result}\n\n[File truncated: showing lines {}-{} of {total_lines} total. \
                 Use offset/limit parameters for specific ranges.]",
                offset + 1,
                end,
            ),
            false,
        )
    } else {
        (result, false)
    }
}

/// Write content to a file, creating parent directories as needed.
pub fn tool_write_file(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let Some(raw_path) = input["path"].as_str() else {
        return ("missing 'path' parameter".to_string(), true);
    };
    let Some(content) = input["content"].as_str() else {
        return ("missing 'content' parameter".to_string(), true);
    };

    let path = match resolve_write_path(raw_path, root) {
        Ok(p) => p,
        Err(e) => return (e, true),
    };

    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return (format!("failed to create directories: {e}"), true);
        }
    }

    match std::fs::write(&path, content) {
        Ok(()) => (
            format!("wrote {} bytes to '{}'", content.len(), path.display()),
            false,
        ),
        Err(e) => (format!("failed to write '{}': {e}", path.display()), true),
    }
}

/// Edit a file by replacing old_string with new_string.
pub fn tool_edit_file(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let Some(raw_path) = input["path"].as_str() else {
        return ("missing 'path' parameter".to_string(), true);
    };
    let Some(old_string) = input["old_string"].as_str() else {
        return ("missing 'old_string' parameter".to_string(), true);
    };
    let Some(new_string) = input["new_string"].as_str() else {
        return ("missing 'new_string' parameter".to_string(), true);
    };

    let path = match resolve_path(raw_path, root) {
        Ok(p) => p,
        Err(e) => return (e, true),
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return (format!("failed to read '{}': {e}", path.display()), true),
    };

    let count = contents.matches(old_string).count();
    if count == 0 {
        return (
            format!("old_string not found in '{}'", path.display()),
            true,
        );
    }
    if count > 1 {
        return (
            format!(
                "old_string found {count} times in '{}' — must be unique",
                path.display()
            ),
            true,
        );
    }

    let updated = contents.replacen(old_string, new_string, 1);
    match std::fs::write(&path, &updated) {
        Ok(()) => (
            format!("edited '{}' (1 replacement)", path.display()),
            false,
        ),
        Err(e) => (format!("failed to write '{}': {e}", path.display()), true),
    }
}

/// Kill a process and its children by PID.
fn kill_process_tree(pid: u32) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .output();
    }
    #[cfg(unix)]
    {
        let _ = std::process::Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output();
    }
}

/// Maximum time a bash command is allowed to run before being killed.
const BASH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(120);

/// Maximum bytes to read from stdout/stderr each to prevent OOM.
const MAX_PIPE_BYTES: usize = 512_000;

/// Spawn a background thread that reads from a pipe into a string (capped).
fn spawn_pipe_reader(pipe: Option<std::process::ChildStdout>) -> std::thread::JoinHandle<String> {
    use std::io::Read;
    std::thread::spawn(move || {
        let mut buf = String::new();
        if let Some(p) = pipe {
            let _ = p.take(MAX_PIPE_BYTES as u64).read_to_string(&mut buf);
        }
        buf
    })
}

/// Spawn a background thread that reads from a stderr pipe into a string (capped).
fn spawn_stderr_reader(pipe: Option<std::process::ChildStderr>) -> std::thread::JoinHandle<String> {
    use std::io::Read;
    std::thread::spawn(move || {
        let mut buf = String::new();
        if let Some(p) = pipe {
            let _ = p.take(MAX_PIPE_BYTES as u64).read_to_string(&mut buf);
        }
        buf
    })
}

/// Combine stdout and stderr into a single result string.
fn assemble_bash_output(stdout: String, stderr: String) -> String {
    let mut result = String::new();
    if !stdout.is_empty() {
        result.push_str(&stdout);
    }
    if !stderr.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str("STDERR:\n");
        result.push_str(&stderr);
    }
    if result.is_empty() {
        result.push_str("(no output)");
    }
    result
}

/// Execute a bash command in the project root.
pub fn tool_bash(input: &serde_json::Value, root: &Path) -> (String, bool) {
    use std::process::Stdio;

    let Some(command) = input["command"].as_str() else {
        return ("missing 'command' parameter".to_string(), true);
    };

    let mut child = match std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return (format!("failed to execute bash: {e}"), true),
    };

    let stdout_handle = spawn_pipe_reader(child.stdout.take());
    let stderr_handle = spawn_stderr_reader(child.stderr.take());
    let child_id = child.id();

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(child.wait());
    });

    match rx.recv_timeout(BASH_TIMEOUT) {
        Ok(Ok(status)) => {
            let stdout = stdout_handle.join().unwrap_or_default();
            let stderr = stderr_handle.join().unwrap_or_default();
            (assemble_bash_output(stdout, stderr), !status.success())
        }
        Ok(Err(e)) => (format!("failed to wait on bash process: {e}"), true),
        Err(_) => {
            kill_process_tree(child_id);
            let _ = stdout_handle.join();
            let _ = stderr_handle.join();
            (
                format!(
                    "Command timed out after {} seconds and was killed.",
                    BASH_TIMEOUT.as_secs()
                ),
                true,
            )
        }
    }
}

/// Find files matching a glob pattern.
pub fn tool_glob(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let Some(pattern) = input["pattern"].as_str() else {
        return ("missing 'pattern' parameter".to_string(), true);
    };

    let search_root = match input["path"].as_str() {
        Some(p) => root.join(p),
        None => root.to_path_buf(),
    };

    let full_pattern = search_root.join(pattern);
    let pattern_str = full_pattern.to_string_lossy();

    match glob::glob(&pattern_str) {
        Ok(entries) => {
            let mut paths: Vec<String> = Vec::new();
            for entry in entries {
                match entry {
                    Ok(path) => {
                        let display = path
                            .strip_prefix(root)
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .to_string();
                        paths.push(display);
                    }
                    Err(e) => {
                        paths.push(format!("(error: {e})"));
                    }
                }
            }
            if paths.is_empty() {
                ("no matches found".to_string(), false)
            } else {
                (paths.join("\n"), false)
            }
        }
        Err(e) => (format!("invalid glob pattern: {e}"), true),
    }
}

/// Search file contents with a regex pattern.
pub fn tool_grep(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let Some(pattern) = input["pattern"].as_str() else {
        return ("missing 'pattern' parameter".to_string(), true);
    };

    let search_path = match input["path"].as_str() {
        Some(p) => root.join(p),
        None => root.to_path_buf(),
    };

    let search_str = search_path.to_string_lossy();
    // Use --max-count to limit output at the source, preventing unbounded reads
    let cmd = format!(
        "rg --no-heading --line-number --color never --max-count 200 -e {} {} 2>/dev/null || grep -rn -m 200 {} {} 2>/dev/null",
        shell_escape(pattern),
        shell_escape(&search_str),
        shell_escape(pattern),
        shell_escape(&search_str),
    );

    let output = match std::process::Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .current_dir(root)
        .output()
    {
        Ok(o) => o,
        Err(e) => return (format!("failed to execute grep: {e}"), true),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return ("no matches found".to_string(), false);
    }

    let lines: Vec<&str> = stdout.lines().collect();
    if lines.len() > 200 {
        let truncated: String = lines[..200].join("\n");
        (
            format!(
                "{truncated}\n\n... ({} total matches, showing first 200)",
                lines.len()
            ),
            false,
        )
    } else {
        (stdout.to_string(), false)
    }
}

/// Simple shell escaping — wraps in single quotes.
fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Format search results as a readable text block.
fn format_search_results(results: &[crate::search::types::SearchResult]) -> String {
    if results.is_empty() {
        return "no matches found".to_string();
    }
    let mut out = String::new();
    for result in results {
        let _ = write!(
            out,
            "{}:{}-{}\n{}\n---\n",
            result.file_path, result.start_line, result.end_line, result.content,
        );
    }
    out
}

/// Search the indexed codebase with a regex pattern.
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

/// Load the full content of a knowledge artifact from `.orqa/process/knowledge/{name}.md`.
pub fn tool_load_knowledge(input: &serde_json::Value, root: &Path) -> (String, bool) {
    let Some(name) = input["name"].as_str() else {
        return ("missing 'name' parameter".to_string(), true);
    };

    // Validate knowledge name: must be a simple filename with no path separators
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return (
            format!("invalid knowledge name '{name}': must not contain path separators"),
            true,
        );
    }

    let knowledge_path = root
        .join(".orqa")
        .join("process")
        .join("knowledge")
        .join(format!("{name}.md"));

    match std::fs::read_to_string(&knowledge_path) {
        Ok(contents) => (contents, false),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (
            format!(
                "knowledge '{name}' not found at '{}'",
                knowledge_path.display()
            ),
            true,
        ),
        Err(e) => (format!("failed to read knowledge '{name}': {e}"), true),
    }
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
