---
id: TASK-122
title: "Define error taxonomy"
description: "Defined typed errors across the Rust/IPC/TypeScript boundary using thiserror on the Rust side and discriminated unions on the TypeScript side."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-029
depends-on: []
scope:
  - Define OrqaError enum with all error variants
  - Define From trait implementations for error composition
  - Design error serialization across the IPC boundary
  - Define TypeScript error handling patterns
acceptance:
  - Error taxonomy covers all domain error cases
  - From implementations enable propagation across module boundaries
  - Error serialization produces meaningful messages for the frontend
---
## What

Defined the complete error taxonomy spanning the Rust OrqaError enum, From trait implementations for cross-module propagation, IPC serialization, and TypeScript discriminated union patterns.

## How

Enumerated all OrqaError variants (FileSystem, Database, Serialization, Sidecar, Governance, NotFound, etc.), documented the From implementations that enable ? propagation, and specified how errors serialize to structured JSON messages consumed by the TypeScript frontend.

## Verification

Error taxonomy documentation covers all domain error cases, From implementations are listed for each module boundary, and IPC serialization format produces meaningful messages for frontend display.
