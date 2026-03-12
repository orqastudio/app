---
id: TASK-245
title: "Add search module tests (embedder.rs + store.rs)"
description: "search/embedder.rs (331 lines) and search/store.rs (394 lines) handle ONNX inference and DuckDB queries with zero test coverage."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-055
acceptance:
  - "embedder.rs has tests for model loading, embedding generation, error handling"
  - "store.rs has tests for insert, search, deletion, edge cases"
  - "Tests use in-memory DuckDB where possible"
  - "make test-rust passes"
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
