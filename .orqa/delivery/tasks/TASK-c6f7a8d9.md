---
id: TASK-c6f7a8d9
title: "Update documentation and commands reference for port allocation changes"
type: task
description: "Update all documentation to reflect the new port allocation, CLI process lifecycle commands, and dev controller demotion. Documentation-first: these updates inform the implementation."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - .orqa/documentation/development/commands.md updated with new CLI commands
  - RULE-c71f1c3f (development-commands) updated with new make-to-orqa mapping
  - Port allocation table documented in the core-framework plugin as the canonical reference
  - RULE-6083347d (dogfood-mode) updated to reference orqa dev instead of make dev
  - All documentation references to old ports (1420, 3002, 9258) are updated
  - Reviewer verifies no stale port references remain in documentation
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Documentation task for port allocation epic"
---

## What

Documentation-first: update all docs before implementation begins. This ensures the implementation targets are clear and all agents have correct reference material.

## Files to Update

- `.orqa/documentation/development/commands.md` — add `orqa daemon|search|mcp` commands
- `RULE-c71f1c3f` — update command mapping table
- `RULE-6083347d` — update dev server section to reference `orqa dev`
- `.orqa/documentation/development/coding-standards.md` — if port conventions are mentioned
- `README.md` — if it references port numbers or dev setup
- Any connector/plugin documentation referencing ports
- **Core-framework plugin** — canonical port allocation table must live here (single source of truth for all port assignments, referenced by all other docs)

## Verification

1. `search_regex` for ports 1420, 3002, 9258 in `.orqa/` returns zero results
2. Commands reference lists all new CLI commands
3. Dev workflow documentation describes `orqa dev` as primary entry point
