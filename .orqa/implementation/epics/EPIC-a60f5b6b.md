---
id: "EPIC-a60f5b6b"
type: epic
title: "Principle enforcement foundations"
description: "Close all gaps between declared principles and mechanical enforcement. Backfill the relationship graph, mechanically enforce all enforceable rules, automate the learning loop, build Pillar 3 tooling, establish a behavioral rule enforcement plan, define priority dimensions, and build the gap audit into repeatable tooling. The system enforces itself going forward."
status: archived
priority: "P1"
created: "2026-03-13"
updated: "2026-03-13"
deadline: null
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 5
  dependencies: 5
rule-overrides: []
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

[RES-0e971367](RES-0e971367) audited the entire governance framework and found six gap patterns:

1. **Relationship graph is declared but unpopulated** — 37/41 accepted ADs lack enforcement relationships; all 22 promoted lessons have empty `evolves-into` fields
2. **Self-compliance is the dominant enforcement mechanism** — 27/45 rules (60%) have no mechanical enforcement
3. **Pillar 3 has zero tooling** — no scope drift detection, no decision persistence, no mid-cycle orientation
4. **The learning loop is conceptual, not operational** — zero automated pipeline stage transitions, manual recurrence tracking, promotion targets never recorded
5. **Linter-enforceable rules are under-leveraged** — component purity, function size, tooltip usage could be mechanically checked but aren't
6. **Hook enforcement is strong where it exists** — the gap is not infrastructure, it's coverage

This epic closes ALL of these gaps. The goal is self-enforcement: after this epic, the system enforces its own principles mechanically — both the rules that can be checked by tooling AND a plan for rules that require behavioral enforcement.

**Promoted observations**: [IMPL-f3629976](IMPL-f3629976), [IMPL-6a8f9612](IMPL-6a8f9612), [IMPL-9520fb0b](IMPL-9520fb0b), [IMPL-c8e2803a](IMPL-c8e2803a), [IMPL-b19a7e02](IMPL-b19a7e02), [IMPL-c726abc2](IMPL-c726abc2)

## Implementation Design

### Phase 1: Relationship Graph Backfill (Data Integrity)

The graph is lying — enforcement chains exist in prose but not in structured relationships. This must be fixed before any tooling can reason about the graph.

**1a. AD → Rule enforcement edges**: For each of the 37 accepted ADs without enforcement relationships, determine:

- Does a rule enforce this AD? If yes, add `enforced-by: RULE-NNN` relationship to the AD and `enforces: AD-NNN` to the rule
- Does a knowledge artifact practice this AD? If yes, add `practiced-by: KNOW-NNN` relationship
- Is this AD a strategy/selection decision with no enforceable constraint? Mark as `intended: true` (no enforcement needed)

**1b. Lesson promoted-to targets**: All 22 promoted lessons have empty `evolves-into` fields. For each, trace what rule/skill/standard it was promoted to and populate the field.

**1c. Extend `verify-pipeline-integrity.mjs`**: Add checks for:

- Accepted ADs without any enforcement/practice relationship (error unless `intended: true`)
- Promoted lessons without `evolves-into` targets (error)
- Rules that reference ADs in body text but don't have `enforces` relationships (warning)

### Phase 2: Mechanical Rule Enforcement (All 27 Self-Compliance Rules)

Convert every self-compliance-only rule to mechanical enforcement where possible. For each of the 27 rules identified in [RES-0e971367](RES-0e971367):

**Linter-enforceable (add ESLint/clippy rules):**

- [RULE-9814ec3c](RULE-9814ec3c): Component purity — no `invoke()` in `$lib/components/` ([PD-9a7d7256](PD-9a7d7256))
- [RULE-9814ec3c](RULE-9814ec3c): Function size limits — flag functions >50 lines
- [RULE-83411442](RULE-83411442): Tooltip usage — no `title=` on interactive elements
- [RULE-eb269afb](RULE-eb269afb): Reusable components — warn on inline empty/loading/error patterns
- [RULE-c382e053](RULE-c382e053): No aliases — detect duplicate keys in type unions and label maps
- [RULE-97e96528](RULE-97e96528): Root cleanliness — lint check on root directory contents

**Hook-enforceable (extend pre-commit hook):**

