---
id: EPIC-061
title: Principle enforcement foundations
description: "Close all gaps between declared principles and mechanical enforcement. Backfill the relationship graph, mechanically enforce all enforceable rules, automate the learning loop, build Pillar 3 tooling, establish a behavioral rule enforcement plan, define priority dimensions, and build the gap audit into repeatable tooling. The system enforces itself going forward."
status: completed
priority: P1
created: 2026-03-13
updated: 2026-03-13
deadline: null
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 5
  dependencies: 5
rule-overrides: []
relationships:
  - target: IMPL-048
    type: informs
    rationale: Auto-generated inverse of informs relationship from IMPL-048
  - target: IMPL-049
    type: informs
    rationale: Auto-generated inverse of informs relationship from IMPL-049
  - target: RES-054
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-054
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-350
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-351
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-362
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-363
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-364
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-365
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-366
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-367
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-368
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-369
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-370
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-371
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-372
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-373
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-374
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-375
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-376
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-377
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-378
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-379
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-380
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-381
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-382
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-383
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-384
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-385
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-386
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-387
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-060
    type: depended-on-by
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: grounded-by
  - target: PILLAR-003
    type: grounded-by
  - target: PILLAR-001
    type: grounded-by
---
## Context

[RES-054](RES-054) audited the entire governance framework and found six gap patterns:

1. **Relationship graph is declared but unpopulated** — 37/41 accepted ADs lack enforcement relationships; all 22 promoted lessons have empty `evolves-into` fields
2. **Self-compliance is the dominant enforcement mechanism** — 27/45 rules (60%) have no mechanical enforcement
3. **Pillar 3 has zero tooling** — no scope drift detection, no decision persistence, no mid-cycle orientation
4. **The learning loop is conceptual, not operational** — zero automated pipeline stage transitions, manual recurrence tracking, promotion targets never recorded
5. **Linter-enforceable rules are under-leveraged** — component purity, function size, tooltip usage could be mechanically checked but aren't
6. **Hook enforcement is strong where it exists** — the gap is not infrastructure, it's coverage

This epic closes ALL of these gaps. The goal is self-enforcement: after this epic, the system enforces its own principles mechanically — both the rules that can be checked by tooling AND a plan for rules that require behavioral enforcement.

**Promoted observations**: [IMPL-044](IMPL-044), [IMPL-045](IMPL-045), [IMPL-046](IMPL-046), [IMPL-047](IMPL-047), [IMPL-048](IMPL-048), [IMPL-049](IMPL-049)

## Implementation Design

### Phase 1: Relationship Graph Backfill (Data Integrity)

The graph is lying — enforcement chains exist in prose but not in structured relationships. This must be fixed before any tooling can reason about the graph.

**1a. AD → Rule enforcement edges**: For each of the 37 accepted ADs without enforcement relationships, determine:
- Does a rule enforce this AD? If yes, add `enforced-by: RULE-NNN` relationship to the AD and `enforces: AD-NNN` to the rule
- Does a skill practice this AD? If yes, add `practiced-by: SKILL-NNN` relationship
- Is this AD a strategy/selection decision with no enforceable constraint? Mark as `intended: true` (no enforcement needed)

**1b. Lesson promoted-to targets**: All 22 promoted lessons have empty `evolves-into` fields. For each, trace what rule/skill/standard it was promoted to and populate the field.

**1c. Extend `verify-pipeline-integrity.mjs`**: Add checks for:
- Accepted ADs without any enforcement/practice relationship (error unless `intended: true`)
- Promoted lessons without `evolves-into` targets (error)
- Rules that reference ADs in body text but don't have `enforces` relationships (warning)

### Phase 2: Mechanical Rule Enforcement (All 27 Self-Compliance Rules)

Convert every self-compliance-only rule to mechanical enforcement where possible. For each of the 27 rules identified in [RES-054](RES-054):

**Linter-enforceable (add ESLint/clippy rules):**
- [RULE-006](RULE-006): Component purity — no `invoke()` in `$lib/components/` ([AD-006](AD-006))
- [RULE-006](RULE-006): Function size limits — flag functions >50 lines
- [RULE-033](RULE-033): Tooltip usage — no `title=` on interactive elements
- [RULE-024](RULE-024): Reusable components — warn on inline empty/loading/error patterns
- [RULE-018](RULE-018): No aliases — detect duplicate keys in type unions and label maps
- [RULE-025](RULE-025): Root cleanliness — lint check on root directory contents

**Hook-enforceable (extend pre-commit hook):**
- [RULE-004](RULE-004): Status transition validation — block invalid state transitions (e.g., `draft→in-progress` skipping `ready`)
- [RULE-003](RULE-003): Config-disk consistency — verify `project.json` artifact paths match actual directories
- [RULE-021](RULE-021): Pillar alignment sections — check doc pages for required section
- [RULE-014](RULE-014): Historical preservation — block deletion of research/task files
- [RULE-010](RULE-010): End-to-end completeness — when a Tauri command is added, check for matching TS interface

