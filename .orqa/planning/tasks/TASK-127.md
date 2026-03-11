---
id: TASK-127
title: "Implement remaining IPC commands across all domains"
description: "Built the full IPC command surface covering settings, governance, tools, sidecar management, and system information."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-030
depends-on: []
scope:
  - Implement settings commands (get/set provider config, model selection)
  - Implement governance commands (scan, analyse, recommend)
  - Implement tool execution commands (file read, write, glob)
  - Implement system info commands (hardware, environment)
  - Register all commands in Tauri app builder
acceptance:
  - All commands are registered and callable from the frontend
  - Each command returns typed results
  - Error handling uses OrqaError
---
## What

Completed the full IPC command surface across settings, governance, file tools, and system info domains.

## How

Implemented `#[tauri::command]` handlers in domain-specific command modules (`settings_commands.rs`, `governance_commands.rs`, `tool_commands.rs`) and registered all commands in `lib.rs`.

## Verification

All commands are callable from the frontend, return typed results, and surface errors via `OrqaError`.
