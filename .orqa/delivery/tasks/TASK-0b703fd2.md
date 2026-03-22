---
id: TASK-0b703fd2
type: task
title: ID generation utilities — Rust and TypeScript
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-4cedf7bc
    type: delivers
  - target: TASK-ad41ea9b
    type: depended-on-by
  - target: TASK-cae3c8c7
    type: depended-on-by
---

# TASK-0b703fd2: ID Generation Utilities

## Acceptance Criteria

1. Rust function `generate_artifact_id(prefix: &str) -> String` in domain module
2. TypeScript function `generateArtifactId(prefix: string): string` in @orqastudio/types or SDK
3. `orqa id generate <type>` CLI command that outputs a new ID
4. All three produce `TYPE-XXXXXXXX` format (8 lowercase hex chars)
5. Collision check against existing graph (warn if collision detected)
