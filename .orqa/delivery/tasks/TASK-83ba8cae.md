---
id: "TASK-83ba8cae"
type: "task"
title: "ID generation utilities — Rust and TypeScript"
status: "captured"
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "EPIC-d1d42012"
    type: "delivers"
---

# TASK-83ba8cae: ID Generation Utilities

## Acceptance Criteria

1. Rust function `generate_artifact_id(prefix: &str) -> String` in domain module
2. TypeScript function `generateArtifactId(prefix: string): string` in @orqastudio/types or SDK
3. `orqa id generate <type>` CLI command that outputs a new ID
4. All three produce `TYPE-XXXXXXXX` format (8 lowercase hex chars)
5. Collision check against existing graph (warn if collision detected)