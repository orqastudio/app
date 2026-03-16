---
id: TASK-116
title: Design IPC command catalogue
description: "Catalogued every Tauri command with its input types, output types, error cases, and domain assignment."
status: completed
created: 2026-03-02
updated: 2026-03-02
acceptance:
  - Every planned IPC command is listed with full type signatures
  - Input/output types are defined as Rust structs
  - Error taxonomy covers all command failure modes
relationships:
  - target: EPIC-029
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-319
    type: depended-on-by
---
## What

Catalogued every planned Tauri IPC command with full input/output type signatures, error cases, and the domain module responsible for implementation.

## How

Grouped commands by domain (conversation, artifacts, governance, settings), defined Rust struct shapes for each command's input and output, and mapped each error case to the OrqaError taxonomy.

## Verification

IPC command catalogue is complete with type signatures for every command, all error cases are mapped, and each command is assigned to a domain module.
