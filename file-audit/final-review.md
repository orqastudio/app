# Final Review: All Targets and Task Lists

**Reviewer:** Reviewer agent
**Date:** 2026-03-26
**Scope:** Schema paths, target plugin, target migration, enforcement configs, task lists, ARCHITECTURE.md consistency, orqa-create skill

---

## Verdict: FAIL

3 issues found. 1 is critical (wrong directory structure in target plugin's governance-steward agent). 2 are moderate (schema/ARCHITECTURE.md misalignment on discovery subdirectories, plugin settings.json denies architecture/ that doesn't exist).

---

## Acceptance Criteria

### 1. Schema paths match ARCHITECTURE.md section 5.1 stage-first structure

- [x] PASS (with caveats noted below)

Every `defaultPath` in `targets/schema.composed.json` uses the stage-first structure:

| Artifact Type | defaultPath | Stage | Matches 5.1? |
|---|---|---|---|
| decision (deprecated) | `.orqa/learning/decisions/` | learning | Yes |
| principle-decision | `.orqa/learning/decisions/` | learning | Yes |
| rule | `.orqa/learning/rules/` | learning | Yes |
| lesson | `.orqa/learning/lessons/` | learning | Yes |
| knowledge | `.orqa/documentation/knowledge/` | documentation | Yes |
| doc | `.orqa/documentation/` | documentation | Yes |
| vision | `.orqa/discovery/vision/` | discovery | Yes |
| pillar | `.orqa/discovery/pillars/` | discovery | Yes |
| persona | `.orqa/discovery/personas/` | discovery | Yes |
| pivot | `.orqa/discovery/pivots/` | discovery | **See caveat 1** |
| discovery-idea | `.orqa/discovery/ideas/` | discovery | Yes |
| discovery-research | `.orqa/discovery/research/` | discovery | Yes |
| discovery-decision | `.orqa/discovery/decisions/` | discovery | **See caveat 2** |
| planning-idea | `.orqa/planning/ideas/` | planning | Yes |
| planning-research | `.orqa/planning/research/` | planning | Yes |
| planning-decision | `.orqa/planning/decisions/` | planning | Yes |
| wireframe | `.orqa/discovery/wireframes/` | discovery | Yes |
| milestone | `.orqa/implementation/milestones/` | implementation | Yes |
| epic | `.orqa/implementation/epics/` | implementation | Yes |
| task | `.orqa/implementation/tasks/` | implementation | Yes |

No `process/`, `delivery/`, or `principles/` paths remain. All paths are stage-first.

**Caveat 1 (MODERATE):** Schema has `pivot` type with `defaultPath: ".orqa/discovery/pivots/"` but ARCHITECTURE.md section 5.1 does NOT list `pivots/` under `discovery/`. The pivot type is defined by `@orqastudio/plugin-agile-discovery` and exists in the schema, but ARCHITECTURE.md's directory tree omits it. This is an ARCHITECTURE.md omission, not a schema error -- the schema correctly places pivots in discovery. ARCHITECTURE.md section 3.4 acknowledges pivots exist in agile-discovery.

**Caveat 2 (MODERATE):** Schema has `discovery-decision` type with `defaultPath: ".orqa/discovery/decisions/"` but ARCHITECTURE.md section 5.1 does NOT list `decisions/` under `discovery/`. Same issue -- the schema correctly reflects the plugin's artifact type, but ARCHITECTURE.md's directory tree is incomplete.

**Recommendation:** Update ARCHITECTURE.md section 5.1 to add `pivots/` and `decisions/` under `discovery/`. These are valid artifact types from `@orqastudio/plugin-agile-discovery`.

---

### 2. Target Claude Code Plugin structure

- [ ] FAIL -- 1 critical issue in governance-steward.md

**Directory structure check:**

| Expected | Actual | Status |
|---|---|---|
| `.claude/settings.json` | Present | PASS |
| `.claude/CLAUDE.md` | Present | PASS |
| `.claude/agents/` (8 files) | 8 files present | PASS |
| NO `.claude/architecture/` | Not present | PASS |
| `plugin/.claude-plugin/plugin.json` | Present | PASS |
| `plugin/skills/` (4 dirs) | 4 dirs: orqa, orqa-create, orqa-save, orqa-validate | PASS |
| `plugin/hooks/hooks.json` | Present | PASS |
| `plugin/scripts/` (9 files) | 9 files present | PASS |
| Skills at plugin root level | Yes, sibling of `.claude-plugin/` | PASS |

