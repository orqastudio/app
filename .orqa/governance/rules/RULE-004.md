---
id: RULE-004
title: Artifact Lifecycle
description: Enforces creation standards, status transitions, promotion gates, and documentation gates for all .orqa/ artifacts.
status: active
created: "2026-03-07"
updated: "2026-03-07"
layer: canon
scope: project
---
Every structured artifact in `.orqa/` follows a defined lifecycle. This rule enforces creation standards, status transitions, promotion gates, documentation gates, and cross-referencing.

**Source of Truth:** `.orqa/documentation/product/artifact-framework.md`

---

## Artifact Creation Standards

### When to Create Artifacts

| Trigger | Artifact Type | Action |
|---------|--------------|--------|
| User mentions a future feature or "we should eventually..." | `IDEA-NNN` | Create in `.orqa/planning/ideas/` with `status: captured` |
| User approves an idea for investigation | Update existing `IDEA-NNN` | Set `status: exploring`, begin research |
| Research validates an idea for implementation | `EPIC-NNN` | Create in `.orqa/planning/epics/` with `status: draft`, update idea `promoted-to` |
| An epic needs investigation work before implementation | Research file | Create in `.orqa/planning/research/`; reference from epic `research-refs` field. Implementation design goes in the epic body. |
| An epic is approved and scoped for implementation | Update `EPIC-NNN` | Set `status: ready` (requires `docs-required` gate satisfied) |
| A task within an epic needs detailed tracking | `TASK-NNN` | Create in `.orqa/planning/tasks/` with `epic:` reference |
| A strategic goal is defined | `MS-NNN` | Create in `.orqa/planning/milestones/` |
| An implementation reveals a reusable pattern | `IMPL-NNN` | Create in `.orqa/governance/lessons/` (see [RULE-017](RULE-017) (lessons-learned)) |
| A question needs investigation before a decision | Research file | Create in `.orqa/planning/research/` |
| Research produces an architectural choice | `AD-NNN` | Create in `.orqa/governance/decisions/`, add entry to `.orqa/documentation/architecture/decisions.md` index |

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
- `review → done`: All verification gates passed (code-reviewer, qa-tester, ux-reviewer), all `docs-produced` items verified as created/updated

The epic body contains the implementation design — data model, IPC contracts, component breakdown, and approach. For investigation-heavy work, the epic may carry a `research-refs` field listing research documents in `.orqa/planning/research/` that informed the design.

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

**Creation rule:** When research produces an architectural choice, an `AD-NNN.md` MUST be created in `.orqa/governance/decisions/` following the Decision schema in `.orqa/documentation/product/artifact-framework.md`.

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
2. **All `research-needed` items are investigated** — research artifacts exist in `.orqa/planning/research/` or the research question has been answered and documented in the idea body
3. **Pillar alignment confirmed** — at least one pillar is listed and justified
4. **User approves promotion** — the orchestrator presents the shaped idea and asks for explicit approval

### Promotion Procedure

1. Create `EPIC-NNN.md` in `.orqa/planning/epics/` with:
   - `milestone` set to the appropriate milestone
   - `status: draft`
   - `priority` computed from scoring dimensions
   - `docs-required` populated based on what documentation needs to exist
   - `docs-produced` populated based on what documentation the work will create
2. Update the source `IDEA-NNN.md`:
   - Set `status: promoted`
   - Set `promoted-to: EPIC-NNN`
3. Update `.orqa/documentation/product/roadmap.md` if the epic adds to or modifies the roadmap

---

## Priority Scoring Enforcement

### When to Score

Every epic MUST have a `scoring` block with all dimensions defined in `.orqa/project.json`. The `score` field is computed from the scoring formula. The `priority` field (P1/P2/P3) is derived from the score using the bands in `project.json`.

### Validation

- All dimension keys in the epic's `scoring` block must match the keys in `project.json`
- All dimension values must be within their defined scale
- The `score` field must match the computed result of the formula
- The `priority` band must match the score against the defined bands

### Deadline Override

A P2 or P3 epic with an imminent deadline (within 2 weeks) should be treated as effectively P1 for scheduling purposes. The priority field stays as computed, but the orchestrator factors the deadline into work ordering.

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
- Recording an architecture decision only in `.orqa/documentation/architecture/decisions.md` without a corresponding `AD-NNN.md` file in `.orqa/governance/decisions/`
- Updating one side of a decision supersession without updating the other
- Using process words (UAT, Phase, Sprint, Round, Audit) in epic titles unless they describe the actual deliverable content — epic titles describe what is achieved, not how work is organised

---

## Related Rules

- [RULE-008](RULE-008) (documentation-first) — documentation is the source of truth; artifacts enforce the documentation-first principle at the workflow level
- [RULE-031](RULE-031) (vision-alignment) — pillar alignment required for all artifacts
- [RULE-017](RULE-017) (lessons-learned) — lesson lifecycle and promotion pipeline
- [RULE-021](RULE-021) (pillar-alignment-docs) — pillar alignment in documentation pages
- [RULE-015](RULE-015) (honest-reporting) — artifact status must reflect reality
