---
id: SKILL-009
title: Orqa Domain Services
description: |
  OrqaStudio domain service patterns: how Rust backend services are structured,
  composed, tested, and wired to Tauri commands. Covers service anatomy, command
  delegation, error propagation, and the boundary between domain logic and framework code.
  Use when: Creating new domain services, refactoring backend logic out of commands,
  adding business logic to the Rust backend, or reviewing service architecture.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: project
category: domain
file-patterns:
  - "backend/src-tauri/src/domain/**"
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Service anatomy and command delegation patterns enforce clear separation between business logic and framework wiring
  - target: TASK-007
    type: informs
  - target: TASK-008
    type: informs
  - target: TASK-024
    type: informs
  - target: TASK-025
    type: informs
  - target: TASK-073
    type: informs
  - target: TASK-176
    type: informs
  - target: TASK-188
    type: informs
  - target: TASK-189
    type: informs
  - target: TASK-415
    type: informs
  - target: TASK-416
    type: informs
  - target: PILLAR-001
    type: informs
---


Domain services live in `backend/src-tauri/src/domain/`. They contain the business logic of OrqaStudio and are the heart of the thick-backend architecture [AD-001](AD-001). Commands in `backend/src-tauri/src/commands/` are thin wrappers that delegate to domain services and repositories.

## Service Anatomy

A domain service is a Rust module that:

1. Contains **pure functions** or **structs with methods** that operate on domain types
2. Returns `Result<T, OrqaError>` for fallible operations
3. Has **no Tauri dependencies** — no `State<'_, AppState>`, no `Channel<T>`, no `#[tauri::command]`
4. Depends only on `crate::domain::*`, `crate::error`, and standard library / crate types
5. Is independently testable without a database, sidecar, or Tauri runtime

### Three Service Shapes

**Shape 1: Stateless function module** — Free functions, no struct. Best for scanning, parsing, one-shot operations.

```rust
// backend/src-tauri/src/domain/governance_scanner.rs
pub fn scan_governance(project_path: &Path) -> Result<GovernanceScanResult, OrqaError> {
    if !project_path.exists() || !project_path.is_dir() {
        return Err(OrqaError::Validation(format!(
            "project path does not exist or is not a directory: {}",
            project_path.display()
        )));
    }
    let areas = scan_claude_areas(project_path);
    let covered = areas.iter().filter(|a| a.covered).count();
    let coverage_ratio = covered as f64 / TOTAL_AREAS as f64;
    Ok(GovernanceScanResult { areas, coverage_ratio })
}
```

**Shape 2: Stateful struct** — Loads data once, provides multiple query methods. Best when initialization is expensive (regex compilation, file parsing).

```rust
// backend/src-tauri/src/domain/enforcement_engine.rs
pub struct EnforcementEngine {
    rules: Vec<EnforcementRule>,
    compiled: Vec<CompiledEntry>,  // Pre-compiled regexes
}

impl EnforcementEngine {
    pub fn load(rules_dir: &Path) -> Result<Self, OrqaError> { /* ... */ }
    pub fn evaluate_file(&self, file_path: &str, new_text: &str) -> Vec<Verdict> { /* ... */ }
    pub fn evaluate_bash(&self, command: &str) -> Vec<Verdict> { /* ... */ }
    pub fn scan(&self, project_path: &Path) -> Result<Vec<ScanFinding>, OrqaError> { /* ... */ }
}
```

**Shape 3: Data types with associated functions** — Types plus parsing/rendering functions. No struct holding state.

```rust
// backend/src-tauri/src/domain/lessons.rs
pub struct Lesson { /* fields */ }
pub struct NewLesson { /* fields */ }

pub fn parse_lesson(content: &str, file_path: &str) -> Result<Lesson, String> { /* ... */ }
pub fn render_lesson(lesson: &Lesson) -> String { /* ... */ }
```

### What Makes These Good Domain Services

- **No `tauri` import** — no framework dependency
- **No `State<'_, AppState>`** — database access is not the service's concern
- **Result-based error handling** — every fallible function returns `Result`
- **Testable in isolation** — tests use `tempfile::tempdir()` or string inputs

## Command-to-Service Delegation

Commands are **thin wrappers**: extract parameters, acquire resources, delegate, return.

### Good: Thin command delegates to domain

```rust
// backend/src-tauri/src/commands/governance_commands.rs
#[tauri::command]
pub fn governance_scan(
    project_id: i64,
    state: State<'_, AppState>,
) -> Result<GovernanceScanResult, OrqaError> {
    let project_path = get_project_path(project_id, &state)?;  // resource acquisition
    scan_governance(Path::new(&project_path))                    // delegation
}
```

### Good: Command delegates to repo

```rust
// backend/src-tauri/src/commands/artifact_commands.rs
#[tauri::command]
pub fn artifact_list(
    project_id: i64,
    artifact_type: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let type_filter = artifact_type.as_deref().map(parse_artifact_type).transpose()?;
    let conn = state.db.lock().map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    artifact_repo::list(&conn, project_id, type_filter.as_ref())
}
```

### Architectural Debt: Thick commands

`stream_commands.rs` (~1000+ lines) contains business logic that should be domain services:

| Current location in stream_commands.rs | Should be |
|---------------------------------------|-----------|
| `build_system_prompt()` | `domain::system_prompt::build()` |
| `load_context_summary()` | `domain::context::load_messages()` |
| `translate_response()` | `domain::stream_translator::translate()` |
| `read_rules()`, `list_skill_catalog()` | `domain::governance_scanner` or new module |

**Symptoms of a thick command:** file exceeds 200 lines, private helpers don't reference `State` or `Channel`, business logic mixed with framework plumbing.

## Private Helpers Pattern

Domain services use private helper functions to stay within function size limits:

```rust
// backend/src-tauri/src/domain/enforcement_engine.rs

/// Public API
pub fn evaluate_file(&self, file_path: &str, new_text: &str) -> Vec<Verdict> { /* ... */ }

/// Private helper — used by evaluate_file and scan
fn prose_excerpt(prose: &str) -> String {
    let trimmed = prose.trim();
    if trimmed.len() <= 200 { trimmed.to_string() }
    else { format!("{}...", &trimmed[..200]) }
}
```

Private helpers are: not `pub`, pure, small (<20 lines), named for their purpose.

## When to Create a New Service

Create a new module in `backend/src-tauri/src/domain/` when:

1. A command file exceeds 200 lines — extract the non-framework logic
2. Logic is reused across multiple commands — shared logic belongs in domain
3. You need to test business logic independently — if testing requires `State`, extract it
4. A new domain concept emerges — new entity types, validation rules, file format parsing

### Checklist for a New Domain Module

- [ ] File lives in `backend/src-tauri/src/domain/`
- [ ] Added to `backend/src-tauri/src/domain/mod.rs`
- [ ] No `tauri` imports
- [ ] Public functions return `Result<T, OrqaError>` or are infallible
- [ ] Domain types derive `Debug, Clone, Serialize, Deserialize`
- [ ] Private helpers under 20 lines; public functions under 50 lines
- [ ] `#[cfg(test)] mod tests` block with unit tests
- [ ] Tests use `tempfile::tempdir()` for filesystem tests

## Service-to-Service Composition

Domain services compose via return types. One service produces a value; another consumes it. No shared mutable state.

**Dependency direction:**
```
commands/ (thin wrappers, Tauri deps)
    → domain/ (pure business logic, no Tauri deps)
    → repo/ (database access, rusqlite deps)
    → error.rs (shared error type)
```

Domain services may depend on other domain modules. They must NOT depend on `commands/` or the Tauri framework.

## Testing Domain Services

### Filesystem Tests with tempdir

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_empty_rules_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let engine = EnforcementEngine::load(dir.path()).expect("should load");
        assert!(engine.rules().is_empty());
    }
}
```

### Pure Function Tests (No Filesystem)

```rust
#[test]
fn parse_valid_lesson() {
    let lesson = parse_lesson(SAMPLE_CONTENT, ".orqa/process/lessons/IMPL-001.md").expect("should parse");
    assert_eq!(lesson.id, "IMPL-001");
}

#[test]
fn render_round_trip() {
    let lesson = parse_lesson(SAMPLE, "path").expect("parse");
    let rendered = render_lesson(&lesson);
    let reparsed = parse_lesson(&rendered, "path").expect("re-parse");
    assert_eq!(reparsed.id, lesson.id);
}
```

### Serialization Roundtrip Tests

```rust
#[test]
fn stream_event_roundtrip() {
    let events = vec![
        StreamEvent::StreamStart { message_id: 1, resolved_model: None },
        StreamEvent::TextDelta { content: "hi".to_string() },
    ];
    for event in &events {
        let json = serde_json::to_string(event).expect("serialize");
        let deserialized: StreamEvent = serde_json::from_str(&json).expect("deserialize");
        let re_json = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(json, re_json);
    }
}
```

## Key Files

| File | Purpose |
|------|---------|
| `backend/src-tauri/src/domain/mod.rs` | Module declarations |
| `backend/src-tauri/src/domain/enforcement_engine.rs` | Stateful service: rule engine |
| `backend/src-tauri/src/domain/governance_scanner.rs` | Stateless service: filesystem scanning |
| `backend/src-tauri/src/domain/project_scanner.rs` | Stateless service: project detection |
| `backend/src-tauri/src/domain/lessons.rs` | Data type + parser/renderer |
| `backend/src-tauri/src/domain/artifact.rs` | Data types + frontmatter parsing |
| `backend/src-tauri/src/domain/provider_event.rs` | IPC event types (serde-tagged enum) |
| `backend/src-tauri/src/domain/enforcement.rs` | Domain types for enforcement |
| `backend/src-tauri/src/domain/process_state.rs` | Process compliance tracking |
| `backend/src-tauri/src/error.rs` | `OrqaError` — canonical error type |
| `backend/src-tauri/src/commands/governance_commands.rs` | Example thin command delegation |
| `backend/src-tauri/src/commands/stream_commands.rs` | Counter-example: thick command (debt) |

## Related Skills

- **composability** — the overarching philosophy this skill implements
- **orqa-ipc-patterns** — the full IPC request chain from frontend to backend
- **orqa-error-composition** — how errors flow through domain services
- **orqa-repository-pattern** — the persistence layer domain services delegate to
