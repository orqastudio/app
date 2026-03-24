---
id: TASK-328b7cb3
type: task
title: "Reconcile EPIC-3ecc76ff"
description: "Standing reconciliation task for EPIC-3ecc76ff. Ensures the epic body stays accurate as tasks are completed: task table, pillars, docs-produced, and scope."
status: ready
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: EPIC-3ecc76ff
    type: delivers
    rationale: "Reconciliation task — verifies epic body accuracy before closure"
---

# Reconcile EPIC-3ecc76ff

This is the standing reconciliation task for [EPIC-3ecc76ff](EPIC-3ecc76ff) (Schema-Driven LSP Enforcement for Artifact Intelligence).

## What to Verify Before Epic Closure

1. **Task table completeness** — every TASK with `delivers → EPIC-3ecc76ff` appears in the epic body's task table
2. **Pillar accuracy** — pillars listed in epic relationships reflect all pillars the work actually serves
3. **Docs-produced accuracy** — any documentation produced during the epic is listed
4. **Scope accuracy** — out-of-scope section reflects actual decisions made during implementation
5. **Current state table** — update the "Current State" table to reflect post-implementation status

## Acceptance Criteria

- [ ] Epic task table lists ALL tasks created during the epic
- [ ] Epic pillars array reflects all pillars served
- [ ] Epic scope section accurately reflects what was in/out of scope
- [ ] No tasks exist for this epic that are missing from the epic body
- [ ] Epic "Current State" table is updated to reflect completed work
