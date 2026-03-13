---
id: RULE-004
title: Artifact Lifecycle
description: Enforces creation standards, status transitions, promotion gates, and documentation gates for all .orqa/ artifacts.
status: active
created: 2026-03-07
updated: 2026-03-13
layer: core
scope:
  - AGENT-003
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Artifact lifecycle enforces structured progression from idea to completion
  - target: RULE-008
    type: informs
    rationale: Artifact lifecycle enforces documentation gates before and after implementation
  - target: RULE-016
    type: informs
    rationale: IDs are identifiers not rankings — priority comes from scoring, not creation order
  - target: RULE-031
    type: informs
    rationale: Pillar alignment is required for all artifacts as part of lifecycle validation
  - target: RULE-017
    type: informs
    rationale: Lesson lifecycle (create, track recurrence, promote) is part of the artifact lifecycle
  - target: RULE-021
    type: informs
    rationale: Documentation gates require pillar alignment sections in produced docs
  - target: RULE-015
    type: informs
    rationale: Artifact status must reflect reality — honest reporting is required for lifecycle transitions
  - type: informed-by
    target: RULE-008
    rationale: Documentation-first defines the docs-required and docs-produced gates this rule enforces
  - type: informed-by
    target: RULE-014
    rationale: Historical preservation rules define which artifact types are deleted vs preserved
  - type: informed-by
    target: RULE-016
    rationale: ID semantics inform how IDs are assigned and interpreted in the lifecycle
  - type: informed-by
    target: RULE-019
    rationale: No-deferred-deliverables enforces completion before epic status transitions to done
  - type: informed-by
    target: RULE-021
    rationale: Pillar alignment docs rules apply at the docs-produced gate
  - type: informed-by
    target: RULE-022
    rationale: Plan compliance verification gates align with lifecycle transition requirements
  - type: informed-by
    target: RULE-027
    rationale: Structure-before-work requires artifacts to exist before implementation begins
  - type: informed-by
    target: RULE-031
    rationale: Vision alignment requirements apply at artifact creation and promotion gates
  - type: informed-by
    target: RULE-032
    rationale: Schema validation enforces required field completeness at each lifecycle stage
  - type: enforces
    target: AD-040
    rationale: AD-040 established the structure-before-work principle that artifact lifecycle gates enforce mechanically
  - type: grounded
    target: IMPL-014
    rationale: Lesson IMPL-014 identified recurring lifecycle gate violations that shaped the non-negotiable transition rules
  - target: IMPL-014
    type: observes
    rationale: Rule promoted from lesson IMPL-014 (epic titles should describe outcomes not process)
  - target: IMPL-021
    type: observes
    rationale: Rule updated from lesson IMPL-021 (open items discovered during implementation need tracking)
  - type: grounded
    target: IMPL-021
    rationale: Lesson IMPL-021 identified the structure-during-work gap and observation triage protocol
  - target: IMPL-022
    type: observes
    rationale: Rule updated from lesson IMPL-022 (epics with all tasks done should surface for user review)
  - type: grounded
    target: IMPL-022
    rationale: Lesson IMPL-022 identified that epic completion visibility requires automatic surfacing
  - target: IMPL-025
    type: observes
    rationale: Rule updated from lesson IMPL-025 (observations must flow forward via triage task)
  - type: grounded
    target: IMPL-025
    rationale: Lesson IMPL-025 identified that every epic producing observations needs a triage task
  - target: IMPL-038
    type: observes
    rationale: Rule updated from lesson IMPL-038 (observation capture is not scope creep — scope at triage)
  - type: grounded
    target: IMPL-038
    rationale: Lesson IMPL-038 clarified that capture and scope decisions must be separated in the triage protocol
  - target: IMPL-042
    type: observes
    rationale: Rule updated from lesson IMPL-042 (epic body drifts from actual work without standing reconciliation task)
  - type: grounded
    target: IMPL-042
    rationale: Lesson IMPL-042 identified the epic body drift pattern and standing reconciliation task solution
  - target: IMPL-044
    type: observes
    rationale: Rule updated from lesson IMPL-044 (idea promotion should scan for related ideas to bundle)
  - type: grounded
    target: IMPL-044
    rationale: Lesson IMPL-044 identified that idea promotion should include a related-ideas scan step
