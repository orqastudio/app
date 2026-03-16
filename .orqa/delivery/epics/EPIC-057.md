---
id: EPIC-057
title: Codebase Audit and Architecture Documentation
description: "Thorough code review ensuring artifact accuracy, documenting undiscovered implementation patterns, removing dead/outdated code, assessing test coverage, aligning linting with coding standards, and producing complete architecture documentation for the target core application."
status: completed
priority: P1
created: 2026-03-12
updated: 2026-03-12
deadline: null
horizon: null
scoring: null
relationships:
  - target: RES-047
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-047
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-258
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-259
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-260
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-261
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-262
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-263
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-264
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-265
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-266
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-267
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-268
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-269
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-270
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-347
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
  - target: RES-047
    type: informs
---
## Context

[RES-047](RES-047) identified significant gaps: 8 untested command modules, zero integration tests, zero component tests, documentation path drift post-restructure, clippy pedantic not explicitly configured, no coverage measurement tooling, and several active epic tasks marked done incorrectly.

This epic fixes the foundation before building more features. It's split into 4 phases: documentation fixes (quick wins), test infrastructure, architecture documentation, and governance cleanup.

## Implementation Design

### Phase 1: Documentation Path Fixes + Dead Code Cleanup

Quick wins. Fix all post-restructure path references, update module trees, remove dead code.

### Phase 2: Test Infrastructure + Coverage Tooling

Set up coverage measurement (`cargo tarpaulin`, Vitest coverage), add tests for untested command modules, create component test infrastructure, add `errors.svelte.ts` store test.

### Phase 3: Architecture Documentation

Complete end-to-end documentation of the target core application: artifact system, knowledge graph, prompt injection pipeline, rule enforcement, learning loop, plugin architecture, component library/SDK extraction, git integration points.

### Phase 4: Governance Cleanup

Fix incorrect task statuses on active epics, remove dead `scope` field references from agent-related docs, tighten [RULE-001](RULE-001) orchestrator exception list re: content creation vs coordination.

## Tasks

| ID | Title | Phase | Description |
|----|-------|-------|-------------|
| [TASK-258](TASK-258) | Fix post-restructure path references in docs | 1 | Update `src-tauri/` → `backend/src-tauri/`, `persistence/` → `repo/` across all `.orqa/` docs |
| [TASK-259](TASK-259) | Update rust-modules.md module tree | 1 | Add `skill_injector.rs`, fix tree structure to match current codebase |
| [TASK-260](TASK-260) | Enable clippy pedantic in Cargo.toml | 1 | Add `[lints.clippy]` section with pedantic enabled, fix resulting warnings |
| [TASK-261](TASK-261) | Set up Rust coverage tooling | 2 | Configure `cargo tarpaulin` or `llvm-cov`, add `make coverage-rust` target |
| [TASK-262](TASK-262) | Set up frontend coverage tooling | 2 | Configure Vitest coverage reporter with 80% threshold, add `make coverage-frontend` |
| [TASK-263](TASK-263) | Add tests for untested command modules | 2 | Write tests for the 8 untested command files |
| [TASK-264](TASK-264) | Create component test infrastructure | 2 | Set up Svelte component testing with `@testing-library/svelte`, write template test |
| [TASK-265](TASK-265) | Add `errors.svelte.ts` store test | 2 | Write test file for the one untested store |
| [TASK-266](TASK-266) | Write core architecture documentation | 3 | End-to-end map of artifact system, knowledge graph, prompt injection, enforcement, learning loop |
| [TASK-267](TASK-267) | Document plugin architecture and SDK extraction plan | 3 | Component library extraction, view registration API, theme tokens, plugin distribution |
| [TASK-268](TASK-268) | Fix [EPIC-005](EPIC-005) task statuses | 4 | Revert [TASK-170](TASK-170) to in-progress, update [TASK-164](TASK-164) |
| [TASK-269](TASK-269) | Tighten [RULE-001](RULE-001) orchestrator content boundary | 4 | Clarify `.orqa/delivery/` exception: structure = orchestrator, content = Writer |
| [TASK-270](TASK-270) | Resolve [AD-032](AD-032) SQLite scoping violation | 4 | Governance tables in SQLite violate [AD-032](AD-032). Decide: ephemeral cache or file-based. |

## Out of Scope

- Implementing E2E Playwright tests (separate epic)
- Achieving 80% coverage (this epic sets up measurement + adds critical missing tests)
- Rewriting the component inventory doc (covered by architecture documentation task)
