---
id: "TASK-f5b9530a"
type: "task"
title: "Add search module tests (embedder.rs + store.rs)"
description: "search/embedder.rs (331 lines) and search/store.rs (394 lines) handle ONNX inference and DuckDB queries with zero test coverage."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "embedder.rs has tests for model loading, embedding generation, error handling"
  - "store.rs has tests for insert, search, deletion, edge cases"
  - "Tests use in-memory DuckDB where possible"
  - "make test-rust passes"
relationships:
  - target: "EPIC-a1555708"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

search/embedder.rs (331 lines) and search/store.rs (394 lines) handle ONNX inference and DuckDB queries with zero test coverage.

## How

To be determined during implementation.

## Verification

- [ ] embedder.rs has tests for model loading, embedding generation, error handling
- [ ] store.rs has tests for insert, search, deletion, edge cases
- [ ] Tests use in-memory DuckDB where possible
- [ ] make test-rust passes
