---
id: SKILL-SW-001
type: skill
name: Software Delivery Management
status: active
layer: plugin
plugin: "@orqastudio/plugin-software-project"
relationships:
  - target: PILLAR-001
    type: grounded-by
---

# Software Delivery Management

You are managing software delivery artifacts in an OrqaStudio project. This skill covers how to create, connect, and progress milestones, epics, tasks, research, and wireframes through the delivery lifecycle.

## Artifact Types

### Milestone (MS-nnn)
Top of the delivery hierarchy. Represents a significant project checkpoint or release.
- Path: `.orqa/delivery/milestones/`
- Key fields: `gate` (optional conditions for milestone completion), `target_date`
- Children: epics connect via `delivers` relationship
- Traceability: milestones trace to pillars transitively through the graph (pillar → idea → epic → milestone), not via direct `grounded-by` links

### Epic (EPIC-nnn)
A body of work delivering a coherent capability. Groups related tasks.
- Path: `.orqa/delivery/epics/`
- Parent: connects to milestone via `delivers`
- Children: tasks connect via `delivers`
- Motivation: connect to the decision that drives this work via `driven-by`
- Origin: epics often `evolves-from` an idea that was validated

### Task (TASK-nnn)
An atomic unit of work that can be completed in a single session.
- Path: `.orqa/delivery/tasks/`
- Parent: connects to epic via `delivers`
- Dependencies: use `depends-on` for sequencing between tasks
- Context: connect to research and lessons via `informed-by`

### Research (RES-nnn)
Investigation or analysis that produces findings to inform decisions and work.
- Path: `.orqa/discovery/research/`
- Key fields: `methodology` (how the research was conducted)
- Output: research `informs` ideas, decisions, and epics
- Informed by: tasks can be `informed-by` research findings

### Wireframe (WF-nnn)
Visual specification of a UI or interaction pattern.
- Path: `.orqa/discovery/wireframes/`
- Output: wireframes `inform` the epics and tasks that implement them

## Creating Artifacts

Always include the full frontmatter:

```yaml
---
id: EPIC-083
type: epic
name: Plugin Distribution System
status: active
relationships:
  - target: MS-010
    type: delivers
  - target: AD-055
    type: driven-by
  - target: PILLAR-001
    type: grounded-by
---
```

Rules:
1. **Every artifact MUST have `id`, `type`, `status`** in frontmatter
2. **Relationships are bidirectional** — when you add `delivers` on a task, add `delivered-by` on the epic
3. **Use canonical relationship keys** — `delivers`, `driven-by`, `grounded-by`, `informed-by`, `informs`, `evolves-from`, `depends-on`
4. **`grounded-by` only points to pillars** — use `informed-by` for other knowledge flow
5. **`driven-by` only comes from decisions** — connect work to the decision that motivates it
6. **Next ID**: scan existing files to find the highest ID and increment

## Status Progression

Delivery statuses follow this flow:
```
captured → exploring → ready → prioritised → active → review → completed
                                                ↕         ↕
                                              hold     blocked
```

Auto-transition rules:
- When all child tasks are `completed` → parent epic moves to `review`
- When `depends-on` targets are unmet → status becomes `blocked`
- When blocked dependencies are met → status returns to `ready`

## Delivery Hierarchy

```
Milestone (MS)
  └── delivers ← Epic (EPIC)
                    └── delivers ← Task (TASK)
```

Query the hierarchy:
- "What delivers to MS-010?" → all epics delivering to that milestone
- "What delivers to EPIC-083?" → all tasks in that epic
- Filter by status to see what's active, blocked, or completed

## Connecting to Core Artifacts

Delivery artifacts connect to governance through the graph — traceability to pillars is transitive (pillar → idea → epic → task), not direct. Each artifact type connects to its nearest governance ancestor:

| Delivery type | → Relationship | → Core type | Purpose |
|---|---|---|---|
| Epic | `driven-by` | Decision | Motivated by architecture decision |
| Epic | `evolves-from` | Idea | Validated idea became work |
| Task | `informed-by` | Research | Research findings guide implementation |
| Task | `informed-by` | Lesson | Past lessons prevent repeat mistakes |
| Research | `informs` | Decision | Findings inform architecture choices |
| Wireframe | `informs` | Epic/Task | Visual spec guides implementation |

Pillars connect to ideas via `grounded-by` at the platform level. You can trace any task back to a pillar by walking: task → (delivers) → epic → (evolves-from) → idea → (grounded-by) → pillar.

## Validation

Run `orqa validate` before committing. The integrity checker verifies:
- All relationship targets exist
- All inverses are present
- Status values are canonical
- Required frontmatter fields are present
- Delivery hierarchy is consistent
