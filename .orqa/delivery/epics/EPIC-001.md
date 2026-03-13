---
id: EPIC-001
title: AI Transparency Wiring
description: Wire the emission logic that connects existing AI transparency types, components, and store handling into a working end-to-end pipeline.
status: done
priority: P1
created: 2026-03-07
updated: 2026-03-07
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-026
docs-required: []
docs-produced: []
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 2
  score: 17.5
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