---
Every structured artifact in `.orqa/` follows a defined lifecycle. This rule enforces creation standards, status transitions, promotion gates, documentation gates, and cross-referencing.

**Source of Truth:** `.orqa/documentation/product/artifact-framework.md`

---

## Artifact Creation Standards

### When to Create Artifacts

| Trigger | Artifact Type | Action |
|---------|--------------|--------|
| User mentions a future feature or "we should eventually..." | `IDEA-NNN` | Create in `.orqa/delivery/ideas/` with `status: captured` |
| User approves an idea for investigation | Update existing `IDEA-NNN` | Set `status: exploring`, begin research |
| Research validates an idea for implementation | `EPIC-NNN` | Create in `.orqa/delivery/epics/` with `status: draft`, update idea `promoted-to` |
| An epic needs investigation work before implementation | Research file | Create in `.orqa/delivery/research/`; reference from epic `research-refs` field. Implementation design goes in the epic body. |
| An epic is approved and scoped for implementation | Update `EPIC-NNN` | Set `status: ready` (requires `docs-required` gate satisfied) |
| A task within an epic needs detailed tracking | `TASK-NNN` | Create in `.orqa/delivery/tasks/` with `epic:` reference |
| An epic is created or moves to `ready` | Reconciliation `TASK-NNN` | Auto-create a standing reconciliation task for the epic (see Epic Reconciliation Task below) |
| A strategic goal is defined | `MS-NNN` | Create in `.orqa/delivery/milestones/` |
| An implementation reveals a reusable pattern | `IMPL-NNN` | Create in `.orqa/process/lessons/` (see [RULE-017](RULE-017) (lessons-learned)) |
| A question needs investigation before a decision | Research file | Create in `.orqa/delivery/research/` |
| Research produces an architectural choice | `AD-NNN` | Create in `.orqa/process/decisions/` |

### ID Assignment

IDs auto-increment per type. To determine the next ID, scan existing files in the type's directory and increment the highest number. IDs are never reused after deletion.

### Required Fields

Every artifact MUST have all fields marked "Required" in the schema defined in `.orqa/documentation/product/artifact-framework.md`. Missing required fields are a review FAIL.

---

## Status Transition Rules

Status transitions MUST follow the defined workflows. Skipping states is forbidden unless explicitly noted.

### Milestone

```
planning ──> active ──> complete
```

- `planning → active`: At least one epic exists with `status: ready` or later
- `active → complete`: The milestone's `gate` question can be answered "yes" — all P1 epics are `done`

### Epic

```
draft ──> ready ──> in-progress ──> review ──> done
```

- `draft → ready`: All `docs-required` items exist and are approved (Documentation Gate — see below)
- `ready → in-progress`: Epic meets Definition of Ready, worktree created, agent assigned
- `in-progress → review`: Implementation complete, submitted for verification gates
- `review → done`: **Human gate (NON-NEGOTIABLE)** — the orchestrator presents a completion summary to the user and receives explicit approval. The summary must include: tasks completed, docs-produced verification, lessons logged during implementation, and any scope changes. The orchestrator MUST NOT mark an epic as done without user confirmation. All verification gates must also have passed (code-reviewer, qa-tester, ux-reviewer), and all `docs-produced` items verified as created/updated

The epic body contains the implementation design — data model, IPC contracts, component breakdown, and approach. For investigation-heavy work, the epic may carry a `research-refs` field listing research documents in `.orqa/delivery/research/` that informed the design.

### Epic Reconciliation Task (NON-NEGOTIABLE)

Every epic MUST have a standing reconciliation task created when the epic is created or moves to `ready`. This task enforces ongoing epic body maintenance throughout the epic lifecycle — not just at closure.

**Creation:** When an epic is created, the orchestrator auto-creates a task:

```
TASK-NNN: "Reconcile EPIC-NNN"
epic: EPIC-NNN
status: in-progress (active for the duration of the epic)
acceptance:
  - "Epic task table lists ALL tasks created during the epic"
  - "Epic pillars array reflects all pillars served"
  - "Epic docs-produced list matches actual documentation created/updated"
  - "Epic scope section accurately reflects what was in/out of scope"
  - "No tasks exist for this epic that are missing from the epic body"
```

