---
id: DOC-SW-421219ce
type: doc
name: Software Delivery Guide
category: how-to
status: active
plugin: "@orqastudio/plugin-software-project"
relationships:
  - target: SKILL-SW-1d47d8d8
    type: synchronised-with
  - target: SKILL-f0efaf83
    type: synchronised-with
  - target: SKILL-353a228b
    type: synchronised-with
  - target: SKILL-1b805150
    type: synchronised-with
  - target: SKILL-170c220e
    type: synchronised-with
  - target: SKILL-bcb42347
    type: synchronised-with
  - target: SKILL-c6d04755
    type: synchronised-with
  - target: SKILL-5124e508
    type: synchronised-with

---

# Software Delivery Guide

This guide explains how OrqaStudio's software delivery system works — how to plan, track, and deliver software projects using structured artifacts that connect your work back to your project's principles.

## Overview

The software delivery plugin adds artifact types for the full development lifecycle:

| Type | Prefix | Section | Purpose |
|------|--------|---------|---------|
| **Milestone** | MS | Delivery | Major checkpoints and releases |
| **Epic** | EPIC | Delivery | Coherent bodies of work |
| **Task** | TASK | Delivery | Atomic work units |
| **Research** | RES | Discovery | Investigation and analysis |
| **Wireframe** | WF | Discovery | Visual specifications |
| **Bug** | BUG | Discovery | Functional and display issues |

## The Delivery Hierarchy

Work is structured in three levels:

```
Milestone  →  defines the destination
  ↑ fulfils
Epic       →  defines the capability
  ↑ delivers
Task       →  defines the work
```

A task **delivers** to an epic. An epic **fulfils** a milestone. Different verbs because the relationships are semantically different — a task contributes incremental work, an epic completes a checkpoint.

## How Everything Connects

Every artifact uses a specific relationship verb that describes the nature of the connection. The verb constrains what can connect to what.

### From Ideas to Delivery

Ideas are the seeds. They enter the graph, get grounded by a pillar and linked to a persona, then flow into work:

```
Pillar ←(grounded-by)← Idea →(benefits)→ Persona
                          │
                ┌─────────┼──────────┐
                ↓         ↓          ↓
            Research   Decision     Epic
           (spawns)  (crystallises) (realises)
```

- An idea **spawns** research (investigation)
- An idea **crystallises** into a decision (a choice about how to proceed)
- An idea is **realised** by an epic or task (the work that makes it real)

### From Research to Implementation

Research produces outputs and feeds knowledge forward:

- Research **produces** wireframes (visual specifications)
- Research **informs** decisions (findings shape choices)
- Research **guides** epics (findings shape delivery work)

### From Decisions to Work

Decisions flow in two directions:

- A decision **drives** an epic (motivates delivery work)
- A decision **governs** a rule (establishes governance in the learning loop)

### From Work to Learning

Work produces learning that feeds back:

- A task **yields** a lesson (something learned during execution)
- A lesson **teaches** a decision (past experience shapes future choices)
- A lesson **cautions** an epic (warnings about what to watch for)
- When a lesson can be enforced, a rule **codifies** it

### Bug Lifecycle

Bugs are corrective — they report issues and get fixed:

- A bug **reports** against an epic, task, or milestone (what's broken)
- A bug **affects** a persona (who is impacted)
- A task **fixes** a bug (the corrective work)

## Status Workflow

All delivery artifacts progress through canonical statuses:

1. **Captured** — recorded but not explored
2. **Exploring** — being investigated or scoped
3. **Ready** — scoped and ready for prioritisation
4. **Prioritised** — scheduled for work
5. **Active** — currently being worked on
6. **Review** — work complete, awaiting verification
7. **Completed** — done and verified

Side states:
- **Hold** — paused intentionally
- **Blocked** — waiting on a dependency (inferred from `depends-on` relationships)
- **Surpassed** — replaced by newer work
- **Archived** — no longer relevant

### Automatic Transitions

The system automatically transitions statuses based on the graph:

- When **all tasks** delivering to an epic are `completed` → the epic moves to `review`
- When a task's **depends-on** targets are not yet completed → the task becomes `blocked`
- When all **depends-on** targets are completed → the blocked task returns to `ready`
- When **all epics** fulfilling a milestone are `completed` → the milestone moves to `review`

These transitions are computed from the graph state — you don't set them manually.

## Traceability

Any artifact traces back to the project's vision through the graph:

```
task →(delivers)→ epic →(realised-by)→ idea →(grounded-by)→ pillar →(upholds)→ vision
```

If you can't trace a task back to a pillar, it means either:
- The epic is missing a `realised-by` link to an idea
- The idea is missing a `grounded-by` link to a pillar
- The idea doesn't fit the vision — consider a **pivot**

## Using the Roadmap View

Navigate to **Delivery → Roadmap** in the sidebar:

- **Horizon Board** — milestones grouped by Now / Next / Later / Completed
- **Status Kanban** — epics or tasks in status columns
- **Drill-down** — click a milestone to see its epics, click an epic to see its tasks

## Quick Reference: All Plugin Relationships

| From | Verb | To | Meaning |
|---|---|---|---|
| idea | `realises` | epic, task | Idea becomes delivery work |
| idea | `spawns` | research | Idea triggers investigation |
| research | `produces` | wireframe | Investigation yields visual spec |
| research | `informs` | decision | Findings inform a choice |
| research | `guides` | epic | Findings shape delivery work |
| lesson | `teaches` | decision | Past experience shapes choices |
| lesson | `cautions` | epic | Past experience warns delivery |
| decision | `drives` | epic | Choice motivates work |
| task | `delivers` | epic | Work rolls up to parent |
| epic | `fulfils` | milestone | Work completes checkpoint |
| task | `depends-on` | task | Must complete first |
| task | `yields` | lesson | Work produces learning |
| task | `fixes` | bug | Work resolves issue |
| bug | `reports` | epic, task, milestone | Issue against work |
| bug | `affects` | persona | Issue impacts user type |

## Best Practices

1. **Start with ideas** — every piece of work should trace back to an idea
2. **Ground ideas in pillars** — if an idea can't ground to a pillar, question it
3. **Link ideas to personas** — if an idea doesn't benefit a persona, question it
4. **Connect epics to decisions** — if there's no decision driving the work, create one
5. **Keep tasks atomic** — if a task takes more than one session, split it
6. **Track bugs separately** — bugs enter as discovery, get fixed through delivery
7. **Run integrity checks** — `orqa validate` catches missing relationships and type violations
