# Migration Tasks: Phases 1-3

Exhaustive, atomic task list. Every task fits one agent context window. Nothing deferred.

---

## Phase 1: Establish Target States and Migration Enforcement

### Step 1: Target Schema + Validation Script (PREREQUISITE)

#### TASK P1-S1-01: Audit and complete `targets/schema.composed.json`

**What:** Review the existing `targets/schema.composed.json` against all plugin `orqa-plugin.json` manifests and `libs/types/src/platform/core.json`. Ensure every artifact type, relationship type, valid status, frontmatter field, and ID prefix is present and correct.

**Files:**
- READ: `libs/types/src/platform/core.json`
- READ: All `plugins/*/orqa-plugin.json` manifests
- READ/MODIFY: `targets/schema.composed.json`

**Acceptance Criteria:**
- [ ] Every artifact type from `core.json` and all plugin manifests appears in the composed schema with correct `key`, `label`, `icon`, `id_prefix`
- [ ] Every relationship type from `core.json` and all plugin manifests appears with correct `key`, `inverse`, `from`, `to`, `semantic`, `constraints`
- [ ] Valid statuses listed per artifact type match plugin-declared status machines
- [ ] Required frontmatter fields listed per artifact type
- [ ] Knowledge size constraints included (500-2000 tokens)
- [ ] Schema is valid JSON and parseable by `scripts/validate-artifacts.mjs`

**Reviewer Checks:**
- Cross-reference every type/relationship in schema against source definitions
- Verify no types or relationships are missing
- Verify no invented types or relationships exist that aren't in any source

---

#### TASK P1-S1-02: Audit and harden `scripts/validate-artifacts.mjs`

**What:** Review the existing 321-line validation script. Ensure it validates ALL required checks listed in ARCHITECTURE.md: required frontmatter fields per type, ID format (`TYPE-hex8`), type-location consistency, relationship target existence, status validity, knowledge size constraints.

**Files:**
- READ/MODIFY: `scripts/validate-artifacts.mjs`
- READ: `targets/schema.composed.json`

**Acceptance Criteria:**
- [ ] Validates required frontmatter fields per artifact type (id, type, title, status at minimum)
- [ ] Validates ID format matches `{id_prefix}-{hex8}` pattern from schema
- [ ] Validates type-key matches the directory the artifact lives in (type-location consistency)
- [ ] Validates relationship targets exist as artifact IDs in the project
- [ ] Validates status values are in the valid set for the artifact type
- [ ] Validates knowledge artifact body length (500-2000 tokens)
- [ ] `--staged` mode works (validates only git-staged `.md` files in `.orqa/`)
- [ ] `--hook` mode respects `ORQA_SKIP_SCHEMA_VALIDATION` env var
- [ ] `--summary` mode prints counts by category
- [ ] Exit code 0 = all valid, 1 = errors found, 2 = script error
- [ ] Running against current `.orqa/` reports known issues (confirms script detects real problems)

**Reviewer Checks:**
- Run script against `.orqa/` directory — confirm it exits non-zero and reports known issues
- Review each validation check has test coverage within the script or via manual test
- Confirm schema is loaded correctly and all fields are used

---

#### TASK P1-S1-03: Run validation script against current `.orqa/` and document baseline

**What:** Execute `node scripts/validate-artifacts.mjs --summary` and capture the output as a baseline. This establishes the "before" state that Phase 6 and 7 must resolve.

**Files:**
- RUN: `node scripts/validate-artifacts.mjs --summary > .state/validation-baseline.txt 2>&1`
- RUN: `node scripts/validate-artifacts.mjs > .state/validation-full.txt 2>&1`
- CREATE: `.state/validation-baseline.txt`

**Acceptance Criteria:**
- [ ] Baseline report exists at `.state/validation-baseline.txt`
- [ ] Full report exists at `.state/validation-full.txt`
- [ ] Script ran without crashing (exit code 1 for validation errors is expected, exit code 2 is a bug)
- [ ] Report shows counts per violation category

**Reviewer Checks:**
- Confirm exit code was 1 (errors found), not 2 (script error)
- Confirm the report includes at least: type-location mismatches, missing required fields, broken relationship links

---

### Step 2: Install Enforcement Configs

#### TASK P1-S2-01: Install ESLint enforcement config

**What:** Copy the target ESLint config into the app and verify it works.

**Files:**
- READ: `targets/enforcement/eslint/eslint.config.js`
- COPY TO: `app/eslint.config.js`
- MODIFY (if needed): `app/package.json` — add eslint dev dependencies

**Acceptance Criteria:**
- [ ] `app/eslint.config.js` matches `targets/enforcement/eslint/eslint.config.js` exactly
- [ ] `npx eslint --max-warnings 0 app/src/lib/` runs without script errors (lint errors are expected and acceptable at this stage)
- [ ] ESLint dependencies are installed (`eslint`, any plugins referenced in config)

**Reviewer Checks:**
- Diff `app/eslint.config.js` against `targets/enforcement/eslint/eslint.config.js` — must be identical
- Run `npx eslint --max-warnings 0 app/src/lib/` — confirm it executes (exit code 0 or 1, not 2)

**Depends on:** None (independent)

---

#### TASK P1-S2-02: Install Clippy enforcement config

**What:** Copy the target Clippy configs into the workspace.

**Files:**
- READ: `targets/enforcement/clippy/clippy.toml`
- READ: `targets/enforcement/clippy/workspace-lints.toml`
- MODIFY: Root `Cargo.toml` — add `[workspace.lints]` section from `workspace-lints.toml`
- COPY/MODIFY: `app/src-tauri/clippy.toml` — replace with target version (or root-level if workspace applies)

**Acceptance Criteria:**
- [ ] `[workspace.lints.clippy]` section in root `Cargo.toml` matches `targets/enforcement/clippy/workspace-lints.toml` contents
- [ ] `clippy.toml` at the appropriate level contains the target settings
- [ ] `cargo clippy --workspace 2>&1` runs without panicking (warnings are expected at this stage)

**Reviewer Checks:**
- Diff workspace lints section against target
- Run `cargo clippy --workspace` — confirm it executes

**Depends on:** None (independent)

---

#### TASK P1-S2-03: Install Markdownlint enforcement config

**What:** Create markdownlint config at project root. The target directory is currently empty, so write the config based on ARCHITECTURE.md requirements.

**Files:**
- CREATE: `.markdownlint.json` (or `.markdownlint-cli2.jsonc`)
- MODIFY: Root `package.json` — add `markdownlint-cli2` dev dependency

**Acceptance Criteria:**
- [ ] `.markdownlint.json` (or equivalent) exists at project root
- [ ] `npx markdownlint-cli2 "**/*.md" --no-globs` runs without script errors (lint errors are expected)
- [ ] `markdownlint-cli2` is listed in root `package.json` devDependencies

**Reviewer Checks:**
- Confirm config file exists and is valid JSON
- Run `npx markdownlint-cli2` against a sample `.md` file — confirm it executes

**Depends on:** None (independent)

---

#### TASK P1-S2-04: Install Prettier enforcement config

**What:** Create Prettier config at project root. The target directory is currently empty, so write the config based on ARCHITECTURE.md requirements.

**Files:**
- CREATE: `.prettierrc`
- CREATE: `.prettierignore`
- MODIFY: Root `package.json` — add `prettier`, `prettier-plugin-svelte`, `prettier-plugin-tailwindcss` dev dependencies

**Acceptance Criteria:**
- [ ] `.prettierrc` exists at project root with sensible defaults (tabs vs spaces, line width, plugins)
- [ ] `.prettierignore` exists and excludes: `target/`, `dist/`, `build/`, `node_modules/`, `.state/`, `*.lock`, generated files
- [ ] `npx prettier --check "app/src/**/*.svelte"` runs without script errors (format differences are expected)
- [ ] All Prettier dependencies are listed in root `package.json` devDependencies

**Reviewer Checks:**
- Confirm both files exist and are valid
- Run `npx prettier --check` against a sample file — confirm it executes
- Verify `.prettierignore` excludes appropriate paths

**Depends on:** None (independent)

---

#### TASK P1-S2-05: Install TypeScript tsconfig enforcement configs

**What:** Copy target tsconfig files to appropriate locations.

**Files:**
- READ: `targets/enforcement/tsconfig/base.json`
- READ: `targets/enforcement/tsconfig/app.json`
- READ: `targets/enforcement/tsconfig/library.json`
- COPY TO: Appropriate locations (base at root or `tsconfig/`, app in `app/`, library for libs)

**Acceptance Criteria:**
- [ ] Base tsconfig is installed and referenceable by app and library configs
- [ ] App tsconfig extends base with app-specific settings
- [ ] Library tsconfig extends base with library-specific settings
- [ ] `npx tsc --noEmit -p <app-tsconfig>` runs without script errors (type errors are expected at this stage)

**Reviewer Checks:**
- Verify tsconfig chain is correct (app extends base, library extends base)
- Run `npx tsc --noEmit` to confirm it executes

**Depends on:** None (independent)

---

#### TASK P1-S2-06: Install target Git hooks

**What:** Copy target pre-commit and post-commit hooks from `targets/enforcement/githooks/` and configure git to use them.

**Files:**
- READ: `targets/enforcement/githooks/pre-commit`
- READ: `targets/enforcement/githooks/post-commit`
- COPY TO: Appropriate hooks location (either `plugins/githooks/hooks/` or `.githooks/`)
- MODIFY: Git config or install script to set `core.hooksPath`

**Acceptance Criteria:**
- [ ] Pre-commit hook is installed and executable (`chmod +x`)
- [ ] Post-commit hook is installed and executable
- [ ] `git config core.hooksPath` points to the hooks directory
- [ ] Pre-commit hook respects `ORQA_SKIP_HOOKS=1` for emergency bypass
- [ ] Pre-commit hook calls `orqa validate --staged` for artifacts, `orqa check clippy/rustfmt` for Rust, `npx eslint` for frontend, `npx markdownlint-cli2` for markdown, `orqa check stubs --staged` for stub scanning, `orqa check lint-suppressions --staged` for suppression audit, `orqa check test-rust --staged` for scoped tests

**Reviewer Checks:**
- Verify hooks are executable
- Verify `git config core.hooksPath` is correctly set
- Read hook scripts — verify they call the correct commands per ARCHITECTURE.md A.4

**Depends on:** P1-S2-01, P1-S2-02, P1-S2-03 (hooks call these tools)

---

### Step 3: Migration `.claude/` Instance

#### TASK P1-S3-01: Write migration `.claude/CLAUDE.md`

**What:** Write the migration-period CLAUDE.md that includes full migration context, architecture references, target state awareness, and phase plan awareness. This is MORE aggressive than the post-migration version — it enforces everything at every step.

**Files:**
- CREATE/MODIFY: `.claude/CLAUDE.md`
- READ: `targets/claude-code-plugin/.claude/CLAUDE.md` (reference for structure)
- READ: `ARCHITECTURE.md` (sections 1-13)

