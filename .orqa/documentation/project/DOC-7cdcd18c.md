---
id: DOC-7cdcd18c
title: "How To: Write Rust Tests in OrqaStudio"
category: how-to
description: Practical guide for writing unit and integration tests for the OrqaStudio Rust backend.
created: 2026-03-14
updated: 2026-03-14
sort: 2
relationships: []
---

## Test Organisation

```
backend/src-tauri/src/
├── commands/
│   ├── artifact_commands.rs     # #[cfg(test)] mod tests { ... } at bottom
│   └── enforcement_commands.rs  # same pattern
├── repo/
│   ├── artifact_repo.rs         # unit tests for every public function
│   └── project_repo.rs
└── domain/
    └── artifact_reader.rs

backend/src-tauri/tests/          # integration tests (cross-module flows)
    └── artifact_lifecycle.rs
```

**Unit tests** go in a `#[cfg(test)] mod tests { ... }` block at the bottom of the
same file as the code under test. They test one module in isolation.

**Integration tests** go in `backend/src-tauri/tests/`. They test cross-module flows
and database interactions using the same in-memory SQLite pattern.

---

## Running Tests

```bash
# All Rust tests
make test-rust

# Watch mode (rerun on file save)
make test-watch-rust

# Single test by name
cargo test --manifest-path backend/src-tauri/Cargo.toml artifact_list_empty
```

---

## Writing a Unit Test

A typical repo-layer test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::repo::project_repo;

    // Helper that every test calls — creates a clean in-memory DB with a project
    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        conn
    }

    #[test]
    fn create_and_get_artifact() {
        let conn = setup();
        let artifact = create(
            &conn,
            1,
            &ArtifactType::Rule,
            ".orqa/process/rules/no-stubs.md",
            "no-stubs",
            "# No Stubs",
            Some("No stubs or placeholders"),
        )
        .expect("create artifact");

        assert_eq!(artifact.artifact_type, ArtifactType::Rule);
        assert_eq!(artifact.name, "no-stubs");

        let fetched = get(&conn, artifact.id).expect("get artifact");
        assert_eq!(fetched.name, "no-stubs");
    }

    #[test]
    fn get_nonexistent_artifact_returns_not_found() {
        let conn = setup();
        let result = get(&conn, 999);
        // Verify the error variant, not just that it's an error
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }
}
```

Key patterns:
- `setup()` helper creates a fresh in-memory DB for each test — never share state
- `.expect("create artifact")` is acceptable **in tests only** — banned in production code
- Assert on the specific error variant with `matches!`, not just `result.is_err()`

---

## Testing with SQLite (In-Memory)

Always use `crate::db::init_memory_db()` for test databases. This creates a
fully-migrated SQLite database in memory — no files, no cleanup, no cross-test
pollution:

```rust
use crate::db::init_memory_db;

let conn = init_memory_db().expect("db init");
// conn is a rusqlite::Connection
// All tables and indexes from migrations are present
// Destroyed when `conn` drops at end of test
```

Do not create real files in tests. If the code under test reads from disk, inject
the path as a parameter and pass a `tempdir()` in tests.

---

## Mocking Boundaries

**Mock at the adapter layer only.** Never mock internal functions or domain logic.

The correct boundary is a Rust trait. Define a trait for I/O, implement it for real
code, implement a mock for tests:

```rust
// Production trait in domain/
pub trait ArtifactScanner: Send + Sync {
    fn scan(&self, path: &Path) -> Result<Vec<Artifact>, OrqaError>;
}

// Real implementation
pub struct FsArtifactScanner;
impl ArtifactScanner for FsArtifactScanner {
    fn scan(&self, path: &Path) -> Result<Vec<Artifact>, OrqaError> {
        // ... real fs walking
    }
}

// Test mock
#[cfg(test)]
struct MockScanner {
    results: Vec<Artifact>,
}
#[cfg(test)]
impl ArtifactScanner for MockScanner {
    fn scan(&self, _path: &Path) -> Result<Vec<Artifact>, OrqaError> {
        Ok(self.results.clone())
    }
}
```

**What to mock:**
- File system access (use a trait, inject in tests)
- External HTTP calls
- `SystemTime::now()` if tests need deterministic timestamps

**What NOT to mock:**
- SQLite (`init_memory_db` is cheaper and more faithful)
- Your own domain logic or repo functions
- The Tauri invoke bridge directly (test commands via their inputs/outputs instead)

---

## Testing Tauri Commands

Commands are plain functions with `#[tauri::command]`. Test the logic directly —
you don't need a running Tauri instance:

```rust
#[tauri::command]
pub fn artifact_list(
    state: State<'_, AppState>,
    artifact_type: Option<String>,
) -> Result<Vec<Artifact>, OrqaError> {
    let conn = state.db.lock()?;
    artifact_repo::list(&conn, state.project_id, artifact_type.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;

    #[test]
    fn artifact_list_empty_returns_empty_vec() {
        // Set up a fresh db with a project
        let conn = init_memory_db().expect("db");
        // Test the repo function the command calls — same code path
        let result = crate::repo::artifact_repo::list(&conn, 1, None).expect("list");
        assert!(result.is_empty());
    }
}
```

Test input validation paths explicitly — pass invalid inputs and assert the correct
`OrqaError` variant comes back.

---

## Coverage Requirements

Every module must maintain **80%+ line coverage**. Run coverage locally with:

```bash
cargo tarpaulin --manifest-path backend/src-tauri/Cargo.toml --fail-under 80
```

Coverage failures block CI. If a function is genuinely untestable (e.g., Tauri
framework wiring), document why in a comment — don't silently omit tests.

---
