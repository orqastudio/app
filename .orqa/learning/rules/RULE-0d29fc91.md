---
id: "RULE-0d29fc91"
type: rule
title: "Code Search Usage"
description: "Prefer semantic search over Grep/Glob for multi-file searches. Load the correct search knowledge for your context."
status: active
enforcement_type: advisory
created: "2026-03-07"
updated: "2026-03-07"
enforcement:

  - engine: behavioral

    message: "Prefer semantic search over Grep/Glob for any search spanning more than one file or directory; load the correct search knowledge for your context"
relationships:

  - target: "PD-7d3d7521"

    type: "enforces"
    rationale: "Auto-generated inverse of enforces relationship from PD-7d3d7521"

  - target: "PD-306eccf1"

    type: "enforces"
    rationale: "Auto-generated inverse of enforces relationship from PD-306eccf1"
---
**Prefer semantic search over Grep/Glob for any search that spans more than one file or directory.**

## Search Implementation

OrqaStudio has one search implementation with two access paths. The same three tools work in both contexts.

| Context | Access Path | Implementation | Tool Names |
| --- | --- | --- | --- |
| **CLI** (Claude Code terminal) | `orqa mcp` — orqastudio MCP server | Native ONNX Runtime + DuckDB engine served over MCP | `search_regex`, `search_semantic`, `search_research` |
| **App** (OrqaStudio UI) | Embedded Tauri commands | Same native engine, no MCP hop | `search_regex`, `search_semantic`, `search_research` |

The underlying search engine is Rust code in `backend/src-tauri/src/search/` using the `ort` crate for ONNX and DuckDB for storage. Both access paths call the same engine — the tools have the same names and query patterns in both contexts.

## How to Determine Your Context

| Signal | Context |
| --- | --- |
| `Read`, `Edit`, `Bash` tools available (PascalCase built-ins) | Claude Code CLI — use search tools via orqastudio MCP server |
| `read`, `edit`, `bash` tools available (lowercase Tauri commands) | OrqaStudio App — use native embedded search |
| Neither search path available | Fallback to Grep/Glob (note in task summary) |

## Enforcement

- The orchestrator and ALL subagents MUST prefer semantic search over Grep/Glob for multi-file searches
- Grep/Glob are only appropriate for single-file lookups or when semantic search is confirmed unavailable
- Load the `search` skill before any research task — it provides query patterns for all three tools

## Shared Query Patterns

The same query patterns work in both contexts:

| Situation | Tool | Example |
| --- | --- | --- |
| Know the exact function/class name | `search_regex` | `create_session` |
| Know the concept, not the file | `search_semantic` | `"error handling in Tauri commands"` |
| Need end-to-end understanding | `search_research` | `"how does streaming work"` |

## Documentation Review (MANDATORY before implementation)

Before writing ANY implementation code, check the project documentation for existing designs, plans, and architecture decisions related to the task. Use `search_research` with a query describing the feature area — it searches docs AND code together.

## When Semantic Search is Unavailable

If search tools are not available in the current session:

1. **Subagents** — Delegate research to a subagent that has search access
2. **Direct fallback** — Only if subagent delegation is impractical, use Grep/Glob
3. **Always note** — State in the task summary that semantic search was unavailable so results may be incomplete

## Related Rules

- [RULE-dd5b69e6](RULE-dd5b69e6) (knowledge-enforcement) — search knowledge is universal, required for every agent
- [RULE-0be7765e](RULE-0be7765e) (error-ownership) — use `search_regex` to find function signatures before calling them
- [RULE-eb269afb](RULE-eb269afb) (reusable-components) — use `search_semantic` to find similar components
- end-to-end-completeness — use `search_research` to map the full request chain
- [RULE-af5771e3](RULE-af5771e3) (no-stubs) — use `search_regex` to verify implementations exist
