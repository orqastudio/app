---
id: "TASK-d02fcc23"
type: "task"
title: "Implement remaining IPC commands across all domains"
description: "Built the full IPC command surface covering settings, governance, tools, sidecar management, and system information."
status: "completed"
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-02T00:00:00.000Z
acceptance:
  - "All commands are registered and callable from the frontend"
  - "Each command returns typed results"
  - "Error handling uses OrqaError"
relationships:
  - target: "EPIC-713c48c6"
    type: "delivers"
    rationale: "Task belongs to this epic"
---
## What

Completed the full IPC command surface across settings, governance, file tools, and system info domains.

## How

Implemented `#[tauri::command]` handlers in domain-specific command modules (`settings_commands.rs`, `governance_commands.rs`, `tool_commands.rs`) and registered all commands in `lib.rs`.

## Verification

All commands are callable from the frontend, return typed results, and surface errors via `OrqaError`.