# Review: Migration Tasks Phases 1-3

## Verdict: FAIL

---

## Acceptance Criteria

### 1. Every ENGINE-classified module from the gap analysis has an extraction task

- [x] PASS

All 42 ENGINE-classified modules from `phase2-04-engine-app-gaps.md` have corresponding extraction tasks:
- 34 domain modules -> covered by P2-S1-01 (types), P2-S2-01 through P2-S2-09 (graph/artifact), P2-S3-01 through P2-S3-04 (enforcement), P2-S5-01 through P2-S5-04 (workflow), P2-S7-01 through P2-S7-03 (prompt), P2-S9-02 and P2-S9-03 (streaming), P2-S6-08 (CLI tool runner)
- 6 plugin modules -> P2-S6-01 through P2-S6-07
- 3 file-based repos -> P2-S3-04, P2-S11-01, P2-S11-02
- 2 already-extracted crates (orqa-validation, orqa-search) -> P2-S2-10, P2-S4-01

Every module from the ENGINE table in the gap analysis has a task. No modules are missing.

---

### 2. Every CLI engine logic file (~32 files) has a migration task

- [ ] FAIL

`phase2-05-root-infra-gaps.md` section 4 identifies **32 CLI source files** containing engine-level business logic in TypeScript (`libs/cli/`):
- `workflow-engine.ts`
- `workflow-resolver.ts`
- `prompt-pipeline.ts`
- `prompt-registry.ts`
- `knowledge-retrieval.ts`
- `gate-engine.ts`
- `agent-spawner.ts`
- `budget-enforcer.ts`
- `graph.ts`
- `validation-engine.ts`
- `enforcement-log.ts`
- (and ~21 more)

ARCHITECTURE.md Phase 2 header explicitly says: "Extract business logic from Tauri backend **and CLI** into Rust library crates."

**The task list contains ZERO tasks for migrating CLI TypeScript engine logic.** Every Phase 2 task addresses only `app/src-tauri/src/` Rust modules. The ~32 CLI files with engine logic (workflow resolution, prompt pipeline, knowledge retrieval, gate engine, agent spawner, budget enforcement) have no extraction tasks.

Additionally, ARCHITECTURE.md Phase 2 item 7 says: "Prompt crate -- prompt generation pipeline **(absorb connector's prompt-injector and knowledge-injector)**". There is no task for absorbing connector prompt/knowledge logic either (though this may belong in Phase 4).

**Impact:** After Phase 2 completes as currently defined, the engine would still be split across Rust (newly extracted) and TypeScript (CLI, untouched). The architecture's "standalone Rust crate" goal would not be met.

---

### 3. All tasks are atomic (fit one agent context window)

- [x] PASS

Every task specifies:
- Exact files to read, move, create, or modify
- Line counts where relevant (all under 1,200 lines per module)
- Concrete acceptance criteria checkboxes
- A focused scope (single module move, single config install, etc.)

The largest tasks are P2-S9-02 (stream loop, 1,042 lines) and P2-S9-03 (tool executor, 1,140 lines). These are at the upper bound but still fit a single agent context window given the clear scope.

P2-S1-01 (create engine crate with all types) lists 14 type files but they are all small struct/enum definitions (24-288 lines each, totaling ~1,400 lines of type definitions). This is borderline but feasible since it is pure type copying with no logic.

---

### 4. All tasks have specific, testable acceptance criteria

- [x] PASS

Every task has:
- Checkbox-format acceptance criteria
- `cargo build/test` success requirements for Rust tasks
- Specific function/type availability checks
- Reviewer checks with concrete verification steps

No task has vague criteria like "code is clean" or "architecture is improved."

---

### 5. No tasks say "consider" or "evaluate" -- they say what to DO

- [x] PASS

Grep for `consider`, `evaluate whether`, `may want to`, `should consider`, `might need` returns zero matches. All tasks use directive language: "Move", "Create", "Implement", "Delete", "Verify".

---

### 6. Dependencies are correct (no task depends on something that comes later)

- [x] PASS (with one note)

