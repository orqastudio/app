---
id: KNOW-21d28aa0
type: knowledge
title: Planning
summary: "Every implementation task follows a strict documentation-first workflow: **Document → Approve → Implement → Verify**. No code is written before documentation is approved."
description: |
  Enforces documentation-first planning for all implementation tasks.
  Plans must start with documentation, get user approval, then implement with
  mandatory verification gates. Prevents documentation drift across sessions.
status: active
created: 2026-03-01
updated: 2026-03-10
category: methodology
user-invocable: false
allowed-tools: "Read, Glob, Grep"
relationships:
  - target: DOC-bad8e26f
    type: synchronised-with

---

Every implementation task follows: **Discuss → Agree → Plan → Approve → Implement → Verify**. Documentation is the source of truth — code that diverges from docs is wrong.

## Collaborative Design Workflow

1. **Discuss** — Explore architecture, data model, UX trade-offs conversationally
2. **Agree** — Reach alignment on approach and key decisions
3. **Plan** — Write formal plan with architectural compliance and systems checklist
4. **Approve** — User approves before any code is written
5. **Implement** — Execute phase by phase with verification gates
6. **Verify** — Audit implementation against documentation

## Pre-Implementation Checklist

Read before ANY code changes: `documentation/reference/` (feature designs), `process/decisions/` (architecture decisions), `delivery/tasks/` (task context), project roadmap. Use search tools first to narrow scope.

## Plan Structure Requirements

### Architectural Compliance

Show HOW each principle is satisfied — not just a list. Verify: architecture decisions, error propagation, layer separation, type safety, end-to-end completeness, coding standards.

### Systems Architecture Checklist

Address each dimension or mark "N/A — [reason]":

| Dimension | What to Address |
| ----------- | ---------------- |
| Data Persistence | Schema, storage, migration |
| IPC Contract | Commands, types, serialization |
| State Management | Where stored, refresh behavior |
| Error Handling | Failure modes, user-facing errors |
| Testing Strategy | Unit, integration, E2E |
| Documentation | Which docs need updating |

### UX-First Design (user-facing changes)

User journeys, component state tables (component x state → what user sees), backend requirements derived from UX.

### Phase Ordering (NON-NEGOTIABLE)

```text
Phase 1: Documentation update (define target state first)
Phase 2: Backend changes (governed by Phase 1 docs)
Phase 3: Frontend changes (governed by Phase 1 docs)
Phase 4: Documentation verification
```text

## Drift Prevention Rules

1. Re-read governing docs at start of every phase
2. Documentation is ALWAYS right — divergent code is wrong
3. No silent improvements — update doc FIRST, then code
4. Documentation compliance is a gate after every phase
5. NEVER proceed with failing checks or undocumented drift
