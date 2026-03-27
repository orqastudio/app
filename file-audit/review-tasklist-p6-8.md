# Review: Migration Task List Phases 6-8

**Reviewer:** Reviewer agent
**Date:** 2026-03-26
**Inputs:** migration-tasks-phase6-8.md, ARCHITECTURE.md section 13 (phases 6-8), phase2-01-governance-gaps.md, phase2-05-root-infra-gaps.md, phase2-06-proposed-restructure.md
**Disk state verified:** Yes (Glob/Bash checks against actual files on disk)

---

## Verdict: FAIL

---

## Acceptance Criteria

- [x] AC1: Every specific script named in the root-infra gap analysis has an explicit delete/keep task -- **PASS** (see details below)
- [x] AC2: Every governance gap has a corresponding task -- **PASS** (see details below)
- [ ] AC3: Every directory move has exact current->target paths -- **FAIL** (two ARCHITECTURE.md Phase 8 moves are missing entirely)
- [ ] AC4: Zero tech debt: every deletion is a task, no "clean up later" -- **FAIL** (cleanup policy for `.state/team/` is referenced but not a task; one task based on stale data)
- [x] AC5: .state/ cleanup policy is defined -- **PASS** (tasks 6.11 and 6.12 cover cleanup)
- [x] AC6: Legacy files (validation_stderr.txt, tmp/, WORKING-DOCUMENT.md) have explicit deletion tasks -- **PASS** (tasks 6.6, 6.7, 6.8)
- [x] AC7: Decision splitting (principle vs planning) is a task -- **PASS** (task 6.21 for classification, task 7.11 for directory split)
- [x] AC8: Knowledge categorization into domain subdirs is specified -- **PASS** (tasks 6.16-6.19 for classification, task 7.12 for directory move)
- [x] AC9: Frontmatter standardization (name->title, status, quoting) is covered -- **PASS** (tasks 7.15, 7.16, 7.17)
- [ ] AC10: All tasks are atomic with testable AC -- **FAIL** (several tasks lack testable AC or are too vague)

---

## Issues Found

### CRITICAL: Two ARCHITECTURE.md Phase 8 items have no corresponding tasks

**ARCHITECTURE.md Phase 8 item 4:** "Move CLI to top-level `cli/`"
- There is NO task in Phase 8 for moving `libs/cli/` to a top-level `cli/` directory.
- The proposed restructure document (phase2-06) keeps CLI at `libs/cli/` (line 249, 422), which contradicts ARCHITECTURE.md.
- **Resolution needed:** Either add a task to move CLI, or note that ARCHITECTURE.md should be updated to reflect the proposed restructure's decision to keep CLI in `libs/`.

**ARCHITECTURE.md Phase 8 item 5:** "Move claude-agent-sdk to top-level `sidecars/`"
- Task 8.8 moves it to `plugins/sidecars/claude-agent-sdk/` instead.
- ARCHITECTURE.md says top-level `sidecars/`, proposed restructure (phase2-06 line 206) says `plugins/sidecars/`.
- **Resolution needed:** The task list follows the proposed restructure, not ARCHITECTURE.md. This is a genuine conflict between the two source documents.

**Impact:** If ARCHITECTURE.md is authoritative, 2 directory moves are wrong or missing. If the proposed restructure is authoritative, ARCHITECTURE.md needs updating (and there should be a task for that).

### CRITICAL: ARCHITECTURE.md Phase 8 item 1 wording is contradictory

ARCHITECTURE.md line 861 says: "Move engine crates to `libs/`"
- Task 8.1 moves crates FROM `libs/` TO `engine/crates/` (the opposite direction).
- The proposed restructure (phase2-06) confirms the correct direction is libs/ -> engine/crates/.
- The ARCHITECTURE.md wording appears to be a typo or stale text.
- **This should be flagged as a known ARCHITECTURE.md error and corrected.**

