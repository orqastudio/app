---
id: "EPIC-e24086ed"
type: "epic"
title: "Code quality audit and enforcement alignment"
description: "Comprehensive audit of codebase against documented coding standards, lint rule coverage, test coverage, dead code, schema validation compliance, and recurring lessons. Findings feed into the learning loop as lessons and enforcement improvements."
status: "captured"
priority: "P2"
created: 2026-03-07T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
horizon: "next"
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 2
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
  - target: "PILLAR-2acd86c1"
    type: "grounded"
    rationale: "Audit findings feed into the learning loop — lessons, rule promotions, enforcement improvements"
  - target: "PILLAR-c9e0a695"
    type: "grounded"
    rationale: "Enforcement alignment ensures documented standards are mechanically enforced, making governance structure tangible"
---

## Why P2

Can't credibly enforce quality on managed projects if our own code has violations. The audit is also a learning loop input — findings feed into lessons and coding standards. As a dogfood project, enforcement gaps undermine product credibility (RULE-998da8ea).

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
| [TASK-4a9f0681](TASK-4a9f0681) | Lint rule coverage audit | ready |
| [TASK-74bb7696](TASK-74bb7696) | Test coverage gap analysis | ready |
| [TASK-9f3a2a69](TASK-9f3a2a69) | Dead code removal | ready |
| [TASK-dbbb669b](TASK-dbbb669b) | Schema validation compliance audit | ready |
| [TASK-7fbf7d28](TASK-7fbf7d28) | Pre-commit hook enforcement verification | ready |
| [TASK-535e93f4](TASK-535e93f4) | Recurring lessons audit and promotion | ready |
| [TASK-2606346b](TASK-2606346b) | Sources of truth audit | ready |
| [TASK-8702813d](TASK-8702813d) | Reconcile EPIC-e24086ed | ready |

## Pillars Served

- **PILLAR-2acd86c1 (Learning Through Reflection)**: Every audit finding is a lesson input. Recurring patterns are promoted to rules. The audit itself exercises the learning loop.
- **PILLAR-c9e0a695 (Clarity Through Structure)**: Enforcement alignment makes governance tangible — documented standards have matching linter rules, pre-commit hooks, and schema validation.

## Implementation Design

Each task is an independent audit activity that produces a findings report. Findings are triaged into:
1. **Immediate fixes** — violations that can be fixed in the same task
2. **Lessons** — patterns that should be logged as IMPL entries
3. **Promotions** — lessons with recurrence >= 2 that should become rules or knowledge updates
4. **Ideas** — larger issues that warrant their own epic

The reconciliation task (TASK-8702813d) verifies all findings have been addressed before the epic closes.
