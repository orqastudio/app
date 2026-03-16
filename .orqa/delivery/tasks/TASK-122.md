---
id: TASK-122
title: Define error taxonomy
description: Defined typed errors across the Rust/IPC/TypeScript boundary using thiserror on the Rust side and discriminated unions on the TypeScript side.
status: completed
created: 2026-03-02
updated: 2026-03-02
acceptance:
  - Error taxonomy covers all domain error cases
  - From implementations enable propagation across module boundaries
  - Error serialization produces meaningful messages for the frontend
relationships:
  - target: EPIC-029
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-319
    type: depended-on-by
---
## What

Defined the complete error taxonomy spanning the Rust OrqaError enum, From trait implementations for cross-module propagation, IPC serialization, and TypeScript discriminated union patterns.

## How

Enumerated all OrqaError variants (FileSystem, Database, Serialization, Sidecar, Governance, NotFound, etc.), documented the From implementations that enable ? propagation, and specified how errors serialize to structured JSON messages consumed by the TypeScript frontend.

## Verification

Error taxonomy documentation covers all domain error cases, From implementations are listed for each module boundary, and IPC serialization format produces meaningful messages for frontend display.
