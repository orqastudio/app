---
id: TASK-467
title: "Add grounding injection to plugin — resolve grounded-by on agents, inject content"
description: Extend the Claude Code plugin to resolve grounded-by relationships on agent definitions and inject the target document content as system-level context at session initialization.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Plugin reads agent definition at session start
  - Plugin resolves all grounded-by relationships to file paths
  - Plugin reads target files and injects body content as systemMessage
  - Grounding content injected BEFORE any task-specific skill loading
  - Deduplication prevents re-injection if grounding was already loaded
  - "Missing grounding files produce a warning, not a crash"
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 3 — mechanical implementation of grounding injection in CLI context
  - target: TASK-412
    type: informed-by
    rationale: Skill injection implementation provides the pattern for grounding injection
  - target: TASK-466
    type: depends-on
  - target: TASK-412
    type: depends-on
  - target: SKILL-020
    type: grounded-by
  - target: SKILL-011
    type: grounded-by
---

## Scope

### Plugin Changes

In the SessionStart hook or prompt-injector.mjs:

1. Read the active agent definition (orchestrator.md for the main session)
2. Parse frontmatter relationships
3. Filter for `type: grounded-by` relationships
4. For each grounded-by target:
   a. Resolve artifact ID to file path (e.g., PILLAR-001 → .orqa/process/pillars/PILLAR-001.md)
   b. Read file content
   c. Strip YAML frontmatter
   d. Collect body content
5. Inject collected content as systemMessage with header: "## Agent Grounding (always in context)"
6. Track injected grounding in session dedup file to prevent re-injection

### Ordering

Grounding is injected FIRST, before:
- Skill injection (TASK-412)
- Artifact graph traversal (prompt-injector.mjs)
- Rule enforcement (rule-engine.mjs)

This ensures the agent's purpose context is loaded before any task-specific knowledge.
