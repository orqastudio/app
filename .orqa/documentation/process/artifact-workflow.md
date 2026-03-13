---
id: DOC-025
title: Artifact Workflow
description: Day-to-day workflow for creating, transitioning, and managing artifacts through their lifecycle.
created: "2026-03-07"
updated: "2026-03-09"
---

**Date:** 2026-03-07

This document describes how artifacts flow through the OrqaStudio™ development process day-to-day. It covers when artifacts are created, how they transition between states, and the gates that govern each transition.

For artifact schemas and field definitions, see `.orqa/documentation/product/artifact-framework.md`.
For enforcement rules, see [RULE-004](RULE-004).

---

## The Artifact Lifecycle at a Glance

```
User has an idea
  → Idea captured (IDEA-NNN, status: captured)
  → Research investigates (status: exploring, .orqa/delivery/research/ artifacts created)
  → Idea scoped and validated (status: shaped)
  → User approves promotion (status: promoted, EPIC-NNN created)

Epic implementation
  → Documentation gate checked (docs-required satisfied → status: ready)
  → Worktree created, agent assigned (status: in-progress)
  → Implementation complete (status: review)
  → Verification gates passed, docs-produced verified (status: done)

Lessons along the way
  → Implementation patterns captured (IMPL-NNN in .orqa/process/lessons/)
  → Recurring patterns promoted to rules/skills

Milestone completion
  → All P1 epics done
  → Gate question answered "yes"
  → Milestone status: complete
```

---

## Capturing Ideas

### When

Whenever the user mentions a future feature, enhancement, or "we should eventually..." concept.

### How

1. Scan `.orqa/delivery/ideas/` to determine the next ID
2. Create `IDEA-NNN.md` with:
   - `status: captured`
   - `pillar` alignment (at least one pillar must apply)
   - `research-needed` listing what needs investigation
   - `tags` for discoverability
3. Add a brief entry in `.orqa/documentation/product/roadmap.md` if the idea is significant enough for roadmap visibility
4. Inform the user the idea has been captured

### What NOT to Do

- Do not start investigating or implementing immediately
- Do not create an epic — ideas need research and shaping first
- Do not skip `research-needed` — every idea has questions that need answering

---

## Exploring and Shaping Ideas

### Exploring (captured → exploring)

Triggered when the user approves investigation of a captured idea.

1. Update `IDEA-NNN.md`: set `status: exploring`
2. For each item in `research-needed`:
   - Create or update research artifacts in `.orqa/delivery/research/`
   - Investigate technical feasibility, UX implications, architectural fit
   - Document findings in the research artifacts
3. If research produces an architectural choice, create an `AD-NNN.md` in `.orqa/process/decisions/` (see Decision Creation below)

### Shaping (exploring → shaped)

When all `research-needed` items have been investigated:

1. Update `IDEA-NNN.md`: set `status: shaped`
2. Summarise findings in the idea body:
   - Proposed scope and approach
   - Technical feasibility assessment
   - Effort estimate
   - Dependencies on other work
3. Present the shaped idea to the user for promotion decision

---

## Promoting Ideas to Epics

### Gate (shaped → promoted)

The idea MUST be `shaped` before promotion. The user MUST explicitly approve.

### Procedure

1. Compute priority score using `.orqa/project.json` dimensions
2. Create `EPIC-NNN.md` in `.orqa/delivery/epics/` with:
   - `status: draft`
   - `milestone` assignment
   - `priority` derived from score
   - `docs-required` — documentation that must exist before implementation
   - `docs-produced` — documentation the implementation will create/update
   - Implementation design in the epic body — approach, phases, and acceptance criteria
3. Update `IDEA-NNN.md`:
   - Set `status: promoted`
   - Set `promoted-to: EPIC-NNN`
4. Update `.orqa/documentation/product/roadmap.md` to include the new epic
5. Update the parent milestone's `epic-count`

---

## Epic Implementation Flow

### Documentation Gate (draft → ready)

Before an epic can begin implementation:

1. The orchestrator checks every item in `docs-required`
2. Each listed document must exist and be current
3. If any document is missing, create it first (delegate to `documentation-writer`)
4. Once all `docs-required` items are satisfied, set `status: ready`

### Starting Work (ready → in-progress)

1. Verify the epic meets the Definition of Ready (see [RULE-004](RULE-004) epic gates)
2. Create a worktree: `git worktree add ../orqa-<epic-name> -b <branch>`
3. Assign the appropriate agent(s)
4. Set `status: in-progress`
5. Update `assignee` field

### Implementation

1. Follow the implementation design in the epic body
2. Tasks are tracked as checklist items in the epic body
3. If a task needs its own tracking, graduate it to `TASK-NNN.md` in `.orqa/delivery/tasks/`
4. Commit regularly to the worktree branch
5. Capture implementation lessons in `.orqa/process/lessons/`

### Review (in-progress → review)

When implementation is complete:

1. Set `status: review`
2. Run verification gates:
   - `code-reviewer`: code quality, compliance, end-to-end completeness
   - `qa-tester`: functional correctness, smoke tests
   - `ux-reviewer`: UI compliance (if applicable)
