# Review: Migration Tasks Phases 9-11

## Verdict: FAIL

Three issues require remediation before this task list is ready for execution.

---

## Acceptance Criteria

### 1. All 25 hardcoded frontend patterns from the audit have specific removal tasks

**PASS**

All 25 patterns from `phase2-04-engine-app-gaps.md` Part 2 are accounted for:

- HIGH #1 (MarkdownLink ARTIFACT_ID_RE) -- TASK 9.1.1
- HIGH #2 (StatusBar sidecarPluginName) -- TASK 9.1.2
- MEDIUM #3-5 (triplicated model options) -- TASK 9.2.1 (consolidated)
- MEDIUM #6 (FrontmatterHeader field classification) -- TASK 9.2.2
- MEDIUM #7 (ArtifactViewer fallback stages) -- TASK 9.2.3
- MEDIUM #8 (ArtifactLanding categoryConfig) -- TASK 9.2.4
- MEDIUM #9 (DynamicArtifactTable sort orders) -- TASK 9.2.5
- MEDIUM #10 (LessonVelocityWidget stages) -- TASK 9.2.6
- MEDIUM #11 (ImprovementTrendsWidget governance types) -- TASK 9.2.7
- MEDIUM #12 (DecisionQueueWidget action labels) -- TASK 9.2.8
- MEDIUM #13 (EmbeddingModelStep model name) -- TASK 9.2.9
- LOW #14 (category-colors.ts) -- TASK 9.5.2
- LOW #15 (tool-display.ts) -- TASK 9.5.3
- LOW #16 (TraceabilityPanel iconForType) -- TASK 9.5.1
- LOW #17 (SettingsCategoryNav) -- TASK 9.4.1
- LOW #18-22, #25 (ShortcutsSettings, ProjectSetupWizard, GraphHealthPanel, SetupComplete, SetupWizard, ArtifactLink) -- correctly omitted. The audit explicitly classifies these as "App feature, not governance," "Reasonable defaults," or "Has override mechanism." No remediation needed.
- LOW #23 (ToolCallCard enforcement regex) -- TASK 9.5.5
- LOW #24 (ExplorerRouter CORE_VIEWS) -- TASK 9.5.4

All items requiring remediation have tasks. Items correctly assessed as non-issues are omitted.

---

### 2. Navigation restructure covers: dashboard, methodology stages, plugins, settings

**PASS**

TASK 9.3.1 covers Dashboard + methodology stages + Plugins + Settings in the activity bar. The task explicitly lists all four elements and requires methodology stages to come from the installed methodology plugin, not hardcoded. Fallback behavior when no methodology plugin is installed is specified (Dashboard + Plugins + Settings only).

---

### 3. Plugin browser includes: available/installed, category filters, plugin groups

**PASS**

- TASK 9.3.2 covers the Plugins top-level nav item and "installed/official/community tabs" plus "category filters (knowledge, methodology, workflow, sidecar, connector, infrastructure)"
- TASK 9.3.3 covers plugin group bundling with methodology + stage plugins

---

### 4. Settings reorganization covers: methodology (with workflow nesting), sidecar, connector, generic plugins

**PASS**

TASK 9.4.1 explicitly lists all four sections: Methodology (with workflow plugins nested underneath), Sidecar, Connector, and Plugins (generic section for all other installed plugins). App settings retention and plugin-generated settings pages are covered.

---

### 5. Navigation settings page removal is explicit

**PASS**

TASK 9.4.1 acceptance criteria includes: "Navigation settings page is removed." The task also lists `NavigationSettings.svelte` in the files section with "(remove or repurpose)." Reviewer checks include verifying `NavigationSettings.svelte` is removed or repurposed.

---

### 6. Roadmap view validation task exists

**FAIL**

ARCHITECTURE.md Phase 9 "Custom Views" section (line 894) explicitly states: "Review the roadmap view to ensure it works with the milestone/epic hierarchy." The task list has TASK 9.6.1 for generic plugin custom views, but NO specific roadmap view validation task. The roadmap view is called out by name in the architecture as a specific validation item. TASK 9.6.1 is about plugin views generally via `PluginViewContainer.svelte`, which may not cover the roadmap view if it is a core view rather than a plugin view.

A dedicated task is needed: verify the roadmap view works with the milestone/epic hierarchy as specified in ARCHITECTURE.md Phase 9.

---

### 7. Every target artifact has a validation task in Phase 10

**FAIL**

The `targets/` directory contains TWO subdirectories:
- `targets/claude-code-plugin/` -- fully covered by Phase 10 tasks (10.2.1-10.2.8)
- `targets/claude-code-migration/` -- has ZERO validation tasks

`targets/claude-code-migration/` contains 22 files:
- 8 agent files (`.claude/agents/*.md`)
- 12 architecture files (`.claude/architecture/*.md`)
- 1 CLAUDE.md (`.claude/CLAUDE.md`)
- 1 settings.json (`.claude/settings.json`)

TASK 10.4.1 says "delete the `targets/` directory entirely" which would remove these files, but there are no tasks to validate them against generated output first. ARCHITECTURE.md Phase 10 says: "For each target artifact from Phase 1: Run the generation pipeline, Compare generated output against hand-written target." If `claude-code-migration` is a separate connector target, it needs its own validation tasks. If it is not a target (e.g., it is the migration-specific connector output that is only used during migration), then TASK 10.4.1 should explicitly acknowledge its removal as a migration artifact, not a generation target.

