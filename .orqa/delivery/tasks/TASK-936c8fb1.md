---
id: TASK-936c8fb1
type: task
title: Update PipelineStepper to show valid transitions
description: "Extend the PipelineStepper component to display which status transitions are valid from the artifact's current state, and make those transitions clickable so users can apply them directly from the stepper without manually editing the artifact file."
status: blocked
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 2
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - PipelineStepper shows which stages the artifact can transition to from its current status
  - "Clickable transitions that update the artifact's status via the SDK"
  - Only shows transitions that are valid for the current artifact type context
relationships:
  - target: EPIC-6ea301d2
    type: delivers
  - target: TASK-6810ef0c
    type: depends-on
---
## What

PipelineStepper currently renders the artifact's current stage in the lifecycle. This task adds interactive transition buttons to the stepper: for each valid next status reachable from the current one, a button appears that — when clicked — calls the artifact-update SDK command to apply the transition. The valid transitions are derived from the same transition map used by the Rust transition engine (TASK-6810ef0c), ensuring the UI and backend agree on what is allowed.

## How

1. Define (or import) a `validTransitions` map in the frontend, keyed by artifact type and current status, returning an array of valid next statuses. This mirrors the transition logic from TASK-6810ef0c.
   Example: `{ epic: { active: ["review"], review: ["completed"] }, task: { ready: ["active"], active: ["completed", "blocked"] } }`.
2. In `PipelineStepper`, derive the available transitions reactively:
   ```svelte
   const availableTransitions = $derived(
     validTransitions[props.artifactType]?.[props.currentStatus] ?? []
   );
   ```
3. Render a row of transition buttons beneath (or alongside) the current stage indicator — one button per available transition labelled with the target status.
4. On click, call the artifact-update Tauri command (via the store, not directly from the component) with the new status value.
5. Disable transition buttons while an update is in flight (loading state).
6. `PipelineStepper` remains a display component — it receives `artifactType`, `currentStatus`, and an `onTransition(newStatus)` callback as props. The parent page/store owns the `invoke()` call.
7. Only show transitions valid for the artifact type passed via props — do not render a generic status picker.

## Verification

- Component test: epic in status `active` renders a single "review" transition button.
- Component test: task in status `ready` renders "active" transition button.
- Component test: artifact type with no transitions from current status renders no transition buttons.
- Component test: transition button is disabled while `isUpdating` prop is true.
- Clicking a transition button invokes `onTransition` callback with the target status string.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)