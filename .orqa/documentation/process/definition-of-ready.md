---
id: DOC-028
title: Definition of Ready
description: Prerequisites that must be met before a task can be started by an implementing agent.
created: "2026-03-02"
updated: "2026-03-08"
---

**Date:** 2026-03-02

The Definition of Ready (DoR) is the gate checklist the orchestrator runs BEFORE delegating any task to an implementing agent. A task that does not meet the DoR is blocked -- the orchestrator documents the gap and asks the user before proceeding.

---

## Why a Definition of Ready?

Starting implementation before the prerequisites are in place is the #1 cause of wasted work and rework. A task with no documentation creates drift from day one. A task with no approved plan results in agents making architectural decisions they are not qualified to make.

The DoR exists to catch these gaps before the first line of code is written.

---

## The Checklist

Run this checklist for every task before delegating it. Check each item that applies to the task type.

### Artifact Trail

- [ ] An artifact exists for this work -- either an `EPIC-NNN` in `.orqa/planning/epics/` or a task within one
- [ ] Epic `docs-required` gate is satisfied -- all listed documents exist and are current
- [ ] Epic status is `ready` or later -- not still `draft`
- [ ] If promoted from an idea -- the source `IDEA-NNN` has `status: shaped` or `promoted`, and `research-needed` items have been addressed

### Documentation

- [ ] Documentation exists for the feature area (`.orqa/documentation/ui/`, `.orqa/documentation/architecture/`) -- **or** has been created and approved in the current session
- [ ] Governing architecture decisions identified and read -- relevant decisions are known
- [ ] IPC contract documented -- applies if the task adds or changes Tauri commands

### Planning

- [ ] Implementation plan approved -- applies if the task requires a plan per `plan-mode-compliance.md` (crosses 3+ files or the IPC boundary)
- [ ] Epic body contains an implementation design section — the approach, phases, and acceptance criteria are documented
- [ ] UX design documented with component state table -- applies if the task is user-facing
- [ ] **Design review passed** (Medium/High risk tasks only): `systems-architect` reviewed and approved the implementation plan for architectural compliance before implementation begins

### Agent Readiness

- [ ] Required reading completed -- the implementing agent's Required Reading section lists the docs to read before starting
- [ ] ChunkHound skill loaded -- mandatory for all agents
- [ ] Domain-specific skills loaded -- relevant skills identified and loaded

### Dependencies and State

- [ ] `.orqa/governance/lessons/` checked for known patterns in this area -- prevents repeating documented mistakes
- [ ] No blocking dependencies -- all tasks listed in `depends-on` in `.orqa/planning/tasks/` have `status: done`
- [ ] Task artifact checked -- the task is not in a blocked state awaiting external resolution
- [ ] Working tree clean -- `git status --short` shows no untracked or modified files
- [ ] No stale stashes -- `git stash list` is empty (or existing stashes are investigated and resolved)

---

## How to Use This Checklist

The orchestrator works through this checklist before using the Task tool to delegate any task.

**If all applicable items are satisfied:** Proceed with task delegation.

**If one or more items cannot be satisfied:**

1. Document the specific gap in the task artifact in `.orqa/planning/tasks/` with the tag `DOC_READY_BLOCK` or `PLAN_READY_BLOCK`
2. Ask the user how to proceed -- provide the specific gap and the options available
3. Do NOT proceed with implementation until the gap is resolved

> [!IMPORTANT]
> The orchestrator MUST NOT delegate implementation to an agent until all applicable DoR items are satisfied. Proceeding without DoR is a process violation that typically results in rework.

---

## Relationship to Plan-Mode Compliance

For tasks that require an implementation plan, the plan itself must satisfy the requirements in [RULE-022](RULE-022):

1. **UX-first design** -- user journeys -> UI design -> component state table -> backend
2. **Architectural compliance section** -- verifies all relevant architecture decisions
3. **Systems architecture checklist** -- data persistence, IPC contract, state management, error handling, testing strategy

A plan that omits any of these is not approved, and the task does not meet DoR.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The DoR makes pre-implementation requirements explicit and auditable — ensuring every task begins with documented context rather than relying on agent judgment about readiness. |
| Learning Through Reflection | The lessons check item ensures that documented implementation patterns are consulted before starting, feeding prior learning directly into new work. |

---

## Related Documents

- Artifact Workflow -- How artifacts flow through the development process
- Artifact Framework -- Artifact schemas and design principles
- Definition of Done -- The gate checklist before a task is marked complete
- Orchestration -- Full orchestrator responsibilities and delegation guide
- Workflow -- Task lifecycle protocol including the before-starting checklist
- [Implementation Lessons](lessons) -- Known patterns to check before starting
