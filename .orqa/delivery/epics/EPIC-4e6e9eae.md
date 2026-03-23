---
id: "EPIC-4e6e9eae"
type: epic
title: "Principle enforcement foundations"
description: "Close all gaps between declared principles and mechanical enforcement. Backfill the relationship graph, mechanically enforce all enforceable rules, automate the learning loop, build Pillar 3 tooling, establish a behavioral rule enforcement plan, define priority dimensions, and build the gap audit into repeatable tooling. The system enforces itself going forward."
status: "completed"
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
  - target: "RES-dd35bbf0"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-dd35bbf0"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-b476bd8e"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-209f1646"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-4017a3de"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2284f3e6"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8f18cf73"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-fc7f41ca"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ab350828"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e5b83fae"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2ecb94ed"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-530bb45d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-9952c534"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-aab2e7a9"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-5f701f54"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-67dec092"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-6b0459ea"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-7cf80542"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-247b6ce4"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-db81ac75"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-75b09cdb"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-074e4088"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-81e74493"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-41d5cd97"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-9e9754a8"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-822d8c82"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-0b53e322"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-fc1f139e"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-92b1419d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-47f0f832"
    type: "delivered-by"
    rationale: "Epic contains this task"
---
## Context

[RES-dd35bbf0](RES-dd35bbf0) audited the entire governance framework and found six gap patterns:

1. **Relationship graph is declared but unpopulated** — 37/41 accepted ADs lack enforcement relationships; all 22 promoted lessons have empty `evolves-into` fields
2. **Self-compliance is the dominant enforcement mechanism** — 27/45 rules (60%) have no mechanical enforcement
3. **Pillar 3 has zero tooling** — no scope drift detection, no decision persistence, no mid-cycle orientation
4. **The learning loop is conceptual, not operational** — zero automated pipeline stage transitions, manual recurrence tracking, promotion targets never recorded
5. **Linter-enforceable rules are under-leveraged** — component purity, function size, tooltip usage could be mechanically checked but aren't
6. **Hook enforcement is strong where it exists** — the gap is not infrastructure, it's coverage

This epic closes ALL of these gaps. The goal is self-enforcement: after this epic, the system enforces its own principles mechanically — both the rules that can be checked by tooling AND a plan for rules that require behavioral enforcement.

**Promoted observations**: [IMPL-40b0b5ca](IMPL-40b0b5ca), [IMPL-8d666f0c](IMPL-8d666f0c), [IMPL-eed5b00a](IMPL-eed5b00a), [IMPL-6f4106d7](IMPL-6f4106d7), [IMPL-e4e56c2f](IMPL-e4e56c2f), [IMPL-3d4cc12c](IMPL-3d4cc12c)

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

Convert every self-compliance-only rule to mechanical enforcement where possible. For each of the 27 rules identified in [RES-dd35bbf0](RES-dd35bbf0):

**Linter-enforceable (add ESLint/clippy rules):**
- [RULE-b49142be](RULE-b49142be): Component purity — no `invoke()` in `$lib/components/` ([AD-61087142](AD-61087142))
- [RULE-b49142be](RULE-b49142be): Function size limits — flag functions >50 lines
- [RULE-89155a7f](RULE-89155a7f): Tooltip usage — no `title=` on interactive elements
- [RULE-cb65b5d0](RULE-cb65b5d0): Reusable components — warn on inline empty/loading/error patterns
- [RULE-9ba80a19](RULE-9ba80a19): No aliases — detect duplicate keys in type unions and label maps
- [RULE-1f30904a](RULE-1f30904a): Root cleanliness — lint check on root directory contents

**Hook-enforceable (extend pre-commit hook):**
- [RULE-7b770593](RULE-7b770593): Status transition validation — block invalid state transitions (e.g., `draft→in-progress` skipping `ready`)
- [RULE-6c0496e0](RULE-6c0496e0): Config-disk consistency — verify `project.json` artifact paths match actual directories
- [RULE-39169bcd](RULE-39169bcd): Pillar alignment sections — check doc pages for required section
- [RULE-6d1c8dc7](RULE-6d1c8dc7): Historical preservation — block deletion of research/task files
- [RULE-1acb1602](RULE-1acb1602): End-to-end completeness — when a Tauri command is added, check for matching TS interface

