---
id: "TASK-efaf25d7"
title: "Enable clippy pedantic in Cargo.toml"
description: "Explicitly configure clippy pedantic lints in Cargo.toml and fix resulting warnings."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
assignee: "AGENT-cc255bc8"
acceptance:
  - "[lints.clippy] section exists in Cargo.toml with pedantic enabled"
  - "make lint-backend passes with zero warnings"
  - "Any necessary #[allow] annotations have documented justification"
relationships:
  - target: "EPIC-4726cb3b"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-54fae8bf"
    type: "depended-on-by"
  - target: "TASK-9ca53d45"
    type: "depended-on-by"
  - target: "TASK-91bc09f9"
    type: "depended-on-by"
---
## What

[RULE-b49142be](RULE-b49142be) claims clippy pedantic is enabled but it's not explicitly configured. Add it properly.

## How

1. Add `[lints.clippy]` section to `backend/src-tauri/Cargo.toml`
2. Enable `pedantic = { level = "warn", priority = -1 }`
3. Run `make lint-backend`, fix all new warnings
4. Document any necessary `#[allow]` exceptions

## Verification

`make lint-backend` passes cleanly. `[lints.clippy]` section visible in Cargo.toml.
