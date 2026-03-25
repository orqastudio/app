---
id: KNOW-51de8fb7
type: knowledge
title: Artifact Status Management
description: |
  Teaches agents when and how to correctly update artifact status fields.
  The unified status vocabulary has 11 values. Agents must update status to reflect
  the true state of thought — especially the review gate, which only humans may clear.
  Use when: Creating, updating, or completing any artifact during task execution.
status: active
created: 2026-03-15
updated: 2026-03-15
category: methodology
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-bad8e26f
    type: synchronised-with
tier: "stage-triggered"
roles:
  - "implementer"
  - "reviewer"
stages:
  - "implement"
  - "review"
tags:
  - "status"
  - "lifecycle"
  - "artifacts"
priority: "P2"
summary: |
  Artifact status management: rules for transitioning artifact status values.
  Status drives workflow progression and enforcement. Query schema for valid
  transitions.
---

## Purpose

Status is not metadata. It is the artifact's current **state of thought** — how far along an idea, task, or decision has been developed, and whether it needs human attention. Agents must keep status accurate because the orchestrator, the user, and the app all read status to understand what is happening.

---

## The 11 Statuses

| Status | When to Use |
|--------|-------------|
| `captured` | The artifact was just created. Nothing has been done with it yet. |
| `exploring` | Active investigation is underway. Research is being gathered. |
| `ready` | The artifact is shaped and well-defined. Nothing is blocking it — it just hasn't been picked up. |
| `prioritised` | A human has decided this is next. It is scheduled for action. |
| `active` | Work is happening right now on this artifact. |
| `hold` | Work is paused intentionally. Not blocked — just deferred. |
| `blocked` | Cannot proceed. A dependency or external factor is preventing progress. |
| `review` | Work is done enough for a human to examine. Needs human attention before proceeding. |
| `completed` | A human has verified the work and confirmed it is finished. |
| `surpassed` | No longer active. Superseded by newer work or a changed context. |
| `recurring` | A pattern that keeps appearing. Used for lessons that need promotion to a rule or skill. |

---

## When Agents Must Update Status

### On creating a new artifact

Set `status: captured`. The artifact exists but nothing has been done with it yet. Even if you immediately start working on it, capture first.

### On beginning work

When you (the agent) start working on an artifact, set `status: active`. Do not leave an artifact in `ready` or `prioritised` while you are actively implementing it.

### On completing work

When your implementation is done, set `status: review`. **Never set `completed` yourself.** Completion requires human verification. Your job is to put the artifact in front of a human for review — not to declare it finished.

### On encountering a blocker

If work cannot proceed because of a dependency or external factor, set `status: blocked`. Include a note in the artifact body explaining what is blocking and what needs to happen to unblock it.

### On pausing without a blocker

If work is pausing for a non-blocking reason (deprioritised, waiting for a related decision, stepping away), set `status: hold`. This distinguishes intentional pausing from being stuck.

### On finding an artifact is no longer relevant

If an artifact is superseded by newer thinking — a better approach was found, the need went away, or the artifact was merged into something else — set `status: surpassed`. Surpassed artifacts are never deleted. They are historical record.

### When a parent artifact's children all reach review

When all tasks in an epic reach `review`, the epic itself moves to `review`. Watch for this pattern and update the parent accordingly.

---

## The Review Gate (Critical)

`review` is the **single human gate** in the system. It is the universal "needs human attention" state.

- An agent setting `review` is saying: "I have done what I can. A human needs to verify this."
- A human setting `completed` is saying: "I have checked this. It is done."

These are two different acts. Agents do the first. Only humans do the second.

**Every path to `completed` passes through `review`.** There is no shortcut.

---

## What Agents Must NOT Do

- **Never set `completed`** without explicit human approval. This applies even if you are confident the work is correct. Confidence is not verification.
- **Never skip `review`**. Going directly from `active` to `completed` is a process violation. All completion goes through `review`.
- **Never leave an artifact in `active` if work has stopped**. If you have finished working on something, move it forward (`review`) or pause it (`hold`). An artifact stuck in `active` with no work happening is misleading.
- **Never set `surpassed` as a way to close work you haven't done**. Surpassed means the work was superseded — not skipped. Use `hold` or `blocked` if you cannot proceed.
- **Never backfill status**. Set status at the moment it changes. Do not update a batch of artifacts after the fact to match the work that was done.

---

## Transition Reference

| Transition | Who | Condition |
|-----------|-----|-----------|
| `captured → exploring` | Human | Human approves investigation |
| `captured → active` | Agent | Immediate work begins on a newly created artifact |
| `exploring → ready` | Human | Investigation is complete, scope is confirmed |
| `ready → active` | Agent | Agent picks up the artifact to implement |
| `ready → prioritised` | Human | Human schedules it as next |
| `prioritised → active` | Agent | Agent begins the prioritised work |
| `active → review` | Agent | Implementation is done, ready for human verification |
| `active → hold` | Agent or Human | Work pauses without a blocker |
| `active → blocked` | Agent | A dependency prevents progress |
| `blocked → active` | Agent or Human | Blocker is resolved |
| `hold → active` | Agent or Human | Work resumes |
| `review → completed` | Human | Human verifies and approves |
| `review → active` | Human | Human sends back for further work |
| Any → `surpassed` | Human | Human decides the artifact is no longer relevant |
| `active → recurring` | System/Agent | Lesson recurrence threshold is reached |
| `recurring → completed` | Human | Human confirms the lesson has been promoted |

---

## Related Artifacts

- [AD-487e045a](AD-487e045a) — Icon-based status representation decision
- [DOC-61ecc85e](DOC-61ecc85e) — Status & Workflow user guide