**Tooling-enforceable (extend verify tools or new scripts):**
- [RULE-035](RULE-035): Skill portability — scan core skills for project-specific paths
- [RULE-040](RULE-040): Provider-agnostic capabilities — check agent definitions use `capabilities` not `tools`
- [RULE-041](RULE-041): Data persistence boundaries — scan for governance data in SQLite or conversation data in files

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

**4a. Related idea surfacing ([IMPL-044](IMPL-044))**: Update [RULE-004](RULE-004) with a mandatory step in the promotion procedure: scan all ideas for thematic overlap before creating the epic.

**4b. Observation capture hook ([IMPL-045](IMPL-045))**: Create a `user-prompt-submit` hook in the plugin that infers observation intent from user prompts and prompts the orchestrator to auto-create IMPL entries.

**4c. Research trigger ([IMPL-047](IMPL-047))**: Update orchestrator behavior (rule or skill) to recognise investigation-class requests and create RES-NNN artifacts before delegating research.

### Phase 5: Behavioral Rule Enforcement Plan

Rules that are inherently non-mechanical still need an enforcement strategy. For each behavioral rule, define how it will be enforced:

| Enforcement Strategy | Applicable To |
|---------------------|---------------|
| **Prompt injection** — rule content injected into agent context at delegation time | [RULE-001](RULE-001) (delegation), [RULE-005](RULE-005) (search usage), [RULE-007](RULE-007) (make targets), [RULE-016](RULE-016) (IDs not priority), [RULE-023](RULE-023) (required reading), [RULE-026](RULE-026) (skill loading), [RULE-027](RULE-027) (structure before work), [RULE-036](RULE-036) (context management) |
| **Output validation** — post-hoc check on agent output for compliance signals | [RULE-015](RULE-015) (honest reporting — check for "What Is NOT Done" section), [RULE-017](RULE-017) (lessons learned — check for IMPL entries in review output), [RULE-019](RULE-019) (no deferred deliverables — check completion reports for deferral language), [RULE-022](RULE-022) (plan compliance — check plan structure) |
| **Skill injection** — domain knowledge loaded before relevant work | [RULE-002](RULE-002) (AD compliance), [RULE-008](RULE-008) (documentation first), [RULE-011](RULE-011) (enforcement before code), [RULE-028](RULE-028) (systems thinking), [RULE-030](RULE-030) (UAT process) |
| **Session hooks** — plugin hooks that trigger at session boundaries | [RULE-013](RULE-013) (git workflow — session-start/end checks), [RULE-039](RULE-039) (session management — session-end commit check) |

For each strategy:
- Define the implementation mechanism (plugin hook, skill content, output parser)
- Create the enforcement artifact (hook script, skill update, validation script)
- Wire into the appropriate trigger point

### Phase 6: Pillar 3 Tooling

[PILLAR-003](PILLAR-003) (Purpose Through Continuity) has zero tooling coverage. Four gate questions need tooling:

**6a. Scope drift detection** — tooling that compares epic scope (task list, deliverables) against what was actually implemented. Surfaces when deliverables were silently added, removed, or deferred without user approval. Enforces [RULE-019](RULE-019).

**6b. Decision persistence** — tooling that captures pending decisions, unanswered questions, and open threads at session boundaries. Ensures nothing is silently lost between sessions. Extends [RULE-039](RULE-039) session state.

**6c. Mid-cycle orientation** — tooling that periodically re-grounds the agent in the original epic purpose during extended work. Surfaces when execution has drifted from intention. Could be a session hook or periodic prompt injection.

**6d. Cognitive load indicators** — tooling that detects when a session has accumulated too much complexity (too many open files, too many uncommitted changes, too many interleaved tasks). Surfaces warnings to the user.

### Phase 7: Priority Framework & Automated Gap Audit

**7a. Priority dimensions**: Define project-level priority dimensions based on [RES-054](RES-054) gap patterns. Encode in `project.json` or a dedicated config artifact. Dimensions to finalize with user input.

**7b. Auto-classification rules**: Define rules that automatically classify work priority based on what it touches (e.g., integrity tooling → CRITICAL).

**7c. Automated gap audit tool**: Build a repeatable version of the [RES-054](RES-054) audit as tooling (extend `verify-pipeline-integrity.mjs` or new script) that:
- Scans all rules and reports enforcement mechanism (mechanical vs self-compliance vs behavioral plan)
- Scans all ADs and reports enforcement chain completeness
- Scans all lessons and reports promotion status / recurrence
- Scans pipeline stage transitions and reports gaps
- Outputs a prioritized gap report
- Runs as part of `make verify` and is surfaceable on the dashboard ([EPIC-060](EPIC-060))

### Phase 8: Close the Loop ([IMPL-048](IMPL-048))

The tooling built in phases 1-7 produces output. Phase 8 runs it all, reviews the results, and creates the follow-up work.

