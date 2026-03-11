---
id: SKILL-013
title: Orqa Native Search
description: |
  How to use Orqa's embedded search tools in the app context: tool names,
  query patterns, tool selection guide, and fallback behavior.
  Use when: Any agent needs to search code or artifacts from within the OrqaStudio app.
status: active
created: "2026-03-01"
updated: "2026-03-11"
layer: core
scope: [AGENT-001, AGENT-002, AGENT-003, AGENT-004, AGENT-005, AGENT-006, AGENT-007]
version: 2.0.0
user-invocable: true
---

Orqa provides three search tools accessible from within the app. This skill covers
how to USE them — for implementation details of the engine itself, see `orqa-search-architecture`.

> **When to load this skill vs `chunkhound`:**
> - **App context** (OrqaStudio UI) → load `orqa-native-search`
> - **CLI context** (Claude Code terminal) → load `chunkhound`
> Both provide `search_regex`, `search_semantic`, and `code_research` — same tool names,
> independent implementations.

## Three Search Tools

### `search_regex` — Exact Pattern Matching

Searches indexed chunks matching a regular expression. Works even if the embedding
model hasn't loaded yet.

**Tool name (in app):** `search_regex`
**Tool name (in CLI):** `mcp__chunkhound__search_regex`

### `search_semantic` — Meaning-Based Search

Finds code chunks most similar in meaning to a natural language query.

**Requires:** Embedding model loaded (check `get_index_status`)
**Tool name (in app):** `search_semantic`
**Tool name (in CLI):** `mcp__chunkhound__search_semantic`

### `code_research` — Architectural Analysis

Combines semantic search results with LLM analysis to produce a structured report
about how a system works.

**Tool name (in app):** `code_research`
**Tool name (in CLI):** `mcp__chunkhound__code_research`

## Query Patterns

| Situation | Tool | Example |
|-----------|------|---------|
| Know the exact name | `search_regex` | `create_session` |
| Know the concept | `search_semantic` | `"error handling in Tauri commands"` |
| Need end-to-end understanding | `code_research` | `"how does streaming work"` |

## When Native Search Is Unavailable

If the embedding model hasn't loaded yet (first startup, download in progress):

- `search_regex` always works (no embeddings needed)
- `search_semantic` and `code_research` will fail gracefully
- Check `get_index_status` for indexing state

## Indexing Commands

| Command | Purpose |
|---------|---------|
| `index_codebase` | Trigger full re-index |
| `get_index_status` | Check indexing state (idle/indexing/complete) |

## Related

- `chunkhound` skill — CLI-context equivalent (external MCP server)
- `orqa-search-architecture` skill — implementation details of the native engine
- [AD-024](AD-024) — Architecture decision documenting the native search engine
