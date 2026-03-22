---
id: AD-1ad08e5f
title: Error Propagation via Result Types
description: "All Rust functions return Result<T, E> with thiserror for error types. No unwrap(), expect(), or panic!() in production code."
status: completed
created: 2026-03-02
updated: 2026-03-13
relationships:
  - target: RULE-b49142be
    type: enforced-by
    rationale: RULE-b49142be enforces this decision via clippy::unwrap_used, clippy::expect_used, and clippy::panic lint entries — no unwrap/expect/panic in production Rust
  - target: DOC-2f60e3a2
    type: documented-by
  - target: DOC-c1d1f212
    type: documented-by
  - target: DOC-dcf96a52
    type: documented-by
  - target: DOC-3c65a1e3
    type: documented-by
  - target: RULE-57ccb4a3
    type: enforced-by
---
## Decision

All Rust functions return `Result<T, E>` using `thiserror` for error types. No `unwrap()`, `expect()`, or `panic!()` in production code.

## Rationale

A desktop app must never crash. Every error path must be handled gracefully and communicated to the user through the UI.

## Consequences

Every command handler returns `Result<T, String>` (or a custom error type). The frontend must handle both success and error responses.
