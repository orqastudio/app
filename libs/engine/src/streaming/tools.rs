// Pure tool implementations for the orqa-engine crate.
//
// Contains all tool handler functions that have no Tauri dependency.
// The app's tool executor calls these for file I/O, glob, grep, and bash
// operations. Search tools (search_regex, search_semantic, code_research)
// and enforcement-aware dispatch remain in the app layer because they depend
// on the search engine and enforcement engine stored in Tauri AppState.

use std::fmt::Write as FmtWrite;
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
///
/// These tools only read data and cannot modify the project. Write and execute
/// tools are not in this list and require explicit user approval.
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

/// Resolve a path from tool input relative to the project root.
///
/// Returns an error string if the resolved path escapes the project root.
/// Absolute paths are allowed but must still be within the project root.
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

/// Resolve a path for writing — the file may not exist yet.
///
/// Canonicalizes the parent directory instead of the full path, since the
/// target file may not exist at resolution time. Returns an error string if
/// the parent directory escapes the project root.
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

/// Read a file's contents, with optional line offset and limit.
///
/// Parameters in `input`:
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
///
/// Overwrites any existing content at the path. Returns a success message
/// with the byte count written, or an error message.
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

/// Edit a file by replacing old_string with new_string exactly once.
///
/// Fails if `old_string` is not found or appears more than once, to prevent
/// ambiguous replacements. Returns a success message or an error message.
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
///
/// Used to enforce the bash timeout: when a command exceeds `BASH_TIMEOUT`,
/// the process tree is killed to free resources.
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

/// Spawn a background thread that reads from a stdout pipe into a string (capped at MAX_PIPE_BYTES).
///
/// Caps the read at `MAX_PIPE_BYTES` to prevent unbounded memory use when a
/// command produces very large output.
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

/// Spawn a background thread that reads from a stderr pipe into a string (capped at MAX_PIPE_BYTES).
///
/// Caps the read at `MAX_PIPE_BYTES` to prevent unbounded memory use when a
/// command produces very large output.
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
///
/// Stdout content appears first; stderr is appended under a "STDERR:" heading.
/// Returns "(no output)" if both streams are empty.
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
///
/// Spawns `bash -c <command>` with `root` as the working directory. Captures
/// stdout and stderr separately, combines them in the result, and enforces a
/// 120-second timeout. Kills the process tree on timeout.
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
///
/// Parameters in `input`:
/// - `pattern` (required): glob pattern relative to `path` or the project root.
/// - `path` (optional): subdirectory to search within, relative to `root`.
///
/// Returns newline-separated matched paths (relative to project root) or an error.
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

/// Search file contents with a regex pattern using ripgrep or grep.
///
/// Parameters in `input`:
/// - `pattern` (required): regex pattern to search for.
/// - `path` (optional): subdirectory to search within, relative to `root`.
///
/// Uses `rg` (ripgrep) when available, falling back to `grep`. Limits to
/// 200 matches to prevent unbounded output.
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

/// Simple shell escaping — wraps the argument in single quotes.
///
/// Single quotes inside the argument are escaped by ending the single-quote
/// context, inserting a literal quote, and resuming the single-quote context.
fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Load the full content of a knowledge artifact from `.orqa/documentation/knowledge/{name}.md`.
///
/// Validates that `name` contains no path separators to prevent directory traversal.
/// Returns the raw file content (including frontmatter) or an informative error.
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

/// Strip YAML frontmatter from a markdown document.
///
/// If the content starts with `---`, everything up to and including the
/// closing `---` line is removed. Returns the body content trimmed of
/// leading whitespace.
pub fn strip_frontmatter(content: &str) -> String {
    if !content.starts_with("---") {
        return content.to_string();
    }
    if let Some(end) = content[3..].find("\n---") {
        content[3 + end + 4..].trim_start().to_string()
    } else {
        content.to_string()
    }
}

/// Format search results as a readable text block.
///
/// Each result appears as `filepath:start-end\ncontent\n---\n`.
/// Returns "no matches found" for an empty result set.
pub fn format_search_results(results: &[crate::search::SearchResult]) -> String {
    if results.is_empty() {
        return "no matches found".to_string();
    }
    let mut out = String::new();
    for result in results {
        let _ = FmtWrite::write_fmt(
            &mut out,
            format_args!(
                "{}:{}-{}\n{}\n---\n",
                result.file_path, result.start_line, result.end_line, result.content,
            ),
        );
    }
    out
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

    #[test]
    fn strip_frontmatter_removes_yaml_block() {
        let content = "---\ntitle: test\n---\nBody content here.";
        let result = strip_frontmatter(content);
        assert_eq!(result, "Body content here.");
        assert!(!result.contains("title:"));
    }

    #[test]
    fn strip_frontmatter_no_frontmatter_unchanged() {
        let content = "Body content here.";
        let result = strip_frontmatter(content);
        assert_eq!(result, content);
    }
}