All dependency chains are forward-looking (earlier phases/steps before later ones). The dependency graph in the summary section matches the individual task dependency declarations.

**Note:** P2-S5-02 (process gates) depends on P2-S5-03 (process state), which has a higher task number. This is semantically correct -- process state must exist before gates can reference it -- but the numbering is misleading. The task numbering implies S5-02 should be done before S5-03, but the dependency says the opposite. This is a minor readability issue, not a correctness issue, since the `Depends on:` field governs execution order.

Similarly, P2-S3-02 (enforcement parser, depends on P2-S3-01) has a note "enforcement engine uses parser" which is backwards -- it is the parser that the engine uses, so the engine (S3-01) should come first or they should be independent. However, looking at the actual dependency: S3-02 depends on S3-01, meaning the enforcement engine module is created first, then the parser is added. This is correct if the `enforcement/mod.rs` must exist before `enforcement/parser.rs` can be added.

No circular dependencies exist. The critical path is correctly identified in the summary.

---

### 7. Nothing is deferred to a later phase that should be done here

- [ ] FAIL

Two items from ARCHITECTURE.md Phase 2 are missing from the task list:

**A. CLI TypeScript engine logic migration** (as detailed in criterion 2). ARCHITECTURE.md Phase 2 says "Extract business logic from Tauri backend and CLI." The task list only extracts from Tauri backend. The 32 CLI files with engine logic have no tasks in any phase (checked phases 4-5 as well -- no mention).

**B. app/.githooks/ logic absorption.** ARCHITECTURE.md Phase 2 item 3 says: "Enforcement crate -- rule evaluation, artifact validation **(absorb app/.githooks/ logic)**". The 15 hand-written validation scripts in `app/.githooks/` (schema validation, link checking, status transitions, pillar alignment, relationships, history, config consistency, stubs, lint suppression, task deps, epic readiness, core graph protection, plugin source protection) have no absorption task. Phase 1 installs target git hooks (P1-S2-06) but Phase 2 does not absorb the existing `.githooks/` enforcement logic into the Rust engine.

---

### 8. Stream loop abstraction task exists with proper trait design requirements

- [x] PASS

Three tasks cover the stream loop abstraction:
- **P2-S9-01** (line 1247): Design `SidecarTransport` and `EventEmitter` traits. AC includes: generic for Tauri/CLI/MCP, no Tauri type leaks, design documented as comments.
- **P2-S9-02** (line 1271): Move stream loop with trait-based abstraction. AC: uses `dyn SidecarTransport` and `dyn EventEmitter`, no Tauri imports in engine.
- **P2-S9-03** (line 1298): Move tool executor with `ToolExecutionContext` trait. AC: filesystem/shell/search abstraction, enforcement hooks preserved.

Additionally, P2-S7-03 (line 1193) creates a `SidecarClient` trait for session title generation.

The trait design requirements are well-specified: generic enough for multiple consumers, no Tauri type leaks, design-before-implementation ordering.

---

### 9. Phase 1 covers schema validation script, enforcement config installation, migration .claude/, all remaining targets

- [x] PASS

Phase 1 contains:
- **Step 1** (3 tasks): Schema composed JSON audit (P1-S1-01), validation script hardening (P1-S1-02), baseline run (P1-S1-03)
- **Step 2** (6 tasks): ESLint (P1-S2-01), Clippy (P1-S2-02), Markdownlint (P1-S2-03), Prettier (P1-S2-04), TypeScript tsconfig (P1-S2-05), Git hooks (P1-S2-06)
- **Step 3** (3 tasks): Migration CLAUDE.md (P1-S3-01), agent definitions (P1-S3-02), settings.json (P1-S3-03)
- **Step 4** (3 tasks): Target Claude Code plugin audit (P1-S4-01), target workflows (P1-S4-02), target plugin manifests (P1-S4-03)

This matches ARCHITECTURE.md Phase 1 section 13 exactly: target schema + validation, enforcement configs, migration .claude/, remaining targets.

---

### 10. Zero tech debt: every deletion is explicit, no "clean up later" items

- [x] PASS (within scope)

