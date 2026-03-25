---
id: TASK-a5c02ac7
type: task
title: "Lessons-as-memory Phase 1 — behavioral override in connector preamble and PreToolUse hook for memory writes"
description: "Implement Phase 1 of lessons-as-memory: inject active lessons as behavioral overrides into the connector preamble, and add a PreToolUse hook that writes agent-observed lessons to the memory system."
status: captured
created: 2026-03-21
updated: 2026-03-21
acceptance:
  - Connector preamble includes a section that reads active lessons from .orqa/process/lessons/ and injects them as behavioral overrides
  - A PreToolUse hook is added that monitors for lesson-worthy observations and writes them to the memory system
  - Injected lessons are deduplicated per session (same lesson not re-injected on every tool call)
  - Lesson injection is scoped — only lessons relevant to the current task domain are injected
  - Memory writes from the hook follow the existing memory file format (frontmatter + body)
  - orqa enforce passes on any new artifacts created by this feature
relationships:
  - target: EPIC-9b58fdcb
    type: delivers
---

## What

Lessons learned during work (`.orqa/process/lessons/`) should influence agent behavior directly — not just sit as archived documents. Phase 1 implements two things:

1. **Connector preamble injection**: At session start, the connector reads active lessons and includes them as behavioral overrides in the system prompt preamble.
2. **PreToolUse hook for memory writes**: A hook that monitors tool calls for patterns matching known lesson triggers and writes new observations to the memory system.

## Why

Lessons exist to prevent recurrence of past mistakes. If they're only in `.orqa/` files and not injected into agent context, they have no behavioral effect. Phase 1 closes the loop: lessons are discovered during work, written to memory, and injected in future sessions.

Research in TASK-09 (research: lessons as memory source) informed this design.

## How

### Preamble injection
1. At connector startup, scan `.orqa/process/lessons/` for active lessons
2. Filter to lessons relevant to the current session scope (task domain, plugin context)
3. Format as behavioral override statements in the preamble section
4. Deduplicate via the existing session-level injected knowledge cache

### PreToolUse hook
1. Add a `PreToolUse` hook entry to the connector's `hooks.json`
2. The hook receives the tool call details and current session context
3. If the tool call matches a known lesson-trigger pattern (e.g., certain file paths, command patterns), write a memory entry
4. Memory writes use the existing memory file format in `C:\Users\Bobbi\.claude\projects\...\memory\`

## Verification

1. A session starting with active lessons shows those lessons in the injected preamble
2. The PreToolUse hook fires on a test tool call that matches a lesson trigger
3. The resulting memory file is valid (frontmatter parseable, body non-empty)
4. `orqa enforce` passes on the updated connector plugin
5. No duplicate lesson injections occur within a single session