**Acceptance Criteria:**
- [ ] CLAUDE.md includes autonomous execution instructions (non-negotiable)
- [ ] References ARCHITECTURE.md as the source of truth
- [ ] Includes team discipline section (TeamCreate, TaskCreate, Agent, TaskUpdate, TeamDelete)
- [ ] Includes hub-spoke orchestration instructions
- [ ] Includes role-based tool constraints table
- [ ] Includes completion gate instructions (strict, no silent deferrals)
- [ ] Includes migration-specific context (current phase, what has been done, what comes next)
- [ ] References target state files in `targets/`
- [ ] Includes git workflow instructions (no --no-verify, rebuild after Rust changes)
- [ ] Includes session protocol (read session state, resume, write state)
- [ ] Includes drift prevention list (DO NOT items)
- [ ] `ORQA_SKIP_SCHEMA_VALIDATION=true` is set — schema validation disabled in hooks until Phase 6-7

**Reviewer Checks:**
- Compare against `targets/claude-code-plugin/.claude/CLAUDE.md` — migration version should be a superset
- Verify all NON-NEGOTIABLE sections from current CLAUDE.md are preserved
- Verify migration-specific instructions are clear and actionable

**Depends on:** None (can be written independently)

---

#### TASK P1-S3-02: Write migration `.claude/agents/` definitions

**What:** Write agent role definitions for the migration period. These should include migration context (architecture references, target state awareness) beyond what the post-migration versions contain.

**Files:**
- CREATE/MODIFY: `.claude/agents/implementer.md`
- CREATE/MODIFY: `.claude/agents/reviewer.md`
- CREATE/MODIFY: `.claude/agents/researcher.md`
- CREATE/MODIFY: `.claude/agents/writer.md`
- CREATE/MODIFY: `.claude/agents/governance-steward.md`
- CREATE/MODIFY: `.claude/agents/planner.md`
- READ: `targets/claude-code-plugin/.claude/agents/` (reference for structure)

**Acceptance Criteria:**
- [ ] All 6 agent roles have definition files
- [ ] Each definition includes: role boundaries, tool access restrictions, completion standards
- [ ] Each definition includes migration-specific context (architecture references, knowledge references)
- [ ] Implementer: can edit source code, can run shell, migration-aware
- [ ] Reviewer: read-only, can run checks, produces verdicts against acceptance criteria
- [ ] Researcher: no edit, no shell, creates research artifacts only
- [ ] Writer: can edit docs, no shell, documentation only
- [ ] Governance Steward: can edit `.orqa/` only, no shell
- [ ] Planner: can read, no edit, produces plans only

**Reviewer Checks:**
- Each agent file has correct tool access restrictions matching CLAUDE.md role table
- Migration context is present and references correct file paths
- No agent has broader permissions than its role allows

**Depends on:** None (can be written independently)

---

#### TASK P1-S3-03: Write migration `.claude/settings.json`

**What:** Write Claude Code settings for the migration period, including hook configurations that call enforcement tooling directly.

**Files:**
- CREATE/MODIFY: `.claude/settings.json`
- READ: `targets/claude-code-plugin/.claude/settings.json` (reference)

**Acceptance Criteria:**
- [ ] `settings.json` exists with valid JSON
- [ ] Hooks are configured to call enforcement tools directly (not via daemon)
- [ ] Pre-commit equivalent hooks call: eslint, clippy, markdownlint, validation script, scoped tests
- [ ] `ORQA_SKIP_SCHEMA_VALIDATION=true` is set in hook environment (disabled until Phase 6-7)
- [ ] Permissions are configured for agent roles

**Reviewer Checks:**
- JSON is valid and parseable
- Hook commands are correct and would execute on the current system
- Environment variable for schema validation skip is present

**Depends on:** P1-S2-01 through P1-S2-06 (hooks call these tools)

---

### Step 4: Remaining Targets

#### TASK P1-S4-01: Audit and complete target Claude Code Plugin

**What:** Review the existing `targets/claude-code-plugin/` contents (`.claude/` and `.claude-plugin/`) for completeness against ARCHITECTURE.md. This target defines what the connector should eventually generate.

**Files:**
- READ: `targets/claude-code-plugin/.claude/CLAUDE.md`
- READ: `targets/claude-code-plugin/.claude/agents/` (all files)
- READ: `targets/claude-code-plugin/.claude/settings.json`
- READ: `targets/claude-code-plugin/.claude/architecture/` (all files)
- READ: `targets/claude-code-plugin/.claude-plugin/plugin.json`
- READ: `targets/claude-code-plugin/.claude-plugin/hooks/`
- READ: `targets/claude-code-plugin/.claude-plugin/scripts/`
- READ: `targets/claude-code-plugin/.claude-plugin/skills/`
- MODIFY (if gaps found): any of the above

**Acceptance Criteria:**
- [ ] `.claude/CLAUDE.md` covers: architecture, team discipline, role constraints, completion gates, git workflow, session protocol, drift prevention
- [ ] Agent definitions exist for all roles in ARCHITECTURE.md's authoritative role list
- [ ] `.claude-plugin/plugin.json` has correct structure (commands, hooks, skills, resources)
- [ ] `.claude-plugin/hooks/` has hook definitions that delegate to engine/CLI
- [ ] `.claude-plugin/skills/` has skill definitions for key workflows
- [ ] Architecture reference docs in `.claude/architecture/` cover all major subsystems
- [ ] No references to obsolete paths (no `process/` nesting, no `tmp/`)

**Reviewer Checks:**
- Every file is internally consistent
- No references to paths that don't exist or will be removed
- Plugin.json is valid JSON and matches the OrqaStudio plugin schema

**Depends on:** P1-S1-01 (needs finalized schema)

---

#### TASK P1-S4-02: Write target resolved workflows

**What:** Write one resolved workflow file per methodology stage. These represent what the engine's workflow resolver should produce.

**Files:**
- CREATE: `targets/workflows/` directory
- CREATE: `targets/workflows/discovery.resolved.json`
- CREATE: `targets/workflows/planning.resolved.json`
- CREATE: `targets/workflows/documentation.resolved.json`
- CREATE: `targets/workflows/implementation.resolved.json`
- CREATE: `targets/workflows/review.resolved.json`
- CREATE: `targets/workflows/learning.resolved.json`
- READ: `plugins/agile-workflow/orqa-plugin.json` (workflow definitions)
- READ: `plugins/*/orqa-plugin.json` (all stage plugins)
- READ: ARCHITECTURE.md section 7 (Workflow Resolution)

**Acceptance Criteria:**
- [ ] One `.resolved.json` file exists per methodology stage
- [ ] Each resolved workflow contains: stage name, entry criteria, exit criteria, artifact types produced, relationships created, agent specializations, knowledge injection triggers
- [ ] Workflows are internally consistent (exit criteria of one stage = entry criteria of the next)
- [ ] All artifact types and relationships referenced exist in `targets/schema.composed.json`
- [ ] Workflows are valid JSON

**Reviewer Checks:**
- Cross-reference each workflow against the plugin that owns that stage
- Verify stage transitions are consistent (no gaps, no overlaps)
- Verify all referenced types/relationships exist in composed schema

**Depends on:** P1-S1-01 (needs finalized schema)

---

#### TASK P1-S4-03: Write target plugin manifests

**What:** Write corrected `orqa-plugin.json` for each first-party plugin with all required taxonomy fields.

**Files:**
- READ: Each `plugins/*/orqa-plugin.json`
- CREATE: `targets/plugin-manifests/` directory
- CREATE: `targets/plugin-manifests/{plugin-name}.orqa-plugin.json` for each plugin

**Acceptance Criteria:**
- [ ] One target manifest exists per first-party plugin
- [ ] Each manifest has `purpose` field (methodology, workflow, knowledge, connector, infrastructure, sidecar)
- [ ] Workflow plugins have `stage_slot` field
- [ ] Category vocabulary is standardized per ARCHITECTURE.md taxonomy
- [ ] Content installation targets use correct stage-first paths (e.g., `.orqa/learning/rules/` not `.orqa/process/rules/`)
- [ ] Schema field naming uses `title` not `name`
- [ ] All files referenced in manifest `provides` actually exist in the plugin source
- [ ] Valid JSON

**Reviewer Checks:**
- Every plugin has a corresponding target manifest
- Every manifest has all required fields per ARCHITECTURE.md section 4.1
- No manifest references files that don't exist in the plugin source

**Depends on:** P1-S1-01 (needs finalized schema for type/relationship validation)

---

## Phase 2: Engine Extraction

### Step 1: Types and Traits First

#### TASK P2-S1-01: Create `libs/engine/` Rust crate with core type definitions

**What:** Create the new `orqa-engine` Rust library crate. Move all pure type definitions (no logic, no I/O) from `app/src-tauri/src/domain/` into the engine crate. Add to Cargo workspace.

**Files to create:**
- `libs/engine/Cargo.toml`
- `libs/engine/src/lib.rs`
- `libs/engine/src/types/mod.rs`
- `libs/engine/src/types/artifact.rs` — `Artifact`, `ArtifactType`, `ArtifactMetadata` structs (from `domain/artifact.rs`, type definitions only, ~100 lines)
- `libs/engine/src/types/enforcement.rs` — `EventType`, `RuleAction`, `Verdict`, `EnforcementRule`, `EnforcementEntry`, `EnforcementViolation` (from `domain/enforcement.rs` 113 lines + `enforcement_violation.rs` 24 lines)
- `libs/engine/src/types/governance.rs` — `GovernanceScanResult`, `GovernanceArea` (from `domain/governance.rs` 43 lines)
- `libs/engine/src/types/health.rs` — `HealthSnapshot` (from `domain/health_snapshot.rs` 48 lines)
- `libs/engine/src/types/lesson.rs` — `Lesson` struct (from `domain/lessons.rs`, struct only)
- `libs/engine/src/types/message.rs` — `Message`, `MessageRole`, `ContentType`, `StreamStatus` (from `domain/message.rs` 192 lines)
- `libs/engine/src/types/project.rs` — `Project`, `ProjectSummary`, `DetectedStack`, `ScanResult` (from `domain/project.rs` 119 lines)
- `libs/engine/src/types/session.rs` — `Session`, `SessionSummary`, `SessionStatus` (from `domain/session.rs` 128 lines)
- `libs/engine/src/types/settings.rs` — `ResolvedTheme`, `SidecarStatus`, `SidecarState` (from `domain/settings.rs` 171 lines)
- `libs/engine/src/types/streaming.rs` — `StreamEvent` enum with 16 variants (from `domain/provider_event.rs` 288 lines)
- `libs/engine/src/types/workflow.rs` — `ProposedTransition`, `GateResult`, `SessionProcessState`, `ToolCallRecord` (from `domain/status_transitions.rs` + `process_state.rs` + `process_gates.rs`, type definitions only)
- `libs/engine/src/types/knowledge.rs` — `KnowledgeMatch` struct (from `domain/knowledge_injector.rs`, struct only)

**Files to modify:**
- Root `Cargo.toml` — add `libs/engine` to workspace members
- `app/src-tauri/Cargo.toml` — add `orqa-engine` as path dependency

**Acceptance Criteria:**
- [ ] `libs/engine/Cargo.toml` exists with `name = "orqa-engine"`, `version = "0.1.4-dev"`, edition 2021
- [ ] Root `Cargo.toml` workspace members includes `"libs/engine"`
- [ ] All listed type structs and enums are defined in the engine crate
- [ ] Types derive `Serialize`, `Deserialize`, `Debug`, `Clone` as appropriate
- [ ] `cargo build -p orqa-engine` succeeds with zero errors
- [ ] `cargo clippy -p orqa-engine` passes
- [ ] No business logic in this crate yet — types only

