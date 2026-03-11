---
id: TASK-205
title: Add prompt-based injection to Rust system prompt builder
description: |
  Implement prompt-based skill injection in the app's Rust system prompt builder
  using the local embeddings engine for semantic similarity matching.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-200
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
