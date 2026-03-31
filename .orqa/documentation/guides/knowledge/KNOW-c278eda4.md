---
id: KNOW-c278eda4
type: knowledge
title: "Agent-optimized: Software Delivery Guide"
description: "Condensed software delivery system — artifact types, relationships, status workflow, and traceability."
status: active
tier: on-demand
relationships:
  - type: synchronised-with
    target: DOC-4554ff3e
---

# Software Delivery — Agent Reference

## Artifact Types

| Type | Prefix | Section | Purpose |
| --- | --- | --- | --- |
| Milestone | MS | Delivery | Major checkpoints and releases |
| Epic | EPIC | Delivery | Coherent bodies of work |
| Task | TASK | Delivery | Atomic work units |
| Research | RES | Discovery | Investigation and analysis |
| Wireframe | WF | Discovery | Visual specifications |
| Bug | BUG | Discovery | Functional and display issues |

## Delivery Hierarchy

Task --(delivers)--> Epic --(fulfils)--> Milestone

## Key Relationship Verbs

| From | Verb | To |
| --- | --- | --- |
| idea | realises | epic, task |
| idea | spawns | research |
| research | produces | wireframe |
| research | informs | decision |
| research | guides | epic |
| decision | drives | epic |
| decision | governs | rule |
| task | delivers | epic |
| epic | fulfils | milestone |
| task | depends-on | task |
| task | yields | lesson |
| task | fixes | bug |
| bug | reports | epic, task, milestone |
| bug | affects | persona |
| lesson | teaches | decision |
| lesson | cautions | epic |

## Status Workflow

Captured -> Exploring -> Ready -> Prioritised -> Active -> Review -> Completed

Side states: Hold, Blocked (inferred from depends-on), Surpassed, Archived

## Automatic Transitions

- All tasks delivering to epic completed -> epic moves to review
- Task depends-on targets not completed -> task becomes blocked
- All depends-on targets completed -> task returns to ready
- All epics fulfilling milestone completed -> milestone moves to review

## Traceability Chain

task -> (delivers) -> epic -> (realised-by) -> idea -> (grounded-by) -> pillar -> (upholds) -> vision

If a task cannot trace to a pillar, either:
- Epic missing realised-by link to idea
- Idea missing grounded-by link to pillar
- Idea does not fit vision — consider a pivot