Within Phases 1-3, deletions are explicit:
- P2-S10-01: DELETE `app/tools/verify-pipeline-integrity.mjs`
- P2-S10-02: DELETE `app/tools/lint-relationships.mjs`
- P2-S10-03: DELETE `app/tools/verify-installed-content.mjs`
- Every module MOVE includes "MODIFY: App -- remove" in the file list

Two minor references to future work exist but are cross-phase references, not deferrals:
- Line 1100: "hardcoded GitHub URLs are preserved (they'll be made configurable later)" -- this is Phase 5 scope
- Line 1217: "placeholder for future generation" (agent module) -- this is Phase 4 scope

No "clean up later" items exist within the Phase 1-3 scope itself.

**However**, the missing `.githooks/` absorption (criterion 7B) means 15 hand-written enforcement scripts would survive Phase 2 without explicit deletion tasks. This IS tech debt left behind.

---

## Issues Found

### CRITICAL: Missing CLI TypeScript engine logic migration (32 files)

- **Location:** Not present in any task list file
- **Reference:** `phase2-05-root-infra-gaps.md` lines 79-81, ARCHITECTURE.md line 727
- **Impact:** After Phase 2, the engine would still be split across Rust and TypeScript. The "standalone Rust crate" architecture goal is not achieved.
- **Required action:** Add tasks for either (a) migrating the 11+ TypeScript engine modules to Rust, or (b) extracting them from `libs/cli` into a separate `@orqastudio/engine` TypeScript package (as recommended in the gap analysis), or (c) explicitly documenting in the architecture that the engine is dual-language and adding the TypeScript extraction to a later phase with explicit task references.

### SIGNIFICANT: Missing app/.githooks/ absorption (15 scripts)

- **Location:** ARCHITECTURE.md line 731 says "absorb app/.githooks/ logic"
- **Reference:** `phase2-05-root-infra-gaps.md` section 11, lines 299-321
- **Impact:** 15 hand-written enforcement scripts survive Phase 2 without being absorbed into the engine enforcement crate. Violates zero-tech-debt principle.
- **Required action:** Add tasks to analyze each of the 15 `.githooks/` validators and either absorb their logic into `orqa_engine::enforcement` or explicitly mark them as "stays in app until githooks plugin generates equivalents" with a task to document this decision.

### MINOR: P2-S5-02/P2-S5-03 numbering vs dependency mismatch

- **Location:** `migration-tasks-phase1-3.md` lines 911-949
- **Impact:** P2-S5-02 (process gates) depends on P2-S5-03 (process state), which is numbered later. Misleading but not incorrect since `Depends on:` governs order.
- **Required action:** Swap numbering so P2-S5-03 becomes P2-S5-02 and vice versa, or add a note.

---

## Summary Statistics

| Metric | Value |
|--------|-------|
| Total tasks | 68 (15 Phase 1 + 45 Phase 2 + 8 Phase 3) |
| Tasks with explicit ACs | 68/68 (100%) |
| Tasks with reviewer checks | 68/68 (100%) |
| Tasks with dependency declarations | 68/68 (100%) |
| Directive language (no "consider") | 68/68 (100%) |
| ENGINE modules covered | 42/42 (100%) |
| CLI engine files covered | 0/32 (0%) -- FAIL |
| app/.githooks/ scripts covered | 0/15 (0%) -- FAIL |

---

## Lessons

- Task lists derived from the Rust backend gap analysis (`phase2-04`) achieved 100% coverage. But the root/infra gap analysis (`phase2-05`) identified a parallel TypeScript engine that was not addressed. **Cross-referencing ALL gap analyses is essential** -- each gap document may identify modules in different parts of the codebase.
- ARCHITECTURE.md Phase 2 says "Tauri backend **and CLI**" but the task writer focused exclusively on the Tauri backend modules. The "and CLI" clause was dropped. Reading architecture phase descriptions word-by-word prevents this.
- The `.githooks/` absorption is explicitly called out in ARCHITECTURE.md Phase 2 item 3 but has no task. Architecture items that say "absorb X" must each become at least one task.