**Reviewer Checks:**
- Verify every struct/enum listed above exists in the engine crate
- Verify `cargo build -p orqa-engine` succeeds
- Verify no logic functions exist — only type definitions and trait implementations (Display, From, etc.)
- Verify Cargo.toml has correct metadata

**Depends on:** None (first task in Phase 2)

---

#### TASK P2-S1-02: Define storage traits in engine crate

**What:** Define trait interfaces for storage operations that the engine needs but doesn't own the implementation of (e.g., SQLite is app-specific, file-based is engine-native).

**Files to create:**
- `libs/engine/src/traits/mod.rs`
- `libs/engine/src/traits/storage.rs` — `ArtifactStore`, `EnforcementRuleStore`, `LessonStore`, `ProjectSettingsStore` traits

**Files to modify:**
- `libs/engine/src/lib.rs` — add `traits` module

**Acceptance Criteria:**
- [ ] `ArtifactStore` trait: `read(path) -> Result<Artifact>`, `write(path, artifact) -> Result<()>`, `scan(dir) -> Result<Vec<Artifact>>`, `delete(path) -> Result<()>`
- [ ] `EnforcementRuleStore` trait: `load_rules(root) -> Result<Vec<EnforcementRule>>`
- [ ] `LessonStore` trait: `read(path) -> Result<Lesson>`, `write(path, lesson) -> Result<()>`, `scan(dir) -> Result<Vec<Lesson>>`
- [ ] `ProjectSettingsStore` trait: `load(root) -> Result<Option<ProjectSettings>>`
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] Traits are generic enough for both file-based and SQLite implementations

**Reviewer Checks:**
- Verify trait methods match the operations currently performed in the app's domain modules
- Verify traits don't leak implementation details (no SQLite types, no Tauri types)
- Verify `cargo build` succeeds

**Depends on:** P2-S1-01 (needs types defined)

---

#### TASK P2-S1-03: Wire app to use engine types (re-export migration)

**What:** Update `app/src-tauri/` to import types from `orqa-engine` instead of defining them locally. Remove duplicated type definitions from app domain modules. Keep logic functions in app for now — only move the type imports.

**Files to modify:**
- `app/src-tauri/src/domain/artifact.rs` — remove struct definitions, `use orqa_engine::types::*`
- `app/src-tauri/src/domain/enforcement.rs` — remove structs, re-export from engine
- `app/src-tauri/src/domain/enforcement_violation.rs` — remove, re-export from engine
- `app/src-tauri/src/domain/governance.rs` — remove structs, re-export from engine
- `app/src-tauri/src/domain/health_snapshot.rs` — remove struct, re-export
- `app/src-tauri/src/domain/message.rs` — remove structs, re-export
- `app/src-tauri/src/domain/project.rs` — remove structs, re-export
- `app/src-tauri/src/domain/session.rs` — remove structs, re-export
- `app/src-tauri/src/domain/settings.rs` — remove structs, re-export
- `app/src-tauri/src/domain/provider_event.rs` — remove enum, re-export
- All `commands/*.rs` files that reference these types — update imports

**Acceptance Criteria:**
- [ ] All type definitions come from `orqa-engine` (single source of truth)
- [ ] `app/src-tauri/` compiles with `cargo build -p orqa-studio`
- [ ] All existing tests pass (`cargo test -p orqa-studio`)
- [ ] No duplicate type definitions remain in app domain modules
- [ ] Logic functions in app still work (they reference engine types now)

**Reviewer Checks:**
- `cargo build -p orqa-studio` succeeds
- `cargo test -p orqa-studio` passes
- Grep for struct definitions that should have moved — verify they're gone from app
- Verify imports point to `orqa_engine::types::*`

**Depends on:** P2-S1-01 (engine types must exist first)

---

### Step 2: Graph Crate (Artifact Reader, Relationship Engine, Traceability)

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each module move must be validated against ARCHITECTURE.md to confirm the target location and API surface are correct.

#### TASK P2-S2-01: Move platform config to engine

**What:** Move `domain/platform_config.rs` (251 lines) to the engine crate. This includes compile-time `core.json` loading, `PlatformConfig` struct, inverse map building, semantic queries.

**Files:**
- MOVE: `app/src-tauri/src/domain/platform_config.rs` -> `libs/engine/src/platform.rs`
- COPY: `libs/types/src/platform/core.json` -> `libs/engine/src/platform/core.json` (or reference via path)
- MODIFY: `libs/engine/src/lib.rs` — add `platform` module
- MODIFY: `app/src-tauri/src/domain/mod.rs` — remove `platform_config`, add `use orqa_engine::platform`
- MODIFY: All app files importing from `domain::platform_config` — update imports

**Acceptance Criteria:**
- [ ] `platform_config()`, `build_inverse_map()`, `build_merged_inverse_map()`, `keys_for_semantic()`, `has_semantic()` all available from `orqa_engine::platform`
- [ ] `PLATFORM` lazy static is in engine
- [ ] `core.json` is embedded at compile time via `include_str!` in the engine crate
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] `cargo test -p orqa-engine` passes (move the 6 tests)
- [ ] `cargo build -p orqa-studio` succeeds (app uses engine re-exports)
- [ ] `cargo test -p orqa-studio` passes

**Reviewer Checks:**
- Verify all 6 tests moved and pass
- Verify `core.json` is correctly embedded
- Verify no platform config code remains in app domain

**Depends on:** P2-S1-01 (engine crate exists), P2-S1-03 (app wired to engine types)

---

#### TASK P2-S2-02: Move artifact core types and parsing to engine

**What:** Move the logic portion of `domain/artifact.rs` (687 lines) to the engine: `parse_artifact()`, `generate_id()`, `normalize_relationships()`. The struct definitions were already moved in P2-S1-01.

**Files:**
- MOVE logic from: `app/src-tauri/src/domain/artifact.rs` -> `libs/engine/src/artifact/mod.rs`
- CREATE: `libs/engine/src/artifact/mod.rs` (parsing, ID generation)
- MODIFY: `libs/engine/src/lib.rs` — add `artifact` module
- MODIFY: `app/src-tauri/src/domain/artifact.rs` — keep only re-exports from engine
- MODIFY: All app files calling these functions — update imports

**Acceptance Criteria:**
- [ ] `parse_artifact()`, `generate_id()`, `normalize_relationships()` available from `orqa_engine::artifact`
- [ ] All tests from `domain/artifact.rs` moved to engine and passing
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] `cargo test -p orqa-engine` passes
- [ ] `cargo build -p orqa-studio` succeeds
- [ ] `cargo test -p orqa-studio` passes

**Reviewer Checks:**
- Verify parse/ID-gen logic is in engine, not app
- Verify all artifact tests pass in engine crate
- Verify app compiles and tests pass

**Depends on:** P2-S1-03 (app using engine types)

---

#### TASK P2-S2-03: Move artifact filesystem operations to engine

**What:** Move `domain/artifact_fs.rs` (301 lines) to engine: `write_artifact_file()`, `scan_directory()`, `read_artifact_file()`, `delete_artifact_file()`.

**Files:**
- MOVE: `app/src-tauri/src/domain/artifact_fs.rs` -> `libs/engine/src/artifact/fs.rs`
- MODIFY: `libs/engine/src/artifact/mod.rs` — add `fs` submodule
- MODIFY: App — remove `artifact_fs.rs`, update imports

**Acceptance Criteria:**
- [ ] All 4 filesystem functions available from `orqa_engine::artifact::fs`
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] `cargo test -p orqa-engine` passes
- [ ] `cargo build -p orqa-studio` succeeds
- [ ] `cargo test -p orqa-studio` passes

**Reviewer Checks:**
- Verify all functions moved
- Verify no I/O code remains in app for artifact operations (except command wrappers)
- All tests pass

**Depends on:** P2-S2-02 (artifact parsing must be in engine first)

---

#### TASK P2-S2-04: Move artifact reader (navigation tree) to engine

**What:** Move `domain/artifact_reader.rs` (874 lines) to engine: `NavigationNode`, `scan_navigation_tree()`.

**Files:**
- MOVE: `app/src-tauri/src/domain/artifact_reader.rs` -> `libs/engine/src/artifact/reader.rs`
- MODIFY: `libs/engine/src/artifact/mod.rs` — add `reader` submodule
- MODIFY: App — remove `artifact_reader.rs`, update imports

**Acceptance Criteria:**
- [ ] `NavigationNode` struct and `scan_navigation_tree()` available from `orqa_engine::artifact::reader`
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] `cargo test -p orqa-engine` passes
- [ ] `cargo build -p orqa-studio` succeeds
- [ ] `cargo test -p orqa-studio` passes

**Reviewer Checks:**
- Verify complete module moved with all tests
- Verify no navigation tree logic remains in app

**Depends on:** P2-S2-03 (artifact fs must be in engine)

---

#### TASK P2-S2-05: Move config loader and paths to engine

**What:** Move `domain/config_loader.rs` (94 lines) and `domain/paths.rs` (242 lines) to engine.

**Files:**
- MOVE: `app/src-tauri/src/domain/config_loader.rs` -> `libs/engine/src/config.rs`
- MOVE: `app/src-tauri/src/domain/paths.rs` -> `libs/engine/src/paths.rs`
- MODIFY: `libs/engine/src/lib.rs` — add modules
- MODIFY: App — remove these files, update imports

**Acceptance Criteria:**
- [ ] `load_project_settings()` available from `orqa_engine::config`
- [ ] `ProjectPaths` and all path resolution available from `orqa_engine::paths`
- [ ] All 4 path tests moved and passing
- [ ] `cargo build/test` succeeds for both engine and app

**Reviewer Checks:**
- Verify all functions and tests moved
- Verify app compiles and tests pass

**Depends on:** P2-S1-03 (app using engine types)

---

#### TASK P2-S2-06: Move project scanner to engine

**What:** Move `domain/project_scanner.rs` (396 lines) to engine: language, framework, package manager detection.

**Files:**
- MOVE: `app/src-tauri/src/domain/project_scanner.rs` -> `libs/engine/src/project/scanner.rs`
- CREATE: `libs/engine/src/project/mod.rs`
- MODIFY: `libs/engine/src/lib.rs` — add `project` module
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] Project scanning functions available from `orqa_engine::project::scanner`
- [ ] All 4 tests moved and passing
- [ ] `cargo build/test` succeeds for both engine and app

**Reviewer Checks:**
- Verify all detection logic (12 languages, 9 frameworks, 6 package managers) is present
- All tests pass

**Depends on:** P2-S2-05 (config loader must be in engine for project settings)

---

#### TASK P2-S2-07: Move project settings types to engine

**What:** Move `domain/project_settings.rs` (251 lines) to engine. This partially re-exports from `orqa_validation` and adds local types.

**Files:**
- MOVE: `app/src-tauri/src/domain/project_settings.rs` -> `libs/engine/src/project/settings.rs`
- MODIFY: `libs/engine/Cargo.toml` — add `orqa-validation` as dependency
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `ProjectRelationshipConfig`, `ProjectPluginConfig`, `NavigationConfig` available from engine
- [ ] Re-exports from `orqa_validation` work through engine
- [ ] `cargo build/test` succeeds for both engine and app

