---
id: DOC-SW-001
type: doc
name: Software Delivery Guide
status: active
layer: plugin
plugin: "@orqastudio/plugin-software-project"
relationships:
  - target: SKILL-SW-001
    type: synchronised-with
  - target: PILLAR-001
    type: grounded-by
---

# Software Delivery Guide

This guide explains how to use OrqaStudio's software delivery system to plan, track, and deliver software projects. The delivery system provides structured artifact types that connect your work back to your project's principles, decisions, and research.

## Overview

The software delivery plugin adds five artifact types to your project:

| Type | Prefix | Section | Purpose |
|------|--------|---------|---------|
| **Milestone** | MS | Delivery | Major project checkpoints and releases |
| **Epic** | EPIC | Delivery | Coherent bodies of work |
| **Task** | TASK | Delivery | Atomic work units |
| **Research** | RES | Discovery | Investigation and analysis |
| **Wireframe** | WF | Discovery | Visual specifications |

## The Delivery Hierarchy

Delivery flows downward through three levels:

```
Milestone  →  defines the destination
  └── Epic  →  defines the capability
        └── Task  →  defines the work
```

Each level connects to its parent through the `delivers` relationship. A task "delivers to" an epic, which "delivers to" a milestone.

### Milestones

Milestones represent significant checkpoints — releases, demos, or phase completions. They sit at the top of the delivery hierarchy.

**When to create a milestone:**
- You're planning a release or demo
- Multiple epics converge on a single deadline
- You need a gate (a set of conditions that must be met)

**Key fields:**
- `gate` — optional conditions for completion (e.g. "all epics completed, QA sign-off")
- `target_date` — when you aim to reach this milestone

### Epics

Epics are bodies of work that deliver a coherent capability. They group related tasks and connect upward to milestones and decisions.

**When to create an epic:**
- A decision requires implementation across multiple tasks
- A capability needs planning before work begins
- You want to track progress toward a coherent goal

**Important connections:**
- `delivers` → the milestone this epic contributes to
- `driven-by` → the architecture decision motivating this work
- `grounded-by` → the principle this epic traces back to
- `evolves-from` → the idea that was validated into this epic

### Tasks

Tasks are atomic work units — small enough to complete in a single session.

**When to create a task:**
- You have a concrete piece of work to do
- The scope is clear and bounded
- You can describe what "done" looks like

**Important connections:**
- `delivers` → the epic this task belongs to
- `depends-on` → other tasks that must complete before this one
- `informed-by` → research or lessons that guide implementation

## Discovery Artifacts

### Research

Research documents capture investigation, analysis, and findings. They exist in the discovery phase and flow their findings into the delivery system.

**When to create research:**
- You need to investigate options before deciding
- You want to document analysis of a problem space
- Technical spikes or prototyping results

**Key connections:**
- `informs` → decisions, ideas, or epics that use these findings
- Research findings often lead to decisions (AD-nnn) which then drive epics

### Wireframes

Wireframes capture visual specifications — UI layouts, interaction flows, or component designs.

**When to create wireframes:**
- You're designing a new view or component
- You need to communicate visual intent before building
- UX decisions need to be documented

**Key connections:**
- `informs` → the epics and tasks that implement this design

## How Delivery Connects to Governance

Every piece of work traces back to your project's principles through the graph — not through direct links, but through the natural chain of relationships:

```
Pillar  ←  grounded-by  ←  Idea
                              ↑ evolves-from
Decision  ←  driven-by  ←  Epic
                              ↑ delivers
Research  ←  informed-by  ←  Task
Lesson    ←  informed-by  ←  Task
```

Traceability is transitive: to answer "why are we doing this task?", walk the graph: task → (delivers) → epic → (evolves-from) → idea → (grounded-by) → pillar. You don't need to connect every artifact directly to a pillar — the graph connects them naturally.

## Status Workflow

All delivery artifacts progress through the same canonical statuses:

1. **Captured** — recorded but not yet explored
2. **Exploring** — being investigated or scoped
3. **Ready** — scoped and ready for prioritisation
4. **Prioritised** — scheduled for work
5. **Active** — currently being worked on
6. **Review** — work complete, awaiting verification
7. **Completed** — done and verified

Side states:
- **Hold** — paused intentionally
- **Blocked** — waiting on a dependency
- **Surpassed** — replaced by newer work
- **Archived** — no longer relevant

### Automatic Transitions

The system can automatically transition statuses:
- When all tasks in an epic are completed → the epic moves to **review**
- When a task's dependencies are unmet → the task becomes **blocked**
- When blocked dependencies are resolved → the task returns to **ready**

## Using the Roadmap View

The Roadmap view provides a visual board for delivery planning:

- **Horizon Board** — milestones grouped by Now / Next / Later / Completed
- **Status Kanban** — epics or tasks in status columns
- **Drill-down** — click a milestone to see its epics, click an epic to see its tasks

Navigate to **Delivery → Roadmap** in the sidebar to access it.

## Best Practices

1. **Start with milestones** — define where you're going before planning how
2. **Connect epics to decisions** — if there's no decision driving the work, create one
3. **Keep tasks atomic** — if a task takes more than one session, split it
4. **Use research before deciding** — capture findings before making architecture decisions
5. **Check traceability** — every epic should have `delivers` (to milestone) and `driven-by` (from decision) or `evolves-from` (from idea)
6. **Run integrity checks** — `orqa validate` catches missing relationships and broken links