- [RULE-b10fe6d1](RULE-b10fe6d1): Status transition validation — block invalid state transitions (e.g., `draft→in-progress` skipping `ready`)
- [RULE-63cc16ad](RULE-63cc16ad): Config-disk consistency — verify `project.json` artifact paths match actual directories
- [RULE-05562ed4](RULE-05562ed4): Pillar alignment sections — check doc pages for required section
- [RULE-484872ef](RULE-484872ef): Historical preservation — block deletion of research/task files
- RULE-b03009da: End-to-end completeness — when a Tauri command is added, check for matching TS interface

**Tooling-enforceable (extend verify tools or new scripts):**

- [RULE-205d9c91](RULE-205d9c91): Skill portability — scan core skills for project-specific paths
- [RULE-8abcbfd5](RULE-8abcbfd5): Provider-agnostic capabilities — check agent definitions use `capabilities` not `tools`
- [RULE-09a238ab](RULE-09a238ab): Data persistence boundaries — scan for governance data in SQLite or conversation data in files

**Behavioral rules (cannot be mechanically enforced — need enforcement plan):**
See Phase 5.

Add all new checks to pre-commit hook staged-file paths.

### Phase 3: Learning Loop Automation (Pipeline Mechanics)

The knowledge maturity pipeline has zero automated stage transitions. This phase adds the mechanical drivers.

**3a. Recurrence auto-tracking**: Extend `verify-pipeline-integrity.mjs` (or a new tool) to:

- Scan review agent output for failure patterns that match existing lessons
- Auto-increment recurrence when a match is found
- Surface lessons with `recurrence >= 2` that haven't been promoted

**3b. Promotion readiness detection**: Add tooling to detect:

- Observations that should be elevated to understanding (maturity assessment signals)
- Understandings that recur and should become rules/skills
- The `evolves-into` field is empty on promoted lessons (enforcement from Phase 1c)

**3c. Stage transition suggestions**: Build a `pipeline-health` check (can be part of `verify-pipeline-integrity.mjs` or a new tool) that reports:

- Stuck observations (active for >N days with no advancement)
- Accepted ADs without corresponding skills
- Skills without corresponding rules
- Rules without verification mechanisms

### Phase 4: Process Automation

**4a. Related idea surfacing ([IMPL-f3629976](IMPL-f3629976))**: Update [RULE-b10fe6d1](RULE-b10fe6d1) with a mandatory step in the promotion procedure: scan all ideas for thematic overlap before creating the epic.

**4b. Observation capture hook ([IMPL-6a8f9612](IMPL-6a8f9612))**: Create a `user-prompt-submit` hook in the plugin that infers observation intent from user prompts and prompts the orchestrator to auto-create IMPL entries.

**4c. Research trigger ([IMPL-c8e2803a](IMPL-c8e2803a))**: Update orchestrator behavior (rule or skill) to recognise investigation-class requests and create RES-NNN artifacts before delegating research.

### Phase 5: Behavioral Rule Enforcement Plan

Rules that are inherently non-mechanical still need an enforcement strategy. For each behavioral rule, define how it will be enforced:

| Enforcement Strategy | Applicable To |
| --------------------- | --------------- |
| **Prompt injection** — rule content injected into agent context at delegation time | [RULE-87ba1b81](RULE-87ba1b81) (delegation), [RULE-0d29fc91](RULE-0d29fc91) (search usage), [RULE-d2c2063a](RULE-d2c2063a) (make targets), [RULE-25baac14](RULE-25baac14) (IDs not priority), [RULE-5965256d](RULE-5965256d) (required reading), [RULE-dd5b69e6](RULE-dd5b69e6) (skill loading), [RULE-d5d28fba](RULE-d5d28fba) (structure before work), [RULE-ef822519](RULE-ef822519) (context management) |
| **Output validation** — post-hoc check on agent output for compliance signals | [RULE-5dd9decd](RULE-5dd9decd) (honest reporting — check for "What Is NOT Done" section), [RULE-c603e90e](RULE-c603e90e) (lessons learned — check for IMPL entries in review output), [RULE-8ee65d73](RULE-8ee65d73) (no deferred deliverables — check completion reports for deferral language), [RULE-dccf4226](RULE-dccf4226) (plan compliance — check plan structure) |
| **Skill injection** — domain knowledge loaded before relevant work | [RULE-05ae2ce7](RULE-05ae2ce7) (AD compliance), [RULE-ec9462d8](RULE-ec9462d8) (documentation first), [RULE-4603207a](RULE-4603207a) (enforcement before code), [RULE-43f1bebc](RULE-43f1bebc) (systems thinking), [RULE-71352dc8](RULE-71352dc8) (UAT process) |
| **Session hooks** — plugin hooks that trigger at session boundaries | [RULE-f609242f](RULE-f609242f) (git workflow — session-start/end checks), [RULE-30a223ca](RULE-30a223ca) (session management — session-end commit check) |

