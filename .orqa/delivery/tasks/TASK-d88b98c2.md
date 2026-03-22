---
id: TASK-d88b98c2
title: Complete stream_commands.rs domain extraction
description: "stream_commands.rs is 2,497 lines with ~1,200 duplicated in domain modules. Complete the extraction so the command file is ~150-200 lines delegating to stream_loop.rs, tool_executor.rs, and system_prompt.rs."
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - "stream_commands.rs contains only #[tauri::command] functions and imports (<250 lines)"
  - "All domain logic delegates to domain/stream_loop.rs, domain/tool_executor.rs, domain/system_prompt.rs"
  - Existing 42 tests still pass (moved or adapted as needed)
  - make check passes with zero warnings
relationships:
  - target: EPIC-5f9fcf48
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-389af55e
    type: depended-on-by
---

## What

stream_commands.rs is 2,497 lines with ~1,200 duplicated in domain modules. Complete the extraction so the command file is ~150-200 lines delegating to stream_loop.rs, tool_executor.rs, and system_prompt.rs.

## How

To be determined during implementation.

## Verification

- [ ] stream_commands.rs contains only #[tauri::command] functions and imports (<250 lines)
- [ ] All domain logic delegates to domain/stream_loop.rs, domain/tool_executor.rs, domain/system_prompt.rs
- [ ] Existing 42 tests still pass (moved or adapted as needed)
- [ ] make check passes with zero warnings