**CLAUDE.md check:**
- References knowledge via MCP/prompt pipeline: PASS (line 144: "Use MCP search to retrieve detailed architecture knowledge on demand")
- No embedded architecture files: PASS
- No architecture/ directory references: PASS

**settings.json check:**
- `ORQA_DRY_RUN=false`: PASS (line 6)
- No `Bash(cat *)` in allow: PASS (grepped, not found)
- Comprehensive deny rules: PASS (34 deny entries covering secrets, targets, ARCHITECTURE.md, settings, scripts)
- **MODERATE ISSUE:** settings.json lines 55-56 deny `Edit(./.claude/architecture/**)` and `Write(./.claude/architecture/**)` but the plugin target has NO `architecture/` directory. These deny rules are harmless (denying access to a nonexistent directory) but are vestigial from the migration target and should be removed for clarity.

**Agent spot-checks (3 files):**

1. **`implementer.md`** -- PASS
   - Tools: `Read,Write,Edit,Bash,Grep,Glob,TaskUpdate,TaskGet` -- correct, no orchestration tools
   - Code comment instructions: Present (lines 44-45)
   - No `Agent`, `TeamCreate`, `TaskCreate`, `SendMessage` tools: Correct

2. **`reviewer.md`** -- PASS
   - Tools: `Read,Bash,Grep,Glob,TaskUpdate,TaskGet` -- correct, no Write/Edit (read-only)
   - Code comment instructions: Not present, but reviewers don't write code -- acceptable
   - No orchestration tools: Correct

