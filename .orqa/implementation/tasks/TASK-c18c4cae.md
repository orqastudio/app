---
id: "TASK-c18c4cae"
type: "task"
title: "Fix MCP server frontmatter missing trim"
description: "The MCP server's extract_frontmatter() in libs/mcp-server/src/graph.rs does not trim the frontmatter text before parsing, unlike the canonical version in libs/validation/src/graph.rs. Fix by re-exporting from orqa_validation or adding the missing .trim() call."
status: archived
priority: "P1"
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
acceptance:
  - "libs/mcp-server/src/graph.rs extract_frontmatter() produces identical output to libs/validation/src/graph.rs for all frontmatter content including edge cases with leading/trailing whitespace"
  - "Ideally re-exports from orqa_validation directly instead of maintaining a separate copy"
  - "cargo check passes with zero warnings on libs/mcp-server"
  - "Existing MCP server tests pass"
relationships:
  - target: "EPIC-0497a1be"
    type: "delivers"
    rationale: "Task delivers work to the deduplication epic"
---

## What

The MCP server has its own copy of `extract_frontmatter()` in `libs/mcp-server/src/graph.rs` that does NOT trim the frontmatter text before returning it. The canonical version in `libs/validation/src/graph.rs` DOES trim. This can produce subtly different YAML parse results for frontmatter with leading/trailing whitespace.

## How

Preferred approach: make `libs/mcp-server` depend on `orqa_validation` and re-export `extract_frontmatter` from the canonical crate. Remove the local copy entirely.

Alternative: add `.trim()` to the MCP server's copy to match the canonical version. Less ideal as it preserves the duplication.

## Files

- `libs/mcp-server/src/graph.rs` — duplicate `extract_frontmatter()`
- `libs/validation/src/graph.rs` — canonical `extract_frontmatter()`