**Tooling-enforceable (extend verify tools or new scripts):**
- [RULE-11c29c9e](RULE-11c29c9e): Skill portability — scan core skills for project-specific paths
- [RULE-92dba0cb](RULE-92dba0cb): Provider-agnostic capabilities — check agent definitions use `capabilities` not `tools`
- [RULE-c95f4444](RULE-c95f4444): Data persistence boundaries — scan for governance data in SQLite or conversation data in files

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

**4a. Related idea surfacing ([IMPL-40b0b5ca](IMPL-40b0b5ca))**: Update [RULE-7b770593](RULE-7b770593) with a mandatory step in the promotion procedure: scan all ideas for thematic overlap before creating the epic.

**4b. Observation capture hook ([IMPL-8d666f0c](IMPL-8d666f0c))**: Create a `user-prompt-submit` hook in the plugin that infers observation intent from user prompts and prompts the orchestrator to auto-create IMPL entries.

**4c. Research trigger ([IMPL-6f4106d7](IMPL-6f4106d7))**: Update orchestrator behavior (rule or skill) to recognise investigation-class requests and create RES-NNN artifacts before delegating research.

### Phase 5: Behavioral Rule Enforcement Plan

Rules that are inherently non-mechanical still need an enforcement strategy. For each behavioral rule, define how it will be enforced:

| Enforcement Strategy | Applicable To |
|---------------------|---------------|
| **Prompt injection** — rule content injected into agent context at delegation time | [RULE-532100d9](RULE-532100d9) (delegation), [RULE-5e03e67b](RULE-5e03e67b) (search usage), [RULE-c71f1c3f](RULE-c71f1c3f) (make targets), [RULE-22783309](RULE-22783309) (IDs not priority), [RULE-b2753bad](RULE-b2753bad) (required reading), [RULE-deab6ea7](RULE-deab6ea7) (skill loading), [RULE-8035e176](RULE-8035e176) (structure before work), [RULE-df24948b](RULE-df24948b) (context management) |
| **Output validation** — post-hoc check on agent output for compliance signals | [RULE-878e5422](RULE-878e5422) (honest reporting — check for "What Is NOT Done" section), [RULE-551bde31](RULE-551bde31) (lessons learned — check for IMPL entries in review output), [RULE-e120bb70](RULE-e120bb70) (no deferred deliverables — check completion reports for deferral language), [RULE-303c1cc8](RULE-303c1cc8) (plan compliance — check plan structure) |
| **Skill injection** — domain knowledge loaded before relevant work | [RULE-65973a88](RULE-65973a88) (AD compliance), [RULE-9daf29c0](RULE-9daf29c0) (documentation first), [RULE-3eccebf3](RULE-3eccebf3) (enforcement before code), [RULE-d90112d9](RULE-d90112d9) (systems thinking), [RULE-4d4f540d](RULE-4d4f540d) (UAT process) |
| **Session hooks** — plugin hooks that trigger at session boundaries | [RULE-633e636d](RULE-633e636d) (git workflow — session-start/end checks), [RULE-e352fd0a](RULE-e352fd0a) (session management — session-end commit check) |

For each strategy:
- Define the implementation mechanism (plugin hook, skill content, output parser)
- Create the enforcement artifact (hook script, skill update, validation script)
- Wire into the appropriate trigger point

### Phase 6: Pillar 3 Tooling

[PILLAR-94b281db](PILLAR-94b281db) (Purpose Through Continuity) has zero tooling coverage. Four gate questions need tooling:

**6a. Scope drift detection** — tooling that compares epic scope (task list, deliverables) against what was actually implemented. Surfaces when deliverables were silently added, removed, or deferred without user approval. Enforces [RULE-e120bb70](RULE-e120bb70).

**6b. Decision persistence** — tooling that captures pending decisions, unanswered questions, and open threads at session boundaries. Ensures nothing is silently lost between sessions. Extends [RULE-e352fd0a](RULE-e352fd0a) session state.

