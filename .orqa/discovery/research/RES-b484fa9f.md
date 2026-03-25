---
id: "RES-b484fa9f"
type: research
title: "Agent, Skill, Rule, and Hook Audit Findings"
description: "Comprehensive audit of all governance artifacts (agents, skills, rules, hooks) for accuracy against the current codebase, cross-layer consistency, and missing/miscategorised artifact analysis."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
relationships:
  - target: "EPIC-5aa11e2f"
    type: "guides"
    rationale: "Research findings informed epic design"
---
# Agent, Skill, Rule, and Hook Audit Findings

**Epic:** [EPIC-5aa11e2f](EPIC-5aa11e2f)
**Tasks completed:** [TASK-1637bc63](TASK-1637bc63) (agents), [TASK-d6030100](TASK-d6030100) (skills), [TASK-88e72cc1](TASK-88e72cc1) (rules), [TASK-81b11647](TASK-81b11647) (hooks), [TASK-3109164e](TASK-3109164e) (cross-layer), [TASK-dfa29194](TASK-dfa29194) (missing/miscategorised)

---


## Executive Summary

Six audits were conducted across all governance layers. The governance framework is structurally sound — no contradictions between layers, no broken role model, no missing agent types. The issues found are primarily:

1. **Stale path references** from the rule rename migration (`dogfood-mode.md` → `[RULE-998da8ea](RULE-998da8ea).md`, etc.) — 20+ instances across agents, skills, and rules
2. **Broken Claude Code hooks** — `.claude/settings.json` points to `.claude/hooks/` which is gitignored and empty
3. **Content duplication** between rules and their companion skills (RULE-dccf4226/planning, [RULE-71352dc8](RULE-71352dc8)/uat-process)
4. **Stale codebase references** — `persistence/` → `repo/`, `TODO.md` removed, shared component inventory outdated
5. **Five orphaned skills** with no loading mechanism
6. **Tier 1 skill mislabeling** — orchestrator claims they auto-load from agent YAML but they don't

**Total findings: 62 across all audits.**

---


## Finding Categories and Counts

| Category | Agents | Skills | Rules | Hooks | Cross-Layer | Misc. | Total |
|----------|--------|--------|-------|-------|-------------|-------|-------|
| Stale file paths | 8 | 2 | 10 | 7 | 1 | — | 28 |
| Stale code patterns | — | 2 | — | — | — | — | 2 |
| Content accuracy | 1 | 2 | 5 | 2 | — | — | 10 |
| Cross-layer inconsistency | — | — | — | — | 6 | — | 6 |
| Missing/orphaned artifacts | — | — | — | — | 7 | 6 | 13 |
| Miscategorisation | — | — | — | — | — | 3 | 3 |
| **Total** | **9** | **6** | **15** | **9** | **14** | **9** | **62** |

---


## Priority 1: Fix Immediately

### F-01: Claude Code lifecycle hooks are broken (TASK-81b11647, Finding 2)

**Impact:** Session-start checks (stashes, worktrees, uncommitted files) and pre-commit reminders silently fail every CLI session.

`.claude/settings.json` references `.claude/hooks/session-start-hook.sh` and `.claude/hooks/pre-commit-reminder.sh`. However `.claude/hooks/` is listed in `.gitignore` (line 54) and no files exist there. The symlink from `.claude/hooks/` to `.orqa/process/hooks/` either doesn't exist or is blocked by gitignore.

**Fix:** Update `.claude/settings.json` to reference `.orqa/process/hooks/` directly:
```
bash "$CLAUDE_PROJECT_DIR/.orqa/process/hooks/session-start-hook.sh"
bash "$CLAUDE_PROJECT_DIR/.orqa/process/hooks/pre-commit-reminder.sh"
```
Also remove `.claude/hooks/` from `.gitignore` or document that the symlink must be created post-clone.

### F-02: Stale `dogfood-mode.md` references in 6 agent definitions (TASK-1637bc63, Finding 3)

Every non-orchestrator agent references `.orqa/process/rules/dogfood-mode.md` which does not exist. The file is `.orqa/process/rules/[RULE-998da8ea](RULE-998da8ea).md`.

**Affected files:** `implementer.md`, `researcher.md`, `reviewer.md`, `writer.md`, `designer.md`, `planner.md`