**Reviewer Checks:**
- Verify `orqa-validation` dependency is correctly wired
- Verify all types accessible through engine API

**Depends on:** P2-S1-01 (engine crate exists)

---

#### TASK P2-S2-08: Move lesson types and parsing to engine

**What:** Move `domain/lessons.rs` (252 lines) to engine: `Lesson` struct, `parse_lesson()`, `render_lesson()`.

**Files:**
- MOVE: `app/src-tauri/src/domain/lessons.rs` -> `libs/engine/src/lesson.rs`
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `Lesson`, `parse_lesson()`, `render_lesson()` available from `orqa_engine::lesson`
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all functions moved
- All tests pass

**Depends on:** P2-S1-01 (engine crate exists)

---

#### TASK P2-S2-09: Move time utilities to engine

**What:** Move `domain/time_utils.rs` (272 lines) to engine: calendar utilities with no external dependencies.

**Files:**
- MOVE: `app/src-tauri/src/domain/time_utils.rs` -> `libs/engine/src/utils/time.rs`
- CREATE: `libs/engine/src/utils/mod.rs`
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] All time utility functions available from `orqa_engine::utils::time`
- [ ] All 10+ tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all tests pass
- Verify no chrono dependency (architecture specifies no chrono)

**Depends on:** P2-S1-01 (engine crate exists)

---

#### TASK P2-S2-10: Integrate with existing `orqa-validation` crate

**What:** Add `orqa-validation` as a dependency of `orqa-engine` and re-export its graph, integrity, and metrics APIs through the engine's public interface.

**Files:**
- MODIFY: `libs/engine/Cargo.toml` — add `orqa-validation` dependency (path)
- CREATE: `libs/engine/src/graph.rs` — re-exports from `orqa_validation`
- CREATE: `libs/engine/src/validation.rs` — re-exports
- CREATE: `libs/engine/src/metrics.rs` — re-exports
- MODIFY: `libs/engine/src/lib.rs` — add modules

**Acceptance Criteria:**
- [ ] `orqa_engine::graph` re-exports `build_artifact_graph`, `validate`, `compute_health`, `auto_fix`, `compute_traceability`, `graph_stats`
- [ ] `orqa_engine::validation` re-exports integrity check types
- [ ] `orqa_engine::metrics` re-exports metric types
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] All validation crate tests still pass: `cargo test -p orqa-validation`

**Reviewer Checks:**
- Verify re-exports match the API surface listed in ARCHITECTURE.md section and gap analysis Part 3
- Verify no duplication (engine re-exports, doesn't re-implement)
- Verify validation crate is unmodified

**Depends on:** P2-S1-01 (engine crate exists)

---

### Step 3: Enforcement Crate

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each module move must be validated against ARCHITECTURE.md to confirm the target location and API surface are correct.

#### TASK P2-S3-01: Move enforcement engine to engine crate

**What:** Move `domain/enforcement_engine.rs` (783 lines) to engine: `EnforcementEngine`, compiled regex evaluation, all 4 evaluate methods.

**Files:**
- MOVE: `app/src-tauri/src/domain/enforcement_engine.rs` -> `libs/engine/src/enforcement/engine.rs`
- CREATE: `libs/engine/src/enforcement/mod.rs`
- MODIFY: `libs/engine/Cargo.toml` — add `regex` dependency
- MODIFY: App — remove, update imports and `AppState`

**Acceptance Criteria:**
- [ ] `EnforcementEngine` with `load()`, `evaluate_file()`, `evaluate_bash()`, `evaluate_scan()`, `evaluate_lint()` available from `orqa_engine::enforcement`
- [ ] All 10+ tests moved and passing
- [ ] `cargo build/test` succeeds for both engine and app
- [ ] App's `EnforcementState` references engine type

**Reviewer Checks:**
- Verify all evaluate methods moved
- Verify all tests pass
- Verify regex compilation logic is intact

**Depends on:** P2-S1-03 (app using engine types — enforcement types must be in engine)

---

#### TASK P2-S3-02: Move enforcement parser to engine crate

**What:** Move `domain/enforcement_parser.rs` (340 lines) to engine: YAML frontmatter parsing for enforcement rule `.md` files.

**Files:**
- MOVE: `app/src-tauri/src/domain/enforcement_parser.rs` -> `libs/engine/src/enforcement/parser.rs`
- MODIFY: `libs/engine/Cargo.toml` — add `serde_yaml` dependency if not already present
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `parse_enforcement_rule()` available from `orqa_engine::enforcement::parser`
- [ ] All 8 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify parser handles all edge cases (missing fields, invalid YAML, etc.)
- All tests pass

**Depends on:** P2-S3-01 (enforcement engine uses parser)

---

#### TASK P2-S3-03: Move governance scanner to engine crate

**What:** Move `domain/governance_scanner.rs` (422 lines) to engine: scans 6 governance areas in `.orqa/`.

**Files:**
- MOVE: `app/src-tauri/src/domain/governance_scanner.rs` -> `libs/engine/src/enforcement/scanner.rs`
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `scan_governance()` available from `orqa_engine::enforcement::scanner`
- [ ] All 6 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all 6 governance areas are scanned
- All tests pass

**Depends on:** P2-S3-01 (enforcement module exists in engine)

---

#### TASK P2-S3-04: Move enforcement file-based repos to engine

**What:** Move `repo/enforcement_rules_repo.rs` (129 lines) to engine. This is a file-based repo that reads rule files from disk — it belongs in the engine, not the app.

**Files:**
- MOVE: `app/src-tauri/src/repo/enforcement_rules_repo.rs` -> `libs/engine/src/enforcement/store.rs`
- IMPLEMENT: The `EnforcementRuleStore` trait (from P2-S1-02) on a `FileEnforcementRuleStore` struct
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `FileEnforcementRuleStore` implements `EnforcementRuleStore` trait
- [ ] `load_rules()` reads from disk and returns parsed rules
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify trait implementation matches trait definition
- Verify file I/O logic is correct

**Depends on:** P2-S1-02 (traits defined), P2-S3-02 (parser in engine)

---

### Step 4: Search Crate Integration

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each integration must be validated against ARCHITECTURE.md to confirm the API surface is correct.

#### TASK P2-S4-01: Integrate `orqa-search` into engine

**What:** Add `orqa-search` as a dependency and re-export its search API through the engine's public interface.

**Files:**
- MODIFY: `libs/engine/Cargo.toml` — add `orqa-search` dependency (path)
- CREATE: `libs/engine/src/search.rs` — re-exports from `orqa_search`
- MODIFY: `libs/engine/src/lib.rs` — add `search` module

**Acceptance Criteria:**
- [ ] `orqa_engine::search` re-exports `SearchEngine`, `search_semantic()`, `search_regex()`, `index()`
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] `cargo test -p orqa-search` still passes (search crate unmodified)

**Reviewer Checks:**
- Verify re-exports match API surface from gap analysis
- Verify search crate is unmodified

**Depends on:** P2-S1-01 (engine crate exists)

---

### Step 5: Workflow Crate

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each module move must be validated against ARCHITECTURE.md to confirm the target location and API surface are correct.

#### TASK P2-S5-01: Move status transitions engine to engine crate

**What:** Move `domain/status_transitions.rs` (880 lines) to engine: `evaluate_transitions()` with 5 named conditions.

**Files:**
- MOVE: `app/src-tauri/src/domain/status_transitions.rs` -> `libs/engine/src/workflow/transitions.rs`
- CREATE: `libs/engine/src/workflow/mod.rs`
- MODIFY: `libs/engine/src/lib.rs` — add `workflow` module
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `evaluate_transitions()` available from `orqa_engine::workflow::transitions`
- [ ] All 5 named conditions work: `all_children_status`, `any_child_status`, `no_children_status`, `parent_status`, `relationship_count`
- [ ] All 15+ tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all 5 conditions are present
- Verify all tests pass
- Verify config-driven evaluation works with schema-provided status rules

**Depends on:** P2-S2-01 (platform config in engine), P2-S1-03 (engine types)

---

#### TASK P2-S5-02: Move process gates to engine crate

> **Note:** This task depends on P2-S5-03 (process state), which has a higher number. Execution order is governed by the `Depends on:` field, not task numbering. Process state must exist before gates can reference it.

**What:** Move `domain/process_gates.rs` (548 lines) to engine: 5 gates evaluating session process state.

**Files:**
- MOVE: `app/src-tauri/src/domain/process_gates.rs` -> `libs/engine/src/workflow/gates.rs`
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `evaluate_process_gates()` available from `orqa_engine::workflow::gates`
- [ ] All 5 gates work: `understand-first`, `docs-before-code`, `plan-before-build`, `evidence-before-done`, `learn-after-doing`
- [ ] All 10+ tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all 5 gates are present
- All tests pass

**Depends on:** P2-S5-03 (process state must be in engine for gates to reference)

---

#### TASK P2-S5-03: Move process state to engine crate

**What:** Move `domain/process_state.rs` (287 lines) to engine: session process state tracking.

**Files:**
- MOVE: `app/src-tauri/src/domain/process_state.rs` -> `libs/engine/src/workflow/state.rs`
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `SessionProcessState`, `ToolCallRecord`, and all aggregation methods available from `orqa_engine::workflow::state`
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all aggregation methods present
- Verify types match what process gates expect

**Depends on:** P2-S1-03 (engine types)

---

#### TASK P2-S5-04: Move workflow tracker to engine crate

**What:** Move `domain/workflow_tracker.rs` (406 lines) to engine: session activity tracking.

**Files:**
- MOVE: `app/src-tauri/src/domain/workflow_tracker.rs` -> `libs/engine/src/workflow/tracker.rs`
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `WorkflowTracker` with all 5 tracking methods available from `orqa_engine::workflow::tracker`
- [ ] All 5 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all tracking methods present
- All tests pass

**Depends on:** P2-S1-03 (engine types)

---

### Step 6: Plugin Crate

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each module move must be validated against ARCHITECTURE.md to confirm the target location and API surface are correct.

#### TASK P2-S6-01: Move plugin manifest to engine

**What:** Move `plugins/manifest.rs` (145 lines) to engine: `PluginManifest`, `read_manifest()`, `validate_manifest()`.

**Files:**
- MOVE: `app/src-tauri/src/plugins/manifest.rs` -> `libs/engine/src/plugin/manifest.rs`
- CREATE: `libs/engine/src/plugin/mod.rs`
- MODIFY: `libs/engine/src/lib.rs` — add `plugin` module
- MODIFY: App — update imports

**Acceptance Criteria:**
- [ ] `PluginManifest`, `read_manifest()`, `validate_manifest()` available from `orqa_engine::plugin::manifest`
- [ ] All 3 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- All tests pass
- No app plugin logic references local manifest types

**Depends on:** P2-S1-01 (engine crate exists)

---

#### TASK P2-S6-02: Move plugin discovery to engine

**What:** Move `plugins/discovery.rs` (70 lines) to engine: `DiscoveredPlugin`, `scan_plugins()`.

**Files:**
- MOVE: `app/src-tauri/src/plugins/discovery.rs` -> `libs/engine/src/plugin/discovery.rs`
- MODIFY: App — update imports

