---
id: "RULE-f10bb5de"
title: "Testing Standards"
description: "Test organisation, coverage requirements, mock boundaries, and isolation rules."
status: "active"
created: "2026-03-07"
updated: "2026-03-07"
relationships:
  - target: "AD-e156310d"
    type: "enforces"
---
**Source of Truth:** This file defines testing patterns. `.orqa/documentation/development/coding-standards.md` defines coverage requirements.

## Test Organisation

| Location | Type | Purpose |
|----------|------|---------|
| `backend/src-tauri/src/**/tests/` | Rust unit tests | Single-module logic, domain functions, utilities |
| `backend/src-tauri/tests/` | Rust integration tests | Cross-module flows, database interactions, file system operations |
| `ui/**/*.test.ts` | Frontend unit tests | Svelte components, stores, utility functions (Vitest) |
| `tests/` (root) | E2E tests | Playwright browser tests against the running Tauri app |

## Coverage Requirements

- **Rust backend**: >=80% per module (`cargo tarpaulin --fail-under 80` or equivalent coverage tool)
- **Frontend**: Run `npm run test` — all tests must pass (Vitest)
- **E2E**: All user flows in `.orqa/documentation/reference/` must have Playwright coverage

## Mock Boundaries

**Mock ONLY at the adapter/boundary layer.** Never mock domain logic, orchestrators, or internal functions.

- Correct: Mock trait implementing `SessionRepository` for testing without SQLite
- Correct: Mock `FileSystem` trait for testing without disk access
- FORBIDDEN: Mocking internal functions, patching private methods, mocking the Tauri invoke bridge directly

## What to Test

- All public functions in Rust domain logic
- All error paths and edge cases
- All Tauri commands (input validation, error responses)
- All Svelte stores (state transitions, reactive updates)
- State transitions in long-running operations (loading -> loaded, loading -> error)

## What NOT to Test

- Tauri framework glue (command registration wiring)
- Trivial getters/setters with no logic
- Third-party library internals
- Type definitions and struct declarations (the compiler handles these)

## Test Isolation

- Each test must be independent — no shared mutable state
- No test order dependence
- Use test fixtures or setup functions, not module-level state
- Clean up any file system artifacts in teardown
- Use in-memory SQLite for database tests where possible

## Running Tests

```bash
# All tests (Rust + frontend)
make test

# Rust tests only
make test-rust

# Frontend unit tests (Vitest)
make test-frontend

# Frontend tests in watch mode
make test-watch

# E2E tests (requires running app)
make test-e2e
```

For the full command reference, see `.orqa/documentation/development/commands.md`.

## Related Rules

- [RULE-b49142be](RULE-b49142be) (coding-standards) — function size limits apply to test helpers too
- [RULE-e9c54567](RULE-e9c54567) (no-stubs) — test doubles are NOT stubs; they implement real traits
- [RULE-57ccb4a3](RULE-57ccb4a3) (error-ownership) — failing tests are YOUR responsibility to fix
