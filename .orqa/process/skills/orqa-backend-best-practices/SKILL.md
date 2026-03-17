---
id: SKILL-043
title: OrqaStudio Backend Best Practices
description: |
  Umbrella skill for all backend implementation work. Establishes composability,
  coding standards, and error handling as always-in-mind principles, then
  references deeper skills for Rust, Tauri, and persistence specifics.
  Use when: Any agent is about to write or modify backend code (backend/src-tauri/, sidecar/).
status: active
created: 2026-03-11
updated: 2026-03-11
layer: project
category: domain
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Establishes composability and error handling principles that make backend code consistent and reviewable
  - target: TASK-176
    type: informs
  - target: TASK-188
    type: informs
  - target: TASK-189
    type: informs
  - target: TASK-190
    type: informs
  - target: PILLAR-001
    type: informs
---

This skill ensures every backend agent has the right mental model before writing code. It does not duplicate content from deeper skills — it establishes principles and points to the right references.

## Always In Mind

### Composability ([SKILL-008](SKILL-008))

Every domain service, command handler, and utility follows OrqaStudio's composability philosophy:

- **Small enough to understand in isolation** — domain functions 20-30 lines, commands 30-50 lines
- **Pure enough to test without the world** — domain logic takes inputs and returns `Result<T, E>`, no side effects
- **Typed enough to compose safely** — `thiserror` for typed errors, `From` impls for `?` propagation
- **Swappable enough to replace without cascading changes** — trait boundaries at integration points

Load the full `composability` skill for the complete philosophy and anti-patterns.

### Coding Standards (DOC-021)

Read `.orqa/documentation/development/coding-standards.md` before writing any code. Key backend standards:

- **Formatting** — `rustfmt` on all code, no exceptions
- **Linting** — `clippy` with pedantic and nursery lint groups, zero warnings
- **Error handling** — `thiserror` for all error types, every function returns `Result<T, E>`, **no `unwrap()` / `expect()` / `panic!()` in production**
- **Types** — IPC types derive `Serialize`, `Deserialize`, `Debug`, `Clone`. Domain types immutable by default.
- **Module organisation** — one module per domain concept, public API via `mod.rs`
- **No TODO comments** — use task artifacts instead

### Error Propagation ([RULE-002](RULE-002))

Error handling is a foundational principle — not optional:

```rust
// CORRECT: typed error with From impl for automatic ? propagation
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

// FORBIDDEN: unwrap, expect, panic in production code
let value = map.get("key").unwrap(); // NEVER
```

### IPC Boundary ([AD-004](AD-004))

Tauri `invoke()` is the ONLY frontend-backend interface. Every backend capability the frontend needs MUST have a `#[tauri::command]` function registered in the app builder.

| Layer | Responsibility |
|-------|---------------|
| `backend/src-tauri/src/commands/` | Thin command handlers — validate, delegate to domain, serialize response |
| `backend/src-tauri/src/domain/` | Business logic — pure functions, no framework dependencies |
| `backend/src-tauri/src/repo/` | Data access — repository pattern, SQLite via rusqlite |

Command handlers should be thin — delegate to domain services, don't contain business logic.

### End-to-End Completeness ([RULE-010](RULE-010))

Every feature requires all 4 layers in the same commit:

1. **Rust command** — `#[tauri::command]` handler
2. **IPC types** — Serializable Rust structs + matching TypeScript interfaces
3. **Svelte component** — UI that calls the command
4. **Store binding** — Reactive store managing state

A backend change without the corresponding frontend wiring is incomplete.

## Deeper Skills (Load When Needed)

| Skill | When to load |
|-------|-------------|
| `rust-async-patterns` | Async/await, Tokio, concurrent patterns |
| `tauri-v2` | Tauri commands, plugins, capabilities, Channel<T> streaming |
| `orqa-ipc-patterns` | IPC contracts, command registration, type serialisation |
| `orqa-domain-services` | Service anatomy, composition, domain boundaries |
| `orqa-error-composition` | OrqaError flow, From impls, error propagation patterns |
| `orqa-repository-pattern` | SQLite repos, migrations, query patterns |
| `orqa-streaming` | Agent SDK → sidecar → Rust → Svelte pipeline |

## Verification

Before committing backend code:

```bash
make format-check    # rustfmt (formatting)
make lint-backend    # clippy pedantic (linting)
make test-rust       # cargo test (unit + integration tests)
```

All three must pass. See [RULE-012](RULE-012) — all errors are your responsibility.
