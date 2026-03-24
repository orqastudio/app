---
id: EPIC-b2f0399e
type: epic
title: "Code quality audit and enforcement alignment"
description: "Comprehensive audit of codebase against documented coding standards, lint rule coverage, test coverage, dead code, schema validation compliance, and recurring lessons. Findings feed into the learning loop as lessons and enforcement improvements."
status: captured
priority: P2
created: 2026-03-07
updated: 2026-03-24
horizon: next
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 2
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
  - target: PILLAR-cdf756ff
    type: grounded
    rationale: "Audit findings feed into the learning loop — lessons, rule promotions, enforcement improvements"
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Enforcement alignment ensures documented standards are mechanically enforced, making governance structure tangible"
  - target: TASK-a1c2d3e4
    type: delivered-by
  - target: TASK-d4f5a6b7
    type: delivered-by
  - target: TASK-a7c8d9e0
    type: delivered-by
  - target: TASK-b2d3e4f5
    type: delivered-by
  - target: TASK-f6b7c8d9
    type: delivered-by
  - target: TASK-b8d9e0f1
    type: delivered-by
  - target: TASK-c3e4f5a6
    type: delivered-by
  - target: TASK-e5a6b7c8
    type: delivered-by
---

## Why P2

Can't credibly enforce quality on managed projects if our own code has violations. The audit is also a learning loop input — findings feed into lessons and coding standards. As a dogfood project, enforcement gaps undermine product credibility (RULE-6083347d).

## Scope

### In Scope

| Area | Description |
|------|-------------|
| Lint rule coverage audit | Verify all documented coding standards in `.orqa/documentation/development/coding-standards.md` have corresponding linter rules |
| Test coverage gaps | Identify modules below 80% coverage threshold |
| Dead code removal | Find unused functions, imports, and components across Rust and TypeScript |
| Schema validation compliance | Verify all `.orqa/` artifacts pass their directory schema.json validation |
| Coding standard enforcement | Verify pre-commit hooks catch all documented violations |
| Recurring lessons | Audit IMPL entries with recurrence >= 2 for systemic issues requiring promotion |
| Sources of truth audit | Find and eliminate duplicate data sources, superseded implementations, and multiple sources for the same data (e.g. Cytoscape/Rust graph health duplication) |

### Out of Scope

- Feature implementation (this is audit and fix only)
- Architecture changes (findings may produce ideas for future epics)
- Performance optimization (separate concern)

## Tasks

| Task | Title | Status |
|------|-------|--------|
| [TASK-a1c2d3e4](TASK-a1c2d3e4) | Lint rule coverage audit | ready |
| [TASK-b2d3e4f5](TASK-b2d3e4f5) | Test coverage gap analysis | ready |
| [TASK-c3e4f5a6](TASK-c3e4f5a6) | Dead code removal | ready |
| [TASK-d4f5a6b7](TASK-d4f5a6b7) | Schema validation compliance audit | ready |
| [TASK-e5a6b7c8](TASK-e5a6b7c8) | Pre-commit hook enforcement verification | ready |
| [TASK-f6b7c8d9](TASK-f6b7c8d9) | Recurring lessons audit and promotion | ready |
| [TASK-b8d9e0f1](TASK-b8d9e0f1) | Sources of truth audit | ready |
| [TASK-a7c8d9e0](TASK-a7c8d9e0) | Reconcile EPIC-b2f0399e | ready |

## Pillars Served

- **PILLAR-cdf756ff (Learning Through Reflection)**: Every audit finding is a lesson input. Recurring patterns are promoted to rules. The audit itself exercises the learning loop.
- **PILLAR-569581e0 (Clarity Through Structure)**: Enforcement alignment makes governance tangible — documented standards have matching linter rules, pre-commit hooks, and schema validation.

## Implementation Design

Each task is an independent audit activity that produces a findings report. Findings are triaged into:
1. **Immediate fixes** — violations that can be fixed in the same task
2. **Lessons** — patterns that should be logged as IMPL entries
3. **Promotions** — lessons with recurrence >= 2 that should become rules or knowledge updates
4. **Ideas** — larger issues that warrant their own epic

The reconciliation task (TASK-a7c8d9e0) verifies all findings have been addressed before the epic closes.
