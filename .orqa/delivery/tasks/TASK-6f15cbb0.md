---
id: TASK-6f15cbb0
type: task
title: Port enforcement engine to Rust backend
description: Implement the rule enforcement engine in Rust for app-native enforcement.
status: surpassed
created: 2026-03-11
updated: 2026-03-11
assignee: AGENT-cc255bc8
docs:
  - DOC-4b4fbc0f
acceptance:
  - Rust module loads rules from .orqa/process/rules/
  - Module parses YAML frontmatter including enforcement array
  - Module evaluates patterns against tool call context
  - Module returns block/warn/allow decisions
  - "Unit tests cover loading, parsing, and pattern matching"
relationships:
  - target: EPIC-3a8ad459
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-11cf4c1d
    type: depends-on
  - target: TASK-4556173e
    type: depended-on-by
  - target: TASK-fa39671d
    type: depended-on-by
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