**Acceptance Criteria:**
- [ ] `DiscoveredPlugin`, `scan_plugins()` available from `orqa_engine::plugin::discovery`
- [ ] 1 test moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify scan only returns plugins with `installed: true` AND `enabled: true`
- Test passes

**Depends on:** P2-S6-01 (manifest types needed)

---

#### TASK P2-S6-03: Move plugin collision detection to engine

**What:** Move `plugins/collision.rs` (188 lines) to engine: relationship collision detection between plugins.

**Files:**
- MOVE: `app/src-tauri/src/plugins/collision.rs` -> `libs/engine/src/plugin/collision.rs`
- MODIFY: App — update imports

**Acceptance Criteria:**
- [ ] `KeyCollision`, `detect_relationship_collisions()` available from `orqa_engine::plugin::collision`
- [ ] All 4 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify semantic_match logic is correct
- All tests pass

**Depends on:** P2-S6-01 (plugin module exists in engine)

---

#### TASK P2-S6-04: Move plugin installer to engine

**What:** Move `plugins/installer.rs` (317 lines) to engine: local and GitHub plugin installation.

**Files:**
- MOVE: `app/src-tauri/src/plugins/installer.rs` -> `libs/engine/src/plugin/installer.rs`
- MODIFY: `libs/engine/Cargo.toml` — add `reqwest`, `sha2`, `tar`, `flate2` dependencies
- MODIFY: App — update imports

**Acceptance Criteria:**
- [ ] `install_from_path()`, `install_from_github()`, `uninstall()` available from `orqa_engine::plugin::installer`
- [ ] All 3 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify async functions work correctly in engine crate
- Verify SHA256 verification logic is intact
- All tests pass

**Depends on:** P2-S6-03 (collision detection used during install)

---

#### TASK P2-S6-05: Move plugin lockfile to engine

**What:** Move `plugins/lockfile.rs` (92 lines) to engine: lockfile read/write.

**Files:**
- MOVE: `app/src-tauri/src/plugins/lockfile.rs` -> `libs/engine/src/plugin/lockfile.rs`
- MODIFY: App — update imports

**Acceptance Criteria:**
- [ ] `Lockfile`, `LockEntry`, `read_lockfile()`, `write_lockfile()` available from `orqa_engine::plugin::lockfile`
- [ ] All 2 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- All tests pass
- JSON serialization is correct

**Depends on:** P2-S6-01 (plugin module exists)

---

#### TASK P2-S6-06: Move plugin registry to engine

**What:** Move `plugins/registry.rs` (159 lines) to engine: official + community registry fetching with TTL cache.

**Files:**
- MOVE: `app/src-tauri/src/plugins/registry.rs` -> `libs/engine/src/plugin/registry.rs`
- MODIFY: App — update imports

**Acceptance Criteria:**
- [ ] `RegistryEntry`, `RegistryCatalog`, `RegistryCache`, `fetch_registry()` available from `orqa_engine::plugin::registry`
- [ ] All 2 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify 1-hour TTL cache logic is correct
- Verify hardcoded GitHub URLs are preserved (they'll be made configurable later)
- All tests pass

**Depends on:** P2-S6-01 (plugin module exists)

---

#### TASK P2-S6-07: Move hook manager to engine

**What:** Move `hooks/manager.rs` (415 lines) to engine: git hook dispatcher generation from plugin registry.

**Files:**
- MOVE: `app/src-tauri/src/hooks/manager.rs` -> `libs/engine/src/plugin/hooks.rs`
- MODIFY: App — update imports, remove `hooks` module

**Acceptance Criteria:**
- [ ] `generate_hook_dispatchers()` available from `orqa_engine::plugin::hooks`
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify hook generation logic is complete
- No hook logic remains in app

**Depends on:** P2-S6-02 (discovery needed), P2-S6-01 (manifest needed)

---

#### TASK P2-S6-08: Move CLI tool runner to engine

**What:** Move `cli_tools/runner.rs` (299 lines) to engine: runs plugin-registered CLI tools.

**Files:**
- MOVE: `app/src-tauri/src/cli_tools/runner.rs` -> `libs/engine/src/plugin/cli_runner.rs`
- MODIFY: App — update imports, simplify `cli_tools` module

**Acceptance Criteria:**
- [ ] `CliToolRunner` available from `orqa_engine::plugin::cli_runner`
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify tool execution logic is intact
- App's `CliToolState` uses engine type

**Depends on:** P2-S6-01 (plugin module exists)

---

### Step 7: Prompt Pipeline

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each module move must be validated against ARCHITECTURE.md to confirm the target location and API surface are correct.

#### TASK P2-S7-01: Move knowledge injector to engine

**What:** Move `domain/knowledge_injector.rs` (477 lines) to engine: ONNX-based cosine similarity matching for knowledge injection.

**Files:**
- MOVE: `app/src-tauri/src/domain/knowledge_injector.rs` -> `libs/engine/src/prompt/knowledge.rs`
- CREATE: `libs/engine/src/prompt/mod.rs`
- MODIFY: `libs/engine/src/lib.rs` — add `prompt` module
- MODIFY: `libs/engine/Cargo.toml` — ensure `orqa-search` dependency for embedder

**Acceptance Criteria:**
- [ ] `KnowledgeInjector`, `load()`, `match_prompt()` available from `orqa_engine::prompt::knowledge`
- [ ] All 3 tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify ONNX embedding computation works through engine
- Verify cosine similarity threshold (0.3) is preserved
- All tests pass

**Depends on:** P2-S4-01 (search integration — uses embedder)

---

#### TASK P2-S7-02: Move system prompt builder to engine

**What:** Move `domain/system_prompt.rs` (270 lines) to engine: constructs system prompts from rules + knowledge + CLAUDE.md.

**Files:**
- MOVE: `app/src-tauri/src/domain/system_prompt.rs` -> `libs/engine/src/prompt/builder.rs`
- MODIFY: App — remove, update imports

**Acceptance Criteria:**
- [ ] `build_system_prompt()` available from `orqa_engine::prompt::builder`
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify template assembly logic is intact
- Verify section markers are preserved

**Depends on:** P2-S7-01 (knowledge injector in engine — prompt builder may reference it)

---

#### TASK P2-S7-03: Move session title generation to engine

**What:** Move `domain/session_title.rs` (116 lines) to engine. This requires abstracting sidecar communication since it calls sidecar for summary.

**Files:**
- MOVE: `app/src-tauri/src/domain/session_title.rs` -> `libs/engine/src/prompt/session_title.rs`
- CREATE: `libs/engine/src/traits/sidecar.rs` — `SidecarClient` trait with `generate_summary()` method
- MODIFY: App — implement `SidecarClient` trait on `SidecarManager`, update imports

**Acceptance Criteria:**
- [ ] `auto_title_session()` available from engine, takes a `&dyn SidecarClient` parameter
- [ ] `SidecarClient` trait defined in `orqa_engine::traits`
- [ ] App implements `SidecarClient` for `SidecarManager`
- [ ] Fallback logic (first user message truncated to 50 chars) preserved
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify trait abstraction doesn't leak Tauri types
- Verify fallback logic is preserved

**Depends on:** P2-S1-02 (traits module exists)

---

### Step 8: Agent Crate (placeholder for future generation)

#### TASK P2-S8-01: Create agent module structure in engine

**What:** Create the module structure for agent generation. This is a placeholder — the actual generation pipeline will be built in Phase 4 (Connector Cleanup). For now, define the types and interfaces.

**Files:**
- CREATE: `libs/engine/src/agent/mod.rs`
- CREATE: `libs/engine/src/agent/types.rs` — `BaseRole`, `AgentSpec`, `TaskAgent` types
- MODIFY: `libs/engine/src/lib.rs` — add `agent` module

**Acceptance Criteria:**
- [ ] `BaseRole` enum with variants matching ARCHITECTURE.md authoritative role list
- [ ] `AgentSpec` struct: role, tool_access, knowledge_refs, task_description
- [ ] `TaskAgent` struct: spec + generated prompt
- [ ] Types derive appropriate traits
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] No generation logic yet — types only

**Reviewer Checks:**
- Verify role list matches ARCHITECTURE.md
- Verify types are sufficient for the generation pipeline to be built against
- No logic, only types

**Depends on:** P2-S1-01 (engine crate exists)

---

### Step 9: Stream Loop Abstraction

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each module move must be validated against ARCHITECTURE.md to confirm the target location and API surface are correct.

#### TASK P2-S9-01: Design sidecar communication trait

**What:** Design the trait abstraction that allows the stream loop to communicate with sidecars without depending on Tauri-specific types. This is a DESIGN task — write the trait definitions but do NOT move stream_loop.rs yet.

**Files:**
- CREATE: `libs/engine/src/traits/sidecar.rs` (extend if exists from P2-S7-03)
- ADD traits: `SidecarTransport` (send/receive messages), `EventEmitter` (emit events to frontend or other consumers)

**Acceptance Criteria:**
- [ ] `SidecarTransport` trait: `send_message()`, `receive_stream()` methods
- [ ] `EventEmitter` trait: `emit_event()` for stream events, `emit_error()` for errors
- [ ] Traits are generic enough for Tauri (channels), CLI (stdout), MCP (IPC)
- [ ] `cargo build -p orqa-engine` succeeds
- [ ] Design documented as comments in the trait file

**Reviewer Checks:**
- Verify traits don't depend on Tauri types
- Verify traits could be implemented for CLI, MCP, and app contexts
- Review design comments for soundness

**Depends on:** P2-S1-02 (traits module exists)

---

#### TASK P2-S9-02: Move stream loop to engine with trait-based abstraction

**What:** Move `domain/stream_loop.rs` (1042 lines) to engine, replacing direct sidecar and Tauri event dependencies with the trait abstractions.

**Files:**
- MOVE: `app/src-tauri/src/domain/stream_loop.rs` -> `libs/engine/src/streaming/loop.rs` (renamed to avoid keyword)
- CREATE: `libs/engine/src/streaming/mod.rs`
- MODIFY: App — implement `SidecarTransport` and `EventEmitter` for Tauri app, update stream_commands.rs
- MODIFY: `libs/engine/src/lib.rs` — add `streaming` module

**Acceptance Criteria:**
- [ ] Stream loop available from `orqa_engine::streaming`
- [ ] Stream loop uses `dyn SidecarTransport` and `dyn EventEmitter` instead of concrete types
- [ ] All 5+ tests moved and passing
- [ ] `cargo build/test` succeeds for both engine and app
- [ ] App provides trait implementations that preserve current behavior

**Reviewer Checks:**
- Verify no Tauri imports in engine streaming module
- Verify trait implementations in app preserve all current behavior
- Verify enforcement integration still works within stream loop
- All tests pass

**Depends on:** P2-S9-01 (trait design), P2-S3-01 (enforcement engine — used in stream loop)

---

#### TASK P2-S9-03: Move tool executor to engine with trait-based abstraction

**What:** Move `domain/tool_executor.rs` (1140 lines) to engine. The tool executor implements 11 tools and uses enforcement hooks — requires abstracting tool execution environment.

**Files:**
- MOVE: `app/src-tauri/src/domain/tool_executor.rs` -> `libs/engine/src/streaming/tools.rs`
- CREATE: `libs/engine/src/traits/executor.rs` — `ToolExecutionContext` trait (filesystem access, shell access, search access)
- MODIFY: App — implement `ToolExecutionContext` for app state