**Fix:** Replace `dogfood-mode.md` with `[RULE-998da8ea](RULE-998da8ea).md` (or `[RULE-998da8ea](RULE-998da8ea)`) in all six agents.

### F-03: Stale rule filename references in 3 agents (TASK-1637bc63, Finding 3)

| Agent | Stale Reference | Correct Reference |
|-------|----------------|-------------------|
| `implementer.md` | `no-stubs.md` | `[RULE-af5771e3](RULE-af5771e3).md` |
| `writer.md` | `pillar-alignment-docs.md` | `[RULE-05562ed4](RULE-05562ed4).md` |
| `planner.md` | `plan-mode-compliance.md` (Required Reading + 2 body refs) | `[RULE-dccf4226](RULE-dccf4226).md` |

### F-04: Orchestrator hooks path wrong (TASK-1637bc63 Finding 5, [TASK-81b11647](TASK-81b11647) Finding 1, [TASK-3109164e](TASK-3109164e) Finding 1.2)

The orchestrator's Hooks Configuration table says `.orqa/hooks/` — should be `.orqa/process/hooks/`.

### F-05: Stale `.orqa/hooks/` references across 7+ documentation files (TASK-81b11647, Finding 7)

| File | Stale Path |
|------|-----------|
| `.orqa/documentation/about/glossary.md` | `.orqa/hooks/` |
| `.orqa/documentation/about/mvp-specification.md` (2 occurrences) | `.orqa/hooks/` |
| `.orqa/documentation/about/journeys.md` | `.orqa/hooks/` |
| `.orqa/documentation/reference/project-configuration.md` | `.orqa/hooks/` |
| `.orqa/documentation/reference/governance-bootstrap.md` | `.orqa/hooks/` |

*Research files (RES-4ae0750b, RES-5435cae9) also have this but should NOT be modified per [RULE-484872ef](RULE-484872ef).*

**Fix:** Update all non-research references to `.orqa/process/hooks/`.

---


## Priority 2: Fix Soon

### F-06: Orchestrator project structure shows `persistence/` — actual directory is `repo/` (TASK-1637bc63 Finding 4, [TASK-d6030100](TASK-d6030100) Finding 1.1, [TASK-3109164e](TASK-3109164e) Finding 2.3)

The project structure in `orchestrator.md` line 344 shows `persistence/` but the actual directory on disk is `backend/src-tauri/src/repo/`. Also shows a `tools/` directory that doesn't exist as a top-level dir.

The `architecture` skill also references `backend/src-tauri/src/persistence/` in its Layer Responsibilities table.

**Fix:** Update orchestrator project structure and architecture skill to use `repo/`.

### F-07: All 6 `TODO.md` rule references are stale (TASK-88e72cc1)

`TODO.md` no longer exists. Rules referencing it: [RULE-9814ec3c](RULE-9814ec3c), [RULE-ec9462d8](RULE-ec9462d8) (2 refs), [RULE-0be7765e](RULE-0be7765e), [RULE-af5771e3](RULE-af5771e3), [RULE-dccf4226](RULE-dccf4226).

**Fix:** Replace with references to `.orqa/delivery/tasks/` or `.orqa/delivery/epics/` as appropriate.

### F-08: [RULE-f609242f](RULE-f609242f) source of truth path broken (TASK-88e72cc1)

References `.orqa/documentation/development/agentic-workflow.md` which doesn't exist. Closest match: `.orqa/documentation/guide/workflow.md`.

### F-09: [RULE-eb269afb](RULE-eb269afb) shared component inventory is 60% wrong (TASK-88e72cc1)

7 of 12 listed components don't exist at `$lib/components/shared/`. 7 components that DO exist are unlisted.

**Components that DON'T exist:** PageToolbar, StatusBadge, ProgressBar, Panel, CodeBlock, MarkdownRenderer (in `content/`), ConversationMessage (is `MessageBubble` in `conversation/`)

**Components that exist but are unlisted:** SmallBadge, SearchInput, MetadataRow, SelectMenu, ThinkingBlock, ArtifactListItem, StatusIndicator

### F-10: [RULE-97e96528](RULE-97e96528) root directory allowlist has 3 phantom entries (TASK-88e72cc1, [TASK-81b11647](TASK-81b11647) Finding 6)

