---
id: TASK-212
title: Extract dev server and worktree content to skills or docs
description: Move dev server lifecycle, worktree workflow, and hooks configuration from the orchestrator prompt to standalone skills or documentation that agents read from the graph.
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-053
depends-on: []
docs: []
skills:
  - SKILL-011
  - SKILL-037
  - SKILL-029
acceptance:
  - Dev server lifecycle content exists as a skill or doc
  - Worktree workflow content exists as a skill or doc
  - Hooks configuration content exists as a skill or doc
  - Orchestrator prompt no longer contains this operational detail
  - Content is still discoverable via the graph
---
## What

The orchestrator prompt contains ~200 lines of dev server lifecycle, worktree workflow, and hooks configuration. This is project-specific operational knowledge that belongs in skills or documentation, not in the universal orchestrator prompt.

## Content to Extract

1. **Dev Server Lifecycle** (~50 lines): `make dev`, `make restart-tauri`, `make kill`, agent restart behaviour → extract to a `dev-workflow` skill or update existing `tauri-v2` skill
2. **Worktree Lifecycle** (~60 lines): worktree create/merge/cleanup, git safety → extract to a `git-workflow` skill or documentation page
3. **Hooks Configuration** (~20 lines): session start hook, pre-commit reminder → extract to plugin documentation

## How

1. Read the current orchestrator.md to identify dev/worktree/hooks content
2. Create or update skills with extracted content
3. Set skill scope fields so they're auto-discovered via graph
4. Remove the content from the orchestrator prompt (done in TASK-211)

## Verification

- Dev server lifecycle content exists as a standalone skill or doc
- Worktree workflow content exists as a standalone skill or doc
- Content is findable via skill scope fields or task docs/skills edges