**8a. Run all enforcement tooling**: Execute `make verify` (extended), all new linter rules, the gap audit tool, pipeline health checks, and behavioral enforcement mechanisms against the full codebase. Capture the complete output.

**8b. Triage findings**: Review every finding from the tooling output. For each:
- Is it a data fix (wrong relationship, missing field)? → Fix immediately
- Is it a new enforcement gap that needs tooling? → Create a task in a follow-up epic
- Is it a behavioral gap with no enforcement plan? → Add to behavioral enforcement plan

**8c. Create follow-up epics**: Group the findings into coherent epics, prioritized using the framework from Phase 7. These epics inherit the priority dimensions and auto-classification rules — the system now prioritizes its own backlog.

**8d. Update planning methodology**: Promote [IMPL-048](IMPL-048) and [IMPL-049](IMPL-049) by updating [RULE-022](RULE-022) to require:
- Any epic producing enforcement or audit tooling includes a loop-closure phase (IMPL-048)
- Out of Scope sections are presented to the user for explicit approval before being committed (IMPL-049, recurrence=2)

## Tasks

| ID | Title | Phase | Depends On |
|----|-------|-------|------------|
| [TASK-350](TASK-350) | Backfill AD → Rule enforcement relationships (37 ADs) | 1 | — |
| [TASK-351](TASK-351) | Backfill lesson promoted-to targets (22 lessons) | 1 | — |
| [TASK-362](TASK-362) | Extend pipeline integrity tool with enforcement chain checks | 1 | [TASK-350](TASK-350), [TASK-351](TASK-351) |
| [TASK-363](TASK-363) | ESLint rules: component purity, tooltip usage, reusable components, alias detection, root cleanliness | 2 | — |
| [TASK-364](TASK-364) | Clippy/custom check: function size limits | 2 | — |
| [TASK-365](TASK-365) | Hook checks: status transitions, config-disk consistency, pillar alignment, historical preservation, E2E completeness | 2 | — |
| [TASK-366](TASK-366) | Tooling checks: skill portability, capability-not-tools, persistence boundaries | 2 | — |
| [TASK-367](TASK-367) | Wire all new checks into pre-commit hook | 2 | [TASK-363](TASK-363), [TASK-364](TASK-364), [TASK-365](TASK-365), [TASK-366](TASK-366) |
| [TASK-368](TASK-368) | Recurrence auto-tracking and promotion readiness detection | 3 | [TASK-362](TASK-362) |
| [TASK-369](TASK-369) | Pipeline stage transition health checks | 3 | [TASK-362](TASK-362) |
| [TASK-370](TASK-370) | Update [RULE-004](RULE-004): related idea surfacing during promotion | 4 | — |
| [TASK-371](TASK-371) | Plugin prompt-submit hook for observation capture | 4 | — |
| [TASK-372](TASK-372) | Research trigger: orchestrator creates RES-NNN before investigation | 4 | — |
| [TASK-379](TASK-379) | Behavioral enforcement plan: prompt injection rules | 5 | — |
| [TASK-380](TASK-380) | Behavioral enforcement plan: output validation rules | 5 | — |
| [TASK-381](TASK-381) | Behavioral enforcement plan: skill injection rules | 5 | — |
| [TASK-382](TASK-382) | Behavioral enforcement plan: session hook rules | 5 | — |
| [TASK-383](TASK-383) | Implement behavioral enforcement mechanisms | 5 | [TASK-379](TASK-379), [TASK-380](TASK-380), [TASK-381](TASK-381), [TASK-382](TASK-382) |
| [TASK-384](TASK-384) | Scope drift detection tooling | 6 | [TASK-362](TASK-362) |
| [TASK-385](TASK-385) | Decision persistence tooling | 6 | — |
| [TASK-386](TASK-386) | Mid-cycle orientation tooling | 6 | — |
| [TASK-387](TASK-387) | Cognitive load indicators | 6 | — |
| [TASK-373](TASK-373) | Define priority dimensions and auto-classification rules | 7 | [TASK-362](TASK-362) |
| [TASK-374](TASK-374) | Automated gap audit tool (repeatable RES-054) | 7 | [TASK-362](TASK-362), [TASK-368](TASK-368), [TASK-369](TASK-369) |
| [TASK-376](TASK-376) | Run all enforcement tooling and review output | 8 | [TASK-367](TASK-367), [TASK-369](TASK-369), [TASK-374](TASK-374), [TASK-383](TASK-383), [TASK-387](TASK-387) |
| [TASK-377](TASK-377) | Create follow-up epics/tasks to address findings | 8 | [TASK-376](TASK-376) |
| [TASK-378](TASK-378) | Update [RULE-022](RULE-022): loop-closure + scope verification requirements | 8 | [TASK-376](TASK-376) |
| [TASK-375](TASK-375) | Reconcile [EPIC-061](EPIC-061) | — | all above |

## Out of Scope

User-approved exclusions:

- App UI for priority management and gap surfacing — [EPIC-060](EPIC-060) handles the dashboard; all outcomes from this epic feed into it
