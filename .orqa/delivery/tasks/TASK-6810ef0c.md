---
id: "TASK-6810ef0c"
type: "task"
title: "Build automatic status transition engine in Rust"
description: "Implement a Rust domain module that evaluates artifact transition conditions across the loaded graph and returns a list of proposed status transitions. The engine does not auto-apply — it returns proposals that callers can approve or queue for auto-apply."
status: "ready"
priority: "P1"
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 5
created: 2026-03-15T00:00:00.000Z
updated: 2026-03-15T00:00:00.000Z
horizon: "active"
acceptance:
  - "Rust module in backend/src-tauri/src/domain/ that evaluates transition conditions"
  - "Checks: all-tasks-completed→epic-review, all-P1-epics-completed→milestone-review, dependency-blocked→task-blocked, recurrence-threshold→lesson-review"
  - "Returns a list of proposed transitions (artifact ID, current status, proposed status, reason)"
  - "Does NOT auto-apply — returns proposals for approval or auto-apply"
relationships:
  - target: "EPIC-6ea301d2"
    type: "delivers"
  - target: "TASK-46c06db8"
    type: "depends-on"
---
## What

A new Rust domain module (`backend/src-tauri/src/domain/transition_engine.rs` or equivalent) that analyses the loaded artifact graph and evaluates a fixed set of transition conditions. When a condition is satisfied, the engine emits a `TransitionProposal` — a value object containing the artifact ID, current status, proposed status, and a human-readable reason string. The engine never mutates state; callers decide whether to auto-apply or queue for human approval.

## How

1. Define a `TransitionProposal` struct:
   ```rust
   pub struct TransitionProposal {
       pub artifact_id: String,
       pub current_status: String,
       pub proposed_status: String,
       pub reason: String,
   }
   ```
2. Implement `evaluate_transitions(graph: &ArtifactGraph) -> Vec<TransitionProposal>` with the following checks:
   - **all-tasks-completed → epic-review**: For each epic with status `active`, if all child tasks have status `completed`, propose transition to `review`.
   - **all-P1-epics-completed → milestone-review**: For each milestone with status `active`, if all P1 epics have status `completed`, propose transition to `review`.
   - **dependency-blocked → task-blocked**: For each task with status `active` or `ready`, if any `depends-on` target has a non-`completed` status, propose transition to `blocked`.
   - **recurrence-threshold → lesson-review**: For each lesson where `recurrence >= 2` and status is not `recurring` or `promoted`, propose transition to `recurring`.
3. Return all proposals as a `Vec<TransitionProposal>`. Empty vec means no transitions are needed.
4. All functions return `Result<_, E>` — no `unwrap()` or `expect()`.
5. Export the module from `domain/mod.rs`.

## Verification

- Unit test: epic with all child tasks `completed` produces a `review` proposal.
- Unit test: epic with at least one task not `completed` produces no proposal.
- Unit test: task with an incomplete dependency produces a `blocked` proposal.
- Unit test: lesson with `recurrence = 2` and status `active` produces a `recurring` proposal.
- Unit test: milestone with all P1 epics `completed` produces a `review` proposal.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)