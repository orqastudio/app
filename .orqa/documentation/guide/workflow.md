---
id: DOC-082
title: Governance Workflow
description: How the OrqaStudio governance workflow operates, from ideas through delivery and learning.
created: "2026-03-14"
updated: "2026-03-14"
---

## The Thinking Loop

OrqaStudio structures work as a continuous loop: **Understand, Plan, Document, Implement, Review, Learn**. Each step produces artifacts that feed into the next.

```
Understand → Plan → Document → Implement → Review → Learn → Understand
```

This is not a linear waterfall. The loop runs at every scale: within a single task, across an epic, and across a milestone. Learning feeds back into understanding, which improves the next plan.

## From Idea to Delivery

### 1. Capture Ideas

When a future possibility is identified, create an `IDEA-NNN` with `status: captured`. Ideas carry `research-needed` fields listing questions that must be answered.

### 2. Explore and Shape

With user approval, an idea moves to `exploring`. Research artifacts are created to answer the open questions. Once all questions are addressed and the scope is clear, the idea moves to `shaped`.

### 3. Promote to Epic

A shaped idea is promoted to an `EPIC-NNN`. The epic contains the implementation design, documentation gates, and priority assessment. The idea's `promoted-to` field links to the epic.

### 4. Plan Tasks

Within an epic, individual `TASK-NNN` artifacts define the work items. Each task has:
- **Acceptance criteria** defining what "done" means
- **Dependencies** on other tasks
- **Skills** that agents need to load

### 5. Implement

Tasks are assigned to agent roles (Implementer, Designer, Writer). The orchestrator coordinates delegation, ensuring skills are loaded and governing documentation is read before work begins.

### 6. Review

An independent Reviewer verifies each task against its acceptance criteria. The implementing agent cannot self-certify quality. Review verdicts are PASS or FAIL.

### 7. Learn

When reviews reveal patterns (recurring mistakes, non-obvious gotchas), lessons are created in `.orqa/process/lessons/`. Lessons that recur frequently are promoted to rules or skill updates.

## Documentation Gates

Epics enforce documentation at two points:

- **`docs-required`**: Documentation that must exist before implementation starts. The epic cannot move to `ready` without these.
- **`docs-produced`**: Documentation that the work must create or update. The epic cannot be marked `done` without these.

This ensures documentation stays in sync with implementation.

## Integrity Checking

OrqaStudio continuously validates the artifact graph:

- **Broken links**: Cross-references that point to non-existent artifacts
- **Missing inverses**: One-sided relationships (A links to B but B does not link back to A)
- **Dependency violations**: In-progress tasks whose dependencies are not done
- **Planning gaps**: Artifacts without milestone or horizon placement
- **Promotion chain integrity**: Promoted lessons must link to real rules

The dashboard surfaces these as warnings and errors, with suggested actions for resolution.

## The Knowledge Pipeline

The pipeline widget tracks how knowledge matures through stages:

```
Observation → Understanding → Principle → Practice → Enforcement → Verification
```

Each stage maps to artifact types:
- **Observation**: Lessons, research findings
- **Understanding**: Research documents, shaped ideas
- **Principle**: Decisions, pillars
- **Practice**: Skills, documentation
- **Enforcement**: Rules with enforcement entries
- **Verification**: Integrity checks, review verdicts

A healthy pipeline shows knowledge flowing through all stages. Bottlenecks indicate areas where knowledge is getting stuck (e.g., many lessons but few rules means the promotion pipeline is blocked).

## Roles

OrqaStudio defines universal roles that can be filled by humans or AI agents:

| Role | Purpose |
|------|---------|
| **Orchestrator** | Coordinates work, manages artifacts, delegates tasks |
| **Implementer** | Builds things (code, configurations, content) |
| **Reviewer** | Checks quality and correctness independently |
| **Researcher** | Investigates questions, gathers information |
| **Planner** | Designs approaches, maps dependencies |
| **Writer** | Creates documentation and specifications |
| **Designer** | Designs interfaces and experiences |

The orchestrator never implements directly. Every implementation task is delegated to the appropriate role with the required skills loaded.
