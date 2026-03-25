---
id: "EPIC-63ff87da"
type: epic
title: "Dogfood Readiness"
description: "Multi-phase sprint to make OrqaStudio ready for self-hosted development (dogfooding). Covers governance alignment, frontend audit fixes, documentation alignment, enforcement engine, tool approval, lesson promotion, and SDK session resume."
status: "completed"
priority: "P1"
created: "2026-03-05"
updated: "2026-03-09"
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 4
relationships:
  - target: "RES-ef35d2de"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-ef35d2de"
  - target: "RES-f27cfd70"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-f27cfd70"
  - target: "TASK-b913bdef"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-3528981f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-0befeab2"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-72910a57"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-39504e66"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
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