---
scope: project
enforcement:
  - event: file
    action: warn
    conditions:
      - field: file_path
        pattern: src-tauri/.*test.*\.rs$
      - field: new_text
        pattern: (mock!|#\[mockall|MockAll|mockall::)
---

# Testing Standards (NON-NEGOTIABLE)

**Source of Truth:** This file defines testing patterns. `docs/development/coding-standards.md` defines coverage requirements.

## Test Organisation

| Location | Type | Purpose |
|----------|------|---------|
| `src-tauri/src/**/tests/` | Rust unit tests | Single-module logic, domain functions, utilities |
| `src-tauri/tests/` | Rust integration tests | Cross-module flows, database interactions, file system operations |
| `ui/**/*.test.ts` | Frontend unit tests | Svelte components, stores, utility functions (Vitest) |
| `tests/` (root) | E2E tests | Playwright browser tests against the running Tauri app |

## Coverage Requirements

- **Rust backend**: >=80% per module (`cargo tarpaulin --fail-under 80` or equivalent coverage tool)
- **Frontend**: Run `npm run test` — all tests must pass (Vitest)
- **E2E**: All user flows in `docs/ui/` must have Playwright coverage

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
# Rust tests
cargo test

# Rust tests with coverage
cargo tarpaulin --out Html

# Frontend unit tests
npm run test

# Frontend tests in watch mode
npm run test -- --watch

# E2E tests (requires running app)
npx playwright test
```

## Related Rules

- `coding-standards.md` — function size limits apply to test helpers too
- `no-stubs.md` — test doubles are NOT stubs; they implement real traits
- `error-ownership.md` — failing tests are YOUR responsibility to fix
