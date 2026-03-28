---
id: "TASK-49bf65bc"
type: "task"
title: "Add tests for untested command modules"
description: "Write unit tests for the 8 Tauri command modules that have zero test coverage."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
assignee: "AGENT-e5dd38e4"
acceptance:
  - "Test modules exist in all 8 command files"
  - "Each command module has at least 3 tests covering happy path, error path, and edge case"
  - "make test-rust passes"
relationships:
  - target: "EPIC-2bf6887a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-1c7cab8d"
    type: "depends-on"
---

## What

Add test coverage for: enforcement_commands, governance_commands, graph_commands, lesson_commands, project_settings_commands, search_commands, stream_commands.

## How

1. Read each command module to understand its functions
2. Write inline `#[cfg(test)] mod tests` with tests for each public function
3. Mock repository/service dependencies using trait objects
4. Test error paths and edge cases

## Verification

`make test-rust` passes. Each of the 8 files now contains a `mod tests` block.
