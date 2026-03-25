---
id: DOC-61ecc85e
type: doc
title: "Status & Workflow"
category: reference
description: "How the unified status system works — what each status means, when it changes, and who changes it."
created: 2026-03-15
updated: 2026-03-15
sort: 11
relationships:
  - target: AD-487e045a
    type: documents
---

## Overview

Every artifact in OrqaStudio carries a `status` field. Status is not a pipeline stage — it is the artifact's current **state of thought**: how far along the idea, task, or decision has been developed, and whether it needs human attention.

The same 12 statuses apply across all artifact types. A task, an epic, an idea, and a lesson all use the same vocabulary. This makes the entire artifact graph readable at a glance. The authoritative status definitions live in `project.json`'s `statuses` array — including valid transitions and auto-rules.

---

## The Status Vocabulary

| Status | Icon | Meaning |
|--------|------|---------|
| `captured` | Circle | Exists but not yet examined. The idea or item has been recorded, nothing more. |
| `exploring` | Compass | Under active investigation. Someone is gathering information or doing research. |
| `ready` | Circle Dot | Shaped and waiting. The artifact is well-defined and ready to be picked up. |
| `prioritised` | Circle Star | Scheduled for action. A human has decided this is next. |
| `active` | Circle Dot Dashed *(spinning)* | Work is happening right now. |
| `hold` | Circle Pause | Paused intentionally. Work is not blocked — just deferred. |
| `blocked` | Circle Stop | Cannot proceed. A dependency or blocker is preventing progress. |
| `review` | Circle User Round | Needs a human. Work is complete enough for a person to verify, approve, or decide. |
| `completed` | Circle Check | Done. A human has verified this and confirmed it is finished. |
| `surpassed` | Circle Minus | No longer active. Superseded by newer thinking, or the need has passed. |
| `archived` | Archive | Preserved for historical reference but no longer relevant to active work. |
| `recurring` | Circle Fading Arrow Up | A pattern that keeps appearing. Typically used for lessons pending promotion to a rule or skill. |

---

## The Progression

Not every artifact passes through every status. The path depends on what the artifact is. But the general shape is:

```
captured → exploring → ready → prioritised → active → review → completed
```

At any point an artifact may move to `hold` (paused), `blocked` (stuck), `surpassed` (superseded), `archived` (preserved but inactive), or — for lessons — `recurring` (pattern detected).

### A typical idea

An idea starts as `captured` the moment it is recorded. When a human approves investigation, it moves to `exploring`. Once the investigation is complete and the idea is shaped with a clear scope, it becomes `ready`. A human then decides whether to promote it — moving it to `prioritised` if it is next in line, or eventually creating an epic from it.

### A typical task

Tasks often start as `ready` (they are already scoped when created as part of an epic). An agent picks up the task and sets it to `active`. When the work is done, the agent sets it to `review` — not `completed`. Completion requires human confirmation.

### A lesson

Lessons may carry `recurring` status when the same mistake has appeared multiple times. This is a signal to the orchestrator: this pattern needs to be promoted to a rule or skill update before the lesson can be marked `completed`.

---

## The Review Gate

`review` is the single human gate in the system.

When an agent finishes work, it does not mark the artifact `completed`. It sets it to `review`. This puts the artifact in a queue for human attention. The human reads the work, verifies it, and decides:

- **Approve** — set to `completed`
- **Request changes** — set back to `active` (with notes)
- **Decline** — set to `surpassed` or leave in `review` for discussion

This pattern applies uniformly. An agent never self-certifies completion.

---

## Automatic vs Manual Transitions

### Transitions the system can make automatically

| Trigger | Transition |
|---------|------------|
| All tasks in an epic reach `review` | Epic moves to `review` |
| An agent starts working on an artifact | Artifact moves to `active` |
| A dependency is resolved (blocked item unblocked) | Artifact may move back to `ready` or `active` |
| A lesson's recurrence count reaches the threshold | Lesson status updates to `recurring` |

### Transitions that require human action

| Transition | Why it requires a human |
|-----------|------------------------|
| `captured → exploring` | The human must decide whether investigation is worth pursuing |
| `exploring → ready` | The human confirms the investigation is complete and the scope is sound |
| `ready → prioritised` | The human decides this is the next thing to work on |
| `review → completed` | The human verifies the work meets the acceptance criteria |
| `review → active` | The human sends it back for further work |
| Any → `surpassed` | The human decides the artifact is no longer relevant |
| `recurring → completed` | The human confirms the lesson has been promoted |

---

## How This Differs from Traditional Project Management

Traditional project management uses statuses as **pipeline stages** — work enters at one end and exits at the other in a fixed sequence. Every item follows the same path.

OrqaStudio statuses are **states of thought**. They answer: "Where is the thinking on this?" An item can sit in `captured` indefinitely — that is not a problem. An item in `exploring` may loop back to `captured` if the investigation reveals the premise was wrong. `surpassed` is not failure — it is honest acknowledgment that thinking has moved on.

The progression is not a conveyor belt. It is a map of how ideas and tasks mature.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The unified status vocabulary makes every artifact's state of thought visible at a glance, across all types, without ambiguity. |
| Learning Through Reflection | The `recurring` status creates a visible signal when patterns repeat, feeding directly into the lesson promotion pipeline. |
| Purpose Through Continuity | Status tracking prevents intent drift — an artifact's journey from `captured` to `completed` is visible, so original purpose isn't lost during implementation. |
