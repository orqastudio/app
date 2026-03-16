---
id: TASK-260
title: Enable clippy pedantic in Cargo.toml
description: Explicitly configure clippy pedantic lints in Cargo.toml and fix resulting warnings.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-002
acceptance:
  - "[lints.clippy] section exists in Cargo.toml with pedantic enabled"
  - make lint-backend passes with zero warnings
  - "Any necessary #[allow] annotations have documented justification"
relationships:
  - target: EPIC-057
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-261
    type: depended-on-by
  - target: TASK-263
    type: depended-on-by
  - target: TASK-347
    type: depended-on-by
---

## What

[RULE-006](RULE-006) claims clippy pedantic is enabled but it's not explicitly configured. Add it properly.

## How

1. Add `[lints.clippy]` section to `backend/src-tauri/Cargo.toml`
2. Enable `pedantic = { level = "warn", priority = -1 }`
3. Run `make lint-backend`, fix all new warnings
4. Document any necessary `#[allow]` exceptions

## Verification

`make lint-backend` passes cleanly. `[lints.clippy]` section visible in Cargo.toml.