**Lifecycle:**
- Starts as `in-progress` when the epic begins
- The orchestrator updates the epic body whenever tasks are added, pillars change, or scope evolves
- Cannot be marked `done` until all other epic tasks are done and the epic body is verified accurate
- Is always the **last task** completed before the epic moves to `review → done`

**What it checks:**
1. Task table completeness — every TASK-NNN with `epic: EPIC-NNN` appears in the epic body
2. Pillar accuracy — pillars array reflects all pillars the work actually serves
3. Docs-produced accuracy — listed docs were actually created/updated
4. Scope accuracy — out-of-scope section reflects actual decisions made during implementation

**Why this exists:** Epic bodies drift as work evolves mid-epic. Tasks get added, pillars change, scope shifts — but nobody updates the epic body until closure. This task makes maintenance the AI's responsibility throughout the epic, not a human cleanup at the end.

### Task

```
todo ──> in-progress ──> done
```

- `todo → in-progress`: Parent epic is `in-progress`, agent is assigned, **and all tasks listed in `depends-on` have `status: done`**
- `in-progress → done`: Acceptance criteria met, verified by reviewer

Tasks are either completed or not. There is no surpassed state for tasks.

### Task Dependency Gate (NON-NEGOTIABLE)

If a task has a `depends-on` field listing other task IDs, those tasks MUST be `done` before the dependent task can move to `in-progress`. This is a hard gate — not a suggestion.

**Before starting any task**, the orchestrator MUST:

1. Check the task's `depends-on` field
2. If non-empty, verify each listed task has `status: done`
3. If any dependency is not done, the task is **blocked** — do not start it
4. Report which dependencies are blocking if asked about task status

**Circular dependencies are forbidden.** If TASK-A depends on TASK-B and TASK-B depends on TASK-A, both tasks are broken — fix the dependency chain before proceeding.

### Research

```
draft ──> complete ──> surpassed
```

- Research documents capture investigation findings and feed into epics or architecture decisions.
- A research document may be marked `surpassed` when newer investigation supersedes it. Set `surpassed-by` field. Do NOT delete — research documents are historical records of reasoning and findings.

### Idea

```
captured ──> exploring ──> shaped ──> promoted
                                  └──> archived
```

- `captured → exploring`: User approves investigation. Research begins on `research-needed` items.
- `exploring → shaped`: All `research-needed` items have been investigated. Research artifacts exist. The idea has a clear scope and proposed approach.
- `shaped → promoted`: User approves promotion. An `EPIC-NNN` is created. The idea's `promoted-to` field is set to the epic ID.
- `shaped → archived`: User decides not to pursue. Reason documented in the idea body.
- Any state → `archived`: User explicitly archives.

**FORBIDDEN transitions:**

- `captured → promoted` — skipping research/shaping is not allowed
- `exploring → promoted` — must be shaped (scoped and validated) before promotion
- Any backward transition without user approval

### Decision

```
proposed ──> accepted ──> superseded
                      └──> deprecated
```

- `proposed → accepted`: Decision reviewed and approved by the user
- `accepted → superseded`: A new decision replaces this one — both the new and old artifacts MUST be updated in the same commit
- `accepted → deprecated`: Decision is no longer relevant (technology removed, context changed) — reason documented in the decision body

**Creation rule:** When research produces an architectural choice, an `AD-NNN.md` MUST be created in `.orqa/process/decisions/` following the Decision schema in `.orqa/documentation/product/artifact-framework.md`.

**Supersession rule:** When a new decision replaces an accepted decision, both the new artifact (`supersedes: AD-<old>`) and the old artifact (`status: superseded`, `superseded-by: AD-<new>`) MUST be updated in the same commit. A one-sided supersession is an integrity violation.

---

## Documentation Gate (NON-NEGOTIABLE)

### Before Epic Implementation Starts (`draft → ready`)

Every epic's `docs-required` field lists documentation that MUST exist before implementation begins. The orchestrator MUST verify:

1. Each listed document exists at the specified path
2. Each document is current (not a stale placeholder)
3. If a document is missing, it must be created and approved BEFORE the epic moves to `ready`

