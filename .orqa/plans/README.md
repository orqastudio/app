---
title: "Implementation Plans"
tags: []
created: 2026-03-06
updated: 2026-03-06
---

# Implementation Plans

Phased implementation plans for OrqaStudio features. Each plan captures the problem statement, target UX, architectural approach, implementation phases, and verification criteria.

Plans live in `.orqa/plans/` and follow a structured lifecycle:

```
Draft --> Approved --> In Progress --> Complete
```

## Active Plans

| Plan | Status | Priority | Description |
|------|--------|----------|-------------|
| [Composability Gate](/plans/composability-gate) | Draft | Critical | Deep composability audit, refactoring, and platform architecture — the gate before dogfooding |

## Plan Structure

Every plan follows a consistent structure:

1. **Problem Statement** — What problem does this solve? Why now?
2. **Guiding Principles** — Constraints and design values shaping the solution
3. **Phases** — Ordered implementation steps with clear scope boundaries
4. **Implementation Order** — Sequenced tasks with agent assignments
5. **Verification** — Measurable criteria for completion
6. **Pillar Alignment** — How the plan serves the Self-Learning Loop and/or Process Governance pillars

## Frontmatter Schema

```yaml
---
title: "Plan Title"
status: draft              # draft | approved | in-progress | complete
priority: critical         # critical | high | medium | low
created: 2026-03-06
updated: 2026-03-06
phases: 4                  # Total number of implementation phases
completed-phases: 0        # Phases verified and merged
depends-on: []             # Other plans this depends on (filenames without .md)
blocks: [dogfooding]       # What this plan blocks (plan names or milestones)
pillar:                    # Which product pillars this serves
  - self-learning-loop
  - process-governance
owner: systems-architect   # Agent or role responsible for orchestrating execution
roadmap-ref: "2i"          # Roadmap phase reference
produces:                  # Artifacts produced on completion
  - skill/orqa-composability
  - rule/composability-enforcement
  - scanner/composability-anti-patterns
scope:                     # Codebase areas affected
  - src-tauri
  - ui
  - sidecar
research-refs:             # Research documents that informed this plan
  - mvp/claude-integration
  - mvp/frontend
tags: [composability, architecture]
---
```

### Field Reference

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `title` | Yes | string | Human-readable plan title |
| `status` | Yes | enum | `draft`, `approved`, `in-progress`, `complete` |
| `priority` | Yes | enum | `critical`, `high`, `medium`, `low` |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `phases` | No | integer | Total number of implementation phases |
| `completed-phases` | No | integer | Number of phases verified and merged |
| `depends-on` | No | string[] | Plan filenames (without `.md`) this depends on |
| `blocks` | No | string[] | Plans or milestones this blocks |
| `pillar` | No | string[] | Product pillars: `self-learning-loop`, `process-governance` |
| `owner` | No | string | Agent or role responsible for execution |
| `roadmap-ref` | No | string | Roadmap phase identifier (e.g., `"2i"`) |
| `produces` | No | string[] | Artifacts produced: `type/name` format (skill, rule, scanner, agent) |
| `scope` | No | string[] | Codebase directories affected |
| `research-refs` | No | string[] | Research doc paths that informed this plan (relative to `.orqa/research/`) |
| `tags` | No | string[] | Freeform tags for filtering and search |

### Traceability

The frontmatter fields create a traceability web across OrqaStudio's governance artifacts:

```
Research  --research-refs-->  Plan  --produces-->  Rules / Skills / Scanners
                                |
                          roadmap-ref --> Roadmap
                                |
                           depends-on / blocks --> Other Plans
                                |
                             pillar --> Product Vision
```

This enables future features like dependency graph visualization, impact analysis ("what breaks if this plan is delayed?"), and automated progress dashboards.

## Plan Lifecycle

- **Draft** — Plan is written and under review. No implementation work begins.
- **Approved** — User has reviewed and approved the plan. Implementation can start.
- **In Progress** — Active implementation underway, tracked phase by phase.
- **Complete** — All phases implemented, verified, and merged. Plan is closed.
