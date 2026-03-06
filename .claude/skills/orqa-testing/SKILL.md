---
name: orqa-testing
description: |
  Orqa Studio testing patterns: make test commands, Rust unit/integration tests,
  Vitest frontend tests, mock boundaries, and test file organization.
  Use when: Writing tests, fixing test failures, setting up test infrastructure,
  or verifying test coverage.
version: 1.0.0
tags: [orqa, testing, vitest, cargo-test, playwright, tdd, mocking]
user-invocable: true
---

# Orqa Testing Patterns

Orqa Studio uses `make` targets for all test commands. Tests are organized by layer: Rust unit tests inline with source, Rust integration tests in `src-tauri/tests/`, frontend unit tests alongside components, and E2E tests in `tests/`.

## Test Commands (MANDATORY)

Always use `make` targets. Never use raw `cargo test` or `npm run test`.

```bash
make test            # All tests (Rust + frontend)
make test-rust       # Rust unit + integration tests
make test-frontend   # Vitest (frontend unit tests)
make test-watch      # Vitest in watch mode
make test-e2e        # Playwright E2E tests (requires running app)
make check           # ALL checks: fmt + clippy + tests + svelte-check + lint
```

## Test File Organization

```
src-tauri/
  src/
    domain/
      project.rs          # Domain logic
      project/tests.rs    # OR inline #[cfg(test)] mod tests
    commands/
      project_commands.rs  # Command handlers
  tests/
    integration_test.rs    # Cross-module integration tests

ui/
  lib/
    stores/
      session.svelte.ts       # Store
      session.test.ts          # Store tests (same directory)
    components/
      layout/
        AppLayout.svelte       # Component
        AppLayout.test.ts      # Component tests (same directory)

tests/                         # E2E tests (Playwright)
  e2e/
    session-flow.spec.ts
```

## Rust Test Patterns

### Unit Test (Inline)

```rust
// src-tauri/src/domain/artifact.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_frontmatter_extracts_yaml() {
        let content = "---\ntitle: Test\ntags: [a, b]\n---\n\nBody content";
        let (fm, body): (DocFrontmatter, String) = parse_frontmatter(content);
        assert_eq!(fm.title, Some("Test".to_string()));
        assert_eq!(fm.tags, vec!["a", "b"]);
        assert!(body.contains("Body content"));
    }

    #[test]
    fn parse_frontmatter_handles_missing_yaml() {
        let content = "No frontmatter here";
        let (fm, body): (DocFrontmatter, String) = parse_frontmatter(content);
        assert!(fm.title.is_none());
        assert_eq!(body, content);
    }
}
```

**Key rules:**
- Use `#[cfg(test)]` module at the bottom of the source file
- Import from `super::*` to access the module's private internals
- Test both happy path and error/edge cases
- Use descriptive test names: `verb_condition_expected_result`

### Integration Test

```rust
// src-tauri/tests/session_integration.rs
use orqa_studio::domain::project::Project;
use orqa_studio::repo::{project_repo, session_repo};

fn setup_db() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    // Run migrations
    conn
}

#[test]
fn create_session_with_valid_project() {
    let conn = setup_db();
    let project = project_repo::create(&conn, "Test", "/tmp/test", None).unwrap();
    let session = session_repo::create(&conn, project.id, "Test Session").unwrap();
    assert_eq!(session.project_id, project.id);
    assert_eq!(session.title, "Test Session");
}
```

**Key rules:**
- Use in-memory SQLite for database tests
- Set up fresh state per test (no shared mutable state)
- Test cross-module interactions (repo + domain)

## Frontend Test Patterns

### Store Test

```typescript
// ui/lib/stores/session.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock the invoke wrapper
vi.mock('$lib/ipc/invoke', () => ({
    invoke: vi.fn(),
    createStreamChannel: vi.fn(),
}));

import { invoke } from '$lib/ipc/invoke';

describe('SessionStore', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    it('loads sessions from backend', async () => {
        const mockSessions = [{ id: 1, title: 'Test' }];
        vi.mocked(invoke).mockResolvedValue(mockSessions);

        // Test store behavior
    });

    it('handles load error gracefully', async () => {
        vi.mocked(invoke).mockRejectedValue(new Error('DB error'));

        // Verify error state is set
    });
});
```

**Key rules:**
- Mock `invoke()` from `$lib/ipc/invoke` — never call the real backend
- Test loading, success, and error states
- Use `vi.clearAllMocks()` in `beforeEach`
- Test stores independently from components

### Component Test

```typescript
// ui/lib/components/layout/StatusBar.test.ts
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import StatusBar from './StatusBar.svelte';

describe('StatusBar', () => {
    it('renders connection status', () => {
        const { getByText } = render(StatusBar, {
            props: { connected: true, model: 'claude-sonnet-4-5-20250514' }
        });
        expect(getByText('Connected')).toBeTruthy();
    });
});
```

## Mock Boundaries

**Mock ONLY at the adapter/boundary layer:**

| OK to Mock | NOT OK to Mock |
|-----------|----------------|
| `invoke()` — the IPC boundary | Domain logic functions |
| File system traits (in Rust) | Internal Rust functions |
| External API responses | Store methods (test stores directly) |
| `SidecarProcess` (child process) | Tauri framework internals |

```rust
// CORRECT: trait-based mock for repository
pub trait SessionRepository {
    fn create(&self, project_id: i64, title: &str) -> Result<Session, OrqaError>;
    fn get(&self, id: i64) -> Result<Session, OrqaError>;
}

// Test uses in-memory implementation
struct InMemorySessionRepo { /* ... */ }
impl SessionRepository for InMemorySessionRepo { /* ... */ }
```

## Coverage Requirements

- **Rust backend**: >=80% per module
- **Frontend**: All stores must have tests
- **E2E**: All primary user flows must have Playwright coverage

## Anti-Patterns

### Testing implementation details
```typescript
// WRONG: testing internal state shape
expect(store._internal_map.size).toBe(3);

// CORRECT: testing observable behavior
expect(store.items.length).toBe(3);
```

### Shared mutable state between tests
```rust
// WRONG: static mut or lazy_static shared across tests
static mut DB: Option<Connection> = None;

// CORRECT: fresh setup per test
fn setup() -> Connection { Connection::open_in_memory().unwrap() }
```

### Skipping error path tests
```typescript
// WRONG: only testing the happy path
it('loads data', async () => { /* ... */ });

// CORRECT: also test the error path
it('handles load failure', async () => {
    vi.mocked(invoke).mockRejectedValue(new Error('fail'));
    // Verify error state
});
```

## Key Files

| File | Purpose |
|------|---------|
| `Makefile` | All test targets (test, test-rust, test-frontend, etc.) |
| `src-tauri/src/` | Rust unit tests (inline `#[cfg(test)]` modules) |
| `src-tauri/tests/` | Rust integration tests |
| `ui/**/*.test.ts` | Frontend unit tests (Vitest) |
| `tests/` | E2E tests (Playwright) |
| `vitest.config.ts` | Vitest configuration |
| `docs/development/coding-standards.md` | Coverage requirements |
