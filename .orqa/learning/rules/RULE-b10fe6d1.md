---
id: "RULE-b10fe6d1"
type: rule
title: "Artifact Lifecycle"
description: "Enforces creation standards, status transitions, promotion gates, and documentation gates for all .orqa/ artifacts."
status: active
enforcement_type: mechanical
created: "2026-03-07"
updated: "2026-03-13"
enforcement:

  - mechanism: behavioral

    message: "Orchestrator enforces human gate before marking epics done; status transitions must follow defined lifecycle gates"

  - mechanism: tool

    command: "orqa enforce"
    description: "Validates artifact status transitions, required fields, and relationship integrity"

  - mechanism: pre-commit

    check: "orqa enforce"
relationships:

  - target: "DOC-db5b37dc"

    type: "documented-by"

  - target: "DOC-ffad3f6b"

    type: "documented-by"

  - target: "PD-3b986859"

    type: "enforces"

  - target: "PD-7fa3f280"

    type: "enforces"

  - target: "PD-339e9223"

    type: "enforces"

  - target: "DOC-28344cd7"

    type: "documented-by"

  - target: "DOC-f6c4ac69"

    type: "documented-by"
---
Every structured artifact in `.orqa/` follows a defined lifecycle. This rule enforces creation standards, status transitions, promotion gates, documentation gates, and cross-referencing.

**Source of Truth:** `.orqa/documentation/about/artifact-framework.md`

---

## Artifact Creation Standards

### When to Create Artifacts

| Trigger | Artifact Type | Action |
| --- | --- | --- |
| User mentions a future feature or "we should eventually..." | `IDEA-NNN` | Create in `.orqa/implementation/ideas/` with `status: captured` |
| User approves an idea for investigation | Update existing `IDEA-NNN` | Set `status: exploring`, begin research |
| Research validates an idea for implementation | `EPIC-NNN` | Create in `.orqa/implementation/epics/` with `status: draft`, update idea `evolves-into` |
| An epic needs investigation work before implementation | Research file | Create in `.orqa/implementation/research/`; reference from epic `research-refs` field. Implementation design goes in the epic body. |
| An epic is approved and scoped for implementation | Update `EPIC-NNN` | Set `status: ready` (requires `docs-required` gate satisfied) |
| A task within an epic needs detailed tracking | `TASK-NNN` | Create in `.orqa/implementation/tasks/` with `epic:` reference |
| An epic is created or moves to `ready` | Reconciliation `TASK-NNN` | Auto-create a standing reconciliation task for the epic (see Epic Reconciliation Task below) |
| A strategic goal is defined | `MS-NNN` | Create in `.orqa/implementation/milestones/` |
| An implementation reveals a reusable pattern | `IMPL-NNN` | Create in `.orqa/process/lessons/` (see [RULE-c603e90e](RULE-c603e90e) (lessons-learned)) |
| A question needs investigation before a decision | Research file | Create in `.orqa/implementation/research/` |
| Research produces an architectural choice | `AD-NNN` | Create in `.orqa/process/decisions/` |

### ID Assignment

IDs auto-increment per type. To determine the next ID, scan existing files in the type's directory and increment the highest number. IDs are never reused after deletion.

### Required Fields

Every artifact MUST have all fields marked "Required" in the schema defined in `.orqa/documentation/about/artifact-framework.md`. Missing required fields are a review FAIL.

---

## Status Transition Rules

Status transitions MUST follow the defined workflows. Skipping states is forbidden unless explicitly noted.

### Milestone

```text
planning ──> active ──> complete
```text

- `planning → active`: At least one epic exists with `status: ready` or later
- `active → complete`: The milestone's `gate` question can be answered "yes" — all P1 epics are `done`

### Epic

```text
draft ──> ready ──> in-progress ──> review ──> done
```text

- `draft → ready`: All `docs-required` items exist and are approved (Documentation Gate — see below)
- `ready → in-progress`: Epic meets Definition of Ready, worktree created, agent assigned
- `in-progress → review`: Implementation complete, submitted for verification gates
- `review → done`: **Human gate (NON-NEGOTIABLE)** — the orchestrator presents a completion summary to the user and receives explicit approval. The summary must include: tasks completed, docs-produced verification, lessons logged during implementation, and any scope changes. The orchestrator MUST NOT mark an epic as done without user confirmation. All verification gates must also have passed (code-reviewer, qa-tester, ux-reviewer), and all `docs-produced` items verified as created/updated

The epic body contains the implementation design — data model, IPC contracts, component breakdown, and approach. For investigation-heavy work, the epic may carry a `research-refs` field listing research documents in `.orqa/implementation/research/` that informed the design.

### Epic Reconciliation Task (NON-NEGOTIABLE)

Every epic MUST have a standing reconciliation task created when the epic is created or moves to `ready`. This task enforces ongoing epic body maintenance throughout the epic lifecycle — not just at closure.

**Creation:** When an epic is created, the orchestrator auto-creates a task:

