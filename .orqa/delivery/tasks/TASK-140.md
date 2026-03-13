---
id: TASK-140
title: Reclassify tech skills as Tier 2 in orchestrator
description: Move rust-async-patterns, svelte5-best-practices, tailwind-design-system, typescript-advanced-types, and tauri-v2 from the Tier 1 table to the Tier 2 injection table in the orchestrator's Skills section.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - Orchestrator Tier 1 table contains only universal skills
  - Five tech skills moved to Tier 2 injection table with appropriate scope triggers
  - Tier 1 definition updated to clarify purpose (universal principles, not domain tech)
  - No agent YAML skills lists contain tech stack skills
---

## What

The orchestrator (CLAUDE.md) lists `rust-async-patterns`, `svelte5-best-practices`, `tailwind-design-system`, `typescript-advanced-types`, and `tauri-v2` as Tier 1 ("declared in agent YAML, loaded automatically"). They are actually Tier 2 — injected by the orchestrator based on task scope, not auto-loaded by agents.

## How

1. Open the orchestrator's Skills section (CLAUDE.md / orchestrator.md)
2. Move the five tech skills from the Tier 1 table into the Tier 2 injection table with appropriate scope triggers (e.g., `rust-async-patterns` when touching `backend/src-tauri/`)
3. Update the Tier 1 definition text to make clear it covers only universal, always-loaded skills
4. Search agent YAML files in `.orqa/process/agents/` for any `skills:` list entries referencing the five tech skills and remove them

## Verification

- [ ] Orchestrator Tier 1 table contains only universal skills
- [ ] Five tech skills moved to Tier 2 injection table with appropriate scope triggers
- [ ] Tier 1 definition updated to clarify purpose (universal principles, not domain tech)
- [ ] No agent YAML skills lists contain tech stack skills
