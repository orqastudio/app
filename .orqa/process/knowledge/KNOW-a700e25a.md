---
id: KNOW-a700e25a
type: knowledge
name: Software Delivery Management
status: active
plugin: "@orqastudio/plugin-software-project"
relationships:
  - target: DOC-4554ff3e
    type: synchronised-with
---

# Software Delivery Management

You are managing software delivery artifacts in a structured project. This knowledge covers how to create, connect, and progress milestones, epics, tasks, research, wireframes, and bugs through the delivery lifecycle.

## Artifact Types

### Milestone
Top of the delivery hierarchy. Represents a significant project checkpoint or release.
- Stored in the project's delivery artifacts directory
- Key fields: `gate` (optional completion conditions), `target_date`
- Children: epics connect via `fulfils`

### Epic
A body of work delivering a coherent capability. Groups related tasks.
- Stored in the project's delivery artifacts directory
- Connects up: `fulfils` → milestone
- Connects down: tasks connect via `delivers`
- Origin: ideas connect via `realises` → epic (the validated idea that became this work)
- Motivation: decisions connect via `drives` → epic (the architecture choice that motivates this)
- Knowledge: research connects via `guides` → epic, lessons via `cautions` → epic

### Task
An atomic unit of work completable in a single session.
- Stored in the project's delivery artifacts directory
- Connects up: `delivers` → epic
- Sequencing: `depends-on` → other tasks
- Output: `yields` → lesson (when something is learned)
- Fixes: `fixes` → bug (when fixing a reported issue)

### Research
Investigation or analysis that produces findings.
- Stored in the project's discovery artifacts directory
- Origin: ideas connect via `spawns` → research
- Output: `produces` → wireframe, `informs` → decision, `guides` → epic

### Wireframe
Visual specification of a UI or interaction pattern.
- Stored in the project's discovery artifacts directory
- Origin: research connects via `produces` → wireframe

### Bug
A functional or display issue reported against existing work.
- Stored in the project's discovery artifacts directory
- Reports against: `reports` → epic, task, or milestone
- Impact: `affects` → persona
- Resolution: tasks connect via `fixes` → bug

## Creating Artifacts

Always include full frontmatter:

```yaml
---
id: <generated-id>
type: epic
name: Plugin Distribution System
status: active
relationships:
  - target: <milestone-id>
    type: fulfils
---
```

Rules:
1. **Every artifact MUST have `id`, `type`, `status`**
2. **Store forward relationships only** — the graph engine computes inverses at query time. Do NOT add stored inverse entries (e.g. `delivered-by` on the epic).
3. **Use the correct verb** — each relationship has specific from/to types. The verb constrains usage.
4. **ID = PREFIX-{first 8 hex of MD5(title)}** — deterministic, verifiable, no external state needed

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
Milestone
  ↑ fulfils
Epic
  ↑ delivers
Task
```

## Traceability

Any delivery artifact traces back to the idea and pillar through the graph:
```
task →(delivers)→ epic ←(realises)← idea ←(grounded)← pillar →(upholds)→ vision
```

## Validation

Before committing, run the project's integrity checker to verify all relationship targets exist, verbs match from/to type constraints, and required frontmatter fields are present.
