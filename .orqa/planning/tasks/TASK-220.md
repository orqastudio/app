---
id: TASK-220
title: Verify orchestrator prompt works at ~200 lines (dogfood session)
description: Run a full dogfood session with the simplified graph-based orchestrator prompt and verify agents can navigate the graph to find the context they need.
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-053
depends-on:
  - TASK-211
  - TASK-214
docs: []
skills:
  - orqa-governance
scope:
  - Start a session with the new ~200-line orchestrator prompt
  - Attempt a typical workflow (research → plan → implement → review)
  - Verify agents read graph nodes on demand
  - Verify agents extend the graph when creating artifacts
  - Document any missing context that needs to be added back
acceptance:
  - A full workflow completes successfully with the simplified prompt
  - Agents find rules, skills, and docs via graph navigation
  - No critical context is missing
  - Any gaps identified are documented for iteration
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
