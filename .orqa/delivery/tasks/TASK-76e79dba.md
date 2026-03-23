---
id: TASK-76e79dba
type: task
title: Write core architecture documentation
description: Complete end-to-end documentation of the target core application architecture.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-ec1b3785
acceptance:
  - "Architecture doc covers: artifact system, knowledge graph, prompt injection, rule enforcement, learning loop"
  - Every module in the codebase appears in the architecture map
  - Data flow diagrams trace end-to-end paths
  - Document lives in .orqa/documentation/development/
relationships:
  - target: EPIC-4726cb3b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-caa1dd3c
    type: depends-on
  - target: TASK-e3e503a9
    type: depends-on
  - target: TASK-d9d85326
    type: depended-on-by
  - target: TASK-91bc09f9
    type: depended-on-by
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