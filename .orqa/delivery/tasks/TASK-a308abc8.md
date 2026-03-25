---
id: "TASK-a308abc8"
type: "task"
title: "Add prompt-based injection to Rust system prompt builder"
description: "Implement prompt-based skill injection in the app's Rust system prompt builder\nusing the local embeddings engine for semantic similarity matching.\n"
status: "completed"
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "Skill embeddings computed and stored at project load"
  - "User prompts matched against skill descriptions by cosine similarity"
  - "Top-N threshold prevents over-injection"
  - "Dedup against skills already in context"
relationships:
  - target: "EPIC-56940fa8"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-2634ad36"
    type: "depends-on"
---

## What

In the app context, use the local ONNX+DuckDB embeddings engine to match user
prompts against skill descriptions and inject the most relevant skills into the
system prompt.

## How

1. Embed each skill's description (one-time at project load)
2. On each user message, embed the prompt
3. Find top-N most relevant skills by cosine similarity
4. Inject skill content into the system prompt
5. Dedup against skills already in context

## Verification

- Skill embeddings are computed and stored
- User prompt returns relevant skills by similarity
- Top-N threshold prevents over-injection