```text
TASK-NNN: "Reconcile EPIC-NNN"
epic: EPIC-NNN
status: in-progress (active for the duration of the epic)
enforcement_type: mechanical
acceptance:
  - "Epic task table lists ALL tasks created during the epic"
  - "Epic pillars array reflects all pillars served"
  - "Epic docs-produced list matches actual documentation created/updated"
  - "Epic scope section accurately reflects what was in/out of scope"
  - "No tasks exist for this epic that are missing from the epic body"
```text

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

```text
todo ──> in-progress ──> done
```text

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

```text
draft ──> complete ──> surpassed
```text

- Research documents capture investigation findings and feed into epics or architecture decisions.
- A research document may be marked `surpassed` when newer investigation supersedes it. Set `surpassed-by` field. Do NOT delete — research documents are historical records of reasoning and findings.

### Idea

```text
captured ──> exploring ──> shaped ──> promoted ──> delivered
                                  │            └──> partially-delivered
                                  └──> archived
Any state ──> discarded
Any state ──> archived
```text

- `captured → exploring`: User approves investigation. Research begins on `research-needed` items.
- `exploring → shaped`: All `research-needed` items have been investigated. Research artifacts exist. The idea has a clear scope and proposed approach.
- `shaped → promoted`: User approves promotion. An `EPIC-NNN` is created. The idea's `evolves-into` field is set to the epic ID.
- `promoted → delivered`: The epic referenced by `evolves-into` has `status: done` and all scope from the original idea has been implemented. **Automated**: the integrity engine transitions this when the epic is done.
- `promoted → partially-delivered`: The epic is `done` but only part of the idea's scope was delivered. The user is prompted to either discard the remaining scope or spin off a new idea (`IDEA-NNN`) for the undelivered portion.
- `shaped → archived`: User decides not to pursue. Reason documented in the idea body.
- Any state → `archived`: User explicitly archives. "Not now, maybe later."
- Any state → `discarded`: User rejects the idea. "No, this isn't worth pursuing." Reason documented in the idea body.

**Partially-delivered workflow:**

1. Integrity engine detects `promoted` idea whose epic is `done`
2. Engine compares idea scope against epic deliverables
3. If fully covered → auto-transition to `delivered`
4. If partially covered → flag as `partially-delivered`, surface to user
5. User decides: discard remaining scope, or create a new idea for the undelivered parts
6. If new idea created, the original idea body documents what was spun off

**FORBIDDEN transitions:**

- `captured → promoted` — skipping research/shaping is not allowed
- `exploring → promoted` — must be shaped (scoped and validated) before promotion
- `delivered → any` — delivered is terminal
- `discarded → any` — discarded is terminal (reopen by creating a new idea instead)
- Any backward transition without user approval

### Decision

```text
proposed ──> accepted ──> superseded
                      └──> deprecated
```text

- `proposed → accepted`: Decision reviewed and approved by the user
- `accepted → superseded`: A new decision replaces this one — both the new and old artifacts MUST be updated in the same commit
- `accepted → deprecated`: Decision is no longer relevant (technology removed, context changed) — reason documented in the decision body

**Creation rule:** When research produces an architectural choice, an `AD-NNN.md` MUST be created in `.orqa/process/decisions/` following the Decision schema in `.orqa/documentation/about/artifact-framework.md`.

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
2. **All `research-needed` items are investigated** — research artifacts exist in `.orqa/implementation/research/` or the research question has been answered and documented in the idea body
3. **Pillar alignment confirmed** — at least one pillar is listed and justified
4. **Related ideas scanned** — before creating the epic, scan all ideas for thematic overlap. Related ideas may be bundled into the same epic or explicitly noted as separate scope
5. **User approves promotion** — the orchestrator presents the shaped idea, any related ideas found, and asks for explicit approval

### Related Idea Scan (MANDATORY)

Before creating the promotion epic, the orchestrator MUST:

1. Scan all `IDEA-NNN.md` files in `.orqa/implementation/ideas/` with status `captured`, `exploring`, or `shaped`
2. Identify ideas with thematic overlap (similar title, description, or pillar alignment)
3. Present related ideas to the user with a recommendation: bundle into the same epic, or note as separate scope
4. If the user chooses to bundle, include the related ideas' scope in the epic and set their `evolves-into` fields
5. If the user chooses to keep separate, document the relationship in the epic body for future reference

This prevents the pattern where multiple ideas addressing the same capability are promoted as separate epics that could have been a single coherent effort.

### Promotion Procedure

1. **Scan for related ideas** (see Related Idea Scan above)
2. Create `EPIC-NNN.md` in `.orqa/implementation/epics/` with:
   - `milestone` set to the appropriate milestone
   - `status: draft`
   - `priority` assessed per project criteria (see DOC-54594c57)
   - `docs-required` populated based on what documentation needs to exist
   - `docs-produced` populated based on what documentation the work will create
3. Update the source `IDEA-NNN.md`:
   - Set `status: promoted`
   - Set `promoted-to: EPIC-NNN`
4. Update any bundled ideas (set `status: promoted`, `promoted-to: EPIC-NNN`)
5. Update `.orqa/documentation/about/roadmap.md` if the epic adds to or modifies the roadmap