**6c. Mid-cycle orientation** — tooling that periodically re-grounds the agent in the original epic purpose during extended work. Surfaces when execution has drifted from intention. Could be a session hook or periodic prompt injection.

**6d. Cognitive load indicators** — tooling that detects when a session has accumulated too much complexity (too many open files, too many uncommitted changes, too many interleaved tasks). Surfaces warnings to the user.

### Phase 7: Priority Framework & Automated Gap Audit

**7a. Priority dimensions**: Define project-level priority dimensions based on [RES-dd35bbf0](RES-dd35bbf0) gap patterns. Encode in `project.json` or a dedicated config artifact. Dimensions to finalize with user input.

**7b. Auto-classification rules**: Define rules that automatically classify work priority based on what it touches (e.g., integrity tooling → CRITICAL).

**7c. Automated gap audit tool**: Build a repeatable version of the [RES-dd35bbf0](RES-dd35bbf0) audit as tooling (extend `verify-pipeline-integrity.mjs` or new script) that:
- Scans all rules and reports enforcement mechanism (mechanical vs self-compliance vs behavioral plan)
- Scans all ADs and reports enforcement chain completeness
- Scans all lessons and reports promotion status / recurrence
- Scans pipeline stage transitions and reports gaps
- Outputs a prioritized gap report
- Runs as part of `make verify` and is surfaceable on the dashboard ([EPIC-e37794bf](EPIC-e37794bf))

### Phase 8: Close the Loop ([IMPL-e4e56c2f](IMPL-e4e56c2f))

The tooling built in phases 1-7 produces output. Phase 8 runs it all, reviews the results, and creates the follow-up work.

**8a. Run all enforcement tooling**: Execute `make verify` (extended), all new linter rules, the gap audit tool, pipeline health checks, and behavioral enforcement mechanisms against the full codebase. Capture the complete output.

**8b. Triage findings**: Review every finding from the tooling output. For each:
- Is it a data fix (wrong relationship, missing field)? → Fix immediately
- Is it a new enforcement gap that needs tooling? → Create a task in a follow-up epic
- Is it a behavioral gap with no enforcement plan? → Add to behavioral enforcement plan

**8c. Create follow-up epics**: Group the findings into coherent epics, prioritized using the framework from Phase 7. These epics inherit the priority dimensions and auto-classification rules — the system now prioritizes its own backlog.

**8d. Update planning methodology**: Promote [IMPL-e4e56c2f](IMPL-e4e56c2f) and [IMPL-3d4cc12c](IMPL-3d4cc12c) by updating [RULE-303c1cc8](RULE-303c1cc8) to require:
- Any epic producing enforcement or audit tooling includes a loop-closure phase (IMPL-e4e56c2f)
- Out of Scope sections are presented to the user for explicit approval before being committed (IMPL-3d4cc12c, recurrence=2)

## Tasks