**If `docs-required` is empty or null:** The epic has no documentation prerequisites — the gate is automatically satisfied.

**If any `docs-required` item is missing:** The epic is blocked. Document the gap and create the documentation first.

### Research Reference Consistency Check

`research-refs` and `docs-required` serve different purposes on epics:

- **`research-refs`** — traceability: "What research informed this design?" (backward-looking)
- **`docs-required`** — gate: "What must exist before we start?" (forward-looking)

These fields intentionally overlap when a research doc is both informative and a prerequisite. When creating or updating an epic, the orchestrator MUST check for consistency:

1. **Every `research-refs` entry should appear in `docs-required`** unless the research is context-only (informative but not blocking). If a research doc is omitted from `docs-required`, annotate the `research-refs` entry with a comment explaining why it is not a prerequisite.
2. **`docs-required` may contain non-research entries** — architecture specs, UI wireframes, and other documentation that must be written before implementation. These do not appear in `research-refs`.
3. **Drift detection**: If `research-refs` lists a `RES-NNN` that is not in `docs-required` and there is no documented reason for the omission, flag it during review as a potential oversight.

### After Epic Implementation Completes (`review → done`)

Every epic's `docs-produced` field lists documentation that this work creates or updates. The code-reviewer MUST verify:

1. Each listed document was actually created or updated
2. The documentation reflects the actual implementation (no drift)
3. If a `docs-produced` item was not created, the epic is NOT done

**If `docs-produced` is empty or null:** No documentation output is expected — the gate is automatically satisfied.

---

## Idea-to-Epic Promotion Gate (NON-NEGOTIABLE)

An idea MUST NOT be promoted to an epic until:

1. **Status is `shaped`** — the idea has been through `exploring` and has clear scope
2. **All `research-needed` items are investigated** — research artifacts exist in `.orqa/delivery/research/` or the research question has been answered and documented in the idea body
3. **Pillar alignment confirmed** — at least one pillar is listed and justified
4. **User approves promotion** — the orchestrator presents the shaped idea and asks for explicit approval

### Promotion Procedure

1. Create `EPIC-NNN.md` in `.orqa/delivery/epics/` with:
   - `milestone` set to the appropriate milestone
   - `status: draft`
   - `priority` assessed per project criteria (see DOC-062)
   - `docs-required` populated based on what documentation needs to exist
   - `docs-produced` populated based on what documentation the work will create
2. Update the source `IDEA-NNN.md`:
   - Set `status: promoted`
   - Set `promoted-to: EPIC-NNN`
3. Update `.orqa/documentation/product/roadmap.md` if the epic adds to or modifies the roadmap

---

## Priority Assessment

### How Priority Is Determined

Priority is an inference-based judgement, not a formula. Each project defines its priority criteria in `.orqa/documentation/product/priority-assessment.md` (DOC-062). Agents read the criteria and assign a priority band (P1/P2/P3) based on how the epic serves the active milestone and whether it blocks other work.

The `scoring` field on epics is optional rationale — freeform dimensions that capture the agent's reasoning. Projects choose their own dimension names. What matters is that the `priority` band is defensible and the rationale is readable.

### Validation

- Every epic MUST have a `priority` field (P1/P2/P3)
- The `scoring` field is optional but recommended for traceability
- Priority should be reassessed when milestones change or significant scope shifts occur

### Deadline Override

A P2 or P3 epic with an imminent deadline (within 2 weeks) should be treated as effectively P1 for scheduling purposes. The priority field stays as assessed, but the orchestrator factors the deadline into work ordering.

---

## Milestone Gate Enforcement

A milestone MUST NOT be marked `complete` until:

1. **All P1 epics are `done`** — every epic with `priority: P1` in the milestone has `status: done`
2. **The gate question can be answered "yes"** — the orchestrator presents the gate question to the user and gets explicit confirmation

P2 and P3 epics may remain in-progress or draft when a milestone is completed — they carry forward or are re-assigned to the next milestone.

---

## Roadmap Synchronisation

### When Artifacts Change

The following changes MUST be reflected in `.orqa/documentation/product/roadmap.md`:

