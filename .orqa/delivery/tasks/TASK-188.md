---
id: TASK-188
title: Port enforcement engine to Rust backend
description: Implement the rule enforcement engine in Rust for app-native enforcement.
status: surpassed
created: 2026-03-11
updated: 2026-03-11
assignee: AGENT-002
docs:
  - DOC-021
acceptance:
  - Rust module loads rules from .orqa/process/rules/
  - Module parses YAML frontmatter including enforcement array
  - Module evaluates patterns against tool call context
  - Module returns block/warn/allow decisions
  - "Unit tests cover loading, parsing, and pattern matching"
relationships:
  - target: EPIC-050
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-415
    type: evolves-from
  - target: TASK-185
    type: depends-on
  - target: SKILL-043
    type: grounded-by
  - target: SKILL-009
    type: grounded-by
  - target: SKILL-010
    type: grounded-by
  - target: TASK-189
    type: depended-on-by
  - target: TASK-340
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
