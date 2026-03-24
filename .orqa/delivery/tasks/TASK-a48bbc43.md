---
id: TASK-a48bbc43
type: task
title: "Consolidate frontmatter parsing to SDK single export"
description: "Reduce 7 copies of frontmatter parsing across the codebase to 2 canonical implementations: one Rust (orqa_validation) and one TypeScript (SDK). App UI and CLI should import from SDK; MCP server should re-export from orqa_validation."
status: captured
priority: P3
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - "libs/sdk/src/utils/frontmatter.ts is the single TypeScript frontmatter parser"
  - "app/ui/src/lib/utils/frontmatter.ts imports from SDK instead of maintaining its own copy"
  - "libs/cli/ has at most one parseFrontmatter() in a shared utility, not inline in each command"
  - "libs/mcp-server/src/graph.rs re-exports extract_frontmatter from orqa_validation (overlaps with TASK-28789c27)"
  - "All consumers produce identical parse results for the same input"
relationships:
  - target: EPIC-5ab0265a
    type: delivers
    rationale: "Task delivers work to the deduplication epic"
  - target: TASK-28789c27
    type: depends-on
    rationale: "MCP server frontmatter fix should be done first to establish the re-export pattern"
  - target: TASK-a0f9197e
    type: depended-on-by
---

## What

7 copies of frontmatter parsing exist across the codebase with subtle differences (trim vs no-trim, regex vs delimiter-based, different return types). See audit category 5 for full inventory.

## How

1. Verify `libs/sdk/src/utils/frontmatter.ts` is the most complete TypeScript implementation
2. Make `app/ui/src/lib/utils/frontmatter.ts` import from `@orqastudio/sdk` instead of copying
3. Create `libs/cli/src/lib/frontmatter.ts` as a shared utility for CLI commands
4. Migrate `libs/cli/src/commands/id.ts::parseFrontmatter()` and `libs/cli/src/commands/audit.ts::parseFrontmatter()` to use the shared utility
5. The connector hook's inline parser (`connectors/claude-code/src/hooks/artifact-enforcement.ts`) is acceptable as a thin adapter with minimal logic

## Files

- `libs/sdk/src/utils/frontmatter.ts` — canonical TypeScript implementation
- `app/ui/src/lib/utils/frontmatter.ts` — copy to remove
- `libs/cli/src/commands/id.ts` — inline parser to extract
- `libs/cli/src/commands/audit.ts` — inline parser to extract