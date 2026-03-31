---
id: IMPL-b86a46ad
type: lesson
title: Governance learnings use write-through pattern — lesson AND memory simultaneously, with session-start reconciliation
category: process
status: active
recurrence: 1
created: 2026-03-24
tags: [memory, lessons, governance, sync-pattern]
relationships:
  - type: cautions
    target: EPIC-3e6cad90
    rationale: "Write-through pattern identifies a coverage gap in the learning pipeline — governance learnings must hit both memory and lesson pipeline"
---

## Observation

The user identified that the auto-memory system and the lessons pipeline serve overlapping but distinct purposes. Process learnings (feedback, project corrections) were being captured only in auto-memory, bypassing the lessons pipeline entirely. This means governance-relevant knowledge was not entering the artifact graph, not tracked for recurrence, and not eligible for promotion to rules.

## Pattern

When the orchestrator learns something about process or governance:

1. It writes to auto-memory (e.g., `feedback_*.md`) because Claude's system prompt instructs it to
2. The lesson is never created in `.orqa/process/lessons/`
3. The learning has no recurrence tracking, no promotion pipeline, no graph edges
4. The same correction recurs across sessions because only auto-memory sees it — not the governance system

## Correct Flow: Write-Through Pattern

The lesson pipeline is the system of record for governance learnings. Auto-memory provides immediate cross-session context. Both are written simultaneously — this is a **write-through** pattern, not a sequential pipeline.

### During a session (write-through)

When a governance learning is captured (user correction, process insight, feedback):

1. **Create the lesson** in `.orqa/process/lessons/IMPL-*.md` (with recurrence, tags, relationships)
2. **Write to auto-memory** at the same time (e.g., `feedback_*.md` or `project_*.md`)
3. Both artifacts exist immediately — the lesson for governance tracking, the memory for next-session context

### At session start (reconciliation)

The orchestrator scans `.orqa/process/lessons/` and checks whether each active/recurring lesson has a corresponding auto-memory entry. Any lesson not yet reflected in memory is synced.

This catches:

- Lessons created by other agents that didn't write memory
- Lessons promoted from other sources (review failures, audit findings)
- Memory entries that were lost or cleaned up

### What goes where

| Knowledge Type | Lesson (.orqa/) | Auto-memory | Both |
| --- | :---: | :---: | :---: |
| Process corrections (feedback) | Y | Y | **Write-through** |
| Project governance state | Y | Y | **Write-through** |
| User preferences (role, style) | --- | Y | Memory only |
| Reference pointers (URLs, locations) | --- | Y | Memory only |
| Non-governance context | --- | Y | Memory only |

The lesson pipeline is the system of record. Auto-memory is the delivery mechanism for immediate context. Neither replaces the other.

## Research

Detailed options analysis captured in memory file `research_lessons_as_memory_source.md`. The recommended approach is a hybrid of behavioral override (connector preamble tells agents to prefer lessons for governance knowledge) and PostToolUse conversion (hook auto-converts governance-relevant memory writes to lesson artifacts).

## Next Steps

- Phase 1: Update connector preamble with write-through guidance (lesson + memory simultaneously)
- Phase 2: Implement session-start reconciliation hook (scan lessons, sync missing to memory)
- Phase 3: Implement PostToolUse hook to catch memory-only writes and prompt lesson creation
- Track recurrence — if this pattern is violated again, promote to rule
