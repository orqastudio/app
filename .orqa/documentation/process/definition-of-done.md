---
id: DOC-027
title: Definition of Done
description: Checklist of gates that must be satisfied before any task is considered complete.
created: "2026-03-02"
updated: "2026-03-08"
---

**Date:** 2026-03-02

The Definition of Done (DoD) is the gate checklist that must be satisfied before any task is marked complete. It is used by the three review agents -- `code-reviewer`, `qa-tester`, and `ux-reviewer` -- as their verification checklist.

---

## Review Level

The orchestrator assigns a risk level at task start. The review gate matches the risk:

| Risk Level | Criteria | Review Gate |
|------------|----------|-------------|
| **Low** | Docs-only, config changes, single-file fixes, rule/agent definition updates | `code-reviewer` only |
| **Medium** | Single-layer changes (Rust-only or frontend-only), test additions | `code-reviewer` + `qa-tester` |
| **High** | Cross-boundary features (Rust + Svelte), IPC contract changes, schema changes, new UI flows | `code-reviewer` + `qa-tester` + `ux-reviewer` (if UI) |

---

## Why a Definition of Done?

"Done" means the user can use the feature right now, as a real user would. Not "it compiles". Not "the types check". Not "tests pass". Actually works.

---

## The Checklist

### Code Quality (checked by `code-reviewer`)

- [ ] `cargo fmt --check` passes -- zero formatting issues
- [ ] `cargo clippy -- -D warnings` passes -- zero warnings
- [ ] `cargo test` passes -- all Rust tests green
- [ ] `npm run check` passes -- svelte-check + TypeScript zero errors
- [ ] `npm run lint` passes -- ESLint zero errors
- [ ] `npm run test` passes -- all frontend tests green
- [ ] Test coverage is 80%+ for all affected modules
- [ ] No stubs, no hardcoded data, no TODO/FIXME comments
- [ ] No backwards compatibility shims, no aliases, no workarounds (see [RULE-018](RULE-018))

### End-to-End Completeness (checked by `code-reviewer`)

- [ ] All 4 layers exist for every new feature:
  - [ ] Rust command (`#[tauri::command]` function)
  - [ ] IPC types (Rust structs with Serialize/Deserialize AND matching TypeScript interfaces)
  - [ ] Svelte component (calls `invoke()` with correct command name)
  - [ ] Store binding (manages reactive state lifecycle)
- [ ] No legacy fallback paths (old commands removed, all callers updated)

### Documentation (checked by `code-reviewer`)

- [ ] Governing docs match implementation -- no drift between docs and code
- [ ] IPC contract documentation updated if commands were added or changed
- [ ] Component state table verified against implementation (if UI-facing)
- [ ] Any moved or deleted pages removed from all sidebar files
- [ ] Epic `docs-produced` items verified -- all listed documents have been created or updated
- [ ] Roadmap updated if this epic changes the project's milestone/epic/idea landscape

### Review Gates

- [ ] `code-reviewer` verdict: **PASS**
- [ ] `qa-tester` verdict: **PASS**
- [ ] `ux-reviewer` verdict: **PASS** (applies if the task is UI-facing)

### Functional Correctness (checked by `qa-tester`)

- [ ] "Would it work?" smoke test -- the feature works as a real user would experience it
- [ ] End-to-end data path verified: UI -> store -> invoke() -> Rust command -> SQLite/filesystem
- [ ] Data persists correctly (survives app restart)

### UX Compliance (checked by `ux-reviewer`, if UI-facing)

- [ ] All UI labels match the relevant `.orqa/documentation/ui/` spec exactly -- no deviations
- [ ] All component states handled: loading, error, empty, loaded, saving, unsaved changes
- [ ] Shared components used (EmptyState, LoadingSpinner, ErrorDisplay, PageToolbar, StatusBadge)
- [ ] No technical jargon in user-facing text

### Learning Loops

- [ ] Any new implementation patterns discovered during this task logged in `.orqa/governance/lessons/` as `IMPL-NNN` files
- [ ] Any review failures that revealed new patterns added as IMPL-NNN entries
- [ ] If a pattern appeared for the second or more time, recurrence count updated

