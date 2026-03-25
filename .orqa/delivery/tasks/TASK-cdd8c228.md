---
id: TASK-cdd8c228
type: task
title: Wire transition engine to artifact graph refresh cycle
description: Integrate the transition engine into the artifact graph refresh pipeline so that proposed transitions are evaluated after every refresh and surfaced to the frontend via a Tauri event. Unambiguous transitions (e.g. dependency-blocked) apply automatically; human-judgement transitions (e.g. epic→review) are queued for approval.
status: blocked
priority: P1
scoring:
  impact: 5
  urgency: 3
  complexity: 4
  dependencies: 4
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - "After every artifact graph refresh, the transition engine evaluates all artifacts"
  - Proposed transitions surfaced as a Tauri event that the frontend can listen to
  - "Auto-apply mode: transitions that are unambiguous (like blocked dependencies) apply automatically"
  - "Manual-approve mode: transitions that need human judgement (like epic→review) are queued for approval"
relationships:
  - target: EPIC-6ea301d2
    type: delivers
  - target: TASK-6810ef0c
    type: depends-on
---
## What

After every artifact graph refresh, call `evaluate_transitions` from TASK-6810ef0c and route the resulting proposals. Proposals are split into two buckets by transition type: auto-apply (unambiguous, deterministic) and manual-approve (requires human judgement). Auto-apply proposals are applied immediately and written back to disk; manual-approve proposals are emitted to the frontend as a Tauri event so the user can review and accept them.

## How

1. In the artifact graph refresh handler (wherever `load_artifact_graph` is called), invoke `evaluate_transitions(&graph)` after the graph is loaded.
2. Classify each `TransitionProposal` by mode:
   - **Auto-apply**: `dependency-blocked → task-blocked` (deterministic, no human input needed)
   - **Manual-approve**: `epic → review`, `milestone → review`, `lesson → recurring`
3. For auto-apply proposals:
   - Apply the status update by writing the new frontmatter value to the artifact file.
   - Re-trigger a graph refresh so the UI reflects the change.
4. For manual-approve proposals:
   - Accumulate them in a `Vec<TransitionProposal>` and emit a Tauri event `"transition-proposals-ready"` with the list as the payload.
5. Define a `#[tauri::command]` `apply_transition(artifact_id: String, proposed_status: String)` that the frontend can call when the user approves a proposal.
6. Register the new command in the Tauri app builder.
7. Add matching TypeScript interfaces for `TransitionProposal` and the event payload.
8. Add a frontend store that listens to the `"transition-proposals-ready"` event and exposes the pending proposals reactively.
9. All Rust functions return `Result<_, E>`. No `unwrap()` or `expect()`.

## Verification

- Integration test: after a graph refresh with a task whose dependency is incomplete, the task file is updated to `blocked` status on disk.
- Integration test: after a graph refresh with an epic whose tasks are all completed, a `"transition-proposals-ready"` event is emitted with the epic proposal.
- Frontend store receives the event and exposes the proposals as reactive state.
- `apply_transition` command writes the new status to the artifact file and is registered in the builder.
- TypeScript interfaces match Rust struct field names.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)