---
id: "SKILL-003"
title: "ChunkHound"
description: "ChunkHound semantic code search: three search modes (regex, semantic, code_research),

  tool selection guide, query patterns, and anti-patterns.

  Use when: Searching a codebase before creating new code, verifying endpoints exist,

  understanding how a system works, finding all callers before refactoring.\n"
status: "active"
created: "2026-03-01"
updated: "2026-03-10"
layer: "project"
scope:
  - "AGENT-001"
  - "AGENT-002"
  - "AGENT-003"
  - "AGENT-004"
  - "AGENT-005"
  - "AGENT-006"
  - "AGENT-007"
category: "tool"
version: "1.0.0"
user-invocable: true
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "ChunkHound enables structured code search and discovery"
---


ChunkHound provides three MCP tools for structured code search. Use them instead of manually grepping through files. The goal is to understand the codebase deeply before making changes, not to accumulate file reads in context.

## Quick Start

### Tool Selection (Use This First)

| Situation | Tool |
|-----------|------|
| Know the exact function or class name | `search_regex` |
| Know the exact route or command name | `search_regex` |
| Need all callers before refactoring | `search_regex` |
| Know the concept, not the file | `search_semantic` |
| About to create a new utility — check it doesn't exist | `search_semantic` |
| Need to understand how a system works end-to-end | `code_research` |
| Implementing a feature touching 3+ files | `code_research` (mandatory first) |
| Debugging a cross-layer issue | `code_research` |
| Embeddings server is not running | `search_regex` only |

## Key Concepts

| Concept | Description |
|---------|-------------|
| DuckDB index | Local database storing code chunks and embeddings — never sent to the cloud |
| ONNX embeddings server | Local HTTP server that converts text to embedding vectors |
| `search_regex` | Exact pattern matching — no embeddings server needed |
| `search_semantic` | Meaning-based search — requires embeddings server at `localhost:11435` |
| `code_research` | Architectural analysis via BFS traversal — requires embeddings server + LLM |

## Architecture Overview

```text
ONNX Embeddings Server           ChunkHound Index (DuckDB)
(localhost:11435)                (local file on disk)
       |                                |
       |  OpenAI-compatible API         |  Stores code chunks +
       |  embedding model               |  semantic vectors
       +--------------------------------+
                     |
                     | MCP tools
                     v
            Agent (mcp__chunkhound__*)
```

All search is local. No query is sent to a cloud API. The index is a DuckDB file on the developer's machine.

## Three MCP Tools

### `search_regex` — Exact Pattern Matching

Searches the index for lines matching a regular expression. Does not require the embeddings server. Fastest tool — use first whenever you know the exact name.

**Use for:**

- Looking up a specific function, struct, or variable by name
- Verifying a Tauri command exists in the backend
- Finding all callers of a function before refactoring it
- Checking IPC type consistency between Rust and TypeScript
- Confirming an import path before using it

**Example patterns:**

```text
create_session                  → all usages of this function
#[tauri::command]               → all IPC command handlers
SessionId                       → newtype usage across codebase
$state()                        → all reactive state declarations
```

### `search_semantic` — Meaning-Based Search

Converts the query to an embedding vector and finds the most semantically similar code chunks. Requires the embeddings server. Use when you know what you need but not what it is called.

**Use for:**

- Finding similar implementations before writing a new one
- Locating the canonical implementation of a concept
- Understanding how a pattern is used across the codebase
- Pre-creation check: "does anything like this exist already?"

**Example queries:**

```text
"error handling in Tauri commands"
"reactive store pattern in Svelte"
"SQLite migration logic"
"IPC type serialization"
"streaming response from Claude API"
```

**Mandatory use case:** Before creating any new module, struct, or utility function, run `search_semantic` to confirm no existing implementation already covers the need.

### `code_research` — Architectural Analysis

Uses BFS graph traversal across the codebase, guided by query expansion and synthesis. Reads deeply across multiple files and layer boundaries, producing a structured markdown report with file citations.

**Use for:**

- Understanding end-to-end request flows across Rust and Svelte layers
- Mapping dependencies before a major refactor
- Debugging cross-layer issues (Svelte → IPC → Rust → SQLite)
- Pre-implementation analysis of any feature touching 3+ files

**Example queries:**

```text
"how does session creation work from UI to database"
"what components display artifact data"
"how are Claude API responses streamed to the frontend"
"what happens when a user sends a message in the chat"
```

**Mandatory use case:** `code_research` is required before implementing any feature that crosses the IPC boundary or touches 3+ files.

## Embeddings Server Prerequisite

`search_semantic` and `code_research` require the local embeddings server running at `localhost:11435`. `search_regex` does NOT require it.

**Health check:**

```bash
curl http://localhost:11435/health
```

If the server is not running, use `search_regex` for known symbols and Grep/Glob as a fallback for everything else. Note in your work summary that semantic search was unavailable.

## Query Patterns

### For `search_regex`

Use literal names, paths, or patterns:

```text
# Function lookup
create_session

# Tauri command verification
#[tauri::command]

# Type usage
SessionMessage

# Svelte component
<ConversationMessage
```

### For `search_semantic`

Use natural language describing the concept:

```text
# Pattern/concept search
"error propagation from Rust to Svelte"
"SQLite repository pattern"
"Tauri event streaming"
"component loading state handling"

# Similarity search
"similar to how the session store manages state"
"like the artifact parsing in the backend"
```

### For `code_research`

Use questions about how things work:

```text
"how does X flow from frontend to database"
"what calls Y and what does Y call"
"trace the lifecycle of a session from creation to deletion"
"what layers are involved in displaying artifacts"
```

## Anti-Patterns

**NEVER manually grep through 10+ files when `search_semantic` can find relevant code in one query.**

**NEVER start a multi-file implementation without running `code_research` first.**

**NEVER create a new utility function without checking for existing implementations via `search_semantic`.**

**NEVER use Grep/Glob for architectural research when ChunkHound is available.**

```text
WRONG: Read file A → Read file B → Read file C → ... (accumulates context, slow)
RIGHT: code_research "how does X work" → focused report → read only the specific files cited
```

## When ChunkHound Is Unavailable

If the MCP tools are not available in the current session:

1. Use `search_regex` patterns as Grep queries (less efficient but same idea)
2. Use Grep/Glob for multi-file searches as a fallback
3. Note in the task summary that ChunkHound was unavailable — results may be less complete

## See Also

- See the project's ChunkHound setup documentation for project-specific configuration, Makefile targets, and index maintenance procedures

## Related Skills

- See the **architecture** skill for using code_research to map architectural boundaries
- See the **planning** skill for pre-implementation research protocol
