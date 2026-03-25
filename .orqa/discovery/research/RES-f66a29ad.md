---
id: "RES-f66a29ad"
type: research
title: "Codebase audit — test coverage, documentation alignment, and quality gaps"
description: "Comprehensive audit of OrqaStudio codebase: test coverage gaps, documentation vs implementation drift, linting configuration, and pre-commit enforcement."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
sources: []
relationships:
  - target: "EPIC-2bf6887a"
    type: "guides"
    rationale: "Research findings informed epic design"
---
## Research Questions

1. What is the current test coverage and where are the gaps?
2. Are documentation and implementation aligned?
3. Is linting/typechecking coverage aligned with coding standards?
4. What dead/outdated code exists?

---

## Findings

### 1. Test Coverage

**Rust backend**: 581 test functions across 53 of 73 files (73% file coverage).

**Tested modules (strong)**:
- All domain modules (22+ files): artifact graph, enforcement engine, governance, lessons, streaming
- 7 of 15 command modules: artifact, message, project, session, settings, setup, sidecar

**Untested command modules (8 files)** — these are Tauri IPC entry points with zero tests:
- `enforcement_commands.rs`, `governance_commands.rs`, `graph_commands.rs`
- `lesson_commands.rs`, `project_settings_commands.rs`, `search_commands.rs`, `stream_commands.rs`

**No integration tests exist** — `backend/src-tauri/tests/` directory is empty. Only inline unit tests.

**Frontend**: 194 test functions across 10 store test files. 10 of 11 stores tested (91%). `errors.svelte.ts` has no test file.

**Zero component tests** — 158 Svelte components with 0 test files. Critical gap.

**No coverage measurement infrastructure** — No `tarpaulin` (Rust) or Vitest coverage reporter configured. [RULE-9814ec3c](RULE-9814ec3c) requires 80%+ but there's no way to measure it.

**No E2E tests** — `tests/` directory referenced in Makefile but doesn't exist with Playwright tests.

### 2. Documentation vs Implementation Alignment

**Critical path mismatches**:
- `rust-modules.md` references `src-tauri/` but code is at `backend/src-tauri/` post-restructure
- `coding-standards.md` references `persistence/` directory but it's actually `repo/`
- `sub-agents.md` references `persistence/` — same issue
- `skill_injector.rs` exists in domain but missing from the module tree in `rust-modules.md`

**Architecture decisions**: All accepted decisions (AD-7121ec20, [AD-2d58941b](AD-2d58941b), [AD-09fc4e65](AD-09fc4e65), [AD-39e2fb81](AD-39e2fb81), etc.) have corresponding implementations. No orphaned decisions found.

**UI component inventory**: 10 components documented as "not yet implemented" in `component-inventory.md`. 158 actual components exist vs ~76 documented. The doc is aspirational.

**IPC/streaming documentation**: Accurate — protocol types and command definitions match code.

### 3. Linting Configuration

**Rust (clippy)**:
- Running `-D warnings` via `make lint-backend` (good)
- **No explicit `clippy::pedantic` in Cargo.toml** — [RULE-9814ec3c](RULE-9814ec3c) claims pedantic is enabled but it's not verified in config
- No `clippy.toml` or `[lints]` section in Cargo.toml
- rustfmt enforced via `make format-check`

**TypeScript/Svelte (ESLint)**:
- `typescript-eslint.recommended` enabled (good)
- `strict: true` in tsconfig.json (good)
- `svelte/no-navigation-without-resolve: off` (intentional — Tauri app, not SvelteKit)
- No custom rules beyond recommended sets

**Pre-commit hook** (`.githooks/pre-commit`):
- Conditional checks based on staged file types
- Runs: format-check, lint-backend, test-rust, typecheck, lint-frontend, test-frontend
- Schema validation for `.orqa/` artifacts
- Stub scanner for TODO/FIXME/HACK, console.log, dbg!, unwrap()
- Task dependency and epic readiness validation
- Core graph protection (disabled when `dogfood: true`)
- **Well-structured and comprehensive**

### 4. Active Epic Status

**[EPIC-9ddef7f9](EPIC-9ddef7f9) (Artifact Browser)** — ~65% complete:
- Phase 1 (Sort/Filter): 80% — toolbar missing for tree-based artifact types
- Phase 2 (References Panel): 100% complete
- Phase 3 (AI Search): 5% — UI skeleton only, doing client-side text search not AI
- [TASK-19a94ac8](TASK-19a94ac8) (README audit) never started
- [TASK-30045ad8](TASK-30045ad8) (AI search) marked done but is client-side only

**[EPIC-9a1eba3f](EPIC-9a1eba3f) (Enforcement Engine)** — ~60% complete:
- Phase 1 (CLI plugin): 90% done — 6 tasks remain (TASK-055c10f7, 185, 186, 188, 189, 190)
- Phase 2 (Rust engine): 30% done

**[EPIC-f079c196](EPIC-f079c196) (Graph-Based Knowledge Injection)** — 100% complete, verified.

---

## Summary of Gaps

| Category | Gap | Severity |
|----------|-----|----------|
| Testing | 8 command modules untested | High |
| Testing | Zero integration tests | High |
| Testing | Zero component tests (158 components) | High |
| Testing | No coverage measurement tools | Medium |
| Testing | No E2E test infrastructure | Medium |
| Docs | Path references to pre-restructure locations | High |
| Docs | `persistence/` → `repo/` in 2 docs | Medium |
| Docs | Module tree missing `skill_injector.rs` | Low |
| Docs | Component inventory outdated | Medium |
| Linting | Clippy pedantic not explicitly configured | Medium |
| Linting | No Vitest coverage thresholds | Medium |
| Epic | [EPIC-9ddef7f9](EPIC-9ddef7f9) [TASK-30045ad8](TASK-30045ad8) marked done incorrectly | Medium |
| Epic | [EPIC-9ddef7f9](EPIC-9ddef7f9) [TASK-19a94ac8](TASK-19a94ac8) never started | Low |

---

## Recommendations

1. **Code review epic** should prioritise: path fixes (quick wins), test coverage for untested commands, and coverage tooling setup
2. **Component testing** is the largest structural gap — 158 components with zero tests
3. **Clippy pedantic** should be explicitly enabled in `Cargo.toml` `[lints]` section
4. **[EPIC-9ddef7f9](EPIC-9ddef7f9)** needs honest status update — [TASK-30045ad8](TASK-30045ad8) should be reverted to `in-progress`