Lists `docs/`, `BLOCKERS.md`, `.pre-commit-config.yaml` — none exist on disk.

### F-11: [RULE-b10fe6d1](RULE-b10fe6d1) references nonexistent milestone fields (TASK-88e72cc1)

References `epic-count` and `completed-epics` as milestone fields. Neither exists in the milestone schema.

### F-12: [RULE-0be7765e](RULE-0be7765e) and [RULE-23699df2](RULE-23699df2) pre-commit hook descriptions inaccurate (TASK-81b11647 Finding 5, [TASK-81b11647](TASK-81b11647) Finding 4)

- [RULE-0be7765e](RULE-0be7765e) says hook runs `make check` — actually runs conditional subsets
- [RULE-23699df2](RULE-23699df2) says hook calls `validate-schema.mjs` directly — actually goes through `validate-artifacts.sh` wrapper

### F-13: Tier 1 skills mislabeled in orchestrator (TASK-3109164e, Finding 4.1)

Orchestrator says Tier 1 skills are "declared in agent YAML, loaded automatically" but `rust-async-patterns`, `svelte5-best-practices`, `tailwind-design-system`, `typescript-advanced-types`, `tauri-v2` do NOT appear in any agent's `skills:` YAML list. They are loaded by the orchestrator at delegation time — making them effectively Tier 2.

**Fix options:**
- A) Add these skill IDs to the relevant agent YAML `skills:` lists (makes them truly Tier 1)
- B) Reclassify in the orchestrator as "orchestrator-injected based on task domain" (acknowledges current reality)

### F-14: Five orphaned skills with no loading mechanism (TASK-3109164e, Finding 4.2)

| Skill | Purpose | Status |
|-------|---------|--------|
| `project-inference` | Detect project characteristics | Future use (app setup flow) |
| `project-migration` | Import from other AI tools | Future use (app setup flow) |
| `project-setup` | Scaffold .orqa/ directory | Future use (app setup flow) |
| `project-type-software` | Software dev preset | Future use (app setup flow) |
| `orqa-plugin-development` | Plugin creation guide | Future use (plugin system) |

These are forward-looking skills for features not yet implemented. They should be documented as such rather than appearing to be active.

### F-15: Skills with stale code patterns (TASK-d6030100)

| Skill | Issue |
|-------|-------|
| `orqa-domain-services` | References `load_context_messages()` — actual function is `load_context_summary()` |
| `orqa-repository-pattern` | Lists 8 of 10 repos — missing `enforcement_rules_repo` and `project_settings_repo` |
| `orqa-governance` | References rule by old filename `artifact-config-integrity.md` — should be `[RULE-63cc16ad](RULE-63cc16ad)` |

### F-16: `tailwind-design-system` skill uses React examples (TASK-d6030100, Finding 6.1)

Component variant examples use `React.ButtonHTMLAttributes`, `forwardRef`, Radix UI primitives. OrqaStudio uses Svelte 5 + shadcn-svelte. As a canon skill the React examples aren't "wrong", but they are misleading for agents working in this Svelte project.

---


## Priority 3: Improvements

### F-17: Content duplication between rules and companion skills (TASK-dfa29194, Check 1)

| Rule | Skill | Duplication |
|------|-------|-------------|
| [RULE-dccf4226](RULE-dccf4226) (Plan Mode Compliance) | `planning` | Full plan template exists in both |
| [RULE-71352dc8](RULE-71352dc8) (UAT Process) | `uat-process` | Full 4-phase methodology in both |
| RULE-b03009da (End-to-End Completeness) | `orqa-ipc-patterns` (partial) | 100+ lines of code examples in rule |
| [RULE-eb269afb](RULE-eb269afb) (Reusable Components) | None | Component inventory = knowledge, not constraint |

**Recommendation:** Rules keep constraints and FORBIDDEN sections. Skills keep methodology, templates, and code examples. Deduplicate on fix.

### F-18: Concepts needing companion artifacts (TASK-dfa29194, Check 3)