### SIGNIFICANT: Task 6.28 is based on stale gap analysis data

Task 6.28 says to "Delete `integrations/claude-agent-sdk/node_modules/` (vendored dependencies including large binaries)."
- **Actual state:** `node_modules/` exists on disk but is already gitignored and NOT tracked in git (`git ls-files` returns 0 files, `git check-ignore` confirms it's ignored).
- The gap analysis (phase2-05 section 7) describes "vendored node_modules in source" as a significant issue, but the files are not actually committed.
- **Task 6.28 should be revised** to simply verify gitignore coverage and delete the local directory if desired, not presented as removing "committed binary blobs."

### SIGNIFICANT: No task for `.state/team/` cleanup POLICY

ARCHITECTURE.md Phase 6 `.state/ Cleanup` section says:
- "Establish cleanup policy: team findings are ephemeral -- promote valuable content to governance artifacts, then delete"
- "CLI should provide `orqa dev clean-teams [--age <days>]` to prune stale team directories"

Tasks 6.11 and 6.12 handle the immediate cleanup (delete empty dirs, prune stale dirs), but there is NO task for:
1. Establishing the cleanup policy as a documented rule or decision artifact
2. Implementing `orqa dev clean-teams` CLI command

The one-time cleanup is covered; the ongoing policy/tooling is not. This is deferred work without explicit acknowledgment.

### SIGNIFICANT: No task for "ensure documentation and knowledge are sourced from correct plugins"

ARCHITECTURE.md Phase 6 Documentation and Knowledge section says: "Ensure documentation and knowledge are sourced from the correct plugins (not orphaned project copies of plugin content)."

Tasks 6.13-6.19 review content for accuracy, duplication, and frontmatter, but none explicitly check whether a given knowledge/doc artifact is an orphaned project copy of content that should come from a plugin. This is a distinct concern from content accuracy.

### MODERATE: Task 6.12 directory listing is incomplete

Task 6.12 lists "40" team directories but:
- Lists only 39 names + `restructure-findings.md` (which is a file, not a directory)
- Missing from the list: `migration-planning/` (confirmed on disk via `ls -d .state/team/*/`)
- The task says "40+" and instructs scanning all directories, so this is a minor accuracy issue, but the enumeration is wrong.

### MODERATE: Task 6.5 lists scripts that do not exist on disk

Task 6.5 "Keep" list includes:
- `app/tools/verify-links.mjs` -- **does not exist on disk** (Glob returned no match for `app/tools/verify-links*`)

These files may have been removed in prior work or the gap analysis inventoried them incorrectly. The task should be updated to match actual disk state.

**Note:** The root-infra gap analysis (phase2-05 section 12) also mentions `summarize-artifact.mjs`, `verify-installed-content.mjs`, and `lint-relationships.mjs` as existing in `app/tools/`, but none of these exist on disk. The task list correctly omits them from delete/keep lists, but this shows the gap analysis inventory is stale for `app/tools/`.

### MODERATE: Task 8.1 Cargo.toml target is missing `engine/crates/core/`

The proposed restructure (phase2-06) specifies 5 engine crates:
```
engine/crates/core/        -- extracted from app/src-tauri/src/domain/
engine/crates/validation/  -- from libs/validation/
engine/crates/search/      -- from libs/search/
engine/crates/mcp-server/  -- from libs/mcp-server/
engine/crates/lsp-server/  -- from libs/lsp-server/
```

Task 8.1 only moves 4 crates (no `core/`). The extraction of `app/src-tauri/src/domain/` business logic into `engine/crates/core/` is described as "the largest structural change" (~8,000+ lines) in the proposed restructure but has NO task anywhere in Phases 6-8.

The Cargo.toml target in task 8.11 also omits `engine/crates/core`.

This may be intentional (deferred to a later phase) but it is not documented as such.

### MINOR: Task 6.1 lists `scripts/validate-artifacts.mjs` as "Keep"

The file exists (confirmed via Glob), but it is not mentioned in the root-infra gap analysis (phase2-05) which enumerates 11 files in `scripts/`. This means either:
- The file was created after the gap analysis was written, or
- The gap analysis missed it

The task's "keep" decision is correct (the file is active), but this shows the gap analysis is not perfectly aligned with disk state.

### MINOR: Knowledge batch file counts don't match

- Task 6.16 says "~38 files" but lists 29 files in the block
- Task 6.17 says "~28 files" but lists 31 files in the block
- Task 6.18 says "~28 files" but lists 32 files in the block
- Task 6.19 says "~20 files" but lists 22 files in the block

The total across all batches (29+31+32+22 = 114) matches the expected 114 KNOW files, so coverage is complete, but the per-batch approximate counts are misleading.

---

## Criterion-by-Criterion Evidence

### AC1: Scripts — delete/keep coverage

Root-infra gap analysis (phase2-05) names these scripts:

**scripts/ (section 8):**
| Script | Gap Analysis Status | Task Coverage |
|--------|-------------------|---------------|
| `install.sh` | Active | 6.1 Keep |
| `sync-versions.sh` | Active | 6.1 Keep |
| `link-all.sh` | Active | 6.1 Keep |
| `monorepo-merge.sh` | Completed | 6.1 Delete |
| `migrate-artifact-ids.mjs` | Completed | 6.1 Delete |
| `standardise-ids.mjs` | Completed | 6.1 Delete |
| `fix-duplicate-frontmatter-keys.mjs` | Completed | 6.1 Delete |
| `fix-missing-inverses.mjs` | Completed | 6.1 Delete |
| `link-skills-to-docs.mjs` | Completed | 6.1 Delete |
| `id-migration-manifest.json` | Migration output | 6.2 Delete |
| `id-standardise-manifest.json` | Migration output | 6.2 Delete |

**tools/ (section 9):**
| Script | Gap Analysis Status | Task Coverage |
|--------|-------------------|---------------|
| `remove-inverse-relationships.mjs` | Completed | 6.3 Delete |
| `migrate-types.mjs` | Completed | 6.3 Delete |
| `debug/dev.mjs` | Active | 6.3 Keep |
| `debug/dev-dashboard.html` | Active | 6.3 Keep |

**app/scripts/ (section 12):**
| Script | Gap Analysis Status | Task Coverage |
|--------|-------------------|---------------|
| `rebuild-artifacts.mjs` | Completed | 6.4 Delete |
| `migration-manifest.json` | Migration output | 6.4 Delete |
| `link-all-plugins.mjs` | Possibly active | NOT ON DISK -- no task needed |

**app/tools/ (section 12):**
All existing files on disk are covered by task 6.5 (8 delete, 7 keep + lib/).

**Verdict: PASS** -- every script that exists on disk has a delete/keep task.

### AC2: Governance gaps

Every gap from phase2-01-governance-gaps.md has a corresponding task:
- process/ nesting: tasks 7.1-7.5, 7.8
- agents/ directory: tasks 7.6-7.7
- grounding/ directory: task 7.9
- connectors/ in .orqa/: task 7.18
- prompt-registry.json: task 6.27
- schema.composed.json missing: not a Phase 6-8 task (it's an engine feature)
- Decisions not split: tasks 6.21, 7.11
- Knowledge not categorized: tasks 6.16-6.19, 7.12
- Documentation partially categorized: tasks 6.13-6.15, 7.13
- DOC in personas/: task 7.10
- Grounding docs -> knowledge: task 7.9
- Wireframes typed as DOC: task 7.14
- AGENT files legacy: tasks 7.6-7.7
- SKILL.md in knowledge: task 6.20
- name vs title: task 7.15
- Missing status: task 7.16
- Inconsistent quoting: task 7.17
- Knowledge frontmatter gaps: tasks 6.16-6.19

**Verdict: PASS**

### AC3: Directory moves

Phase 7 moves (all correct):
- `.orqa/process/decisions/` -> `.orqa/decisions/` (7.1)
- `.orqa/process/knowledge/` -> `.orqa/knowledge/` (7.2)
- `.orqa/process/rules/` -> `.orqa/rules/` (7.3)
- `.orqa/process/lessons/` -> `.orqa/lessons/` (7.4)
- `.orqa/process/workflows/` -> handled (7.5)
- `.orqa/process/agents/` -> deleted (7.6)
- `.orqa/process/` -> deleted (7.8)

Phase 8 moves:
- `libs/validation/` -> `engine/crates/validation/` (8.1) -- correct
- `libs/search/` -> `engine/crates/search/` (8.1) -- correct
- `libs/mcp-server/` -> `engine/crates/mcp-server/` (8.1) -- correct
- `libs/lsp-server/` -> `engine/crates/lsp-server/` (8.1) -- correct
- Plugin moves (8.4-8.8) -- all have exact paths, correct
- `integrations/claude-agent-sdk/` -> `plugins/sidecars/claude-agent-sdk/` (8.8) -- conflicts with ARCHITECTURE.md
- **MISSING: `libs/cli/` -> `cli/`** (ARCHITECTURE.md item 4)
- **MISSING: `app/src-tauri/src/domain/` -> `engine/crates/core/`** (proposed restructure Phase 1)

**Verdict: FAIL** -- two moves from ARCHITECTURE.md are missing, one from proposed restructure is missing.

### AC5: .state/ cleanup policy

- Task 6.11: delete empty team directories -- covered
- Task 6.12: prune stale team directories -- covered
- Missing: `orqa dev clean-teams` CLI command and documented policy

**Verdict: PASS** (the immediate cleanup is defined; the ongoing policy is a separate concern that could reasonably be a follow-up CLI feature task, not a content migration task)

### AC10: Atomic tasks with testable AC

Most tasks have clear, testable AC. Exceptions:
- Task 6.12 AC "Any valuable unpromoted content is documented in a list before deletion" is subjective
- Task 7.5 AC depends on determining the "correct location" at implementation time -- not fully specified
- Task 7.18 requires runtime assessment of `injector-config.json` usage -- AC depends on investigation results

**Verdict: FAIL** -- tasks 7.5 and 7.18 have investigative AC that cannot be verified without first doing the work.

---

## Summary of Required Fixes

1. **Add tasks or document intentional omission** for ARCHITECTURE.md Phase 8 items 4 (CLI move) and 5 (sidecar location discrepancy)
2. **Add task or document deferral** for `engine/crates/core/` extraction from `app/src-tauri/src/domain/`
3. **Update task 6.28** to reflect actual git state (node_modules already gitignored and untracked)
4. **Update task 6.12** directory listing to include `migration-planning/` and fix `restructure-findings.md` annotation
5. **Update task 6.5** keep list to remove `app/tools/verify-links.mjs` (does not exist)
6. **Fix knowledge batch counts** in tasks 6.16-6.19 to match actual file listings
7. **Add task for ARCHITECTURE.md correction** of Phase 8 item 1 wording ("Move engine crates to `libs/`" should say "to `engine/crates/`")
8. **Consider adding task** for plugin content source verification (orphaned project copies)
9. **Consider adding task** for `.state/team/` cleanup policy documentation and `orqa dev clean-teams` CLI command

---

## Lessons

- Gap analysis documents become stale quickly when work continues between analysis and task creation. Tasks should be verified against disk state, not just gap analysis text.
- When multiple source documents exist (ARCHITECTURE.md, proposed restructure, gap analyses), conflicts between them create ambiguous tasks. The task list should explicitly note which source it follows when sources disagree.
- Tasks that require investigation before defining the target state (7.5, 7.18) should be split into a research task and an implementation task.