For each strategy:

- Define the implementation mechanism (plugin hook, skill content, output parser)
- Create the enforcement artifact (hook script, skill update, validation script)
- Wire into the appropriate trigger point

### Phase 6: Pillar 3 Tooling

[PILLAR-a6a4bbbb](PILLAR-a6a4bbbb) (Purpose Through Continuity) has zero tooling coverage. Four gate questions need tooling:

**6a. Scope drift detection** — tooling that compares epic scope (task list, deliverables) against what was actually implemented. Surfaces when deliverables were silently added, removed, or deferred without user approval. Enforces [RULE-8ee65d73](RULE-8ee65d73).

**6b. Decision persistence** — tooling that captures pending decisions, unanswered questions, and open threads at session boundaries. Ensures nothing is silently lost between sessions. Extends [RULE-30a223ca](RULE-30a223ca) session state.

**6c. Mid-cycle orientation** — tooling that periodically re-grounds the agent in the original epic purpose during extended work. Surfaces when execution has drifted from intention. Could be a session hook or periodic prompt injection.

**6d. Cognitive load indicators** — tooling that detects when a session has accumulated too much complexity (too many open files, too many uncommitted changes, too many interleaved tasks). Surfaces warnings to the user.

### Phase 7: Priority Framework & Automated Gap Audit

**7a. Priority dimensions**: Define project-level priority dimensions based on [RES-0e971367](RES-0e971367) gap patterns. Encode in `project.json` or a dedicated config artifact. Dimensions to finalize with user input.

**7b. Auto-classification rules**: Define rules that automatically classify work priority based on what it touches (e.g., integrity tooling → CRITICAL).

**7c. Automated gap audit tool**: Build a repeatable version of the [RES-0e971367](RES-0e971367) audit as tooling (extend `verify-pipeline-integrity.mjs` or new script) that:

- Scans all rules and reports enforcement mechanism (mechanical vs self-compliance vs behavioral plan)
- Scans all ADs and reports enforcement chain completeness
- Scans all lessons and reports promotion status / recurrence
- Scans pipeline stage transitions and reports gaps
- Outputs a prioritized gap report
- Runs as part of `make verify` and is surfaceable on the dashboard ([EPIC-82dd0bd2](EPIC-82dd0bd2))

### Phase 8: Close the Loop ([IMPL-b19a7e02](IMPL-b19a7e02))

The tooling built in phases 1-7 produces output. Phase 8 runs it all, reviews the results, and creates the follow-up work.

**8a. Run all enforcement tooling**: Execute `make verify` (extended), all new linter rules, the gap audit tool, pipeline health checks, and behavioral enforcement mechanisms against the full codebase. Capture the complete output.

**8b. Triage findings**: Review every finding from the tooling output. For each:

- Is it a data fix (wrong relationship, missing field)? → Fix immediately
- Is it a new enforcement gap that needs tooling? → Create a task in a follow-up epic
- Is it a behavioral gap with no enforcement plan? → Add to behavioral enforcement plan

**8c. Create follow-up epics**: Group the findings into coherent epics, prioritized using the framework from Phase 7. These epics inherit the priority dimensions and auto-classification rules — the system now prioritizes its own backlog.

**8d. Update planning methodology**: Promote [IMPL-b19a7e02](IMPL-b19a7e02) and [IMPL-c726abc2](IMPL-c726abc2) by updating [RULE-dccf4226](RULE-dccf4226) to require:

- Any epic producing enforcement or audit tooling includes a loop-closure phase (IMPL-b19a7e02)
- Out of Scope sections are presented to the user for explicit approval before being committed (IMPL-c726abc2, recurrence=2)

## Tasks

