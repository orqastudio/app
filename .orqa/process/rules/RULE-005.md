---
id: RULE-5e03e67b
title: Code Search Usage
description: Prefer semantic search over Grep/Glob for multi-file searches. Load the correct search knowledge for your context.
status: active
created: 2026-03-07
updated: 2026-03-07
relationships:
  - target: AD-99c2a969
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-99c2a969
  - target: AD-0f6286cd
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-0f6286cd
---
**Prefer semantic search over Grep/Glob for any search that spans more than one file or directory.**

## Two Search Knowledge Artifacts — Context-Dependent Loading

OrqaStudio has two independent search implementations that provide the same three tools. Load the correct knowledge artifact for your context.

| Context | Knowledge to Load | Implementation | Tool Names |
|---------|--------------|----------------|------------|
| **CLI** (Claude Code terminal) | `chunkhound` | External MCP server, localhost:11435, OpenAI-compatible embedding API | `mcp__chunkhound__search_regex`, `mcp__chunkhound__search_semantic`, `mcp__chunkhound__code_research` |
| **App** (OrqaStudio UI) | `orqa-native-search` | Embedded Rust engine, ONNX Runtime + DuckDB, no HTTP server | `search_regex`, `search_semantic`, `code_research` |

**These are completely independent implementations.** ChunkHound is an external Python tool accessed via MCP protocol. The native search engine is Rust code in `backend/src-tauri/src/search/` using the `ort` crate for ONNX and DuckDB for storage. They share the same tool names and query patterns but have no code in common.

## How to Determine Your Context

| Signal | Context |
|--------|---------|
| `mcp__chunkhound__*` tools are available | CLI — load `chunkhound` |
| `search_regex` / `search_semantic` are Tauri commands | App — load `orqa-native-search` |
| Neither is available | Fallback to Grep/Glob (note in task summary) |

## Enforcement

- The orchestrator and ALL subagents MUST prefer semantic search over Grep/Glob for multi-file searches
- Grep/Glob are only appropriate for single-file lookups or when semantic search is confirmed unavailable
- Every agent's YAML frontmatter MUST include BOTH `chunkhound` and `orqa-native-search` in its `knowledge:` list
- At runtime, agents load whichever knowledge artifact matches their current context

## Shared Query Patterns

Both knowledge artifacts use identical query patterns — the interfaces are the same:

| Situation | Tool | Example |
|-----------|------|---------|
| Know the exact function/class name | `search_regex` | `create_session` |
| Know the concept, not the file | `search_semantic` | `"error handling in Tauri commands"` |
| Need end-to-end understanding | `code_research` | `"how does streaming work"` |

## Documentation Review (MANDATORY before implementation)

Before writing ANY implementation code, check the project documentation for existing designs, plans, and architecture decisions related to the task. Use `code_research` with a query describing the feature area — it searches docs AND code together.

## When Semantic Search is Unavailable

If neither tool set is available in the current session:

1. **Subagents** — Delegate research to a subagent that has search access
2. **Direct fallback** — Only if subagent delegation is impractical, use Grep/Glob
3. **Always note** — State in the task summary that semantic search was unavailable so results may be incomplete

## Related Rules

- [RULE-deab6ea7](RULE-deab6ea7) (knowledge-enforcement) — search knowledge is universal, required for every agent
- [RULE-57ccb4a3](RULE-57ccb4a3) (error-ownership) — use `search_regex` to find function signatures before calling them
- [RULE-cb65b5d0](RULE-cb65b5d0) (reusable-components) — use `search_semantic` to find similar components
- [RULE-1acb1602](RULE-1acb1602) (end-to-end-completeness) — use `code_research` to map the full request chain
- [RULE-e9c54567](RULE-e9c54567) (no-stubs) — use `search_regex` to verify implementations exist
