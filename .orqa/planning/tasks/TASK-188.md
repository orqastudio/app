---
id: TASK-188
title: Port enforcement engine to Rust backend
description: Implement the rule enforcement engine in Rust for app-native enforcement.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-050
depends-on:
  - TASK-185
assignee: AGENT-002
docs:
  - DOC-021
skills:
  - SKILL-043
  - SKILL-009
  - SKILL-010
  - SKILL-045
acceptance:
  - Rust module loads rules from .orqa/governance/rules/
  - Module parses YAML frontmatter including enforcement array
  - Module evaluates patterns against tool call context
  - Module returns block/warn/allow decisions
  - Unit tests cover loading, parsing, and pattern matching
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