**Acceptance Criteria:**
- [ ] All 11 tool implementations available from `orqa_engine::streaming::tools`
- [ ] `ToolExecutionContext` trait abstracts filesystem and shell access
- [ ] Enforcement hooks (`evaluate_file`, `evaluate_bash`) work through engine enforcement module
- [ ] All 8+ tests moved and passing
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify all 11 tools present and functional
- Verify enforcement integration is correct
- Verify no Tauri types in engine
- Verify `MAX_TOOL_OUTPUT_CHARS` (100,000) is preserved
- All tests pass

**Depends on:** P2-S9-02 (stream loop — tool executor is called from stream loop), P2-S3-01 (enforcement)

---

### Step 10: Absorb `app/tools/` Engine Logic

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each absorption must validate the JS logic against existing engine enforcement capabilities before deleting.

#### TASK P2-S10-01: Migrate `verify-pipeline-integrity.mjs` logic to engine

**What:** The JavaScript file `app/tools/verify-pipeline-integrity.mjs` implements governance enforcement logic that should be in the engine. Analyze its logic and ensure equivalent functionality exists in the engine's enforcement module (either already covered by moved Rust code or needs new implementation).

**Files:**
- READ: `app/tools/verify-pipeline-integrity.mjs`
- READ: `libs/engine/src/enforcement/` (all files)
- MODIFY (if needed): `libs/engine/src/enforcement/` — add any missing checks
- DELETE: `app/tools/verify-pipeline-integrity.mjs` (if fully absorbed)

**Acceptance Criteria:**
- [ ] Every validation check in `verify-pipeline-integrity.mjs` has an equivalent in the engine
- [ ] If any checks are NOT covered by existing engine code, they are implemented in the engine
- [ ] `app/tools/verify-pipeline-integrity.mjs` is deleted
- [ ] Pre-commit hook updated to call engine equivalent (via `orqa validate` CLI)
- [ ] `cargo build/test` succeeds

**Reviewer Checks:**
- Side-by-side comparison of JS checks vs engine checks — every JS check must have an engine equivalent
- Deleted JS file is not referenced anywhere
- Pre-commit hook still works

**Depends on:** P2-S3-01 through P2-S3-04 (enforcement fully in engine)

---

#### TASK P2-S10-02: Migrate `lint-relationships.mjs` logic to engine

**What:** `app/tools/lint-relationships.mjs` implements relationship validation. Ensure equivalent functionality exists in the engine (likely in `orqa-validation` already).

**Files:**
- READ: `app/tools/lint-relationships.mjs` (if it exists)
- READ: `libs/validation/src/checks/` — existing relationship validation
- MODIFY (if needed): Engine enforcement or validation module
- DELETE: `app/tools/lint-relationships.mjs`

**Acceptance Criteria:**
- [ ] Every relationship validation check has an engine equivalent
- [ ] JS file deleted
- [ ] `cargo build/test` succeeds

**Reviewer Checks:**
- Verify all relationship checks covered
- No dangling references to deleted file

**Depends on:** P2-S3-01 (enforcement in engine), P2-S2-10 (validation integrated)

---

#### TASK P2-S10-03: Migrate `verify-installed-content.mjs` logic to engine

**What:** `app/tools/verify-installed-content.mjs` verifies plugin content installation. Ensure equivalent functionality exists in the engine plugin module.

**Files:**
- READ: `app/tools/verify-installed-content.mjs` (if it exists — filename from gap analysis listing)
- MODIFY (if needed): `libs/engine/src/plugin/` — add content verification
- DELETE: The JS file

**Acceptance Criteria:**
- [ ] Plugin content verification available from engine plugin module
- [ ] JS file deleted (or confirmed not to exist — the file listed in gap analysis may be at a different exact path)
- [ ] `cargo build/test` succeeds

**Reviewer Checks:**
- Verify content verification logic is complete in engine
- No dangling references

**Depends on:** P2-S6-01 through P2-S6-08 (plugin system in engine)

---

### Step 11: File-Based Repos Migration

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each module move must be validated against ARCHITECTURE.md to confirm the target location and API surface are correct.

#### TASK P2-S11-01: Move lesson repo to engine

**What:** Move `repo/lesson_repo.rs` (371 lines) to engine. This is file-based lesson storage.

**Files:**
- MOVE: `app/src-tauri/src/repo/lesson_repo.rs` -> `libs/engine/src/lesson/store.rs`
- IMPLEMENT: `LessonStore` trait on `FileLessonStore`
- MODIFY: App — update imports, use trait

**Acceptance Criteria:**
- [ ] `FileLessonStore` implements `LessonStore` trait
- [ ] All lesson repo functionality preserved
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify trait implementation is complete
- Verify file I/O logic matches original

**Depends on:** P2-S1-02 (traits), P2-S2-08 (lesson types in engine)

---

#### TASK P2-S11-02: Move project settings repo to engine

**What:** Move `repo/project_settings_repo.rs` (124 lines) to engine. File-based project.json I/O.

**Files:**
- MOVE: `app/src-tauri/src/repo/project_settings_repo.rs` -> `libs/engine/src/project/store.rs`
- IMPLEMENT: `ProjectSettingsStore` trait on `FileProjectSettingsStore`
- MODIFY: App — update imports

**Acceptance Criteria:**
- [ ] `FileProjectSettingsStore` implements `ProjectSettingsStore` trait
- [ ] `cargo build/test` succeeds for both

**Reviewer Checks:**
- Verify trait implementation is complete
- Verify JSON serialization is correct

**Depends on:** P2-S1-02 (traits), P2-S2-07 (project settings types in engine)

---

### Step 12: Final Engine Verification

#### TASK P2-S12-01: Full engine crate compilation and test verification

**What:** Run a complete build and test suite for the engine crate and all consumers to verify the extraction is complete and correct.

**Files:**
- RUN: `cargo build --workspace`
- RUN: `cargo test --workspace`
- RUN: `cargo clippy --workspace`
- VERIFY: Engine crate has all modules listed in ARCHITECTURE.md gap analysis

**Acceptance Criteria:**
- [ ] `cargo build --workspace` succeeds with zero errors
- [ ] `cargo test --workspace` passes all tests
- [ ] `cargo clippy --workspace` passes (or only has pre-existing warnings)
- [ ] Engine crate contains modules: `types`, `traits`, `artifact`, `enforcement`, `workflow`, `search`, `graph`, `validation`, `metrics`, `prompt`, `plugin`, `agent`, `streaming`, `platform`, `config`, `paths`, `project`, `lesson`, `utils`
- [ ] App crate contains ONLY: commands, sidecar, servers, db, state, watcher, logging, startup, error, setup, and SQLite repos
- [ ] No business logic remains in app domain modules (only re-exports and thin wrappers)

**Reviewer Checks:**
- Run `cargo build --workspace` — must succeed
- Run `cargo test --workspace` — must pass
- Enumerate engine modules — verify against list
- Enumerate remaining app domain modules — verify only glue/bridge code remains
- Check for any `domain/` files in app that contain logic (not re-exports)

**Depends on:** ALL Phase 2 tasks

---

#### TASK P2-S12-02: Update `orqa-mcp-server` to depend on `orqa-engine`

**What:** Update the MCP server crate to depend on `orqa-engine` instead of directly on `orqa-validation` and `orqa-search`.

**Files:**
- MODIFY: `libs/mcp-server/Cargo.toml` — replace `orqa-validation` and `orqa-search` deps with `orqa-engine`
- MODIFY: `libs/mcp-server/src/` — update all imports to use `orqa_engine::*`

**Acceptance Criteria:**
- [ ] MCP server depends only on `orqa-engine` (not individual crates directly)
- [ ] `cargo build -p orqa-mcp-server` succeeds
- [ ] `cargo test -p orqa-mcp-server` passes

**Reviewer Checks:**
- Verify Cargo.toml has only `orqa-engine` dependency (for engine functionality)
- Verify imports use `orqa_engine::` prefix
- Build and test pass

**Depends on:** P2-S12-01 (engine verified)

---

#### TASK P2-S12-03: Update `orqa-lsp-server` to depend on `orqa-engine`

**What:** Update the LSP server crate to depend on `orqa-engine` instead of directly on `orqa-validation` and `orqa-search`.

**Files:**
- MODIFY: `libs/lsp-server/Cargo.toml` — replace deps with `orqa-engine`
- MODIFY: `libs/lsp-server/src/` — update all imports

**Acceptance Criteria:**
- [ ] LSP server depends only on `orqa-engine`
- [ ] `cargo build -p orqa-lsp-server` succeeds
- [ ] `cargo test -p orqa-lsp-server` passes

**Reviewer Checks:**
- Same as P2-S12-02 but for LSP server

**Depends on:** P2-S12-01 (engine verified)

---

## Phase 3: Daemon

### Step 1: System Tray + Process Lifecycle

#### TASK P3-S1-01: Create daemon crate with system tray

**What:** Create the standalone daemon binary crate with system tray icon and context menu.

**Files:**
- CREATE: `daemon/Cargo.toml`
- CREATE: `daemon/src/main.rs` — binary entry point
- CREATE: `daemon/src/tray.rs` — system tray with context menu (process status, app launch, quit)
- MODIFY: Root `Cargo.toml` — add `daemon` to workspace members

**Acceptance Criteria:**
- [ ] `daemon/` directory exists with valid Cargo crate
- [ ] Daemon binary starts and shows system tray icon
- [ ] Context menu shows: status indicator, "Open App" action, "Quit" action
- [ ] Daemon runs as a background process (not tied to terminal)
- [ ] `cargo build -p orqa-daemon` succeeds
- [ ] Daemon depends on `orqa-engine`

**Reviewer Checks:**
- Binary starts without errors
- System tray is visible
- Context menu items work
- Daemon persists after closing terminal (on supported platforms)

**Depends on:** P2-S12-01 (engine crate complete)

---

#### TASK P3-S1-02: Implement daemon process management

**What:** Implement PID file management, single-instance enforcement, graceful shutdown, and health endpoint.

**Files:**
- CREATE: `daemon/src/process.rs` — PID file at `.state/daemon.pid`, lock management
- CREATE: `daemon/src/health.rs` — HTTP health endpoint at configured port
- MODIFY: `daemon/src/main.rs` — integrate process management

**Acceptance Criteria:**
- [ ] PID file written to `.state/daemon.pid` on startup
- [ ] Second daemon instance detects existing PID and exits with error
- [ ] Graceful shutdown on SIGTERM/SIGINT: releases PID file, stops watchers, closes connections
- [ ] Health endpoint responds at `http://localhost:{port}/health` with JSON `{"status": "ok", "uptime_seconds": N}`
- [ ] Port configurable via `ORQA_PORT_BASE` env var (default 9120, daemon uses base+0)

**Reviewer Checks:**
- Start daemon, verify PID file exists
- Try starting second daemon — must fail
- Kill daemon with SIGTERM — PID file must be cleaned up
- Curl health endpoint — verify response

**Depends on:** P3-S1-01 (daemon crate exists)

---

### Step 2: File Watchers

#### TASK P3-S2-01: Implement file watchers for plugin/rule/workflow changes

**What:** Port the file watcher concept from `app/src-tauri/src/watcher.rs` to the daemon, watching for changes to plugins, enforcement rules, and workflow definitions.

