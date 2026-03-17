---
id: EPIC-037
title: Dogfood Readiness
description: "Multi-phase sprint to make OrqaStudio ready for self-hosted development (dogfooding). Covers governance alignment, frontend audit fixes, documentation alignment, enforcement engine, tool approval, lesson promotion, and SDK session resume."
status: completed
priority: P1
created: 2026-03-05
updated: 2026-03-09
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 4
relationships:
  - target: RES-018
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-018
  - target: RES-019
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-019
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-019
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-020
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-021
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-022
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-327
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: RULE-009
    type: informed-by
  - target: RULE-007
    type: informed-by
---
## Implementation Design

### Phase 1: Governance Alignment
- Hook paths updated to use `$CLAUDE_PROJECT_DIR`
- TODO rewritten for dogfood milestone
- Governance artifacts aligned with codebase

### Phase 2: Frontend Audit
- Debug logging removed
- `any` types fixed
- Documentation aligned with implementation

### Phase 3: Function Decomposition
- Oversized functions broken down
- Root directory cleaned

### Phase 4: Enforcement Engine
- Governance scanning logic
- Tool approval workflow via Channel<T>
- Model selection UI
- Enforcement UI and scanner dashboard
- Process violation detection and display

### Phase 5: Self-Learning Loop
- Lesson promotion pipeline: IMPL entries → recurrence tracking → rule promotion
- Config-driven recurrence threshold

### Phase 6: SDK Integration
- Session resume across app restarts
- `code_research` native tool implementation
- Process violation hook fixes

## Git Evidence

- `1481f00` through `3a469da` — Full sprint (2026-03-05)
- `0aab794` — Fix error swallowing and settings persistence

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
