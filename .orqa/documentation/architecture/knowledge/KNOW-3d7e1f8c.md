---
id: KNOW-3d7e1f8c
type: knowledge
status: active
title: "Principle Decisions vs Planning Decisions"
description: "Two decision artifact types — their differences, when to use each, and where they live in the .orqa/ structure"
tier: stage-triggered
created: 2026-03-29
roles: [governance-steward, writer, orchestrator, planner]
paths: [.orqa/learning/decisions/, .orqa/planning/decisions/]
tags: [decisions, principle-decision, planning-decision, governance]
relationships:
  - type: synchronised-with
    target: DOC-80a4cf76
---

# Principle Decisions vs Planning Decisions

## Two Decision Types

OrqaStudio has two distinct decision artifact types. Using the wrong type misplaces the artifact and creates confusion about what can be changed.

| Dimension | `principle-decision` | `planning-decision` |
| --------- | -------------------- | ------------------- |
| **ID Prefix** | `PD-` | `PLANNING-` |
| **Location** | `learning/decisions/` | `planning/decisions/` |
| **Scope** | Architectural, wide-reaching | Tactical, implementation-level |
| **Stability** | Rarely changes | May evolve as implementation progresses |
| **Effect of violation** | Breaks the system's foundational design | Creates implementation inconsistency |
| **Examples** | No inheritance in state machines; forward-only relationships; Rust/TypeScript boundary | PR strategy for a sprint; branching model for a feature; test scope for a phase |

## Principle Decisions (`PD-`)

Overarching architecture decisions with wide-reaching consequences. These define the invariants of the system.

**Characteristics:**

- Once made, changing them requires a significant coordinated effort
- Violations are architectural debts, not just implementation inconsistencies
- Should be referenced in related implementation work
- Live in `learning/decisions/` — they are architectural learnings, not tactical plans

**Examples:**

- No workflow inheritance (each plugin owns complete state machine)
- Forward-only relationship storage (graph computes inverses)
- Rust below, TypeScript at UI surface only
- Daemon is the business logic boundary for MCP/LSP
- No backwards compatibility during pre-release

## Planning Decisions (`PLANNING-`)

Implementation-level tactical decisions. Expected to evolve as implementation progresses.

**Characteristics:**

- Reflect the current best approach for a specific delivery concern
- May be superseded by new information or changed requirements
- Live in `planning/decisions/` — they are part of delivery planning
- Can be archived when superseded without affecting foundational architecture

**Examples:**

- Sprint PR strategy (monolithic vs split)
- Migration phase sequencing decision
- Test environment configuration choice
- Branching strategy for a feature

## Decision Frontmatter

```yaml
---
id: PD-xxxxxxxx        # or PLANNING-xxxxxxxx
type: principle-decision   # or planning-decision
title: "No State Machine Inheritance"
description: "Each workflow plugin owns its complete state machine with no inheritance"
status: active
created: 2026-03-01
updated: 2026-03-01
relationships:
  - type: governs
    target: DOC-41ccf7c4
---
```

## Common Mistakes

- Using `principle-decision` for a sprint-level tactical choice — those are `planning-decision`
- Using `planning-decision` for a foundational architectural invariant — those are `principle-decision`
- Placing either type in the wrong location (PD must be in `learning/decisions/`, PLANNING must be in `planning/decisions/`)
- Creating a decision artifact for something that should be a rule — if it's an actionable behavioral constraint, write a `rule` instead
