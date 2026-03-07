---
scope: project
---

# Artifact Lifecycle (NON-NEGOTIABLE)

Every structured artifact in `.orqa/` follows a defined lifecycle. This rule enforces creation standards, status transitions, promotion gates, documentation gates, and cross-referencing.

**Source of Truth:** `docs/product/artifact-framework.md`

---

## Artifact Creation Standards

### When to Create Artifacts

| Trigger | Artifact Type | Action |
|---------|--------------|--------|
| User mentions a future feature or "we should eventually..." | `IDEA-NNN` | Create in `.orqa/ideas/` with `status: captured` |
| User approves an idea for investigation | Update existing `IDEA-NNN` | Set `status: exploring`, begin research |
| Research validates an idea for implementation | `EPIC-NNN` | Create in `.orqa/epics/` with `status: draft`, update idea `promoted-to` |
| An epic needs a design document | Plan file | Create in `.orqa/plans/`, reference from epic `plan` field |
| An epic is approved and scoped for implementation | Update `EPIC-NNN` | Set `status: ready` (requires `docs-required` gate satisfied) |
| A task within an epic needs detailed tracking | `TASK-NNN` | Create in `.orqa/tasks/` with `epic` reference |
| A strategic goal is defined | `MS-NNN` | Create in `.orqa/milestones/` |
| An implementation reveals a reusable pattern | `IMPL-NNN` | Create in `.orqa/lessons/` (see `lessons-learned.md`) |
| A question needs investigation before a decision | Research file | Create in `.orqa/research/` |

### ID Assignment

IDs auto-increment per type. To determine the next ID, scan existing files in the type's directory and increment the highest number. IDs are never reused after deletion.

### Required Fields

Every artifact MUST have all fields marked "Required" in the schema defined in `docs/product/artifact-framework.md`. Missing required fields are a review FAIL.

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

### Task

```
todo ──> in-progress ──> done
```

- `todo → in-progress`: Parent epic is `in-progress`, agent is assigned
- `in-progress → done`: Acceptance criteria met, verified by reviewer

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

---

## Documentation Gate (NON-NEGOTIABLE)

### Before Epic Implementation Starts (`draft → ready`)

Every epic's `docs-required` field lists documentation that MUST exist before implementation begins. The orchestrator MUST verify:

1. Each listed document exists at the specified path
2. Each document is current (not a stale placeholder)
3. If a document is missing, it must be created and approved BEFORE the epic moves to `ready`

**If `docs-required` is empty or null:** The epic has no documentation prerequisites — the gate is automatically satisfied.

**If any `docs-required` item is missing:** The epic is blocked. Document the gap and create the documentation first.

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
2. **All `research-needed` items are investigated** — research artifacts exist in `.orqa/research/` or the research question has been answered and documented in the idea body
3. **Pillar alignment confirmed** — at least one pillar is listed and justified
4. **User approves promotion** — the orchestrator presents the shaped idea and asks for explicit approval

### Promotion Procedure

1. Create `EPIC-NNN.md` in `.orqa/epics/` with:
   - `milestone` set to the appropriate milestone
   - `status: draft`
   - `priority` computed from scoring dimensions
   - `docs-required` populated based on what documentation needs to exist
   - `docs-produced` populated based on what documentation the work will create
2. Update the source `IDEA-NNN.md`:
   - Set `status: promoted`
   - Set `promoted-to: EPIC-NNN`
3. Update `docs/product/roadmap.md` if the epic adds to or modifies the roadmap
4. Update the parent `MS-NNN.md` milestone's `epic-count` if applicable

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
3. **Epic counts are accurate** — `epic-count` and `completed-epics` match the actual state of `.orqa/epics/`

P2 and P3 epics may remain in-progress or draft when a milestone is completed — they carry forward or are re-assigned to the next milestone.

---

## Roadmap Synchronisation

### When Artifacts Change

The following changes MUST be reflected in `docs/product/roadmap.md`:

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
2. **No broken references** — `depends-on`, `blocks`, `promoted-to`, `plan` all point to existing artifacts
3. **Status consistency** — a milestone marked `active` has at least one `in-progress` or `ready` epic
4. **Count accuracy** — milestone `epic-count` and `completed-epics` match reality
5. **Frontmatter completeness** — all required fields are present and non-empty

---

## FORBIDDEN Patterns

- Creating an epic without a milestone reference
- Starting implementation on an epic whose `docs-required` gate is not satisfied
- Promoting an idea directly to `promoted` from `captured` (skipping research)
- Marking a milestone complete when P1 epics are not done
- Leaving `promoted-to` null on an idea with `status: promoted`
- Creating duplicate IDs (two artifacts with the same ID)
- Modifying artifact IDs after creation

---

## Related Rules

- `documentation-first.md` — documentation is the source of truth; artifacts enforce the documentation-first principle at the workflow level
- `vision-alignment.md` — pillar alignment required for all artifacts
- `plan-mode-compliance.md` — plans referenced by epics must meet plan structure requirements
- `lessons-learned.md` — lesson lifecycle and promotion pipeline
- `pillar-alignment-docs.md` — pillar alignment in documentation pages
- `honest-reporting.md` — artifact status must reflect reality
