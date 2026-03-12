---
id: TASK-201
title: Add injection map entries to relevant rules
description: |
  Add enforcement entries with action:inject to rules that govern specific
  code areas, mapping file path patterns to the skills that should be
  auto-loaded.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-199
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
- `sidecars/orqa-sidecar/src/**` → `orqa-streaming`
- `ui/src/lib/components/**` → `component-extraction`, `svelte5-best-practices`
- `ui/src/lib/stores/**` → `orqa-store-patterns`, `orqa-store-orchestration`
- `.orqa/**` → `orqa-governance`, `orqa-documentation`

## Verification

- Each mapping corresponds to an existing skill in `.orqa/team/skills/`
- Writing to a mapped path triggers skill injection
- Skills are returned as systemMessage content
