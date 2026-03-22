---
id: TASK-6fa0243a
title: Verify orchestrator prompt works at ~200 lines (dogfood session)
description: Run a full dogfood session with the simplified graph-based orchestrator prompt and verify agents can navigate the graph to find the context they need.
status: completed
created: 2026-03-12
updated: 2026-03-12
docs: []
acceptance:
  - A full workflow completes successfully with the simplified prompt
  - "Agents find rules, skills, and docs via graph navigation"
  - No critical context is missing
  - Any gaps identified are documented for iteration
relationships:
  - target: EPIC-dc1e3e4b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-0c6ac8d8
    type: depends-on
  - target: TASK-b72ead56
    type: depends-on
  - target: TASK-dd9c8538
    type: depended-on-by
---
## What

The ultimate test of the graph-based orchestrator: run a real session with the ~200-line prompt and verify everything works.

## How

1. Symlink CLAUDE.md to the new orchestrator prompt
2. Start a fresh session
3. Attempt a typical workflow (research → plan → implement → review)
4. Document any missing context, broken graph traversals, or confused agents

## Verification

- A full workflow completes without the agent being unable to find context
- Agents discover rules, skills, and docs via graph navigation
- Any gaps are documented as issues for iteration