3. **`orchestrator.md`** -- PASS
   - Tools: `Read,Glob,Grep,Agent,TeamCreate,TaskCreate,TaskUpdate,TaskGet,TaskList,SendMessage,TeamDelete` -- correct orchestration tools, no Write/Edit/Bash
   - Code comment instructions: Not applicable (doesn't write code)

4. **`governance-steward.md`** -- **CRITICAL FAIL**
   - Tools: `Read,Write,Edit,Glob,Grep,TaskUpdate,TaskGet` -- correct, no Bash
   - Code comment instructions: Present (lines 50-52)
   - **CRITICAL: Lines 38-48 contain a WRONG directory structure:**
     ```
     .orqa/
       delivery/          # epics, tasks, milestones
       discovery/         # ideas, research, personas, pillars, vision
       knowledge/         # domain knowledge artifacts
       principles/        # principle decisions
       planning/          # planning ideas, research, decisions, wireframes
       documentation/     # doc artifacts
       process/           # agents, rules, skills (core process)
       lessons/           # lesson artifacts
     ```
   - This uses OLD paths (`delivery/`, `knowledge/`, `principles/`, `process/`, `lessons/`) that contradict ARCHITECTURE.md section 5.1 and the schema.
   - The migration target's governance-steward.md (correctly) references section 5.1.
   - **The plugin target's governance-steward.md was NOT updated when the stage-first structure was applied.** This agent would direct governance stewards to create artifacts in wrong locations.

5. **`designer.md`** -- PASS
   - Tools: `Read,Write,Edit,Glob,Grep,TaskUpdate,TaskGet` -- correct, no Bash
   - Code comment instructions: Present (lines 36-38)

---

### 3. Migration Claude Code instance

- [x] PASS

**Directory structure:**
- `.claude/settings.json`: Present
- `.claude/CLAUDE.md`: Present (11,138 bytes)
- `.claude/agents/` (8 files): Present, all 8 agent files
- `.claude/architecture/` (12 files): Present, 12 files (13th `targets.md` also present = 13 total, which exceeds the expected 12 but `targets.md` is a valid addition)

**settings.json:**
- `ORQA_DRY_RUN=true`: PASS (line 6)
- `ORQA_SKIP_SCHEMA_VALIDATION=true`: PASS (line 7)
- No `Bash(cat *)` in allow: PASS
- Comprehensive deny rules: PASS (matches plugin target's deny rules)

**CLAUDE.md sections:**
- Mandatory review: PASS (line 66: "Mandatory Independent Review")
- No autonomous decisions: PASS (line 148: "No Autonomous Decisions")
- Discovery reporting: PASS (line 157: "Discovery During Execution")
- Zero tech debt: PASS (line 53: "Zero Tech Debt Enforcement")
- NEVER list: PASS (line 167: "NEVER List" with 10 entries)
- Phase gating: PASS (line 43: "Phase Gating (STRICT)")

**architecture/governance.md:**
- Stage-first `.orqa/` structure: PASS -- section 5.1 matches ARCHITECTURE.md exactly

**Agent spot-checks (3 files):**

1. **`implementer.md`** -- PASS
   - Migration context: PASS (lines 11, 15-16 reference architecture docs and migration)
   - Architecture references: PASS (lines 56-69 list all architecture files)
   - Code comment instructions: PASS (lines 72-73)
   - Zero tech debt section: PASS (lines 30-37)

2. **`governance-steward.md`** -- PASS
   - Migration context: PASS (line 3 references "during the migration")
   - Target structure references section 5.1: PASS (line 21)
   - **NOTE:** Lines 51-65 show a directory structure that uses `principles/`, `delivery/`, `process/` -- these are the OLD paths. However, line 21 says "The target `.orqa/` structure is defined in `.claude/architecture/governance.md` section 5.1" which IS correct. The inline tree is wrong but the authoritative reference is correct. This is a **moderate issue** -- an agent might follow the inline tree instead of the referenced file. But the migration instance is transitional and less impactful than the plugin target.

3. **`reviewer.md`** -- PASS
   - Migration context: PASS (line 20 "Review against ARCHITECTURE.md and the architecture files")
   - Architecture references: PASS (lines 60-72)
   - Code comment instructions: Not applicable (doesn't write code)
   - Legacy code checks: PASS (lines 36-38, 44)

---

### 4. Enforcement configs

- [x] PASS

`targets/enforcement/` contains all 6 expected directories:

| Directory | Present |
|---|---|
| `eslint/` | Yes |
| `clippy/` | Yes |
| `tsconfig/` | Yes |
| `markdownlint/` | Yes |
| `prettier/` | Yes |
| `githooks/` | Yes |

---

### 5. Task lists spot-check (5 random tasks across 4 files)

Task lists are in `file-audit/migration-tasks-phase{1-3,4-5,6-8,9-11}.md`.

**Task P1-S1-01 (phase 1-3, line 11):**
- Paths: References `targets/schema.composed.json` -- correct
- AC: 6 specific, testable checkboxes -- PASS
- "Review against architecture" note: Present via reviewer checks -- PASS

**Task P4-PRE-1 (phase 4-5, line 17):**
- Paths: No `.orqa/` paths (daemon endpoint) -- N/A
- AC: 8 specific, testable checkboxes -- PASS
- Review note: Present in reviewer checks -- PASS

**Task 6.21 (phase 6-8, decision classification):**
- Paths: References `.orqa/learning/decisions/` and `.orqa/planning/decisions/` -- stage-first, correct
- AC: Testable -- PASS
- Review note: Phase header has "Review against architecture" -- PASS

**Task 7.1 (phase 6-8, line 666):**
- Paths: Moves FROM `.orqa/process/decisions/` TO `.orqa/learning/decisions/` and `.orqa/planning/decisions/` -- correct (source is old path being migrated, target is stage-first)
- AC: Testable, includes directory existence checks -- PASS
- Review note: Phase header has "Review against architecture" -- PASS

**Task 9.1.1 (phase 9-11, line 13):**
- Paths: Frontend component paths (no `.orqa/`) -- N/A for structure
- AC: 5 specific, testable checkboxes -- PASS
- Review note: Phase header has "Review against architecture" -- PASS

No old destination paths found in task lists (grepped for `.orqa/delivery/`, `.orqa/principles/`, `.orqa/process/` as destination paths -- only legitimate references to these as SOURCE paths being migrated FROM).

---

### 6. ARCHITECTURE.md consistency

- [x] PASS (with caveat)

**Section 5.1 (`.orqa/` structure):** Uses stage-first structure consistently. No `process/`, `delivery/`, or `principles/` at top level.

**Section 12 (proposed codebase structure, lines 1050-1079):** Correctly shows `.claude/agents/` with 8 agent files, `plugin/` with correct structure. No old paths.

**Appendix A.3 (line 1157-1174):** References section 5.1, lists correct migration steps. Line 1166 says "Split decisions -- create `principles/` and `planning/` subdirectories" which could be confusing -- this refers to subdirectories within stage directories (e.g., `learning/decisions/` for principle decisions), not a top-level `principles/` directory. The wording is slightly ambiguous but the reference to section 5.1 clarifies intent.

**Caveat:** Section 5.1 is missing `pivots/` and `decisions/` under `discovery/` (see AC1 caveats). These are valid schema types that should appear in the directory tree.

---

### 7. Orqa-create skill paths

- [x] PASS

`targets/claude-code-plugin/plugin/skills/orqa-create/SKILL.md` paths match section 5.1:

| Type | Location in SKILL.md | Matches 5.1? |
|---|---|---|
| task | `.orqa/implementation/tasks/` | Yes |
| epic | `.orqa/implementation/epics/` | Yes |
| milestone | `.orqa/implementation/milestones/` | Yes |
| discovery-idea | `.orqa/discovery/ideas/` | Yes |
| planning-idea | `.orqa/planning/ideas/` | Yes |
| implementation-idea | `.orqa/implementation/ideas/` | Yes |
| discovery-research | `.orqa/discovery/research/` | Yes |
| planning-research | `.orqa/planning/research/` | Yes |
| wireframe | `.orqa/discovery/wireframes/` | Yes |
| principle-decision | `.orqa/learning/decisions/` | Yes |
| planning-decision | `.orqa/planning/decisions/` | Yes |
| rule | `.orqa/learning/rules/` | Yes |
| lesson | `.orqa/learning/lessons/` | Yes |
| knowledge | `.orqa/documentation/<category>/knowledge/` | Yes |
| doc | `.orqa/documentation/<category>/` | Yes |
| persona | `.orqa/discovery/personas/` | Yes |
| pillar | `.orqa/discovery/pillars/` | Yes |
| vision | `.orqa/discovery/vision/` | Yes |

**Note:** `pivot` and `discovery-decision` are absent from the skill but present in the schema. These may be less commonly created types, but their absence is a minor gap.

---

### 8. Remaining issues

**CRITICAL:**

1. **Plugin target governance-steward.md has wrong directory structure** (`targets/claude-code-plugin/.claude/agents/governance-steward.md` lines 38-48). Uses `delivery/`, `knowledge/`, `principles/`, `process/`, `lessons/` instead of stage-first structure. This is the agent that creates governance artifacts -- if it follows these paths, artifacts will be created in wrong locations. Must be fixed before migration.

**MODERATE:**

2. **ARCHITECTURE.md section 5.1 missing `pivots/` and `decisions/` under `discovery/`**. These artifact types exist in the schema (from `@orqastudio/plugin-agile-discovery`) but the canonical directory tree omits them. Should be added for completeness.

3. **Plugin target settings.json has vestigial `architecture/` deny rules** (lines 55-56). The plugin target has no `.claude/architecture/` directory. These rules are harmless but misleading.

4. **Migration target governance-steward.md also has old paths in its inline directory tree** (lines 51-65) but correctly references `governance.md` section 5.1 as the authoritative source. Less critical since it's transitional, but could confuse agents.

5. **Orqa-create skill missing `pivot` and `discovery-decision` types**. These exist in the schema but are not listed as creatable artifact types.

**INFORMATIONAL (from previous reviews, already documented):**

6. All 4 task list review verdicts are FAIL. The issues are documented in `review-tasklist-p{1-3,4-5,6-8,9-11}.md`. These are task list gaps (missing CLI engine migration tasks, ARCHITECTURE.md conflicts with proposed restructure, etc.) that need resolution before migration execution but do not block the current target-state work.

---

## Summary

The stage-first structure migration across targets is **nearly complete**. The schema, orqa-create skill, migration CLAUDE.md, and task list destination paths are all correct. The one critical blocker is the plugin target's governance-steward agent file containing the old directory structure, which must be fixed before this work can be committed.
