---
id: EPIC-001
title: AI Transparency Wiring
description: "Wire the emission logic that connects existing AI transparency types, components, and store handling into a working end-to-end pipeline."
status: completed
priority: P1
created: 2026-03-07
updated: 2026-03-07
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: RES-026
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-026
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-001
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-002
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-003
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-311
    type: delivered-by
    rationale: Epic contains this task
  - target: DOC-036
    type: informed-by
    rationale: Referenced in documentation page Artifact Framework
  - target: PILLAR-001
    type: grounded-by
  - target: IDEA-013
    type: evolves-from
---
## Why P1

Can't debug reasoning without seeing what's sent to the model. This is a reasoning platform — transparency into what the AI receives and thinks is foundational.

## Context

- `StreamEvent::SystemPromptSent` and `StreamEvent::ContextInjected` types: defined in Rust + TypeScript
- `ContextEntry.svelte` component: production-ready (36 lines)
- `ContextDetailDialog.svelte`: production-ready (182 lines, tabs for Structured/Raw)
- `ThinkingBlock.svelte`: production-ready (45 lines, auto-collapse, streaming indicator)
- Store accumulation for thinking deltas: done

## Tasks

- [x] [[TASK-001](TASK-001)] Emit `SystemPromptSent` event from `stream_commands.rs` (backend-engineer)
- [x] [[TASK-002](TASK-002)] Verify end-to-end rendering (qa-tester)
- [x] [[TASK-003](TASK-003)] Update streaming pipeline documentation (documentation-writer)

## Additional Completed Work

- [x] [[TASK-004](TASK-004)] Emit `ContextInjected` event when prior messages exist in session (backend-engineer)

## Out of Scope (handled by other epics)
- `show_thinking` project setting toggle — [EPIC-002](EPIC-002) (Settings UI)
- Custom system prompt — [EPIC-002](EPIC-002) (Settings UI)

## Notes

- `ContextEntry` and `ThinkingBlock` rendering is already wired in `ConversationView.svelte`
- Store accumulation for `system_prompt_sent` and `context_injected` events already works
- The only missing piece is the Rust backend emission of `SystemPromptSent`

## Implementation Design

Implementation approach to be defined during planning.
