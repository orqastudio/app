---
id: "EPIC-e045ab6d"
type: epic
title: "AI Transparency Wiring"
description: "Wire the emission logic that connects existing AI transparency types, components, and store handling into a working end-to-end pipeline."
status: "completed"
priority: "P1"
created: "2026-03-07"
updated: "2026-03-07"
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: "RES-b126f17a"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-b126f17a"
  - target: "TASK-58a9d218"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-df17333f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-fa777da9"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-b6aa7a3b"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "IDEA-bfeda324"
    type: "realised-by"
  - target: "DOC-01ddd8aa"
    type: "documented-by"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
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

- [x] [[TASK-58a9d218](TASK-58a9d218)] Emit `SystemPromptSent` event from `stream_commands.rs` (backend-engineer)
- [x] [[TASK-df17333f](TASK-df17333f)] Verify end-to-end rendering (qa-tester)
- [x] [[TASK-fa777da9](TASK-fa777da9)] Update streaming pipeline documentation (documentation-writer)

## Additional Completed Work

- [x] [[TASK-5e893805](TASK-5e893805)] Emit `ContextInjected` event when prior messages exist in session (backend-engineer)

## Out of Scope (handled by other epics)
- `show_thinking` project setting toggle — [EPIC-642234ba](EPIC-642234ba) (Settings UI)
- Custom system prompt — [EPIC-642234ba](EPIC-642234ba) (Settings UI)

## Notes

- `ContextEntry` and `ThinkingBlock` rendering is already wired in `ConversationView.svelte`
- Store accumulation for `system_prompt_sent` and `context_injected` events already works
- The only missing piece is the Rust backend emission of `SystemPromptSent`

## Implementation Design

Implementation approach to be defined during planning.