### Artifact Updates

- [ ] Epic `status` updated to `done` in `.orqa/planning/epics/EPIC-NNN.md`
- [ ] All task statuses updated to `done` in `.orqa/planning/tasks/TASK-NNN.md`
- [ ] Epic task checkboxes all checked in `.orqa/planning/epics/EPIC-NNN.md`
- [ ] Parent milestone `completed-epics` count updated in `.orqa/planning/milestones/MS-NNN.md`
- [ ] If all P1 epics in the milestone are done — check whether milestone gate is satisfied
- [ ] Roadmap entry updated to reflect completion (items checked, status noted)

### Deliverable Completeness (checked by orchestrator)

- [ ] Every item in the epic's roadmap entry has been implemented — not deferred, not scaffolded
- [ ] Every item in the epic's plan has a completed task
- [ ] No "out of scope" items that were in the original roadmap scope (see [RULE-019](RULE-019))
- [ ] If any item was descoped, user approval for the descope is documented

### Commit & Session State

- [ ] All changes committed with a descriptive commit message
- [ ] Session state written to `tmp/session-state.md` (tasks completed, verification results, next steps)
- [ ] If Rust backend changes: orchestrator offers to run `make restart-tauri` (Vite stays alive — see `dogfood-mode.md`)

### Worktree Cleanup

- [ ] All changes committed in the worktree
- [ ] Branch merged to main
- [ ] Background processes in the worktree killed
- [ ] Branch deleted: `git branch -d <branch>`
- [ ] Worktree removed: `git worktree remove ../orqa-<task>`
- [ ] Worktree directory confirmed gone
- [ ] Post-merge verification: `cargo build && npm run build` both succeed

---

## How to Use This Checklist

**Implementing agent:** After completing work, review the checklist. Do NOT self-certify -- request the appropriate reviewer(s).

**`code-reviewer`:** Work through the Code Quality, End-to-End Completeness, and Documentation sections. Issue PASS or FAIL with evidence.

**`qa-tester`:** Work through the Functional Correctness section. Issue PASS or FAIL with evidence.

**`ux-reviewer`:** Work through the UX Compliance section. Issue PASS or FAIL with evidence.

**Orchestrator:** Confirm ALL applicable DoD items are satisfied before reporting task complete to the user.

> [!IMPORTANT]
> The orchestrator MUST NOT mark a task complete without all applicable DoD items checked.

---

## Evidence Requirements

Claiming "it works" without evidence is not verification. Each reviewer MUST include:

**For Tauri commands:**

```text
EVIDENCE: invoke('save_session', { session })
cargo test test_save_session -- passed
Rust command returns Ok(SessionId("sess-abc123"))
Verdict: REAL data -- session persisted to SQLite
```

**For frontend components:**

```text
EVIDENCE: Session panel
Component renders session history from store.
EmptyState shown when no sessions exist. LoadingSpinner shown during fetch.
Store calls invoke('list_sessions') and populates reactive state.
```

**For full chain verification:**

```text
EVIDENCE: End-to-end chain for save_session
1. Component: SessionPanel.svelte calls sessionStore.save()
2. Store: ui/src/lib/stores/session.svelte.ts calls invoke('save_session')
3. Command: backend/src-tauri/src/commands/session.rs -- #[tauri::command] fn save_session exists
4. Persistence: session written to SQLite via repository
Chain: COMPLETE
```

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The DoD makes the definition of "complete" explicit and structured — removing subjectivity from task completion and making governance requirements visible at every review gate. |
| Learning Through Reflection | The Learning Loops section of the checklist ensures that review failures feed the lesson promotion pipeline, so each completed task improves the process for future tasks. |

---

## Related Documents

- [Artifact Workflow](DOC-025) -- How artifacts flow through the development process
- [Artifact Framework](DOC-036) -- Artifact schemas and design principles
- [Definition of Ready](DOC-028) -- The gate checklist before implementation starts
- [Orchestration](DOC-030) -- Orchestrator verification gate protocol
- [Workflow](DOC-035) -- Full task lifecycle including review gate steps
- [Implementation Lessons](lessons) -- Patterns to document when review fails
