---
id: RES-f27cfd70
type: discovery-research
title: "Enforcement Engine & Process Compliance"
description: "Design and implementation of the process enforcement engine: governance scanning, tool approval UI, model selection, lesson promotion pipeline, and process violation detection."
status: completed
created: 2026-03-05
updated: 2026-03-09
relationships:
  - target: EPIC-63ff87da
    type: guides
    rationale: "Research findings informed the design of Dogfood Readiness"
---

## Scope

Six implementation phases delivered in a single sprint:

1. **Enforcement engine** (Phase 2+3+11) — Core scanning logic, tool approval workflow, model selection
2. **Enforcement UI** (Phase 4+5+7+10) — Scanner dashboard, governance scanners, process violation display, context handling
3. **Lesson promotion pipeline** (Phase 6) — Self-learning loop: IMPL entries → recurrence tracking → rule promotion
4. **SDK session resume** — Resume conversations across app restarts
5. **code_research tool** — Native implementation of the architectural analysis tool
6. **Process violations** — Real-time detection and display of governance rule violations

## Key Design Decisions

- Governance analysis runs in Rust domain layer, not in commands
- Tool approval uses Channel\<T\> for streaming approval requests to the frontend
- Lesson promotion is config-driven (recurrence threshold in project.json)
- Process violations are non-blocking warnings, not hard errors

## Git Evidence

- `54acaf4` — Enforcement engine, tool approval, model selection
- `e2047a9` — Enforcement UI, governance scanners, process enforcement
- `ebabb95` — Lesson promotion pipeline
- `3a469da` — SDK session resume, code_research, process violations