**Files:**
- CREATE: `daemon/src/watcher.rs` — file system watcher using `notify` crate
- MODIFY: `daemon/src/main.rs` — start watcher on daemon startup

**Acceptance Criteria:**
- [ ] Watches `.orqa/` directory for changes (500ms debounce)
- [ ] Watches `plugins/` directory for plugin changes
- [ ] Ignores `.git/`, `node_modules/`, `target/`
- [ ] On change: triggers relevant engine re-evaluation (graph rebuild, enforcement reload, etc.)
- [ ] Logs watched events at debug level
- [ ] `cargo build -p orqa-daemon` succeeds

**Reviewer Checks:**
- Verify debounce timing
- Verify ignore patterns
- Verify change triggers appropriate engine actions
- Verify no memory leaks from watcher threads

**Depends on:** P3-S1-01 (daemon crate exists)

---

### Step 3: MCP Server Integration

#### TASK P3-S3-01: Integrate MCP server into daemon

**What:** The existing `orqa-mcp-server` crate runs as a standalone binary. Integrate it into the daemon so the daemon can spawn and manage the MCP server.

**Files:**
- MODIFY: `daemon/src/main.rs` — add MCP server startup
- CREATE: `daemon/src/mcp.rs` — MCP server lifecycle (start, stop, restart)
- MODIFY: `daemon/Cargo.toml` — add `orqa-mcp-server` or `orqa-engine` dependency for MCP tools

**Acceptance Criteria:**
- [ ] MCP server starts as part of daemon startup
- [ ] MCP server consumes engine crate for all business logic
- [ ] MCP server is accessible via stdio transport (for Claude Code) or IPC
- [ ] MCP tools are registered and functional (artifact read/write, graph queries, search, etc.)
- [ ] System tray shows MCP server status
- [ ] `cargo build -p orqa-daemon` succeeds

**Reviewer Checks:**
- Verify MCP server starts and responds to tool calls
- Verify it uses engine crate APIs (not duplicated logic)
- Verify system tray reflects MCP status

**Depends on:** P3-S1-02 (daemon process management), P2-S12-02 (MCP server uses engine)

---

### Step 4: LSP Server Integration

#### TASK P3-S4-01: Integrate LSP server into daemon

**What:** Integrate the existing `orqa-lsp-server` crate into the daemon for artifact validation and diagnostics.

**Files:**
- CREATE: `daemon/src/lsp.rs` — LSP server lifecycle
- MODIFY: `daemon/src/main.rs` — add LSP server startup
- MODIFY: `daemon/Cargo.toml` — add dependencies

**Acceptance Criteria:**
- [ ] LSP server starts as part of daemon startup
- [ ] LSP server validates against composed schema (from engine)
- [ ] Diagnostics are published for artifact validation errors
- [ ] System tray shows LSP server status
- [ ] `cargo build -p orqa-daemon` succeeds

**Reviewer Checks:**
- Verify LSP server starts and publishes diagnostics
- Verify it validates against the composed schema
- Verify diagnostics appear in editors that support LSP

**Depends on:** P3-S1-02 (daemon process management), P2-S12-03 (LSP server uses engine)

---

### Step 5: Unified Logging

#### TASK P3-S5-01: Implement unified daemon logging

**What:** Set up structured logging for the daemon that covers all subsystems (watcher, MCP, LSP, engine operations).

**Files:**
- CREATE: `daemon/src/logging.rs` — structured logging setup
- MODIFY: `daemon/src/main.rs` — initialize logging early

**Acceptance Criteria:**
- [ ] Structured logging (JSON format) to `.state/daemon.log`
- [ ] Log rotation or size limit (e.g., 10MB max, rotate to `.state/daemon.log.1`)
- [ ] Log levels configurable via `RUST_LOG` env var
- [ ] All subsystem events are tagged with subsystem name (mcp, lsp, watcher, engine)
- [ ] Console output for interactive use (colored, human-readable)
- [ ] `cargo build -p orqa-daemon` succeeds

**Reviewer Checks:**
- Start daemon, perform operations, check log file format
- Verify structured JSON format
- Verify subsystem tagging
- Verify log rotation works

**Depends on:** P3-S1-01 (daemon crate exists)

---

### Step 6: Daemon CLI Integration

#### TASK P3-S6-01: Add daemon management to `orqa` CLI

**What:** Add CLI commands for managing the daemon: `orqa daemon start`, `orqa daemon stop`, `orqa daemon status`, `orqa daemon restart`.

**Files:**
- MODIFY: `libs/cli/src/commands/daemon.ts` — add start/stop/status/restart subcommands
- MODIFY: `libs/cli/src/lib/daemon-client.ts` — health check, PID file reading

**Acceptance Criteria:**
- [ ] `orqa daemon start` starts the daemon (if not already running)
- [ ] `orqa daemon stop` sends SIGTERM to daemon PID
- [ ] `orqa daemon status` reports: running/stopped, PID, uptime, MCP status, LSP status
- [ ] `orqa daemon restart` stops then starts
- [ ] CLI reads PID from `.state/daemon.pid`
- [ ] CLI calls health endpoint to verify daemon is responsive

**Reviewer Checks:**
- Run each command and verify behavior
- Verify error handling (daemon not running, stale PID file, port conflict)
- Verify status output includes all subsystem states

**Depends on:** P3-S1-02 (daemon process management with PID file and health endpoint)

---

### Step 7: Full Daemon Verification

#### TASK P3-S7-01: End-to-end daemon verification

**What:** Run a comprehensive verification of the daemon: startup, all subsystems, file watching, MCP tool calls, LSP diagnostics, graceful shutdown.

**Files:**
- RUN: `cargo build -p orqa-daemon`
- RUN: Start daemon
- TEST: MCP tool calls (artifact read, graph query, search)
- TEST: LSP diagnostics (open a malformed artifact, verify diagnostic)
- TEST: File watcher (modify an artifact, verify daemon re-evaluates)
- TEST: CLI management (`orqa daemon status/stop/start`)
- TEST: Graceful shutdown

**Acceptance Criteria:**
- [ ] Daemon starts cleanly with system tray
- [ ] MCP server responds to at least 3 tool calls correctly
- [ ] LSP server publishes diagnostics for artifact validation errors
- [ ] File watcher detects changes and triggers re-evaluation
- [ ] `orqa daemon status` reports all subsystems running
- [ ] `orqa daemon stop` shuts down gracefully (PID file removed, ports released)
- [ ] `cargo test --workspace` still passes (no regressions)

**Reviewer Checks:**
- Observe each test step
- Verify no error messages in daemon log
- Verify clean shutdown
- Verify workspace tests pass

**Depends on:** ALL Phase 3 tasks

---

### Step 13: CLI TypeScript Engine Logic Review

> **Note:** ARCHITECTURE.md Phase 2 says "Extract business logic from Tauri backend **and CLI** into Rust library crates." The CLI (`libs/cli/src/lib/`) contains 28 files with engine-level business logic (workflow resolution, prompt pipeline, knowledge retrieval, gate engine, agent spawner, budget enforcement, etc.). These are NOT blind migration tasks -- each file must be reviewed against the architecture to determine: keep in TypeScript engine, adapt for Rust engine consumption, or drop if duplicated by Rust extraction.

#### TASK P2-S13-01: Review CLI engine files -- workflow and prompt domain

**What:** Review the following CLI engine files against ARCHITECTURE.md to determine their disposition. For each file, document: (a) what engine capability it implements, (b) whether the Rust engine extraction (Steps 1-12) already covers this capability, (c) recommended action (keep as TypeScript engine, adapt to consume Rust engine, or drop as duplicate).

**Review against architecture -> keep/adapt/drop. Never blind copy.**

**Files to review:**
- `libs/cli/src/lib/workflow-engine.ts` -- workflow resolution
- `libs/cli/src/lib/workflow-resolver.ts` -- workflow step resolution
- `libs/cli/src/lib/prompt-pipeline.ts` -- prompt generation pipeline
- `libs/cli/src/lib/prompt-registry.ts` -- prompt template registry
- `libs/cli/src/lib/knowledge-retrieval.ts` -- knowledge injection for prompts
- `libs/cli/src/lib/gate-engine.ts` -- process gate evaluation
- `libs/cli/src/lib/agent-spawner.ts` -- agent delegation and spawning
- `libs/cli/src/lib/budget-enforcer.ts` -- token budget enforcement
- `libs/cli/src/lib/token-tracker.ts` -- token usage tracking
- `libs/cli/src/lib/validation-engine.ts` -- artifact validation
- `libs/cli/src/lib/enforcement-log.ts` -- enforcement event logging

**Acceptance Criteria:**
- [ ] Each file has a documented disposition: keep / adapt / drop
- [ ] For "keep" files: rationale explains why TypeScript implementation is needed alongside Rust
- [ ] For "adapt" files: description of how the file should be changed to consume Rust engine APIs
- [ ] For "drop" files: confirmation that equivalent functionality exists in Rust engine (with specific module reference)
- [ ] Findings written to `.state/team/*/task-*.md`
- [ ] No files are modified in this task -- review only

**Reviewer Checks:**
- Verify every file has a disposition
- Verify dispositions are consistent with ARCHITECTURE.md Phase 2 goals
- Verify "drop" files truly have Rust equivalents

**Depends on:** P2-S12-01 (engine extraction complete -- need to know what Rust covers before reviewing TypeScript)

---

#### TASK P2-S13-02: Review CLI engine files -- graph and plugin domain

**What:** Review the following CLI engine files against ARCHITECTURE.md. Same approach as P2-S13-01.

**Review against architecture -> keep/adapt/drop. Never blind copy.**

**Files to review:**
- `libs/cli/src/lib/graph.ts` -- graph browsing and queries
- `libs/cli/src/lib/manifest.ts` -- plugin manifest reading
- `libs/cli/src/lib/registry.ts` -- plugin registry access
- `libs/cli/src/lib/installer.ts` -- plugin installation
- `libs/cli/src/lib/lockfile.ts` -- plugin lockfile management
- `libs/cli/src/lib/content-lifecycle.ts` -- plugin content lifecycle
- `libs/cli/src/lib/injector-config.ts` -- injector configuration
- `libs/cli/src/lib/agent-file-generator.ts` -- agent file generation
- `libs/cli/src/lib/config-generator.ts` -- config file generation

**Acceptance Criteria:**
- [ ] Each file has a documented disposition: keep / adapt / drop
- [ ] For "keep" files: rationale explains why TypeScript implementation is needed
- [ ] For "adapt" files: description of required changes
- [ ] For "drop" files: confirmation of Rust equivalent
- [ ] Findings written to `.state/team/*/task-*.md`
- [ ] No files are modified in this task -- review only

**Reviewer Checks:**
- Same as P2-S13-01

**Depends on:** P2-S12-01 (engine extraction complete)

---

#### TASK P2-S13-03: Review CLI engine files -- infrastructure and utilities

**What:** Review the following CLI engine files against ARCHITECTURE.md. Same approach as P2-S13-01.

**Review against architecture -> keep/adapt/drop. Never blind copy.**