| ID | Title | Phase | Depends On |
| ---- | ------- | ------- | ------------ |
| [TASK-520f6e7d](TASK-520f6e7d) | Backfill AD → Rule enforcement relationships (37 ADs) | 1 | — |
| [TASK-bf0ee06e](TASK-bf0ee06e) | Backfill lesson promoted-to targets (22 lessons) | 1 | — |
| [TASK-445e8155](TASK-445e8155) | Extend pipeline integrity tool with enforcement chain checks | 1 | [TASK-520f6e7d](TASK-520f6e7d), [TASK-bf0ee06e](TASK-bf0ee06e) |
| [TASK-6a07cfc9](TASK-6a07cfc9) | ESLint rules: component purity, tooltip usage, reusable components, alias detection, root cleanliness | 2 | — |
| [TASK-da07ad35](TASK-da07ad35) | Clippy/custom check: function size limits | 2 | — |
| [TASK-31f82835](TASK-31f82835) | Hook checks: status transitions, config-disk consistency, pillar alignment, historical preservation, E2E completeness | 2 | — |
| [TASK-a034493b](TASK-a034493b) | Tooling checks: skill portability, capability-not-tools, persistence boundaries | 2 | — |
| [TASK-1f74e00b](TASK-1f74e00b) | Wire all new checks into pre-commit hook | 2 | [TASK-6a07cfc9](TASK-6a07cfc9), [TASK-da07ad35](TASK-da07ad35), [TASK-31f82835](TASK-31f82835), [TASK-a034493b](TASK-a034493b) |
| [TASK-32091895](TASK-32091895) | Recurrence auto-tracking and promotion readiness detection | 3 | [TASK-445e8155](TASK-445e8155) |
| [TASK-d6e26c99](TASK-d6e26c99) | Pipeline stage transition health checks | 3 | [TASK-445e8155](TASK-445e8155) |
| [TASK-aa7a08df](TASK-aa7a08df) | Update [RULE-b10fe6d1](RULE-b10fe6d1): related idea surfacing during promotion | 4 | — |
| [TASK-e11d1abe](TASK-e11d1abe) | Plugin prompt-submit hook for observation capture | 4 | — |
| [TASK-d6965613](TASK-d6965613) | Research trigger: orchestrator creates RES-NNN before investigation | 4 | — |
| [TASK-2a33d99e](TASK-2a33d99e) | Behavioral enforcement plan: prompt injection rules | 5 | — |
| [TASK-26dac5ca](TASK-26dac5ca) | Behavioral enforcement plan: output validation rules | 5 | — |
| [TASK-ee6ea3d2](TASK-ee6ea3d2) | Behavioral enforcement plan: skill injection rules | 5 | — |
| [TASK-3ee643f7](TASK-3ee643f7) | Behavioral enforcement plan: session hook rules | 5 | — |
| [TASK-80c27efd](TASK-80c27efd) | Implement behavioral enforcement mechanisms | 5 | [TASK-2a33d99e](TASK-2a33d99e), [TASK-26dac5ca](TASK-26dac5ca), [TASK-ee6ea3d2](TASK-ee6ea3d2), [TASK-3ee643f7](TASK-3ee643f7) |
| [TASK-495614cb](TASK-495614cb) | Scope drift detection tooling | 6 | [TASK-445e8155](TASK-445e8155) |
| [TASK-5289f70e](TASK-5289f70e) | Decision persistence tooling | 6 | — |
| [TASK-c34a185e](TASK-c34a185e) | Mid-cycle orientation tooling | 6 | — |
| [TASK-291be1ff](TASK-291be1ff) | Cognitive load indicators | 6 | — |
| [TASK-7d94eb29](TASK-7d94eb29) | Define priority dimensions and auto-classification rules | 7 | [TASK-445e8155](TASK-445e8155) |
| [TASK-52101e2a](TASK-52101e2a) | Automated gap audit tool (repeatable RES-0e971367) | 7 | [TASK-445e8155](TASK-445e8155), [TASK-32091895](TASK-32091895), [TASK-d6e26c99](TASK-d6e26c99) |
| [TASK-79ff025c](TASK-79ff025c) | Run all enforcement tooling and review output | 8 | [TASK-1f74e00b](TASK-1f74e00b), [TASK-d6e26c99](TASK-d6e26c99), [TASK-52101e2a](TASK-52101e2a), [TASK-80c27efd](TASK-80c27efd), [TASK-291be1ff](TASK-291be1ff) |
| [TASK-a72df473](TASK-a72df473) | Create follow-up epics/tasks to address findings | 8 | [TASK-79ff025c](TASK-79ff025c) |
| [TASK-920d562a](TASK-920d562a) | Update [RULE-dccf4226](RULE-dccf4226): loop-closure + scope verification requirements | 8 | [TASK-79ff025c](TASK-79ff025c) |
| [TASK-9471304a](TASK-9471304a) | Reconcile [EPIC-a60f5b6b](EPIC-a60f5b6b) | — | all above |

## Out of Scope

User-approved exclusions:

- App UI for priority management and gap surfacing — [EPIC-82dd0bd2](EPIC-82dd0bd2) handles the dashboard; all outcomes from this epic feed into it