3. Verify all `docs-produced` items were actually created or updated
4. If any gate fails, fix and re-review

### Completion (review → done)

When all verification gates pass:

1. Set `status: done`
2. Merge worktree to main
3. Clean up worktree and branch
4. Update parent milestone `completed-epics` count
5. Log any new lessons in `.orqa/process/lessons/`

---

## Task Graduation

Most tasks live as checklist items in their parent epic:

```markdown
## Tasks

- [x] Emit SystemPromptSent event from stream_commands.rs
- [ ] Create ContextEntry component
- [ ] Wire context entries into ConversationView
```

A task graduates to a separate `TASK-NNN.md` file when it needs:

- Detailed acceptance criteria
- Agent assignment tracking
- Discussion or design notes
- Its own scope and file list

### Creating a Graduated Task

1. Scan `.orqa/delivery/tasks/` for the next ID
2. Create `TASK-NNN.md` with `epic` reference
3. Replace the checklist item with a reference: `- [ ] [TASK-NNN] Emit SystemPromptSent event`

---

## Milestone Lifecycle

### Activation (planning → active)

A milestone becomes active when:

- At least one of its epics has `status: ready` or later
- The user has confirmed the milestone's strategic direction

### Completion (active → complete)

A milestone is complete when:

1. All P1 epics have `status: done`
2. The user confirms the gate question can be answered "yes"
3. `epic-count` and `completed-epics` are accurate

P2/P3 epics may still be in progress — they carry forward to the next milestone or remain in the current one.

---

## Lesson Lifecycle

See [RULE-017](RULE-017) for the full lesson lifecycle.

Summary:

1. Implementation patterns discovered during work → create `IMPL-NNN.md`
2. Track recurrence when the same pattern appears again
3. At recurrence threshold (>= 2): promote to a rule, coding standard, or skill
4. Update the lesson's `promoted-to` field after promotion

---

## Decision Creation

### When

When research produces an architectural choice that affects the system — a technology selection, a structural constraint, an interface contract, or a rejected alternative with a documented reason.

### How

1. Scan `.orqa/process/decisions/` to determine the next ID (`AD-NNN`)
2. Create `AD-NNN.md` with required frontmatter:
   - `status: proposed` initially; advance to `accepted` once the user has reviewed and approved
   - `category` set to the appropriate domain (`ipc`, `data`, `ui`, `security`, `tooling`, `process`)
   - `research-refs` linking to the research artifact(s) that produced this decision
3. Write the decision body in three sections:
   - **Context** — what situation or question prompted this decision
   - **Decision** — what was chosen and the key reasons
   - **Consequences** — what becomes easier, harder, or newly constrained
### Supersession

When a new decision replaces an existing accepted decision:

1. Create the new `AD-NNN.md` with `supersedes: AD-<old>` in frontmatter
2. Update the superseded decision: set `status: superseded` and `superseded-by: AD-<new>`
3. Both artifacts MUST be updated in the same commit — a supersession is incomplete if only one side is updated

### What NOT to Do

- Every decision MUST be an individual `AD-NNN.md` artifact in `.orqa/process/decisions/`
- Do not modify an accepted decision in place — supersede it with a new decision instead
- Do not leave a decision at `proposed` indefinitely — either accept it or archive it with a reason

---

## Roadmap Synchronisation

The roadmap (`.orqa/documentation/product/roadmap.md`) must stay in sync with artifacts:

| Event | Roadmap Update |
|-------|----------------|
| New idea captured | Add to ideas/future section (if significant) |
| Idea promoted to epic | Move from ideas to the appropriate milestone section |
| New epic added | Add under milestone section with ID reference |
| Epic priority changes | Update ordering within milestone section |
| Milestone completed | Update milestone status in roadmap |

---

## Cross-Reference Integrity

The orchestrator periodically verifies:

- Every epic's `milestone` points to an existing `MS-NNN.md`
- Every task's `epic` points to an existing `EPIC-NNN.md`
- Every epic's `depends-on` and `blocks` point to existing epics
- Every idea's `promoted-to` (when set) points to an existing epic
- Every epic's `research-refs` (when set) point to existing research files in `.orqa/delivery/research/`
- Every decision's `supersedes` (when set) points to an existing `AD-NNN.md`
- Every decision's `superseded-by` (when set) points to an existing `AD-NNN.md`
- Milestone `epic-count` matches the actual number of epics referencing it
- Milestone `completed-epics` matches the count of epics with `status: done`

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The artifact workflow makes the entire development process visible and navigable — from initial idea through research, planning, implementation, and completion. Every decision is recorded in a structured, browsable format. |
| Learning Through Reflection | Ideas, research, and lessons form a learning loop. The promotion pipeline (idea→epic, lesson→rule, research→decision) ensures knowledge compounds and mistakes are not repeated. |

---

## Related Documents

- Artifact Framework — Schemas, field definitions, design principles
- Definition of Ready — Gate checklist before task/epic implementation starts
- Definition of Done — Gate checklist before task/epic is marked complete
- Workflow — General development workflow and task lifecycle
- Orchestration — Orchestrator responsibilities