- New epic added to a milestone → add to roadmap under the milestone section
- Epic priority changes → update roadmap ordering
- New idea captured → add to roadmap ideas/future section if significant
- Idea promoted to epic → move from ideas section to the milestone section
- Milestone completed → update roadmap status

### Cross-Referencing

- Every epic in the roadmap must reference its `EPIC-NNN` ID
- Every idea in the roadmap must reference its `IDEA-NNN` ID
- Every milestone in the roadmap must reference its `MS-NNN` ID

---

## Epic Readiness Surfacing

When all tasks in an epic have `status: done` but the epic itself is not yet `status: done`, the orchestrator MUST proactively surface this to the user. This prevents epics from silently stalling between task completion and formal closure.

The surfacing must include:
1. List of completed tasks
2. Any observations logged during implementation (IMPL entries)
3. A prompt asking the user to review and approve epic completion

In the app, this should be surfaced via a dashboard notification or tool output. In the CLI, the orchestrator raises it in conversation.

---

## Observation Triage During Epics

When observations (IMPL entries) are created during an epic's implementation, a triage task MUST be auto-created on the first observation. Subsequent observations accumulate under the same triage task. The triage task is completed as part of epic closure.

For each observation, the triage outcome MUST be one of:
1. **Implement now** — the observation reveals a gap that blocks or undermines the epic's goals. Create a task within this epic.
2. **Promote** — the observation is mature enough to become a rule, skill update, or architectural decision. Do it in this epic or create a task.
3. **Defer to idea** — the observation is valid but out of scope. Create an `IDEA-NNN` with a relationship edge so it enters the planning pipeline.

"Leave it sitting" is NOT a valid triage outcome for an epic that is trying to close.

---

## Artifact Integrity Checks

The orchestrator SHOULD periodically verify:

1. **No orphaned artifacts** — every epic references an existing milestone, every task references an existing epic
2. **No broken references** — `depends-on`, `blocks`, `promoted-to`, `research-refs`, `supersedes`, `superseded-by` all point to existing artifacts
3. **Status consistency** — a milestone marked `active` has at least one `in-progress` or `ready` epic
4. **Frontmatter completeness** — all required fields are present and non-empty
5. **Research-refs / docs-required consistency** — every `RES-NNN` in `research-refs` either appears in `docs-required` or has a documented reason for omission
6. **Promotion chain integrity** — every lesson with `promoted-to: RULE-NNN` points to an existing rule, and that rule's `promoted-from` points back to the lesson

---

## FORBIDDEN Patterns

- Creating an epic without a milestone reference
- Starting implementation on an epic whose `docs-required` gate is not satisfied
- Promoting an idea directly to `promoted` from `captured` (skipping research)
- Marking a milestone complete when P1 epics are not done
- Leaving `promoted-to` null on an idea with `status: promoted`
- Creating duplicate IDs (two artifacts with the same ID)
- Modifying artifact IDs after creation
- Recording an architecture decision without a corresponding `AD-NNN.md` file in `.orqa/process/decisions/`
- Updating one side of a decision supersession without updating the other
- Using process words (UAT, Phase, Sprint, Round, Audit) in epic titles unless they describe the actual deliverable content — epic titles describe what is achieved, not how work is organised
- Marking an epic as `done` without explicit user approval (human gate violation)
- Closing an epic with untriaged observations — every IMPL entry created during the epic must have a forward path
- Creating an epic without a corresponding reconciliation task
- Marking an epic as done when the reconciliation task is not done
- Adding tasks to an epic without updating the epic body's task table

---

## Related Rules

- [RULE-008](RULE-008) (documentation-first) — documentation is the source of truth; artifacts enforce the documentation-first principle at the workflow level
- [RULE-016](RULE-016) (artifact-id-semantics) — IDs are identifiers not rankings; priority comes from scoring
- [RULE-031](RULE-031) (vision-alignment) — pillar alignment required for all artifacts
- [RULE-017](RULE-017) (lessons-learned) — lesson lifecycle and promotion pipeline
- [RULE-021](RULE-021) (pillar-alignment-docs) — pillar alignment in documentation pages
- [RULE-015](RULE-015) (honest-reporting) — artifact status must reflect reality
