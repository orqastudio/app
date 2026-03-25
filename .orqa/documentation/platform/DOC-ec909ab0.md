---
id: DOC-ec909ab0
type: doc
title: Tool Definitions
category: reference
description: "Definitions, parameters, and execution model for tools available to the AI agent within OrqaStudio sessions."
created: 2026-03-02
updated: 2026-03-10
sort: 16
relationships:
  - target: AD-d01b9e0a
    type: documents
  - target: AD-e4a3b5da
    type: documents
---

**Date:** 2026-03-02 | **References:** [AD-e4a3b5da](AD-e4a3b5da), [AD-d01b9e0a](AD-d01b9e0a)

This document defines the 10 tools that OrqaStudio exposes to the Claude Agent SDK. Tools are implemented in Rust in `backend/src-tauri/src/domain/tool_executor.rs` and registered in the sidecar's in-process MCP server (`sidecar/src/providers/claude-agent.ts`). Tool names use no prefix — the agent sees them as `read_file`, `write_file`, etc.

---

## Table of Contents

- [Execution Model](#execution-model)
- [Tool List](#tool-list)
  - [read_file](#read_file)
  - [write_file](#write_file)
  - [edit_file](#edit_file)
  - [bash](#bash)
  - [glob](#glob)
  - [grep](#grep)
  - [search_regex](#search_regex)
  - [search_semantic](#search_semantic)
  - [code_research](#code_research)
  - [load_skill](#load_skill)
- [Approval Model](#approval-model)
- [Output Truncation](#output-truncation)
- [Security Model](#security-model)

---

## Execution Model

Tools run in-process in the Rust backend. There is no separate MCP server process.

### Registration

The sidecar creates an in-process MCP server via `createSdkMcpServer()` from the Agent SDK. Each of the 10 tools is registered with a name, description, and Zod parameter schema. When the Agent SDK invokes a tool, the sidecar calls `executeToolViaRust()`.

### Execution flow

```
Agent SDK invokes tool
        |
        v
sidecar: executeToolViaRust()
    emits tool_execute NDJSON to stdout
    emits tool_use_start NDJSON to stdout
    waits for tool_result on stdin
        |
        v
Rust: stream_loop.rs receives tool_execute
    calls execute_tool(name, input, state)  ← domain/tool_executor.rs
    runs enforcement checks (write/bash tools)
    dispatches to tool handler
    truncates output if > 100,000 chars
    writes tool_result NDJSON to sidecar stdin
        |
        v
sidecar: resolves pending promise
    emits tool_result NDJSON to stdout
    returns content to Agent SDK
```

### Enforcement

Before executing `write_file`, `edit_file`, and `bash`, the tool executor runs enforcement checks via the loaded `EnforcementEngine`. A `Block` verdict returns an error to the agent. A `Warn` verdict logs and continues.

### Key constants (tool_executor.rs)

| Constant | Value | Purpose |
|----------|-------|---------|
| `MAX_TOOL_OUTPUT_CHARS` | 100,000 | Truncation threshold for all tool outputs |
| `DEFAULT_READ_FILE_MAX_LINES` | 2,000 | Default line limit for `read_file` |

---

## Tool List

### read_file

Read a file's contents with optional line range selection.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `path` | string | Yes | — | Path to the file. Absolute or relative to project root. |
| `offset` | integer | No | 0 | Zero-based line number to start reading from. |
| `limit` | integer | No | 2000 | Maximum number of lines to return. |

**Behaviour:** Lines are returned as plain text (no line-number prefix). When the file has more lines than the effective limit, a truncation notice is appended: `[File truncated: showing lines N-M of T total. Use offset/limit parameters for specific ranges.]`

**Read-only:** Yes — auto-approved without user interaction.

---

### write_file

Write content to a file, creating parent directories as needed. If the file already exists it is overwritten.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | Yes | Path to the file. Absolute or relative to project root. |
| `content` | string | Yes | Full content to write. |

**Behaviour:** Parent directories are created with `create_dir_all` if they do not exist. The file is written with `std::fs::write`. Returns a message with the byte count written.

**Read-only:** No — requires user approval (or enforcement pre-check).

---

### edit_file

Replace an exact string in a file. The old string must appear exactly once unless the replacement fails with a count error.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | string | Yes | Path to the file. Must exist. |
| `old_string` | string | Yes | The exact text to find. Must appear exactly once. |
| `new_string` | string | Yes | The replacement text. |

**Behaviour:** Reads the file, counts occurrences of `old_string`. Returns an error if count is 0 (not found) or > 1 (ambiguous). On success, writes the file with one replacement applied.

**Read-only:** No — requires user approval (or enforcement pre-check).

---

### bash

Execute a shell command in the project root directory.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `command` | string | Yes | Shell command to run via `bash -c`. |

**Behaviour:** Spawns a `bash -c <command>` subprocess with the project root as working directory. Stdout and stderr are captured concurrently in background threads (capped at 512,000 bytes each). The process is killed after a 120-second timeout. On timeout, returns an error message. On success, returns stdout followed by stderr (with a `STDERR:` prefix if stderr is non-empty).

**Read-only:** No — requires user approval (or enforcement pre-check).

---

### glob

Find files matching a glob pattern.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `pattern` | string | Yes | Glob pattern (e.g. `**/*.rs`, `src/**/*.ts`). |
| `path` | string | No | Directory to search in. Defaults to project root. |

**Behaviour:** Joins the search directory with the pattern and expands it using the `glob` crate. Returns matching paths relative to the project root, one per line. Returns `"no matches found"` if the pattern matches nothing.

**Read-only:** Yes — auto-approved without user interaction.

---

### grep

Search file contents for lines matching a regular expression.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `pattern` | string | Yes | Regular expression to search for. |
| `path` | string | No | File or directory to search. Defaults to project root. |

**Behaviour:** Runs `rg` (ripgrep) if available, falling back to `grep`. Uses `--max-count 200` to bound output. Returns matching lines in `file:line:content` format. Caps output at 200 lines with a notice if exceeded.

**Read-only:** Yes — auto-approved without user interaction.

---

### search_regex

Search the indexed codebase with an exact regex pattern. Requires the codebase to be indexed first.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `pattern` | string | Yes | — | Regular expression pattern. |
| `path` | string | No | — | Optional path filter. |
| `max_results` | integer | No | 20 | Maximum results to return. |

**Behaviour:** Delegates to the native search engine (`state.search`). Returns results as `file_path:start_line-end_line\ncontent\n---\n` blocks. Returns an error if the search index has not been initialized.

**Read-only:** Yes — auto-approved without user interaction.

---

### search_semantic

Search the indexed codebase using natural language. Requires the codebase to be indexed with embeddings first.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | string | Yes | — | Natural language query describing the code to find. |
| `max_results` | integer | No | 10 | Maximum results to return. |

**Behaviour:** Delegates to the native search engine's semantic search capability using ONNX-based embeddings and vector similarity. Returns results in the same format as `search_regex`. Returns an error if the search index has not been initialized.

**Read-only:** Yes — auto-approved without user interaction.

---

### code_research

Combined regex and semantic search. Best for understanding how a feature works end-to-end.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | string | Yes | — | A natural language query or identifier to research. |
| `max_results` | integer | No | 10 | Total maximum results (split between regex and semantic). |

**Behaviour:** Runs both semantic search (using the query verbatim) and regex search (using the query with special characters escaped). Merges results into two sections: `## Semantic Matches` and `## Regex Matches`. Returns an error if the search index has not been initialized.

**Read-only:** Yes — auto-approved without user interaction.

---

### load_skill

Load the full content of a project skill document by name.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Knowledge name (e.g. `rust-async-patterns`). Must not contain path separators. |

**Behaviour:** Reads `.orqa/knowledge/{name}/KNOW.md` from the project root. Returns the file contents on success. Returns an error if the knowledge name contains `/`, `\`, or `..`, or if the file does not exist.

**Read-only:** Yes — auto-approved without user interaction.

---

## Approval Model

Tool calls flow through two layers before execution:

### Layer 1: canUseTool (sidecar)

The Agent SDK's `canUseTool` callback fires for every tool invocation. The sidecar emits `tool_approval_request` for every call. Rust's stream loop auto-approves read-only tools. Write/execute tools pause the stream and wait for user input.

### Layer 2: Enforcement (Rust)

Before `write_file`, `edit_file`, and `bash` execute, the `EnforcementEngine` evaluates governance rules. A `Block` verdict aborts the tool call and returns an error to the agent.

### Read-only vs write tools

| Tool | Auto-approved | Notes |
|------|--------------|-------|
| `read_file` | Yes | |
| `glob` | Yes | |
| `grep` | Yes | |
| `search_regex` | Yes | |
| `search_semantic` | Yes | |
| `load_skill` | Yes | |
| `code_research` | Yes | |
| `write_file` | No | Requires user approval + enforcement check |
| `edit_file` | No | Requires user approval + enforcement check |
| `bash` | No | Requires user approval + enforcement check |

---

## Output Truncation

All tool outputs are passed through `truncate_tool_output()` before being sent to the sidecar. Any output exceeding 100,000 characters is truncated with a notice appended:

```
[Output truncated: N chars total, showing first 100000]
```

The `read_file` tool applies its own line-based truncation first (at `DEFAULT_READ_FILE_MAX_LINES = 2000`) before the character-level truncation applies.

---

## Security Model

### Path resolution

All path-accepting tools resolve paths relative to the project root using `resolve_path()` or `resolve_write_path()`. Absolute paths are accepted. The resolved path is checked with `starts_with(root_canon)` — paths outside the project root are rejected.

For write operations, where the file may not yet exist, `resolve_write_path()` canonicalizes the parent directory instead.

### Bash sandboxing

`bash` commands run via `std::process::Command::new("bash")` with `current_dir` set to the project root. There is no shell scope restriction at the OS level — the command runs with the user's full privileges. Enforcement rules (`enforce_bash`) are the primary guard against dangerous commands. A 120-second timeout kills the process on overrun.

---

## Related Documents

- Architecture Decisions — [AD-e4a3b5da](AD-e4a3b5da) (tool execution model), [AD-d01b9e0a](AD-d01b9e0a) (security model)
- Streaming Pipeline — How tool events flow through the NDJSON protocol

---
