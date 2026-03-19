---
id: SKILL-SW-1d47d8d8
type: skill
name: Software Delivery Management
status: active
plugin: "@orqastudio/plugin-software-project"
relationships:
  - target: DOC-SW-421219ce
    type: synchronised-with
---

# Software Delivery Management

You are managing software delivery artifacts in an OrqaStudio project. This skill covers how to create, connect, and progress milestones, epics, tasks, research, wireframes, and bugs through the delivery lifecycle.

## Artifact Types

### Milestone (MS-nnn)
Top of the delivery hierarchy. Represents a significant project checkpoint or release.
- Path: `.orqa/delivery/milestones/`
- Key fields: `gate` (optional completion conditions), `target_date`
- Children: epics connect via `fulfils`

### Epic (EPIC-nnn)
A body of work delivering a coherent capability. Groups related tasks.
- Path: `.orqa/delivery/epics/`
- Connects up: `fulfils` → milestone
- Connects down: tasks connect via `delivers`
- Origin: `realised-by` → idea (the validated idea that became this work)
- Motivation: `driven-by` → decision (the architecture choice that motivates this)
- Knowledge: `guided-by` → research, `cautioned-by` → lesson

### Task (TASK-nnn)
An atomic unit of work completable in a single session.
- Path: `.orqa/delivery/tasks/`
- Connects up: `delivers` → epic
- Sequencing: `depends-on` → other tasks
- Output: `yields` → lesson (when something is learned)
- Fixes: `fixes` → bug (when fixing a reported issue)

### Research (RES-nnn)
Investigation or analysis that produces findings.
- Path: `.orqa/discovery/research/`
- Origin: `spawned-by` → idea
- Output: `produces` → wireframe, `informs` → decision, `guides` → epic

### Wireframe (WF-nnn)
Visual specification of a UI or interaction pattern.
- Path: `.orqa/discovery/wireframes/`
- Origin: `produced-by` → research

### Bug (BUG-nnn)
A functional or display issue reported against existing work.
- Path: `.orqa/discovery/bugs/`
- Reports against: `reports` → epic, task, or milestone
- Impact: `affects` → persona
- Resolution: `fixed-by` → task

## Creating Artifacts

Always include full frontmatter:

```yaml
---
id: EPIC-596dc061
type: epic
name: Plugin Distribution System
status: active
relationships:
  - target: MS-010
    type: fulfils
  - target: AD-c6abc8e6
    type: driven-by
  - target: IDEA-3354aefe
    type: realised-by
---
```

Rules:
1. **Every artifact MUST have `id`, `type`, `status`**
2. **Relationships are bidirectional** — when you add `delivers` on a task, add `delivered-by` on the epic
3. **Use the correct verb** — each relationship has specific from/to types. The verb constrains usage.
4. **Next ID**: scan existing files to find the highest ID and increment

## Relationship Quick Reference

| From | Verb | To | When to use |
|---|---|---|---|
| idea | `realises` | epic, task | Idea becomes delivery work |
| idea | `spawns` | research | Idea triggers investigation |
| research | `produces` | wireframe | Investigation yields visual spec |
| research | `informs` | decision | Findings inform a choice |
| research | `guides` | epic | Findings guide delivery work |
| lesson | `teaches` | decision | Past mistake informs future choice |
| lesson | `cautions` | epic | Past mistake warns delivery work |
| decision | `drives` | epic | Choice motivates a body of work |
| task | `delivers` | epic | Work rolls up to parent |
| epic | `fulfils` | milestone | Body of work completes checkpoint |
| task | `depends-on` | task | Sequencing between tasks |
| task | `yields` | lesson | Work produces a learning |
| task | `fixes` | bug | Work resolves a reported issue |
| bug | `reports` | epic, task, milestone | Issue reported against work |
| bug | `affects` | persona | Issue impacts a user type |

## Status Progression

```
captured → exploring → ready → prioritised → active → review → completed
                                                ↕
                                           hold / blocked
```

- When all child tasks are `completed` → parent epic moves to `review`
- When `depends-on` targets are unmet → status becomes `blocked`
- When blocked dependencies are met → status returns to `ready`

## Delivery Hierarchy

```
Milestone (MS)
  ↑ fulfils
Epic (EPIC)
  ↑ delivers
Task (TASK)
```

## Traceability

Any artifact traces back to the idea and pillar through the graph:
```
task →(delivers)→ epic →(realised-by)→ idea →(grounded-by)→ pillar →(upholds)→ vision
```

## Validation

Run `orqa validate` before committing. The integrity checker verifies all relationship targets exist, inverses are present, verbs match from/to type constraints, and required frontmatter fields are present.
