---
title: "Tool Definitions"
category: architecture
tags: []
created: 2026-03-02
updated: 2026-03-04
---

# Tool Definitions

**Date:** 2026-03-02 | **Status:** Phase 0e specification | **References:** [Claude Integration Research](/research/claude-integration) (AD-010)

This document defines the six core tools that Orqa Studio exposes to the Agent SDK sidecar as a custom MCP server. Each tool is implemented natively in Rust, registered via `createSdkMcpServer()`, and rendered in the conversation UI as a collapsible tool call card. The Agent SDK's built-in tools are disabled (`tools: []`) so that all tool execution flows through Orqa Studio.

---

## Table of Contents

- [MCP Server Registration](#mcp-server-registration)
- [Tool Definitions](#tool-definitions-1)
  - [Read](#read)
  - [Write](#write)
  - [Edit](#edit)
  - [Bash](#bash)
  - [Glob](#glob)
  - [Grep](#grep)
- [Tool Approval Matrix](#tool-approval-matrix)
- [Result Truncation](#result-truncation)
- [Security Model](#security-model)

---

## MCP Server Registration

Orqa Studio's tools are exposed to the Agent SDK sidecar as a custom MCP server. The sidecar registers the server when it spawns, and all tool calls from Claude route through it.

### Registration Flow

1. The Rust backend starts a stdio-based MCP server (stdin/stdout JSON-RPC 2.0) as part of the sidecar lifecycle.
2. The TypeScript sidecar registers it in the Agent SDK configuration:
   ```typescript
   const agent = new Agent({
     tools: [], // disable all built-in tools
     mcpServers: {
       orqa: {
         type: "stdio",
         command: "<path-to-orqa-mcp-process>",
       },
     },
   });
   ```
3. The MCP server responds to `tools/list` with all six tool definitions (schemas below).
4. When Claude emits a `tool_use` content block, the Agent SDK routes it to the Orqa Studio MCP server.
5. Orqa Studio executes the tool natively in Rust and returns the result via `tool_result`.
6. The sidecar forwards the result to Claude for the next conversation turn.

### MCP Protocol Details

- **Transport:** stdio (stdin/stdout), JSON-RPC 2.0 with Content-Length headers (standard MCP stdio transport).
- **Capabilities:** The server advertises `tools` capability only. No `resources`, `prompts`, or `sampling`.
- **Tool names:** Prefixed with `orqa_` in the MCP namespace to avoid collisions with user-provided MCP servers (e.g., `orqa_read`, `orqa_write`). Claude sees these as `Read`, `Write`, etc. via the `title` field.

### Rust Implementation

The MCP server is implemented in the Rust backend using direct JSON-RPC message parsing over stdin/stdout. Key crates:

| Crate | Purpose |
|-------|---------|
| `serde_json` | JSON-RPC message serialization/deserialization |
| `tokio` | Async I/O for stdin/stdout and tool execution |
| `schemars` | Derive JSON Schema from Rust structs for `tools/list` |

The MCP server process is spawned by the Rust backend as a child process (or runs as a thread within the main process, communicating with the sidecar via pipes allocated during sidecar spawn). The exact process topology is an implementation detail -- the contract is the MCP stdio protocol.

---

## Tool Definitions

### Read

**Description:** Reads a file from the local filesystem and returns its contents. Supports line range selection for large files.

#### MCP Tool Schema

```json
{
  "name": "orqa_read",
  "title": "Read",
  "description": "Reads a file from the local filesystem. Returns the file contents with line numbers. Supports optional line offset and limit for large files.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "file_path": {
        "type": "string",
        "description": "Absolute path to the file to read."
      },
      "offset": {
        "type": "integer",
        "minimum": 1,
        "description": "Line number to start reading from (1-indexed). Optional; defaults to 1."
      },
      "limit": {
        "type": "integer",
        "minimum": 1,
        "description": "Maximum number of lines to read. Optional; defaults to 2000."
      }
    },
    "required": ["file_path"],
    "additionalProperties": false
  }
}
```

#### Parameter Schema

| Parameter | Type | Required | Default | Validation |
|-----------|------|----------|---------|------------|
| `file_path` | `string` | Yes | -- | Must be an absolute path. Must pass path security validation (see [Security Model](#security-model)). Must not be a directory. |
| `offset` | `integer` | No | `1` | Must be >= 1. |
| `limit` | `integer` | No | `2000` | Must be >= 1. Clamped to 10000 max server-side. |

#### Rust Implementation

```rust
// Key crate: std::fs, tokio::fs
// 1. Validate path against security scopes
// 2. Read file with tokio::fs::read_to_string (or BufReader for large files)
// 3. Split into lines, apply offset/limit
// 4. Prepend line numbers (cat -n format: "{line_num}\t{content}")
// 5. Truncate lines longer than 2000 characters
// 6. Return formatted content
```

**Crates:** `tokio::fs` for async file I/O, `encoding_rs` for non-UTF-8 detection.

**Key logic:** Binary file detection (first 8KB heuristic). If binary, return an error message rather than garbled content. Lines longer than 2000 characters are truncated with a `[truncated]` suffix.

#### Result Format

**Success:**
```json
{
  "type": "text",
  "text": "     1\t// src/auth/handler.rs\n     2\tuse crate::auth::Session;\n     3\t..."
}
```

**Error:**
```json
{
  "type": "text",
  "text": "Error: File not found: /home/user/project/missing.rs",
  "isError": true
}
```

#### UI Rendering Spec

| Aspect | Specification |
|--------|---------------|
| **Icon** | `magnifying-glass` (lucide: `Search`) |
| **Collapsed summary** | `Read {relative_file_path}` -- path relative to project root |
| **Status badge** | `Completed` (green) or `Error` (red) |
| **Expanded: Input** | Key-value: `file_path`, `offset` (if non-default), `limit` (if non-default) |
| **Expanded: Result** | File contents in monospace with line numbers. Syntax highlighting based on file extension. If > 50 lines, show first 50 with "Show more" toggle. |
| **Expanded: Error** | Red-tinted background, warning icon, error message text. |

---

### Write

**Description:** Creates or overwrites a file with the provided content.

#### MCP Tool Schema

```json
{
  "name": "orqa_write",
  "title": "Write",
  "description": "Writes content to a file, creating it if it doesn't exist or overwriting if it does. Creates parent directories as needed.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "file_path": {
        "type": "string",
        "description": "Absolute path to the file to write."
      },
      "content": {
        "type": "string",
        "description": "The full content to write to the file."
      }
    },
    "required": ["file_path", "content"],
    "additionalProperties": false
  }
}
```

#### Parameter Schema

| Parameter | Type | Required | Default | Validation |
|-----------|------|----------|---------|------------|
| `file_path` | `string` | Yes | -- | Must be an absolute path. Must pass path security validation. Parent directory must be within allowed scope. |
| `content` | `string` | Yes | -- | No maximum enforced at schema level; subject to result truncation limits for logging. |

#### Rust Implementation

```rust
// Key crate: tokio::fs
// 1. Validate path against security scopes
// 2. Create parent directories if needed (tokio::fs::create_dir_all)
// 3. Read existing file content (if any) for diff generation
// 4. Write content atomically: write to temp file, then rename
// 5. Return success with diff (if file existed) or creation confirmation
```

**Crates:** `tokio::fs` for async file I/O, `tempfile` for atomic writes, `similar` for diff generation.

**Key logic:** Atomic write prevents partial writes on crash. If the file existed before, compute a unified diff between old and new content for the UI result. If it is a new file, indicate creation.

#### Result Format

**Success (new file):**
```json
{
  "type": "text",
  "text": "File created: /home/user/project/src/auth/constant_time.rs (42 lines)"
}
```

**Success (overwrite):**
```json
{
  "type": "text",
  "text": "File written: /home/user/project/src/auth/handler.rs (3 additions, 2 deletions)"
}
```

**Error:**
```json
{
  "type": "text",
  "text": "Error: Permission denied: /etc/passwd",
  "isError": true
}
```

#### UI Rendering Spec

| Aspect | Specification |
|--------|---------------|
| **Icon** | `pencil` (lucide: `Pencil`) |
| **Collapsed summary** | `Write {relative_file_path}` -- path relative to project root |
| **Status badge** | `Completed` (green) or `Error` (red) |
| **Expanded: Input** | Key-value: `file_path`. Content shown as monospace code block with syntax highlighting. If content > 50 lines, truncated with "Show more" toggle. |
| **Expanded: Result (new file)** | Green-tinted confirmation: "File created" with line count. Full content available via "Show content" toggle. |
| **Expanded: Result (overwrite)** | Diff view component (`DiffView.svelte`): red lines for deletions, green lines for additions. Unified diff format. |
| **Expanded: Error** | Red-tinted background, warning icon, error message text. |

---

### Edit

**Description:** Performs exact string replacement in a file. Finds `old_string` and replaces it with `new_string`.

#### MCP Tool Schema

```json
{
  "name": "orqa_edit",
  "title": "Edit",
  "description": "Performs an exact string replacement in a file. The old_string must appear exactly once in the file (unless replace_all is true). Preserves file encoding and line endings.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "file_path": {
        "type": "string",
        "description": "Absolute path to the file to edit."
      },
      "old_string": {
        "type": "string",
        "description": "The exact text to find and replace. Must match exactly (including whitespace and indentation)."
      },
      "new_string": {
        "type": "string",
        "description": "The replacement text. Must differ from old_string."
      },
      "replace_all": {
        "type": "boolean",
        "default": false,
        "description": "If true, replace all occurrences of old_string. If false (default), old_string must appear exactly once."
      }
    },
    "required": ["file_path", "old_string", "new_string"],
    "additionalProperties": false
  }
}
```

#### Parameter Schema

| Parameter | Type | Required | Default | Validation |
|-----------|------|----------|---------|------------|
| `file_path` | `string` | Yes | -- | Must be an absolute path. Must pass path security validation. File must exist. |
| `old_string` | `string` | Yes | -- | Must not be empty. Must exist in the file. If `replace_all` is false, must appear exactly once. |
| `new_string` | `string` | Yes | -- | Must differ from `old_string`. |
| `replace_all` | `boolean` | No | `false` | -- |

#### Rust Implementation

```rust
// Key crate: tokio::fs, similar
// 1. Validate path against security scopes
// 2. Read existing file content
// 3. Count occurrences of old_string in file content
// 4. If replace_all is false and count != 1, return error
// 5. If count == 0, return error (old_string not found)
// 6. Perform replacement: content.replace(old_string, new_string) or content.replacen(old_string, new_string, 1)
// 7. Write file atomically (temp file + rename)
// 8. Generate unified diff between old and new content
// 9. Return diff and success message
```

**Crates:** `tokio::fs` for async file I/O, `tempfile` for atomic writes, `similar` for diff generation.

**Key logic:** Exact string matching (not regex). The uniqueness check (exactly one occurrence unless `replace_all`) prevents ambiguous edits. Preserves original line endings (CRLF vs LF) by detecting the file's line ending style before replacement.

#### Result Format

**Success:**
```json
{
  "type": "text",
  "text": "--- a/src/auth/handler.rs\n+++ b/src/auth/handler.rs\n@@ -40,3 +40,3 @@\n-    if password == stored_hash {\n+    if constant_time_eq(password, stored_hash) {\n\nFile edited successfully (1 replacement)."
}
```

**Error (not unique):**
```json
{
  "type": "text",
  "text": "Error: old_string appears 3 times in /home/user/project/src/auth/handler.rs. Use replace_all: true to replace all occurrences, or provide a larger context string to make the match unique.",
  "isError": true
}
```

**Error (not found):**
```json
{
  "type": "text",
  "text": "Error: old_string not found in /home/user/project/src/auth/handler.rs. Ensure the string matches exactly, including whitespace and indentation.",
  "isError": true
}
```

#### UI Rendering Spec

| Aspect | Specification |
|--------|---------------|
| **Icon** | `pencil` (lucide: `Pencil`) |
| **Collapsed summary** | `Edit {relative_file_path}` -- path relative to project root |
| **Status badge** | `Completed` (green) or `Error` (red) |
| **Expanded: Input** | Key-value pairs: `file_path`, `old_string` (monospace, red-tinted background), `new_string` (monospace, green-tinted background), `replace_all` (if true). |
| **Expanded: Result** | Diff view component (`DiffView.svelte`): unified diff with red deletions and green additions. Context lines shown around the change. Success message below the diff. |
| **Expanded: Error** | Red-tinted background, warning icon, error message text. For "not unique" errors, show the count of occurrences found. |

---

### Bash

**Description:** Executes a shell command and returns its output (stdout and stderr).

#### MCP Tool Schema

```json
{
  "name": "orqa_bash",
  "title": "Bash",
  "description": "Executes a bash command in the project directory. Returns stdout and stderr. Commands run with the project root as the working directory. Long-running commands are terminated after a timeout.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "command": {
        "type": "string",
        "description": "The bash command to execute."
      },
      "timeout": {
        "type": "integer",
        "minimum": 1000,
        "maximum": 600000,
        "description": "Optional timeout in milliseconds. Default: 120000 (2 minutes). Maximum: 600000 (10 minutes)."
      }
    },
    "required": ["command"],
    "additionalProperties": false
  }
}
```

#### Parameter Schema

| Parameter | Type | Required | Default | Validation |
|-----------|------|----------|---------|------------|
| `command` | `string` | Yes | -- | Must not be empty. Subject to shell command restrictions (see [Security Model](#security-model)). |
| `timeout` | `integer` | No | `120000` | Must be between 1000 and 600000 (1 second to 10 minutes). |

#### Rust Implementation

```rust
// Key crate: tokio::process, tauri_plugin_shell
// 1. Validate command against shell restrictions
// 2. Spawn process via tauri-plugin-shell (pre-declared shell scope)
//    - Shell: bash (or sh fallback) on macOS/Linux, bash (Git Bash/MSYS2) on Windows
//    - Working directory: project root
//    - Environment: inherited, with PATH adjustments if needed
// 3. Capture stdout and stderr concurrently
// 4. Apply timeout via tokio::time::timeout
// 5. On completion: return exit code, stdout, stderr
// 6. On timeout: kill process tree, return timeout error
```

**Crates:** `tauri-plugin-shell` for sandboxed process spawning, `tokio::process` as fallback, `tokio::time` for timeout enforcement.

**Key logic:** Commands execute via `tauri-plugin-shell` which requires pre-declared shell scopes in `src-tauri/capabilities/default.json`. The shell scope uses argument validators (regex patterns) to restrict which commands can execute. Process group kill (`kill -- -$PGID`) ensures child processes are cleaned up on timeout.

**Security considerations:**
- Commands run with the privileges of the Orqa Studio process (the desktop user).
- Shell scopes in Tauri capabilities restrict which executables can be invoked.
- The working directory is always set to the project root.
- Environment variables are inherited but sensitive variables can be filtered.
- See [Security Model](#security-model) for the full restriction list.

#### Result Format

**Success:**
```json
{
  "type": "text",
  "text": "Exit code: 0\n\nStdout:\ntest result: ok. 42 passed; 0 failed; 0 ignored\n\nStderr:\n   Compiling orqa-studio v0.1.0"
}
```

**Error (non-zero exit):**
```json
{
  "type": "text",
  "text": "Exit code: 101\n\nStdout:\n\nStderr:\nerror[E0433]: failed to resolve: use of undeclared crate `constant_time`",
  "isError": true
}
```

**Error (timeout):**
```json
{
  "type": "text",
  "text": "Error: Command timed out after 120000ms. Process was terminated.\n\nPartial stdout:\n...\n\nPartial stderr:\n...",
  "isError": true
}
```

#### UI Rendering Spec

| Aspect | Specification |
|--------|---------------|
| **Icon** | `terminal` (lucide: `Terminal`) |
| **Collapsed summary** | `Bash {command}` -- the command string, truncated to 80 characters with ellipsis |
| **Status badge** | `Completed` (green) if exit code 0, `Error` (red) if non-zero or timeout |
| **Expanded: Input** | `command` displayed in monospace with dark background (shell-style). `timeout` shown only if non-default. |
| **Expanded: Result** | Monospace output block. Stdout and stderr shown sequentially (or interleaved if captured that way). Exit code shown as a small label. Horizontal scroll for long lines. If output > 100 lines, show first 100 with "Show full output" toggle. |
| **Expanded: Error** | Same monospace layout. Red-tinted background for stderr section. Warning icon next to exit code. |

---

### Glob

**Description:** Fast file pattern matching. Returns file paths matching a glob pattern.

#### MCP Tool Schema

```json
{
  "name": "orqa_glob",
  "title": "Glob",
  "description": "Searches for files matching a glob pattern. Returns matching file paths sorted by modification time (most recent first). Respects .gitignore rules.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "pattern": {
        "type": "string",
        "description": "Glob pattern to match files (e.g., '**/*.rs', 'src/**/*.ts', '*.json')."
      },
      "path": {
        "type": "string",
        "description": "Directory to search in. Optional; defaults to the project root."
      }
    },
    "required": ["pattern"],
    "additionalProperties": false
  }
}
```

#### Parameter Schema

| Parameter | Type | Required | Default | Validation |
|-----------|------|----------|---------|------------|
| `pattern` | `string` | Yes | -- | Must not be empty. Must be a valid glob pattern. |
| `path` | `string` | No | Project root | Must be an absolute path. Must pass path security validation. Must be a directory. |

#### Rust Implementation

```rust
// Key crate: globset, ignore
// 1. Validate path against security scopes
// 2. Build glob matcher from pattern (globset::Glob)
// 3. Walk directory tree using ignore::WalkBuilder (respects .gitignore)
// 4. Collect matching paths
// 5. Sort by modification time (most recent first)
// 6. Return list of absolute paths
```

**Crates:** `globset` for glob pattern compilation, `ignore` for directory walking with `.gitignore` support (same crate used by ripgrep), `tokio` for async execution.

**Key logic:** Uses the `ignore` crate's `WalkBuilder` which automatically respects `.gitignore`, `.git/info/exclude`, and global gitignore. Hidden files (dotfiles) are excluded by default. The walker is configured with reasonable defaults: follow symlinks = false, max depth = none (full recursion). Results are capped at 10,000 entries to prevent memory exhaustion on huge repositories.

#### Result Format

**Success:**
```json
{
  "type": "text",
  "text": "Found 23 files:\n/home/user/project/src/auth/handler.rs\n/home/user/project/src/auth/handler_test.rs\n/home/user/project/src/auth/mod.rs\n..."
}
```

**Success (no matches):**
```json
{
  "type": "text",
  "text": "No files matched the pattern '**/*.xyz' in /home/user/project"
}
```

**Error:**
```json
{
  "type": "text",
  "text": "Error: Invalid glob pattern: '**[': unclosed character class",
  "isError": true
}
```

#### UI Rendering Spec

| Aspect | Specification |
|--------|---------------|
| **Icon** | `list` (lucide: `List`) |
| **Collapsed summary** | `Glob {pattern}` -- the glob pattern |
| **Status badge** | `Completed` (green) or `Error` (red) |
| **Expanded: Input** | Key-value: `pattern`, `path` (if non-default, shown relative to project root). |
| **Expanded: Result** | List of file paths, each on its own line. Paths displayed relative to project root. File count shown as a header ("Found 23 files"). If > 50 files, show first 50 with "Show all ({count})" toggle. Each path is non-interactive in Phase 1 (clickable to open in Phase 2). |
| **Expanded: Error** | Red-tinted background, warning icon, error message text. |

---

### Grep

**Description:** Searches file contents for a regex pattern. Returns matching lines with context.

#### MCP Tool Schema

```json
{
  "name": "orqa_grep",
  "title": "Grep",
  "description": "Searches file contents for lines matching a regular expression pattern. Supports file type filtering, glob filtering, and context lines. Powered by ripgrep.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "pattern": {
        "type": "string",
        "description": "Regular expression pattern to search for."
      },
      "path": {
        "type": "string",
        "description": "File or directory to search in. Optional; defaults to the project root."
      },
      "glob": {
        "type": "string",
        "description": "Glob pattern to filter files (e.g., '*.rs', '*.{ts,tsx}'). Optional."
      },
      "type": {
        "type": "string",
        "description": "File type filter (e.g., 'rust', 'ts', 'py'). Uses ripgrep type definitions. Optional."
      },
      "case_insensitive": {
        "type": "boolean",
        "default": false,
        "description": "If true, perform case-insensitive matching."
      },
      "context": {
        "type": "integer",
        "minimum": 0,
        "maximum": 10,
        "description": "Number of context lines to show before and after each match. Default: 0."
      },
      "output_mode": {
        "type": "string",
        "enum": ["content", "files_with_matches", "count"],
        "default": "content",
        "description": "Output mode. 'content': show matching lines. 'files_with_matches': show only file paths. 'count': show match counts per file."
      },
      "head_limit": {
        "type": "integer",
        "minimum": 1,
        "description": "Limit output to the first N results. Optional; defaults to unlimited."
      }
    },
    "required": ["pattern"],
    "additionalProperties": false
  }
}
```

#### Parameter Schema

| Parameter | Type | Required | Default | Validation |
|-----------|------|----------|---------|------------|
| `pattern` | `string` | Yes | -- | Must not be empty. Must be a valid regex. |
| `path` | `string` | No | Project root | Must be an absolute path. Must pass path security validation. |
| `glob` | `string` | No | -- | Must be a valid glob pattern if provided. |
| `type` | `string` | No | -- | Must be a recognized ripgrep file type if provided. |
| `case_insensitive` | `boolean` | No | `false` | -- |
| `context` | `integer` | No | `0` | Must be between 0 and 10. |
| `output_mode` | `string` | No | `"content"` | Must be one of: `content`, `files_with_matches`, `count`. |
| `head_limit` | `integer` | No | Unlimited | Must be >= 1 if provided. |

#### Rust Implementation

```rust
// Key crate: grep_regex, grep_searcher, grep_matcher (ripgrep internals)
// 1. Validate path against security scopes
// 2. Build regex matcher (grep_regex::RegexMatcher)
// 3. Configure searcher (grep_searcher::Searcher) with context lines
// 4. Walk directory using ignore::WalkBuilder (respects .gitignore)
// 5. Apply glob/type filters
// 6. Search each file, collect matches
// 7. Format output based on output_mode
// 8. Apply head_limit if specified
// 9. Return formatted results
```

**Crates:** `grep-regex`, `grep-searcher`, `grep-matcher` (the ripgrep library crates), `ignore` for directory walking with `.gitignore` support.

**Key logic:** Uses ripgrep's library crates directly rather than spawning a `rg` subprocess. This gives native Rust performance and avoids requiring ripgrep to be installed on the user's system. The `ignore` crate provides `.gitignore`-aware file walking. Binary files are automatically skipped. Search results are capped at 10,000 matches to prevent memory exhaustion.

#### Result Format

**Success (content mode):**
```json
{
  "type": "text",
  "text": "src/auth/handler.rs:42:    if password == stored_hash {\nsrc/auth/handler.rs:58:    if constant_time_eq(token, stored_token) {\n\n2 matches in 1 file"
}
```

**Success (files_with_matches mode):**
```json
{
  "type": "text",
  "text": "src/auth/handler.rs\nsrc/auth/middleware.rs\nsrc/auth/session.rs\n\n3 files contain matches"
}
```

**Success (count mode):**
```json
{
  "type": "text",
  "text": "src/auth/handler.rs:2\nsrc/auth/middleware.rs:1\n\n3 matches in 2 files"
}
```

**Success (no matches):**
```json
{
  "type": "text",
  "text": "No matches found for pattern 'nonexistent_fn' in /home/user/project"
}
```

**Error:**
```json
{
  "type": "text",
  "text": "Error: Invalid regex pattern: '(unclosed': unclosed group",
  "isError": true
}
```

#### UI Rendering Spec

| Aspect | Specification |
|--------|---------------|
| **Icon** | `magnifying-glass` (lucide: `Search`) |
| **Collapsed summary** | `Grep "{pattern}" {path}` -- pattern in quotes, path relative to project root (or omitted if project root) |
| **Status badge** | `Completed` (green) or `Error` (red) |
| **Expanded: Input** | Key-value pairs: `pattern` (monospace), `path` (if non-default), `glob` (if set), `type` (if set), `case_insensitive` (if true), `context` (if non-zero), `output_mode` (if non-default). |
| **Expanded: Result (content)** | Monospace output. Each match shows `file:line:content`. Matched text within each line is highlighted (bold or colored). If > 50 result lines, show first 50 with "Show all ({count} matches)" toggle. |
| **Expanded: Result (files_with_matches)** | List of file paths relative to project root. File count header. |
| **Expanded: Result (count)** | File paths with match counts. Total count header. |
| **Expanded: Error** | Red-tinted background, warning icon, error message text. |

---

## Tool Approval Matrix

Phase 1 operates in auto-approve mode -- all tool calls execute immediately without user confirmation. Tool call results are displayed as read-only cards in the conversation. Phase 2 introduces the approval flow via the Agent SDK's `canUseTool` callback.

### Phase 1 (MVP): Auto-Approve All

All six tools execute immediately when invoked by Claude. The UI displays tool calls and their results after execution. This is sufficient for the dogfooding milestone where the user is the developer and trusts the tool execution within their own project.

### Phase 2: Approval Flow

The `canUseTool` callback in the Agent SDK sidecar routes approval requests to the Orqa Studio UI. The following matrix defines the default approval behavior. Users can customize this in settings.

| Tool | Default Behavior | Rationale |
|------|-----------------|-----------|
| **Read** | Auto-approve | Read-only, no side effects. |
| **Glob** | Auto-approve | Read-only, no side effects. |
| **Grep** | Auto-approve | Read-only, no side effects. |
| **Write** | Require approval | Creates or overwrites files. User should confirm. |
| **Edit** | Require approval | Modifies existing files. User should confirm. |
| **Bash** | Require approval | Executes arbitrary commands. Always confirm. |

**Approval UI:** When a tool requires approval, the tool call card renders in a `Pending` state (yellow badge) with `Approve` and `Deny` buttons. The conversation pauses until the user responds. Approved tools execute and transition to `Completed`/`Error`. Denied tools return a denial message to Claude, which can adjust its approach.

**Configurable overrides (Phase 2 settings):**
- "Trust this session" -- auto-approve all tools for the current session.
- Per-tool override -- e.g., auto-approve Edit for this project.
- "Always ask" -- require approval for all tools including Read.

---

## Result Truncation

Tool outputs can be large (full file contents, verbose command output, thousands of grep matches). Truncation prevents memory exhaustion in the sidecar, the Rust backend, and the UI.

### Truncation Rules

| Threshold | Action |
|-----------|--------|
| **> 1 MB total result text** | Truncate to 1 MB. Append `\n\n[Result truncated: {original_size} bytes, showing first 1 MB]`. The `isError` flag is NOT set (truncation is informational, not an error). |
| **> 10,000 lines (Read)** | Truncate to 10,000 lines. Append `\n\n[Output truncated: {total_lines} lines, showing first 10,000]`. |
| **> 10,000 matches (Grep)** | Stop searching after 10,000 matches. Append `\n\n[Search stopped: more than 10,000 matches. Narrow your pattern or use glob/type filters.]`. |
| **> 10,000 files (Glob)** | Stop collecting after 10,000 files. Append `\n\n[Results capped at 10,000 files. Narrow your pattern or specify a subdirectory.]`. |
| **> 1 MB (Bash stdout/stderr)** | Truncate combined output to 1 MB. Append truncation notice. Exit code is always preserved regardless of truncation. |
| **Single line > 2,000 characters (Read)** | Truncate line to 2,000 characters with `[truncated]` suffix. |

### UI-Level Truncation

The UI applies its own display limits independently of the MCP-level truncation:

| Tool | Default display limit | Expand action |
|------|----------------------|---------------|
| Read result | 50 lines | "Show more" loads remaining lines (from the full MCP result stored in the message) |
| Write/Edit diff | 50 lines | "Show full diff" |
| Bash output | 100 lines | "Show full output" |
| Glob file list | 50 files | "Show all ({count})" |
| Grep matches | 50 lines | "Show all ({count} matches)" |

UI truncation is purely a rendering concern -- the full result is stored in the `message_blocks` table and available on demand.

---

## Security Model

All tools operate within Orqa Studio's security model defined in AD-011. Security is enforced at the Rust level before any tool executes.

### Path Validation

Every tool that accepts a file path performs the following validation chain:

1. **Absolute path required:** Reject relative paths. All paths must be absolute.
2. **Canonicalization:** Resolve symlinks and `..` components via `std::fs::canonicalize()` to prevent path traversal attacks (e.g., `/project/../../../etc/passwd`).
3. **Scope check:** The canonicalized path must fall within one of:
   - The active project root directory (primary scope).
   - `$HOME` (broad scope, per AD-011), excluding denied paths.
4. **Denied path check:** Deny always takes precedence over allow. The following paths are always denied:

| Denied Path | Reason |
|-------------|--------|
| `~/.ssh/` | SSH private keys |
| `~/.gnupg/` | GPG private keys |
| `~/.aws/credentials` | AWS credentials |
| `~/.config/gcloud/` | GCP credentials |
| `~/.azure/` | Azure credentials |
| `~/.kube/config` | Kubernetes credentials |
| `~/.docker/config.json` | Docker credentials |
| `~/.netrc` | Network credentials |
| `~/.env` | Environment secrets (home directory) |
| Any path containing `.git/objects` | Git internal data (large, binary) |

5. **Tauri scope enforcement:** In addition to Orqa Studio's own validation, Tauri's compiled-in capability scopes provide a second layer of defense. If a path is outside the Tauri scope, the plugin-level call will fail even if Orqa Studio's validation has a bug.

### Shell Command Restrictions

The Bash tool operates under `tauri-plugin-shell` scope rules:

1. **Pre-declared commands:** Shell commands must be executed through a pre-declared shell scope in `src-tauri/capabilities/default.json`. The scope declares `bash` (or `sh`) as the allowed shell with argument validation.
2. **Working directory:** Always set to the project root. Commands cannot change the working directory to escape the project.
3. **No backgrounding by default:** Commands run in the foreground with a timeout. The tool supports an explicit timeout parameter but does not allow `&` backgrounding that could outlive the session.
4. **Process tree cleanup:** On timeout or cancellation, the entire process group is killed (not just the shell process).
5. **Environment filtering:** Sensitive environment variables (API keys, tokens) are removed from the command environment before execution.

### Write/Edit Safeguards

File modification tools include additional protections:

1. **Atomic writes:** Write and Edit use temp-file-then-rename to prevent partial writes on crash.
2. **No binary overwrite:** If the target file is detected as binary (and the content being written is text), warn in the result (but do not block -- some binary formats like SVG are text).
3. **Encoding preservation:** Edit detects the file's encoding (UTF-8, UTF-8 BOM, UTF-16, Latin-1) and preserves it. Write defaults to UTF-8.
4. **Line ending preservation:** Edit detects and preserves the file's line ending style (LF vs CRLF). Write uses the platform default.

### Rate Limiting

To prevent runaway tool loops (e.g., Claude repeatedly calling Bash in an infinite retry loop):

| Limit | Value | Action |
|-------|-------|--------|
| Max tool calls per turn | 50 | After 50 tool calls in a single Claude turn, return an error asking Claude to pause and summarize. |
| Max concurrent Bash executions | 3 | Queue additional Bash calls. Prevents fork bombs. |
| Max total Bash execution time per turn | 10 minutes | After 10 minutes cumulative Bash time in a single turn, reject further Bash calls. |

---

## Related Documents

- [Architecture Decisions](/architecture/decisions) -- AD-010 (tool implementation as MCP), AD-011 (security model)
- [MVP Specification](/product/mvp-specification) -- F-004 (tool call display), F-012 (MCP tool server)
- [Conversation View Wireframe](/wireframes/conversation-view) -- Tool call card visual specifications
- [Streaming Pipeline](/architecture/decisions) -- AD-009 (how tool events flow through the system)
- [SQLite Schema](/architecture/sqlite-schema) -- `message_blocks` table where tool call/result data is stored
