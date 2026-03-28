---
id: KNOW-73490bde
type: knowledge
name: Tauri v2 Patterns
summary: "Tauri v2 Patterns. All frontend-backend communication uses `#[tauri::command]` functions. No HTTP servers, no WebSockets, no shared memory."
status: active
plugin: "@orqastudio/plugin-tauri"
relationships:
  - target: DOC-13c73ecf
    type: synchronised-with
  - target: AGENT-65b56a0b
    type: employed-by
---

# Tauri v2 Patterns

## IPC Boundary

All frontend-backend communication uses `#[tauri::command]` functions. No HTTP servers, no WebSockets, no shared memory.

```rust
#[tauri::command]
pub fn my_command(state: State<'_, AppState>) -> Result<MyResponse, OrqaError> {
    // ...
}
```text

## Error Handling

Use `thiserror` for error types. All commands return `Result<T, OrqaError>`. No `unwrap()` or `expect()` in production code.

```rust
#[derive(Debug, thiserror::Error)]
pub enum OrqaError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```text

## State Management

`AppState` behind `Mutex` in Tauri's managed state. Lazy initialization pattern.

## Domain-Driven

Domain logic in `src/domain/`. Commands in `src/commands/`. Repositories in `src/repo/`. Commands are thin delegation layers — domain functions do the work.

## Composability

Functions under 50 lines. Pure domain functions that take inputs and return outputs. Side effects isolated at boundaries (commands, repos).
