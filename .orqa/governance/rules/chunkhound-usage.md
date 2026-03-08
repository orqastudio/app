---
id: chunkhound-usage
title: "ChunkHound Usage"
description: "Prefer semantic search over Grep/Glob for multi-file searches. ChunkHound is mandatory for all agents."
scope: system
---


The `chunkhound` skill contains tool selection guides, query patterns, and anti-patterns. Load it. **Prefer semantic search over Grep/Glob for any search that spans more than one file or directory.**

## Dual-Context Tool Names

Semantic search is available in two contexts with different tool names. Both are permanent and first-class.

| Context | Tool Names | How They Work |
|---------|-----------|---------------|
| **CLI** (Claude Code) | `mcp__chunkhound__search_regex`, `mcp__chunkhound__search_semantic`, `mcp__chunkhound__code_research` | Via ChunkHound MCP server configured in `.mcp.json` |
| **App** (OrqaStudio) | `search_regex`, `search_semantic`, `code_research` | Native embedded search (ONNX embeddings + DuckDB in `src-tauri/src/search/`) |

The tools do the same thing — only the names differ. The `chunkhound` skill teaches query patterns that work in both contexts. Use whichever set is available in your current environment.

## Enforcement

- The orchestrator and ALL subagents MUST prefer semantic search over Grep/Glob for multi-file searches
- Grep/Glob are only appropriate for single-file lookups or when semantic search is confirmed unavailable
- Every agent's YAML frontmatter MUST include `chunkhound` in its `skills:` list
- Every agent's YAML frontmatter MUST list both CLI and app tool names

## Documentation Review (MANDATORY before implementation)

Before writing ANY implementation code, check the project documentation for existing designs, plans, and architecture decisions related to the task. Use `code_research` with a query describing the feature area — it searches docs AND code together.

## When Semantic Search is Unavailable

If neither tool name set is available in the current session:

1. **Subagents** — Delegate research to a subagent that has search access
2. **Direct fallback** — Only if subagent delegation is impractical, use Grep/Glob
3. **Always note** — State in the task summary that semantic search was unavailable so results may be incomplete

## Related Rules

- `skill-enforcement.md` — `chunkhound` is a universal skill required for every agent
- `error-ownership.md` — use `search_regex` to find function signatures before calling them
- `reusable-components.md` — use `search_semantic` to find similar components
- `end-to-end-completeness.md` — use `code_research` to map the full request chain
- `no-stubs.md` — use `search_regex` to verify implementations exist
