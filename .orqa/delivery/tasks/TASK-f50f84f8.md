---
id: "TASK-f50f84f8"
type: "task"
title: "Write core architecture documentation"
description: "Complete end-to-end documentation of the target core application architecture."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
assignee: "AGENT-bbad3d30"
acceptance:
  - "Architecture doc covers: artifact system, knowledge graph, prompt injection, rule enforcement, learning loop"
  - "Every module in the codebase appears in the architecture map"
  - "Data flow diagrams trace end-to-end paths"
  - "Document lives in .orqa/documentation/development/"
relationships:
  - target: "EPIC-2bf6887a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-d1b856b5"
    type: "depends-on"
  - target: "TASK-cc7df87c"
    type: "depends-on"
---

## What

Create comprehensive architecture documentation that maps the entire OrqaStudio core application end-to-end.

## How

1. Map every Rust module, its purpose, and its dependencies
2. Map every frontend store and its relationship to backend commands
3. Document the streaming pipeline (LLM → sidecar → Rust → Svelte)
4. Document the artifact system (scanning, graph, rendering)
5. Document the enforcement pipeline (rules → engine → gates → injection)
6. Document the learning loop (lessons → promotion → rules)
7. Document the prompt injection pipeline (system prompt → skills → context)

## Verification

A reader can trace any feature from UI to database by following the architecture doc.
