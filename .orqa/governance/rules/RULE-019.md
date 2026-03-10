---
id: RULE-019
title: No Deferred Deliverables
description: If a deliverable is in scope, it ships with the epic. Deferring scoped deliverables is forbidden.
status: active
created: "2026-03-07"
updated: "2026-03-07"
layer: canon
scope: system
---
## The Rule

**If a deliverable is listed in an epic's scope (roadmap items, task list, or plan), it ships with that epic. No exceptions.**

Deferring a scoped deliverable to a future epic is forbidden. It creates invisible gaps — the epic is marked "done" but the user's expectation (based on the roadmap) is unmet. This is a form of dishonest reporting.

## What Counts as a Deliverable

A deliverable is any item that appears in:

- The epic's roadmap entry (e.g., D1 items in `.orqa/documentation/product/roadmap.md`)
- The epic's task list in `.orqa/epics/EPIC-NNN.md`
- The epic's plan in `.orqa/planning/plans/`
- The epic's `docs-produced` list

If it's listed, it's in scope. If it's in scope, it ships.

## Before Planning Begins

During planning, the orchestrator MUST reconcile the epic's scope with the roadmap entry:

1. Read the roadmap entry for the epic (e.g., D1 items under Milestone 1)
2. Ensure every roadmap item appears as a task in the plan
3. If an item genuinely cannot be completed without another epic's work being done first, **flag it to the user** before planning is approved — not after implementation starts
4. The user decides whether to: (a) include the prerequisite work in this epic, (b) explicitly remove the item from this epic's roadmap scope, or (c) restructure the epics

## During Implementation

- If an agent discovers a deliverable is harder than expected, the orchestrator reports this to the user and asks for direction — NOT silently defers it
- "Will be wired up in EPIC-NNN" is FORBIDDEN in completion reports
- "Deferred to EPIC-NNN" is FORBIDDEN unless the user explicitly approved the deferral during planning

## The Completion Check

Before marking an epic as done, the orchestrator MUST:

1. Re-read the epic's roadmap entry
2. Verify every listed item has been implemented (not just planned, not just typed — implemented and verified)
3. If any item is missing, the epic is NOT done

## Cross-Epic Dependencies

Sometimes a deliverable genuinely depends on infrastructure from another epic. This is handled at planning time, not implementation time:

| Situation | Resolution |
|-----------|------------|
| Item depends on another epic's infrastructure | Include the minimum infrastructure in THIS epic, or get user approval to remove the item from scope |
| Item is trivially blocked | Build the prerequisite as part of this epic |
| Item requires a large body of work from another domain | Flag to user: "This item requires X, which is EPIC-NNN scope. Should I include it here or remove it from this epic's deliverables?" |

**The key principle:** The user should never be surprised that a deliverable was skipped. Every scope reduction is a user decision, not an agent decision.

## FORBIDDEN Patterns

```text
## Out of Scope (handled by other epics)
- `ContextInjected` emission — depends on EPIC-003
```
^ This is the orchestrator unilaterally removing a deliverable without user approval.

```text
## What Is NOT Done
- custom_prompt is always None — EPIC-002 will populate this
```
^ This is deferring a deliverable. If custom_prompt is in scope, implement it. If it's genuinely out of scope, the user must have approved its removal.

## Related Rules

- [RULE-015](RULE-015) (honest-reporting) — deferring deliverables is a form of false "complete"
- [RULE-022](RULE-022) (plan-mode-compliance) — plans must reconcile with roadmap scope
- [RULE-020](RULE-020) (no-stubs) — scaffolded implementations are not deliverables
- [RULE-004](RULE-004) (artifact-lifecycle) — epic status transitions require all deliverables complete
