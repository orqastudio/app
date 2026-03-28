---
id: "TASK-577f3ed9"
type: "task"
title: "Port enforcement engine to Rust backend"
description: "Implement the rule enforcement engine in Rust for app-native enforcement."
status: archived
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-11T00:00:00.000Z
docs:
  - "DOC-9814ec3c"
acceptance:
  - "Rust module loads rules from .orqa/process/rules/"
  - "Module parses YAML frontmatter including enforcement array"
  - "Module evaluates patterns against tool call context"
  - "Module returns block/warn/allow decisions"
  - "Unit tests cover loading, parsing, and pattern matching"
relationships:
  - target: "EPIC-9a1eba3f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-d38a48c9"
    type: "depends-on"
---

## What

Port the companion plugin's rule engine logic to Rust so the app can enforce
rules natively without depending on the CLI plugin.

## How

1. Create `backend/src-tauri/src/domain/enforcement.rs` module
2. Implement rule loading from filesystem (reuse artifact scanner frontmatter parsing)
3. Implement enforcement pattern evaluation using `regex` crate
4. Implement decision logic (block/warn/allow)
5. Write unit tests

## Verification

- `cargo test` passes for enforcement module
- Engine produces same decisions as the plugin for the same rule set