**Files to review:**
- `libs/cli/src/lib/frontmatter.ts` -- frontmatter parsing
- `libs/cli/src/lib/daemon-client.ts` -- daemon communication
- `libs/cli/src/lib/ports.ts` -- port management
- `libs/cli/src/lib/root.ts` -- project root detection
- `libs/cli/src/lib/symlink.ts` -- symlink management
- `libs/cli/src/lib/license.ts` -- license management
- `libs/cli/src/lib/readme.ts` -- readme generation
- `libs/cli/src/lib/version-sync.ts` -- version synchronization

**Acceptance Criteria:**
- [ ] Each file has a documented disposition: keep / adapt / drop
- [ ] Infrastructure/utility files may correctly stay in CLI (not engine logic)
- [ ] Findings written to `.state/team/*/task-*.md`
- [ ] No files are modified in this task -- review only

**Reviewer Checks:**
- Verify infrastructure files are correctly distinguished from engine logic
- Verify dispositions are consistent

**Depends on:** P2-S12-01 (engine extraction complete)

---

#### TASK P2-S13-04: Execute CLI engine file dispositions

**What:** Based on the review findings from P2-S13-01 through P2-S13-03, execute the recommended dispositions:
- **Drop** files: delete from `libs/cli/src/lib/`, update `libs/cli/src/index.ts` exports
- **Adapt** files: modify to consume Rust engine APIs via daemon client or FFI
- **Keep** files: no changes needed (document as intentionally TypeScript)

**Review against architecture -> keep/adapt/drop. Never blind copy.**

**Files:**
- `libs/cli/src/lib/` -- files identified for drop/adapt in review tasks
- `libs/cli/src/index.ts` -- update exports to remove dropped modules
- `libs/cli/package.json` -- update if dependencies change

**Acceptance Criteria:**
- [ ] All "drop" files are deleted
- [ ] All "adapt" files are modified per review recommendations
- [ ] `libs/cli/src/index.ts` exports updated (no dangling references)
- [ ] `npx tsc --noEmit` in `libs/cli/` succeeds
- [ ] CLI commands still work after changes
- [ ] No engine-level business logic remains in CLI without documented justification

**Reviewer Checks:**
- Verify every disposition from P2-S13-01..03 was executed
- Verify `npx tsc` succeeds
- Verify CLI is functional

**Depends on:** P2-S13-01, P2-S13-02, P2-S13-03 (review findings needed)

---

### Step 14: app/.githooks/ Absorption Review

> **Note:** ARCHITECTURE.md Phase 2 item 3 says "Enforcement crate -- rule evaluation, artifact validation **(absorb app/.githooks/ logic)**". The 15 hand-written validation scripts in `app/.githooks/` implement the operational enforcement suite. These must be reviewed against the engine enforcement crate to determine absorption strategy.

#### TASK P2-S14-01: Review app/.githooks/ scripts against engine enforcement

**What:** Review each of the 15 `.githooks/` scripts against the engine enforcement module to determine: (a) whether the Rust enforcement crate already covers the check, (b) whether the check should be absorbed into the engine, (c) whether the check should remain as a plugin-generated hook script.

**Review against architecture -> keep/adapt/drop. Never blind copy.**

**Files to review:**
- `app/.githooks/pre-commit` -- orchestrator script
- `app/.githooks/validate-schema.mjs` -- schema validation
- `app/.githooks/validate-relationships.mjs` -- relationship validation
- `app/.githooks/validate-status-transitions.mjs` -- status transition validation
- `app/.githooks/validate-pillar-alignment.mjs` -- pillar alignment checks
- `app/.githooks/validate-config-disk.mjs` -- config/disk consistency
- `app/.githooks/validate-historical-preservation.sh` -- history preservation
- `app/.githooks/validate-epic-readiness.sh` -- epic readiness checks
- `app/.githooks/validate-task-deps.sh` -- task dependency validation
- `app/.githooks/validate-lint-suppressions.mjs` -- lint suppression audit
- `app/.githooks/validate-plugin-sources.mjs` -- plugin source protection
- `app/.githooks/validate-artifacts.sh` -- artifact validation
- `app/.githooks/scan-stubs.sh` -- stub scanning
- `app/.githooks/autolink-artifacts.mjs` -- auto-linking
- `app/.githooks/collect-rule-overrides.mjs` -- rule override collection

**Acceptance Criteria:**
- [ ] Each script has a documented disposition: absorb-into-engine / keep-as-hook / drop-as-redundant
- [ ] For "absorb" scripts: mapping to specific engine enforcement module/function
- [ ] For "keep" scripts: explanation of why hook-level enforcement is appropriate (not engine-level)
- [ ] For "drop" scripts: confirmation of equivalent engine check
- [ ] Findings identify which checks the `githooks` plugin should generate vs which the engine handles directly
- [ ] Findings written to `.state/team/*/task-*.md`
- [ ] No files are modified in this task -- review only

**Reviewer Checks:**
- Verify every script has a disposition
- Verify dispositions align with ARCHITECTURE.md: "Git hooks -- generated by plugins from engine rules"
- Verify no enforcement logic is silently lost

**Depends on:** P2-S3-01 through P2-S3-04 (enforcement crate must be complete to assess overlap)

---

#### TASK P2-S14-02: Execute app/.githooks/ absorption

**What:** Based on the review findings from P2-S14-01, execute the recommended dispositions:
- **Absorb** scripts: implement equivalent checks in `libs/engine/src/enforcement/`, expose via `orqa validate` or `orqa check`
- **Keep** scripts: document as "stays until githooks plugin generates equivalents"
- **Drop** scripts: delete

**Review against architecture -> keep/adapt/drop. Never blind copy.**

**Files:**
- `app/.githooks/` -- scripts identified for absorption/drop
- `libs/engine/src/enforcement/` -- add absorbed checks
- `libs/cli/src/commands/check.ts` -- expose new engine checks via CLI

**Acceptance Criteria:**
- [ ] All "absorb" checks are implemented in engine enforcement module
- [ ] All "drop" scripts are deleted
- [ ] "Keep" scripts are documented with justification
- [ ] `cargo build/test -p orqa-engine` succeeds
- [ ] Pre-commit hook still provides all enforcement checks (via engine or retained scripts)
- [ ] No enforcement coverage is lost

**Reviewer Checks:**
- Verify all absorbed checks have engine equivalents
- Verify pre-commit enforcement is undiminished
- Verify no scripts are deleted without equivalent coverage

**Depends on:** P2-S14-01 (review findings needed)

---

## Dependency Summary

### Phase 1 Critical Path
```
P1-S1-01 (schema) ──> P1-S4-01 (target plugin)
                  ──> P1-S4-02 (target workflows)
                  ──> P1-S4-03 (target manifests)
P1-S1-02 (validation script) ──> P1-S1-03 (baseline)
P1-S2-01..05 (enforcement configs) ──> P1-S2-06 (git hooks)
P1-S2-06 (git hooks) ──> P1-S3-03 (settings.json)
```

### Phase 2 Critical Path
```
P2-S1-01 (types) ──> P2-S1-02 (traits) ──> P2-S1-03 (wire app)
P2-S1-03 ──> P2-S2-* (graph/artifact)
P2-S1-03 ──> P2-S3-* (enforcement)
P2-S1-03 ──> P2-S5-* (workflow)
P2-S4-01 (search) ──> P2-S7-01 (knowledge)
P2-S7-01 ──> P2-S7-02 (prompt builder)
P2-S3-01 (enforcement) ──> P2-S9-02 (stream loop)
P2-S9-02 ──> P2-S9-03 (tool executor)
ALL ──> P2-S12-01 (verification)
P2-S12-01 ──> P2-S12-02, P2-S12-03 (MCP/LSP update)
P2-S12-01 ──> P2-S13-01..03 (CLI review) ──> P2-S13-04 (CLI execute)
P2-S3-04 ──> P2-S14-01 (githooks review) ──> P2-S14-02 (githooks execute)
```

### Phase 3 Critical Path
```
P2-S12-01 (engine complete) ──> P3-S1-01 (daemon crate)
P3-S1-01 ──> P3-S1-02 (process management)
P3-S1-02 ──> P3-S2-01 (watchers)
P3-S1-02 + P2-S12-02 ──> P3-S3-01 (MCP integration)
P3-S1-02 + P2-S12-03 ──> P3-S4-01 (LSP integration)
P3-S3-01 + P3-S4-01 ──> P3-S6-01 (CLI integration)
ALL ──> P3-S7-01 (verification)
```

### Parallelization Opportunities

**Phase 1 — can run in parallel:**
- P1-S2-01, P1-S2-02, P1-S2-03, P1-S2-04, P1-S2-05 (all independent enforcement configs)
- P1-S3-01, P1-S3-02 (migration CLAUDE.md and agents are independent)
- P1-S4-02, P1-S4-03 (target workflows and manifests are independent after schema)

**Phase 2 — can run in parallel after P2-S1-03:**
- P2-S2-08 (lessons), P2-S2-09 (time utils), P2-S2-10 (validation integration) — independent
- P2-S3-01..04 (enforcement) vs P2-S5-01..04 (workflow) vs P2-S6-01..08 (plugin) — three independent extraction tracks after types are wired
- P2-S4-01 (search integration) — independent of other tracks
- P2-S13-01..03 (CLI review tasks) — can run in parallel after P2-S12-01
- P2-S14-01 (githooks review) — can run in parallel with S13 after enforcement is done

**Phase 3 — limited parallelism:**
- P3-S3-01 (MCP) and P3-S4-01 (LSP) can run in parallel
- P3-S5-01 (logging) can run in parallel with P3-S2-01 (watchers)

---

## Task Count Summary

| Phase | Step | Tasks |
|-------|------|-------|
| **Phase 1** | Step 1: Schema + Validation | 3 |
| **Phase 1** | Step 2: Enforcement Configs | 6 |
| **Phase 1** | Step 3: Migration .claude/ | 3 |
| **Phase 1** | Step 4: Remaining Targets | 3 |
| **Phase 1 Total** | | **15** |
| **Phase 2** | Step 1: Types + Traits | 3 |
| **Phase 2** | Step 2: Graph/Artifact | 10 |
| **Phase 2** | Step 3: Enforcement | 4 |
| **Phase 2** | Step 4: Search | 1 |
| **Phase 2** | Step 5: Workflow | 4 |
| **Phase 2** | Step 6: Plugin | 8 |
| **Phase 2** | Step 7: Prompt Pipeline | 3 |
| **Phase 2** | Step 8: Agent (placeholder) | 1 |
| **Phase 2** | Step 9: Stream Loop | 3 |
| **Phase 2** | Step 10: app/tools/ Absorption | 3 |
| **Phase 2** | Step 11: File-Based Repos | 2 |
| **Phase 2** | Step 12: Verification | 3 |
| **Phase 2** | Step 13: CLI TypeScript Engine Review | 4 |
| **Phase 2** | Step 14: app/.githooks/ Absorption | 2 |
| **Phase 2 Total** | | **51** |
| **Phase 3** | Step 1: System Tray + Process | 2 |
| **Phase 3** | Step 2: File Watchers | 1 |
| **Phase 3** | Step 3: MCP Integration | 1 |
| **Phase 3** | Step 4: LSP Integration | 1 |
| **Phase 3** | Step 5: Logging | 1 |
| **Phase 3** | Step 6: CLI Integration | 1 |
| **Phase 3** | Step 7: Verification | 1 |
| **Phase 3 Total** | | **8** |
| **Grand Total** | | **74** |