| ID | Title | Phase | Depends On |
|----|-------|-------|------------|
| [TASK-b476bd8e](TASK-b476bd8e) | Backfill AD → Rule enforcement relationships (37 ADs) | 1 | — |
| [TASK-209f1646](TASK-209f1646) | Backfill lesson promoted-to targets (22 lessons) | 1 | — |
| [TASK-4017a3de](TASK-4017a3de) | Extend pipeline integrity tool with enforcement chain checks | 1 | [TASK-b476bd8e](TASK-b476bd8e), [TASK-209f1646](TASK-209f1646) |
| [TASK-2284f3e6](TASK-2284f3e6) | ESLint rules: component purity, tooltip usage, reusable components, alias detection, root cleanliness | 2 | — |
| [TASK-8f18cf73](TASK-8f18cf73) | Clippy/custom check: function size limits | 2 | — |
| [TASK-fc7f41ca](TASK-fc7f41ca) | Hook checks: status transitions, config-disk consistency, pillar alignment, historical preservation, E2E completeness | 2 | — |
| [TASK-ab350828](TASK-ab350828) | Tooling checks: skill portability, capability-not-tools, persistence boundaries | 2 | — |
| [TASK-e5b83fae](TASK-e5b83fae) | Wire all new checks into pre-commit hook | 2 | [TASK-2284f3e6](TASK-2284f3e6), [TASK-8f18cf73](TASK-8f18cf73), [TASK-fc7f41ca](TASK-fc7f41ca), [TASK-ab350828](TASK-ab350828) |
| [TASK-2ecb94ed](TASK-2ecb94ed) | Recurrence auto-tracking and promotion readiness detection | 3 | [TASK-4017a3de](TASK-4017a3de) |
| [TASK-530bb45d](TASK-530bb45d) | Pipeline stage transition health checks | 3 | [TASK-4017a3de](TASK-4017a3de) |
| [TASK-9952c534](TASK-9952c534) | Update [RULE-7b770593](RULE-7b770593): related idea surfacing during promotion | 4 | — |
| [TASK-aab2e7a9](TASK-aab2e7a9) | Plugin prompt-submit hook for observation capture | 4 | — |
| [TASK-5f701f54](TASK-5f701f54) | Research trigger: orchestrator creates RES-NNN before investigation | 4 | — |
| [TASK-074e4088](TASK-074e4088) | Behavioral enforcement plan: prompt injection rules | 5 | — |
| [TASK-81e74493](TASK-81e74493) | Behavioral enforcement plan: output validation rules | 5 | — |
| [TASK-41d5cd97](TASK-41d5cd97) | Behavioral enforcement plan: skill injection rules | 5 | — |
| [TASK-9e9754a8](TASK-9e9754a8) | Behavioral enforcement plan: session hook rules | 5 | — |
| [TASK-822d8c82](TASK-822d8c82) | Implement behavioral enforcement mechanisms | 5 | [TASK-074e4088](TASK-074e4088), [TASK-81e74493](TASK-81e74493), [TASK-41d5cd97](TASK-41d5cd97), [TASK-9e9754a8](TASK-9e9754a8) |
| [TASK-0b53e322](TASK-0b53e322) | Scope drift detection tooling | 6 | [TASK-4017a3de](TASK-4017a3de) |
| [TASK-fc1f139e](TASK-fc1f139e) | Decision persistence tooling | 6 | — |
| [TASK-92b1419d](TASK-92b1419d) | Mid-cycle orientation tooling | 6 | — |
| [TASK-47f0f832](TASK-47f0f832) | Cognitive load indicators | 6 | — |
| [TASK-67dec092](TASK-67dec092) | Define priority dimensions and auto-classification rules | 7 | [TASK-4017a3de](TASK-4017a3de) |
| [TASK-6b0459ea](TASK-6b0459ea) | Automated gap audit tool (repeatable RES-dd35bbf0) | 7 | [TASK-4017a3de](TASK-4017a3de), [TASK-2ecb94ed](TASK-2ecb94ed), [TASK-530bb45d](TASK-530bb45d) |
| [TASK-247b6ce4](TASK-247b6ce4) | Run all enforcement tooling and review output | 8 | [TASK-e5b83fae](TASK-e5b83fae), [TASK-530bb45d](TASK-530bb45d), [TASK-6b0459ea](TASK-6b0459ea), [TASK-822d8c82](TASK-822d8c82), [TASK-47f0f832](TASK-47f0f832) |
| [TASK-db81ac75](TASK-db81ac75) | Create follow-up epics/tasks to address findings | 8 | [TASK-247b6ce4](TASK-247b6ce4) |
| [TASK-75b09cdb](TASK-75b09cdb) | Update [RULE-303c1cc8](RULE-303c1cc8): loop-closure + scope verification requirements | 8 | [TASK-247b6ce4](TASK-247b6ce4) |
| [TASK-7cf80542](TASK-7cf80542) | Reconcile [EPIC-4e6e9eae](EPIC-4e6e9eae) | — | all above |

## Out of Scope

User-approved exclusions:

- App UI for priority management and gap surfacing — [EPIC-e37794bf](EPIC-e37794bf) handles the dashboard; all outcomes from this epic feed into it