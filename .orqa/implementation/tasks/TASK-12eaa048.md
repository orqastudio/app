---
id: "TASK-12eaa048"
type: "task"
title: "Refactor skill sync to proactive-only — coding standards and agent preloads"
status: captured
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "EPIC-1358323e"
    type: "delivers"
  - target: "TASK-49b455ac"
    type: "depends-on"
---

# TASK-12eaa048: Skill Sync Refactor

## Acceptance Criteria

1. sync-skills.mjs only syncs skills that need to be proactively available:
   - Agent preload skills (referenced in agent `skills:` frontmatter)
   - Coding standards skills (composability, centralized-logging, best-practices, etc.)
   - Intent-mapped skills (referenced in prompt-injector INTENT_MAP)
2. All other skills available via MCP on demand (graph_query + graph_read)
3. Sync script has a clear list of proactive skills (not "sync everything")
4. Orchestrator prompt documents that non-proactive skills are fetched via MCP
