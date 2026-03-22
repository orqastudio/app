---
id: "TASK-df17333f"
title: "Verify end-to-end AI transparency rendering"
description: "Verify the full AI transparency pipeline works end-to-end: Rust emission to Channel<T> to store accumulation to component rendering."
status: "completed"
created: "2026-03-07"
updated: "2026-03-07"
assignee: "AGENT-b0774726"
acceptance:
  - "System prompt sent (N chars) appears inline above assistant response when project is loaded"
  - "Clicking the entry opens ContextDetailDialog with governance prompt text visible"
  - "ThinkingBlock renders during streaming if thinking deltas arrive"
  - "No context entry appears when no project is loaded"
  - "Conversation streaming works normally in all cases (no regression)"
relationships:
  - target: "EPIC-e045ab6d"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-b6aa7a3b"
    type: "depended-on-by"
---
## What

After [TASK-58a9d218](TASK-58a9d218) is implemented, verify the full pipeline works: Rust emission → Channel<T> → store accumulation → component rendering.

## Test Cases

1. **With project loaded:** Send a message → verify "System prompt sent (N chars)" appears inline
2. **Inspect dialog:** Click the context entry → verify dialog opens with Structured/Raw tabs
3. **Structured tab:** Shows "Governance Prompt" section with char count and content
4. **Raw tab:** Shows full prompt text in code block
5. **Without project:** Send a message → no context entry → no error
6. **ThinkingBlock:** If thinking deltas arrive, verify amber block appears and auto-collapses
7. **No regression:** Tool calls, streaming text, cancel all still work

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