---

## Priority Assessment

### How Priority Is Determined

Priority is an inference-based judgement, not a formula. Each project defines its priority criteria in `.orqa/documentation/about/priority-assessment.md` (DOC-54594c57). Agents read the criteria and assign a priority band (P1/P2/P3) based on how the epic serves the active milestone and whether it blocks other work.

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

The following changes MUST be reflected in `.orqa/documentation/about/roadmap.md`:

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
2. **Promote** — the observation is mature enough to become a rule, knowledge update, or architectural decision. Do it in this epic or create a task.
3. **Defer to idea** — the observation is valid but out of scope. Create an `IDEA-NNN` with a relationship edge so it enters the planning pipeline.

"Leave it sitting" is NOT a valid triage outcome for an epic that is trying to close.

---

## Planning Placement Rule (NON-NEGOTIABLE)

Every planning artifact (idea, epic, task) must be **placed** — either reachable from a milestone or explicitly assigned a planning horizon. Unplaced artifacts are invisible to planning and integrity checks.

### Placement Resolution

An artifact is **placed** if any of these conditions hold:

| Artifact | Direct placement | Indirect placement |
| --- | --- | --- |
| **Epic** | Has `milestone` set | — |
| **Task** | Has `milestone` set | Has `epic` → that epic has `milestone` |
| **Idea** | Has `milestone` set | Has `evolves-into` → that epic has `milestone` |

If none of the above → the artifact MUST have a `horizon` field:

| Horizon | Meaning | Integrity behaviour |
| --- | --- | --- |
| `active` | Actively planned for current work | Surfaced in dashboard planning view |
| `next` | Think about for the next milestone | Reviewed during milestone transitions |
| `later` | Worth doing, no timeline yet | Visible in backlog views |
| `someday` | Parked deliberately, revisit periodically | Lowest visibility, periodic review |
| `null` / unset | **Untriaged** | Integrity engine flags: "This artifact has no planning placement" |

### Automated Transitions

The integrity engine SHOULD automate these transitions:

1. **Idea delivery**: When an epic reaches `done`, all ideas with `evolves-into` pointing to it transition to `delivered` (if fully covered) or `partially-delivered` (if only partly covered)
2. **Untriaged flagging**: Any planning artifact with no milestone (direct or indirect) and no horizon is flagged as untriaged
3. **Milestone transition review**: When a milestone moves to `complete`, all `horizon: next` artifacts are surfaced for triage — should they become `active` for the new milestone?

---

## Artifact Integrity Checks

The orchestrator SHOULD periodically verify:

1. **No orphaned artifacts** — every epic references an existing milestone, every task references an existing epic
2. **No broken references** — `depends-on`, `blocks`, `evolves-into`, `research-refs`, `evolves-into`, `evolves-from` all point to existing artifacts
3. **Status consistency** — a milestone marked `active` has at least one `in-progress` or `ready` epic
4. **Frontmatter completeness** — all required fields are present and non-empty
5. **Research-refs / docs-required consistency** — every `RES-NNN` in `research-refs` either appears in `docs-required` or has a documented reason for omission
6. **Promotion chain integrity** — every lesson with `promoted-to: RULE-NNN` points to an existing rule, and that rule's `promoted-from` points back to the lesson
7. **Planning placement** — every idea, epic, and task is either reachable from a milestone or has a horizon set. Unplaced artifacts are flagged as untriaged.
8. **Idea delivery tracking** — every `promoted` idea whose epic is `done` should be transitioned to `delivered` or `partially-delivered`

---

## FORBIDDEN Patterns

- Creating an epic without either a milestone or a horizon (must be placed)
- Starting implementation on an epic whose `docs-required` gate is not satisfied
- Promoting an idea directly to `promoted` from `captured` (skipping research)
- Marking a milestone complete when P1 epics are not done
- Leaving `evolves-into` null on an idea with `status: promoted`
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

## Artifact Cross-Reference Format

All artifact cross-references MUST use markdown link syntax where the link target is the artifact ID matching the standard format. The display text is typically the artifact ID itself: `[EPIC-797972a7](EPIC-797972a7)`.

**FORBIDDEN**: bare artifact IDs in prose (e.g., `see EPIC-797972a7`), file paths, or web-style paths.

---

## Related Rules

- [RULE-ec9462d8](RULE-ec9462d8) (documentation-first) — documentation is the source of truth; artifacts enforce the documentation-first principle at the workflow level
- [RULE-25baac14](RULE-25baac14) (artifact-id-semantics) — IDs are identifiers not rankings; priority comes from scoring
- [RULE-1b238fc8](RULE-1b238fc8) (vision-alignment) — pillar alignment required for all artifacts
- [RULE-c603e90e](RULE-c603e90e) (lessons-learned) — lesson lifecycle and promotion pipeline
- [RULE-05562ed4](RULE-05562ed4) (pillar-alignment-docs) — pillar alignment in documentation pages
- [RULE-5dd9decd](RULE-5dd9decd) (honest-reporting) — artifact status must reflect reality
