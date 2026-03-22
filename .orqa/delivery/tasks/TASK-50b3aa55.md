---
id: TASK-50b3aa55
title: Add injection map entries to relevant rules
description: |
  Add enforcement entries with action:inject to rules that govern specific
  code areas, mapping file path patterns to the skills that should be
  auto-loaded.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - "Enforcement entries with action:inject added to relevant rules"
  - Each path-to-skill mapping references an existing skill
  - Writing to a mapped path triggers skill injection
relationships:
  - target: EPIC-4440cdd4
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-6cd46196
    type: depends-on
  - target: TASK-34007190
    type: depended-on-by
  - target: TASK-d6727d2f
    type: depended-on-by
---

## What

Add `inject` enforcement entries to existing rules, implementing the Tier 2
skill injection table from the orchestrator's delegation protocol as mechanical
enforcement.

## How

Add enforcement entries to appropriate rules mapping path patterns to skills:

- `backend/src-tauri/src/domain/**` → `orqa-domain-services`, `orqa-error-composition`
- `backend/src-tauri/src/commands/**` → `orqa-ipc-patterns`, `orqa-error-composition`
- `backend/src-tauri/src/repo/**` → `orqa-repository-pattern`
- `sidecars/claude-agentsdk-sidecar/src/**` → `orqa-streaming`
- `ui/src/lib/components/**` → `component-extraction`, `svelte5-best-practices`
- `ui/src/lib/stores/**` → `orqa-store-patterns`, `orqa-store-orchestration`
- `.orqa/**` → `orqa-governance`, `orqa-documentation`

## Verification

- Each mapping corresponds to an existing skill in `.orqa/process/skills/`
- Writing to a mapped path triggers skill injection
- Skills are returned as systemMessage content
