---
id: TASK-266
title: Write core architecture documentation
description: Complete end-to-end documentation of the target core application architecture.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-007
acceptance:
  - "Architecture doc covers: artifact system, knowledge graph, prompt injection, rule enforcement, learning loop"
  - Every module in the codebase appears in the architecture map
  - Data flow diagrams trace end-to-end paths
  - Document lives in .orqa/documentation/development/
relationships:
  - target: EPIC-057
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-258
    type: depends-on
  - target: TASK-259
    type: depends-on
  - target: TASK-267
    type: depended-on-by
  - target: TASK-347
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