| Concept | Has Rule? | Has Skill? | Gap |
|---------|-----------|------------|-----|
| Artifact link format | No | `orqa-documentation` (FORBIDDEN section) | Need rule — binary constraint hidden in a skill |
| Skill portability constraints | No | `skills-maintenance` (NON-NEGOTIABLE section) | Need rule — hard constraints without enforcement path |
| Systems thinking methodology | [RULE-43f1bebc](RULE-43f1bebc) | No | Need companion skill — rule says "ask questions" but no HOW |
| Component reuse patterns | [RULE-eb269afb](RULE-eb269afb) | No dedicated skill | Low priority — inventory is small |

### F-19: `composability` layer classification (TASK-d6030100, Finding 3.1)

Marked `layer: canon` but contains OrqaStudio-specific file paths and code examples. The composability *principles* are portable; the *examples* are project-specific.

**Options:**
- A) Keep `layer: canon`, replace OrqaStudio examples with generic ones
- B) Change to `layer: project` (requires CLAUDE.md update since it's listed as Tier 1 universal)

### F-20: Empty Related Rules sections on 5 rules (TASK-88e72cc1)

[RULE-05ae2ce7](RULE-05ae2ce7), [RULE-f609242f](RULE-f609242f), [RULE-05562ed4](RULE-05562ed4), [RULE-97e96528](RULE-97e96528), [RULE-43f1bebc](RULE-43f1bebc) have the "Related Rules" heading but no content.

### F-21: Frontmatter field ordering inconsistent across rules (TASK-88e72cc1)

Most rules don't follow the documented `propertyOrder` from schema.json. Purely cosmetic but violates convention.

### F-22: [RULE-83411442](RULE-83411442) scope field uses `software-engineering` (TASK-88e72cc1)

Not a documented valid value. Rule schema has no enum constraint on `scope`, but artifact-framework documents valid values as `system | domain | project | role | artifact`.

### F-23: Suggested new hooks (TASK-dfa29194, Check 6)

| Hook | Priority | What It Checks |
|------|----------|---------------|
| Stub scanner | High | [RULE-af5771e3](RULE-af5771e3) explicitly calls for this — grep staged files for TODO/FIXME/HACK |
| Task dependency validator | Medium | Check `depends-on` tasks are `done` before starting |
| Epic readiness validator | Medium | Check `docs-required` paths exist on disk |

### F-24: Implicit conventions not captured (TASK-dfa29194, Check 5)

| Convention | Where Found |
|-----------|-------------|
| Context window management guidelines | orchestrator.md Section 1 |
| Tool access restrictions per role | Agent YAML `tools:` lists |
| `user-invocable` skill field semantics | Skill YAML — undocumented |
| Session management / overnight mode | orchestrator.md — convention, not rule |

### F-25: No lessons ready for promotion (TASK-dfa29194, Check 4)

All 15 lessons checked. 6 already promoted. Remaining 9 are at recurrence 1 — below the threshold of 2.

---


## Recommended Implementation Approach

### Batch 1: Path fixes (mechanical, low risk)
Fix all stale paths in one commit — agent `dogfood-mode.md` refs, orchestrator hooks path, documentation `.orqa/hooks/` refs, rule `TODO.md` refs, [RULE-f609242f](RULE-f609242f) source of truth, orchestrator `persistence/` → `repo/`. ~30 file edits.

### Batch 2: Hook fix (critical)
Fix `.claude/settings.json` to point to `.orqa/process/hooks/` directly. Update `.gitignore` if needed.

### Batch 3: Content accuracy
Update [RULE-eb269afb](RULE-eb269afb) component inventory, [RULE-b10fe6d1](RULE-b10fe6d1) milestone fields, [RULE-0be7765e](RULE-0be7765e)/032 pre-commit descriptions, skill code patterns (function names, repo inventory).

### Batch 4: Structural improvements (needs user direction)
- Tier 1 skill resolution (option A vs B)
- `composability` layer classification
- Rule/skill deduplication strategy
- New companion artifacts (link format rule, systems thinking skill)
- Orphaned skills disposition
- New hooks (stub scanner)

---


## Files Examined

All 7 agent definitions, all 37 skill definitions, all 33 rule files, all hook scripts and configurations, the orchestrator (CLAUDE.md), `.claude/settings.json`, `.gitignore`, `.githooks/*`, and targeted codebase verification via Glob/Grep.