Either way, the current task list does not account for these 22 files.

---

### 8. Every architecture split file has both DOC and KNOW conversion tasks in Phase 11

**FAIL**

All 12 architecture split files have DOC conversion tasks (11.1.1-11.1.12). However, only 7 of 12 have KNOW creation tasks:

| Architecture File | DOC Task | KNOW Task | Status |
|-------------------|----------|-----------|--------|
| core.md | 11.1.1 | 11.2.1 (3 KNOW) | COVERED |
| plugins.md | 11.1.2 | 11.2.2 (3 KNOW) | COVERED |
| governance.md | 11.1.3 | 11.2.3 (2 KNOW) | COVERED |
| agents.md | 11.1.4 | 11.2.4 (3 KNOW) | COVERED |
| connector.md | 11.1.5 | 11.2.5 (2 KNOW) | COVERED |
| enforcement.md | 11.1.6 | 11.2.6 (2 KNOW) | COVERED |
| decisions.md | 11.1.7 | NONE | MISSING |
| structure.md | 11.1.8 | NONE | MISSING |
| glossary.md | 11.1.9 | 11.2.7 (1-2 KNOW) | COVERED |
| targets.md | 11.1.10 | NONE | MISSING |
| migration.md | 11.1.11 | NONE | MISSING |
| audit.md | 11.1.12 | NONE | MISSING |

5 files (decisions, structure, targets, migration, audit) have DOC tasks but no KNOW tasks.

ARCHITECTURE.md Phase 11 says: "Ensure the documentation/knowledge hierarchy is complete -- every architectural concept has both a doc and derived knowledge." TASK 11.3.4 verifies "Every DOC has at least one derived KNOW" but it is a verification task, not a creation task. If these 5 DOCs don't have KNOW tasks, 11.3.4 will fail with no task assigned to create the missing artifacts.

However, one could argue that some of these (targets.md, migration.md) become historical reference post-migration and may not warrant knowledge artifacts for agent injection. If that is the intentional design, it should be explicitly documented in the task list with a justification for the omission. As written, the architecture requires "every architectural concept has both a doc and derived knowledge" without exception.

---

### 9. file-audit/ and targets/ removal tasks exist

**PASS**

- TASK 10.4.1 removes `targets/` directory (after all validation)
- TASK 11.3.1 removes `file-audit/` directory
- Both tasks include acceptance criteria for verifying no stale references remain

---

### 10. All tasks are atomic with testable AC, nothing deferred

**PASS** (with note)

All tasks have:
- Clear "What" description fitting one agent context window
- Specific file paths
- Checkboxed acceptance criteria
- Reviewer checks

No task defers work to a follow-up. Dependencies are clearly documented in the dependency chain.

**Note:** The summary table at the bottom of the file has incorrect task counts:
- Phase 9: claims 20 tasks, actual count is 22 (2 + 9 + 3 + 1 + 5 + 1 + 1)
- Phase 10: claims 14 tasks, actual count is 17 (1 + 8 + 6 + 2)
- Phase 11: claims 20 tasks, actual count is 23 (12 + 7 + 4)
- Total: claims 54 tasks, actual count is 62

This is a cosmetic issue that does not affect execution but should be corrected to avoid confusion.

---

## Issues Found

### Issue 1: Missing roadmap view validation task (Phase 9)
- **Location:** `file-audit/migration-tasks-phase9-11.md` section 9.6
- **ARCHITECTURE.md reference:** line 894: "Review the roadmap view to ensure it works with the milestone/epic hierarchy"
- **Fix:** Add TASK 9.6.2 specifically for roadmap view validation

### Issue 2: Missing validation tasks for `targets/claude-code-migration/` (Phase 10)
- **Location:** `file-audit/migration-tasks-phase9-11.md` section 10.2
- **Actual files:** 22 files in `targets/claude-code-migration/` with no Phase 10 validation tasks
- **Fix:** Either add validation tasks for these 22 files, or add explicit justification for why they are excluded (e.g., migration-specific artifacts not produced by a generation pipeline)

### Issue 3: 5 architecture files missing KNOW creation tasks (Phase 11)
- **Location:** `file-audit/migration-tasks-phase9-11.md` section 11.2
- **Missing KNOW tasks for:** decisions.md, structure.md, targets.md, migration.md, audit.md
- **Fix:** Add TASK 11.2.8 through 11.2.12 for the missing KNOW artifacts, OR explicitly document in the task list why these DOCs are exempt from the "every DOC has derived KNOW" requirement

### Issue 4: Incorrect summary task counts (cosmetic)
- **Location:** `file-audit/migration-tasks-phase9-11.md` lines 1417-1422
- **Fix:** Update Phase 9 to 22, Phase 10 to 17, Phase 11 to 23, Total to 62

---

## Lessons

- When creating task lists from architecture specs, verify every named item in the spec has a corresponding task -- "roadmap view" was called out by name but not given a task.
- When targets directories contain multiple subdirectories, every subdirectory needs explicit coverage or documented exclusion.
- Verification tasks (like "check that every DOC has a KNOW") are not substitutes for creation tasks -- if the creation tasks don't exist, the verification will catch the gap but there will be no assigned task to fix it.
- Summary counts should be verified by actual enumeration, not estimated.
