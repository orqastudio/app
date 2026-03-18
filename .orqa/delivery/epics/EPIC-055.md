---
id: EPIC-055
title: Codebase Quality and Composability Alignment
description: |
  Address critical code quality issues found by RES-043: eliminate stream_commands.rs
  duplication, add frontend and search test coverage, fix error handling gaps, and
  audit orphaned backend commands.
status: completed
priority: P1
created: 2026-03-12
updated: 2026-03-12
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: RES-043
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-043
  - target: MS-001
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-243
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-244
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-245
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-246
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-247
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-248
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-249
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-250
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-345
    type: delivered-by
    rationale: Epic contains this task
  - target: RES-043
    type: guided-by
---
## Context

[RES-043](RES-043) audited the entire codebase (23k lines Rust, 15k lines Svelte/TS, 4k lines
sidecar) against composability principles, error handling standards, IPC boundary discipline,
test coverage, and security. The architecture is fundamentally sound, but several quality
issues need addressing.

**Key findings:**

- `stream_commands.rs` is a 2,497-line god-file that duplicates ~1,200 lines already extracted
  into domain modules — the extraction was started but never completed
- Zero frontend tests exist — no `.test.ts` files anywhere in `ui/`
- Search module (`embedder.rs`, `store.rs`) has no test coverage despite being 725 lines of
  complex ONNX + DuckDB logic
- `db.rs` migration code silently swallows errors with `.unwrap_or(false)`
- 29 backend commands have no frontend callers — some are sidecar tool-loop commands, others
  may be orphaned
- Search errors lose type information via `.map_err(|e| e.to_string())`

**What's already good (no action needed):**

- Frontend: perfect Svelte 5 compliance, zero `any` types, near-perfect component purity
- IPC boundary clean — all frontend `invoke()` calls resolve to registered commands
- Type consistency across Rust/TypeScript boundary (StreamEvent 14 variants match)
- Security: path traversal protection, tool approval gates, no hardcoded secrets
- Domain modules well-structured with good test coverage (541 Rust tests)

---

## Implementation Design

Work is organised by impact and dependency order.

### Phase 1: Critical — Eliminate God-File

Complete the domain extraction that was started. `stream_commands.rs` should only contain
the 3 `#[tauri::command]` functions (~150-200 lines), delegating all logic to:
- `domain/stream_loop.rs` — stream processing
- `domain/tool_executor.rs` — tool execution
- `domain/system_prompt.rs` — system prompt building

### Phase 2: Test Coverage

Add tests where gaps are highest-risk:
- Frontend store tests (all 10 stores)
- Search module tests (`embedder.rs`, `store.rs`)
- Utility function tests

### Phase 3: Error Handling Hardening

- Add `From<duckdb::Error>` to `OrqaError`
- Fix `db.rs` migration `.unwrap_or(false)` patterns
- Standardize `lib.rs` tracker error handling

### Phase 4: Cleanup

- Audit 29 backend-only commands for orphaned code
- Consider `AppState` decomposition into sub-structs

---

## Tasks

### Phase 1: God-File Elimination

| ID | Title |
|----|-------|
| [TASK-243](TASK-243) | Complete stream_commands.rs domain extraction |

### Phase 2: Test Coverage

| ID | Title |
|----|-------|
| [TASK-244](TASK-244) | Add frontend store unit tests (all 10 stores) |
| [TASK-245](TASK-245) | Add search module tests (embedder.rs + store.rs) |

### Phase 3: Error Handling

| ID | Title |
|----|-------|
| [TASK-246](TASK-246) | Add From<duckdb::Error> to OrqaError and fix search error propagation |
| [TASK-247](TASK-247) | Fix db.rs migration error handling (.unwrap_or patterns) |

### Phase 4: Cleanup

| ID | Title |
|----|-------|
| [TASK-248](TASK-248) | Audit backend-only commands — identify and remove orphaned code |
| [TASK-249](TASK-249) | Decompose AppState into grouped sub-structs |

### Phase 5: Dev Tooling

| ID | Title |
|----|-------|
| [TASK-250](TASK-250) | Unified dev logging — stream all info/error logs to OrqaDev Compose UI |

---

## Out of Scope

- E2E Playwright tests (separate effort, empty `tests/` directory exists)
- Moving `AppLayout.svelte` invoke to a store (minor, not worth a task)
- Repo boilerplate macro generation (low priority optimisation)
- Sidecar refactoring (already clean)
