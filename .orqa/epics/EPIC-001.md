---
id: EPIC-001
title: "AI Transparency Wiring"
status: draft
priority: P1
milestone: MS-001
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 2
score: 17.5
roadmap-ref: "D1"
docs-required:
  - docs/architecture/streaming-pipeline.md
docs-produced:
  - docs/architecture/streaming-pipeline.md (update with SystemPromptSent/ContextInjected emission)
tags: [streaming, transparency, reasoning]
---

# AI Transparency Wiring

The types, components, and store handling for AI transparency all exist. Missing: the emission logic that connects them.

## Why P1

Can't debug reasoning without seeing what's sent to the model. This is a reasoning platform — transparency into what the AI receives and thinks is foundational.

## Context

- `StreamEvent::SystemPromptSent` and `StreamEvent::ContextInjected` types: defined in Rust + TypeScript
- `ContextEntry.svelte` component: production-ready (36 lines)
- `ContextDetailDialog.svelte`: production-ready (182 lines, tabs for Structured/Raw)
- `ThinkingBlock.svelte`: production-ready (45 lines, auto-collapse, streaming indicator)
- Store accumulation for thinking deltas: done

## Tasks

- [ ] Emit `SystemPromptSent` event from `stream_commands.rs` — read custom + governance prompts, emit before calling sidecar
- [ ] Emit `ContextInjected` event when context is injected (see EPIC-003)
- [ ] Render `ContextEntry` components in `ConversationView` above assistant messages
- [ ] Wire `ThinkingBlock` rendering in `ConversationView` when `show_thinking` is enabled
