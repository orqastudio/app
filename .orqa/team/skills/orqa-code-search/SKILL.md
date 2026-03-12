---
id: SKILL-005
title: Orqa Code Search
description: |
  Context-aware code search wrapper. Detects whether you are in CLI (ChunkHound MCP)
  or App (native DuckDB+ONNX) context and provides the appropriate search patterns.
  Three tools: search_regex, search_semantic, code_research.
  Use when: Searching the codebase before creating new code, verifying endpoints exist,
  understanding how a system works, finding all callers before refactoring.
status: active
created: "2026-03-01"
updated: "2026-03-12"
layer: core
scope: [AGENT-001, AGENT-002, AGENT-003, AGENT-004, AGENT-005, AGENT-006, AGENT-007]
version: 1.1.0
user-invocable: true
---

Context-aware code search wrapper. Detects your runtime context and provides the right
tool names and patterns. **You do not need to load `chunkhound` or `orqa-native-search`
directly — this skill handles context resolution.**

## Context Detection

Check which tools are available to determine your context:

| Available Tools | Context | Tool Names to Use |
|----------------|---------|-------------------|
| `mcp__chunkhound__search_regex`, `mcp__chunkhound__search_semantic`, `mcp__chunkhound__code_research` | **CLI** (Claude Code terminal) | Use the `mcp__chunkhound__*` prefixed names |
| `search_regex`, `search_semantic`, `code_research` as Tauri commands or tool-executor tools | **App** (OrqaStudio) | Use the unprefixed names |
| Neither set available | **Fallback** | Use Grep/Glob, note in task summary that semantic search was unavailable |

## Implementation Details by Context

**CLI context:** Search is provided by ChunkHound, an external MCP server. It uses its own
DuckDB index and an OpenAI-compatible embedding server at localhost:11435. See the
`chunkhound` skill for implementation-specific details (health checks, server prerequisites).

**App context:** Search is provided by the app's native Rust engine using embedded DuckDB and
ONNX Runtime — no external server needed. See the `orqa-native-search` skill for
implementation-specific details (indexing, model loading).

## Tool Selection Guide

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

## Query Patterns

### search_regex — Exact Pattern Matching

Use literal names, paths, or regex patterns:

```text
create_session                  → all usages of this function
#[tauri::command]               → all IPC command handlers
SessionId                       → newtype usage across codebase
$state()                        → all reactive state declarations
```

### search_semantic — Meaning-Based Search

Use natural language describing the concept:

```text
"error handling in Tauri commands"
"reactive store pattern in Svelte"
"SQLite migration logic"
"streaming response from AI provider"
```

**Mandatory use case:** Before creating any new module, struct, or utility function, run
`search_semantic` to confirm no existing implementation already covers the need.

### code_research — Architectural Analysis

Use questions about how things work:

```text
"how does session creation flow from UI to database"
"what components display artifact data"
"how are AI responses streamed to the frontend"
"what happens when a user sends a message"
```

**Mandatory use case:** Required before implementing any feature that crosses the IPC
boundary or touches 3+ files.

## Anti-Patterns

- **NEVER** manually grep through 10+ files when `search_semantic` can find relevant code in one query
- **NEVER** start a multi-file implementation without running `code_research` first
- **NEVER** create a new utility function without checking for existing implementations via `search_semantic`
- **NEVER** use Grep/Glob for architectural research when code search is available

```text
WRONG: Read file A → Read file B → Read file C → ... (accumulates context, slow)
RIGHT: code_research "how does X work" → focused report → read only the specific files cited
```

## Related Skills

- `chunkhound` — CLI implementation details (MCP server, health checks, embeddings server)
- `orqa-native-search` — App implementation details (ONNX Runtime, DuckDB, DirectML)
- `architecture` — Using code_research to map architectural boundaries
- `planning` — Pre-implementation research protocol
