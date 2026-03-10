---
id: EPIC-037
title: Dogfood Readiness
status: done
priority: P1
milestone: MS-001
pillars: [PILLAR-001]
description: Multi-phase sprint to make OrqaStudio ready for self-hosted development (dogfooding). Covers governance alignment, frontend audit fixes, documentation alignment, enforcement engine, tool approval, lesson promotion, and SDK session resume.
created: 2026-03-05
updated: 2026-03-09
research-refs: [RES-018, RES-019]
docs-required: []
docs-produced: [.orqa/governance/rules/dogfood-mode.md, .orqa/governance/rules/development-commands.md]
scoring:
  user-value: 5
  pillar-alignment: 5
  dependency-weight: 5
  effort: 5
  risk: 4
  score: 24
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
