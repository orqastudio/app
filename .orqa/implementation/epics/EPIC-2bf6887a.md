---
id: "EPIC-2bf6887a"
type: epic
title: "Codebase Audit and Architecture Documentation"
description: "Thorough code review ensuring artifact accuracy, documenting undiscovered implementation patterns, removing dead/outdated code, assessing test coverage, aligning linting with coding standards, and producing complete architecture documentation for the target core application."
status: archived
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
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

[RES-f66a29ad](RES-f66a29ad) identified significant gaps: 8 untested command modules, zero integration tests, zero component tests, documentation path drift post-restructure, clippy pedantic not explicitly configured, no coverage measurement tooling, and several active epic tasks marked done incorrectly.

This epic fixes the foundation before building more features. It's split into 4 phases: documentation fixes (quick wins), test infrastructure, architecture documentation, and governance cleanup.

## Implementation Design

### Phase 1: Documentation Path Fixes + Dead Code Cleanup

Quick wins. Fix all post-restructure path references, update module trees, remove dead code.

### Phase 2: Test Infrastructure + Coverage Tooling

Set up coverage measurement (`cargo tarpaulin`, Vitest coverage), add tests for untested command modules, create component test infrastructure, add `errors.svelte.ts` store test.

### Phase 3: Architecture Documentation

Complete end-to-end documentation of the target core application: artifact system, knowledge graph, prompt injection pipeline, rule enforcement, learning loop, plugin architecture, component library/SDK extraction, git integration points.

### Phase 4: Governance Cleanup

Fix incorrect task statuses on active epics, remove dead `scope` field references from agent-related docs, tighten [RULE-87ba1b81](RULE-87ba1b81) orchestrator exception list re: content creation vs coordination.

## Tasks

| ID | Title | Phase | Description |
| ---- | ------- | ------- | ------------- |
| [TASK-d1b856b5](TASK-d1b856b5) | Fix post-restructure path references in docs | 1 | Update `src-tauri/` → `backend/src-tauri/`, `persistence/` → `repo/` across all `.orqa/` docs |
| [TASK-cc7df87c](TASK-cc7df87c) | Update rust-modules.md module tree | 1 | Add `skill_injector.rs`, fix tree structure to match current codebase |
| [TASK-1c7cab8d](TASK-1c7cab8d) | Enable clippy pedantic in Cargo.toml | 1 | Add `[lints.clippy]` section with pedantic enabled, fix resulting warnings |
| [TASK-c8090c99](TASK-c8090c99) | Set up Rust coverage tooling | 2 | Configure `cargo tarpaulin` or `llvm-cov`, add `make coverage-rust` target |
| [TASK-a8cd2f21](TASK-a8cd2f21) | Set up frontend coverage tooling | 2 | Configure Vitest coverage reporter with 80% threshold, add `make coverage-frontend` |
| [TASK-49bf65bc](TASK-49bf65bc) | Add tests for untested command modules | 2 | Write tests for the 8 untested command files |
| [TASK-59a35835](TASK-59a35835) | Create component test infrastructure | 2 | Set up Svelte component testing with `@testing-library/svelte`, write template test |
| [TASK-316e5adf](TASK-316e5adf) | Add `errors.svelte.ts` store test | 2 | Write test file for the one untested store |
| [TASK-f50f84f8](TASK-f50f84f8) | Write core architecture documentation | 3 | End-to-end map of artifact system, knowledge graph, prompt injection, enforcement, learning loop |
| [TASK-cfeed07a](TASK-cfeed07a) | Document plugin architecture and SDK extraction plan | 3 | Component library extraction, view registration API, theme tokens, plugin distribution |
| [TASK-48464435](TASK-48464435) | Fix [EPIC-9ddef7f9](EPIC-9ddef7f9) task statuses | 4 | Revert [TASK-30045ad8](TASK-30045ad8) to in-progress, update [TASK-19a94ac8](TASK-19a94ac8) |
| [TASK-d7346ee9](TASK-d7346ee9) | Tighten [RULE-87ba1b81](RULE-87ba1b81) orchestrator content boundary | 4 | Clarify `.orqa/implementation/` exception: structure = orchestrator, content = Writer |
| [TASK-62b0543c](TASK-62b0543c) | Resolve [PD-859ed163](PD-859ed163) SQLite scoping violation | 4 | Governance tables in SQLite violate [PD-859ed163](PD-859ed163). Decide: ephemeral cache or file-based. |

## Out of Scope

- Implementing E2E Playwright tests (separate epic)
- Achieving 80% coverage (this epic sets up measurement + adds critical missing tests)
- Rewriting the component inventory doc (covered by architecture documentation task)
