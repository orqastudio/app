---
id: RULE-019
title: No Deferred Deliverables
description: If a deliverable is in scope, it ships with the epic. Deferring scoped deliverables is forbidden.
status: active
created: 2026-03-07
updated: 2026-03-07
layer: core
enforcement:
  - event: file
    action: warn
    paths:
      - .orqa/**
    pattern: (?i)(deferred to|will be (wired|handled|done|implemented) (in|by) EPIC|out of scope.*handled by|future epic|separate epic will)
    message: "Deferral language detected. RULE-019: if a deliverable is in scope, it ships NOW. Never defer to a future epic without explicit user approval."
  - event: file
    action: warn
    paths:
      - .orqa/delivery/tasks/**
      - .orqa/delivery/epics/**
    pattern: status:\s*done
    message: "Task/epic being marked done. RULE-019: verify ALL acceptance criteria are met and no deliverables were silently deferred."
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: No deferred deliverables ensures scope clarity and completion integrity
  - target: RULE-015
    type: informs
    rationale: Deferring scoped deliverables is a form of dishonest reporting
  - target: RULE-022
    type: informs
    rationale: Plans must reconcile with roadmap scope before implementation begins
  - target: RULE-020
    type: informs
    rationale: Scaffolded implementations that defer wiring are not deliverables — they are stubs
  - target: RULE-004
    type: informs
    rationale: Epic status transitions require all deliverables to be complete before marking done
  - type: informed-by
    target: RULE-015
    rationale: Honest reporting requires scope reductions to be user decisions, not agent decisions
  - type: informed-by
    target: RULE-027
    rationale: Structure before work defines acceptance criteria that prevent silent scope reduction
  - target: IMPL-016
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-016
  - target: IMPL-049
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-049
  - type: scoped-to
    target: AGENT-002
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
---
## The Rule

**If a deliverable is listed in an epic's scope (roadmap items, task list, or plan), it ships with that epic. No exceptions.**

Deferring a scoped deliverable to a future epic is forbidden. It creates invisible gaps — the epic is marked "done" but the user's expectation (based on the roadmap) is unmet. This is a form of dishonest reporting.

## What Counts as a Deliverable

A deliverable is any item that appears in:

- The epic's roadmap entry (e.g., D1 items in `.orqa/documentation/product/roadmap.md`)
- The epic's task list in `.orqa/delivery/epics/EPIC-NNN.md`
- The epic's implementation design (in the epic body)
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

## Scope Decisions Are Human Decisions (NON-NEGOTIABLE)

**AI agents MUST NEVER decide what is or is not in scope.** Agents can and should advise on scope — flagging items that seem too large, identifying dependencies, recommending scope changes — but the decision to include, exclude, or defer a deliverable is always made by the user.

This applies at every stage:

- **Planning**: Agents propose scope; the user approves it
- **Implementation**: If an agent discovers something should be descoped, it stops and asks — it does not unilaterally remove it
- **Completion**: Agents surface all outstanding items for user guidance — they do not classify items as "not blocking" or "minor" and move on

### No Assumptions — Ask, Don't Guess

When an agent encounters ambiguity about scope, priority, or approach:

- **If the ambiguity does not block current work**: continue with other tasks, collect the questions, and ask them all at the next natural checkpoint or at completion
- **If the ambiguity blocks the current task**: stop immediately and ask before proceeding
- **Never guess and continue** — a wrong assumption creates more work than a brief pause to clarify

### The "Not Blocking" Anti-Pattern

Framing outstanding work as "none of these are blocking" or "these are minor cleanup items" is a way of dismissing work without user input. It signals "I don't think this matters" — but that judgement is not the agent's to make.

**When work remains at assumed completion:**

1. **Tend towards doing it anyway** — if the items are small and clearly within scope, complete them rather than listing them as deferred
2. **If genuinely uncertain**, surface every outstanding item explicitly to the user with enough context for them to decide: do it now, track it, or descope it
3. **Never batch outstanding items into a dismissive summary** like "a few minor cleanup items remain" — enumerate them individually

**FORBIDDEN language in completion reports:**

- "None of these are blocking" — the user decides what blocks
- "Minor cleanup remaining" — the user decides what's minor
- "Out of scope for this epic" — the user decides scope
- "Can be done in a future session" — the user decides timing
- "Low priority items" — the user decides priority

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

```text
## Remaining Items
- 322 bare-ID warnings — not blocking
- 8 stale docs — minor cleanup
- 5 skill renames — deferred due to complexity
```
^ This is the agent triaging outstanding work without user input. Each item must be surfaced individually for the user to decide.

## Related Rules

- [RULE-015](RULE-015) (honest-reporting) — deferring deliverables is a form of false "complete"
- [RULE-022](RULE-022) (plan-mode-compliance) — plans must reconcile with roadmap scope
- [RULE-020](RULE-020) (no-stubs) — scaffolded implementations are not deliverables
- [RULE-004](RULE-004) (artifact-lifecycle) — epic status transitions require all deliverables complete
