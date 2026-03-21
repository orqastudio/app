---
id: "EPIC-4726cb3b"
title: "Codebase Audit and Architecture Documentation"
description: "Thorough code review ensuring artifact accuracy, documenting undiscovered implementation patterns, removing dead/outdated code, assessing test coverage, aligning linting with coding standards, and producing complete architecture documentation for the target core application."
status: "completed"
priority: "P1"
created: "2026-03-12"
updated: "2026-03-12"
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
relationships:
  - target: "RES-9bcc7279"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-9bcc7279"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-caa1dd3c"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e3e503a9"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-efaf25d7"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-54fae8bf"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-0baf0584"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-9ca53d45"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e0cbdfe2"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e5775999"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-76e79dba"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-d9d85326"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-dce77e0b"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-7e7d1e02"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-c75be77c"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-91bc09f9"
    type: "delivered-by"
    rationale: "Epic contains this task"
---
## Context

[RES-9bcc7279](RES-9bcc7279) identified significant gaps: 8 untested command modules, zero integration tests, zero component tests, documentation path drift post-restructure, clippy pedantic not explicitly configured, no coverage measurement tooling, and several active epic tasks marked done incorrectly.

This epic fixes the foundation before building more features. It's split into 4 phases: documentation fixes (quick wins), test infrastructure, architecture documentation, and governance cleanup.

## Implementation Design

### Phase 1: Documentation Path Fixes + Dead Code Cleanup

Quick wins. Fix all post-restructure path references, update module trees, remove dead code.

### Phase 2: Test Infrastructure + Coverage Tooling

Set up coverage measurement (`cargo tarpaulin`, Vitest coverage), add tests for untested command modules, create component test infrastructure, add `errors.svelte.ts` store test.

### Phase 3: Architecture Documentation

Complete end-to-end documentation of the target core application: artifact system, knowledge graph, prompt injection pipeline, rule enforcement, learning loop, plugin architecture, component library/SDK extraction, git integration points.

### Phase 4: Governance Cleanup

Fix incorrect task statuses on active epics, remove dead `scope` field references from agent-related docs, tighten [RULE-532100d9](RULE-532100d9) orchestrator exception list re: content creation vs coordination.

## Tasks

| ID | Title | Phase | Description |
|----|-------|-------|-------------|
| [TASK-caa1dd3c](TASK-caa1dd3c) | Fix post-restructure path references in docs | 1 | Update `src-tauri/` → `backend/src-tauri/`, `persistence/` → `repo/` across all `.orqa/` docs |
| [TASK-e3e503a9](TASK-e3e503a9) | Update rust-modules.md module tree | 1 | Add `skill_injector.rs`, fix tree structure to match current codebase |
| [TASK-efaf25d7](TASK-efaf25d7) | Enable clippy pedantic in Cargo.toml | 1 | Add `[lints.clippy]` section with pedantic enabled, fix resulting warnings |
| [TASK-54fae8bf](TASK-54fae8bf) | Set up Rust coverage tooling | 2 | Configure `cargo tarpaulin` or `llvm-cov`, add `make coverage-rust` target |
| [TASK-0baf0584](TASK-0baf0584) | Set up frontend coverage tooling | 2 | Configure Vitest coverage reporter with 80% threshold, add `make coverage-frontend` |
| [TASK-9ca53d45](TASK-9ca53d45) | Add tests for untested command modules | 2 | Write tests for the 8 untested command files |
| [TASK-e0cbdfe2](TASK-e0cbdfe2) | Create component test infrastructure | 2 | Set up Svelte component testing with `@testing-library/svelte`, write template test |
| [TASK-e5775999](TASK-e5775999) | Add `errors.svelte.ts` store test | 2 | Write test file for the one untested store |
| [TASK-76e79dba](TASK-76e79dba) | Write core architecture documentation | 3 | End-to-end map of artifact system, knowledge graph, prompt injection, enforcement, learning loop |
| [TASK-d9d85326](TASK-d9d85326) | Document plugin architecture and SDK extraction plan | 3 | Component library extraction, view registration API, theme tokens, plugin distribution |
| [TASK-dce77e0b](TASK-dce77e0b) | Fix [EPIC-6787bb93](EPIC-6787bb93) task statuses | 4 | Revert [TASK-21b461ea](TASK-21b461ea) to in-progress, update [TASK-db9be55f](TASK-db9be55f) |
| [TASK-7e7d1e02](TASK-7e7d1e02) | Tighten [RULE-532100d9](RULE-532100d9) orchestrator content boundary | 4 | Clarify `.orqa/delivery/` exception: structure = orchestrator, content = Writer |
| [TASK-c75be77c](TASK-c75be77c) | Resolve [AD-2aa4d6db](AD-2aa4d6db) SQLite scoping violation | 4 | Governance tables in SQLite violate [AD-2aa4d6db](AD-2aa4d6db). Decide: ephemeral cache or file-based. |

## Out of Scope

- Implementing E2E Playwright tests (separate epic)
- Achieving 80% coverage (this epic sets up measurement + adds critical missing tests)
- Rewriting the component inventory doc (covered by architecture documentation task)
